use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path as StdPath;
use std::sync::OnceLock;
use uuid::Uuid;

use crate::db::Database;
use crate::AppState;

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn http_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(45))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new())
    })
}

pub fn ollama_base_url() -> String {
    std::env::var("OLLAMA_URL").unwrap_or_else(|_| "http://127.0.0.1:11434".to_string())
}

pub fn default_twenty_model() -> String {
    std::env::var("TWENTY_AGENT_MODEL")
        .or_else(|_| std::env::var("OLLAMA_MODEL"))
        .unwrap_or_else(|_| "conrad-n8n-agent:0.6b".to_string())
}

pub fn resolve_workspace(headers: &HeaderMap, param: Option<&str>) -> String {
    if let Some(ws) = param {
        let low = ws.trim().to_lowercase();
        if low == "pala" || low == "conrad-ki" || low == "all" {
            return low;
        }
    }
    if let Some(host) = headers.get("host").and_then(|v| v.to_str().ok()) {
        if host.to_lowercase().contains("pala.") {
            return "pala".to_string();
        }
    }
    "conrad-ki".to_string()
}

pub fn strip_think_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(start) = rest.find("<think>") {
        out.push_str(&rest[..start]);
        if let Some(end) = rest[start..].find("</think>") {
            rest = &rest[start + end + "</think>".len()..];
        } else {
            rest = "";
            break;
        }
    }
    out.push_str(rest);
    out.trim().to_string()
}

pub fn init_and_seed_twenty(db: &Database) -> anyhow::Result<()> {
    {
        let conn = db.lock_conn();
        create_schema(&conn)?;
    }

    let existing_companies: i64 = {
        let conn = db.lock_conn();
        conn.query_row("SELECT COUNT(*) FROM twenty_companies", [], |r| r.get(0))
            .unwrap_or(0)
    };

    if existing_companies == 0 {
        let stats = sync_all_sql_dumps(db)?;
        tracing::info!(
            "Twenty Rust CRM seeded from Postgres dumps: {:?}",
            stats
        );
    } else {
        tracing::info!(
            "Twenty Rust CRM ready ({} companies in SQLite)",
            existing_companies
        );
    }
    Ok(())
}

fn create_schema(conn: &Connection) -> anyhow::Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS twenty_workspaces (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            domain TEXT NOT NULL,
            locale TEXT NOT NULL DEFAULT 'de-DE',
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS twenty_members (
            id TEXT PRIMARY KEY,
            workspace TEXT NOT NULL DEFAULT 'conrad-ki',
            first_name TEXT NOT NULL DEFAULT '',
            last_name TEXT NOT NULL DEFAULT '',
            email TEXT NOT NULL DEFAULT '',
            avatar_url TEXT NOT NULL DEFAULT '',
            locale TEXT NOT NULL DEFAULT 'de-DE',
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS twenty_companies (
            id TEXT PRIMARY KEY,
            workspace TEXT NOT NULL DEFAULT 'conrad-ki',
            name TEXT NOT NULL,
            domain_label TEXT NOT NULL DEFAULT '',
            domain_url TEXT NOT NULL DEFAULT '',
            street1 TEXT NOT NULL DEFAULT '',
            city TEXT NOT NULL DEFAULT '',
            postcode TEXT NOT NULL DEFAULT '',
            state TEXT NOT NULL DEFAULT '',
            country TEXT NOT NULL DEFAULT 'Deutschland',
            employees INTEGER,
            linkedin_url TEXT NOT NULL DEFAULT '',
            x_url TEXT NOT NULL DEFAULT '',
            arr_micros INTEGER NOT NULL DEFAULT 0,
            arr_currency TEXT NOT NULL DEFAULT 'EUR',
            icp INTEGER NOT NULL DEFAULT 0,
            position REAL NOT NULL DEFAULT 0,
            created_by_name TEXT NOT NULL DEFAULT 'System',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS twenty_people (
            id TEXT PRIMARY KEY,
            workspace TEXT NOT NULL DEFAULT 'conrad-ki',
            first_name TEXT NOT NULL DEFAULT '',
            last_name TEXT NOT NULL DEFAULT '',
            email TEXT NOT NULL DEFAULT '',
            phone TEXT NOT NULL DEFAULT '',
            phone_country_code TEXT NOT NULL DEFAULT 'DE',
            phone_calling_code TEXT NOT NULL DEFAULT '+49',
            job_title TEXT NOT NULL DEFAULT '',
            city TEXT NOT NULL DEFAULT '',
            linkedin_url TEXT NOT NULL DEFAULT '',
            company_id TEXT,
            position REAL NOT NULL DEFAULT 0,
            created_by_name TEXT NOT NULL DEFAULT 'System',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS twenty_opportunities (
            id TEXT PRIMARY KEY,
            workspace TEXT NOT NULL DEFAULT 'conrad-ki',
            name TEXT NOT NULL,
            amount_micros INTEGER NOT NULL DEFAULT 0,
            currency_code TEXT NOT NULL DEFAULT 'EUR',
            close_date TEXT,
            stage TEXT NOT NULL DEFAULT 'NEW',
            company_id TEXT,
            point_of_contact_id TEXT,
            owner_id TEXT,
            position REAL NOT NULL DEFAULT 0,
            created_by_name TEXT NOT NULL DEFAULT 'System',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS twenty_notes (
            id TEXT PRIMARY KEY,
            workspace TEXT NOT NULL DEFAULT 'conrad-ki',
            title TEXT NOT NULL,
            body_markdown TEXT NOT NULL DEFAULT '',
            company_id TEXT,
            person_id TEXT,
            opportunity_id TEXT,
            position REAL NOT NULL DEFAULT 0,
            created_by_name TEXT NOT NULL DEFAULT 'System',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS twenty_tasks (
            id TEXT PRIMARY KEY,
            workspace TEXT NOT NULL DEFAULT 'conrad-ki',
            title TEXT NOT NULL,
            body_markdown TEXT NOT NULL DEFAULT '',
            status TEXT NOT NULL DEFAULT 'TODO',
            due_at TEXT,
            company_id TEXT,
            person_id TEXT,
            opportunity_id TEXT,
            assignee_id TEXT,
            position REAL NOT NULL DEFAULT 0,
            created_by_name TEXT NOT NULL DEFAULT 'System',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS twenty_timeline (
            id TEXT PRIMARY KEY,
            workspace TEXT NOT NULL DEFAULT 'conrad-ki',
            happens_at TEXT NOT NULL,
            name TEXT NOT NULL,
            company_id TEXT,
            person_id TEXT,
            opportunity_id TEXT,
            note_id TEXT,
            task_id TEXT,
            actor TEXT NOT NULL DEFAULT 'System',
            details_json TEXT NOT NULL DEFAULT '{}'
        );

        CREATE INDEX IF NOT EXISTS idx_twenty_companies_ws ON twenty_companies(workspace, name);
        CREATE INDEX IF NOT EXISTS idx_twenty_people_ws ON twenty_people(workspace, company_id);
        CREATE INDEX IF NOT EXISTS idx_twenty_opp_ws ON twenty_opportunities(workspace, stage, company_id);
        CREATE INDEX IF NOT EXISTS idx_twenty_notes_ws ON twenty_notes(workspace, company_id, person_id, opportunity_id);
        CREATE INDEX IF NOT EXISTS idx_twenty_tasks_ws ON twenty_tasks(workspace, status, company_id);
        CREATE INDEX IF NOT EXISTS idx_twenty_tl_ws ON twenty_timeline(workspace, happens_at);
        ",
    )?;

    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT OR IGNORE INTO twenty_workspaces (id, name, domain, locale, created_at) VALUES (?1, ?2, ?3, 'de-DE', ?4)",
        params!["conrad-ki", "Conrad KI", "crm.conrad-ki.com", now],
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO twenty_workspaces (id, name, domain, locale, created_at) VALUES (?1, ?2, ?3, 'de-DE', ?4)",
        params!["pala", "Pala Marketing", "pala.crm.conrad-ki.com", now],
    )?;
    Ok(())
}

fn unescape_pg_copy(raw: &str) -> Option<String> {
    if raw == r"\N" {
        return None;
    }
    if !raw.contains('\\') {
        return Some(raw.to_string());
    }
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => out.push('\n'),
                Some('r') => out.push('\r'),
                Some('t') => out.push('\t'),
                Some('\\') => out.push('\\'),
                Some('b') => out.push('\x08'),
                Some('f') => out.push('\x0c'),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => out.push('\\'),
            }
        } else {
            out.push(c);
        }
    }
    Some(out)
}

fn extract_blocknote_text(v: &Value, out: &mut Vec<String>) {
    match v {
        Value::Array(arr) => {
            for item in arr {
                extract_blocknote_text(item, out);
            }
        }
        Value::Object(map) => {
            if let Some(Value::String(t)) = map.get("text") {
                if !t.is_empty() {
                    out.push(t.clone());
                }
            }
            if let Some(content) = map.get("content") {
                extract_blocknote_text(content, out);
            }
            if let Some(children) = map.get("children") {
                extract_blocknote_text(children, out);
            }
        }
        _ => {}
    }
}

fn resolve_body_markdown(md: Option<&str>, blocknote: Option<&str>) -> String {
    if let Some(m) = md {
        if !m.trim().is_empty() {
            return m.to_string();
        }
    }
    if let Some(bn) = blocknote {
        if let Ok(v) = serde_json::from_str::<Value>(bn) {
            let mut parts = Vec::new();
            extract_blocknote_text(&v, &mut parts);
            if !parts.is_empty() {
                return parts.join(" ");
            }
        }
    }
    String::new()
}

fn parse_copy_header(line: &str) -> Option<(String, Vec<String>)> {
    if !line.starts_with("COPY ") || !line.contains(" FROM stdin;") {
        return None;
    }
    let open_paren = line.find('(')?;
    let close_paren = line.rfind(") FROM stdin;")?;
    let table_full = line[5..open_paren].trim();
    let table_name = table_full
        .split('.')
        .last()
        .unwrap_or(table_full)
        .trim_matches('"')
        .to_string();
    let cols: Vec<String> = line[open_paren + 1..close_paren]
        .split(',')
        .map(|s| s.trim().trim_matches('"').to_string())
        .collect();
    Some((table_name, cols))
}

struct CopyRow<'a> {
    idx: &'a HashMap<String, usize>,
    fields: Vec<Option<String>>,
}

impl<'a> CopyRow<'a> {
    fn get(&self, col: &str) -> Option<&str> {
        self.idx
            .get(col)
            .and_then(|&i| self.fields.get(i))
            .and_then(|o| o.as_deref())
    }
    fn get_str(&self, col: &str) -> String {
        self.get(col).unwrap_or("").to_string()
    }
    fn get_i64(&self, col: &str) -> i64 {
        self.get(col)
            .and_then(|s| s.trim().parse::<f64>().ok())
            .map(|f| f as i64)
            .unwrap_or(0)
    }
    fn get_opt_i64(&self, col: &str) -> Option<i64> {
        self.get(col)
            .and_then(|s| s.trim().parse::<f64>().ok())
            .map(|f| f as i64)
    }
    fn get_f64(&self, col: &str) -> f64 {
        self.get(col)
            .and_then(|s| s.trim().parse::<f64>().ok())
            .unwrap_or(0.0)
    }
    fn is_deleted(&self) -> bool {
        self.get("deletedAt").map(|s| !s.is_empty()).unwrap_or(false)
    }
}

pub fn sync_all_sql_dumps(db: &Database) -> anyhow::Result<Value> {
    let sources = [
        ("conrad-ki", "/opt/n8n/conrad-ki-backup-v1.19.0.sql"),
        ("pala", "/opt/n8n/pala-twenty-backup-v1.19.0.sql"),
    ];

    let mut summary = serde_json::Map::new();
    let mut conn = db.lock_conn();
    let tx = conn.transaction()?;

    for (ws, path_str) in sources {
        let p = StdPath::new(path_str);
        if !p.exists() {
            summary.insert(ws.to_string(), json!({"status": "missing_file", "path": path_str}));
            continue;
        }
        let file = File::open(p)?;
        let reader = BufReader::new(file);

        let mut active_table: Option<String> = None;
        let mut col_map: HashMap<String, usize> = HashMap::new();

        let mut c_companies = 0i64;
        let mut c_people = 0i64;
        let mut c_opps = 0i64;
        let mut c_notes = 0i64;
        let mut c_note_targets = 0i64;
        let mut c_tasks = 0i64;
        let mut c_task_targets = 0i64;
        let mut c_members = 0i64;

        for line_res in reader.lines() {
            let line = match line_res {
                Ok(l) => l,
                Err(_) => continue,
            };

            if active_table.is_none() {
                if let Some((tbl, cols)) = parse_copy_header(&line) {
                    if matches!(
                        tbl.as_str(),
                        "company"
                            | "person"
                            | "opportunity"
                            | "note"
                            | "noteTarget"
                            | "task"
                            | "taskTarget"
                            | "workspaceMember"
                    ) {
                        col_map.clear();
                        for (i, c) in cols.into_iter().enumerate() {
                            col_map.insert(c, i);
                        }
                        active_table = Some(tbl);
                    }
                }
                continue;
            }

            if line == r"\." {
                active_table = None;
                col_map.clear();
                continue;
            }

            let fields: Vec<Option<String>> = line.split('\t').map(unescape_pg_copy).collect();
            let row = CopyRow {
                idx: &col_map,
                fields,
            };

            if row.is_deleted() {
                continue;
            }
            let id = match row.get("id") {
                Some(v) if !v.is_empty() => v.to_string(),
                _ => continue,
            };

            match active_table.as_deref() {
                Some("workspaceMember") => {
                    tx.execute(
                        "INSERT OR REPLACE INTO twenty_members
                         (id, workspace, first_name, last_name, email, avatar_url, locale, created_at)
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                        params![
                            id,
                            ws,
                            row.get_str("nameFirstName"),
                            row.get_str("nameLastName"),
                            row.get_str("userEmail"),
                            row.get_str("avatarUrl"),
                            row.get("locale").unwrap_or("de-DE"),
                            row.get("createdAt").unwrap_or("2026-01-01T00:00:00Z"),
                        ],
                    )?;
                    c_members += 1;
                }
                Some("company") => {
                    let icp = if matches!(row.get("idealCustomerProfile"), Some("t" | "true" | "1")) {
                        1i64
                    } else {
                        0i64
                    };
                    tx.execute(
                        "INSERT OR REPLACE INTO twenty_companies
                         (id, workspace, name, domain_label, domain_url, street1, city, postcode, state, country,
                          employees, linkedin_url, x_url, arr_micros, arr_currency, icp, position, created_by_name, created_at, updated_at)
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)",
                        params![
                            id,
                            ws,
                            row.get("name").unwrap_or("Unbenannte Firma"),
                            row.get_str("domainNamePrimaryLinkLabel"),
                            row.get_str("domainNamePrimaryLinkUrl"),
                            row.get_str("addressAddressStreet1"),
                            row.get_str("addressAddressCity"),
                            row.get_str("addressAddressPostcode"),
                            row.get_str("addressAddressState"),
                            row.get("addressAddressCountry").filter(|s| !s.is_empty()).unwrap_or("Deutschland"),
                            row.get_opt_i64("employees"),
                            row.get_str("linkedinLinkPrimaryLinkUrl"),
                            row.get_str("xLinkPrimaryLinkUrl"),
                            row.get_i64("annualRecurringRevenueAmountMicros"),
                            row.get("annualRecurringRevenueCurrencyCode").filter(|s| !s.is_empty()).unwrap_or("EUR"),
                            icp,
                            row.get_f64("position"),
                            row.get("createdByName").unwrap_or("System"),
                            row.get("createdAt").unwrap_or("2026-01-01T00:00:00Z"),
                            row.get("updatedAt").unwrap_or("2026-01-01T00:00:00Z"),
                        ],
                    )?;
                    c_companies += 1;
                }
                Some("person") => {
                    tx.execute(
                        "INSERT OR REPLACE INTO twenty_people
                         (id, workspace, first_name, last_name, email, phone, phone_country_code, phone_calling_code,
                          job_title, city, linkedin_url, company_id, position, created_by_name, created_at, updated_at)
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
                        params![
                            id,
                            ws,
                            row.get_str("nameFirstName"),
                            row.get_str("nameLastName"),
                            row.get_str("emailsPrimaryEmail"),
                            row.get_str("phonesPrimaryPhoneNumber"),
                            row.get("phonesPrimaryPhoneCountryCode").filter(|s| !s.is_empty()).unwrap_or("DE"),
                            row.get("phonesPrimaryPhoneCallingCode").filter(|s| !s.is_empty()).unwrap_or("+49"),
                            row.get_str("jobTitle"),
                            row.get_str("city"),
                            row.get_str("linkedinLinkPrimaryLinkUrl"),
                            row.get("companyId"),
                            row.get_f64("position"),
                            row.get("createdByName").unwrap_or("System"),
                            row.get("createdAt").unwrap_or("2026-01-01T00:00:00Z"),
                            row.get("updatedAt").unwrap_or("2026-01-01T00:00:00Z"),
                        ],
                    )?;
                    c_people += 1;
                }
                Some("opportunity") => {
                    tx.execute(
                        "INSERT OR REPLACE INTO twenty_opportunities
                         (id, workspace, name, amount_micros, currency_code, close_date, stage, company_id,
                          point_of_contact_id, owner_id, position, created_by_name, created_at, updated_at)
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
                        params![
                            id,
                            ws,
                            row.get("name").unwrap_or("Opportunity"),
                            row.get_i64("amountAmountMicros"),
                            row.get("amountCurrencyCode").filter(|s| !s.is_empty()).unwrap_or("EUR"),
                            row.get("closeDate"),
                            row.get("stage").filter(|s| !s.is_empty()).unwrap_or("NEW"),
                            row.get("companyId"),
                            row.get("pointOfContactId"),
                            row.get("ownerId"),
                            row.get_f64("position"),
                            row.get("createdByName").unwrap_or("System"),
                            row.get("createdAt").unwrap_or("2026-01-01T00:00:00Z"),
                            row.get("updatedAt").unwrap_or("2026-01-01T00:00:00Z"),
                        ],
                    )?;
                    c_opps += 1;
                }
                Some("note") => {
                    let body = resolve_body_markdown(row.get("bodyV2Markdown"), row.get("bodyV2Blocknote"));
                    tx.execute(
                        "INSERT OR REPLACE INTO twenty_notes
                         (id, workspace, title, body_markdown, company_id, person_id, opportunity_id,
                          position, created_by_name, created_at, updated_at)
                         VALUES (?1, ?2, ?3, ?4, NULL, NULL, NULL, ?5, ?6, ?7, ?8)",
                        params![
                            id,
                            ws,
                            row.get("title").filter(|s| !s.is_empty()).unwrap_or("Notiz"),
                            body,
                            row.get_f64("position"),
                            row.get("createdByName").unwrap_or("System"),
                            row.get("createdAt").unwrap_or("2026-01-01T00:00:00Z"),
                            row.get("updatedAt").unwrap_or("2026-01-01T00:00:00Z"),
                        ],
                    )?;
                    c_notes += 1;
                }
                Some("noteTarget") => {
                    if let Some(note_id) = row.get("noteId") {
                        tx.execute(
                            "UPDATE twenty_notes
                             SET company_id = COALESCE(company_id, ?1),
                                 person_id = COALESCE(person_id, ?2),
                                 opportunity_id = COALESCE(opportunity_id, ?3)
                             WHERE id = ?4",
                            params![
                                row.get("targetCompanyId"),
                                row.get("targetPersonId"),
                                row.get("targetOpportunityId"),
                                note_id
                            ],
                        )?;
                        c_note_targets += 1;
                    }
                }
                Some("task") => {
                    let body = resolve_body_markdown(row.get("bodyV2Markdown"), row.get("bodyV2Blocknote"));
                    tx.execute(
                        "INSERT OR REPLACE INTO twenty_tasks
                         (id, workspace, title, body_markdown, status, due_at, company_id, person_id, opportunity_id,
                          assignee_id, position, created_by_name, created_at, updated_at)
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6, NULL, NULL, NULL, ?7, ?8, ?9, ?10, ?11)",
                        params![
                            id,
                            ws,
                            row.get("title").filter(|s| !s.is_empty()).unwrap_or("Aufgabe"),
                            body,
                            row.get("status").filter(|s| !s.is_empty()).unwrap_or("TODO"),
                            row.get("dueAt"),
                            row.get("assigneeId"),
                            row.get_f64("position"),
                            row.get("createdByName").unwrap_or("System"),
                            row.get("createdAt").unwrap_or("2026-01-01T00:00:00Z"),
                            row.get("updatedAt").unwrap_or("2026-01-01T00:00:00Z"),
                        ],
                    )?;
                    c_tasks += 1;
                }
                Some("taskTarget") => {
                    if let Some(task_id) = row.get("taskId") {
                        tx.execute(
                            "UPDATE twenty_tasks
                             SET company_id = COALESCE(company_id, ?1),
                                 person_id = COALESCE(person_id, ?2),
                                 opportunity_id = COALESCE(opportunity_id, ?3)
                             WHERE id = ?4",
                            params![
                                row.get("targetCompanyId"),
                                row.get("targetPersonId"),
                                row.get("targetOpportunityId"),
                                task_id
                            ],
                        )?;
                        c_task_targets += 1;
                    }
                }
                _ => {}
            }
        }

        summary.insert(
            ws.to_string(),
            json!({
                "members": c_members,
                "companies": c_companies,
                "people": c_people,
                "opportunities": c_opps,
                "notes": c_notes,
                "noteTargets": c_note_targets,
                "tasks": c_tasks,
                "taskTargets": c_task_targets
            }),
        );
    }

    tx.commit()?;
    Ok(Value::Object(summary))
}

#[derive(Debug, Deserialize)]
pub struct ListParams {
    pub workspace: Option<String>,
    pub q: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub stage: Option<String>,
    pub status: Option<String>,
    pub icp: Option<bool>,
    #[serde(rename = "hasDomain")]
    pub has_domain: Option<bool>,
    pub sort: Option<String>,
    #[serde(rename = "companyId")]
    pub company_id: Option<String>,
    #[serde(rename = "personId")]
    pub person_id: Option<String>,
    #[serde(rename = "opportunityId")]
    pub opportunity_id: Option<String>,
}

fn ws_filter_clause(ws: &str) -> (&'static str, String) {
    if ws == "all" {
        ("1=1", String::new())
    } else {
        ("workspace = ?1", ws.to_string())
    }
}

pub async fn get_stats_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<ListParams>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, q.workspace.as_deref());
    let mut ollama_models: Vec<String> = Vec::new();
    if let Ok(resp) = http_client()
        .get(format!("{}/api/tags", ollama_base_url()))
        .timeout(std::time::Duration::from_secs(2))
        .send()
        .await
    {
        if let Ok(v) = resp.json::<Value>().await {
            if let Some(arr) = v.get("models").and_then(|m| m.as_array()) {
                for m in arr {
                    if let Some(n) = m.get("name").and_then(|x| x.as_str()) {
                        ollama_models.push(n.to_string());
                    }
                }
            }
        }
    }
    let conn = state.db.lock_conn();

    let count_for = |tbl: &str, filter_ws: &str| -> i64 {
        if filter_ws == "all" {
            conn.query_row(&format!("SELECT COUNT(*) FROM {tbl}"), [], |r| r.get(0))
                .unwrap_or(0)
        } else {
            conn.query_row(
                &format!("SELECT COUNT(*) FROM {tbl} WHERE workspace = ?1"),
                [filter_ws],
                |r| r.get(0),
            )
            .unwrap_or(0)
        }
    };

    let mut stage_map = serde_json::Map::new();
    let stage_sql = if ws == "all" {
        "SELECT stage, COUNT(*), COALESCE(SUM(amount_micros),0) FROM twenty_opportunities GROUP BY stage".to_string()
    } else {
        "SELECT stage, COUNT(*), COALESCE(SUM(amount_micros),0) FROM twenty_opportunities WHERE workspace = ?1 GROUP BY stage".to_string()
    };
    let mut total_pipeline_micros: i64 = 0;
    if let Ok(mut stmt) = conn.prepare(&stage_sql) {
        let mut rows = if ws == "all" {
            stmt.query([])
        } else {
            stmt.query([ws.as_str()])
        }
        .unwrap();
        while let Ok(Some(r)) = rows.next() {
            let st: String = r.get(0).unwrap_or_else(|_| "NEW".to_string());
            let cnt: i64 = r.get(1).unwrap_or(0);
            let amt: i64 = r.get(2).unwrap_or(0);
            total_pipeline_micros += amt;
            stage_map.insert(
                st,
                json!({
                    "count": cnt,
                    "amountMicros": amt,
                    "amountEur": (amt as f64) / 1_000_000.0
                }),
            );
        }
    }

    let mut task_status_map = serde_json::Map::new();
    let task_sql = if ws == "all" {
        "SELECT status, COUNT(*) FROM twenty_tasks GROUP BY status"
    } else {
        "SELECT status, COUNT(*) FROM twenty_tasks WHERE workspace = ?1 GROUP BY status"
    };
    if let Ok(mut stmt) = conn.prepare(task_sql) {
        let mut rows = if ws == "all" {
            stmt.query([])
        } else {
            stmt.query([ws.as_str()])
        }
        .unwrap();
        while let Ok(Some(r)) = rows.next() {
            let st: String = r.get(0).unwrap_or_else(|_| "TODO".to_string());
            let cnt: i64 = r.get(1).unwrap_or(0);
            task_status_map.insert(st, json!(cnt));
        }
    }

    Json(json!({
        "activeWorkspace": ws,
        "companies": count_for("twenty_companies", &ws),
        "people": count_for("twenty_people", &ws),
        "opportunities": count_for("twenty_opportunities", &ws),
        "notes": count_for("twenty_notes", &ws),
        "tasks": count_for("twenty_tasks", &ws),
        "members": count_for("twenty_members", &ws),
        "totalPipelineMicros": total_pipeline_micros,
        "totalPipelineEur": (total_pipeline_micros as f64) / 1_000_000.0,
        "byStage": Value::Object(stage_map),
        "tasksByStatus": Value::Object(task_status_map),
        "ollamaModels": ollama_models,
        "defaultModel": default_twenty_model(),
        "workspaces": {
            "conrad-ki": {
                "companies": count_for("twenty_companies", "conrad-ki"),
                "people": count_for("twenty_people", "conrad-ki"),
                "opportunities": count_for("twenty_opportunities", "conrad-ki"),
                "notes": count_for("twenty_notes", "conrad-ki"),
                "tasks": count_for("twenty_tasks", "conrad-ki")
            },
            "pala": {
                "companies": count_for("twenty_companies", "pala"),
                "people": count_for("twenty_people", "pala"),
                "opportunities": count_for("twenty_opportunities", "pala"),
                "notes": count_for("twenty_notes", "pala"),
                "tasks": count_for("twenty_tasks", "pala")
            },
            "all": {
                "companies": count_for("twenty_companies", "all"),
                "people": count_for("twenty_people", "all"),
                "opportunities": count_for("twenty_opportunities", "all"),
                "notes": count_for("twenty_notes", "all"),
                "tasks": count_for("twenty_tasks", "all")
            }
        }
    }))
}

pub async fn get_metadata_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<ListParams>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, q.workspace.as_deref());
    let conn = state.db.lock_conn();
    let count_tbl = |tbl: &str| -> i64 {
        if ws == "all" {
            conn.query_row(&format!("SELECT COUNT(*) FROM {tbl}"), [], |r| r.get(0)).unwrap_or(0)
        } else {
            conn.query_row(&format!("SELECT COUNT(*) FROM {tbl} WHERE workspace = ?1"), [&ws], |r| r.get(0)).unwrap_or(0)
        }
    };
    Json(json!({
        "workspace": ws,
        "objects": [
            {
                "nameSingular": "company",
                "namePlural": "companies",
                "labelPlural": "Companies",
                "icon": "IconBuildingSkyscraper",
                "recordCount": count_tbl("twenty_companies"),
                "fields": [
                    {"name": "name", "label": "Name", "type": "TEXT", "isCustom": false},
                    {"name": "domainName", "label": "Domain Name", "type": "LINKS", "isCustom": false},
                    {"name": "address", "label": "Address", "type": "ADDRESS", "isCustom": false},
                    {"name": "employees", "label": "Employees", "type": "NUMBER", "isCustom": false},
                    {"name": "annualRecurringRevenue", "label": "ARR", "type": "CURRENCY", "isCustom": false},
                    {"name": "idealCustomerProfile", "label": "ICP", "type": "BOOLEAN", "isCustom": false},
                    {"name": "linkedinLink", "label": "LinkedIn", "type": "LINKS", "isCustom": false}
                ]
            },
            {
                "nameSingular": "person",
                "namePlural": "people",
                "labelPlural": "People",
                "icon": "IconUser",
                "recordCount": count_tbl("twenty_people"),
                "fields": [
                    {"name": "name", "label": "Name", "type": "FULL_NAME", "isCustom": false},
                    {"name": "emails", "label": "Emails", "type": "EMAILS", "isCustom": false},
                    {"name": "phones", "label": "Phones", "type": "PHONES", "isCustom": false},
                    {"name": "jobTitle", "label": "Job Title", "type": "TEXT", "isCustom": false},
                    {"name": "city", "label": "City", "type": "TEXT", "isCustom": false},
                    {"name": "company", "label": "Company", "type": "RELATION", "isCustom": false}
                ]
            },
            {
                "nameSingular": "opportunity",
                "namePlural": "opportunities",
                "labelPlural": "Opportunities",
                "icon": "IconTargetArrow",
                "recordCount": count_tbl("twenty_opportunities"),
                "stages": ["NEW", "SCREENING", "MEETING", "PROPOSAL", "CUSTOMER"],
                "fields": [
                    {"name": "name", "label": "Name", "type": "TEXT", "isCustom": false},
                    {"name": "amount", "label": "Amount", "type": "CURRENCY", "isCustom": false},
                    {"name": "stage", "label": "Stage", "type": "SELECT", "options": ["NEW","SCREENING","MEETING","PROPOSAL","CUSTOMER"]},
                    {"name": "closeDate", "label": "Close Date", "type": "DATE_TIME", "isCustom": false},
                    {"name": "company", "label": "Company", "type": "RELATION", "isCustom": false},
                    {"name": "pointOfContact", "label": "Point of Contact", "type": "RELATION", "isCustom": false}
                ]
            },
            {
                "nameSingular": "note",
                "namePlural": "notes",
                "labelPlural": "Notes",
                "icon": "IconNotes",
                "recordCount": count_tbl("twenty_notes"),
                "fields": [
                    {"name": "title", "label": "Title", "type": "TEXT", "isCustom": false},
                    {"name": "bodyV2", "label": "Body", "type": "RICH_TEXT_V2", "isCustom": false},
                    {"name": "noteTargets", "label": "Relations", "type": "RELATION", "isCustom": false}
                ]
            },
            {
                "nameSingular": "task",
                "namePlural": "tasks",
                "labelPlural": "Tasks",
                "icon": "IconCheckbox",
                "recordCount": count_tbl("twenty_tasks"),
                "statuses": ["TODO", "IN_PROGRESS", "DONE"],
                "fields": [
                    {"name": "title", "label": "Title", "type": "TEXT", "isCustom": false},
                    {"name": "status", "label": "Status", "type": "SELECT", "options": ["TODO","IN_PROGRESS","DONE"]},
                    {"name": "dueAt", "label": "Due Date", "type": "DATE_TIME", "isCustom": false},
                    {"name": "bodyV2", "label": "Body", "type": "RICH_TEXT_V2", "isCustom": false},
                    {"name": "taskTargets", "label": "Relations", "type": "RELATION", "isCustom": false}
                ]
            }
        ]
    }))
}

pub async fn sync_sql_handler(State(state): State<AppState>) -> impl IntoResponse {
    match sync_all_sql_dumps(&state.db) {
        Ok(res) => (StatusCode::OK, Json(json!({"ok": true, "synced": res}))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"ok": false, "error": e.to_string()})),
        ),
    }
}

fn company_row_to_json(r: &rusqlite::Row<'_>) -> rusqlite::Result<Value> {
    let arr_micros: i64 = r.get("arr_micros")?;
    let currency: String = r.get("arr_currency")?;
    let domain_url: String = r.get("domain_url")?;
    let domain_label: String = r.get("domain_label")?;
    let linkedin_url: String = r.get("linkedin_url")?;
    let x_url: String = r.get("x_url")?;
    Ok(json!({
        "id": r.get::<_, String>("id")?,
        "workspace": r.get::<_, String>("workspace")?,
        "name": r.get::<_, String>("name")?,
        "domainName": { "primaryLinkUrl": domain_url, "primaryLinkLabel": domain_label },
        "address": {
            "addressStreet1": r.get::<_, String>("street1")?,
            "addressCity": r.get::<_, String>("city")?,
            "addressPostcode": r.get::<_, String>("postcode")?,
            "addressState": r.get::<_, String>("state")?,
            "addressCountry": r.get::<_, String>("country")?
        },
        "employees": r.get::<_, Option<i64>>("employees")?,
        "linkedinLink": { "primaryLinkUrl": linkedin_url },
        "xLink": { "primaryLinkUrl": x_url },
        "annualRecurringRevenue": { "amountMicros": arr_micros, "currencyCode": currency, "amountEur": (arr_micros as f64) / 1_000_000.0 },
        "idealCustomerProfile": r.get::<_, i64>("icp")? != 0,
        "position": r.get::<_, f64>("position")?,
        "createdBy": { "name": r.get::<_, String>("created_by_name")? },
        "createdAt": r.get::<_, String>("created_at")?,
        "updatedAt": r.get::<_, String>("updated_at")?
    }))
}

fn query_companies_internal(conn: &Connection, ws: &str, q: Option<&str>, limit: i64, offset: i64) -> anyhow::Result<(Vec<Value>, i64)> {
    let (items, total, _, _, _) = query_companies_filtered(conn, ws, q, limit, offset, None, None, None)?;
    Ok((items, total))
}

fn query_companies_filtered(
    conn: &Connection,
    ws: &str,
    q: Option<&str>,
    limit: i64,
    offset: i64,
    icp: Option<bool>,
    has_domain: Option<bool>,
    sort: Option<&str>,
) -> anyhow::Result<(Vec<Value>, i64, i64, i64, i64)> {
    let mut conds = Vec::new();
    if ws != "all" {
        conds.push(format!("workspace = '{}'", ws.replace('\'', "")));
    }
    if icp == Some(true) {
        conds.push("icp != 0".to_string());
    }
    if has_domain == Some(true) {
        conds.push("trim(domain_url) != ''".to_string());
    }
    if let Some(sq) = q {
        let pat = sq.trim().to_lowercase().replace('\'', "");
        if !pat.is_empty() {
            conds.push(format!(
                "(lower(name) LIKE '%{pat}%' OR lower(city) LIKE '%{pat}%' OR lower(domain_url) LIKE '%{pat}%')"
            ));
        }
    }
    let where_sql = if conds.is_empty() { "1=1".to_string() } else { conds.join(" AND ") };
    let order_sql = match sort.unwrap_or("") {
        "name_asc" => "name ASC",
        "employees_desc" => "COALESCE(employees, 0) DESC, name ASC",
        "arr_desc" => "COALESCE(arr_micros, 0) DESC, name ASC",
        _ => "updated_at DESC, name ASC",
    };

    let (total, with_domain_count, icp_count, total_arr_micros): (i64, i64, i64, i64) = conn.query_row(
        &format!(
            "SELECT COUNT(*),
                    SUM(CASE WHEN trim(COALESCE(domain_url,'')) != '' THEN 1 ELSE 0 END),
                    SUM(CASE WHEN icp != 0 THEN 1 ELSE 0 END),
                    COALESCE(SUM(arr_micros), 0)
             FROM twenty_companies WHERE {where_sql}"
        ),
        [],
        |r| Ok((r.get(0)?, r.get::<_, Option<i64>>(1)?.unwrap_or(0), r.get::<_, Option<i64>>(2)?.unwrap_or(0), r.get::<_, Option<i64>>(3)?.unwrap_or(0))),
    )?;

    let sql = format!(
        "SELECT * FROM twenty_companies WHERE {where_sql} ORDER BY {order_sql} LIMIT ?1 OFFSET ?2"
    );
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query(params![limit, offset])?;
    let mut items = Vec::new();
    while let Some(r) = rows.next()? {
        items.push(company_row_to_json(r)?);
    }
    Ok((items, total, with_domain_count, icp_count, total_arr_micros))
}

pub async fn list_companies_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(p): Query<ListParams>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, p.workspace.as_deref());
    let limit = p.limit.unwrap_or(50).clamp(1, 500);
    let offset = p.offset.unwrap_or(0).max(0);
    let conn = state.db.lock_conn();
    match query_companies_filtered(&conn, &ws, p.q.as_deref(), limit, offset, p.icp, p.has_domain, p.sort.as_deref()) {
        Ok((items, total, with_domain, icp_cnt, total_arr_micros)) => (
            StatusCode::OK,
            Json(json!({
                "data": items,
                "totalCount": total,
                "workspace": ws,
                "limit": limit,
                "offset": offset,
                "aggregates": {
                    "withDomainCount": with_domain,
                    "icpCount": icp_cnt,
                    "totalArrEur": (total_arr_micros as f64) / 1_000_000.0
                }
            })),
        ),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))),
    }
}

pub async fn get_company_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let res = conn
        .query_row("SELECT * FROM twenty_companies WHERE id = ?1", [&id], company_row_to_json)
        .optional();
    match res {
        Ok(Some(v)) => (StatusCode::OK, Json(v)),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Company not found"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}

pub fn create_company_internal(conn: &Connection, ws: &str, v: &Value) -> anyhow::Result<Value> {
    let id = v.get("id").and_then(|x| x.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string()).unwrap_or_else(|| Uuid::new_v4().to_string());
    let name = v.get("name").and_then(|x| x.as_str()).unwrap_or("Neue Firma").trim().to_string();
    let domain_url = v.pointer("/domainName/primaryLinkUrl").or_else(|| v.get("domainUrl")).and_then(|x| x.as_str()).unwrap_or("").to_string();
    let domain_label = v.pointer("/domainName/primaryLinkLabel").and_then(|x| x.as_str()).unwrap_or(&domain_url).to_string();
    let city = v.pointer("/address/addressCity").or_else(|| v.get("city")).and_then(|x| x.as_str()).unwrap_or("").to_string();
    let street1 = v.pointer("/address/addressStreet1").or_else(|| v.get("street1")).and_then(|x| x.as_str()).unwrap_or("").to_string();
    let postcode = v.pointer("/address/addressPostcode").or_else(|| v.get("postcode")).and_then(|x| x.as_str()).unwrap_or("").to_string();
    let country = v.pointer("/address/addressCountry").or_else(|| v.get("country")).and_then(|x| x.as_str()).unwrap_or("Deutschland").to_string();
    let employees = v.get("employees").and_then(|x| x.as_i64());
    let linkedin = v.pointer("/linkedinLink/primaryLinkUrl").or_else(|| v.get("linkedinUrl")).and_then(|x| x.as_str()).unwrap_or("").to_string();
    let arr_micros = v.pointer("/annualRecurringRevenue/amountMicros").or_else(|| v.get("arrMicros")).and_then(|x| x.as_i64()).unwrap_or(0);
    let icp = if v.get("idealCustomerProfile").and_then(|x| x.as_bool()).unwrap_or(false) { 1i64 } else { 0i64 };
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO twenty_companies
         (id, workspace, name, domain_label, domain_url, street1, city, postcode, state, country, employees, linkedin_url, x_url, arr_micros, arr_currency, icp, position, created_by_name, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, '', ?9, ?10, ?11, '', ?12, 'EUR', ?13, 0, 'Pure Rust CRM', ?14, ?14)",
        params![id, ws, name, domain_label, domain_url, street1, city, postcode, country, employees, linkedin, arr_micros, icp, now],
    )?;
    record_timeline(conn, ws, "company.created", Some(&id), None, None, None, None, &format!("Firma erstellt: {}", name));
    let out = conn.query_row("SELECT * FROM twenty_companies WHERE id = ?1", [&id], company_row_to_json)?;
    Ok(out)
}

pub async fn create_company_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(p): Query<ListParams>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, v.get("workspace").and_then(|x| x.as_str()).or(p.workspace.as_deref()));
    let conn = state.db.lock_conn();
    match create_company_internal(&conn, &ws, &v) {
        Ok(c) => (StatusCode::CREATED, Json(c)),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()}))),
    }
}

pub async fn update_company_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let now = Utc::now().to_rfc3339();
    if let Some(name) = v.get("name").and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE twenty_companies SET name = ?1, updated_at = ?2 WHERE id = ?3", params![name, now, id]);
    }
    if let Some(domain) = v.pointer("/domainName/primaryLinkUrl").or_else(|| v.get("domainUrl")).and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE twenty_companies SET domain_url = ?1, domain_label = ?1, updated_at = ?2 WHERE id = ?3", params![domain, now, id]);
    }
    if let Some(city) = v.pointer("/address/addressCity").or_else(|| v.get("city")).and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE twenty_companies SET city = ?1, updated_at = ?2 WHERE id = ?3", params![city, now, id]);
    }
    if let Some(street1) = v.pointer("/address/addressStreet1").or_else(|| v.get("street1")).and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE twenty_companies SET street1 = ?1, updated_at = ?2 WHERE id = ?3", params![street1, now, id]);
    }
    if let Some(emp) = v.get("employees").and_then(|x| x.as_i64()) {
        let _ = conn.execute("UPDATE twenty_companies SET employees = ?1, updated_at = ?2 WHERE id = ?3", params![emp, now, id]);
    }
    if let Some(arr_micros) = v
        .pointer("/annualRecurringRevenue/amountMicros")
        .and_then(|x| x.as_i64())
        .or_else(|| v.get("arrMicros").and_then(|x| x.as_i64()))
        .or_else(|| v.get("arrEur").and_then(|x| x.as_f64()).map(|e| (e * 1_000_000.0) as i64))
    {
        let _ = conn.execute("UPDATE twenty_companies SET arr_micros = ?1, updated_at = ?2 WHERE id = ?3", params![arr_micros, now, id]);
    }
    if let Some(icp) = v.get("idealCustomerProfile").and_then(|x| x.as_bool()) {
        let _ = conn.execute("UPDATE twenty_companies SET icp = ?1, updated_at = ?2 WHERE id = ?3", params![if icp {1} else {0}, now, id]);
    }
    match conn.query_row("SELECT * FROM twenty_companies WHERE id = ?1", [&id], company_row_to_json).optional() {
        Ok(Some(row)) => (StatusCode::OK, Json(row)),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}

pub async fn delete_company_handler(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let n = conn.execute("DELETE FROM twenty_companies WHERE id = ?1", [&id]).unwrap_or(0);
    Json(json!({"deleted": n > 0, "id": id}))
}

fn person_row_to_json(r: &rusqlite::Row<'_>) -> rusqlite::Result<Value> {
    let first: String = r.get("first_name")?;
    let last: String = r.get("last_name")?;
    let comp_name: Option<String> = r.get("company_name").ok().flatten();
    let cid: Option<String> = r.get("company_id")?;
    Ok(json!({
        "id": r.get::<_, String>("id")?,
        "workspace": r.get::<_, String>("workspace")?,
        "name": { "firstName": first, "lastName": last },
        "fullName": format!("{} {}", first, last).trim().to_string(),
        "emails": { "primaryEmail": r.get::<_, String>("email")? },
        "phones": {
            "primaryPhoneNumber": r.get::<_, String>("phone")?,
            "primaryPhoneCountryCode": r.get::<_, String>("phone_country_code")?,
            "primaryPhoneCallingCode": r.get::<_, String>("phone_calling_code")?
        },
        "jobTitle": r.get::<_, String>("job_title")?,
        "city": r.get::<_, String>("city")?,
        "linkedinLink": { "primaryLinkUrl": r.get::<_, String>("linkedin_url")? },
        "companyId": cid,
        "company": comp_name.map(|cn| json!({"id": cid, "name": cn})),
        "createdBy": { "name": r.get::<_, String>("created_by_name")? },
        "createdAt": r.get::<_, String>("created_at")?,
        "updatedAt": r.get::<_, String>("updated_at")?
    }))
}

pub async fn list_people_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(p): Query<ListParams>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, p.workspace.as_deref());
    let limit = p.limit.unwrap_or(100).clamp(1, 500);
    let offset = p.offset.unwrap_or(0).max(0);
    let conn = state.db.lock_conn();

    let mut conds = Vec::new();
    if ws != "all" {
        conds.push(format!("p.workspace = '{}'", ws.replace('\'', "")));
    }
    if let Some(cid) = &p.company_id {
        conds.push(format!("p.company_id = '{}'", cid.replace('\'', "")));
    }
    if let Some(q) = &p.q {
        let pat = q.trim().to_lowercase().replace('\'', "");
        if !pat.is_empty() {
            conds.push(format!(
                "(lower(p.first_name || ' ' || p.last_name) LIKE '%{pat}%' OR lower(p.email) LIKE '%{pat}%' OR lower(p.city) LIKE '%{pat}%')"
            ));
        }
    }
    let where_sql = if conds.is_empty() { "1=1".to_string() } else { conds.join(" AND ") };
    let total: i64 = conn
        .query_row(&format!("SELECT COUNT(*) FROM twenty_people p WHERE {where_sql}"), [], |r| r.get(0))
        .unwrap_or(0);
    let sql = format!(
        "SELECT p.*, c.name AS company_name FROM twenty_people p
         LEFT JOIN twenty_companies c ON p.company_id = c.id
         WHERE {where_sql}
         ORDER BY p.updated_at DESC, p.last_name ASC LIMIT ?1 OFFSET ?2"
    );
    let mut items = Vec::new();
    if let Ok(mut stmt) = conn.prepare(&sql) {
        if let Ok(mut rows) = stmt.query(params![limit, offset]) {
            while let Ok(Some(r)) = rows.next() {
                if let Ok(v) = person_row_to_json(r) {
                    items.push(v);
                }
            }
        }
    }
    Json(json!({ "data": items, "totalCount": total, "workspace": ws }))
}

pub fn create_person_internal(conn: &Connection, ws: &str, v: &Value) -> anyhow::Result<Value> {
    let id = v.get("id").and_then(|x| x.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string()).unwrap_or_else(|| Uuid::new_v4().to_string());
    let first = v.pointer("/name/firstName").or_else(|| v.get("firstName")).and_then(|x| x.as_str()).unwrap_or("").trim().to_string();
    let last = v.pointer("/name/lastName").or_else(|| v.get("lastName")).and_then(|x| x.as_str()).unwrap_or("").trim().to_string();
    let email = v.pointer("/emails/primaryEmail").or_else(|| v.get("email")).and_then(|x| x.as_str()).unwrap_or("").trim().to_string();
    let phone = v.pointer("/phones/primaryPhoneNumber").or_else(|| v.get("phone")).and_then(|x| x.as_str()).unwrap_or("").trim().to_string();
    let job = v.get("jobTitle").and_then(|x| x.as_str()).unwrap_or("").trim().to_string();
    let city = v.get("city").and_then(|x| x.as_str()).unwrap_or("").trim().to_string();
    let company_id = v.get("companyId").and_then(|x| x.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string());
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO twenty_people
         (id, workspace, first_name, last_name, email, phone, phone_country_code, phone_calling_code, job_title, city, linkedin_url, company_id, position, created_by_name, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'DE', '+49', ?7, ?8, '', ?9, 0, 'Pure Rust CRM', ?10, ?10)",
        params![id, ws, first, last, email, phone, job, city, company_id, now],
    )?;
    record_timeline(conn, ws, "person.created", company_id.as_deref(), Some(&id), None, None, None, &format!("Kontakt erstellt: {} {}", first, last));
    let out = conn.query_row(
        "SELECT p.*, c.name AS company_name FROM twenty_people p LEFT JOIN twenty_companies c ON p.company_id = c.id WHERE p.id = ?1",
        [&id],
        person_row_to_json,
    )?;
    Ok(out)
}

pub async fn create_person_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(p): Query<ListParams>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, v.get("workspace").and_then(|x| x.as_str()).or(p.workspace.as_deref()));
    let conn = state.db.lock_conn();
    match create_person_internal(&conn, &ws, &v) {
        Ok(res) => (StatusCode::CREATED, Json(res)),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()}))),
    }
}

pub async fn update_person_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let now = Utc::now().to_rfc3339();
    if let Some(fnm) = v.pointer("/name/firstName").or_else(|| v.get("firstName")).and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE twenty_people SET first_name = ?1, updated_at = ?2 WHERE id = ?3", params![fnm, now, id]);
    }
    if let Some(lnm) = v.pointer("/name/lastName").or_else(|| v.get("lastName")).and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE twenty_people SET last_name = ?1, updated_at = ?2 WHERE id = ?3", params![lnm, now, id]);
    }
    if let Some(em) = v.pointer("/emails/primaryEmail").or_else(|| v.get("email")).and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE twenty_people SET email = ?1, updated_at = ?2 WHERE id = ?3", params![em, now, id]);
    }
    if let Some(ph) = v.pointer("/phones/primaryPhoneNumber").or_else(|| v.get("phone")).and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE twenty_people SET phone = ?1, updated_at = ?2 WHERE id = ?3", params![ph, now, id]);
    }
    if let Some(jt) = v.get("jobTitle").and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE twenty_people SET job_title = ?1, updated_at = ?2 WHERE id = ?3", params![jt, now, id]);
    }
    if let Some(ct) = v.get("city").and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE twenty_people SET city = ?1, updated_at = ?2 WHERE id = ?3", params![ct, now, id]);
    }
    match conn.query_row(
        "SELECT p.*, c.name AS company_name FROM twenty_people p LEFT JOIN twenty_companies c ON p.company_id = c.id WHERE p.id = ?1",
        [&id],
        person_row_to_json,
    ).optional() {
        Ok(Some(row)) => (StatusCode::OK, Json(row)),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}

pub async fn delete_person_handler(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let n = conn.execute("DELETE FROM twenty_people WHERE id = ?1", [&id]).unwrap_or(0);
    Json(json!({"deleted": n > 0, "id": id}))
}

fn opportunity_row_to_json(r: &rusqlite::Row<'_>) -> rusqlite::Result<Value> {
    let amt_micros: i64 = r.get("amount_micros")?;
    let curr: String = r.get("currency_code")?;
    let cid: Option<String> = r.get("company_id")?;
    let comp_name: Option<String> = r.get("company_name").ok().flatten();
    let pid: Option<String> = r.get("point_of_contact_id")?;
    let contact_name: Option<String> = r.get("contact_name").ok().flatten();
    Ok(json!({
        "id": r.get::<_, String>("id")?,
        "workspace": r.get::<_, String>("workspace")?,
        "name": r.get::<_, String>("name")?,
        "amount": {
            "amountMicros": amt_micros,
            "currencyCode": curr,
            "amountEur": (amt_micros as f64) / 1_000_000.0
        },
        "closeDate": r.get::<_, Option<String>>("close_date")?,
        "stage": r.get::<_, String>("stage")?,
        "companyId": cid,
        "company": comp_name.map(|cn| json!({"id": cid, "name": cn})),
        "pointOfContactId": pid,
        "pointOfContact": contact_name.map(|pn| json!({"id": pid, "fullName": pn})),
        "position": r.get::<_, f64>("position")?,
        "createdBy": { "name": r.get::<_, String>("created_by_name")? },
        "createdAt": r.get::<_, String>("created_at")?,
        "updatedAt": r.get::<_, String>("updated_at")?
    }))
}

pub async fn list_opportunities_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(p): Query<ListParams>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, p.workspace.as_deref());
    let limit = p.limit.unwrap_or(150).clamp(1, 500);
    let offset = p.offset.unwrap_or(0).max(0);
    let conn = state.db.lock_conn();

    let mut conds = Vec::new();
    if ws != "all" {
        conds.push(format!("o.workspace = '{}'", ws.replace('\'', "")));
    }
    if let Some(st) = &p.stage {
        if !st.trim().is_empty() {
            conds.push(format!("upper(o.stage) = '{}'", st.trim().to_uppercase().replace('\'', "")));
        }
    }
    if let Some(cid) = &p.company_id {
        conds.push(format!("o.company_id = '{}'", cid.replace('\'', "")));
    }
    if let Some(q) = &p.q {
        let pat = q.trim().to_lowercase().replace('\'', "");
        if !pat.is_empty() {
            conds.push(format!("(lower(o.name) LIKE '%{pat}%' OR lower(COALESCE(c.name,'')) LIKE '%{pat}%')"));
        }
    }
    let where_sql = if conds.is_empty() { "1=1".to_string() } else { conds.join(" AND ") };
    let total: i64 = conn
        .query_row(
            &format!("SELECT COUNT(*) FROM twenty_opportunities o LEFT JOIN twenty_companies c ON o.company_id = c.id WHERE {where_sql}"),
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    let sql = format!(
        "SELECT o.*, c.name AS company_name, TRIM(COALESCE(p.first_name,'') || ' ' || COALESCE(p.last_name,'')) AS contact_name
         FROM twenty_opportunities o
         LEFT JOIN twenty_companies c ON o.company_id = c.id
         LEFT JOIN twenty_people p ON o.point_of_contact_id = p.id
         WHERE {where_sql}
         ORDER BY o.updated_at DESC, o.position ASC LIMIT ?1 OFFSET ?2"
    );
    let mut items = Vec::new();
    if let Ok(mut stmt) = conn.prepare(&sql) {
        if let Ok(mut rows) = stmt.query(params![limit, offset]) {
            while let Ok(Some(r)) = rows.next() {
                if let Ok(v) = opportunity_row_to_json(r) {
                    items.push(v);
                }
            }
        }
    }
    Json(json!({ "data": items, "totalCount": total, "workspace": ws }))
}

pub fn create_opportunity_internal(conn: &Connection, ws: &str, v: &Value) -> anyhow::Result<Value> {
    let id = v.get("id").and_then(|x| x.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string()).unwrap_or_else(|| Uuid::new_v4().to_string());
    let name = v.get("name").and_then(|x| x.as_str()).unwrap_or("Neue Opportunity").trim().to_string();
    let amount_micros = v.pointer("/amount/amountMicros")
        .and_then(|x| x.as_i64())
        .or_else(|| v.get("amountMicros").and_then(|x| x.as_i64()))
        .or_else(|| v.get("amountEur").and_then(|x| x.as_f64()).map(|e| (e * 1_000_000.0) as i64))
        .unwrap_or(0);
    let curr = v.pointer("/amount/currencyCode").and_then(|x| x.as_str()).unwrap_or("EUR").to_string();
    let stage = v.get("stage").and_then(|x| x.as_str()).filter(|s| !s.is_empty()).unwrap_or("NEW").to_uppercase();
    let close_date = v.get("closeDate").and_then(|x| x.as_str()).map(|s| s.to_string());
    let company_id = v.get("companyId").and_then(|x| x.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string());
    let poc_id = v.get("pointOfContactId").and_then(|x| x.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string());
    let now = Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO twenty_opportunities
         (id, workspace, name, amount_micros, currency_code, close_date, stage, company_id, point_of_contact_id, owner_id, position, created_by_name, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, NULL, 0, 'Pure Rust CRM', ?10, ?10)",
        params![id, ws, name, amount_micros, curr, close_date, stage, company_id, poc_id, now],
    )?;
    record_timeline(conn, ws, "opportunity.created", company_id.as_deref(), poc_id.as_deref(), Some(&id), None, None, &format!("Deal erstellt: {} ({})", name, stage));
    let out = conn.query_row(
        "SELECT o.*, c.name AS company_name, TRIM(COALESCE(p.first_name,'') || ' ' || COALESCE(p.last_name,'')) AS contact_name
         FROM twenty_opportunities o LEFT JOIN twenty_companies c ON o.company_id = c.id LEFT JOIN twenty_people p ON o.point_of_contact_id = p.id WHERE o.id = ?1",
        [&id],
        opportunity_row_to_json,
    )?;
    Ok(out)
}

pub async fn create_opportunity_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(p): Query<ListParams>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, v.get("workspace").and_then(|x| x.as_str()).or(p.workspace.as_deref()));
    let conn = state.db.lock_conn();
    match create_opportunity_internal(&conn, &ws, &v) {
        Ok(res) => (StatusCode::CREATED, Json(res)),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()}))),
    }
}

pub async fn update_opportunity_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let now = Utc::now().to_rfc3339();
    if let Some(st) = v.get("stage").and_then(|x| x.as_str()) {
        let up = st.to_uppercase();
        let _ = conn.execute("UPDATE twenty_opportunities SET stage = ?1, updated_at = ?2 WHERE id = ?3", params![up, now, id]);
    }
    if let Some(nm) = v.get("name").and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE twenty_opportunities SET name = ?1, updated_at = ?2 WHERE id = ?3", params![nm, now, id]);
    }
    if let Some(micros) = v.pointer("/amount/amountMicros").and_then(|x| x.as_i64()).or_else(|| v.get("amountEur").and_then(|x| x.as_f64()).map(|e| (e * 1_000_000.0) as i64)) {
        let _ = conn.execute("UPDATE twenty_opportunities SET amount_micros = ?1, updated_at = ?2 WHERE id = ?3", params![micros, now, id]);
    }
    if let Some(cd) = v.get("closeDate").and_then(|x| x.as_str()) {
        let cd_opt = if cd.trim().is_empty() { None } else { Some(cd) };
        let _ = conn.execute("UPDATE twenty_opportunities SET close_date = ?1, updated_at = ?2 WHERE id = ?3", params![cd_opt, now, id]);
    }
    match conn.query_row(
        "SELECT o.*, c.name AS company_name, TRIM(COALESCE(p.first_name,'') || ' ' || COALESCE(p.last_name,'')) AS contact_name
         FROM twenty_opportunities o LEFT JOIN twenty_companies c ON o.company_id = c.id LEFT JOIN twenty_people p ON o.point_of_contact_id = p.id WHERE o.id = ?1",
        [&id],
        opportunity_row_to_json,
    ).optional() {
        Ok(Some(row)) => (StatusCode::OK, Json(row)),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}

pub async fn delete_opportunity_handler(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let n = conn.execute("DELETE FROM twenty_opportunities WHERE id = ?1", [&id]).unwrap_or(0);
    Json(json!({"deleted": n > 0, "id": id}))
}

fn note_row_to_json(r: &rusqlite::Row<'_>) -> rusqlite::Result<Value> {
    let body: String = r.get("body_markdown")?;
    Ok(json!({
        "id": r.get::<_, String>("id")?,
        "workspace": r.get::<_, String>("workspace")?,
        "title": r.get::<_, String>("title")?,
        "bodyV2": { "markdown": body },
        "bodyMarkdown": body,
        "companyId": r.get::<_, Option<String>>("company_id")?,
        "companyName": r.get::<_, Option<String>>("company_name").ok().flatten(),
        "personId": r.get::<_, Option<String>>("person_id")?,
        "opportunityId": r.get::<_, Option<String>>("opportunity_id")?,
        "createdBy": { "name": r.get::<_, String>("created_by_name")? },
        "createdAt": r.get::<_, String>("created_at")?,
        "updatedAt": r.get::<_, String>("updated_at")?
    }))
}

pub async fn list_notes_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(p): Query<ListParams>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, p.workspace.as_deref());
    let limit = p.limit.unwrap_or(50).clamp(1, 300);
    let offset = p.offset.unwrap_or(0).max(0);
    let conn = state.db.lock_conn();

    let mut conds = Vec::new();
    if ws != "all" {
        conds.push(format!("n.workspace = '{}'", ws.replace('\'', "")));
    }
    if let Some(cid) = &p.company_id {
        conds.push(format!("n.company_id = '{}'", cid.replace('\'', "")));
    }
    if let Some(pid) = &p.person_id {
        conds.push(format!("n.person_id = '{}'", pid.replace('\'', "")));
    }
    if let Some(oid) = &p.opportunity_id {
        conds.push(format!("n.opportunity_id = '{}'", oid.replace('\'', "")));
    }
    if let Some(q) = &p.q {
        let pat = q.trim().to_lowercase().replace('\'', "");
        if !pat.is_empty() {
            conds.push(format!("(lower(n.title) LIKE '%{pat}%' OR lower(n.body_markdown) LIKE '%{pat}%')"));
        }
    }
    let where_sql = if conds.is_empty() { "1=1".to_string() } else { conds.join(" AND ") };
    let total: i64 = conn
        .query_row(&format!("SELECT COUNT(*) FROM twenty_notes n WHERE {where_sql}"), [], |r| r.get(0))
        .unwrap_or(0);
    let sql = format!(
        "SELECT n.*, c.name AS company_name FROM twenty_notes n
         LEFT JOIN twenty_companies c ON n.company_id = c.id
         WHERE {where_sql}
         ORDER BY n.updated_at DESC LIMIT ?1 OFFSET ?2"
    );
    let mut items = Vec::new();
    if let Ok(mut stmt) = conn.prepare(&sql) {
        if let Ok(mut rows) = stmt.query(params![limit, offset]) {
            while let Ok(Some(r)) = rows.next() {
                if let Ok(v) = note_row_to_json(r) {
                    items.push(v);
                }
            }
        }
    }
    Json(json!({ "data": items, "totalCount": total, "workspace": ws }))
}

pub fn create_note_internal(conn: &Connection, ws: &str, v: &Value) -> anyhow::Result<Value> {
    let id = v.get("id").and_then(|x| x.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string()).unwrap_or_else(|| Uuid::new_v4().to_string());
    let title = v.get("title").and_then(|x| x.as_str()).unwrap_or("Notiz").trim().to_string();
    let body = v.pointer("/bodyV2/markdown")
        .or_else(|| v.get("bodyMarkdown"))
        .or_else(|| v.get("body"))
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string();
    let company_id = v.get("companyId").and_then(|x| x.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string());
    let person_id = v.get("personId").and_then(|x| x.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string());
    let opp_id = v.get("opportunityId").and_then(|x| x.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string());
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO twenty_notes (id, workspace, title, body_markdown, company_id, person_id, opportunity_id, position, created_by_name, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0, 'Pure Rust CRM', ?8, ?8)",
        params![id, ws, title, body, company_id, person_id, opp_id, now],
    )?;
    record_timeline(conn, ws, "note.created", company_id.as_deref(), person_id.as_deref(), opp_id.as_deref(), Some(&id), None, &title);
    let out = conn.query_row(
        "SELECT n.*, c.name AS company_name FROM twenty_notes n LEFT JOIN twenty_companies c ON n.company_id = c.id WHERE n.id = ?1",
        [&id],
        note_row_to_json,
    )?;
    Ok(out)
}

pub async fn create_note_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(p): Query<ListParams>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, v.get("workspace").and_then(|x| x.as_str()).or(p.workspace.as_deref()));
    let conn = state.db.lock_conn();
    match create_note_internal(&conn, &ws, &v) {
        Ok(res) => (StatusCode::CREATED, Json(res)),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()}))),
    }
}

pub async fn update_note_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let now = Utc::now().to_rfc3339();
    if let Some(title) = v.get("title").and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE twenty_notes SET title = ?1, updated_at = ?2 WHERE id = ?3", params![title, now, id]);
    }
    if let Some(body) = v.pointer("/bodyV2/markdown").or_else(|| v.get("bodyMarkdown")).or_else(|| v.get("body")).and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE twenty_notes SET body_markdown = ?1, updated_at = ?2 WHERE id = ?3", params![body, now, id]);
    }
    match conn.query_row(
        "SELECT n.*, c.name AS company_name FROM twenty_notes n LEFT JOIN twenty_companies c ON n.company_id = c.id WHERE n.id = ?1",
        [&id],
        note_row_to_json,
    ).optional() {
        Ok(Some(row)) => (StatusCode::OK, Json(row)),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}

pub async fn delete_note_handler(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let n = conn.execute("DELETE FROM twenty_notes WHERE id = ?1", [&id]).unwrap_or(0);
    Json(json!({"deleted": n > 0, "id": id}))
}

fn task_row_to_json(r: &rusqlite::Row<'_>) -> rusqlite::Result<Value> {
    let body: String = r.get("body_markdown")?;
    Ok(json!({
        "id": r.get::<_, String>("id")?,
        "workspace": r.get::<_, String>("workspace")?,
        "title": r.get::<_, String>("title")?,
        "bodyV2": { "markdown": body },
        "bodyMarkdown": body,
        "status": r.get::<_, String>("status")?,
        "dueAt": r.get::<_, Option<String>>("due_at")?,
        "companyId": r.get::<_, Option<String>>("company_id")?,
        "companyName": r.get::<_, Option<String>>("company_name").ok().flatten(),
        "personId": r.get::<_, Option<String>>("person_id")?,
        "opportunityId": r.get::<_, Option<String>>("opportunity_id")?,
        "createdBy": { "name": r.get::<_, String>("created_by_name")? },
        "createdAt": r.get::<_, String>("created_at")?,
        "updatedAt": r.get::<_, String>("updated_at")?
    }))
}

pub async fn list_tasks_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(p): Query<ListParams>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, p.workspace.as_deref());
    let limit = p.limit.unwrap_or(100).clamp(1, 300);
    let offset = p.offset.unwrap_or(0).max(0);
    let conn = state.db.lock_conn();

    let mut conds = Vec::new();
    if ws != "all" {
        conds.push(format!("t.workspace = '{}'", ws.replace('\'', "")));
    }
    if let Some(st) = &p.status {
        if !st.trim().is_empty() {
            conds.push(format!("upper(t.status) = '{}'", st.trim().to_uppercase().replace('\'', "")));
        }
    }
    if let Some(cid) = &p.company_id {
        conds.push(format!("t.company_id = '{}'", cid.replace('\'', "")));
    }
    if let Some(q) = &p.q {
        let pat = q.trim().to_lowercase().replace('\'', "");
        if !pat.is_empty() {
            conds.push(format!("(lower(t.title) LIKE '%{pat}%' OR lower(t.body_markdown) LIKE '%{pat}%')"));
        }
    }
    let where_sql = if conds.is_empty() { "1=1".to_string() } else { conds.join(" AND ") };
    let total: i64 = conn
        .query_row(&format!("SELECT COUNT(*) FROM twenty_tasks t WHERE {where_sql}"), [], |r| r.get(0))
        .unwrap_or(0);
    let sql = format!(
        "SELECT t.*, c.name AS company_name FROM twenty_tasks t
         LEFT JOIN twenty_companies c ON t.company_id = c.id
         WHERE {where_sql}
         ORDER BY CASE t.status WHEN 'TODO' THEN 0 WHEN 'IN_PROGRESS' THEN 1 ELSE 2 END, t.updated_at DESC LIMIT ?1 OFFSET ?2"
    );
    let mut items = Vec::new();
    if let Ok(mut stmt) = conn.prepare(&sql) {
        if let Ok(mut rows) = stmt.query(params![limit, offset]) {
            while let Ok(Some(r)) = rows.next() {
                if let Ok(v) = task_row_to_json(r) {
                    items.push(v);
                }
            }
        }
    }
    Json(json!({ "data": items, "totalCount": total, "workspace": ws }))
}

pub fn create_task_internal(conn: &Connection, ws: &str, v: &Value) -> anyhow::Result<Value> {
    let id = v.get("id").and_then(|x| x.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string()).unwrap_or_else(|| Uuid::new_v4().to_string());
    let title = v.get("title").and_then(|x| x.as_str()).unwrap_or("Aufgabe").trim().to_string();
    let body = v.pointer("/bodyV2/markdown").or_else(|| v.get("bodyMarkdown")).or_else(|| v.get("body")).and_then(|x| x.as_str()).unwrap_or("").to_string();
    let status = v.get("status").and_then(|x| x.as_str()).filter(|s| !s.is_empty()).unwrap_or("TODO").to_uppercase();
    let due_at = v.get("dueAt").and_then(|x| x.as_str()).map(|s| s.to_string());
    let company_id = v.get("companyId").and_then(|x| x.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string());
    let person_id = v.get("personId").and_then(|x| x.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string());
    let opp_id = v.get("opportunityId").and_then(|x| x.as_str()).filter(|s| !s.is_empty()).map(|s| s.to_string());
    let now = Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO twenty_tasks (id, workspace, title, body_markdown, status, due_at, company_id, person_id, opportunity_id, assignee_id, position, created_by_name, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, NULL, 0, 'Pure Rust CRM', ?10, ?10)",
        params![id, ws, title, body, status, due_at, company_id, person_id, opp_id, now],
    )?;
    record_timeline(conn, ws, "task.created", company_id.as_deref(), person_id.as_deref(), opp_id.as_deref(), None, Some(&id), &title);
    let out = conn.query_row(
        "SELECT t.*, c.name AS company_name FROM twenty_tasks t LEFT JOIN twenty_companies c ON t.company_id = c.id WHERE t.id = ?1",
        [&id],
        task_row_to_json,
    )?;
    Ok(out)
}

pub async fn create_task_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(p): Query<ListParams>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, v.get("workspace").and_then(|x| x.as_str()).or(p.workspace.as_deref()));
    let conn = state.db.lock_conn();
    match create_task_internal(&conn, &ws, &v) {
        Ok(res) => (StatusCode::CREATED, Json(res)),
        Err(e) => (StatusCode::BAD_REQUEST, Json(json!({"error": e.to_string()}))),
    }
}

pub async fn update_task_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let now = Utc::now().to_rfc3339();
    if let Some(st) = v.get("status").and_then(|x| x.as_str()) {
        let up = st.to_uppercase();
        let _ = conn.execute("UPDATE twenty_tasks SET status = ?1, updated_at = ?2 WHERE id = ?3", params![up, now, id]);
    }
    if let Some(title) = v.get("title").and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE twenty_tasks SET title = ?1, updated_at = ?2 WHERE id = ?3", params![title, now, id]);
    }
    if let Some(body) = v.pointer("/bodyV2/markdown").or_else(|| v.get("bodyMarkdown")).and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE twenty_tasks SET body_markdown = ?1, updated_at = ?2 WHERE id = ?3", params![body, now, id]);
    }
    if let Some(due) = v.get("dueAt").and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE twenty_tasks SET due_at = ?1, updated_at = ?2 WHERE id = ?3", params![due, now, id]);
    }
    match conn.query_row(
        "SELECT t.*, c.name AS company_name FROM twenty_tasks t LEFT JOIN twenty_companies c ON t.company_id = c.id WHERE t.id = ?1",
        [&id],
        task_row_to_json,
    ).optional() {
        Ok(Some(row)) => (StatusCode::OK, Json(row)),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Not found"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}

pub async fn delete_task_handler(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let n = conn.execute("DELETE FROM twenty_tasks WHERE id = ?1", [&id]).unwrap_or(0);
    Json(json!({"deleted": n > 0, "id": id}))
}

pub async fn record_timeline_handler(
    State(state): State<AppState>,
    Query(p): Query<ListParams>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let mut notes = Vec::new();
    let mut tasks = Vec::new();
    let mut opps = Vec::new();
    let mut people = Vec::new();
    let mut events = Vec::new();

    if let Some(cid) = &p.company_id {
        if let Ok(mut stmt) = conn.prepare("SELECT n.*, c.name AS company_name FROM twenty_notes n LEFT JOIN twenty_companies c ON n.company_id = c.id WHERE n.company_id = ?1 ORDER BY n.created_at DESC LIMIT 30") {
            if let Ok(mut rows) = stmt.query([cid]) { while let Ok(Some(r)) = rows.next() { if let Ok(v) = note_row_to_json(r) { notes.push(v); } } }
        }
        if let Ok(mut stmt) = conn.prepare("SELECT t.*, c.name AS company_name FROM twenty_tasks t LEFT JOIN twenty_companies c ON t.company_id = c.id WHERE t.company_id = ?1 ORDER BY t.created_at DESC LIMIT 30") {
            if let Ok(mut rows) = stmt.query([cid]) { while let Ok(Some(r)) = rows.next() { if let Ok(v) = task_row_to_json(r) { tasks.push(v); } } }
        }
        if let Ok(mut stmt) = conn.prepare("SELECT o.*, c.name AS company_name, TRIM(COALESCE(p.first_name,'') || ' ' || COALESCE(p.last_name,'')) AS contact_name FROM twenty_opportunities o LEFT JOIN twenty_companies c ON o.company_id = c.id LEFT JOIN twenty_people p ON o.point_of_contact_id = p.id WHERE o.company_id = ?1 ORDER BY o.created_at DESC LIMIT 20") {
            if let Ok(mut rows) = stmt.query([cid]) { while let Ok(Some(r)) = rows.next() { if let Ok(v) = opportunity_row_to_json(r) { opps.push(v); } } }
        }
        if let Ok(mut stmt) = conn.prepare("SELECT p.*, c.name AS company_name FROM twenty_people p LEFT JOIN twenty_companies c ON p.company_id = c.id WHERE p.company_id = ?1 ORDER BY p.created_at DESC LIMIT 20") {
            if let Ok(mut rows) = stmt.query([cid]) { while let Ok(Some(r)) = rows.next() { if let Ok(v) = person_row_to_json(r) { people.push(v); } } }
        }
        if let Ok(mut stmt) = conn.prepare("SELECT happens_at, name, actor, details_json FROM twenty_timeline WHERE company_id = ?1 ORDER BY happens_at DESC LIMIT 30") {
            if let Ok(mut rows) = stmt.query([cid]) {
                while let Ok(Some(r)) = rows.next() {
                    events.push(json!({
                        "happensAt": r.get::<_, String>(0).unwrap_or_default(),
                        "name": r.get::<_, String>(1).unwrap_or_default(),
                        "actor": r.get::<_, String>(2).unwrap_or_default(),
                        "details": r.get::<_, String>(3).unwrap_or_default(),
                    }));
                }
            }
        }
    } else if let Some(pid) = &p.person_id {
        if let Ok(mut stmt) = conn.prepare("SELECT n.*, c.name AS company_name FROM twenty_notes n LEFT JOIN twenty_companies c ON n.company_id = c.id WHERE n.person_id = ?1 ORDER BY n.created_at DESC LIMIT 30") {
            if let Ok(mut rows) = stmt.query([pid]) { while let Ok(Some(r)) = rows.next() { if let Ok(v) = note_row_to_json(r) { notes.push(v); } } }
        }
        if let Ok(mut stmt) = conn.prepare("SELECT t.*, c.name AS company_name FROM twenty_tasks t LEFT JOIN twenty_companies c ON t.company_id = c.id WHERE t.person_id = ?1 ORDER BY t.created_at DESC LIMIT 30") {
            if let Ok(mut rows) = stmt.query([pid]) { while let Ok(Some(r)) = rows.next() { if let Ok(v) = task_row_to_json(r) { tasks.push(v); } } }
        }
    } else if let Some(oid) = &p.opportunity_id {
        if let Ok(mut stmt) = conn.prepare("SELECT n.*, c.name AS company_name FROM twenty_notes n LEFT JOIN twenty_companies c ON n.company_id = c.id WHERE n.opportunity_id = ?1 ORDER BY n.created_at DESC LIMIT 30") {
            if let Ok(mut rows) = stmt.query([oid]) { while let Ok(Some(r)) = rows.next() { if let Ok(v) = note_row_to_json(r) { notes.push(v); } } }
        }
        if let Ok(mut stmt) = conn.prepare("SELECT t.*, c.name AS company_name FROM twenty_tasks t LEFT JOIN twenty_companies c ON t.company_id = c.id WHERE t.opportunity_id = ?1 ORDER BY t.created_at DESC LIMIT 30") {
            if let Ok(mut rows) = stmt.query([oid]) { while let Ok(Some(r)) = rows.next() { if let Ok(v) = task_row_to_json(r) { tasks.push(v); } } }
        }
    }

    Json(json!({
        "notes": notes,
        "tasks": tasks,
        "opportunities": opps,
        "people": people,
        "events": events
    }))
}

fn record_timeline(
    conn: &Connection,
    ws: &str,
    name: &str,
    company_id: Option<&str>,
    person_id: Option<&str>,
    opp_id: Option<&str>,
    note_id: Option<&str>,
    task_id: Option<&str>,
    summary: &str,
) {
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let details = json!({ "summary": summary }).to_string();
    let _ = conn.execute(
        "INSERT INTO twenty_timeline (id, workspace, happens_at, name, company_id, person_id, opportunity_id, note_id, task_id, actor, details_json)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 'Pure Rust CRM', ?10)",
        params![id, ws, now, name, company_id, person_id, opp_id, note_id, task_id, details],
    );
}

// =========================================================================
// Twenty-Compatible GraphQL Engine (Queries + Multi-Aliased Mutations)
// =========================================================================

pub async fn graphql_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, body.get("workspace").and_then(|x| x.as_str()));
    let query = body.get("query").and_then(|x| x.as_str()).unwrap_or("");
    let variables = body.get("variables").cloned().unwrap_or_else(|| json!({}));
    let res = execute_graphql(&state.db, &ws, query, &variables);
    (StatusCode::OK, Json(res))
}

pub fn execute_graphql(db: &Database, ws: &str, query: &str, variables: &Value) -> Value {
    let q_norm = substitute_graphql_vars(query, variables);
    let conn = db.lock_conn();
    let trimmed = q_norm.trim();

    if trimmed.contains("__schema") || trimmed.contains("__type") {
        return json!({
            "data": {
                "__schema": {
                    "queryType": { "name": "Query" },
                    "mutationType": { "name": "Mutation" },
                    "types": [
                        { "name": "Company", "kind": "OBJECT" },
                        { "name": "Person", "kind": "OBJECT" },
                        { "name": "Opportunity", "kind": "OBJECT" },
                        { "name": "Note", "kind": "OBJECT" },
                        { "name": "Task", "kind": "OBJECT" }
                    ]
                }
            }
        });
    }

    if trimmed.starts_with("mutation") || trimmed.contains("createOpportunity") || trimmed.contains("createPerson") || trimmed.contains("createCompany") || trimmed.contains("createNote") || trimmed.contains("createTask") || trimmed.contains("updateOpportunity") {
        return execute_graphql_mutations(&conn, ws, &q_norm);
    }

    let mut data_map = serde_json::Map::new();
    let ops = [
        "companies",
        "people",
        "opportunities",
        "notes",
        "tasks",
        "workspaceMembers",
    ];
    let mut matched_any = false;

    for op in ops {
        if let Some(pos) = find_op_call(&q_norm, op) {
            matched_any = true;
            let args = extract_parens_block(&q_norm[pos..]).unwrap_or_default();
            let data_obj = parse_gql_object_arg(&args);
            let limit = data_obj.get("first").and_then(|x| x.as_i64()).unwrap_or(50).clamp(1, 500);
            let q_filter = data_obj
                .pointer("/filter/name/ilike")
                .or_else(|| data_obj.pointer("/filter/name/like"))
                .or_else(|| data_obj.pointer("/filter/name/eq"))
                .and_then(|x| x.as_str())
                .map(|s| s.trim_matches('%').to_string());

            match op {
                "companies" => {
                    if let Ok((items, total)) = query_companies_internal(&conn, ws, q_filter.as_deref(), limit, 0) {
                        let edges: Vec<Value> = items.into_iter().map(|n| json!({ "node": n })).collect();
                        data_map.insert(op.to_string(), json!({ "edges": edges, "totalCount": total, "pageInfo": { "hasNextPage": total > limit } }));
                    }
                }
                "people" => {
                    let sql = if ws == "all" {
                        "SELECT p.*, c.name AS company_name FROM twenty_people p LEFT JOIN twenty_companies c ON p.company_id = c.id ORDER BY p.updated_at DESC LIMIT ?1".to_string()
                    } else {
                        format!("SELECT p.*, c.name AS company_name FROM twenty_people p LEFT JOIN twenty_companies c ON p.company_id = c.id WHERE p.workspace = '{}' ORDER BY p.updated_at DESC LIMIT ?1", ws.replace('\'', ""))
                    };
                    let mut edges = Vec::new();
                    if let Ok(mut stmt) = conn.prepare(&sql) {
                        if let Ok(mut rows) = stmt.query([limit]) {
                            while let Ok(Some(r)) = rows.next() {
                                if let Ok(n) = person_row_to_json(r) {
                                    edges.push(json!({ "node": n }));
                                }
                            }
                        }
                    }
                    data_map.insert(op.to_string(), json!({ "totalCount": edges.len(), "edges": edges }));
                }
                "opportunities" => {
                    let sql = if ws == "all" {
                        "SELECT o.*, c.name AS company_name, '' AS contact_name FROM twenty_opportunities o LEFT JOIN twenty_companies c ON o.company_id = c.id ORDER BY o.updated_at DESC LIMIT ?1".to_string()
                    } else {
                        format!("SELECT o.*, c.name AS company_name, '' AS contact_name FROM twenty_opportunities o LEFT JOIN twenty_companies c ON o.company_id = c.id WHERE o.workspace = '{}' ORDER BY o.updated_at DESC LIMIT ?1", ws.replace('\'', ""))
                    };
                    let mut edges = Vec::new();
                    if let Ok(mut stmt) = conn.prepare(&sql) {
                        if let Ok(mut rows) = stmt.query([limit]) {
                            while let Ok(Some(r)) = rows.next() {
                                if let Ok(n) = opportunity_row_to_json(r) {
                                    edges.push(json!({ "node": n }));
                                }
                            }
                        }
                    }
                    data_map.insert(op.to_string(), json!({ "totalCount": edges.len(), "edges": edges }));
                }
                "notes" => {
                    let sql = if ws == "all" {
                        "SELECT n.*, c.name AS company_name FROM twenty_notes n LEFT JOIN twenty_companies c ON n.company_id = c.id ORDER BY n.updated_at DESC LIMIT ?1".to_string()
                    } else {
                        format!("SELECT n.*, c.name AS company_name FROM twenty_notes n LEFT JOIN twenty_companies c ON n.company_id = c.id WHERE n.workspace = '{}' ORDER BY n.updated_at DESC LIMIT ?1", ws.replace('\'', ""))
                    };
                    let mut edges = Vec::new();
                    if let Ok(mut stmt) = conn.prepare(&sql) {
                        if let Ok(mut rows) = stmt.query([limit]) {
                            while let Ok(Some(r)) = rows.next() {
                                if let Ok(n) = note_row_to_json(r) {
                                    edges.push(json!({ "node": n }));
                                }
                            }
                        }
                    }
                    data_map.insert(op.to_string(), json!({ "totalCount": edges.len(), "edges": edges }));
                }
                "tasks" => {
                    let sql = if ws == "all" {
                        "SELECT t.*, c.name AS company_name FROM twenty_tasks t LEFT JOIN twenty_companies c ON t.company_id = c.id ORDER BY t.updated_at DESC LIMIT ?1".to_string()
                    } else {
                        format!("SELECT t.*, c.name AS company_name FROM twenty_tasks t LEFT JOIN twenty_companies c ON t.company_id = c.id WHERE t.workspace = '{}' ORDER BY t.updated_at DESC LIMIT ?1", ws.replace('\'', ""))
                    };
                    let mut edges = Vec::new();
                    if let Ok(mut stmt) = conn.prepare(&sql) {
                        if let Ok(mut rows) = stmt.query([limit]) {
                            while let Ok(Some(r)) = rows.next() {
                                if let Ok(n) = task_row_to_json(r) {
                                    edges.push(json!({ "node": n }));
                                }
                            }
                        }
                    }
                    data_map.insert(op.to_string(), json!({ "totalCount": edges.len(), "edges": edges }));
                }
                "workspaceMembers" => {
                    let mut edges = Vec::new();
                    if let Ok(mut stmt) = conn.prepare("SELECT id, first_name, last_name, email FROM twenty_members WHERE workspace = ?1 LIMIT ?2") {
                        if let Ok(mut rows) = stmt.query(params![ws, limit]) {
                            while let Ok(Some(r)) = rows.next() {
                                edges.push(json!({
                                    "node": {
                                        "id": r.get::<_, String>(0).unwrap_or_default(),
                                        "name": {
                                            "firstName": r.get::<_, String>(1).unwrap_or_default(),
                                            "lastName": r.get::<_, String>(2).unwrap_or_default()
                                        },
                                        "userEmail": r.get::<_, String>(3).unwrap_or_default()
                                    }
                                }));
                            }
                        }
                    }
                    data_map.insert(op.to_string(), json!({ "edges": edges }));
                }
                _ => {}
            }
        }
    }

    if !matched_any {
        return json!({
            "data": {},
            "errors": [{ "message": "Unrecognized GraphQL query on Pure-Rust Twenty engine" }]
        });
    }
    json!({ "data": Value::Object(data_map) })
}

fn execute_graphql_mutations(conn: &Connection, ws: &str, query: &str) -> Value {
    let mut data_map = serde_json::Map::new();
    let mut errors = Vec::new();
    let mut i = 0usize;
    let bytes = query.as_bytes();
    let mutation_names = [
        "createCompany",
        "createPerson",
        "createOpportunity",
        "updateOpportunity",
        "createNoteTarget",
        "createNote",
        "createTaskTarget",
        "createTask",
        "updateTask",
    ];

    while i < bytes.len() {
        let mut next_hit: Option<(usize, &str)> = None;
        for m in mutation_names {
            if let Some(rel) = query[i..].find(m) {
                let abs = i + rel;
                let prev_ok = abs == 0 || !bytes[abs - 1].is_ascii_alphanumeric();
                let after = abs + m.len();
                let next_ok = after < bytes.len() && matches!(bytes[after], b'(' | b' ' | b'\n' | b'\r' | b'\t');
                if prev_ok && next_ok && (next_hit.is_none() || abs < next_hit.unwrap().0) {
                    next_hit = Some((abs, m));
                }
            }
        }

        let Some((pos, m_name)) = next_hit else {
            break;
        };

        // Check for optional GraphQL alias right before `m_name`, e.g. `opp: createOpportunity`
        let prefix = query[i..pos].trim_end();
        let alias = if prefix.ends_with(':') {
            let without_colon = prefix[..prefix.len() - 1].trim_end();
            without_colon
                .rsplit(|c: char| !c.is_ascii_alphanumeric() && c != '_')
                .next()
                .filter(|s| !s.is_empty())
                .unwrap_or(m_name)
                .to_string()
        } else {
            m_name.to_string()
        };

        let after_name = pos + m_name.len();
        let (args_str, consumed) = extract_parens_with_len(&query[after_name..]).unwrap_or_default();
        i = after_name + consumed;

        let arg_val = parse_gql_object_arg(&args_str);
        let input_data = arg_val.get("data").unwrap_or(&arg_val);

        let res = match m_name {
            "createCompany" => create_company_internal(conn, ws, input_data),
            "createPerson" => create_person_internal(conn, ws, input_data),
            "createOpportunity" => create_opportunity_internal(conn, ws, input_data),
            "updateOpportunity" => {
                let id = arg_val.get("id").and_then(|x| x.as_str()).unwrap_or("");
                if !id.is_empty() {
                    if let Some(st) = input_data.get("stage").and_then(|x| x.as_str()) {
                        let _ = conn.execute("UPDATE twenty_opportunities SET stage = ?1, updated_at = ?2 WHERE id = ?3", params![st.to_uppercase(), Utc::now().to_rfc3339(), id]);
                    }
                    conn.query_row(
                        "SELECT o.*, c.name AS company_name, '' AS contact_name FROM twenty_opportunities o LEFT JOIN twenty_companies c ON o.company_id = c.id WHERE o.id = ?1",
                        [id],
                        opportunity_row_to_json,
                    ).map_err(anyhow::Error::from)
                } else {
                    Err(anyhow::anyhow!("Missing id for updateOpportunity"))
                }
            }
            "createNote" => create_note_internal(conn, ws, input_data),
            "createNoteTarget" => {
                let nid = input_data.get("noteId").and_then(|x| x.as_str()).unwrap_or("");
                let _ = conn.execute(
                    "UPDATE twenty_notes SET company_id = COALESCE(?1, company_id), person_id = COALESCE(?2, person_id), opportunity_id = COALESCE(?3, opportunity_id) WHERE id = ?4",
                    params![
                        input_data.get("targetCompanyId").and_then(|x| x.as_str()),
                        input_data.get("targetPersonId").and_then(|x| x.as_str()),
                        input_data.get("targetOpportunityId").and_then(|x| x.as_str()),
                        nid
                    ],
                );
                Ok(json!({ "id": Uuid::new_v4().to_string(), "noteId": nid }))
            }
            "createTask" => create_task_internal(conn, ws, input_data),
            "createTaskTarget" => {
                let tid = input_data.get("taskId").and_then(|x| x.as_str()).unwrap_or("");
                let _ = conn.execute(
                    "UPDATE twenty_tasks SET company_id = COALESCE(?1, company_id), person_id = COALESCE(?2, person_id), opportunity_id = COALESCE(?3, opportunity_id) WHERE id = ?4",
                    params![
                        input_data.get("targetCompanyId").and_then(|x| x.as_str()),
                        input_data.get("targetPersonId").and_then(|x| x.as_str()),
                        input_data.get("targetOpportunityId").and_then(|x| x.as_str()),
                        tid
                    ],
                );
                Ok(json!({ "id": Uuid::new_v4().to_string(), "taskId": tid }))
            }
            "updateTask" => {
                let id = arg_val.get("id").and_then(|x| x.as_str()).unwrap_or("");
                if let Some(st) = input_data.get("status").and_then(|x| x.as_str()) {
                    let _ = conn.execute("UPDATE twenty_tasks SET status = ?1, updated_at = ?2 WHERE id = ?3", params![st.to_uppercase(), Utc::now().to_rfc3339(), id]);
                }
                conn.query_row(
                    "SELECT t.*, c.name AS company_name FROM twenty_tasks t LEFT JOIN twenty_companies c ON t.company_id = c.id WHERE t.id = ?1",
                    [id],
                    task_row_to_json,
                ).map_err(anyhow::Error::from)
            }
            _ => Err(anyhow::anyhow!("Unsupported mutation")),
        };

        match res {
            Ok(val) => {
                data_map.insert(alias, val);
            }
            Err(e) => {
                errors.push(json!({ "message": e.to_string(), "path": [alias] }));
            }
        }
    }

    if errors.is_empty() {
        json!({ "data": Value::Object(data_map) })
    } else {
        json!({ "data": Value::Object(data_map), "errors": errors })
    }
}

fn substitute_graphql_vars(q: &str, vars: &Value) -> String {
    let mut out = q.to_string();
    if let Some(obj) = vars.as_object() {
        for (k, v) in obj {
            let ph = format!("${}", k);
            let repl = match v {
                Value::String(s) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
                _ => v.to_string(),
            };
            out = out.replace(&ph, &repl);
        }
    }
    out
}

fn find_op_call(s: &str, op: &str) -> Option<usize> {
    let mut start = 0;
    while let Some(rel) = s[start..].find(op) {
        let abs = start + rel;
        let prev_ok = abs == 0 || !s.as_bytes()[abs - 1].is_ascii_alphanumeric();
        let after = abs + op.len();
        let next_ok = after >= s.len() || matches!(s.as_bytes()[after], b'(' | b'{' | b' ' | b'\n' | b'\r' | b'\t');
        if prev_ok && next_ok {
            return Some(after);
        }
        start = after;
    }
    None
}

fn extract_parens_block(s: &str) -> Option<String> {
    extract_parens_with_len(s).map(|(inner, _)| inner)
}

fn extract_parens_with_len(s: &str) -> Option<(String, usize)> {
    let trimmed_start = s.len() - s.trim_start().len();
    let rest = &s[trimmed_start..];
    if !rest.starts_with('(') {
        return None;
    }
    let mut depth = 0i32;
    let mut in_str = false;
    let mut escaped = false;
    for (idx, ch) in rest.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if in_str {
            if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_str = false;
            }
            continue;
        }
        match ch {
            '"' => in_str = true,
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some((rest[1..idx].to_string(), trimmed_start + idx + 1));
                }
            }
            _ => {}
        }
    }
    None
}

fn parse_gql_object_arg(arg_content: &str) -> Value {
    let trimmed = arg_content.trim();
    if trimmed.is_empty() {
        return json!({});
    }
    let wrapped = format!("{{{}}}", trimmed);
    if let Ok(v) = serde_json::from_str::<Value>(&wrapped) {
        return v;
    }
    // Quote unquoted GraphQL object keys (`name: "Foo"` -> `"name": "Foo"`) while preserving strings.
    let mut json_like = String::with_capacity(wrapped.len() + 32);
    let chars: Vec<char> = wrapped.chars().collect();
    let mut i = 0;
    let mut in_str = false;
    while i < chars.len() {
        let c = chars[i];
        if in_str {
            json_like.push(c);
            if c == '\\' && i + 1 < chars.len() {
                i += 1;
                json_like.push(chars[i]);
            } else if c == '"' {
                in_str = false;
            }
            i += 1;
            continue;
        }
        if c == '"' {
            in_str = true;
            json_like.push(c);
            i += 1;
            continue;
        }
        if (c.is_ascii_alphabetic() || c == '_')
            && (json_like.ends_with('{') || json_like.ends_with(',') || json_like.ends_with(' ') || json_like.ends_with('\n'))
        {
            let mut j = i;
            while j < chars.len() && (chars[j].is_ascii_alphanumeric() || chars[j] == '_') {
                j += 1;
            }
            let ident: String = chars[i..j].iter().collect();
            let mut k = j;
            while k < chars.len() && chars[k].is_whitespace() {
                k += 1;
            }
            if k < chars.len() && chars[k] == ':' {
                json_like.push('"');
                json_like.push_str(&ident);
                json_like.push('"');
                i = j;
                continue;
            } else if matches!(ident.as_str(), "NEW" | "SCREENING" | "MEETING" | "PROPOSAL" | "CUSTOMER" | "TODO" | "IN_PROGRESS" | "DONE" | "EUR" | "USD") {
                json_like.push('"');
                json_like.push_str(&ident);
                json_like.push('"');
                i = j;
                continue;
            }
        }
        json_like.push(c);
        i += 1;
    }
    // Normalize whitespace commas if GraphQL used newlines instead of commas between fields
    serde_json::from_str::<Value>(&json_like).unwrap_or_else(|_| json!({}))
}

// =========================================================================
// Native 0.6B/1.7B Tool-Calling Agent for Twenty CRM + n8n
// =========================================================================

#[derive(Debug, Deserialize)]
pub struct TwentyAgentReq {
    #[serde(alias = "prompt", alias = "query", alias = "text")]
    pub message: String,
    pub workspace: Option<String>,
    pub model: Option<String>,
}

pub fn twenty_n8n_tool_definitions() -> Value {
    json!([
        {
            "type": "function",
            "function": {
                "name": "Twenty_Search_Companies",
                "description": "Sucht Firmen in Twenty CRM nach Name, Stadt oder Domain.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "Suchbegriff (Firmenname, Stadt oder Domain)" },
                        "limit": { "type": "integer", "description": "Maximale Trefferanzahl" }
                    },
                    "required": ["query"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "Twenty_Create_Person",
                "description": "Legt einen neuen Ansprechpartner / Kontakt in Twenty CRM an.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "firstName": { "type": "string" },
                        "lastName": { "type": "string" },
                        "email": { "type": "string" },
                        "phone": { "type": "string" },
                        "city": { "type": "string" },
                        "companyId": { "type": "string" }
                    },
                    "required": ["firstName", "lastName"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "Twenty_Create_Opportunity",
                "description": "Erstellt einen neuen Deal / Opportunity in der Twenty CRM Pipeline.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "name": { "type": "string" },
                        "stage": { "type": "string", "enum": ["NEW", "SCREENING", "MEETING", "PROPOSAL", "CUSTOMER"] },
                        "amountEur": { "type": "number" },
                        "companyId": { "type": "string" }
                    },
                    "required": ["name", "stage"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "Twenty_Create_Note",
                "description": "Erstellt eine Notiz in Twenty CRM, optional verknuepft mit Firma, Kontakt oder Deal.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "title": { "type": "string" },
                        "bodyMarkdown": { "type": "string" },
                        "companyId": { "type": "string" },
                        "personId": { "type": "string" },
                        "opportunityId": { "type": "string" }
                    },
                    "required": ["title", "bodyMarkdown"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "Twenty_Create_Task",
                "description": "Erstellt eine Follow-up Aufgabe in Twenty CRM.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "title": { "type": "string" },
                        "status": { "type": "string", "enum": ["TODO", "IN_PROGRESS", "DONE"] },
                        "dueAt": { "type": "string" },
                        "companyId": { "type": "string" }
                    },
                    "required": ["title"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "N8N_Execute_Workflow",
                "description": "Startet einen lokalen n8n Workflow in der Pure-Rust Engine.",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "workflowId": { "type": "string" },
                        "payload": { "type": "object" }
                    },
                    "required": ["workflowId"]
                }
            }
        },
        {
            "type": "function",
            "function": {
                "name": "Gmail_Create_Draft",
                "description": "Erstellt einen E-Mail-Entwurf (niemals automatischer Versand!).",
                "parameters": {
                    "type": "object",
                    "properties": {
                        "to": { "type": "string" },
                        "subject": { "type": "string" },
                        "body": { "type": "string" }
                    },
                    "required": ["to", "subject", "body"]
                }
            }
        }
    ])
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedToolCall {
    pub name: String,
    pub arguments: Value,
}

fn parse_gemma_kv_args(args_str: &str) -> Value {
    let mut map = serde_json::Map::new();
    let bytes = args_str.as_bytes();
    let len = bytes.len();
    let mut i = 0;

    while i < len {
        while i < len && (bytes[i].is_ascii_whitespace() || bytes[i] == b',') {
            i += 1;
        }
        if i >= len || bytes[i] == b'}' {
            break;
        }
        let key_start = i;
        while i < len && bytes[i] != b':' && bytes[i] != b'}' {
            i += 1;
        }
        if i >= len || bytes[i] != b':' {
            break;
        }
        let key = args_str[key_start..i].trim().trim_matches('"').trim_matches('\'').to_string();
        i += 1; // skip ':'
        while i < len && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i >= len {
            break;
        }

        let val: Value = if args_str[i..].starts_with("<escape>") {
            i += "<escape>".len();
            if let Some(rel_end) = args_str[i..].find("<escape>") {
                let s = args_str[i..i + rel_end].trim().to_string();
                i += rel_end + "<escape>".len();
                Value::String(s)
            } else {
                let s = args_str[i..].trim().to_string();
                i = len;
                Value::String(s)
            }
        } else if bytes[i] == b'"' || bytes[i] == b'\'' {
            let q = bytes[i];
            i += 1;
            let val_start = i;
            while i < len && bytes[i] != q {
                if bytes[i] == b'\\' && i + 1 < len {
                    i += 2;
                } else {
                    i += 1;
                }
            }
            let s = args_str[val_start..i].trim().to_string();
            if i < len {
                i += 1;
            }
            Value::String(s)
        } else {
            let val_start = i;
            let mut depth = 0i32;
            while i < len {
                if bytes[i] == b'{' || bytes[i] == b'[' {
                    depth += 1;
                } else if bytes[i] == b'}' || bytes[i] == b']' {
                    if depth == 0 {
                        break;
                    }
                    depth -= 1;
                } else if bytes[i] == b',' && depth == 0 {
                    break;
                }
                i += 1;
            }
            let raw = args_str[val_start..i].trim();
            if raw.eq_ignore_ascii_case("true") {
                Value::Bool(true)
            } else if raw.eq_ignore_ascii_case("false") {
                Value::Bool(false)
            } else if let Ok(n) = raw.parse::<i64>() {
                json!(n)
            } else if let Ok(f) = raw.parse::<f64>() {
                json!(f)
            } else if (raw.starts_with('{') && raw.ends_with('}')) || (raw.starts_with('[') && raw.ends_with(']')) {
                serde_json::from_str::<Value>(raw).unwrap_or_else(|_| Value::String(raw.to_string()))
            } else {
                Value::String(raw.to_string())
            }
        };

        if !key.is_empty() {
            map.insert(key, val);
        }
    }
    Value::Object(map)
}

pub fn extract_tool_calls_from_ollama(resp_json: &Value) -> Vec<ParsedToolCall> {
    let mut out = Vec::new();
    if let Some(arr) = resp_json.pointer("/message/tool_calls").and_then(|x| x.as_array()) {
        for tc in arr {
            if let Some(name) = tc.pointer("/function/name").and_then(|x| x.as_str()) {
                let raw_args = tc.pointer("/function/arguments").cloned().unwrap_or_else(|| json!({}));
                let arguments = match raw_args {
                    Value::String(s) => serde_json::from_str::<Value>(&s).unwrap_or_else(|_| json!({"raw": s})),
                    other => other,
                };
                out.push(ParsedToolCall {
                    name: name.to_string(),
                    arguments,
                });
            }
        }
    }
    if out.is_empty() {
        if let Some(content) = resp_json.pointer("/message/content").and_then(|x| x.as_str()) {
            let cleaned = strip_think_tags(content);

            // 1. Qwen / LFM2.5 XML format: <tool_call>{...}</tool_call>
            let mut rest = cleaned.as_str();
            while let Some(s) = rest.find("<tool_call>") {
                if let Some(e) = rest[s..].find("</tool_call>") {
                    let inner = rest[s + "<tool_call>".len()..s + e].trim();
                    if let Ok(v) = serde_json::from_str::<Value>(inner) {
                        if let Some(name) = v.get("name").and_then(|x| x.as_str()) {
                            let arguments = v.get("arguments").cloned().unwrap_or_else(|| json!({}));
                            out.push(ParsedToolCall {
                                name: name.to_string(),
                                arguments,
                            });
                        }
                    }
                    rest = &rest[s + e + "</tool_call>".len()..];
                } else {
                    break;
                }
            }

            // 2. FunctionGemma format: <start_function_call>call:Name{k:<escape>v<escape>}[<end_function_call>]
            if out.is_empty() && cleaned.contains("<start_function_call>") {
                let mut grest = cleaned.as_str();
                while let Some(s) = grest.find("<start_function_call>") {
                    let after_tag = &grest[s + "<start_function_call>".len()..];
                    let chunk_end = after_tag.find("<end_function_call>").unwrap_or(after_tag.len());
                    let chunk = after_tag[..chunk_end].trim();
                    let call_body = chunk.strip_prefix("call:").unwrap_or(chunk);
                    if let Some(brace_idx) = call_body.find('{') {
                        let name = call_body[..brace_idx].trim().to_string();
                        let args_part = call_body[brace_idx + 1..].trim();
                        let args_trimmed = args_part.strip_suffix('}').unwrap_or(args_part);
                        if !name.is_empty() {
                            out.push(ParsedToolCall {
                                name,
                                arguments: parse_gemma_kv_args(args_trimmed),
                            });
                        }
                    }
                    if chunk_end < after_tag.len() {
                        grest = &after_tag[chunk_end + "<end_function_call>".len()..];
                    } else {
                        break;
                    }
                }
            }

            // 3. Raw JSON object with { "name": "...", "arguments": {...} }
            if out.is_empty() {
                let raw_json_candidate = cleaned
                    .trim()
                    .trim_start_matches("```json")
                    .trim_start_matches("```")
                    .trim_end_matches("```")
                    .trim();
                if let Ok(v) = serde_json::from_str::<Value>(raw_json_candidate) {
                    if let Some(name) = v.get("name").and_then(|x| x.as_str()) {
                        let arguments = v.get("arguments").cloned().unwrap_or_else(|| json!({}));
                        out.push(ParsedToolCall {
                            name: name.to_string(),
                            arguments,
                        });
                    }
                }
            }
        }
    }
    out
}

pub async fn execute_single_crm_tool(
    db: &Database,
    ws: &str,
    tc: &ParsedToolCall,
) -> Value {
    match tc.name.as_str() {
        "Twenty_Search_Companies" => {
            let query = tc.arguments.get("query").and_then(|x| x.as_str()).unwrap_or("");
            let limit = tc.arguments.get("limit").and_then(|x| x.as_i64()).unwrap_or(10).clamp(1, 30);
            let conn = db.lock_conn();
            match query_companies_internal(&conn, ws, Some(query), limit, 0) {
                Ok((items, total)) => json!({ "tool": tc.name, "ok": true, "totalCount": total, "items": items }),
                Err(e) => json!({ "tool": tc.name, "ok": false, "error": e.to_string() }),
            }
        }
        "Twenty_Create_Person" => {
            let conn = db.lock_conn();
            match create_person_internal(&conn, ws, &tc.arguments) {
                Ok(p) => json!({ "tool": tc.name, "ok": true, "person": p }),
                Err(e) => json!({ "tool": tc.name, "ok": false, "error": e.to_string() }),
            }
        }
        "Twenty_Create_Opportunity" => {
            let conn = db.lock_conn();
            match create_opportunity_internal(&conn, ws, &tc.arguments) {
                Ok(o) => json!({ "tool": tc.name, "ok": true, "opportunity": o }),
                Err(e) => json!({ "tool": tc.name, "ok": false, "error": e.to_string() }),
            }
        }
        "Twenty_Create_Note" => {
            let conn = db.lock_conn();
            match create_note_internal(&conn, ws, &tc.arguments) {
                Ok(n) => json!({ "tool": tc.name, "ok": true, "note": n }),
                Err(e) => json!({ "tool": tc.name, "ok": false, "error": e.to_string() }),
            }
        }
        "Twenty_Create_Task" => {
            let conn = db.lock_conn();
            match create_task_internal(&conn, ws, &tc.arguments) {
                Ok(t) => json!({ "tool": tc.name, "ok": true, "task": t }),
                Err(e) => json!({ "tool": tc.name, "ok": false, "error": e.to_string() }),
            }
        }
        "Gmail_Create_Draft" => {
            let to = tc.arguments.get("to").and_then(|x| x.as_str()).unwrap_or("");
            let subj = tc.arguments.get("subject").and_then(|x| x.as_str()).unwrap_or("");
            let body = tc.arguments.get("body").and_then(|x| x.as_str()).unwrap_or("");
            let draft_id = format!("draft-{}", &Uuid::new_v4().to_string()[..8]);
            let conn = db.lock_conn();
            let _ = create_note_internal(
                &conn,
                ws,
                &json!({
                    "title": format!("E-Mail Entwurf an {}: {}", to, subj),
                    "bodyMarkdown": format!("**An:** {}\n**Betreff:** {}\n**Status:** Entwurf (draftOnly=true, nicht gesendet)\n\n{}", to, subj, body)
                }),
            );
            json!({
                "tool": tc.name,
                "ok": true,
                "draftOnly": true,
                "draftId": draft_id,
                "to": to,
                "subject": subj,
                "message": "E-Mail-Entwurf gespeichert (kein automatischer Versand)."
            })
        }
        "N8N_Execute_Workflow" => {
            let wf_id = tc.arguments.get("workflowId").and_then(|x| x.as_str()).unwrap_or("");
            let payload = tc.arguments.get("payload").cloned().unwrap_or_else(|| json!({}));
            match crate::n8n_engine::execute_workflow_by_id(db, wf_id, payload, None).await {
                Ok(res) => json!({ "tool": tc.name, "ok": true, "execution": res }),
                Err(e) => json!({ "tool": tc.name, "ok": false, "error": e.to_string() }),
            }
        }
        other => json!({ "tool": other, "ok": false, "error": "Unknown tool" }),
    }
}

pub async fn twenty_agent_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<TwentyAgentReq>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, req.workspace.as_deref());
    let model = req.model.clone().unwrap_or_else(default_twenty_model);
    let url = format!("{}/api/chat", ollama_base_url());
    let local_now = Utc::now() + chrono::Duration::hours(1);
    let today_str = local_now.format("%Y-%m-%d").to_string();
    let tomorrow_str = (local_now + chrono::Duration::days(1)).format("%Y-%m-%d").to_string();
    let tomorrow_10 = format!("{}T10:00:00+01:00", tomorrow_str);
    let sys_prompt = if model.contains("gemma") {
        format!(
            "You are a model that can do function calling with the following functions. \
             Workspace: '{}'. Heutiges Datum: {} (morgen: {}, morgen 10 Uhr = {}). \
             Antworte danach kurz auf Deutsch, erfinde niemals Daten und nutze bei E-Mails strikt Gmail_Create_Draft (draftOnly=true).",
            ws, today_str, tomorrow_str, tomorrow_10
        )
    } else {
        format!(
            "Du bist der integrierte Twenty CRM & n8n KI-Assistent fuer Workspace '{}' (Conrad KI / Pala). \
             Heutiges Datum: {} (morgen: {}, morgen 10 Uhr = {}). \
             Antworte auf Deutsch, praezise und nutze die verfuegbaren Tools fuer CRM-Suchen, Kontakte, Deals, Notizen, Aufgaben, \
             n8n-Workflows und E-Mail-Entwuerfe (immer draftOnly=true, niemals E-Mails direkt versenden).",
            ws, today_str, tomorrow_str, tomorrow_10
        )
    };

    let payload = json!({
        "model": model,
        "stream": false,
        "think": false,
        "options": { "temperature": 0.1, "num_ctx": 4096, "num_predict": 384 },
        "messages": [
            { "role": "system", "content": sys_prompt },
            { "role": "user", "content": req.message }
        ],
        "tools": twenty_n8n_tool_definitions()
    });

    let started = std::time::Instant::now();
    let resp = match http_client().post(&url).json(&payload).send().await {
        Ok(r) => r,
        Err(e) => {
            return (
                StatusCode::BAD_GATEWAY,
                Json(json!({ "ok": false, "error": format!("Ollama nicht erreichbar: {}", e), "model": model })),
            );
        }
    };

    let status = resp.status();
    let mut raw: Value = resp.json().await.unwrap_or_else(|_| json!({}));
    if !status.is_success() {
        // Auto-fallback to 1.7b if requested 0.6b isn't loaded yet
        if model == "conrad-n8n-agent:0.6b" {
            let fallback_model = "conrad-n8n-agent:1.7b".to_string();
            let mut p2 = payload.clone();
            p2["model"] = json!(fallback_model);
            if let Ok(r2) = http_client().post(&url).json(&p2).send().await {
                if r2.status().is_success() {
                    raw = r2.json().await.unwrap_or(raw);
                    return finalize_agent_response(&state.db, &ws, &fallback_model, &req.message, &url, &sys_prompt, raw, started).await;
                }
            }
        }
        return (
            StatusCode::BAD_GATEWAY,
            Json(json!({ "ok": false, "model": model, "ollamaStatus": status.as_u16(), "raw": raw })),
        );
    }

    finalize_agent_response(&state.db, &ws, &model, &req.message, &url, &sys_prompt, raw, started).await
}

async fn finalize_agent_response(
    db: &Database,
    ws: &str,
    model: &str,
    user_msg: &str,
    url: &str,
    sys_prompt: &str,
    raw: Value,
    started: std::time::Instant,
) -> (StatusCode, Json<Value>) {
    let tool_calls = extract_tool_calls_from_ollama(&raw);
    let mut tool_results = Vec::new();
    for tc in &tool_calls {
        tool_results.push(execute_single_crm_tool(db, ws, tc).await);
    }

    let mut reply_text = raw
        .pointer("/message/content")
        .and_then(|x| x.as_str())
        .map(strip_think_tags)
        .unwrap_or_default();
    if reply_text.contains("<tool_call>") || reply_text.contains("<start_function_call>") {
        reply_text.clear();
    }

    if !tool_calls.is_empty() && reply_text.is_empty() {
        let mut msgs = vec![
            json!({ "role": "system", "content": sys_prompt }),
            json!({ "role": "user", "content": user_msg }),
            json!({
                "role": "assistant",
                "content": "",
                "tool_calls": tool_calls.iter().map(|tc| json!({
                    "type": "function",
                    "function": { "name": tc.name, "arguments": tc.arguments }
                })).collect::<Vec<_>>()
            }),
        ];
        for (idx, tr) in tool_results.iter().enumerate() {
            let tc_name = tool_calls.get(idx).map(|t| t.name.as_str()).unwrap_or("tool");
            msgs.push(json!({
                "role": "tool",
                "name": tc_name,
                "content": tr.to_string()
            }));
        }
        let synth_payload = json!({
            "model": model,
            "stream": false,
            "think": false,
            "options": { "temperature": 0.1, "num_ctx": 4096, "num_predict": 256 },
            "messages": msgs
        });
        if let Ok(r2) = http_client().post(url).json(&synth_payload).send().await {
            if let Ok(v2) = r2.json::<Value>().await {
                let candidate = v2
                    .pointer("/message/content")
                    .and_then(|x| x.as_str())
                    .map(strip_think_tags)
                    .unwrap_or_default();
                if !candidate.contains("<start_function_call>") && !candidate.contains("<tool_call>") {
                    reply_text = candidate;
                }
            }
        }
        if reply_text.is_empty() {
            reply_text = format!(
                "{} Tool(s) ausgefuehrt: {}",
                tool_calls.len(),
                tool_calls.iter().map(|t| t.name.as_str()).collect::<Vec<_>>().join(", ")
            );
        }
    }

    (
        StatusCode::OK,
        Json(json!({
            "ok": true,
            "model": model,
            "workspace": ws,
            "reply": reply_text,
            "toolCalls": tool_calls,
            "toolResults": tool_results,
            "elapsedMs": started.elapsed().as_millis() as i64
        })),
    )
}

fn read_proc_kb_field(path: &str, key: &str) -> i64 {
    if let Ok(txt) = std::fs::read_to_string(path) {
        for line in txt.lines() {
            if let Some(rest) = line.strip_prefix(key) {
                let val_str = rest.trim().split_whitespace().next().unwrap_or("0");
                if let Ok(kb) = val_str.parse::<i64>() {
                    return kb;
                }
            }
        }
    }
    0
}

pub async fn showcase_metrics_handler(State(state): State<AppState>) -> impl IntoResponse {
    // 1. Gather Ollama tags & running models BEFORE locking SQLite
    let mut registered_models: Vec<Value> = Vec::new();
    if let Ok(resp) = http_client().get(format!("{}/api/tags", ollama_base_url())).send().await {
        if let Ok(v) = resp.json::<Value>().await {
            if let Some(arr) = v.get("models").and_then(|x| x.as_array()) {
                for m in arr {
                    let name = m.get("name").and_then(|x| x.as_str()).unwrap_or("");
                    let size_bytes = m.get("size").and_then(|x| x.as_i64()).unwrap_or(0);
                    registered_models.push(json!({
                        "name": name,
                        "sizeMb": ((size_bytes as f64) / 1_048_576.0 * 10.0).round() / 10.0
                    }));
                }
            }
        }
    }

    let mut active_ollama: Vec<Value> = Vec::new();
    if let Ok(resp) = http_client().get(format!("{}/api/ps", ollama_base_url())).send().await {
        if let Ok(v) = resp.json::<Value>().await {
            if let Some(arr) = v.get("models").and_then(|x| x.as_array()) {
                for m in arr {
                    let name = m.get("name").and_then(|x| x.as_str()).unwrap_or("");
                    let size_bytes = m.get("size").and_then(|x| x.as_i64()).unwrap_or(0);
                    active_ollama.push(json!({
                        "name": name,
                        "ramMb": ((size_bytes as f64) / 1_048_576.0 * 10.0).round() / 10.0
                    }));
                }
            }
        }
    }

    // 2. Read live Linux /proc memory telemetry
    let rss_kb = read_proc_kb_field("/proc/self/status", "VmRSS:");
    let mem_total_kb = read_proc_kb_field("/proc/meminfo", "MemTotal:");
    let mem_avail_kb = read_proc_kb_field("/proc/meminfo", "MemAvailable:");
    let swap_total_kb = read_proc_kb_field("/proc/meminfo", "SwapTotal:");
    let swap_free_kb = read_proc_kb_field("/proc/meminfo", "SwapFree:");

    // 3. Query real SQLite counts across Twenty CRM & n8n Engine
    let conn = state.db.lock_conn();
    let count_q = |sql: &str| -> i64 { conn.query_row(sql, [], |r| r.get(0)).unwrap_or(0) };

    let companies = count_q("SELECT COUNT(*) FROM twenty_companies");
    let people = count_q("SELECT COUNT(*) FROM twenty_people");
    let opportunities = count_q("SELECT COUNT(*) FROM twenty_opportunities");
    let notes = count_q("SELECT COUNT(*) FROM twenty_notes");
    let tasks = count_q("SELECT COUNT(*) FROM twenty_tasks");
    let n8n_workflows = count_q("SELECT COUNT(*) FROM n8n_workflows");
    let n8n_active = count_q("SELECT COUNT(*) FROM n8n_workflows WHERE active = 1");
    let n8n_execs = count_q("SELECT COUNT(*) FROM n8n_executions");

    let db_bytes = std::fs::metadata(
        std::env::var("CONRAD_DB_PATH").unwrap_or_else(|_| "/opt/conrad-os/conrad_os.db".to_string()),
    )
    .map(|m| m.len() as i64)
    .unwrap_or(0);

    let legacy_total_eur: f64 = 5000.0;
    let rust_server_eur: f64 = 19.0;
    let savings_pct = ((legacy_total_eur - rust_server_eur) / legacy_total_eur * 1000.0_f64).round() / 10.0_f64;

    Json(json!({
        "ok": true,
        "timestamp": Utc::now().to_rfc3339(),
        "rustBinary": {
            "name": "conrad-crm (Twenty CRM + n8n Engine + Relays)",
            "rssKb": rss_kb,
            "rssMb": ((rss_kb as f64) / 1024.0 * 10.0).round() / 10.0,
            "sqliteDbMb": ((db_bytes as f64) / 1_048_576.0 * 10.0).round() / 10.0
        },
        "serverMemory": {
            "totalMb": mem_total_kb / 1024,
            "availableMb": mem_avail_kb / 1024,
            "swapUsedMb": (swap_total_kb - swap_free_kb) / 1024
        },
        "crm": {
            "companies": companies,
            "people": people,
            "opportunities": opportunities,
            "notes": notes,
            "tasks": tasks,
            "totalRecords": companies + people + opportunities + notes + tasks
        },
        "n8n": {
            "workflows": n8n_workflows,
            "activeWorkflows": n8n_active,
            "executions": n8n_execs
        },
        "ollama": {
            "defaultModel": default_twenty_model(),
            "registeredModels": registered_models,
            "loadedNow": active_ollama
        },
        "costComparison": {
            "legacyMonthlyEur": legacy_total_eur,
            "legacyAnnualEur": legacy_total_eur * 12.0,
            "conradMonthlyEur": rust_server_eur,
            "conradAnnualEur": rust_server_eur * 12.0,
            "monthlySavingsEur": legacy_total_eur - rust_server_eur,
            "annualSavingsEur": (legacy_total_eur - rust_server_eur) * 12.0,
            "savingsPercent": savings_pct,
            "breakdown": [
                {
                    "category": "Unternehmens-CRM (15 Lizenzen, Workflows & API)",
                    "legacyStack": "Salesforce / HubSpot Enterprise",
                    "legacyEur": 1800,
                    "rustStack": "Pure-Rust Twenty CRM (/twenty + /graphql, unbegrenzte Nutzer)",
                    "rustEur": 0
                },
                {
                    "category": "Workflow-Automatisierung & Integrationen",
                    "legacyStack": "Zapier Company / Make Enterprise / n8n Cloud",
                    "legacyEur": 850,
                    "rustStack": "Pure-Rust n8n Engine (/n8n, 58 Workflows) + Relays",
                    "rustEur": 0
                },
                {
                    "category": "KI-Assistent, Tool Calling & E-Mail-Triage",
                    "legacyStack": "OpenAI / Claude Team + KI-Add-ons",
                    "legacyEur": 1450,
                    "rustStack": "Nachtrainierte 0.6B / 270M / 1.2B Modelle (Ollama 127.0.0.1)",
                    "rustEur": 0
                },
                {
                    "category": "Cloud-Datenbank, Node/Docker-Cluster & Hosting",
                    "legacyStack": "Managed PostgreSQL, Redis, Kubernetes-Cluster & Monitoring",
                    "legacyEur": 900,
                    "rustStack": "1x Hetzner Cloud-Server (<20 €/Monat) + SQLite WAL",
                    "rustEur": 19
                }
            ]
        }
    }))
}
