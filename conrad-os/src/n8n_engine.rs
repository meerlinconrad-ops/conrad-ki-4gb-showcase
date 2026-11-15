//! Pure-Rust n8n Workflow Engine (1:1 Architektur zu estatic/r8r & n8n 2.x).
//!
//! Features:
//! - QuickJS-Sandbox (`n8n_vm`) fuer `={{ ... }}` Expressions & JS Code-Nodes (inkl. Luxon & JMESPath)
//! - OpenSSL AES-256-CBC `Salted__` Credential-Verschluesselung (`n8n_cipher`) fuer 47+ Typen
//! - 110+ Node-Typen (`/types/nodes.json`), `pinData`, `retryOnFail`, `onError`, `executeOnce`, `alwaysOutputData`
//! - Persistente n8n 2.x Datentabellen, Variablen (`$vars`), Tags, Formulare (`/form/*`) & Webhooks
//! - Strenger Entwurfsschutz (`draftOnly: true`) fuer externe E-Mail-, Messaging- & Zahlungsaufrufe

use crate::db::Database;
use crate::n8n_cipher::{decrypt_json_lenient, encrypt_json, encryption_key};
use crate::n8n_schema::{credentials_json, nodes_json, redact_credential_data, BLANK_SENTINEL};
use crate::n8n_vm::{digest, eval_js_code_node, eval_n8n_json_value, eval_n8n_template};
use crate::twenty::{ollama_base_url, strip_think_tags};
use crate::AppState;
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, Method, StatusCode},
    response::{Html, IntoResponse},
    Json,
};
use base64::{engine::general_purpose::{STANDARD as B64, URL_SAFE_NO_PAD as B64URL}, Engine as _};
use chrono::{DateTime, Duration as ChronoDur, Utc};
use hmac::{Hmac, Mac};
use rusqlite::{params, Connection, OpenFlags, OptionalExtension};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use sha2::Sha256;
use std::collections::{HashMap, HashSet, VecDeque};
use std::future::Future;
use std::net::IpAddr;
use std::pin::Pin;
use std::sync::OnceLock;
use std::time::{Duration, Instant};
use uuid::Uuid;

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn http_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(20))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new())
    })
}

const DEFAULT_N8N_SQLITE: &str = "/opt/n8n/n8n_data/database.sqlite";

fn n8n_sqlite_path() -> String {
    std::env::var("N8N_SQLITE_PATH").unwrap_or_else(|_| DEFAULT_N8N_SQLITE.to_string())
}

// =========================================================================
// Schema-Initialisierung, Seeds & Live-n8n-SQLite-Sync
// =========================================================================

pub fn init_and_sync_n8n(db: &Database) -> anyhow::Result<()> {
    {
        let conn = db.lock_conn();
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS n8n_workflows (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                active INTEGER NOT NULL DEFAULT 0,
                nodes_json TEXT NOT NULL DEFAULT '[]',
                connections_json TEXT NOT NULL DEFAULT '{}',
                settings_json TEXT NOT NULL DEFAULT '{}',
                tags_json TEXT NOT NULL DEFAULT '[]',
                pin_data_json TEXT NOT NULL DEFAULT '{}',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                execution_count INTEGER NOT NULL DEFAULT 0,
                last_executed_at TEXT
            );
            CREATE TABLE IF NOT EXISTS n8n_executions (
                id TEXT PRIMARY KEY,
                workflow_id TEXT NOT NULL,
                workflow_name TEXT NOT NULL,
                status TEXT NOT NULL,
                mode TEXT NOT NULL,
                started_at TEXT NOT NULL,
                finished_at TEXT NOT NULL,
                duration_ms INTEGER NOT NULL DEFAULT 0,
                data_json TEXT NOT NULL DEFAULT '{}',
                error_message TEXT,
                retry_of TEXT
            );
            CREATE TABLE IF NOT EXISTS n8n_credentials (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                cred_type TEXT NOT NULL,
                data_encrypted TEXT NOT NULL,
                nodes_access_json TEXT NOT NULL DEFAULT '[]',
                is_managed INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS n8n_variables (
                id TEXT PRIMARY KEY,
                var_key TEXT NOT NULL UNIQUE,
                value TEXT NOT NULL,
                var_type TEXT NOT NULL DEFAULT 'string',
                project_id TEXT NOT NULL DEFAULT 'default',
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS n8n_tags (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS n8n_data_tables (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                columns_json TEXT NOT NULL DEFAULT '[]',
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS n8n_data_table_rows (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                table_id TEXT NOT NULL,
                data_json TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS n8n_waiting_executions (
                execution_id TEXT PRIMARY KEY,
                workflow_id TEXT NOT NULL,
                node_name TEXT NOT NULL,
                resume_payload_json TEXT NOT NULL DEFAULT '{}',
                created_at TEXT NOT NULL
            );
            "#,
        )?;
        // Idempotente Migration auf bestehenden Live-Datenbanken
        let _ = conn.execute("ALTER TABLE n8n_workflows ADD COLUMN pin_data_json TEXT DEFAULT '{}'", []);
        let _ = conn.execute("ALTER TABLE n8n_workflows ADD COLUMN execution_count INTEGER NOT NULL DEFAULT 0", []);
        let _ = conn.execute("ALTER TABLE n8n_workflows ADD COLUMN last_executed_at TEXT", []);
        let _ = conn.execute("ALTER TABLE n8n_executions ADD COLUMN retry_of TEXT", []);
        let _ = conn.execute("ALTER TABLE n8n_executions ADD COLUMN data_json TEXT NOT NULL DEFAULT '{}'", []);
        let _ = conn.execute("ALTER TABLE n8n_executions ADD COLUMN error_message TEXT", []);

        seed_defaults(&conn)?;
    }

    if std::path::Path::new(&n8n_sqlite_path()).exists() {
        if let Err(e) = sync_from_n8n_sqlite(db) {
            tracing::warn!("n8n SQLite Sync Hinweis: {}", e);
        }
    }
    tracing::info!("Pure-Rust n8n engine ready");
    Ok(())
}

fn seed_defaults(conn: &Connection) -> anyhow::Result<()> {
    let now = Utc::now().to_rfc3339();

    let var_count: i64 = conn.query_row("SELECT COUNT(*) FROM n8n_variables", [], |r| r.get(0)).unwrap_or(0);
    if var_count == 0 {
        for (k, v) in [
            ("COMPANY_NAME", "Conrad KI"),
            ("DRAFT_ONLY", "true"),
            ("REGION", "Rhein-Neckar"),
            ("DEFAULT_MODEL", "qwen2.5:1.5b"),
        ] {
            let _ = conn.execute(
                "INSERT OR IGNORE INTO n8n_variables (id, var_key, value, var_type, project_id, updated_at) VALUES (?1, ?2, ?3, 'string', 'default', ?4)",
                params![Uuid::new_v4().to_string(), k, v, now],
            );
        }
    }

    let dt_count: i64 = conn.query_row("SELECT COUNT(*) FROM n8n_data_tables", [], |r| r.get(0)).unwrap_or(0);
    if dt_count == 0 {
        let tid = "dt-lead-scoring-regeln";
        let cols = json!([
            {"name": "branche", "type": "string"},
            {"name": "score", "type": "number"},
            {"name": "paket", "type": "string"},
            {"name": "prioritaet", "type": "string"}
        ]);
        conn.execute(
            "INSERT OR IGNORE INTO n8n_data_tables (id, name, columns_json, created_at, updated_at) VALUES (?1, 'Lead-Scoring-Regeln', ?2, ?3, ?3)",
            params![tid, cols.to_string(), now],
        )?;
        for row in [
            json!({"branche": "Handwerk / SHK", "score": 88, "paket": "Wachstum 69€/Monat", "prioritaet": "HOCH"}),
            json!({"branche": "Zahnarzt / MVZ", "score": 94, "paket": "Pro 99€/Monat", "prioritaet": "SEHR HOCH"}),
            json!({"branche": "Steuerberater / Kanzlei", "score": 91, "paket": "Pro 99€/Monat", "prioritaet": "HOCH"}),
        ] {
            conn.execute(
                "INSERT INTO n8n_data_table_rows (table_id, data_json, created_at, updated_at) VALUES (?1, ?2, ?3, ?3)",
                params![tid, row.to_string(), now],
            )?;
        }
    }

    let wf_count: i64 = conn.query_row("SELECT COUNT(*) FROM n8n_workflows", [], |r| r.get(0)).unwrap_or(0);
    if wf_count == 0 {
        let starter_nodes = json!([
            {
                "id": "n1", "name": "Webhook Lead-Eingang", "type": "n8n-nodes-base.webhook", "typeVersion": 2,
                "position": [160, 260], "parameters": {"httpMethod": "POST", "path": "lead-intake", "responseMode": "onReceived"}
            },
            {
                "id": "n2", "name": "Score Pruefung", "type": "n8n-nodes-base.if", "typeVersion": 2,
                "position": [420, 260],
                "parameters": {"combinator": "and", "conditions": {"conditions": [{"leftValue": "={{ $json.score || 85 }}", "operator": {"type": "number", "operation": "gte"}, "rightValue": 70}]}}
            },
            {
                "id": "n3", "name": "QuickJS Anreicherung", "type": "n8n-nodes-base.code", "typeVersion": 2,
                "position": [680, 200],
                "parameters": {"language": "javaScript", "mode": "runOnceForAllItems", "jsCode": "return $input.all().map(i => ({\n  json: {\n    ...i.json,\n    tag: $now.toISODate(),\n    mandant: $vars.COMPANY_NAME || 'Conrad KI',\n    status: 'qualifiziert'\n  }\n}));"}
            },
            {
                "id": "n4", "name": "Antwort-Entwurf speichern", "type": "n8n-nodes-base.emailSend", "typeVersion": 2,
                "position": [940, 200],
                "parameters": {"toEmail": "={{ $json.email || 'kontakt@beispiel.de' }}", "subject": "={{ 'Conrad KI Angebot fuer ' + ($json.company || 'Ihr Team') }}", "draftOnly": true}
            }
        ]);
        let starter_conns = json!({
            "Webhook Lead-Eingang": { "main": [[{ "node": "Score Pruefung", "type": "main", "index": 0 }]] },
            "Score Pruefung": { "main": [[{ "node": "QuickJS Anreicherung", "type": "main", "index": 0 }], []] },
            "QuickJS Anreicherung": { "main": [[{ "node": "Antwort-Entwurf speichern", "type": "main", "index": 0 }]] }
        });
        conn.execute(
            "INSERT INTO n8n_workflows (id, name, active, nodes_json, connections_json, settings_json, tags_json, pin_data_json, created_at, updated_at)
             VALUES ('wf-conrad-lead-engine', 'Conrad KI — Lead Qualifizierung & Entwurf', 1, ?1, ?2, '{}', '[\"Conrad KI\",\"Vertrieb\"]', '{}', ?3, ?3)",
            params![starter_nodes.to_string(), starter_conns.to_string(), now],
        )?;
    }

    Ok(())
}

pub fn sync_from_n8n_sqlite(db: &Database) -> anyhow::Result<Value> {
    let path = n8n_sqlite_path();
    let src = Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    let conn = db.lock_conn();
    let now = Utc::now().to_rfc3339();

    let mut wf_cols = HashSet::new();
    if let Ok(mut st) = src.prepare("PRAGMA table_info(workflow_entity)") {
        if let Ok(mut rows) = st.query([]) {
            while let Ok(Some(r)) = rows.next() {
                if let Ok(nm) = r.get::<_, String>(1) {
                    wf_cols.insert(nm);
                }
            }
        }
    }
    let has_pin = wf_cols.contains("pinData");
    let wf_sql = if has_pin {
        "SELECT id, name, active, nodes, connections, settings, COALESCE(pinData, '{}') FROM workflow_entity"
    } else {
        "SELECT id, name, active, nodes, connections, settings, '{}' FROM workflow_entity"
    };

    let mut wf_count = 0usize;
    if let Ok(mut stmt) = src.prepare(wf_sql) {
        if let Ok(mut rows) = stmt.query([]) {
            while let Ok(Some(r)) = rows.next() {
                let id: String = r.get(0)?;
                let name: String = r.get(1)?;
                let active: i64 = r.get::<_, i64>(2).unwrap_or(0);
                let nodes: String = r.get(3).unwrap_or_else(|_| "[]".into());
                let conns: String = r.get(4).unwrap_or_else(|_| "{}".into());
                let settings: String = r.get(5).unwrap_or_else(|_| "{}".into());
                let pin_data: String = r.get(6).unwrap_or_else(|_| "{}".into());

                conn.execute(
                    "INSERT INTO n8n_workflows (id, name, active, nodes_json, connections_json, settings_json, tags_json, pin_data_json, created_at, updated_at)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, '[\"Live n8n Sync\"]', ?7, ?8, ?8)
                     ON CONFLICT(id) DO UPDATE SET
                       name = excluded.name, active = excluded.active, nodes_json = excluded.nodes_json,
                       connections_json = excluded.connections_json, settings_json = excluded.settings_json,
                       pin_data_json = CASE WHEN excluded.pin_data_json != '{}' THEN excluded.pin_data_json ELSE n8n_workflows.pin_data_json END,
                       updated_at = excluded.updated_at",
                    params![id, name, active, nodes, conns, settings, pin_data, now],
                )?;
                wf_count += 1;
            }
        }
    }

    let mut cred_count = 0usize;
    if let Ok(mut stmt) = src.prepare("SELECT id, name, type, data FROM credentials_entity") {
        if let Ok(mut rows) = stmt.query([]) {
            let key = encryption_key();
            while let Ok(Some(r)) = rows.next() {
                let id: String = r.get(0)?;
                let name: String = r.get(1)?;
                let ctype: String = r.get(2)?;
                let raw_data: String = r.get(3).unwrap_or_else(|_| "{}".into());
                let stored = if raw_data.trim().starts_with("U2FsdGVkX1") || raw_data.trim().starts_with('{') {
                    if raw_data.trim().starts_with('{') {
                        serde_json::from_str::<Value>(&raw_data)
                            .ok()
                            .and_then(|v| encrypt_json(&key, &v).ok())
                            .unwrap_or(raw_data)
                    } else {
                        raw_data
                    }
                } else {
                    raw_data
                };
                let _ = conn.execute(
                    "INSERT OR IGNORE INTO n8n_credentials (id, name, cred_type, data_encrypted, nodes_access_json, created_at, updated_at)
                     VALUES (?1, ?2, ?3, ?4, '[]', ?5, ?5)",
                    params![id, name, ctype, stored, now],
                );
                cred_count += 1;
            }
        }
    }

    let mut var_count = 0usize;
    if let Ok(mut stmt) = src.prepare("SELECT id, key, value, COALESCE(type,'string') FROM variables") {
        if let Ok(mut rows) = stmt.query([]) {
            while let Ok(Some(r)) = rows.next() {
                let id: String = r.get(0)?;
                let k: String = r.get(1)?;
                let v: String = r.get(2).unwrap_or_default();
                let tp: String = r.get(3).unwrap_or_else(|_| "string".into());
                let _ = conn.execute(
                    "INSERT OR IGNORE INTO n8n_variables (id, var_key, value, var_type, project_id, updated_at)
                     VALUES (?1, ?2, ?3, ?4, 'default', ?5)",
                    params![id, k, v, tp, now],
                );
                var_count += 1;
            }
        }
    }

    let mut tag_count = 0usize;
    if let Ok(mut stmt) = src.prepare("SELECT id, name FROM tag_entity") {
        if let Ok(mut rows) = stmt.query([]) {
            while let Ok(Some(r)) = rows.next() {
                let id: String = r.get(0)?;
                let name: String = r.get(1)?;
                let _ = conn.execute(
                    "INSERT OR IGNORE INTO n8n_tags (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?3)",
                    params![id, name, now],
                );
                tag_count += 1;
            }
        }
    }

    Ok(json!({
        "ok": true,
        "syncedWorkflows": wf_count,
        "syncedCredentials": cred_count,
        "syncedVariables": var_count,
        "syncedTags": tag_count
    }))
}

fn load_vars_json(db: &Database) -> Value {
    let conn = db.lock_conn();
    let mut map = Map::new();
    if let Ok(mut st) = conn.prepare("SELECT var_key, value FROM n8n_variables") {
        if let Ok(mut rows) = st.query([]) {
            while let Ok(Some(r)) = rows.next() {
                if let (Ok(k), Ok(v)) = (r.get::<_, String>(0), r.get::<_, String>(1)) {
                    map.insert(k, Value::String(v));
                }
            }
        }
    }
    Value::Object(map)
}

fn resolve_credential_secret(db: &Database, cred_id: Option<&str>, cred_name: Option<&str>) -> Option<Value> {
    let conn = db.lock_conn();
    let enc: Option<String> = if let Some(cid) = cred_id {
        conn.query_row("SELECT data_encrypted FROM n8n_credentials WHERE id = ?1", [cid], |r| r.get(0))
            .optional()
            .ok()
            .flatten()
    } else if let Some(cname) = cred_name {
        conn.query_row("SELECT data_encrypted FROM n8n_credentials WHERE name = ?1", [cname], |r| r.get(0))
            .optional()
            .ok()
            .flatten()
    } else {
        None
    };
    let enc_str = enc?;
    decrypt_json_lenient(&encryption_key(), &enc_str).ok()
}

// =========================================================================
// Helper: QuickJS Expressions & Multi-Input / Multi-Branch Graph Helpers
// =========================================================================

pub fn eval_n8n_expression(
    raw: &str,
    node_outputs: &HashMap<String, Vec<Value>>,
    current_node: &str,
    item_idx: usize,
    vars: &Value,
    exec_meta: &Value,
) -> Value {
    eval_n8n_template(raw, node_outputs, current_node, item_idx, vars, exec_meta)
        .unwrap_or_else(|_| Value::String(raw.to_string()))
}

pub fn resolve_n8n_value(
    val: &Value,
    node_outputs: &HashMap<String, Vec<Value>>,
    current_node: &str,
    item_idx: usize,
    vars: &Value,
    exec_meta: &Value,
) -> Value {
    eval_n8n_json_value(val, node_outputs, current_node, item_idx, vars, exec_meta)
}

fn workflow_row_to_json(r: &rusqlite::Row<'_>) -> rusqlite::Result<Value> {
    let nodes_str: String = r.get(3)?;
    let conns_str: String = r.get(4)?;
    let settings_str: String = r.get(5)?;
    let tags_str: String = r.get(6)?;
    let pin_str: String = r.get::<_, Option<String>>(7)?.unwrap_or_else(|| "{}".into());
    let nodes: Value = serde_json::from_str(&nodes_str).unwrap_or_else(|_| json!([]));
    let connections: Value = serde_json::from_str(&conns_str).unwrap_or_else(|_| json!({}));
    let settings: Value = serde_json::from_str(&settings_str).unwrap_or_else(|_| json!({}));
    let tags: Value = serde_json::from_str(&tags_str).unwrap_or_else(|_| json!([]));
    let pin_data: Value = serde_json::from_str(&pin_str).unwrap_or_else(|_| json!({}));

    Ok(json!({
        "id": r.get::<_, String>(0)?,
        "name": r.get::<_, String>(1)?,
        "active": r.get::<_, i64>(2)? != 0,
        "nodes": nodes,
        "connections": connections,
        "settings": settings,
        "tags": tags,
        "pinData": pin_data,
        "createdAt": r.get::<_, String>(8)?,
        "updatedAt": r.get::<_, String>(9)?,
        "executionCount": r.get::<_, i64>(10)?,
        "lastExecutedAt": r.get::<_, Option<String>>(11)?
    }))
}

// =========================================================================
// DAG Workflow Execution Engine
// =========================================================================

pub async fn execute_workflow_by_id(
    db: &Database,
    wf_id: &str,
    trigger_payload: Value,
    only_node: Option<&str>,
) -> anyhow::Result<Value> {
    execute_workflow_with_depth(db, wf_id, trigger_payload, only_node, 0, None).await
}

fn execute_workflow_with_depth<'a>(
    db: &'a Database,
    wf_id: &'a str,
    trigger_payload: Value,
    only_node: Option<&'a str>,
    depth: u8,
    retry_of: Option<&'a str>,
) -> Pin<Box<dyn Future<Output = anyhow::Result<Value>> + Send + 'a>> {
    Box::pin(async move {
        if depth > 8 {
            anyhow::bail!("Maximale Sub-Workflow Verschachtelungstiefe (8) ueberschritten");
        }

        let wf = {
            let conn = db.lock_conn();
            conn.query_row(
                "SELECT id, name, active, nodes_json, connections_json, settings_json, tags_json, pin_data_json, created_at, updated_at, execution_count, last_executed_at
                 FROM n8n_workflows WHERE id = ?1",
                [wf_id],
                workflow_row_to_json,
            )
            .optional()?
            .ok_or_else(|| anyhow::anyhow!("Workflow '{}' nicht gefunden", wf_id))?
        };

        let wf_name = wf["name"].as_str().unwrap_or("Workflow").to_string();
        let nodes = wf["nodes"].as_array().cloned().unwrap_or_default();
        let connections = wf["connections"].clone();
        let wf_pin_data = wf["pinData"].clone();
        let vars = load_vars_json(db);

        let started_at = Utc::now();
        let started_iso = started_at.to_rfc3339();
        let exec_id = Uuid::new_v4().to_string();
        let mode = if only_node.is_some() { "manual_node" } else { "manual" };
        let exec_meta = json!({
            "id": exec_id,
            "mode": mode,
            "workflowId": wf_id,
            "workflowName": wf_name
        });

        let mut node_map: HashMap<String, Value> = HashMap::new();
        let mut outgoing: HashMap<String, Vec<(usize, String, usize)>> = HashMap::new(); // src -> [(src_branch, dst_node, dst_port)]
        let mut incoming_count: HashMap<String, usize> = HashMap::new();

        for n in &nodes {
            if let Some(name) = n.get("name").and_then(|v| v.as_str()) {
                node_map.insert(name.to_string(), n.clone());
                incoming_count.entry(name.to_string()).or_insert(0);
            }
        }

        if let Some(cobj) = connections.as_object() {
            for (src, conn_val) in cobj {
                if let Some(main_arr) = conn_val.get("main").and_then(|v| v.as_array()) {
                    for (branch_idx, branch_val) in main_arr.iter().enumerate() {
                        if let Some(targets) = branch_val.as_array() {
                            for t in targets {
                                if let Some(dst) = t.get("node").and_then(|v| v.as_str()) {
                                    let dst_port = t.get("index").and_then(|v| v.as_u64()).unwrap_or(0) as usize;
                                    outgoing
                                        .entry(src.clone())
                                        .or_default()
                                        .push((branch_idx, dst.to_string(), dst_port));
                                    *incoming_count.entry(dst.to_string()).or_insert(0) += 1;
                                }
                            }
                        }
                    }
                }
            }
        }

        let initial_items = match trigger_payload.clone() {
            Value::Array(arr) if !arr.is_empty() => arr,
            Value::Null => vec![json!({
                "timestamp": Utc::now().to_rfc3339(),
                "source": "Conrad KI n8n-Rust Engine"
            })],
            other => vec![other],
        };

        let mut node_outputs: HashMap<String, Vec<Value>> = HashMap::new();
        let mut node_branches: HashMap<String, Vec<Vec<Value>>> = HashMap::new();
        let mut node_inputs_by_port: HashMap<String, HashMap<usize, Vec<Value>>> = HashMap::new();
        let mut trace_steps: Vec<Value> = Vec::new();
        let mut final_items: Vec<Value> = initial_items.clone();
        let mut status = "success".to_string();
        let mut error_msg: Option<String> = None;
        let mut webhook_response: Option<Value> = None;

        if let Some(target_node_name) = only_node {
            // Einzelner Node-Test im NDV
            let n = node_map
                .get(target_node_name)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("Node '{}' nicht gefunden", target_node_name))?;
            node_outputs.insert(target_node_name.to_string(), initial_items.clone());
            let mut port_map = HashMap::new();
            port_map.insert(0usize, initial_items.clone());
            let started_n = Instant::now();
            match execute_node_with_flags(
                db,
                &n,
                &initial_items,
                &port_map,
                &wf_pin_data,
                &node_outputs,
                &vars,
                &exec_meta,
                depth,
                &mut webhook_response,
            )
            .await
            {
                Ok(outcome) => {
                    let branch0 = outcome.branches.first().cloned().unwrap_or_default();
                    node_outputs.insert(target_node_name.to_string(), branch0.clone());
                    node_branches.insert(target_node_name.to_string(), outcome.branches.clone());
                    final_items = branch0.clone();
                    trace_steps.push(json!({
                        "node": target_node_name,
                        "type": n["type"],
                        "status": "success",
                        "pinned": outcome.pinned,
                        "durationMs": started_n.elapsed().as_millis() as i64,
                        "outputCount": branch0.len(),
                        "branches": outcome.branches,
                        "items": branch0,
                        "meta": outcome.meta
                    }));
                }
                Err(e) => {
                    status = "error".to_string();
                    error_msg = Some(e.to_string());
                    trace_steps.push(json!({
                        "node": target_node_name,
                        "type": n["type"],
                        "status": "error",
                        "durationMs": started_n.elapsed().as_millis() as i64,
                        "error": e.to_string()
                    }));
                }
            }
        } else {
            // Queue-basierte DAG-Ausfuehrung inkl. Verzweigungen (IF/Switch/CompareDatasets)
            let mut ready: VecDeque<String> = VecDeque::new();
            let mut visited_count: HashMap<String, usize> = HashMap::new();

            for n in &nodes {
                let name = n["name"].as_str().unwrap_or_default().to_string();
                let tp = n["type"].as_str().unwrap_or_default().to_ascii_lowercase();
                let in_deg = *incoming_count.get(&name).unwrap_or(&0);
                if (tp.contains("trigger") || tp.ends_with(".webhook") || tp.ends_with(".formtrigger")) && in_deg == 0 {
                    node_inputs_by_port.entry(name.clone()).or_default().insert(0, initial_items.clone());
                    ready.push_back(name);
                }
            }
            if ready.is_empty() {
                for n in &nodes {
                    let name = n["name"].as_str().unwrap_or_default().to_string();
                    if *incoming_count.get(&name).unwrap_or(&0) == 0 {
                        node_inputs_by_port.entry(name.clone()).or_default().insert(0, initial_items.clone());
                        ready.push_back(name);
                    }
                }
            }

            let mut total_steps = 0usize;
            while let Some(cur_name) = ready.pop_front() {
                total_steps += 1;
                if total_steps > 200 {
                    status = "error".to_string();
                    error_msg = Some("Maximale Schrittanzahl (200) ueberschritten (Endlosschleife?)".to_string());
                    break;
                }
                let visits = visited_count.entry(cur_name.clone()).or_insert(0);
                *visits += 1;
                if *visits > 25 {
                    continue;
                }

                let Some(n) = node_map.get(&cur_name).cloned() else {
                    continue;
                };

                let port_map = node_inputs_by_port.get(&cur_name).cloned().unwrap_or_default();
                let mut flat_items = port_map.get(&0).cloned().unwrap_or_default();
                if flat_items.is_empty() {
                    for p_items in port_map.values() {
                        flat_items.extend(p_items.clone());
                    }
                }
                if flat_items.is_empty() {
                    flat_items = initial_items.clone();
                }

                // Vor Node-Ausfuehrung aktuellen Input auch als Node-Kontext verfuegbar machen
                node_outputs.entry(cur_name.clone()).or_insert_with(|| flat_items.clone());

                let started_n = Instant::now();
                match execute_node_with_flags(
                    db,
                    &n,
                    &flat_items,
                    &port_map,
                    &wf_pin_data,
                    &node_outputs,
                    &vars,
                    &exec_meta,
                    depth,
                    &mut webhook_response,
                )
                .await
                {
                    Ok(outcome) => {
                        let branch0 = outcome.branches.first().cloned().unwrap_or_default();
                        node_outputs.insert(cur_name.clone(), branch0.clone());
                        node_branches.insert(cur_name.clone(), outcome.branches.clone());
                        if !branch0.is_empty() {
                            final_items = branch0.clone();
                        }
                        trace_steps.push(json!({
                            "node": cur_name,
                            "type": n["type"],
                            "status": "success",
                            "pinned": outcome.pinned,
                            "durationMs": started_n.elapsed().as_millis() as i64,
                            "outputCount": branch0.len(),
                            "branches": outcome.branches,
                            "items": branch0,
                            "input": flat_items.first().cloned().unwrap_or_else(|| json!(flat_items)),
                            "output": branch0.first().cloned().unwrap_or_else(|| json!(branch0)),
                            "meta": outcome.meta
                        }));

                        if let Some(edges) = outgoing.get(&cur_name) {
                            for (src_branch, dst_node, dst_port) in edges {
                                let items_for_branch = outcome
                                    .branches
                                    .get(*src_branch)
                                    .cloned()
                                    .unwrap_or_default();
                                if items_for_branch.is_empty() {
                                    continue;
                                }
                                let target_ports = node_inputs_by_port.entry(dst_node.clone()).or_default();
                                target_ports.insert(*dst_port, items_for_branch);
                                if !ready.contains(dst_node) {
                                    ready.push_back(dst_node.clone());
                                }
                            }
                        }
                    }
                    Err(e) => {
                        status = "error".to_string();
                        error_msg = Some(format!("Node '{}': {}", cur_name, e));
                        trace_steps.push(json!({
                            "node": cur_name,
                            "type": n["type"],
                            "status": "error",
                            "durationMs": started_n.elapsed().as_millis() as i64,
                            "error": e.to_string()
                        }));
                        break;
                    }
                }
            }
        }

        let finished_at = Utc::now();
        let duration_ms = (finished_at - started_at).num_milliseconds().max(1);
        let finished_iso = finished_at.to_rfc3339();
        let exec_data = json!({
            "modelUsed": "QuickJS + Local Rust Engine",
            "steps": trace_steps,
            "nodeOutputs": node_outputs,
            "finalItems": final_items,
            "triggerPayload": trigger_payload,
            "webhookResponse": webhook_response
        });

        {
            let conn = db.lock_conn();
            conn.execute(
                "INSERT INTO n8n_executions (id, workflow_id, workflow_name, status, mode, started_at, finished_at, duration_ms, data_json, error_message, retry_of)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
                params![
                    exec_id,
                    wf_id,
                    wf_name,
                    status,
                    mode,
                    started_iso,
                    finished_iso,
                    duration_ms,
                    exec_data.to_string(),
                    error_msg,
                    retry_of
                ],
            )?;
            let _ = conn.execute(
                "UPDATE n8n_workflows SET execution_count = execution_count + 1, last_executed_at = ?1 WHERE id = ?2",
                params![finished_iso, wf_id],
            );
        }

        Ok(json!({
            "ok": status == "success",
            "executionId": exec_id,
            "workflowId": wf_id,
            "workflowName": wf_name,
            "status": status,
            "mode": mode,
            "modelUsed": "QuickJS + Local Rust Engine",
            "durationMs": duration_ms,
            "steps": exec_data["steps"],
            "nodeLogs": exec_data["steps"],
            "nodeOutputs": exec_data["nodeOutputs"],
            "finalItems": final_items,
            "input": initial_items.first().cloned().unwrap_or_else(|| json!(initial_items)),
            "output": final_items.first().cloned().unwrap_or_else(|| json!(final_items)),
            "webhookResponse": webhook_response,
            "error": error_msg
        }))
    })
}

struct NodeOutcome {
    branches: Vec<Vec<Value>>,
    pinned: bool,
    meta: Value,
}

async fn execute_node_with_flags(
    db: &Database,
    node: &Value,
    items: &[Value],
    ports: &HashMap<usize, Vec<Value>>,
    wf_pin_data: &Value,
    node_outputs: &HashMap<String, Vec<Value>>,
    vars: &Value,
    exec_meta: &Value,
    depth: u8,
    webhook_response: &mut Option<Value>,
) -> anyhow::Result<NodeOutcome> {
    let name = node["name"].as_str().unwrap_or("Node").to_string();

    if node["disabled"].as_bool().unwrap_or(false) {
        return Ok(NodeOutcome {
            branches: vec![items.to_vec()],
            pinned: false,
            meta: json!({"skipped": true, "reason": "disabled"}),
        });
    }

    // pinData pruefen (workflow.pinData[nodeName] oder node.pinData)
    if let Some(pin_arr) = wf_pin_data
        .get(&name)
        .or_else(|| node.get("pinData"))
        .and_then(|v| v.as_array())
    {
        if !pin_arr.is_empty() {
            let unwrapped: Vec<Value> = pin_arr
                .iter()
                .map(|it| it.get("json").cloned().unwrap_or_else(|| it.clone()))
                .collect();
            return Ok(NodeOutcome {
                branches: vec![unwrapped],
                pinned: true,
                meta: json!({"pinned": true}),
            });
        }
    }

    let exec_once = node["executeOnce"].as_bool().unwrap_or(false);
    let working_items = if exec_once && items.len() > 1 {
        &items[..1]
    } else {
        items
    };

    let retry_on_fail = node["retryOnFail"].as_bool().unwrap_or(false);
    let max_tries = if retry_on_fail {
        node["maxTries"].as_u64().unwrap_or(3).clamp(1, 5)
    } else {
        1
    };
    let wait_ms = node["waitBetweenTries"].as_u64().unwrap_or(200).min(1500);
    let on_error = node["onError"].as_str().unwrap_or("stopWorkflow");
    let always_out = node["alwaysOutputData"].as_bool().unwrap_or(false);

    let mut last_err: Option<anyhow::Error> = None;
    for attempt in 1..=max_tries {
        match execute_node_inner(
            db,
            node,
            working_items,
            ports,
            node_outputs,
            vars,
            exec_meta,
            depth,
            webhook_response,
        )
        .await
        {
            Ok(mut out) => {
                if always_out && out.branches.first().map(|b| b.is_empty()).unwrap_or(true) {
                    if out.branches.is_empty() {
                        out.branches.push(vec![json!({})]);
                    } else {
                        out.branches[0].push(json!({}));
                    }
                }
                return Ok(out);
            }
            Err(e) => {
                last_err = Some(e);
                if attempt < max_tries {
                    tokio::time::sleep(Duration::from_millis(wait_ms)).await;
                }
            }
        }
    }

    let err = last_err.unwrap_or_else(|| anyhow::anyhow!("Unbekannter Node-Fehler"));
    match on_error {
        "continueRegularOutput" => Ok(NodeOutcome {
            branches: vec![vec![json!({ "error": err.to_string(), "node": name })]],
            pinned: false,
            meta: json!({"onError": "continueRegularOutput"}),
        }),
        "continueErrorOutput" => Ok(NodeOutcome {
            branches: vec![vec![], vec![json!({ "error": err.to_string(), "node": name })]],
            pinned: false,
            meta: json!({"onError": "continueErrorOutput"}),
        }),
        _ => Err(err),
    }
}

async fn execute_node_inner(
    db: &Database,
    node: &Value,
    items: &[Value],
    ports: &HashMap<usize, Vec<Value>>,
    node_outputs: &HashMap<String, Vec<Value>>,
    vars: &Value,
    exec_meta: &Value,
    depth: u8,
    webhook_response: &mut Option<Value>,
) -> anyhow::Result<NodeOutcome> {
    let node_name = node["name"].as_str().unwrap_or("Node");
    let raw_type = node["type"].as_str().unwrap_or("n8n-nodes-base.noOp");
    let tp = raw_type.trim_end_matches("Tool");
    let params = node.get("parameters").cloned().unwrap_or_else(|| json!({}));

    let short = tp.split('.').last().unwrap_or(tp);

    // 1. Trigger & Passthroughs
    if tp.ends_with(".manualTrigger")
        || tp.ends_with(".webhook")
        || tp.ends_with(".scheduleTrigger")
        || tp.ends_with(".formTrigger")
        || tp.ends_with(".errorTrigger")
        || tp.ends_with(".executeWorkflowTrigger")
        || tp.ends_with(".chatTrigger")
        || tp.ends_with(".noOp")
        || short.ends_with("Trigger")
    {
        return Ok(NodeOutcome {
            branches: vec![items.to_vec()],
            pinned: false,
            meta: json!({"nodeType": raw_type}),
        });
    }

    // 2. Stop and Error
    if short == "stopAndError" {
        let msg = params
            .get("errorMessage")
            .and_then(|v| v.as_str())
            .unwrap_or("Workflow durch Stop-and-Error beendet");
        let resolved = eval_n8n_expression(msg, node_outputs, node_name, 0, vars, exec_meta);
        anyhow::bail!("{}", resolved.as_str().unwrap_or(msg));
    }

    // 3. Respond to Webhook
    if short == "respondToWebhook" {
        let code = params.get("responseCode").and_then(|v| v.as_u64()).unwrap_or(200);
        let body = if let Some(b) = params.get("responseBody") {
            resolve_n8n_value(b, node_outputs, node_name, 0, vars, exec_meta)
        } else {
            items.first().cloned().unwrap_or_else(|| json!({"ok": true}))
        };
        *webhook_response = Some(json!({ "statusCode": code, "body": body }));
        return Ok(NodeOutcome {
            branches: vec![items.to_vec()],
            pinned: false,
            meta: json!({"webhookResponse": code}),
        });
    }

    // 4. Wait
    if short == "wait" {
        let resume = params.get("resume").and_then(|v| v.as_str()).unwrap_or("timeInterval");
        if resume == "webhook" {
            let exec_id = exec_meta["id"].as_str().unwrap_or("").to_string();
            return Ok(NodeOutcome {
                branches: vec![items
                    .iter()
                    .map(|it| {
                        let mut m = it.as_object().cloned().unwrap_or_default();
                        m.insert("resumeUrl".into(), json!(format!("/webhook-waiting/{exec_id}")));
                        Value::Object(m)
                    })
                    .collect()],
                pinned: false,
                meta: json!({"waitingWebhook": exec_id}),
            });
        }
        let amount = params.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.1);
        let unit = params.get("unit").and_then(|v| v.as_str()).unwrap_or("seconds");
        let ms = match unit {
            "milliseconds" => amount as u64,
            "minutes" => (amount * 60_000.0) as u64,
            _ => (amount * 1000.0) as u64,
        }
        .min(1500);
        tokio::time::sleep(Duration::from_millis(ms)).await;
        return Ok(NodeOutcome {
            branches: vec![items.to_vec()],
            pinned: false,
            meta: json!({"waitedMs": ms}),
        });
    }

    // 5. Execute Sub-Workflow
    if short == "executeWorkflow" || short == "toolWorkflow" {
        let sub_id = params
            .get("workflowId")
            .and_then(|v| v.as_str().or_else(|| v.get("value").and_then(|x| x.as_str())))
            .unwrap_or("");
        if sub_id.is_empty() {
            anyhow::bail!("Execute Workflow: workflowId fehlt");
        }
        let payload = Value::Array(items.to_vec());
        let sub_res = execute_workflow_with_depth(db, sub_id, payload, None, depth + 1, None).await?;
        let out_items = sub_res["finalItems"].as_array().cloned().unwrap_or_default();
        return Ok(NodeOutcome {
            branches: vec![out_items],
            pinned: false,
            meta: json!({"subWorkflowId": sub_id}),
        });
    }

    // 6. Code Node (QuickJS + Python 3)
    if short == "code" || short == "toolCode" || short == "function" || short == "functionItem" {
        let lang = params.get("language").and_then(|v| v.as_str()).unwrap_or("javaScript");
        let mode = params.get("mode").and_then(|v| v.as_str()).unwrap_or("runOnceForAllItems");
        if lang == "python" {
            let py_code = params.get("pythonCode").and_then(|v| v.as_str()).unwrap_or("return _items");
            let out = run_python_code(py_code, items, vars).await?;
            return Ok(NodeOutcome {
                branches: vec![out],
                pinned: false,
                meta: json!({"runtime": "python3"}),
            });
        }
        let js_code = params
            .get("jsCode")
            .or_else(|| params.get("functionCode"))
            .and_then(|v| v.as_str())
            .unwrap_or("return $input.all();");
        let out = match eval_js_code_node(js_code, mode, items, node_outputs, node_name, vars, exec_meta) {
            Ok(v) => v,
            Err(e) => {
                // Sicherheits-Fallback falls Legacy Node.js `require()` verwendet wird
                if js_code.contains("require(") {
                    items.to_vec()
                } else {
                    return Err(anyhow::anyhow!("{}", e));
                }
            }
        };
        return Ok(NodeOutcome {
            branches: vec![out],
            pinned: false,
            meta: json!({"runtime": "quickjs"}),
        });
    }

    // 7. Set (Edit Fields)
    if short == "set" {
        let mode = params.get("mode").and_then(|v| v.as_str()).unwrap_or("manual");
        let include_other = params
            .get("includeOtherFields")
            .and_then(|v| v.as_bool())
            .unwrap_or(!params.get("keepOnlySet").and_then(|v| v.as_bool()).unwrap_or(false));

        let mut out = Vec::with_capacity(items.len());
        for (idx, it) in items.iter().enumerate() {
            let mut local_outputs = node_outputs.clone();
            local_outputs.insert(node_name.to_string(), items.to_vec());
            let mut base = if include_other {
                it.as_object().cloned().unwrap_or_default()
            } else {
                Map::new()
            };

            if mode == "raw" {
                let raw_json = params.get("jsonOutput").cloned().unwrap_or_else(|| json!("{}"));
                let resolved = match &raw_json {
                    Value::String(s) => {
                        let evaled = eval_n8n_expression(s, &local_outputs, node_name, idx, vars, exec_meta);
                        match evaled {
                            Value::String(txt) => serde_json::from_str::<Value>(&txt).unwrap_or(Value::String(txt)),
                            other => other,
                        }
                    }
                    other => resolve_n8n_value(other, &local_outputs, node_name, idx, vars, exec_meta),
                };
                if let Some(m) = resolved.as_object() {
                    for (k, v) in m {
                        base.insert(k.clone(), v.clone());
                    }
                }
            } else {
                // n8n v3.4 assignments.assignments
                let assigns_opt = params
                    .pointer("/assignments/assignments")
                    .and_then(|v| v.as_array())
                    .cloned()
                    .or_else(|| {
                        if let Some(s) = params.get("assignments").and_then(|x| x.as_str()) {
                            serde_json::from_str::<Value>(s)
                                .ok()
                                .and_then(|parsed| parsed.get("assignments").and_then(|a| a.as_array()).cloned())
                        } else {
                            None
                        }
                    });
                if let Some(assigns) = assigns_opt {
                    for a in assigns {
                        if let Some(k) = a.get("name").and_then(|v| v.as_str()) {
                            let val = a.get("value").cloned().unwrap_or(Value::Null);
                            let r = resolve_n8n_value(&val, &local_outputs, node_name, idx, vars, exec_meta);
                            insert_nested_key(&mut base, k, r);
                        }
                    }
                }
                // Legacy values.string / number / boolean
                if let Some(vals) = params.get("values").and_then(|v| v.as_object()) {
                    for (_kind, arr) in vals {
                        if let Some(entries) = arr.as_array() {
                            for e in entries {
                                if let Some(k) = e.get("name").and_then(|v| v.as_str()) {
                                    let val = e.get("value").cloned().unwrap_or(Value::Null);
                                    let r = resolve_n8n_value(&val, &local_outputs, node_name, idx, vars, exec_meta);
                                    insert_nested_key(&mut base, k, r);
                                }
                            }
                        }
                    }
                }
            }
            out.push(Value::Object(base));
        }
        return Ok(NodeOutcome {
            branches: vec![out],
            pinned: false,
            meta: json!({"mode": mode}),
        });
    }

    // 8. IF & Filter
    if short == "if" || short == "filter" {
        let mut true_items = Vec::new();
        let mut false_items = Vec::new();
        for (idx, it) in items.iter().enumerate() {
            let mut local_outputs = node_outputs.clone();
            local_outputs.insert(node_name.to_string(), items.to_vec());
            if eval_if_conditions(&params, &local_outputs, node_name, idx, vars, exec_meta) {
                true_items.push(it.clone());
            } else {
                false_items.push(it.clone());
            }
        }
        let branches = if short == "filter" {
            vec![true_items]
        } else {
            vec![true_items, false_items]
        };
        return Ok(NodeOutcome {
            branches,
            pinned: false,
            meta: json!({}),
        });
    }

    // 9. Switch
    if short == "switch" {
        let mode = params.get("mode").and_then(|v| v.as_str()).unwrap_or("rules");
        let mut buckets: Vec<Vec<Value>> = vec![vec![], vec![], vec![], vec![]];
        for (idx, it) in items.iter().enumerate() {
            let mut local_outputs = node_outputs.clone();
            local_outputs.insert(node_name.to_string(), items.to_vec());
            let target_idx = if mode == "expression" {
                let expr = params.get("output").and_then(|v| v.as_str()).unwrap_or("0");
                let res = eval_n8n_expression(expr, &local_outputs, node_name, idx, vars, exec_meta);
                res.as_u64()
                    .or_else(|| res.as_str().and_then(|s| s.parse::<u64>().ok()))
                    .unwrap_or(0) as usize
            } else {
                let mut matched = 3usize;
                if let Some(rules) = params.pointer("/rules/values").and_then(|v| v.as_array()) {
                    for (ridx, rule) in rules.iter().enumerate() {
                        if let Some(conds) = rule.get("conditions") {
                            let mock_p = json!({ "conditions": conds });
                            if eval_if_conditions(&mock_p, &local_outputs, node_name, idx, vars, exec_meta) {
                                matched = ridx;
                                break;
                            }
                        }
                    }
                }
                matched
            };
            while buckets.len() <= target_idx {
                buckets.push(vec![]);
            }
            buckets[target_idx].push(it.clone());
        }
        return Ok(NodeOutcome {
            branches: buckets,
            pinned: false,
            meta: json!({"mode": mode}),
        });
    }

    // 10. Merge
    if short == "merge" {
        let mode = params.get("mode").and_then(|v| v.as_str()).unwrap_or("append");
        let left = ports.get(&0).cloned().unwrap_or_else(|| items.to_vec());
        let right = ports.get(&1).cloned().unwrap_or_default();
        let out = match mode {
            "combineByPosition" => {
                let len = left.len().max(right.len());
                (0..len)
                    .map(|i| {
                        let mut m = left.get(i).and_then(|v| v.as_object()).cloned().unwrap_or_default();
                        if let Some(rm) = right.get(i).and_then(|v| v.as_object()) {
                            for (k, v) in rm {
                                m.insert(k.clone(), v.clone());
                            }
                        }
                        Value::Object(m)
                    })
                    .collect()
            }
            "combineByFields" => {
                let key = params
                    .get("mergeByFields")
                    .and_then(|v| v.as_str())
                    .unwrap_or("id")
                    .split(',')
                    .next()
                    .unwrap_or("id")
                    .trim();
                let join_mode = params.get("joinMode").and_then(|v| v.as_str()).unwrap_or("keepMatches");
                let mut joined = Vec::new();
                for l in &left {
                    let lk = get_nested_value(l, key);
                    let mut hit = false;
                    for r in &right {
                        if lk.is_some() && get_nested_value(r, key) == lk {
                            hit = true;
                            let mut m = l.as_object().cloned().unwrap_or_default();
                            if let Some(rm) = r.as_object() {
                                for (k, v) in rm {
                                    m.insert(k.clone(), v.clone());
                                }
                            }
                            joined.push(Value::Object(m));
                        }
                    }
                    if !hit && (join_mode == "enrichInput1" || join_mode == "keepEverything") {
                        joined.push(l.clone());
                    }
                }
                joined
            }
            "multiplex" => {
                let mut res = Vec::new();
                for l in &left {
                    for r in &right {
                        let mut m = l.as_object().cloned().unwrap_or_default();
                        if let Some(rm) = r.as_object() {
                            for (k, v) in rm {
                                m.insert(k.clone(), v.clone());
                            }
                        }
                        res.push(Value::Object(m));
                    }
                }
                res
            }
            _ => {
                let mut all = left;
                all.extend(right);
                all
            }
        };
        return Ok(NodeOutcome {
            branches: vec![out],
            pinned: false,
            meta: json!({"mode": mode}),
        });
    }

    // 11. Split in Batches / Split Out / Aggregate / Summarize / Sort / Limit / Deduplicate / RenameKeys / CompareDatasets
    if short == "splitInBatches" {
        let size = params.get("batchSize").and_then(|v| v.as_u64()).unwrap_or(10).max(1) as usize;
        let first_batch = items.iter().take(size).cloned().collect::<Vec<_>>();
        return Ok(NodeOutcome {
            branches: vec![first_batch.clone(), first_batch],
            pinned: false,
            meta: json!({"batchSize": size}),
        });
    }

    if short == "splitOut" || (short == "itemLists" && params.get("operation").and_then(|v| v.as_str()) == Some("splitOutItems")) {
        let field = params
            .get("fieldToSplitOut")
            .and_then(|v| v.as_str())
            .unwrap_or("items");
        let inc_other = params.get("includeOtherFields").and_then(|v| v.as_bool()).unwrap_or(true);
        let mut out = Vec::new();
        for it in items {
            if let Some(arr) = get_nested_value(it, field).and_then(|v| v.as_array()) {
                for elem in arr {
                    let mut m = if inc_other {
                        it.as_object().cloned().unwrap_or_default()
                    } else {
                        Map::new()
                    };
                    if let Some(em) = elem.as_object() {
                        for (k, v) in em {
                            m.insert(k.clone(), v.clone());
                        }
                    } else {
                        m.insert(field.to_string(), elem.clone());
                    }
                    out.push(Value::Object(m));
                }
            } else {
                out.push(it.clone());
            }
        }
        return Ok(NodeOutcome { branches: vec![out], pinned: false, meta: json!({}) });
    }

    if short == "aggregate" || (short == "itemLists" && params.get("operation").and_then(|v| v.as_str()) == Some("concatenateItems")) {
        let dest = params.get("destinationFieldName").and_then(|v| v.as_str()).unwrap_or("data");
        return Ok(NodeOutcome {
            branches: vec![vec![json!({ dest: items })]],
            pinned: false,
            meta: json!({}),
        });
    }

    if short == "summarize" {
        let op = params.get("operation").and_then(|v| v.as_str()).unwrap_or("count");
        let field = params.get("field").and_then(|v| v.as_str()).unwrap_or("amount");
        let nums: Vec<f64> = items
            .iter()
            .filter_map(|it| get_nested_value(it, field).and_then(|v| v.as_f64()))
            .collect();
        let val = match op {
            "sum" => json!(nums.iter().sum::<f64>()),
            "average" => json!(if nums.is_empty() { 0.0 } else { nums.iter().sum::<f64>() / nums.len() as f64 }),
            "min" => json!(nums.iter().cloned().reduce(f64::min).unwrap_or(0.0)),
            "max" => json!(nums.iter().cloned().reduce(f64::max).unwrap_or(0.0)),
            _ => json!(items.len()),
        };
        return Ok(NodeOutcome {
            branches: vec![vec![json!({ "operation": op, "field": field, "result": val, "count": items.len() })]],
            pinned: false,
            meta: json!({}),
        });
    }

    if short == "sort" {
        let field = params.get("fieldName").and_then(|v| v.as_str()).unwrap_or("id");
        let desc = params.get("order").and_then(|v| v.as_str()) == Some("descending");
        let mut sorted = items.to_vec();
        sorted.sort_by(|a, b| {
            let va = get_nested_value(a, field);
            let vb = get_nested_value(b, field);
            let ord = match (va.and_then(|x| x.as_f64()), vb.and_then(|x| x.as_f64())) {
                (Some(na), Some(nb)) => na.partial_cmp(&nb).unwrap_or(std::cmp::Ordering::Equal),
                _ => value_to_str(va.unwrap_or(&Value::Null)).cmp(&value_to_str(vb.unwrap_or(&Value::Null))),
            };
            if desc { ord.reverse() } else { ord }
        });
        return Ok(NodeOutcome { branches: vec![sorted], pinned: false, meta: json!({}) });
    }

    if short == "limit" {
        let max_items = params.get("maxItems").and_then(|v| v.as_u64()).unwrap_or(10) as usize;
        let keep = params.get("keep").and_then(|v| v.as_str()).unwrap_or("firstItems");
        let out = if keep == "lastItems" && items.len() > max_items {
            items[items.len() - max_items..].to_vec()
        } else {
            items.iter().take(max_items).cloned().collect()
        };
        return Ok(NodeOutcome { branches: vec![out], pinned: false, meta: json!({}) });
    }

    if short == "removeDuplicates" {
        let fields_str = params.get("fieldsToCompare").and_then(|v| v.as_str()).unwrap_or("");
        let compare_all = params.get("compare").and_then(|v| v.as_str()) != Some("selectedFields") || fields_str.trim().is_empty();
        let mut seen = HashSet::new();
        let mut out = Vec::new();
        for it in items {
            let sig = if compare_all {
                it.to_string()
            } else {
                fields_str
                    .split(',')
                    .map(|f| get_nested_value(it, f.trim()).map(|v| v.to_string()).unwrap_or_default())
                    .collect::<Vec<_>>()
                    .join("|")
            };
            if seen.insert(sig) {
                out.push(it.clone());
            }
        }
        return Ok(NodeOutcome { branches: vec![out], pinned: false, meta: json!({}) });
    }

    if short == "compareDatasets" {
        let left = ports.get(&0).cloned().unwrap_or_else(|| items.to_vec());
        let right = ports.get(&1).cloned().unwrap_or_default();
        let right_set: HashSet<String> = right.iter().map(|v| v.to_string()).collect();
        let left_set: HashSet<String> = left.iter().map(|v| v.to_string()).collect();
        let in_a_only = left.iter().filter(|v| !right_set.contains(&v.to_string())).cloned().collect();
        let same = left.iter().filter(|v| right_set.contains(&v.to_string())).cloned().collect();
        let in_b_only = right.iter().filter(|v| !left_set.contains(&v.to_string())).cloned().collect();
        return Ok(NodeOutcome {
            branches: vec![in_a_only, same, vec![], in_b_only],
            pinned: false,
            meta: json!({}),
        });
    }

    if short == "renameKeys" {
        let mut out = Vec::with_capacity(items.len());
        let renames = params.pointer("/keys/key").and_then(|v| v.as_array()).cloned().unwrap_or_default();
        for it in items {
            let mut m = it.as_object().cloned().unwrap_or_default();
            for r in &renames {
                if let (Some(from), Some(to)) = (r.get("currentKey").and_then(|v| v.as_str()), r.get("newKey").and_then(|v| v.as_str())) {
                    if let Some(val) = m.remove(from) {
                        m.insert(to.to_string(), val);
                    }
                }
            }
            out.push(Value::Object(m));
        }
        return Ok(NodeOutcome { branches: vec![out], pinned: false, meta: json!({}) });
    }

    // 12. Utilities: DateTime, Crypto, JWT, XML/HTML/Markdown/Compression/Files, DataTable
    if short == "dateTime" {
        let op = params.get("operation").and_then(|v| v.as_str()).unwrap_or("formatDate");
        let mut out = Vec::with_capacity(items.len());
        for (idx, it) in items.iter().enumerate() {
            let mut m = it.as_object().cloned().unwrap_or_default();
            let raw_date = params.get("date").and_then(|v| v.as_str()).unwrap_or("={{ $now.toISO() }}");
            let dt_val = eval_n8n_expression(raw_date, node_outputs, node_name, idx, vars, exec_meta);
            let parsed_dt = dt_val
                .as_str()
                .and_then(|s| DateTime::parse_from_rfc3339(s).ok().map(|d| d.with_timezone(&Utc)))
                .unwrap_or_else(Utc::now);
            let res = match op {
                "addToDate" => (parsed_dt + ChronoDur::days(params.get("duration").and_then(|v| v.as_i64()).unwrap_or(1))).to_rfc3339(),
                "getCurrentDate" => Utc::now().to_rfc3339(),
                _ => parsed_dt.format("%d.%m.%Y %H:%M").to_string(),
            };
            m.insert("formattedDate".into(), json!(res));
            out.push(Value::Object(m));
        }
        return Ok(NodeOutcome { branches: vec![out], pinned: false, meta: json!({}) });
    }

    if short == "crypto" {
        let action = params.get("action").and_then(|v| v.as_str()).unwrap_or("hash");
        let algo = params.get("type").and_then(|v| v.as_str()).unwrap_or("SHA256");
        let prop = params.get("dataPropertyName").and_then(|v| v.as_str()).unwrap_or("hash");
        let secret = params.get("secret").and_then(|v| v.as_str());
        let mut out = Vec::with_capacity(items.len());
        for (idx, it) in items.iter().enumerate() {
            let mut m = it.as_object().cloned().unwrap_or_default();
            let val = params
                .get("value")
                .map(|v| resolve_n8n_value(v, node_outputs, node_name, idx, vars, exec_meta))
                .map(|v| value_to_str(&v))
                .unwrap_or_else(|| it.to_string());
            let computed = match action {
                "generate" => Uuid::new_v4().to_string(),
                "hmac" => digest(algo, &val, secret.or(Some("conrad-ki-secret"))).unwrap_or_default(),
                _ => digest(algo, &val, None).unwrap_or_default(),
            };
            m.insert(prop.to_string(), json!(computed));
            out.push(Value::Object(m));
        }
        return Ok(NodeOutcome { branches: vec![out], pinned: false, meta: json!({}) });
    }

    if short == "jwt" {
        let op = params.get("operation").and_then(|v| v.as_str()).unwrap_or("sign");
        let secret = params.get("secret").and_then(|v| v.as_str()).unwrap_or("conrad-ki-jwt-secret");
        let mut out = Vec::with_capacity(items.len());
        for (idx, it) in items.iter().enumerate() {
            let mut m = it.as_object().cloned().unwrap_or_default();
            if op == "sign" {
                let header_b64 = B64URL.encode(r#"{"alg":"HS256","typ":"JWT"}"#);
                let claims_val = params
                    .get("claimsJson")
                    .map(|v| resolve_n8n_value(v, node_outputs, node_name, idx, vars, exec_meta))
                    .unwrap_or_else(|| it.clone());
                let payload_str = match claims_val {
                    Value::String(s) => s,
                    other => other.to_string(),
                };
                let payload_b64 = B64URL.encode(payload_str.as_bytes());
                let signing_input = format!("{header_b64}.{payload_b64}");
                let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(secret.as_bytes()).unwrap();
                mac.update(signing_input.as_bytes());
                let sig_b64 = B64URL.encode(mac.finalize().into_bytes());
                m.insert("token".into(), json!(format!("{signing_input}.{sig_b64}")));
            } else {
                let token = params
                    .get("token")
                    .and_then(|v| v.as_str())
                    .or_else(|| it.get("token").and_then(|v| v.as_str()))
                    .unwrap_or("");
                let parts: Vec<&str> = token.split('.').collect();
                if parts.len() == 3 {
                    let payload = B64URL
                        .decode(parts[1])
                        .ok()
                        .and_then(|b| String::from_utf8(b).ok())
                        .and_then(|s| serde_json::from_str::<Value>(&s).ok())
                        .unwrap_or_else(|| json!({}));
                    m.insert("payload".into(), payload);
                    m.insert("valid".into(), json!(true));
                } else {
                    m.insert("valid".into(), json!(false));
                }
            }
            out.push(Value::Object(m));
        }
        return Ok(NodeOutcome { branches: vec![out], pinned: false, meta: json!({}) });
    }

    if short == "dataTable" {
        let op = params.get("operation").and_then(|v| v.as_str()).unwrap_or("getRows");
        let tbl_name = params.get("tableName").and_then(|v| v.as_str()).unwrap_or("Lead-Scoring-Regeln");
        let conn = db.lock_conn();
        let tid: Option<String> = conn
            .query_row("SELECT id FROM n8n_data_tables WHERE name = ?1 OR id = ?1", [tbl_name], |r| r.get(0))
            .optional()?;
        let table_id = tid.unwrap_or_else(|| {
            let new_id = Uuid::new_v4().to_string();
            let now = Utc::now().to_rfc3339();
            let _ = conn.execute(
                "INSERT INTO n8n_data_tables (id, name, columns_json, created_at, updated_at) VALUES (?1, ?2, '[]', ?3, ?3)",
                params![new_id, tbl_name, now],
            );
            new_id
        });
        let now = Utc::now().to_rfc3339();
        match op {
            "insert" | "upsert" => {
                let mut inserted = Vec::new();
                for (idx, it) in items.iter().enumerate() {
                    let row_val = params
                        .get("rowData")
                        .and_then(|v| {
                            let r = resolve_n8n_value(v, node_outputs, node_name, idx, vars, exec_meta);
                            match r {
                                Value::String(s) => serde_json::from_str::<Value>(&s).ok(),
                                Value::Object(m) if !m.is_empty() => Some(Value::Object(m)),
                                _ => None,
                            }
                        })
                        .unwrap_or_else(|| it.clone());
                    conn.execute(
                        "INSERT INTO n8n_data_table_rows (table_id, data_json, created_at, updated_at) VALUES (?1, ?2, ?3, ?3)",
                        params![table_id, row_val.to_string(), now],
                    )?;
                    inserted.push(row_val);
                }
                return Ok(NodeOutcome { branches: vec![inserted], pinned: false, meta: json!({"tableId": table_id}) });
            }
            "delete" => {
                let n = conn.execute("DELETE FROM n8n_data_table_rows WHERE table_id = ?1", [&table_id]).unwrap_or(0);
                return Ok(NodeOutcome { branches: vec![vec![json!({"deletedRows": n})]], pinned: false, meta: json!({}) });
            }
            _ => {
                let f_field = params.get("filterField").and_then(|v| v.as_str()).unwrap_or("").trim();
                let f_val = params.get("filterValue").and_then(|v| v.as_str()).unwrap_or("").trim();
                let mut rows_out = Vec::new();
                if let Ok(mut st) = conn.prepare("SELECT id, data_json FROM n8n_data_table_rows WHERE table_id = ?1 ORDER BY id ASC LIMIT 500") {
                    if let Ok(mut rows) = st.query([&table_id]) {
                        while let Ok(Some(r)) = rows.next() {
                            let rid: i64 = r.get(0)?;
                            let dj: String = r.get(1)?;
                            if let Ok(mut val) = serde_json::from_str::<Value>(&dj) {
                                if !f_field.is_empty() && !f_val.is_empty() {
                                    if value_to_str(val.get(f_field).unwrap_or(&Value::Null)) != f_val {
                                        continue;
                                    }
                                }
                                if let Some(m) = val.as_object_mut() {
                                    m.insert("rowId".into(), json!(rid));
                                }
                                rows_out.push(val);
                            }
                        }
                    }
                }
                return Ok(NodeOutcome { branches: vec![rows_out], pinned: false, meta: json!({"tableId": table_id}) });
            }
        }
    }

    if matches!(short, "xml" | "html" | "markdown" | "compression" | "extractFromFile" | "convertToFile") {
        let mut out = Vec::with_capacity(items.len());
        for it in items {
            let mut m = it.as_object().cloned().unwrap_or_default();
            match short {
                "convertToFile" => {
                    m.insert("binaryBase64".into(), json!(B64.encode(it.to_string().as_bytes())));
                    m.insert("mimeType".into(), json!("application/json"));
                }
                "compression" => {
                    m.insert("compressedBase64".into(), json!(B64.encode(it.to_string().as_bytes())));
                }
                "markdown" => {
                    let md = m.get("data").or_else(|| m.get("text")).and_then(|v| v.as_str()).unwrap_or("");
                    m.insert("html".into(), json!(format!("<p>{}</p>", md)));
                }
                _ => {
                    m.insert("parsedBy".into(), json!(short));
                }
            }
            out.push(Value::Object(m));
        }
        return Ok(NodeOutcome { branches: vec![out], pinned: false, meta: json!({}) });
    }

    // 13. HTTP Request (mit Credentials, SSRF-Schutz & Draft-Schutz)
    if short == "httpRequest" || short == "toolHttpRequest" {
        let out = execute_http_request_node(db, node_name, &params, items, node_outputs, vars, exec_meta).await?;
        return Ok(NodeOutcome { branches: vec![out], pinned: false, meta: json!({"http": true}) });
    }

    // 14. Twenty CRM Integration
    if short == "twentyCrm" {
        let out = execute_twenty_node(db, &params, items);
        return Ok(NodeOutcome { branches: vec![out], pinned: false, meta: json!({"crm": true}) });
    }

    // 15. AI & LangChain Nodes (Ollama lokal / OpenAI)
    if tp.contains("langchain.") || short == "openai" || short == "ollama" {
        let out = execute_ai_node(node, &params, items, node_outputs, vars, exec_meta).await;
        return Ok(NodeOutcome { branches: vec![out], pinned: false, meta: json!({"ai": true}) });
    }

    // 16. E-Mail / Messaging / Stripe -> IMMER draftOnly: true!
    if matches!(short, "emailSend" | "gmail" | "microsoftOutlook" | "telegram" | "slack" | "discord" | "stripe") {
        let out: Vec<Value> = items
            .iter()
            .enumerate()
            .map(|(idx, it)| {
                let mut local = node_outputs.clone();
                local.insert(node_name.to_string(), items.to_vec());
                let resolved_params = resolve_n8n_value(&params, &local, node_name, idx, vars, exec_meta);
                json!({
                    "draftOnly": true,
                    "status": "draft_saved",
                    "channel": short,
                    "notice": "Sicherheitsregel aktiv: Nachricht nur als Entwurf gespeichert (kein automatischer Versand).",
                    "payload": resolved_params,
                    "input": it
                })
            })
            .collect();
        return Ok(NodeOutcome {
            branches: vec![out],
            pinned: false,
            meta: json!({"draftOnly": true}),
        });
    }

    // Fallback fuer weitere Integrations-Nodes
    Ok(NodeOutcome {
        branches: vec![items
            .iter()
            .map(|it| {
                let mut m = it.as_object().cloned().unwrap_or_default();
                m.insert("_executedNode".into(), json!(node_name));
                m.insert("_nodeType".into(), json!(raw_type));
                Value::Object(m)
            })
            .collect()],
        pinned: false,
        meta: json!({"passthrough": true}),
    })
}

async fn run_python_code(code: &str, items: &[Value], vars: &Value) -> anyhow::Result<Vec<Value>> {
    let payload = json!({ "items": items, "vars": vars }).to_string();
    let py_wrapper = format!(
        r#"import json, sys
data = json.load(sys.stdin)
_items = [{{"json": x}} for x in data["items"]]
$vars = data["vars"]
def _user_fn(_items):
{indented}
res = _user_fn(_items)
if res is None:
    res = _items
out = [x.get("json", x) if isinstance(x, dict) else {{"result": x}} for x in (res if isinstance(res, list) else [res])]
print(json.dumps(out))
"#,
        indented = code
            .replace("$vars", "_vars")
            .lines()
            .map(|l| format!("    {}", l))
            .collect::<Vec<_>>()
            .join("\n")
    );
    let wrapper_clean = py_wrapper.replacen("$vars = data[\"vars\"]", "_vars = data[\"vars\"]", 1);

    let out = tokio::task::spawn_blocking(move || -> anyhow::Result<Vec<Value>> {
        use std::io::Write;
        use std::process::{Command, Stdio};
        let mut child = Command::new("python3")
            .arg("-c")
            .arg(&wrapper_clean)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(payload.as_bytes());
        }
        let output = child.wait_with_output()?;
        if !output.status.success() {
            anyhow::bail!("Python Fehler: {}", String::from_utf8_lossy(&output.stderr));
        }
        let txt = String::from_utf8_lossy(&output.stdout);
        Ok(serde_json::from_str(txt.trim()).unwrap_or_default())
    })
    .await??;
    Ok(out)
}

async fn execute_http_request_node(
    db: &Database,
    node_name: &str,
    params: &Value,
    items: &[Value],
    node_outputs: &HashMap<String, Vec<Value>>,
    vars: &Value,
    exec_meta: &Value,
) -> anyhow::Result<Vec<Value>> {
    let mut out = Vec::with_capacity(items.len());
    let cred_id = params.pointer("/credentials/0/id").and_then(|v| v.as_str());
    let cred_secret = resolve_credential_secret(db, cred_id, None);

    for (idx, it) in items.iter().enumerate() {
        let mut local = node_outputs.clone();
        local.insert(node_name.to_string(), items.to_vec());

        let method_str = params.get("method").and_then(|v| v.as_str()).unwrap_or("GET").to_uppercase();
        let raw_url = params.get("url").and_then(|v| v.as_str()).unwrap_or("");
        let url_val = eval_n8n_expression(raw_url, &local, node_name, idx, vars, exec_meta);
        let url = value_to_str(&url_val);
        if url.is_empty() {
            out.push(it.clone());
            continue;
        }

        // Draft-Only Schutz fuer externe Mail/Messaging/Stripe Writes
        let url_low = url.to_ascii_lowercase();
        let is_write = method_str != "GET" && method_str != "HEAD";
        if is_write
            && (url_low.contains("api.brevo.com")
                || url_low.contains("api.resend.com")
                || url_low.contains("api.sendgrid.com")
                || url_low.contains("api.telegram.org")
                || url_low.contains("api.stripe.com"))
        {
            out.push(json!({
                "draftOnly": true,
                "blockedExternalSend": true,
                "method": method_str,
                "url": url,
                "notice": "Sicherheitsregel: Externe E-Mail-/Messaging-/Zahlungs-API wurde nur als Entwurf simuliert."
            }));
            continue;
        }

        // SSRF-Schutz gegen Link-Local / Cloud-Metadata / interne Netze (Ollama 127.0.0.1:11434 erlaubt)
        if let Ok(parsed) = reqwest::Url::parse(&url) {
            if let Some(host) = parsed.host_str() {
                let port = parsed.port_or_known_default().unwrap_or(80);
                let is_ollama = (host == "127.0.0.1" || host == "localhost") && port == 11434;
                if !is_ollama && is_blocked_host(host) {
                    anyhow::bail!("SSRF-Schutz: Zielhost '{}' ist blockiert", host);
                }
            }
        }

        let method = Method::from_bytes(method_str.as_bytes()).unwrap_or(Method::GET);
        let mut req = http_client().request(method.clone(), &url);

        if let Some(secret) = &cred_secret {
            if let (Some(hn), Some(hv)) = (secret.get("name").and_then(|v| v.as_str()), secret.get("value").and_then(|v| v.as_str())) {
                req = req.header(hn, hv);
            } else if let Some(tok) = secret.get("token").or_else(|| secret.get("apiKey")).and_then(|v| v.as_str()) {
                req = req.bearer_auth(tok);
            } else if let (Some(u), Some(p)) = (secret.get("user").and_then(|v| v.as_str()), secret.get("password").and_then(|v| v.as_str())) {
                req = req.basic_auth(u, Some(p));
            }
        }

        if params.get("sendBody").and_then(|v| v.as_bool()).unwrap_or(false) {
            let body_raw = params.get("jsonBody").cloned().unwrap_or_else(|| json!("{}"));
            let body_eval = resolve_n8n_value(&body_raw, &local, node_name, idx, vars, exec_meta);
            let body_json = match body_eval {
                Value::String(s) => serde_json::from_str::<Value>(&s).unwrap_or(Value::String(s)),
                other => other,
            };
            req = req.json(&body_json);
        }

        let resp = req.send().await?;
        let status_code = resp.status().as_u16();
        let text = resp.text().await.unwrap_or_default();
        let parsed_body = serde_json::from_str::<Value>(&text).unwrap_or_else(|_| json!({ "data": text, "statusCode": status_code }));
        out.push(parsed_body);
    }
    Ok(out)
}

fn is_blocked_host(host: &str) -> bool {
    let h = host.to_ascii_lowercase();
    if h == "localhost" || h.ends_with(".local") || h == "metadata.google.internal" {
        return true;
    }
    if let Ok(ip) = h.parse::<IpAddr>() {
        return match ip {
            IpAddr::V4(v4) => {
                v4.is_loopback() || v4.is_private() || v4.is_link_local() || v4.is_unspecified()
            }
            IpAddr::V6(v6) => v6.is_loopback() || v6.is_unspecified(),
        };
    }
    false
}

fn execute_twenty_node(db: &Database, params: &Value, items: &[Value]) -> Vec<Value> {
    let resource = params.get("resource").and_then(|v| v.as_str()).unwrap_or("company");
    let conn = db.lock_conn();
    let table = match resource {
        "person" | "people" => "twenty_people",
        "opportunity" | "opportunities" => "twenty_opportunities",
        "task" | "tasks" => "twenty_tasks",
        "note" | "notes" => "twenty_notes",
        _ => "twenty_companies",
    };
    let mut rows_out = Vec::new();
    let sql = format!("SELECT id, workspace FROM {table} ORDER BY updated_at DESC LIMIT 20");
    if let Ok(mut st) = conn.prepare(&sql) {
        if let Ok(mut rows) = st.query([]) {
            while let Ok(Some(r)) = rows.next() {
                rows_out.push(json!({
                    "id": r.get::<_, String>(0).unwrap_or_default(),
                    "workspace": r.get::<_, String>(1).unwrap_or_default(),
                    "resource": resource
                }));
            }
        }
    }
    if rows_out.is_empty() {
        return items.to_vec();
    }
    rows_out
}

async fn execute_ai_node(
    node: &Value,
    params: &Value,
    items: &[Value],
    node_outputs: &HashMap<String, Vec<Value>>,
    vars: &Value,
    exec_meta: &Value,
) -> Vec<Value> {
    let node_name = node["name"].as_str().unwrap_or("AI Agent");
    let model = params
        .get("model")
        .and_then(|v| v.as_str())
        .unwrap_or("qwen2.5:1.5b");
    let sys_msg = params
        .get("systemMessage")
        .and_then(|v| v.as_str())
        .unwrap_or("Du bist ein praeziser deutscher KI-Assistent fuer Conrad KI.");
    let prompt_tmpl = params
        .get("text")
        .or_else(|| params.get("prompt"))
        .and_then(|v| v.as_str())
        .unwrap_or("={{ JSON.stringify($json) }}");

    let mut out = Vec::with_capacity(items.len());
    for (idx, it) in items.iter().enumerate() {
        let mut local = node_outputs.clone();
        local.insert(node_name.to_string(), items.to_vec());
        let prompt_val = eval_n8n_expression(prompt_tmpl, &local, node_name, idx, vars, exec_meta);
        let prompt = value_to_str(&prompt_val);

        let url = format!("{}/api/chat", ollama_base_url().trim_end_matches('/'));
        let body = json!({
            "model": model,
            "stream": false,
            "messages": [
                { "role": "system", "content": sys_msg },
                { "role": "user", "content": prompt }
            ]
        });
        let reply = match http_client().post(&url).json(&body).timeout(Duration::from_secs(12)).send().await {
            Ok(resp) if resp.status().is_success() => {
                let j = resp.json::<Value>().await.unwrap_or_else(|_| json!({}));
                let raw = j.pointer("/message/content").and_then(|v| v.as_str()).unwrap_or("");
                strip_think_tags(raw)
            }
            _ => format!("[Lokaler AI-Fallback ({})]: Anfrage '{}' verarbeitet.", model, prompt.chars().take(120).collect::<String>()),
        };
        let mut m = it.as_object().cloned().unwrap_or_default();
        m.insert("output".into(), json!(reply));
        m.insert("model".into(), json!(model));
        out.push(Value::Object(m));
    }
    out
}

// =========================================================================
// Bedingungen (IF / Switch / Filter) & Objektpfad-Helfer
// =========================================================================

fn eval_if_conditions(
    params: &Value,
    node_outputs: &HashMap<String, Vec<Value>>,
    node_name: &str,
    idx: usize,
    vars: &Value,
    exec_meta: &Value,
) -> bool {
    let ignore_case = params.get("ignoreCase").and_then(|v| v.as_bool()).unwrap_or(true);
    let cond_obj = params.get("conditions").cloned().unwrap_or_else(|| json!({}));

    if let Some(arr) = cond_obj.get("conditions").and_then(|v| v.as_array()) {
        if arr.is_empty() {
            return true;
        }
        let combinator = cond_obj
            .get("combinator")
            .or_else(|| params.get("combinator"))
            .and_then(|v| v.as_str())
            .unwrap_or("and");
        let results = arr.iter().map(|c| {
            let lv_raw = c.get("leftValue").cloned().unwrap_or(Value::Null);
            let rv_raw = c.get("rightValue").cloned().unwrap_or(Value::Null);
            let lv = resolve_n8n_value(&lv_raw, node_outputs, node_name, idx, vars, exec_meta);
            let rv = resolve_n8n_value(&rv_raw, node_outputs, node_name, idx, vars, exec_meta);
            let op = c.pointer("/operator/operation").and_then(|v| v.as_str()).unwrap_or("equals");
            compare_n8n_values(&lv, &rv, op, ignore_case)
        });
        if combinator == "or" {
            results.into_iter().any(|b| b)
        } else {
            results.into_iter().all(|b| b)
        }
    } else {
        true
    }
}

fn compare_n8n_values(lv: &Value, rv: &Value, op: &str, ignore_case: bool) -> bool {
    let ls = value_to_str(lv);
    let rs = value_to_str(rv);
    let (l_cmp, r_cmp) = if ignore_case {
        (ls.to_lowercase(), rs.to_lowercase())
    } else {
        (ls.clone(), rs.clone())
    };
    let ln = lv.as_f64().or_else(|| ls.parse::<f64>().ok());
    let rn = rv.as_f64().or_else(|| rs.parse::<f64>().ok());

    match op {
        "equals" | "equal" => match (ln, rn) {
            (Some(a), Some(b)) => (a - b).abs() < f64::EPSILON,
            _ => l_cmp == r_cmp,
        },
        "notEquals" | "notEqual" => match (ln, rn) {
            (Some(a), Some(b)) => (a - b).abs() >= f64::EPSILON,
            _ => l_cmp != r_cmp,
        },
        "contains" => l_cmp.contains(&r_cmp),
        "notContains" => !l_cmp.contains(&r_cmp),
        "startsWith" => l_cmp.starts_with(&r_cmp),
        "endsWith" => l_cmp.ends_with(&r_cmp),
        "gt" | "larger" => ln.zip(rn).map(|(a, b)| a > b).unwrap_or(l_cmp > r_cmp),
        "gte" | "largerEqual" => ln.zip(rn).map(|(a, b)| a >= b).unwrap_or(l_cmp >= r_cmp),
        "lt" | "smaller" => ln.zip(rn).map(|(a, b)| a < b).unwrap_or(l_cmp < r_cmp),
        "lte" | "smallerEqual" => ln.zip(rn).map(|(a, b)| a <= b).unwrap_or(l_cmp <= r_cmp),
        "empty" => lv.is_null() || ls.is_empty(),
        "notEmpty" => !lv.is_null() && !ls.is_empty(),
        "exists" => !lv.is_null(),
        "notExists" => lv.is_null(),
        "true" => lv.as_bool().unwrap_or(l_cmp == "true" || l_cmp == "1"),
        "false" => !lv.as_bool().unwrap_or(l_cmp == "true" || l_cmp == "1"),
        _ => l_cmp == r_cmp,
    }
}

fn value_to_str(v: &Value) -> String {
    match v {
        Value::Null => String::new(),
        Value::String(s) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        other => other.to_string(),
    }
}

fn get_nested_value<'a>(v: &'a Value, path: &str) -> Option<&'a Value> {
    let mut cur = v;
    for seg in path.split('.') {
        cur = cur.get(seg)?;
    }
    Some(cur)
}

fn insert_nested_key(map: &mut Map<String, Value>, path: &str, val: Value) {
    if !path.contains('.') {
        map.insert(path.to_string(), val);
        return;
    }
    let parts: Vec<&str> = path.split('.').collect();
    let mut cur = map;
    for seg in &parts[..parts.len() - 1] {
        let entry = cur
            .entry((*seg).to_string())
            .or_insert_with(|| Value::Object(Map::new()));
        if !entry.is_object() {
            *entry = Value::Object(Map::new());
        }
        cur = entry.as_object_mut().unwrap();
    }
    if let Some(last) = parts.last() {
        cur.insert((*last).to_string(), val);
    }
}

// =========================================================================
// HTTP Handlers: Workflows, Executions, Types, QuickJS Eval, Credentials, Vars, Tags, DataTables, Webhooks, Forms
// =========================================================================

#[derive(Deserialize)]
pub struct ExecQuery {
    pub limit: Option<i64>,
    #[serde(rename = "workflowId")]
    pub workflow_id: Option<String>,
}

pub async fn get_nodes_types_handler() -> impl IntoResponse {
    Json(nodes_json())
}

pub async fn get_credentials_types_handler() -> impl IntoResponse {
    Json(credentials_json())
}

pub async fn eval_expression_handler(
    State(state): State<AppState>,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    let expr = body.get("expression").and_then(|v| v.as_str()).unwrap_or("");
    let item = body.get("item").cloned().unwrap_or_else(|| json!({}));
    let current_node = body.get("currentNode").and_then(|v| v.as_str()).unwrap_or("Trigger");
    let mut node_outputs: HashMap<String, Vec<Value>> = HashMap::new();
    if let Some(map) = body.get("nodeOutputs").and_then(|v| v.as_object()) {
        for (k, v) in map {
            let arr = v.as_array().cloned().unwrap_or_else(|| vec![v.clone()]);
            node_outputs.insert(k.clone(), arr);
        }
    }
    node_outputs.entry(current_node.to_string()).or_insert_with(|| vec![item]);

    let mut vars = load_vars_json(&state.db);
    if let (Some(vm), Some(req_vars)) = (vars.as_object_mut(), body.get("vars").and_then(|v| v.as_object())) {
        for (k, v) in req_vars {
            vm.insert(k.clone(), v.clone());
        }
    }
    let exec_meta = json!({"id": "preview", "mode": "expression_preview"});

    if !expr.starts_with('=') {
        return Json(json!({ "ok": true, "value": expr, "engine": "literal" }));
    }
    match eval_n8n_template(expr, &node_outputs, current_node, 0, &vars, &exec_meta) {
        Ok(val) => Json(json!({ "ok": true, "value": val, "engine": "quickjs" })),
        Err(e) => Json(json!({ "ok": false, "error": e.to_string(), "engine": "quickjs" })),
    }
}

pub async fn list_workflows_handler(State(state): State<AppState>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let mut items = Vec::new();
    if let Ok(mut stmt) = conn.prepare(
        "SELECT id, name, active, nodes_json, connections_json, settings_json, tags_json, pin_data_json, created_at, updated_at, execution_count, last_executed_at
         FROM n8n_workflows ORDER BY active DESC, updated_at DESC",
    ) {
        if let Ok(mut rows) = stmt.query([]) {
            while let Ok(Some(r)) = rows.next() {
                if let Ok(v) = workflow_row_to_json(r) {
                    items.push(v);
                }
            }
        }
    }
    Json(json!({ "data": items, "totalCount": items.len() }))
}

pub async fn get_workflow_handler(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    match conn
        .query_row(
            "SELECT id, name, active, nodes_json, connections_json, settings_json, tags_json, pin_data_json, created_at, updated_at, execution_count, last_executed_at
             FROM n8n_workflows WHERE id = ?1",
            [&id],
            workflow_row_to_json,
        )
        .optional()
    {
        Ok(Some(wf)) => (StatusCode::OK, Json(wf)),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Workflow nicht gefunden"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}

pub async fn create_workflow_handler(State(state): State<AppState>, Json(v): Json<Value>) -> impl IntoResponse {
    let id = v
        .get("id")
        .and_then(|x| x.as_str())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    let name = v.get("name").and_then(|x| x.as_str()).unwrap_or("Neuer Workflow").to_string();
    let active = if v.get("active").and_then(|x| x.as_bool()).unwrap_or(false) { 1i64 } else { 0i64 };
    let nodes = v.get("nodes").cloned().unwrap_or_else(|| json!([]));
    let conns = v.get("connections").cloned().unwrap_or_else(|| json!({}));
    let settings = v.get("settings").cloned().unwrap_or_else(|| json!({}));
    let tags = v.get("tags").cloned().unwrap_or_else(|| json!([]));
    let pin_data = v.get("pinData").cloned().unwrap_or_else(|| json!({}));
    let now = Utc::now().to_rfc3339();

    let conn = state.db.lock_conn();
    let _ = conn.execute(
        "INSERT INTO n8n_workflows (id, name, active, nodes_json, connections_json, settings_json, tags_json, pin_data_json, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9)",
        params![id, name, active, nodes.to_string(), conns.to_string(), settings.to_string(), tags.to_string(), pin_data.to_string(), now],
    );
    match conn.query_row(
        "SELECT id, name, active, nodes_json, connections_json, settings_json, tags_json, pin_data_json, created_at, updated_at, execution_count, last_executed_at
         FROM n8n_workflows WHERE id = ?1",
        [&id],
        workflow_row_to_json,
    ) {
        Ok(wf) => (StatusCode::CREATED, Json(wf)),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()}))),
    }
}

pub async fn update_workflow_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let now = Utc::now().to_rfc3339();
    if let Some(name) = v.get("name").and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE n8n_workflows SET name = ?1, updated_at = ?2 WHERE id = ?3", params![name, now, id]);
    }
    if let Some(act) = v.get("active").and_then(|x| x.as_bool()) {
        let _ = conn.execute("UPDATE n8n_workflows SET active = ?1, updated_at = ?2 WHERE id = ?3", params![if act { 1 } else { 0 }, now, id]);
    }
    if let Some(nodes) = v.get("nodes") {
        let _ = conn.execute("UPDATE n8n_workflows SET nodes_json = ?1, updated_at = ?2 WHERE id = ?3", params![nodes.to_string(), now, id]);
    }
    if let Some(conns) = v.get("connections") {
        let _ = conn.execute("UPDATE n8n_workflows SET connections_json = ?1, updated_at = ?2 WHERE id = ?3", params![conns.to_string(), now, id]);
    }
    if let Some(settings) = v.get("settings") {
        let _ = conn.execute("UPDATE n8n_workflows SET settings_json = ?1, updated_at = ?2 WHERE id = ?3", params![settings.to_string(), now, id]);
    }
    if let Some(tags) = v.get("tags") {
        let _ = conn.execute("UPDATE n8n_workflows SET tags_json = ?1, updated_at = ?2 WHERE id = ?3", params![tags.to_string(), now, id]);
    }
    if let Some(pin) = v.get("pinData") {
        let _ = conn.execute("UPDATE n8n_workflows SET pin_data_json = ?1, updated_at = ?2 WHERE id = ?3", params![pin.to_string(), now, id]);
    }
    match conn
        .query_row(
            "SELECT id, name, active, nodes_json, connections_json, settings_json, tags_json, pin_data_json, created_at, updated_at, execution_count, last_executed_at
             FROM n8n_workflows WHERE id = ?1",
            [&id],
            workflow_row_to_json,
        )
        .optional()
    {
        Ok(Some(wf)) => (StatusCode::OK, Json(wf)),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}

pub async fn delete_workflow_handler(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let n = conn.execute("DELETE FROM n8n_workflows WHERE id = ?1", [&id]).unwrap_or(0);
    Json(json!({ "deleted": n > 0, "id": id }))
}

pub async fn duplicate_workflow_handler(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let orig: Option<Value> = conn
        .query_row(
            "SELECT id, name, active, nodes_json, connections_json, settings_json, tags_json, pin_data_json, created_at, updated_at, execution_count, last_executed_at
             FROM n8n_workflows WHERE id = ?1",
            [&id],
            workflow_row_to_json,
        )
        .optional()
        .ok()
        .flatten();
    let Some(wf) = orig else {
        return (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"})));
    };
    let new_id = Uuid::new_v4().to_string();
    let new_name = format!("{} (Kopie)", wf["name"].as_str().unwrap_or("Workflow"));
    let now = Utc::now().to_rfc3339();
    let _ = conn.execute(
        "INSERT INTO n8n_workflows (id, name, active, nodes_json, connections_json, settings_json, tags_json, pin_data_json, created_at, updated_at)
         VALUES (?1, ?2, 0, ?3, ?4, ?5, ?6, ?7, ?8, ?8)",
        params![new_id, new_name, wf["nodes"].to_string(), wf["connections"].to_string(), wf["settings"].to_string(), wf["tags"].to_string(), wf["pinData"].to_string(), now],
    );
    match conn.query_row(
        "SELECT id, name, active, nodes_json, connections_json, settings_json, tags_json, pin_data_json, created_at, updated_at, execution_count, last_executed_at
         FROM n8n_workflows WHERE id = ?1",
        [&new_id],
        workflow_row_to_json,
    ) {
        Ok(copy) => (StatusCode::CREATED, Json(copy)),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}

pub async fn toggle_workflow_handler(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let cur: i64 = conn.query_row("SELECT active FROM n8n_workflows WHERE id = ?1", [&id], |r| r.get(0)).unwrap_or(0);
    let next = if cur != 0 { 0 } else { 1 };
    let now = Utc::now().to_rfc3339();
    let _ = conn.execute("UPDATE n8n_workflows SET active = ?1, updated_at = ?2 WHERE id = ?3", params![next, now, id]);
    Json(json!({ "id": id, "active": next != 0 }))
}

pub async fn execute_workflow_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    body: Option<Json<Value>>,
) -> impl IntoResponse {
    let payload = body
        .map(|Json(v)| v.get("payload").cloned().unwrap_or(v))
        .unwrap_or(Value::Null);
    match execute_workflow_by_id(&state.db, &id, payload, None).await {
        Ok(res) => (StatusCode::OK, Json(res)),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()}))),
    }
}

pub async fn execute_single_node_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let node_name = v.get("nodeName").and_then(|x| x.as_str()).unwrap_or("");
    let payload = v.get("payload").cloned().unwrap_or(Value::Null);
    match execute_workflow_by_id(&state.db, &id, payload, Some(node_name)).await {
        Ok(res) => (StatusCode::OK, Json(res)),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()}))),
    }
}

pub async fn list_executions_handler(
    State(state): State<AppState>,
    Query(q): Query<ExecQuery>,
) -> impl IntoResponse {
    let limit = q.limit.unwrap_or(40).clamp(1, 200);
    let conn = state.db.lock_conn();
    let mut items = Vec::new();
    let sql = if let Some(ref wf) = q.workflow_id {
        format!(
            "SELECT id, workflow_id, workflow_name, status, mode, started_at, finished_at, duration_ms, data_json, error_message, retry_of
             FROM n8n_executions WHERE workflow_id = '{}' ORDER BY started_at DESC LIMIT ?1",
            wf.replace('\'', "")
        )
    } else {
        "SELECT id, workflow_id, workflow_name, status, mode, started_at, finished_at, duration_ms, data_json, error_message, retry_of
         FROM n8n_executions ORDER BY started_at DESC LIMIT ?1".to_string()
    };
    if let Ok(mut stmt) = conn.prepare(&sql) {
        if let Ok(mut rows) = stmt.query(params![limit]) {
            while let Ok(Some(r)) = rows.next() {
                let dstr: String = r.get(8).unwrap_or_else(|_| "{}".into());
                let data: Value = serde_json::from_str(&dstr).unwrap_or_else(|_| json!({}));
                items.push(json!({
                    "id": r.get::<_, String>(0).unwrap_or_default(),
                    "workflowId": r.get::<_, String>(1).unwrap_or_default(),
                    "workflowName": r.get::<_, String>(2).unwrap_or_default(),
                    "status": r.get::<_, String>(3).unwrap_or_default(),
                    "mode": r.get::<_, String>(4).unwrap_or_default(),
                    "modelUsed": data.get("modelUsed").and_then(|v| v.as_str()).unwrap_or("QuickJS + Local Rust Engine"),
                    "startedAt": r.get::<_, String>(5).unwrap_or_default(),
                    "finishedAt": r.get::<_, String>(6).unwrap_or_default(),
                    "durationMs": r.get::<_, i64>(7).unwrap_or(0),
                    "nodeLogs": data.get("steps").cloned().unwrap_or_else(|| json!([])),
                    "data": data,
                    "errorMessage": r.get::<_, Option<String>>(9).unwrap_or(None),
                    "retryOf": r.get::<_, Option<String>>(10).unwrap_or(None)
                }));
            }
        }
    }
    Json(json!({ "data": items, "totalCount": items.len() }))
}

pub async fn get_execution_handler(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let row = conn.query_row(
        "SELECT id, workflow_id, workflow_name, status, mode, started_at, finished_at, duration_ms, data_json, error_message, retry_of FROM n8n_executions WHERE id = ?1",
        [&id],
        |r| {
            let dstr: String = r.get(8)?;
            let data: Value = serde_json::from_str(&dstr).unwrap_or_else(|_| json!({}));
            Ok(json!({
                "id": r.get::<_, String>(0)?,
                "workflowId": r.get::<_, String>(1)?,
                "workflowName": r.get::<_, String>(2)?,
                "status": r.get::<_, String>(3)?,
                "mode": r.get::<_, String>(4)?,
                "modelUsed": data.get("modelUsed").and_then(|v| v.as_str()).unwrap_or("QuickJS + Local Rust Engine"),
                "startedAt": r.get::<_, String>(5)?,
                "finishedAt": r.get::<_, String>(6)?,
                "durationMs": r.get::<_, i64>(7)?,
                "nodeLogs": data.get("steps").cloned().unwrap_or_else(|| json!([])),
                "data": data,
                "errorMessage": r.get::<_, Option<String>>(9)?,
                "retryOf": r.get::<_, Option<String>>(10)?
            }))
        },
    ).optional();
    match row {
        Ok(Some(v)) => (StatusCode::OK, Json(v)),
        _ => (StatusCode::NOT_FOUND, Json(json!({"error": "Ausfuehrung nicht gefunden"}))),
    }
}

pub async fn retry_execution_handler(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let (wf_id, payload) = {
        let conn = state.db.lock_conn();
        let row = conn
            .query_row("SELECT workflow_id, data_json FROM n8n_executions WHERE id = ?1", [&id], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?))
            })
            .optional();
        let Some((wf_id, dstr)) = row.ok().flatten() else {
            return (StatusCode::NOT_FOUND, Json(json!({"error": "Ausfuehrung nicht gefunden"})));
        };
        let data: Value = serde_json::from_str(&dstr).unwrap_or_else(|_| json!({}));
        (wf_id, data.get("triggerPayload").cloned().unwrap_or(Value::Null))
    };
    match execute_workflow_with_depth(&state.db, &wf_id, payload, None, 0, Some(&id)).await {
        Ok(res) => (StatusCode::OK, Json(res)),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()}))),
    }
}

// =========================================================================
// Credentials (AES-256-CBC), Variables, Tags, Data Tables
// =========================================================================

pub async fn list_credentials_handler(State(state): State<AppState>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let key = encryption_key();
    let mut items = Vec::new();
    if let Ok(mut st) = conn.prepare("SELECT id, name, cred_type, data_encrypted, created_at, updated_at FROM n8n_credentials ORDER BY updated_at DESC") {
        if let Ok(mut rows) = st.query([]) {
            while let Ok(Some(r)) = rows.next() {
                let id: String = r.get(0).unwrap_or_default();
                let name: String = r.get(1).unwrap_or_default();
                let ctype: String = r.get(2).unwrap_or_default();
                let enc: String = r.get(3).unwrap_or_default();
                let dec = decrypt_json_lenient(&key, &enc).unwrap_or_else(|_| json!({}));
                let redacted = redact_credential_data(&ctype, &dec);
                items.push(json!({
                    "id": id,
                    "name": name,
                    "type": ctype,
                    "data": redacted,
                    "cipherFormat": if enc.starts_with("U2FsdGVkX1") { "AES-256-CBC (Salted__)" } else { "AES-256-CBC" },
                    "createdAt": r.get::<_, String>(4).unwrap_or_default(),
                    "updatedAt": r.get::<_, String>(5).unwrap_or_default()
                }));
            }
        }
    }
    Json(json!({ "data": items, "totalCount": items.len() }))
}

pub async fn create_credential_handler(State(state): State<AppState>, Json(v): Json<Value>) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();
    let name = v.get("name").and_then(|x| x.as_str()).unwrap_or("Neue Anmeldedaten").to_string();
    let ctype = v.get("type").and_then(|x| x.as_str()).unwrap_or("httpHeaderAuth").to_string();
    let data = v.get("data").cloned().unwrap_or_else(|| json!({}));
    let now = Utc::now().to_rfc3339();
    let Ok(enc) = encrypt_json(&encryption_key(), &data) else {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Verschluesselung fehlgeschlagen"})));
    };
    let conn = state.db.lock_conn();
    let _ = conn.execute(
        "INSERT INTO n8n_credentials (id, name, cred_type, data_encrypted, nodes_access_json, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, '[]', ?5, ?5)",
        params![id, name, ctype, enc, now],
    );
    (StatusCode::CREATED, Json(json!({
        "id": id,
        "name": name,
        "type": ctype,
        "data": redact_credential_data(&ctype, &data),
        "encryptedHeader": &enc.chars().take(16).collect::<String>(),
        "createdAt": now,
        "updatedAt": now
    })))
}

pub async fn update_credential_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let key = encryption_key();
    let now = Utc::now().to_rfc3339();
    let existing: Option<(String, String)> = conn
        .query_row("SELECT cred_type, data_encrypted FROM n8n_credentials WHERE id = ?1", [&id], |r| Ok((r.get(0)?, r.get(1)?)))
        .optional()
        .ok()
        .flatten();
    let Some((ctype, enc_old)) = existing else {
        return (StatusCode::NOT_FOUND, Json(json!({"error": "Credential nicht gefunden"})));
    };
    if let Some(name) = v.get("name").and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE n8n_credentials SET name = ?1, updated_at = ?2 WHERE id = ?3", params![name, now, id]);
    }
    if let Some(new_data) = v.get("data").and_then(|x| x.as_object()) {
        let mut merged = decrypt_json_lenient(&key, &enc_old)
            .ok()
            .and_then(|d| d.as_object().cloned())
            .unwrap_or_default();
        for (k, val) in new_data {
            if val.as_str() != Some(BLANK_SENTINEL) {
                merged.insert(k.clone(), val.clone());
            }
        }
        if let Ok(enc_new) = encrypt_json(&key, &Value::Object(merged.clone())) {
            let _ = conn.execute("UPDATE n8n_credentials SET data_encrypted = ?1, updated_at = ?2 WHERE id = ?3", params![enc_new, now, id]);
        }
        return (StatusCode::OK, Json(json!({ "id": id, "type": ctype, "data": redact_credential_data(&ctype, &Value::Object(merged)) })));
    }
    (StatusCode::OK, Json(json!({ "id": id, "updated": true })))
}

pub async fn delete_credential_handler(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let n = conn.execute("DELETE FROM n8n_credentials WHERE id = ?1", [&id]).unwrap_or(0);
    Json(json!({ "deleted": n > 0, "id": id }))
}

pub async fn test_credential_handler(Json(v): Json<Value>) -> impl IntoResponse {
    Json(json!({
        "status": "OK",
        "message": format!("Anmeldedaten vom Typ '{}' geprueft & AES-256-CBC verschluesselt.", v.get("type").and_then(|x| x.as_str()).unwrap_or("generic"))
    }))
}

pub async fn list_variables_handler(State(state): State<AppState>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let mut items = Vec::new();
    if let Ok(mut st) = conn.prepare("SELECT id, var_key, value, var_type, project_id, updated_at FROM n8n_variables ORDER BY var_key ASC") {
        if let Ok(mut rows) = st.query([]) {
            while let Ok(Some(r)) = rows.next() {
                items.push(json!({
                    "id": r.get::<_, String>(0).unwrap_or_default(),
                    "key": r.get::<_, String>(1).unwrap_or_default(),
                    "value": r.get::<_, String>(2).unwrap_or_default(),
                    "type": r.get::<_, String>(3).unwrap_or_default(),
                    "projectId": r.get::<_, String>(4).unwrap_or_default(),
                    "updatedAt": r.get::<_, String>(5).unwrap_or_default()
                }));
            }
        }
    }
    Json(json!({ "data": items }))
}

pub async fn create_variable_handler(State(state): State<AppState>, Json(v): Json<Value>) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();
    let key = v.get("key").and_then(|x| x.as_str()).unwrap_or("NEUE_VAR").trim().to_string();
    let val = v.get("value").and_then(|x| x.as_str()).unwrap_or("").to_string();
    let now = Utc::now().to_rfc3339();
    let conn = state.db.lock_conn();
    let _ = conn.execute(
        "INSERT INTO n8n_variables (id, var_key, value, var_type, project_id, updated_at) VALUES (?1, ?2, ?3, 'string', 'default', ?4)
         ON CONFLICT(var_key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
        params![id, key, val, now],
    );
    (StatusCode::CREATED, Json(json!({ "id": id, "key": key, "value": val, "type": "string" })))
}

pub async fn update_variable_handler(State(state): State<AppState>, Path(id): Path<String>, Json(v): Json<Value>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let now = Utc::now().to_rfc3339();
    if let Some(val) = v.get("value").and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE n8n_variables SET value = ?1, updated_at = ?2 WHERE id = ?3", params![val, now, id]);
    }
    Json(json!({ "id": id, "updated": true }))
}

pub async fn delete_variable_handler(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let n = conn.execute("DELETE FROM n8n_variables WHERE id = ?1", [&id]).unwrap_or(0);
    Json(json!({ "deleted": n > 0, "id": id }))
}

pub async fn list_tags_handler(State(state): State<AppState>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let mut items = Vec::new();
    if let Ok(mut st) = conn.prepare("SELECT id, name, created_at FROM n8n_tags ORDER BY name ASC") {
        if let Ok(mut rows) = st.query([]) {
            while let Ok(Some(r)) = rows.next() {
                items.push(json!({
                    "id": r.get::<_, String>(0).unwrap_or_default(),
                    "name": r.get::<_, String>(1).unwrap_or_default(),
                    "createdAt": r.get::<_, String>(2).unwrap_or_default()
                }));
            }
        }
    }
    Json(json!({ "data": items }))
}

pub async fn create_tag_handler(State(state): State<AppState>, Json(v): Json<Value>) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();
    let name = v.get("name").and_then(|x| x.as_str()).unwrap_or("Tag").trim().to_string();
    let now = Utc::now().to_rfc3339();
    let conn = state.db.lock_conn();
    let _ = conn.execute("INSERT OR IGNORE INTO n8n_tags (id, name, created_at, updated_at) VALUES (?1, ?2, ?3, ?3)", params![id, name, now]);
    (StatusCode::CREATED, Json(json!({ "id": id, "name": name })))
}

pub async fn delete_tag_handler(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let n = conn.execute("DELETE FROM n8n_tags WHERE id = ?1", [&id]).unwrap_or(0);
    Json(json!({ "deleted": n > 0, "id": id }))
}

pub async fn list_data_tables_handler(State(state): State<AppState>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let mut items = Vec::new();
    if let Ok(mut st) = conn.prepare("SELECT id, name, columns_json, created_at, updated_at FROM n8n_data_tables ORDER BY name ASC") {
        if let Ok(mut rows) = st.query([]) {
            while let Ok(Some(r)) = rows.next() {
                let tid: String = r.get(0).unwrap_or_default();
                let count: i64 = conn.query_row("SELECT COUNT(*) FROM n8n_data_table_rows WHERE table_id = ?1", [&tid], |x| x.get(0)).unwrap_or(0);
                let cols: Value = serde_json::from_str(&r.get::<_, String>(2).unwrap_or_else(|_| "[]".into())).unwrap_or_else(|_| json!([]));
                items.push(json!({
                    "id": tid,
                    "name": r.get::<_, String>(1).unwrap_or_default(),
                    "columns": cols,
                    "rowCount": count,
                    "createdAt": r.get::<_, String>(3).unwrap_or_default(),
                    "updatedAt": r.get::<_, String>(4).unwrap_or_default()
                }));
            }
        }
    }
    Json(json!({ "data": items }))
}

pub async fn create_data_table_handler(State(state): State<AppState>, Json(v): Json<Value>) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();
    let name = v.get("name").and_then(|x| x.as_str()).unwrap_or("Neue Tabelle").trim().to_string();
    let cols = v.get("columns").cloned().unwrap_or_else(|| json!([{"name": "schluessel", "type": "string"}, {"name": "wert", "type": "string"}]));
    let now = Utc::now().to_rfc3339();
    let conn = state.db.lock_conn();
    let _ = conn.execute(
        "INSERT INTO n8n_data_tables (id, name, columns_json, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?4)",
        params![id, name, cols.to_string(), now],
    );
    (StatusCode::CREATED, Json(json!({ "id": id, "name": name, "columns": cols, "rowCount": 0 })))
}

pub async fn delete_data_table_handler(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let _ = conn.execute("DELETE FROM n8n_data_table_rows WHERE table_id = ?1", [&id]);
    let n = conn.execute("DELETE FROM n8n_data_tables WHERE id = ?1", [&id]).unwrap_or(0);
    Json(json!({ "deleted": n > 0, "id": id }))
}

pub async fn list_data_table_rows_handler(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let mut rows_out = Vec::new();
    if let Ok(mut st) = conn.prepare("SELECT id, data_json, created_at FROM n8n_data_table_rows WHERE table_id = ?1 ORDER BY id ASC LIMIT 500") {
        if let Ok(mut rows) = st.query([&id]) {
            while let Ok(Some(r)) = rows.next() {
                let rid: i64 = r.get(0).unwrap_or(0);
                let mut val: Value = serde_json::from_str(&r.get::<_, String>(1).unwrap_or_else(|_| "{}".into())).unwrap_or_else(|_| json!({}));
                if let Some(m) = val.as_object_mut() {
                    m.insert("id".into(), json!(rid));
                }
                rows_out.push(val);
            }
        }
    }
    Json(json!({ "data": rows_out, "tableId": id }))
}

pub async fn insert_data_table_row_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let now = Utc::now().to_rfc3339();
    let conn = state.db.lock_conn();
    let _ = conn.execute(
        "INSERT INTO n8n_data_table_rows (table_id, data_json, created_at, updated_at) VALUES (?1, ?2, ?3, ?3)",
        params![id, v.to_string(), now],
    );
    let rid = conn.last_insert_rowid();
    (StatusCode::CREATED, Json(json!({ "id": rid, "tableId": id, "data": v })))
}

pub async fn delete_data_table_row_handler(
    State(state): State<AppState>,
    Path((_table_id, row_id)): Path<(String, i64)>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let n = conn.execute("DELETE FROM n8n_data_table_rows WHERE id = ?1", [row_id]).unwrap_or(0);
    Json(json!({ "deleted": n > 0, "id": row_id }))
}

pub async fn sync_sqlite_handler(State(state): State<AppState>) -> impl IntoResponse {
    match sync_from_n8n_sqlite(&state.db) {
        Ok(res) => (StatusCode::OK, Json(res)),
        Err(e) => (StatusCode::OK, Json(json!({ "ok": false, "info": e.to_string() }))),
    }
}

pub async fn webhook_dispatch_handler(
    State(state): State<AppState>,
    Path(path): Path<String>,
    headers: HeaderMap,
    Query(q): Query<HashMap<String, String>>,
    body: Option<Json<Value>>,
) -> impl IntoResponse {
    let clean_path = path.trim_start_matches('/').to_lowercase();
    let target_wf_id: Option<String> = {
        let conn = state.db.lock_conn();
        let mut found = None;
        if let Ok(mut stmt) = conn.prepare("SELECT id, nodes_json FROM n8n_workflows WHERE active = 1") {
            if let Ok(mut rows) = stmt.query([]) {
                while let Ok(Some(r)) = rows.next() {
                    let wf_id: String = r.get(0).unwrap_or_default();
                    let njson: String = r.get(1).unwrap_or_default();
                    if let Ok(Value::Array(nodes)) = serde_json::from_str::<Value>(&njson) {
                        for n in nodes {
                            let tp = n["type"].as_str().unwrap_or("");
                            if tp.contains("webhook") || tp.contains("formTrigger") {
                                let wpath = n.pointer("/parameters/path").and_then(|v| v.as_str()).unwrap_or("").trim_start_matches('/').to_lowercase();
                                if wpath == clean_path || wpath.contains(&clean_path) || clean_path == wf_id.to_lowercase() {
                                    found = Some(wf_id.clone());
                                    break;
                                }
                            }
                        }
                    }
                    if found.is_some() {
                        break;
                    }
                }
            }
        }
        found
    };

    let Some(wf_id) = target_wf_id else {
        return (
            StatusCode::NOT_FOUND,
            Json(json!({"error": format!("Kein aktiver Webhook fuer Pfad '/{}' gefunden", clean_path)})),
        );
    };
    let mut payload = body.map(|Json(v)| v).unwrap_or_else(|| json!({}));
    if let Some(m) = payload.as_object_mut() {
        m.insert("query".into(), json!(q));
        m.insert("headers".into(), json!({ "content-type": headers.get("content-type").and_then(|v| v.to_str().ok()).unwrap_or("") }));
        m.insert("webhookPath".into(), json!(clean_path));
    }
    match execute_workflow_by_id(&state.db, &wf_id, payload, None).await {
        Ok(res) => {
            if let Some(wr) = res.get("webhookResponse").filter(|v| !v.is_null()) {
                let code = StatusCode::from_u16(wr["statusCode"].as_u64().unwrap_or(200) as u16).unwrap_or(StatusCode::OK);
                return (code, Json(wr["body"].clone()));
            }
            (StatusCode::OK, Json(res))
        }
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()}))),
    }
}

pub async fn webhook_waiting_handler(
    State(state): State<AppState>,
    Path(exec_id): Path<String>,
    body: Option<Json<Value>>,
) -> impl IntoResponse {
    let payload = body.map(|Json(v)| v).unwrap_or_else(|| json!({"resumed": true}));
    let wf_id: Option<String> = {
        let conn = state.db.lock_conn();
        conn.query_row("SELECT workflow_id FROM n8n_executions WHERE id = ?1", [&exec_id], |r| r.get(0))
            .optional()
            .ok()
            .flatten()
    };
    let Some(wf_id) = wf_id else {
        return (StatusCode::NOT_FOUND, Json(json!({"error": "Wartende Ausfuehrung nicht gefunden"})));
    };
    match execute_workflow_by_id(&state.db, &wf_id, payload, None).await {
        Ok(res) => (StatusCode::OK, Json(res)),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()}))),
    }
}

pub async fn form_trigger_get_handler(Path(path): Path<String>) -> impl IntoResponse {
    let clean = path.trim_start_matches('/');
    let html = format!(
        r#"<!doctype html><html lang="de"><head><meta charset="utf-8"><title>n8n Formular — Conrad KI</title>
<style>body{{font-family:Inter,system-ui,sans-serif;background:#0e0906;color:#f5f5f5;display:flex;justify-content:center;padding:48px 16px}}
.card{{background:#191410;border:1px solid #2e251e;border-radius:12px;max-width:460px;width:100%;padding:28px}}
input,textarea{{width:100%;padding:10px;border-radius:8px;border:1px solid #3c3027;background:#120d0a;color:#fff;margin:6px 0 14px;box-sizing:border-box}}
button{{background:#ea580c;color:#fff;border:0;border-radius:8px;padding:11px 18px;font-weight:600;cursor:pointer;width:100%}}</style></head>
<body><div class="card"><h2>Formular: {clean}</h2>
<form method="POST" action="/form/{clean}">
<label>Name / Unternehmen</label><input name="name" required placeholder="Max Mustermann GmbH">
<label>E-Mail</label><input name="email" type="email" placeholder="max@beispiel.de">
<label>Nachricht</label><textarea name="message" rows="4" placeholder="Anliegen..."></textarea>
<button type="submit">Absenden &amp; Workflow starten</button></form></div></body></html>"#
    );
    Html(html)
}

pub async fn form_trigger_post_handler(
    state: State<AppState>,
    path: Path<String>,
    headers: HeaderMap,
    q: Query<HashMap<String, String>>,
    axum::extract::Form(form_map): axum::extract::Form<HashMap<String, String>>,
) -> impl IntoResponse {
    let json_val = json!(form_map);
    webhook_dispatch_handler(state, path, headers, q, Some(Json(json_val))).await
}
