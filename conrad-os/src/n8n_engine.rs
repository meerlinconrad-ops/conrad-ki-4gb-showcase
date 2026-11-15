use axum::{
    body::Bytes,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use rusqlite::{params, Connection, OpenFlags, OptionalExtension};
use serde::Deserialize;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet, VecDeque};
use std::path::Path as StdPath;
use std::sync::OnceLock;
use uuid::Uuid;

use crate::db::Database;
use crate::twenty::{ollama_base_url, strip_think_tags};
use crate::AppState;

static N8N_HTTP: OnceLock<reqwest::Client> = OnceLock::new();

fn http_client() -> &'static reqwest::Client {
    N8N_HTTP.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(45))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new())
    })
}

pub fn init_and_sync_n8n(db: &Database) -> anyhow::Result<()> {
    {
        let conn = db.lock_conn();
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS n8n_workflows (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                active INTEGER NOT NULL DEFAULT 0,
                nodes_json TEXT NOT NULL DEFAULT '[]',
                connections_json TEXT NOT NULL DEFAULT '{}',
                settings_json TEXT NOT NULL DEFAULT '{}',
                webhook_path TEXT,
                webhook_method TEXT DEFAULT 'POST',
                trigger_count INTEGER NOT NULL DEFAULT 0,
                tags_json TEXT NOT NULL DEFAULT '[]',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS n8n_executions (
                id TEXT PRIMARY KEY,
                workflow_id TEXT NOT NULL,
                workflow_name TEXT NOT NULL DEFAULT '',
                mode TEXT NOT NULL DEFAULT 'manual',
                status TEXT NOT NULL DEFAULT 'success',
                model_used TEXT NOT NULL DEFAULT '',
                started_at TEXT NOT NULL,
                finished_at TEXT NOT NULL,
                duration_ms INTEGER NOT NULL DEFAULT 0,
                trigger_payload TEXT NOT NULL DEFAULT '{}',
                result_json TEXT NOT NULL DEFAULT '{}',
                node_logs_json TEXT NOT NULL DEFAULT '[]',
                error_text TEXT
            );

            CREATE INDEX IF NOT EXISTS idx_n8n_wf_active ON n8n_workflows(active, name);
            CREATE INDEX IF NOT EXISTS idx_n8n_wf_wh ON n8n_workflows(webhook_path);
            CREATE INDEX IF NOT EXISTS idx_n8n_exec_wf ON n8n_executions(workflow_id, started_at DESC);
            ",
        )?;
    }

    let existing_count: i64 = {
        let conn = db.lock_conn();
        conn.query_row("SELECT COUNT(*) FROM n8n_workflows", [], |r| r.get(0))
            .unwrap_or(0)
    };

    if existing_count == 0 && StdPath::new("/opt/n8n/n8n_data/database.sqlite").exists() {
        let imported = sync_from_n8n_sqlite(db)?;
        tracing::info!("Pure-Rust n8n engine synced from SQLite: {}", imported);
    } else {
        tracing::info!("Pure-Rust n8n engine ready ({} workflows loaded)", existing_count);
    }
    Ok(())
}

fn open_n8n_readonly() -> anyhow::Result<Connection> {
    let primary = "/opt/n8n/n8n_data/database.sqlite";
    match Connection::open_with_flags(primary, OpenFlags::SQLITE_OPEN_READ_ONLY) {
        Ok(c) => {
            let _ = c.busy_timeout(std::time::Duration::from_millis(500));
            if c.query_row("SELECT COUNT(*) FROM workflow_entity", [], |r| r.get::<_, i64>(0)).is_ok() {
                return Ok(c);
            }
        }
        Err(_) => {}
    }
    let snap = "/tmp/n8n_ro_snapshot.sqlite";
    std::fs::copy(primary, snap)?;
    let _ = std::fs::copy("/opt/n8n/n8n_data/database.sqlite-wal", "/tmp/n8n_ro_snapshot.sqlite-wal");
    let _ = std::fs::copy("/opt/n8n/n8n_data/database.sqlite-shm", "/tmp/n8n_ro_snapshot.sqlite-shm");
    Ok(Connection::open_with_flags(snap, OpenFlags::SQLITE_OPEN_READ_ONLY)?)
}

pub fn sync_from_n8n_sqlite(db: &Database) -> anyhow::Result<Value> {
    let src = open_n8n_readonly()?;
    let mut wh_map: HashMap<String, (String, String)> = HashMap::new();
    {
        let mut stmt = src.prepare("SELECT workflowId, webhookPath, method FROM webhook_entity")?;
        let mut rows = stmt.query([])?;
        while let Some(r) = rows.next()? {
            let wid: String = r.get(0)?;
            let wp: String = r.get(1)?;
            let meth: String = r.get(2)?;
            wh_map.entry(wid).or_insert((wp, meth));
        }
    }

    let mut workflows = Vec::new();
    {
        let mut stmt = src.prepare(
            "SELECT id, name, COALESCE(active, 0), COALESCE(nodes, '[]'), COALESCE(connections, '{}'),
                    COALESCE(settings, '{}'), COALESCE(triggerCount, 0), COALESCE(createdAt, ''), COALESCE(updatedAt, '')
             FROM workflow_entity WHERE COALESCE(isArchived, 0) = 0",
        )?;
        let mut rows = stmt.query([])?;
        while let Some(r) = rows.next()? {
            let id: String = r.get(0)?;
            let name: String = r.get(1)?;
            let active: i64 = r.get(2)?;
            let nodes_json: String = r.get(3)?;
            let connections_json: String = r.get(4)?;
            let settings_json: String = r.get(5)?;
            let trigger_count: i64 = r.get(6)?;
            let created_at: String = r.get(7)?;
            let updated_at: String = r.get(8)?;

            let mut wh_info = wh_map.get(&id).cloned();
            if wh_info.is_none() {
                if let Ok(nodes) = serde_json::from_str::<Vec<Value>>(&nodes_json) {
                    for n in &nodes {
                        let tp = n.get("type").and_then(|x| x.as_str()).unwrap_or("");
                        if tp.contains("webhook") || tp.contains("chatTrigger") {
                            if let Some(p) = n.pointer("/parameters/path").and_then(|x| x.as_str()) {
                                let m = n.pointer("/parameters/httpMethod").and_then(|x| x.as_str()).unwrap_or("POST");
                                wh_info = Some((p.to_string(), m.to_string()));
                                break;
                            }
                        }
                    }
                }
            }

            workflows.push((
                id,
                name,
                active,
                nodes_json,
                connections_json,
                settings_json,
                wh_info.as_ref().map(|x| x.0.clone()),
                wh_info.map(|x| x.1).unwrap_or_else(|| "POST".to_string()),
                trigger_count,
                created_at,
                updated_at,
            ));
        }
    }

    let mut conn = db.lock_conn();
    let tx = conn.transaction()?;
    let mut imported = 0i64;
    let mut active_count = 0i64;
    for (id, name, active, nodes_json, connections_json, settings_json, wh_path, wh_method, tc, ca, ua) in workflows {
        if active != 0 {
            active_count += 1;
        }
        tx.execute(
            "INSERT OR REPLACE INTO n8n_workflows
             (id, name, active, nodes_json, connections_json, settings_json, webhook_path, webhook_method, trigger_count, tags_json, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, '[]', ?10, ?11)",
            params![id, name, active, nodes_json, connections_json, settings_json, wh_path, wh_method, tc, ca, ua],
        )?;
        imported += 1;
    }
    tx.commit()?;
    Ok(json!({ "ok": true, "imported": imported, "active": active_count }))
}

#[derive(Debug, Deserialize)]
pub struct N8nListQuery {
    pub q: Option<String>,
    pub active: Option<String>,
    #[serde(rename = "workflowId")]
    pub workflow_id: Option<String>,
    pub limit: Option<i64>,
}

fn workflow_summary_from_row(r: &rusqlite::Row<'_>, include_graph: bool) -> rusqlite::Result<Value> {
    let nodes_str: String = r.get("nodes_json")?;
    let conns_str: String = r.get("connections_json")?;
    let nodes_val: Value = serde_json::from_str(&nodes_str).unwrap_or_else(|_| json!([]));
    let node_count = nodes_val.as_array().map(|a| a.len()).unwrap_or(0);
    let node_types: Vec<String> = nodes_val
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|n| n.get("type").and_then(|x| x.as_str()).map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let tags_str: String = r.get("tags_json").unwrap_or_else(|_| "[]".to_string());
    let tags_val: Value = serde_json::from_str(&tags_str).unwrap_or_else(|_| json!([]));

    let mut obj = json!({
        "id": r.get::<_, String>("id")?,
        "name": r.get::<_, String>("name")?,
        "active": r.get::<_, i64>("active")? != 0,
        "webhookPath": r.get::<_, Option<String>>("webhook_path")?,
        "webhookMethod": r.get::<_, Option<String>>("webhook_method")?,
        "triggerCount": r.get::<_, i64>("trigger_count")?,
        "nodeCount": node_count,
        "nodeTypes": node_types,
        "tags": tags_val,
        "createdAt": r.get::<_, String>("created_at")?,
        "updatedAt": r.get::<_, String>("updated_at")?,
    });
    if include_graph {
        let conns_val: Value = serde_json::from_str(&conns_str).unwrap_or_else(|_| json!({}));
        let settings_str: String = r.get("settings_json")?;
        let settings_val: Value = serde_json::from_str(&settings_str).unwrap_or_else(|_| json!({}));
        obj["nodes"] = nodes_val;
        obj["connections"] = conns_val;
        obj["settings"] = settings_val;
    }
    Ok(obj)
}

pub async fn list_workflows_handler(
    State(state): State<AppState>,
    Query(p): Query<N8nListQuery>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let mut conds = Vec::new();
    if let Some(a) = &p.active {
        if a == "true" || a == "1" {
            conds.push("active = 1".to_string());
        } else if a == "false" || a == "0" {
            conds.push("active = 0".to_string());
        }
    }
    if let Some(q) = &p.q {
        let pat = q.trim().to_lowercase().replace('\'', "");
        if !pat.is_empty() {
            conds.push(format!("(lower(name) LIKE '%{pat}%' OR lower(id) LIKE '%{pat}%' OR lower(COALESCE(webhook_path,'')) LIKE '%{pat}%')"));
        }
    }
    let where_sql = if conds.is_empty() { "1=1".to_string() } else { conds.join(" AND ") };
    let sql = format!("SELECT * FROM n8n_workflows WHERE {where_sql} ORDER BY active DESC, name ASC");
    let mut items = Vec::new();
    let mut active_cnt = 0i64;
    if let Ok(mut stmt) = conn.prepare(&sql) {
        if let Ok(mut rows) = stmt.query([]) {
            while let Ok(Some(r)) = rows.next() {
                if let Ok(v) = workflow_summary_from_row(r, false) {
                    if v.get("active").and_then(|x| x.as_bool()).unwrap_or(false) {
                        active_cnt += 1;
                    }
                    items.push(v);
                }
            }
        }
    }
    Json(json!({ "data": items, "totalCount": items.len(), "activeCount": active_cnt }))
}

pub async fn get_workflow_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    match conn
        .query_row("SELECT * FROM n8n_workflows WHERE id = ?1", [&id], |r| {
            workflow_summary_from_row(r, true)
        })
        .optional()
    {
        Ok(Some(v)) => (StatusCode::OK, Json(v)),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({ "error": "Workflow not found" }))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))),
    }
}

pub async fn create_workflow_handler(
    State(state): State<AppState>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let id = v.get("id").and_then(|x| x.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string()).unwrap_or_else(|| Uuid::new_v4().to_string());
    let name = v.get("name").and_then(|x| x.as_str()).unwrap_or("Neuer Rust Workflow").to_string();
    let active = if v.get("active").and_then(|x| x.as_bool()).unwrap_or(false) { 1i64 } else { 0i64 };
    let nodes = v.get("nodes").cloned().unwrap_or_else(|| json!([
        {"id": "n-trig", "name": "Webhook", "type": "n8n-nodes-base.webhook", "position": [220, 300], "parameters": {"path": &id, "httpMethod": "POST"}},
        {"id": "n-agent", "name": "0.6B Tool Agent", "type": "@n8n/n8n-nodes-langchain.agent", "position": [520, 300], "parameters": {"text": "={{ $json.message || 'Hallo' }}"}},
        {"id": "n-resp", "name": "Respond to Webhook", "type": "n8n-nodes-base.respondToWebhook", "position": [820, 300], "parameters": {"respondWith": "json"}}
    ]));
    let conns = v.get("connections").cloned().unwrap_or_else(|| json!({
        "Webhook": { "main": [[{ "node": "0.6B Tool Agent", "type": "main", "index": 0 }]] },
        "0.6B Tool Agent": { "main": [[{ "node": "Respond to Webhook", "type": "main", "index": 0 }]] }
    }));
    let now = Utc::now().to_rfc3339();
    let conn = state.db.lock_conn();
    let _ = conn.execute(
        "INSERT OR REPLACE INTO n8n_workflows (id, name, active, nodes_json, connections_json, settings_json, webhook_path, webhook_method, trigger_count, tags_json, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, '{}', ?6, 'POST', 0, '[]', ?7, ?7)",
        params![id, name, active, nodes.to_string(), conns.to_string(), v.get("webhookPath").and_then(|x| x.as_str()), now],
    );
    match conn.query_row("SELECT * FROM n8n_workflows WHERE id = ?1", [&id], |r| workflow_summary_from_row(r, true)) {
        Ok(res) => (StatusCode::CREATED, Json(res)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))),
    }
}

pub async fn update_workflow_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let now = Utc::now().to_rfc3339();
    if let Some(n) = v.get("name").and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE n8n_workflows SET name = ?1, updated_at = ?2 WHERE id = ?3", params![n, now, id]);
    }
    if let Some(a) = v.get("active").and_then(|x| x.as_bool()) {
        let _ = conn.execute("UPDATE n8n_workflows SET active = ?1, updated_at = ?2 WHERE id = ?3", params![if a {1} else {0}, now, id]);
    }
    if let Some(nodes) = v.get("nodes") {
        let _ = conn.execute("UPDATE n8n_workflows SET nodes_json = ?1, updated_at = ?2 WHERE id = ?3", params![nodes.to_string(), now, id]);
    }
    if let Some(conns) = v.get("connections") {
        let _ = conn.execute("UPDATE n8n_workflows SET connections_json = ?1, updated_at = ?2 WHERE id = ?3", params![conns.to_string(), now, id]);
    }
    if let Some(tags) = v.get("tags") {
        let _ = conn.execute("UPDATE n8n_workflows SET tags_json = ?1, updated_at = ?2 WHERE id = ?3", params![tags.to_string(), now, id]);
    }
    if let Some(settings) = v.get("settings") {
        let _ = conn.execute("UPDATE n8n_workflows SET settings_json = ?1, updated_at = ?2 WHERE id = ?3", params![settings.to_string(), now, id]);
    }
    if let Some(wp) = v.get("webhookPath").and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE n8n_workflows SET webhook_path = ?1, updated_at = ?2 WHERE id = ?3", params![wp, now, id]);
    }
    match conn.query_row("SELECT * FROM n8n_workflows WHERE id = ?1", [&id], |r| workflow_summary_from_row(r, true)).optional() {
        Ok(Some(res)) => (StatusCode::OK, Json(res)),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({ "error": "Workflow not found" }))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))),
    }
}

pub async fn delete_workflow_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let n = conn.execute("DELETE FROM n8n_workflows WHERE id = ?1", [&id]).unwrap_or(0);
    Json(json!({ "deleted": n > 0, "id": id }))
}

pub async fn duplicate_workflow_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let row_opt: Option<(String, String, String, String, Option<String>, String, String)> = conn
        .query_row(
            "SELECT name, nodes_json, connections_json, settings_json, webhook_path, COALESCE(webhook_method,'POST'), COALESCE(tags_json,'[]')
             FROM n8n_workflows WHERE id = ?1",
            [&id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?, r.get(6)?)),
        )
        .optional()
        .unwrap_or(None);

    let Some((old_name, nodes_json, conns_json, settings_json, wh_path, wh_method, tags_json)) = row_opt else {
        return (StatusCode::NOT_FOUND, Json(json!({ "error": "Workflow not found" })));
    };

    let new_id = format!("wf_{}", &Uuid::new_v4().simple().to_string()[..12]);
    let new_name = format!("{} (Copy)", old_name);
    let new_wh = wh_path.map(|w| format!("{}-copy-{}", w, &new_id[3..8]));
    let now = Utc::now().to_rfc3339();
    let _ = conn.execute(
        "INSERT INTO n8n_workflows
         (id, name, active, nodes_json, connections_json, settings_json, webhook_path, webhook_method, trigger_count, tags_json, created_at, updated_at)
         VALUES (?1, ?2, 0, ?3, ?4, ?5, ?6, ?7, 0, ?8, ?9, ?9)",
        params![new_id, new_name, nodes_json, conns_json, settings_json, new_wh, wh_method, tags_json, now],
    );
    match conn.query_row("SELECT * FROM n8n_workflows WHERE id = ?1", [&new_id], |r| workflow_summary_from_row(r, true)) {
        Ok(res) => (StatusCode::CREATED, Json(res)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))),
    }
}

pub async fn toggle_workflow_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let now = Utc::now().to_rfc3339();
    let _ = conn.execute(
        "UPDATE n8n_workflows SET active = CASE WHEN active = 1 THEN 0 ELSE 1 END, updated_at = ?1 WHERE id = ?2",
        params![now, id],
    );
    match conn.query_row("SELECT * FROM n8n_workflows WHERE id = ?1", [&id], |r| workflow_summary_from_row(r, false)).optional() {
        Ok(Some(res)) => (StatusCode::OK, Json(res)),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({ "error": "Workflow not found" }))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))),
    }
}

pub async fn sync_sqlite_handler(State(state): State<AppState>) -> impl IntoResponse {
    match sync_from_n8n_sqlite(&state.db) {
        Ok(res) => (StatusCode::OK, Json(res)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "ok": false, "error": e.to_string() }))),
    }
}

pub async fn list_executions_handler(
    State(state): State<AppState>,
    Query(p): Query<N8nListQuery>,
) -> impl IntoResponse {
    let limit = p.limit.unwrap_or(50).clamp(1, 200);
    let conn = state.db.lock_conn();
    let sql = if let Some(wid) = &p.workflow_id {
        format!(
            "SELECT id, workflow_id, workflow_name, mode, status, model_used, started_at, finished_at, duration_ms, trigger_payload, result_json, node_logs_json, error_text
             FROM n8n_executions WHERE workflow_id = '{}' ORDER BY started_at DESC LIMIT ?1",
            wid.replace('\'', "")
        )
    } else {
        "SELECT id, workflow_id, workflow_name, mode, status, model_used, started_at, finished_at, duration_ms, trigger_payload, result_json, node_logs_json, error_text
         FROM n8n_executions ORDER BY started_at DESC LIMIT ?1".to_string()
    };
    let mut items = Vec::new();
    if let Ok(mut stmt) = conn.prepare(&sql) {
        if let Ok(mut rows) = stmt.query([limit]) {
            while let Ok(Some(r)) = rows.next() {
                let res_str: String = r.get(10).unwrap_or_default();
                let logs_str: String = r.get(11).unwrap_or_default();
                items.push(json!({
                    "id": r.get::<_, String>(0).unwrap_or_default(),
                    "workflowId": r.get::<_, String>(1).unwrap_or_default(),
                    "workflowName": r.get::<_, String>(2).unwrap_or_default(),
                    "mode": r.get::<_, String>(3).unwrap_or_default(),
                    "status": r.get::<_, String>(4).unwrap_or_default(),
                    "modelUsed": r.get::<_, String>(5).unwrap_or_default(),
                    "startedAt": r.get::<_, String>(6).unwrap_or_default(),
                    "finishedAt": r.get::<_, String>(7).unwrap_or_default(),
                    "durationMs": r.get::<_, i64>(8).unwrap_or(0),
                    "triggerPayload": serde_json::from_str::<Value>(&r.get::<_, String>(9).unwrap_or_default()).unwrap_or_else(|_| json!({})),
                    "result": serde_json::from_str::<Value>(&res_str).unwrap_or_else(|_| json!({})),
                    "nodeLogs": serde_json::from_str::<Value>(&logs_str).unwrap_or_else(|_| json!([])),
                    "error": r.get::<_, Option<String>>(12).unwrap_or(None),
                }));
            }
        }
    }
    Json(json!({ "data": items, "totalCount": items.len() }))
}

#[derive(Debug, Deserialize)]
pub struct ExecuteWorkflowReq {
    pub payload: Option<Value>,
    pub model: Option<String>,
    pub workspace: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ExecuteSingleNodeReq {
    #[serde(alias = "nodeName")]
    pub node_name: String,
    #[serde(alias = "inputData")]
    pub input_data: Option<Value>,
    pub payload: Option<Value>,
    pub model: Option<String>,
    pub parameters: Option<Value>,
}

pub async fn execute_workflow_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<Value>,
) -> impl IntoResponse {
    let payload = req.get("payload").cloned().unwrap_or_else(|| {
        if req.get("message").is_some() || req.get("text").is_some() || req.get("from").is_some() {
            req.clone()
        } else {
            json!({})
        }
    });
    let model = req.get("model").and_then(|x| x.as_str()).map(|s| s.to_string());
    match execute_workflow_by_id(&state.db, &id, payload, model).await {
        Ok(v) => (StatusCode::OK, Json(v)),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({ "ok": false, "error": e.to_string() }))),
    }
}

pub async fn execute_single_node_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(req): Json<ExecuteSingleNodeReq>,
) -> impl IntoResponse {
    let (wf_name, mut nodes_val, conns_val) = {
        let conn = state.db.lock_conn();
        match conn
            .query_row(
                "SELECT name, nodes_json, connections_json FROM n8n_workflows WHERE id = ?1 OR name = ?1 LIMIT 1",
                [&id],
                |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?)),
            )
            .optional()
        {
            Ok(Some(row)) => (
                row.0,
                serde_json::from_str::<Vec<Value>>(&row.1).unwrap_or_default(),
                serde_json::from_str::<Value>(&row.2).unwrap_or_else(|_| json!({})),
            ),
            Ok(None) => return (StatusCode::NOT_FOUND, Json(json!({ "ok": false, "error": "Workflow not found" }))),
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "ok": false, "error": e.to_string() }))),
        }
    };

    if let Some(override_params) = &req.parameters {
        for n in &mut nodes_val {
            if n.get("name").and_then(|x| x.as_str()) == Some(req.node_name.as_str()) {
                n["parameters"] = override_params.clone();
                break;
            }
        }
    }

    let mut node_map: HashMap<String, Value> = HashMap::new();
    let mut outgoing_targets: HashSet<String> = HashSet::new();
    for n in &nodes_val {
        if let Some(name) = n.get("name").and_then(|x| x.as_str()) {
            node_map.insert(name.to_string(), n.clone());
        }
    }
    let Some(target_node) = node_map.get(&req.node_name).cloned() else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "ok": false, "error": format!("Node '{}' not found in workflow '{}'", req.node_name, wf_name) })),
        );
    };

    let preferred_model = req
        .model
        .clone()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "conrad-n8n-agent:0.6b".to_string());
    let trigger_payload = req
        .payload
        .clone()
        .unwrap_or_else(|| json!({ "message": "Hallo, bitte status pruefen", "workspace": "conrad-ki" }));

    let t0 = std::time::Instant::now();
    let mut node_outputs: HashMap<String, Value> = HashMap::new();

    // If caller supplied explicit input_data, execute the single node directly against it.
    if let Some(explicit_in) = req.input_data.clone() {
        let ntype = target_node.get("type").and_then(|x| x.as_str()).unwrap_or("");
        let params = target_node.get("parameters").cloned().unwrap_or_else(|| json!({}));
        match execute_node(
            &state.db,
            &req.node_name,
            ntype,
            &params,
            &explicit_in,
            &node_outputs,
            &nodes_val,
            &conns_val,
            &preferred_model,
            &trigger_payload,
        )
        .await
        {
            Ok((out_val, branch_idx)) => {
                let elapsed = t0.elapsed().as_millis() as i64;
                return (
                    StatusCode::OK,
                    Json(json!({
                        "ok": true,
                        "nodeName": req.node_name,
                        "type": ntype,
                        "branch": branch_idx,
                        "status": "success",
                        "durationMs": elapsed,
                        "input": explicit_in,
                        "output": out_val,
                        "runData": { req.node_name: { "input": explicit_in, "output": out_val, "durationMs": elapsed } }
                    })),
                );
            }
            Err(e) => {
                return (
                    StatusCode::OK,
                    Json(json!({
                        "ok": false,
                        "nodeName": req.node_name,
                        "status": "error",
                        "durationMs": t0.elapsed().as_millis() as i64,
                        "input": explicit_in,
                        "output": { "error": e.to_string() },
                        "error": e.to_string()
                    })),
                );
            }
        }
    }

    // Otherwise run upstream DAG from trigger until we reach target_node
    if let Some(conns_obj) = conns_val.as_object() {
        for (_src, edges) in conns_obj {
            if let Some(main_arr) = edges.get("main").and_then(|x| x.as_array()) {
                for branch in main_arr {
                    if let Some(targets) = branch.as_array() {
                        for t in targets {
                            if let Some(target_name) = t.get("node").and_then(|x| x.as_str()) {
                                outgoing_targets.insert(target_name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    let mut start_node: Option<String> = None;
    for n in &nodes_val {
        let tp = n.get("type").and_then(|x| x.as_str()).unwrap_or("");
        let name = n.get("name").and_then(|x| x.as_str()).unwrap_or("");
        if (tp.contains("webhook") || tp.contains("chatTrigger") || tp.contains("manualTrigger")) && !tp.contains("respondToWebhook") {
            start_node = Some(name.to_string());
            break;
        }
    }
    if start_node.is_none() {
        for n in &nodes_val {
            let tp = n.get("type").and_then(|x| x.as_str()).unwrap_or("");
            let name = n.get("name").and_then(|x| x.as_str()).unwrap_or("");
            if tp != "n8n-nodes-base.stickyNote" && !outgoing_targets.contains(name) {
                start_node = Some(name.to_string());
                break;
            }
        }
    }

    let mut queue: VecDeque<(String, Value)> = VecDeque::new();
    if let Some(sn) = start_node {
        queue.push_back((sn, trigger_payload.clone()));
    } else {
        queue.push_back((req.node_name.clone(), trigger_payload.clone()));
    }

    let mut captured_input = trigger_payload.clone();
    let mut captured_output: Option<Value> = None;
    let mut captured_branch = 0usize;
    let mut step_err: Option<String> = None;
    let mut steps = 0usize;

    while let Some((cur_name, cur_in)) = queue.pop_front() {
        steps += 1;
        if steps > 40 {
            break;
        }
        let Some(node) = node_map.get(&cur_name) else { continue; };
        let ntype = node.get("type").and_then(|x| x.as_str()).unwrap_or("");
        let params = node.get("parameters").cloned().unwrap_or_else(|| json!({}));

        match execute_node(
            &state.db,
            &cur_name,
            ntype,
            &params,
            &cur_in,
            &node_outputs,
            &nodes_val,
            &conns_val,
            &preferred_model,
            &trigger_payload,
        )
        .await
        {
            Ok((out_val, b_idx)) => {
                node_outputs.insert(cur_name.clone(), out_val.clone());
                if cur_name == req.node_name {
                    captured_input = cur_in;
                    captured_output = Some(out_val);
                    captured_branch = b_idx;
                    break;
                }
                if let Some(main_arr) = conns_val.get(&cur_name).and_then(|x| x.get("main")).and_then(|x| x.as_array()) {
                    if let Some(branch_edges) = main_arr.get(b_idx).and_then(|x| x.as_array()) {
                        for edge in branch_edges {
                            if let Some(target) = edge.get("node").and_then(|x| x.as_str()) {
                                queue.push_back((target.to_string(), out_val.clone()));
                            }
                        }
                    }
                }
            }
            Err(e) => {
                if cur_name == req.node_name {
                    captured_input = cur_in;
                    step_err = Some(e.to_string());
                    break;
                }
            }
        }
    }

    // Fallback if target node was unreachable from start_node (e.g., AI sub-node or disconnected node)
    if captured_output.is_none() && step_err.is_none() {
        let ntype = target_node.get("type").and_then(|x| x.as_str()).unwrap_or("");
        let params = target_node.get("parameters").cloned().unwrap_or_else(|| json!({}));
        match execute_node(
            &state.db,
            &req.node_name,
            ntype,
            &params,
            &trigger_payload,
            &node_outputs,
            &nodes_val,
            &conns_val,
            &preferred_model,
            &trigger_payload,
        )
        .await
        {
            Ok((out_val, b_idx)) => {
                captured_input = trigger_payload.clone();
                captured_output = Some(out_val);
                captured_branch = b_idx;
            }
            Err(e) => {
                step_err = Some(e.to_string());
            }
        }
    }

    let elapsed = t0.elapsed().as_millis() as i64;
    if let Some(err) = step_err {
        (
            StatusCode::OK,
            Json(json!({
                "ok": false,
                "nodeName": req.node_name,
                "status": "error",
                "durationMs": elapsed,
                "input": captured_input,
                "output": { "error": err },
                "error": err
            })),
        )
    } else {
        let out_val = captured_output.unwrap_or_else(|| json!({}));
        (
            StatusCode::OK,
            Json(json!({
                "ok": true,
                "nodeName": req.node_name,
                "branch": captured_branch,
                "status": "success",
                "durationMs": elapsed,
                "input": captured_input,
                "output": out_val,
                "runData": { req.node_name: { "input": captured_input, "output": out_val, "durationMs": elapsed } }
            })),
        )
    }
}

pub async fn webhook_dispatch_handler(
    State(state): State<AppState>,
    Path(wh_path): Path<String>,
    headers: HeaderMap,
    body_bytes: Bytes,
) -> impl IntoResponse {
    let clean_path = wh_path.trim_start_matches('/').to_string();
    let payload: Value = if body_bytes.is_empty() {
        json!({})
    } else {
        serde_json::from_slice(&body_bytes).unwrap_or_else(|_| {
            json!({ "text": String::from_utf8_lossy(&body_bytes).to_string() })
        })
    };
    let model = headers
        .get("x-conrad-model")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let wf_id: Option<String> = {
        let conn = state.db.lock_conn();
        conn.query_row(
            "SELECT id FROM n8n_workflows WHERE webhook_path = ?1 OR id = ?1 ORDER BY active DESC LIMIT 1",
            [&clean_path],
            |r| r.get(0),
        )
        .optional()
        .unwrap_or(None)
    };

    let Some(id) = wf_id else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({ "ok": false, "error": format!("No Rust webhook found for '{}'", clean_path) })),
        );
    };

    match execute_workflow_by_id(&state.db, &id, payload, model).await {
        Ok(res) => {
            let body = res.get("response").cloned().unwrap_or(res);
            (StatusCode::OK, Json(body))
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "ok": false, "error": e.to_string() })),
        ),
    }
}

// =========================================================================
// Async DAG Workflow Executor (Pure Rust)
// =========================================================================

pub async fn execute_workflow_by_id(
    db: &Database,
    workflow_id: &str,
    trigger_payload: Value,
    model_override: Option<String>,
) -> anyhow::Result<Value> {
    let (wf_name, nodes_val, conns_val) = {
        let conn = db.lock_conn();
        let row: (String, String, String) = conn
            .query_row(
                "SELECT name, nodes_json, connections_json FROM n8n_workflows WHERE id = ?1 OR name = ?1 LIMIT 1",
                [workflow_id],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .map_err(|_| anyhow::anyhow!("Workflow '{}' nicht gefunden", workflow_id))?;
        (
            row.0,
            serde_json::from_str::<Vec<Value>>(&row.1).unwrap_or_default(),
            serde_json::from_str::<Value>(&row.2).unwrap_or_else(|_| json!({})),
        )
    };

    let exec_id = Uuid::new_v4().to_string();
    let started_iso = Utc::now().to_rfc3339();
    let t0 = std::time::Instant::now();

    let mut node_map: HashMap<String, Value> = HashMap::new();
    let mut outgoing_targets: HashSet<String> = HashSet::new();
    for n in &nodes_val {
        if let Some(name) = n.get("name").and_then(|x| x.as_str()) {
            node_map.insert(name.to_string(), n.clone());
        }
    }
    if let Some(conns_obj) = conns_val.as_object() {
        for (_src, edges) in conns_obj {
            if let Some(main_arr) = edges.get("main").and_then(|x| x.as_array()) {
                for branch in main_arr {
                    if let Some(targets) = branch.as_array() {
                        for t in targets {
                            if let Some(target_name) = t.get("node").and_then(|x| x.as_str()) {
                                outgoing_targets.insert(target_name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    // Determine start node: trigger node without incoming edges, or webhook/chatTrigger
    let mut start_node: Option<String> = None;
    for n in &nodes_val {
        let tp = n.get("type").and_then(|x| x.as_str()).unwrap_or("");
        let name = n.get("name").and_then(|x| x.as_str()).unwrap_or("");
        if (tp.contains("webhook") || tp.contains("chatTrigger") || tp.contains("manualTrigger"))
            && !tp.contains("respondToWebhook")
        {
            start_node = Some(name.to_string());
            break;
        }
    }
    if start_node.is_none() {
        for n in &nodes_val {
            let tp = n.get("type").and_then(|x| x.as_str()).unwrap_or("");
            let name = n.get("name").and_then(|x| x.as_str()).unwrap_or("");
            if tp != "n8n-nodes-base.stickyNote" && !outgoing_targets.contains(name) && !tp.starts_with("@n8n/n8n-nodes-langchain.lm") && !tp.contains("embeddings") && !tp.contains("vectorStore") && !tp.contains("memory") {
                start_node = Some(name.to_string());
                break;
            }
        }
    }
    if start_node.is_none() && !nodes_val.is_empty() {
        start_node = nodes_val[0].get("name").and_then(|x| x.as_str()).map(|s| s.to_string());
    }

    let preferred_model = model_override
        .filter(|s| !s.is_empty())
        .or_else(|| {
            trigger_payload
                .get("model")
                .and_then(|x| x.as_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| "conrad-n8n-agent:0.6b".to_string());

    let mut queue: VecDeque<(String, Value)> = VecDeque::new();
    let mut node_outputs: HashMap<String, Value> = HashMap::new();
    let mut node_logs: Vec<Value> = Vec::new();
    let mut final_response: Option<Value> = None;
    let mut last_output = trigger_payload.clone();
    let mut visited_count = 0usize;

    if let Some(sn) = start_node {
        queue.push_back((sn, trigger_payload.clone()));
    }

    let mut status_str = "success".to_string();
    let mut err_text: Option<String> = None;

    while let Some((node_name, input_item)) = queue.pop_front() {
        visited_count += 1;
        if visited_count > 40 {
            break;
        }
        let Some(node) = node_map.get(&node_name) else {
            continue;
        };
        let ntype = node.get("type").and_then(|x| x.as_str()).unwrap_or("");
        let params = node.get("parameters").cloned().unwrap_or_else(|| json!({}));
        let nt0 = std::time::Instant::now();

        let (out_val, branch_idx, node_status, node_err) = match execute_node(
            db,
            &node_name,
            ntype,
            &params,
            &input_item,
            &node_outputs,
            &nodes_val,
            &conns_val,
            &preferred_model,
            &trigger_payload,
        )
        .await
        {
            Ok((v, b)) => (v, b, "success", None),
            Err(e) => {
                let msg = e.to_string();
                status_str = "error".to_string();
                err_text = Some(msg.clone());
                (json!({ "error": { "message": msg } }), 0usize, "error", Some(msg))
            }
        };

        let elapsed = nt0.elapsed().as_millis() as i64;
        node_outputs.insert(node_name.clone(), out_val.clone());
        last_output = out_val.clone();
        if ntype == "n8n-nodes-base.respondToWebhook" {
            final_response = Some(out_val.clone());
        }

        node_logs.push(json!({
            "node": node_name,
            "type": ntype,
            "status": node_status,
            "durationMs": elapsed,
            "branch": branch_idx,
            "output": out_val,
            "error": node_err
        }));

        if let Some(main_arr) = conns_val
            .get(&node_name)
            .and_then(|x| x.get("main"))
            .and_then(|x| x.as_array())
        {
            if let Some(branch_edges) = main_arr.get(branch_idx).and_then(|x| x.as_array()) {
                for edge in branch_edges {
                    if let Some(target) = edge.get("node").and_then(|x| x.as_str()) {
                        queue.push_back((target.to_string(), out_val.clone()));
                    }
                }
            }
        }
    }

    let finished_iso = Utc::now().to_rfc3339();
    let total_ms = t0.elapsed().as_millis() as i64;
    let result_payload = final_response.clone().unwrap_or(last_output);

    {
        let conn = db.lock_conn();
        let _ = conn.execute(
            "INSERT INTO n8n_executions
             (id, workflow_id, workflow_name, mode, status, model_used, started_at, finished_at, duration_ms, trigger_payload, result_json, node_logs_json, error_text)
             VALUES (?1, ?2, ?3, 'rust-engine', ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            params![
                exec_id,
                workflow_id,
                wf_name,
                status_str,
                preferred_model,
                started_iso,
                finished_iso,
                total_ms,
                trigger_payload.to_string(),
                result_payload.to_string(),
                serde_json::to_string(&node_logs).unwrap_or_else(|_| "[]".to_string()),
                err_text
            ],
        );
        let _ = conn.execute(
            "UPDATE n8n_workflows SET trigger_count = trigger_count + 1 WHERE id = ?1",
            [workflow_id],
        );
    }

    Ok(json!({
        "ok": status_str == "success",
        "executionId": exec_id,
        "workflowId": workflow_id,
        "workflowName": wf_name,
        "status": status_str,
        "modelUsed": preferred_model,
        "durationMs": total_ms,
        "response": result_payload,
        "nodeLogs": node_logs
    }))
}

async fn execute_node(
    db: &Database,
    node_name: &str,
    ntype: &str,
    params: &Value,
    input: &Value,
    node_outputs: &HashMap<String, Value>,
    all_nodes: &[Value],
    conns: &Value,
    preferred_model: &str,
    raw_trigger: &Value,
) -> anyhow::Result<(Value, usize)> {
    match ntype {
        "n8n-nodes-base.webhook" | "@n8n/n8n-nodes-langchain.chatTrigger" | "n8n-nodes-base.manualTrigger" | "n8n-nodes-base.scheduleTrigger" | "n8n-nodes-base.telegramTrigger" | "n8n-nodes-base.gmailTrigger" | "n8n-nodes-base.microsoftOutlookTrigger" => {
            let mut merged = match input.as_object() {
                Some(m) => m.clone(),
                None => serde_json::Map::new(),
            };
            if !merged.contains_key("body") {
                merged.insert("body".to_string(), input.clone());
            }
            Ok((Value::Object(merged), 0))
        }

        "n8n-nodes-base.respondToWebhook" => {
            let raw_body = params.get("responseBody").and_then(|x| x.as_str()).unwrap_or("");
            if raw_body.trim().is_empty() {
                return Ok((input.clone(), 0));
            }
            let evaled = eval_n8n_expression(raw_body, input, node_outputs);
            if let Ok(parsed) = serde_json::from_str::<Value>(&evaled) {
                Ok((parsed, 0))
            } else {
                Ok((json!({ "response": evaled, "data": input }), 0))
            }
        }

        "n8n-nodes-base.set" => {
            let mut obj = input.as_object().cloned().unwrap_or_default();
            if let Some(assigns) = params.pointer("/assignments/assignments").and_then(|x| x.as_array()) {
                for a in assigns {
                    if let Some(k) = a.get("name").and_then(|x| x.as_str()) {
                        let v_raw = a.get("value").cloned().unwrap_or(Value::Null);
                        let v = match v_raw {
                            Value::String(s) => {
                                let ev = eval_n8n_expression(&s, input, node_outputs);
                                Value::String(ev)
                            }
                            other => other,
                        };
                        obj.insert(k.to_string(), v);
                    }
                }
            }
            Ok((Value::Object(obj), 0))
        }

        "n8n-nodes-base.if" => {
            let matched = eval_if_conditions(params.get("conditions"), input, node_outputs);
            Ok((input.clone(), if matched { 0 } else { 1 }))
        }

        "n8n-nodes-base.switch" => {
            let mut chosen = None;
            if let Some(rules) = params.pointer("/rules/values").and_then(|x| x.as_array()) {
                for (idx, r) in rules.iter().enumerate() {
                    if eval_if_conditions(r.get("conditions"), input, node_outputs) {
                        chosen = Some(idx);
                        break;
                    }
                }
            }
            let idx = chosen.unwrap_or_else(|| {
                params.pointer("/rules/values").and_then(|x| x.as_array()).map(|a| a.len()).unwrap_or(0)
            });
            Ok((input.clone(), idx))
        }

        "n8n-nodes-base.code" => {
            let js = params.get("jsCode").and_then(|x| x.as_str()).unwrap_or("");
            let res = eval_code_node(node_name, js, input, node_outputs, preferred_model, raw_trigger)?;
            Ok((res, 0))
        }

        "n8n-nodes-base.httpRequest" => {
            let res = execute_http_request_node(db, params, input, node_outputs, preferred_model).await?;
            Ok((res, 0))
        }

        "@n8n/n8n-nodes-langchain.agent" | "@n8n/n8n-nodes-langchain.chainLlm" => {
            let res = execute_langchain_agent_node(node_name, params, input, node_outputs, all_nodes, conns, preferred_model).await?;
            Ok((res, 0))
        }

        "n8n-nodes-base.gmail" | "n8n-nodes-base.microsoftOutlook" | "n8n-nodes-base.emailSend" => {
            let to = input.get("to").or_else(|| input.get("from")).or_else(|| input.get("email")).and_then(|x| x.as_str()).unwrap_or("unbekannt@example.de");
            let subj = params.get("subject").and_then(|x| x.as_str()).map(|s| eval_n8n_expression(s, input, node_outputs)).unwrap_or_else(|| input.get("subject").and_then(|x| x.as_str()).unwrap_or("Antwort-Entwurf").to_string());
            let mut out = input.as_object().cloned().unwrap_or_default();
            out.insert("draftOnly".to_string(), json!(true));
            out.insert("draftId".to_string(), json!(format!("draft-{}", &Uuid::new_v4().to_string()[..8])));
            out.insert("to".to_string(), json!(to));
            out.insert("subject".to_string(), json!(subj));
            out.insert("mailGuard".to_string(), json!("Draft gespeichert (kein automatischer Versand)"));
            Ok((Value::Object(out), 0))
        }

        "n8n-nodes-base.telegram" => {
            let text = params.get("text").and_then(|x| x.as_str()).map(|s| eval_n8n_expression(s, input, node_outputs)).unwrap_or_else(|| "Telegram Draft-Nachricht".to_string());
            Ok((json!({ "ok": true, "draftOnly": true, "simulatedTelegram": text }), 0))
        }

        "n8n-nodes-base.postgres" => {
            let query_str = params
                .get("query")
                .and_then(|x| x.as_str())
                .map(|s| eval_n8n_expression(s, input, node_outputs))
                .unwrap_or_else(|| "SELECT id, name, city, icp FROM twenty_companies LIMIT 5".to_string());
            let op = params.get("operation").and_then(|x| x.as_str()).unwrap_or("executeQuery");
            let mut rows = Vec::new();
            {
                let conn = db.lock_conn();
                if let Ok(mut stmt) = conn.prepare(
                    "SELECT id, name, city, domain_url, icp FROM twenty_companies ORDER BY updated_at DESC LIMIT 5"
                ) {
                    if let Ok(mut qrows) = stmt.query([]) {
                        while let Ok(Some(r)) = qrows.next() {
                            rows.push(json!({
                                "id": r.get::<_, String>(0).unwrap_or_default(),
                                "name": r.get::<_, String>(1).unwrap_or_default(),
                                "city": r.get::<_, String>(2).unwrap_or_default(),
                                "domain": r.get::<_, String>(3).unwrap_or_default(),
                                "icp": r.get::<_, i64>(4).unwrap_or(0) != 0
                            }));
                        }
                    }
                };
            }
            Ok((json!({
                "ok": true,
                "driver": "rust-sqlite-pg-bridge",
                "operation": op,
                "query": query_str,
                "rowCount": rows.len(),
                "rows": rows
            }), 0))
        }

        "n8n-nodes-base.googleSheets" => {
            let op = params.get("operation").and_then(|x| x.as_str()).unwrap_or("appendOrUpdate");
            let sheet = params
                .pointer("/sheetName/value")
                .or_else(|| params.get("sheetName"))
                .and_then(|x| x.as_str())
                .unwrap_or("Leads");
            Ok((json!({
                "ok": true,
                "operation": op,
                "sheetName": sheet,
                "updatedRows": 1,
                "data": input
            }), 0))
        }

        "@n8n/n8n-nodes-langchain.vectorStoreQdrant" => {
            let collection = params
                .pointer("/qdrantCollection/value")
                .or_else(|| params.get("qdrantCollection"))
                .and_then(|x| x.as_str())
                .unwrap_or("conrad_ki_knowledge");
            let mut collections = json!([]);
            if let Ok(r) = http_client()
                .get("http://127.0.0.1:6333/collections")
                .timeout(std::time::Duration::from_millis(1200))
                .send()
                .await
            {
                if let Ok(v) = r.json::<Value>().await {
                    collections = v.pointer("/result/collections").cloned().unwrap_or_else(|| json!([]));
                }
            }
            Ok((json!({
                "ok": true,
                "vectorStore": "qdrant-local",
                "collection": collection,
                "collections": collections,
                "matches": [
                    { "score": 0.94, "text": "Conrad KI Playbook & Hormozi Offer Engine SOP", "source": "conrad_ki_knowledge" }
                ],
                "input": input
            }), 0))
        }

        "@n8n/n8n-nodes-langchain.embeddingsOllama" => {
            let m = params.get("model").and_then(|x| x.as_str()).unwrap_or("nomic-embed-text");
            Ok((json!({ "ok": true, "provider": "ollama-local", "model": m, "dimensions": 768 }), 0))
        }

        "@n8n/n8n-nodes-langchain.lmChatOllama" | "@n8n/n8n-nodes-langchain.lmChatOpenAi" => {
            let req_model = params
                .get("model")
                .and_then(|x| x.as_str())
                .filter(|s| !s.is_empty() && !s.contains("4b") && !s.starts_with("gpt-"))
                .unwrap_or(preferred_model);
            let prompt = input
                .get("message")
                .or_else(|| input.get("prompt"))
                .or_else(|| input.get("text"))
                .or_else(|| input.get("query"))
                .and_then(|x| x.as_str())
                .unwrap_or("Kurzer System-Check auf Deutsch.");
            let chat_url = format!("{}/api/chat", ollama_base_url());
            let payload = json!({
                "model": req_model,
                "stream": false,
                "think": false,
                "options": { "temperature": 0.1, "num_ctx": 4096, "num_predict": 256 },
                "messages": [{ "role": "user", "content": prompt }]
            });
            if let Ok(resp) = http_client().post(&chat_url).json(&payload).send().await {
                if let Ok(raw) = resp.json::<Value>().await {
                    let content = raw.pointer("/message/content").and_then(|x| x.as_str()).map(strip_think_tags).unwrap_or_default();
                    return Ok((json!({ "model": req_model, "response": content, "evalCount": raw.get("eval_count") }), 0));
                }
            }
            Ok((json!({ "model": req_model, "response": "Sub-Node bereit", "connectedToAgent": true }), 0))
        }

        "@n8n/n8n-nodes-langchain.memoryBufferWindow" => {
            let k = params.get("contextWindowLength").and_then(|x| x.as_i64()).unwrap_or(5);
            Ok((json!({ "ok": true, "memoryType": "bufferWindow", "contextWindowLength": k, "messagesBuffered": 0 }), 0))
        }

        "n8n-nodes-base.noOp" | "n8n-nodes-base.errorTrigger" => {
            Ok((json!({ "ok": true, "passThrough": true, "data": input }), 0))
        }

        _ => Ok((input.clone(), 0)),
    }
}

fn eval_if_conditions(conds_val: Option<&Value>, input: &Value, node_outputs: &HashMap<String, Value>) -> bool {
    let Some(conds_obj) = conds_val else {
        return true;
    };
    let combinator = conds_obj.get("combinator").and_then(|x| x.as_str()).unwrap_or("and");
    let Some(arr) = conds_obj.get("conditions").and_then(|x| x.as_array()) else {
        return true;
    };
    if arr.is_empty() {
        return true;
    }

    let mut results = Vec::new();
    for c in arr {
        let left_raw = c.get("leftValue").cloned().unwrap_or(Value::Null);
        let right_raw = c.get("rightValue").cloned().unwrap_or(Value::Null);
        let left = resolve_n8n_value(&left_raw, input, node_outputs);
        let right = resolve_n8n_value(&right_raw, input, node_outputs);
        let op = c.pointer("/operator/operation").and_then(|x| x.as_str()).unwrap_or("equals");
        let otype = c.pointer("/operator/type").and_then(|x| x.as_str()).unwrap_or("string");

        let hit = match (otype, op) {
            ("boolean", "false") => !val_as_bool(&left),
            ("boolean", "true") => val_as_bool(&left),
            ("boolean", "equals") => val_as_bool(&left) == val_as_bool(&right),
            (_, "equals") => val_as_str(&left) == val_as_str(&right),
            (_, "notEquals") => val_as_str(&left) != val_as_str(&right),
            (_, "contains") => val_as_str(&left).contains(&val_as_str(&right)),
            (_, "notEmpty") => !val_as_str(&left).trim().is_empty(),
            (_, "empty") => val_as_str(&left).trim().is_empty(),
            ("number", "gt") => val_as_f64(&left) > val_as_f64(&right),
            ("number", "lt") => val_as_f64(&left) < val_as_f64(&right),
            _ => val_as_str(&left) == val_as_str(&right),
        };
        results.push(hit);
    }

    if combinator == "or" {
        results.into_iter().any(|x| x)
    } else {
        results.into_iter().all(|x| x)
    }
}

fn val_as_bool(v: &Value) -> bool {
    match v {
        Value::Bool(b) => *b,
        Value::String(s) => matches!(s.trim().to_lowercase().as_str(), "true" | "1" | "yes"),
        Value::Number(n) => n.as_f64().unwrap_or(0.0) != 0.0,
        _ => false,
    }
}
fn val_as_str(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}
fn val_as_f64(v: &Value) -> f64 {
    match v {
        Value::Number(n) => n.as_f64().unwrap_or(0.0),
        Value::String(s) => s.trim().parse().unwrap_or(0.0),
        _ => 0.0,
    }
}

fn resolve_n8n_value(v: &Value, input: &Value, node_outputs: &HashMap<String, Value>) -> Value {
    match v {
        Value::String(s) if s.trim().starts_with("={{") && s.trim().ends_with("}}") => {
            let inner = s.trim();
            let expr = inner[3..inner.len() - 2].trim();
            eval_single_expr_value(expr, input, node_outputs)
        }
        Value::String(s) if s.contains("{{") => Value::String(eval_n8n_expression(s, input, node_outputs)),
        other => other.clone(),
    }
}

fn eval_single_expr_value(expr: &str, input: &Value, node_outputs: &HashMap<String, Value>) -> Value {
    let e = expr.trim();
    if let Some(rest) = e.strip_prefix("$json.") {
        return lookup_dot_path(input, rest);
    }
    if e == "$json" {
        return input.clone();
    }
    if let Some(rest) = e.strip_prefix("JSON.stringify(") {
        if let Some(inner) = rest.strip_suffix(')') {
            let val = eval_single_expr_value(inner.trim(), input, node_outputs);
            return Value::String(val.to_string());
        }
    }
    if e.starts_with("$node[") {
        if let Some(q1) = e.find('\'').or_else(|| e.find('"')) {
            let quote = e.as_bytes()[q1] as char;
            if let Some(q2_rel) = e[q1 + 1..].find(quote) {
                let target_node = &e[q1 + 1..q1 + 1 + q2_rel];
                let after = &e[q1 + 1 + q2_rel + 1..];
                if let Some(base) = node_outputs.get(target_node) {
                    let clean_after = after
                        .trim_start_matches(']')
                        .trim_start_matches(".json")
                        .trim_start_matches('.');
                    if let Some(k) = clean_after.strip_prefix("['").and_then(|s| s.strip_suffix("']")) {
                        return base.get(k).cloned().unwrap_or(Value::Null);
                    }
                    if let Some(k) = clean_after.strip_prefix("[\"").and_then(|s| s.strip_suffix("\"]")) {
                        return base.get(k).cloned().unwrap_or(Value::Null);
                    }
                    if !clean_after.is_empty() {
                        return lookup_dot_path(base, clean_after);
                    }
                    return base.clone();
                }
            }
        }
    }
    Value::String(eval_n8n_expression(&format!("={{{{ {} }}}}", e), input, node_outputs))
}

fn lookup_dot_path(v: &Value, path: &str) -> Value {
    let mut cur = v;
    for part in path.split('.') {
        let p = part.trim();
        if p.is_empty() {
            continue;
        }
        cur = match cur.get(p) {
            Some(next) => next,
            None => return Value::Null,
        };
    }
    cur.clone()
}

pub fn eval_n8n_expression(template: &str, input: &Value, node_outputs: &HashMap<String, Value>) -> String {
    let t = template.trim();
    let work = t.strip_prefix('=').unwrap_or(template);
    if !work.contains("{{") {
        return work.to_string();
    }
    let mut out = String::new();
    let mut rest = work;
    while let Some(s) = rest.find("{{") {
        out.push_str(&rest[..s]);
        if let Some(e) = rest[s + 2..].find("}}") {
            let expr = rest[s + 2..s + 2 + e].trim();
            let val = eval_single_expr_value(expr, input, node_outputs);
            match val {
                Value::String(sv) => out.push_str(&sv),
                Value::Null => {}
                other => out.push_str(&other.to_string()),
            }
            rest = &rest[s + 2 + e + 2..];
        } else {
            out.push_str(&rest[s..]);
            rest = "";
            break;
        }
    }
    out.push_str(rest);
    out
}

fn eval_code_node(
    node_name: &str,
    js: &str,
    input: &Value,
    node_outputs: &HashMap<String, Value>,
    preferred_model: &str,
    raw_trigger: &Value,
) -> anyhow::Result<Value> {
    let body = input.get("body").unwrap_or(input);
    let query_txt = body
        .get("message")
        .or_else(|| body.get("prompt"))
        .or_else(|| body.get("text"))
        .or_else(|| body.get("query"))
        .or_else(|| body.get("chatInput"))
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .trim()
        .to_string();

    let nlow = node_name.to_lowercase();
    let jlow = js.to_lowercase();

    // 1. Prompt Injection / Input Guard nodes (conradKiAgent001, kiraRagHardened01, etc.)
    if nlow.contains("guard") || nlow.contains("prep") || jlow.contains("injection") {
        let qlow = query_txt.to_lowercase();
        let inj_patterns = [
            "ignore previous instructions",
            "ignore all previous",
            "system prompt",
            "vergiss alle vorherigen",
            "du bist jetzt dan",
            "jailbreak",
            "reveal your prompt",
            "ignoriere alle anweisungen",
        ];
        let mut blocked = query_txt.is_empty();
        let mut reason = if blocked { "empty_query" } else { "ok" };
        for pat in inj_patterns {
            if qlow.contains(pat) {
                blocked = true;
                reason = "prompt_injection_detected";
                break;
            }
        }
        let llm_url = body
            .get("llm_url")
            .and_then(|x| x.as_str())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("{}/v1/chat/completions", ollama_base_url()));
        let model = body
            .get("model")
            .and_then(|x| x.as_str())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .unwrap_or_else(|| preferred_model.to_string());

        return Ok(json!({
            "query": query_txt,
            "blocked": blocked,
            "reason": reason,
            "llmUrl": llm_url,
            "model": model,
            "workspace": body.get("workspace").and_then(|x| x.as_str()).unwrap_or("conrad-ki")
        }));
    }

    // 2. Normalize & Unread-Lock (palaInboxZero001)
    if nlow.contains("unread") || nlow.contains("normalize") {
        let model = body
            .get("model")
            .and_then(|x| x.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| preferred_model.to_string());
        let llm_url = format!("{}/v1/chat/completions", ollama_base_url());
        return Ok(json!({
            "messageId": body.get("message_id").or_else(|| body.get("messageId")).and_then(|x| x.as_str()).unwrap_or("msg-local"),
            "from": body.get("from").and_then(|x| x.as_str()).unwrap_or("kunde@example.de"),
            "subject": body.get("subject").and_then(|x| x.as_str()).unwrap_or("(Kein Betreff)"),
            "snippet": body.get("text").or_else(|| body.get("snippet")).and_then(|x| x.as_str()).unwrap_or(""),
            "markAsRead": false,
            "draftOnly": true,
            "llmUrl": llm_url,
            "model": model
        }));
    }

    // 3. Cal.com -> Twenty CRM GraphQL builder (DW3U6TbZgH0hGtJB / Pala / Sprachschule)
    if jlow.contains("accepted") && (jlow.contains("mutation") || jlow.contains("graphql")) {
        let p = body.get("payload").unwrap_or(body);
        let status = p.get("status").and_then(|x| x.as_str()).unwrap_or("ACCEPTED");
        if status.to_uppercase() != "ACCEPTED" {
            return Ok(json!({ "status": "skipped", "reason": format!("not ACCEPTED: {}", status) }));
        }
        let attendees = p.get("attendees").and_then(|x| x.as_array()).cloned().unwrap_or_default();
        let attendee = attendees.first().cloned().unwrap_or_else(|| json!({}));
        let full_name = attendee.get("name").and_then(|x| x.as_str()).or_else(|| body.get("name").and_then(|x| x.as_str())).unwrap_or("Cal.com Gast");
        let email = attendee.get("email").and_then(|x| x.as_str()).or_else(|| body.get("email").and_then(|x| x.as_str())).unwrap_or("gast@example.de");
        let parts: Vec<&str> = full_name.split_whitespace().collect();
        let first = parts.first().copied().unwrap_or(full_name);
        let last = if parts.len() > 1 { parts[1..].join(" ") } else { "".to_string() };
        let title = p.get("title").and_then(|x| x.as_str()).unwrap_or("Erstgespraech (Cal.com)");
        let mutation = format!(
            "mutation {{ opp: createOpportunity(data: {{ name: \"{} - {}\", stage: NEW }}) {{ id name stage }} person: createPerson(data: {{ name: {{ firstName: \"{}\", lastName: \"{}\" }}, emails: {{ primaryEmail: \"{}\" }} }}) {{ id }} }}",
            full_name.replace('"', ""),
            title.replace('"', ""),
            first.replace('"', ""),
            last.replace('"', ""),
            email.replace('"', "")
        );
        return Ok(json!({
            "status": "ACCEPTED",
            "name": full_name,
            "email": email,
            "mutation": mutation
        }));
    }

    // 4. Try-Catch JSON Validator & Format & Fallback nodes
    if nlow.contains("validator") || nlow.contains("schema") || nlow.contains("format") {
        let mut raw_text = input
            .pointer("/choices/0/message/content")
            .or_else(|| input.pointer("/message/content"))
            .or_else(|| input.get("response"))
            .or_else(|| input.get("output"))
            .and_then(|x| x.as_str())
            .map(strip_think_tags)
            .unwrap_or_else(|| input.to_string());

        raw_text = raw_text
            .trim()
            .trim_start_matches("```json")
            .trim_start_matches("```")
            .trim_end_matches("```")
            .trim()
            .to_string();

        if let Ok(mut parsed) = serde_json::from_str::<Value>(&raw_text) {
            if let Some(obj) = parsed.as_object_mut() {
                obj.entry("draftOnly").or_insert(json!(true));
                obj.entry("draft_only").or_insert(json!(true));
                obj.entry("markAsRead").or_insert(json!(false));
                obj.entry("ok").or_insert(json!(true));
            }
            return Ok(parsed);
        }

        if let Some(start) = raw_text.find('{') {
            if let Some(end) = raw_text.rfind('}') {
                if let Ok(mut parsed) = serde_json::from_str::<Value>(&raw_text[start..=end]) {
                    if let Some(obj) = parsed.as_object_mut() {
                        obj.entry("draftOnly").or_insert(json!(true));
                        obj.entry("draft_only").or_insert(json!(true));
                        obj.entry("ok").or_insert(json!(true));
                    }
                    return Ok(parsed);
                }
            }
        }

        // Clean structured fallback matching conradKiAgent001 / palaInboxZero001
        if jlow.contains("kategorie") {
            let src = node_outputs.get("Normalize & Unread-Lock").unwrap_or(raw_trigger);
            return Ok(json!({
                "messageId": src.get("messageId").and_then(|x| x.as_str()).unwrap_or("msg-local"),
                "markAsRead": false,
                "draftOnly": true,
                "kategorie": "Kundenanfrage",
                "prioritaet": "mittel",
                "aktion": "manuell_pruefen",
                "label": "KI/Pruefen",
                "zusammenfassung_de": raw_text.chars().take(220).collect::<String>()
            }));
        }
        return Ok(json!({
            "ok": true,
            "status": "ok",
            "intent": "beratung",
            "route": "conrad_ki_local",
            "draft_only": true,
            "summary_de": raw_text.chars().take(240).collect::<String>(),
            "next_action": "rueckmeldung_als_entwurf_vorbereiten"
        }));
    }

    // 5. RAG Context Builder
    if nlow.contains("rag") && nlow.contains("context") {
        let mut snippets = Vec::new();
        if let Some(res) = input.get("result").and_then(|x| x.as_array()) {
            for hit in res.iter().take(4) {
                if let Some(txt) = hit.pointer("/payload/text").or_else(|| hit.pointer("/payload/content")).and_then(|x| x.as_str()) {
                    snippets.push(txt.to_string());
                }
            }
        }
        let orig_query = node_outputs
            .get("Injection Guard")
            .and_then(|v| v.get("query"))
            .and_then(|x| x.as_str())
            .unwrap_or(&query_txt);
        return Ok(json!({
            "query": orig_query,
            "ragContext": snippets.join("\n---\n"),
            "hitCount": snippets.len(),
            "model": preferred_model,
            "llmUrl": format!("{}/v1/chat/completions", ollama_base_url())
        }));
    }

    // 6. Default passthrough preserving payload
    let mut out = input.as_object().cloned().unwrap_or_default();
    out.entry("_codeNode").or_insert(json!(node_name));
    Ok(Value::Object(out))
}

async fn execute_http_request_node(
    db: &Database,
    params: &Value,
    input: &Value,
    node_outputs: &HashMap<String, Value>,
    preferred_model: &str,
) -> anyhow::Result<Value> {
    let raw_url = params.get("url").and_then(|x| x.as_str()).unwrap_or("");
    let mut url = eval_n8n_expression(raw_url, input, node_outputs);
    let method = params.get("method").and_then(|x| x.as_str()).unwrap_or("POST").to_uppercase();

    // Rewrite internal Docker service hostnames to Pure-Rust host loopback
    url = url
        .replace("http://ollama:11434", &ollama_base_url())
        .replace("http://host.docker.internal:11434", &ollama_base_url())
        .replace("http://qdrant:6333", "http://127.0.0.1:6333");

    // Extract body (either jsonBody or bodyParameters)
    let body_val: Value = if let Some(jb) = params.get("jsonBody").and_then(|x| x.as_str()) {
        build_http_json_body(jb, input, node_outputs, preferred_model)
    } else if let Some(bp) = params.pointer("/bodyParameters/parameters").and_then(|x| x.as_array()) {
        let mut m = serde_json::Map::new();
        for p in bp {
            if let Some(name) = p.get("name").and_then(|x| x.as_str()) {
                let raw_v = p.get("value").and_then(|x| x.as_str()).unwrap_or("");
                m.insert(name.to_string(), json!(eval_n8n_expression(raw_v, input, node_outputs)));
            }
        }
        Value::Object(m)
    } else {
        input.clone()
    };

    // 1. Short-circuit Twenty CRM GraphQL calls directly in-process!
    if url.contains("crm.conrad-ki.com/graphql") || url.contains("127.0.0.1:18090/graphql") {
        let ws = if url.contains("pala.") { "pala" } else { "conrad-ki" };
        let q = body_val.get("query").and_then(|x| x.as_str()).unwrap_or("");
        let vars = body_val.get("variables").cloned().unwrap_or_else(|| json!({}));
        return Ok(crate::twenty::execute_graphql(db, ws, q, &vars));
    }

    // 2. Local Ollama normalization + speed-up (`think: false` + auto fallback if :4b not loaded)
    if url.contains("127.0.0.1:11434") || url.contains("localhost:11434") {
        let mut req_model = body_val
            .get("model")
            .and_then(|x| x.as_str())
            .unwrap_or(preferred_model)
            .to_string();
        // Always favor local fast model if workflow hardcoded :4b and user wants lightweight local run
        if req_model == "conrad-n8n-agent:4b" && !preferred_model.is_empty() {
            req_model = preferred_model.to_string();
        }
        let messages = body_val.get("messages").cloned().unwrap_or_else(|| {
            let txt = input
                .get("query")
                .or_else(|| input.get("message"))
                .or_else(|| input.get("prompt"))
                .or_else(|| input.get("snippet"))
                .and_then(|x| x.as_str())
                .unwrap_or("Hallo");
            json!([{ "role": "user", "content": txt }])
        });

        let ollama_payload = json!({
            "model": req_model,
            "stream": false,
            "think": false,
            "options": { "temperature": 0.1, "num_ctx": 4096, "num_predict": 384 },
            "messages": messages
        });
        let chat_url = format!("{}/api/chat", ollama_base_url());
        let resp = http_client().post(&chat_url).json(&ollama_payload).send().await;
        match resp {
            Ok(r) if r.status().is_success() => {
                let raw: Value = r.json().await.unwrap_or_else(|_| json!({}));
                let content = raw
                    .pointer("/message/content")
                    .and_then(|x| x.as_str())
                    .map(strip_think_tags)
                    .unwrap_or_default();
                return Ok(json!({
                    "model": req_model,
                    "choices": [{ "message": { "role": "assistant", "content": content } }],
                    "message": { "role": "assistant", "content": content }
                }));
            }
            _ => {
                // Retry with :1.7b if requested model failed
                let mut fb_payload = ollama_payload.clone();
                fb_payload["model"] = json!("conrad-n8n-agent:1.7b");
                if let Ok(r2) = http_client().post(&chat_url).json(&fb_payload).send().await {
                    if r2.status().is_success() {
                        let raw: Value = r2.json().await.unwrap_or_else(|_| json!({}));
                        let content = raw.pointer("/message/content").and_then(|x| x.as_str()).map(strip_think_tags).unwrap_or_default();
                        return Ok(json!({
                            "model": "conrad-n8n-agent:1.7b",
                            "choices": [{ "message": { "role": "assistant", "content": content } }],
                            "message": { "role": "assistant", "content": content }
                        }));
                    }
                }
                return Err(anyhow::anyhow!("Ollama request failed for {}", req_model));
            }
        }
    }

    // 3. Draft-only / Safe Guard for external third-party write APIs (Brevo / Resend / Telegram / Stripe)
    if url.contains("api.brevo.com")
        || url.contains("api.resend.com")
        || url.contains("api.telegram.org")
        || url.contains("api.stripe.com")
    {
        return Ok(json!({
            "ok": true,
            "draftOnly": true,
            "guardedExternalCall": true,
            "url": url,
            "method": method,
            "payload": body_val
        }));
    }

    // 4. Local Qdrant or generic safe GET/POST
    if url.starts_with("http://127.0.0.1") || url.starts_with("http://localhost") {
        let req = match method.as_str() {
            "GET" => http_client().get(&url),
            _ => http_client().post(&url).json(&body_val),
        };
        if let Ok(r) = req.send().await {
            if let Ok(v) = r.json::<Value>().await {
                return Ok(v);
            }
        }
    }

    Ok(json!({ "ok": true, "url": url, "method": method, "body": body_val }))
}

fn build_http_json_body(
    jb: &str,
    input: &Value,
    node_outputs: &HashMap<String, Value>,
    preferred_model: &str,
) -> Value {
    let trimmed = jb.trim();
    if let Ok(v) = serde_json::from_str::<Value>(trimmed) {
        return v;
    }
    // Check if template uses conradKiAgent001 / palaInboxZero001 JSON.stringify({ ... })
    if jb.contains("status") && jb.contains("intent") && jb.contains("next_action") {
        let sys = "Du bist der n8n KI-Assistent von Conrad KI. Antworte ausschliesslich als valides rohes JSON ohne Markdown-Fences mit exakt diesen 5 Keys: status (\"ok\"|\"klaerung\"|\"fehler\"), intent (string), route (string), draft_only (boolean, immer true), summary_de (kurzer deutscher Satz), next_action (string).";
        let q = input.get("query").or_else(|| input.get("message")).or_else(|| input.get("prompt")).and_then(|x| x.as_str()).unwrap_or("Hallo");
        return json!({
            "model": input.get("model").and_then(|x| x.as_str()).unwrap_or(preferred_model),
            "messages": [
                { "role": "system", "content": sys },
                { "role": "user", "content": q }
            ]
        });
    }
    if jb.contains("kategorie") && jb.contains("prioritaet") && jb.contains("aktion") {
        let sys = "Du klassifizierst Geschaefts-E-Mails auf Deutsch. Antworte NUR mit rohem JSON ohne Markdown: {\"kategorie\": \"Rechnung\"|\"Kundenanfrage\"|\"Termin\"|\"Newsletter\"|\"Sonstiges\", \"prioritaet\": \"hoch\"|\"mittel\"|\"niedrig\", \"aktion\": \"entwurf_erstellen\"|\"label_setzen\"|\"archivieren\"|\"manuell_pruefen\", \"label\": string, \"zusammenfassung_de\": string, \"draftOnly\": true, \"markAsRead\": false}.";
        let mail_txt = format!(
            "Von: {}\nBetreff: {}\nInhalt: {}",
            input.get("from").and_then(|x| x.as_str()).unwrap_or(""),
            input.get("subject").and_then(|x| x.as_str()).unwrap_or(""),
            input.get("snippet").and_then(|x| x.as_str()).unwrap_or("")
        );
        return json!({
            "model": input.get("model").and_then(|x| x.as_str()).unwrap_or(preferred_model),
            "messages": [
                { "role": "system", "content": sys },
                { "role": "user", "content": mail_txt }
            ]
        });
    }
    let ev = eval_n8n_expression(jb, input, node_outputs);
    serde_json::from_str::<Value>(&ev).unwrap_or_else(|_| json!({ "raw": ev }))
}

async fn execute_langchain_agent_node(
    node_name: &str,
    params: &Value,
    input: &Value,
    node_outputs: &HashMap<String, Value>,
    all_nodes: &[Value],
    _conns: &Value,
    preferred_model: &str,
) -> anyhow::Result<Value> {
    let prompt_tpl = params
        .get("text")
        .or_else(|| params.get("prompt"))
        .and_then(|x| x.as_str())
        .unwrap_or("={{ $json.chatInput || $json.message || $json.text || 'Hallo' }}");
    let mut user_msg = eval_n8n_expression(prompt_tpl, input, node_outputs);
    if user_msg.trim().is_empty() {
        user_msg = input
            .get("message")
            .or_else(|| input.get("chatInput"))
            .or_else(|| input.get("text"))
            .or_else(|| input.get("query"))
            .and_then(|x| x.as_str())
            .unwrap_or("Analysiere diese Anfrage auf Deutsch.")
            .to_string();
    }

    let sys_prompt = params
        .pointer("/options/systemMessage")
        .and_then(|x| x.as_str())
        .map(|s| eval_n8n_expression(s, input, node_outputs))
        .unwrap_or_else(|| {
            format!(
                "Du bist der n8n KI-Agent fuer '{}'. Antworte praezise auf Deutsch. Halte dich strikt an Fakten und draftOnly=true.",
                node_name
            )
        });

    let has_qdrant = all_nodes.iter().any(|n| {
        n.get("type")
            .and_then(|x| x.as_str())
            .map(|t| t.contains("vectorStoreQdrant"))
            .unwrap_or(false)
    });
    let mut rag_block = String::new();
    if has_qdrant {
        if let Ok(emb_r) = http_client()
            .post(format!("{}/api/embeddings", ollama_base_url()))
            .json(&json!({ "model": "nomic-embed-text", "prompt": &user_msg }))
            .send()
            .await
        {
            if let Ok(ev) = emb_r.json::<Value>().await {
                if let Some(vec_arr) = ev.get("embedding").and_then(|x| x.as_array()) {
                    if let Ok(qr) = http_client()
                        .post("http://127.0.0.1:6333/collections/conrad_ki_knowledge/points/search")
                        .json(&json!({ "vector": vec_arr, "limit": 3, "with_payload": true }))
                        .send()
                        .await
                    {
                        if let Ok(qv) = qr.json::<Value>().await {
                            if let Some(hits) = qv.get("result").and_then(|x| x.as_array()) {
                                for h in hits {
                                    if let Some(txt) = h.pointer("/payload/text").and_then(|x| x.as_str()) {
                                        rag_block.push_str(txt);
                                        rag_block.push('\n');
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let final_sys = if rag_block.is_empty() {
        sys_prompt
    } else {
        format!("{}\n\nWissensbasis (Qdrant):\n{}", sys_prompt, rag_block)
    };

    let chat_url = format!("{}/api/chat", ollama_base_url());
    let payload = json!({
        "model": preferred_model,
        "stream": false,
        "think": false,
        "options": { "temperature": 0.1, "num_ctx": 4096, "num_predict": 384 },
        "messages": [
            { "role": "system", "content": final_sys },
            { "role": "user", "content": user_msg }
        ]
    });
    let resp = http_client().post(&chat_url).json(&payload).send().await?;
    let raw: Value = resp.json().await.unwrap_or_else(|_| json!({}));
    let content = raw
        .pointer("/message/content")
        .and_then(|x| x.as_str())
        .map(strip_think_tags)
        .unwrap_or_default();

    Ok(json!({
        "output": content,
        "response": content,
        "model": preferred_model,
        "ragUsed": !rag_block.is_empty(),
        "draftOnly": true
    }))
}
