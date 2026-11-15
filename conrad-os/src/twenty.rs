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
        if low == "conrad-ki" || low == "handwerk" || low == "all" {
            return low;
        }
    }
    if let Some(host) = headers.get("host").and_then(|v| v.to_str().ok()) {
        if host.to_lowercase().contains("handwerk.") {
            return "handwerk".to_string();
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

    let needs_reseed = {
        let conn = db.lock_conn();
        has_real_twenty_data(&conn)
    };

    if needs_reseed {
        let stats = seed_twenty_demo_data(db)?;
        tracing::info!("Conrad CRM Demo-Daten initialisiert/bereinigt: {:?}", stats);
    } else {
        let existing_companies: i64 = {
            let conn = db.lock_conn();
            conn.query_row("SELECT COUNT(*) FROM twenty_companies", [], |r| r.get(0))
                .unwrap_or(0)
        };
        tracing::info!(
            "Conrad CRM bereit ({} Demo-Firmen in SQLite)",
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

        CREATE TABLE IF NOT EXISTS twenty_field_metadata (
            id TEXT PRIMARY KEY,
            workspace TEXT NOT NULL DEFAULT 'all',
            object_name TEXT NOT NULL,
            name TEXT NOT NULL,
            label TEXT NOT NULL,
            field_type TEXT NOT NULL DEFAULT 'TEXT',
            options_json TEXT NOT NULL DEFAULT '[]',
            default_value TEXT NOT NULL DEFAULT '',
            is_custom INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS twenty_views (
            id TEXT PRIMARY KEY,
            workspace TEXT NOT NULL DEFAULT 'conrad-ki',
            object_name TEXT NOT NULL,
            name TEXT NOT NULL,
            layout TEXT NOT NULL DEFAULT 'table',
            filter_json TEXT NOT NULL DEFAULT '{}',
            sort_json TEXT NOT NULL DEFAULT '{}',
            columns_json TEXT NOT NULL DEFAULT '[]',
            position INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS twenty_favorites (
            id TEXT PRIMARY KEY,
            workspace TEXT NOT NULL DEFAULT 'conrad-ki',
            label TEXT NOT NULL,
            target_type TEXT NOT NULL DEFAULT 'object',
            target_id TEXT NOT NULL,
            position INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS twenty_webhooks (
            id TEXT PRIMARY KEY,
            workspace TEXT NOT NULL DEFAULT 'conrad-ki',
            target_url TEXT NOT NULL,
            event_pattern TEXT NOT NULL DEFAULT '*',
            description TEXT NOT NULL DEFAULT '',
            active INTEGER NOT NULL DEFAULT 1,
            secret TEXT NOT NULL DEFAULT '',
            created_at TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS twenty_api_keys (
            id TEXT PRIMARY KEY,
            workspace TEXT NOT NULL DEFAULT 'conrad-ki',
            name TEXT NOT NULL,
            key_prefix TEXT NOT NULL,
            key_hash TEXT NOT NULL,
            expires_at TEXT,
            created_at TEXT NOT NULL
        );
        ",
    )?;

    for tbl in [
        "twenty_companies",
        "twenty_people",
        "twenty_opportunities",
        "twenty_notes",
        "twenty_tasks",
    ] {
        let _ = conn.execute(
            &format!("ALTER TABLE {tbl} ADD COLUMN custom_fields_json TEXT NOT NULL DEFAULT '{{}}'"),
            [],
        );
    }

    let now = Utc::now().to_rfc3339();
    conn.execute("DELETE FROM twenty_workspaces WHERE id = 'pala'", [])?;
    conn.execute(
        "INSERT INTO twenty_workspaces (id, name, domain, locale, created_at) VALUES (?1, ?2, ?3, 'de-DE', ?4)
         ON CONFLICT(id) DO UPDATE SET name = excluded.name, domain = excluded.domain",
        params!["conrad-ki", "Conrad KI Demo", "crm.conrad-ki.com", now],
    )?;
    conn.execute(
        "INSERT INTO twenty_workspaces (id, name, domain, locale, created_at) VALUES (?1, ?2, ?3, 'de-DE', ?4)
         ON CONFLICT(id) DO UPDATE SET name = excluded.name, domain = excluded.domain",
        params!["handwerk", "Handwerk & Mittelstand Demo", "handwerk.crm.conrad-ki.com", now],
    )?;
    Ok(())
}

fn attach_custom_fields(mut obj: Value, r: &rusqlite::Row<'_>) -> Value {
    let raw = r
        .get::<_, Option<String>>("custom_fields_json")
        .ok()
        .flatten()
        .unwrap_or_else(|| "{}".to_string());
    let custom: serde_json::Map<String, Value> = serde_json::from_str(&raw).unwrap_or_default();
    if let Some(m) = obj.as_object_mut() {
        for (k, v) in &custom {
            if !m.contains_key(k) {
                m.insert(k.clone(), v.clone());
            }
        }
        m.insert("customFields".into(), Value::Object(custom));
    }
    obj
}

fn save_custom_fields_for_record(conn: &Connection, table: &str, object_name: &str, id: &str, v: &Value) {
    let existing_raw: String = conn
        .query_row(&format!("SELECT COALESCE(custom_fields_json, '{{}}') FROM {table} WHERE id = ?1"), [id], |r| r.get(0))
        .unwrap_or_else(|_| "{}".into());
    let mut map: serde_json::Map<String, Value> = serde_json::from_str(&existing_raw).unwrap_or_default();
    let mut changed = false;

    if let Some(cf) = v.get("customFields").and_then(|x| x.as_object()) {
        for (k, val) in cf {
            map.insert(k.clone(), val.clone());
            changed = true;
        }
    }

    if let Ok(mut st) = conn.prepare("SELECT name FROM twenty_field_metadata WHERE object_name = ?1") {
        if let Ok(mut rows) = st.query([object_name]) {
            while let Ok(Some(r)) = rows.next() {
                if let Ok(fname) = r.get::<_, String>(0) {
                    if let Some(val) = v.get(&fname) {
                        map.insert(fname, val.clone());
                        changed = true;
                    }
                }
            }
        }
    }

    if changed {
        let now = Utc::now().to_rfc3339();
        let _ = conn.execute(
            &format!("UPDATE {table} SET custom_fields_json = ?1, updated_at = ?2 WHERE id = ?3"),
            params![Value::Object(map).to_string(), now, id],
        );
    }
}

fn has_real_twenty_data(conn: &Connection) -> bool {
    let companies_count: i64 = conn
        .query_row("SELECT COUNT(*) FROM twenty_companies", [], |r| r.get(0))
        .unwrap_or(0);
    if companies_count == 0 || companies_count > 60 {
        return true;
    }

    let bad_ws: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM twenty_workspaces WHERE id = 'pala' OR LOWER(name) LIKE '%pala%'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    if bad_ws > 0 {
        return true;
    }

    let has_handwerk: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM twenty_workspaces WHERE id = 'handwerk'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    if has_handwerk == 0 {
        return true;
    }

    let bad_companies: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM twenty_companies
             WHERE workspace = 'pala'
                OR created_by_name IN ('Hermes', 'Pure Rust CRM')
                OR LOWER(name) LIKE '%ohrladen%'
                OR LOWER(name) LIKE '%vvs%'
                OR LOWER(name) LIKE '%pala%'
                OR LOWER(name) LIKE '%schlossgarten%'
                OR LOWER(name) LIKE '%imkerei%'
                OR LOWER(name) LIKE '%gruia%'
                OR LOWER(name) LIKE '%halmhuber%'
                OR LOWER(name) LIKE '%a.t.u%'
                OR LOWER(name) LIKE '%heflott%'
                OR LOWER(name) LIKE '%schuster%'
                OR LOWER(domain_url) LIKE '%pala%'
                OR LOWER(domain_url) LIKE '%vvs%'
                OR LOWER(domain_url) LIKE '%ohrladen%'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    if bad_companies > 0 {
        return true;
    }

    let bad_people: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM twenty_people
             WHERE workspace = 'pala'
                OR LOWER(email) LIKE '%pala%'
                OR LOWER(email) LIKE '%vvs%'
                OR LOWER(email) LIKE '%ohrladen%'
                OR LOWER(email) LIKE '%online.de%'
                OR LOWER(email) LIKE '%gmail.com%'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    if bad_people > 0 {
        return true;
    }

    let bad_members: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM twenty_members
             WHERE workspace = 'pala'
                OR LOWER(first_name) LIKE '%pala%'
                OR LOWER(email) LIKE '%pala%'
                OR LOWER(email) LIKE '%online.de%'
                OR LOWER(email) LIKE '%gmail.com%'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    if bad_members > 0 {
        return true;
    }

    let bad_notes: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM twenty_notes
             WHERE workspace = 'pala'
                OR LOWER(title) LIKE '%ohrladen%'
                OR LOWER(title) LIKE '%vvs%'
                OR LOWER(title) LIKE '%pala%'",
            [],
            |r| r.get(0),
        )
        .unwrap_or(0);
    bad_notes > 0
}

pub fn seed_twenty_demo_data(db: &Database) -> anyhow::Result<Value> {
    let mut conn = db.lock_conn();
    let tx = conn.transaction()?;
    let now = Utc::now().to_rfc3339();

    for tbl in [
        "twenty_timeline",
        "twenty_tasks",
        "twenty_notes",
        "twenty_opportunities",
        "twenty_people",
        "twenty_companies",
        "twenty_members",
        "twenty_favorites",
        "twenty_views",
        "twenty_field_metadata",
        "twenty_webhooks",
        "twenty_api_keys",
    ] {
        tx.execute(&format!("DELETE FROM {tbl}"), [])?;
    }

    tx.execute("DELETE FROM twenty_workspaces WHERE id = 'pala'", [])?;
    tx.execute(
        "INSERT INTO twenty_workspaces (id, name, domain, locale, created_at)
         VALUES ('conrad-ki', 'Conrad KI Demo', 'crm.conrad-ki.com', 'de-DE', ?1)
         ON CONFLICT(id) DO UPDATE SET name = excluded.name, domain = excluded.domain",
        params![now],
    )?;
    tx.execute(
        "INSERT INTO twenty_workspaces (id, name, domain, locale, created_at)
         VALUES ('handwerk', 'Handwerk & Mittelstand Demo', 'handwerk.crm.conrad-ki.com', 'de-DE', ?1)
         ON CONFLICT(id) DO UPDATE SET name = excluded.name, domain = excluded.domain",
        params![now],
    )?;

    let members = [
        ("mem-demo-1", "conrad-ki", "Lukas", "Bauer", "lukas.bauer@conrad-ki-demo.de"),
        ("mem-demo-2", "conrad-ki", "Hannah", "Schaefer", "hannah.schaefer@conrad-ki-demo.de"),
        ("mem-demo-3", "handwerk", "Tobias", "Hoffmann", "tobias.hoffmann@handwerk-demo.de"),
        ("mem-demo-4", "handwerk", "Miriam", "Keller", "miriam.keller@handwerk-demo.de"),
    ];
    for (id, ws, fnm, lnm, email) in members {
        tx.execute(
            "INSERT INTO twenty_members (id, workspace, first_name, last_name, email, avatar_url, locale, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, '', 'de-DE', ?6)",
            params![id, ws, fnm, lnm, email, now],
        )?;
    }

    // Custom Field Metadata fuer Demo-Firmen
    for (fid, name, label, ftype, opts, def_val) in [
        (
            "cf-demo-branche",
            "branche",
            "Branche",
            "SELECT",
            r#"["Handwerk","Dienstleistung","Kanzlei","Praxis","Produktion","Logistik"]"#,
            "Handwerk",
        ),
        ("cf-demo-paket", "paket", "KI-Paket", "TEXT", "[]", "Wachstum 69 EUR/Mo"),
        ("cf-demo-score", "kiScore", "KI-Score", "NUMBER", "[]", "85"),
    ] {
        tx.execute(
            "INSERT INTO twenty_field_metadata
             (id, workspace, object_name, name, label, field_type, options_json, default_value, is_custom, created_at)
             VALUES (?1, 'all', 'company', ?2, ?3, ?4, ?5, ?6, 1, ?7)",
            params![fid, name, label, ftype, opts, def_val, now],
        )?;
    }

    // 30 fiktive deutsche KMU-Firmen (18 in conrad-ki, 12 in handwerk)
    let companies: [(&str, &str, &str, &str, &str, &str, &str, &str, i64, i64, i64, &str, &str, i64); 30] = [
        ("cmp-demo-01", "conrad-ki", "Meisterbetrieb Elektro Weber GmbH", "elektro-weber-demo.de", "Mannheimer Str. 42", "Schwetzingen", "68723", "Baden-Württemberg", 24, 14_400_000_000, 1, "Handwerk", "Pro 99 EUR/Mo", 96),
        ("cmp-demo-02", "conrad-ki", "SHK Haustechnik Rhein-Neckar GmbH", "shk-rhein-neckar-demo.de", "Kurpfalzring 18", "Mannheim", "68165", "Baden-Württemberg", 38, 24_000_000_000, 1, "Handwerk", "Pro 99 EUR/Mo", 94),
        ("cmp-demo-03", "conrad-ki", "Kanzlei Dr. Bergmann & Partner", "kanzlei-bergmann-demo.de", "Bergheimer Str. 74", "Heidelberg", "69115", "Baden-Württemberg", 19, 31_200_000_000, 1, "Kanzlei", "Pro 99 EUR/Mo", 93),
        ("cmp-demo-04", "conrad-ki", "Metallbau Hartmann KG", "metallbau-hartmann-demo.de", "Daimlerstr. 9", "Weinheim", "69469", "Baden-Württemberg", 45, 18_600_000_000, 1, "Produktion", "Wachstum 69 EUR/Mo", 90),
        ("cmp-demo-05", "conrad-ki", "Zahnarztpraxis am Marktplatz", "praxis-marktplatz-demo.de", "Marktplatz 6", "Wiesloch", "69168", "Baden-Württemberg", 14, 12_400_000_000, 1, "Praxis", "Wachstum 69 EUR/Mo", 91),
        ("cmp-demo-06", "conrad-ki", "Schreinerei Becker & Söhne GmbH", "schreinerei-becker-demo.de", "Industriestr. 21", "Hockenheim", "68766", "Baden-Württemberg", 16, 9_800_000_000, 1, "Handwerk", "Wachstum 69 EUR/Mo", 88),
        ("cmp-demo-07", "conrad-ki", "Steuerberatung König & Kollegen", "steuer-koenig-demo.de", "Bismarckstr. 51", "Ludwigshafen", "67059", "Rheinland-Pfalz", 22, 26_500_000_000, 1, "Kanzlei", "Pro 99 EUR/Mo", 95),
        ("cmp-demo-08", "conrad-ki", "Autohaus Kurpfalz GmbH", "autohaus-kurpfalz-demo.de", "Iggelheimer Str. 33", "Speyer", "67346", "Rheinland-Pfalz", 52, 22_000_000_000, 0, "Dienstleistung", "Wachstum 69 EUR/Mo", 76),
        ("cmp-demo-09", "conrad-ki", "Bäckerei & Konditorei Lindner GmbH", "baeckerei-lindner-demo.de", "Hauptstr. 14", "Ladenburg", "68526", "Baden-Württemberg", 31, 8_400_000_000, 0, "Produktion", "Basis 39 EUR/Mo", 72),
        ("cmp-demo-10", "conrad-ki", "Spedition Vogel & Partner GmbH", "spedition-vogel-demo.de", "Rheinau-Ring 102", "Mannheim", "68219", "Baden-Württemberg", 64, 34_500_000_000, 1, "Logistik", "Pro 99 EUR/Mo", 92),
        ("cmp-demo-11", "conrad-ki", "Optik & Akustik am Rathaus GmbH", "optik-rathaus-demo.de", "Rathausstr. 8", "Heidelberg", "69117", "Baden-Württemberg", 11, 7_900_000_000, 1, "Dienstleistung", "Basis 39 EUR/Mo", 84),
        ("cmp-demo-12", "conrad-ki", "Physiotherapie Vitalis Rhein-Neckar", "vitalis-physio-demo.de", "Carl-Theodor-Str. 29", "Schwetzingen", "68723", "Baden-Württemberg", 9, 6_200_000_000, 0, "Praxis", "Basis 39 EUR/Mo", 74),
        ("cmp-demo-13", "conrad-ki", "Dachdeckerei & Zimmerei Krüger GmbH", "dachbau-krueger-demo.de", "Am Industriehof 5", "Viernheim", "68519", "Hessen", 27, 11_900_000_000, 1, "Handwerk", "Wachstum 69 EUR/Mo", 89),
        ("cmp-demo-14", "conrad-ki", "IT-Systemhaus Neckarblick GmbH", "neckarblick-it-demo.de", "Wieblinger Weg 17", "Heidelberg", "69123", "Baden-Württemberg", 29, 28_400_000_000, 1, "Dienstleistung", "Pro 99 EUR/Mo", 91),
        ("cmp-demo-15", "conrad-ki", "Malerbetrieb Steinbach Meisterbetrieb", "maler-steinbach-demo.de", "Kirchenstr. 12", "Brühl", "68782", "Baden-Württemberg", 13, 5_400_000_000, 0, "Handwerk", "Basis 39 EUR/Mo", 71),
        ("cmp-demo-16", "conrad-ki", "Ingenieurbüro Klose & Partner mbB", "ing-klose-demo.de", "Kriegsstr. 88", "Karlsruhe", "76133", "Baden-Württemberg", 34, 29_700_000_000, 1, "Dienstleistung", "Pro 99 EUR/Mo", 90),
        ("cmp-demo-17", "conrad-ki", "Garten- & Landschaftsbau Sommer GmbH", "galabau-sommer-demo.de", "Neulandstr. 44", "Sinsheim", "74889", "Baden-Württemberg", 21, 8_900_000_000, 0, "Handwerk", "Wachstum 69 EUR/Mo", 77),
        ("cmp-demo-18", "conrad-ki", "Gebäudetechnik Richter & Co. KG", "gt-richter-demo.de", "Wormser Str. 61", "Frankenthal", "67227", "Rheinland-Pfalz", 41, 19_800_000_000, 1, "Handwerk", "Pro 99 EUR/Mo", 89),
        ("cmp-demo-19", "handwerk", "Sanitär & Heizung Meisterbetrieb Wolf GmbH", "shk-wolf-demo.de", "Neckarvorlandstr. 19", "Mannheim", "68169", "Baden-Württemberg", 18, 10_500_000_000, 1, "Handwerk", "Wachstum 69 EUR/Mo", 93),
        ("cmp-demo-20", "handwerk", "Elektrotechnik Braun & Sohn KG", "elektro-braun-demo.de", "Hauptstr. 83", "Eppelheim", "69214", "Baden-Württemberg", 25, 13_800_000_000, 1, "Handwerk", "Pro 99 EUR/Mo", 91),
        ("cmp-demo-21", "handwerk", "Schlosserei & Stahlbau Neumann GmbH", "stahlbau-neumann-demo.de", "Bahnhofstr. 31", "Neckargemünd", "69151", "Baden-Württemberg", 33, 17_200_000_000, 1, "Produktion", "Pro 99 EUR/Mo", 88),
        ("cmp-demo-22", "handwerk", "Kälte- & Klimatechnik Albrecht GmbH", "klima-albrecht-demo.de", "Altrottstr. 14", "Walldorf", "69190", "Baden-Württemberg", 22, 15_600_000_000, 1, "Handwerk", "Wachstum 69 EUR/Mo", 90),
        ("cmp-demo-23", "handwerk", "Fliesen- & Natursteinwerk Lehner", "fliesen-lehner-demo.de", "St.-Ilgener-Str. 27", "Leimen", "69181", "Baden-Württemberg", 12, 6_800_000_000, 0, "Handwerk", "Basis 39 EUR/Mo", 75),
        ("cmp-demo-24", "handwerk", "Bauunternehmen Kraft & Söhne GmbH", "bau-kraft-demo.de", "Kammerforststr. 5", "Bruchsal", "76646", "Baden-Württemberg", 58, 32_400_000_000, 1, "Produktion", "Pro 99 EUR/Mo", 94),
        ("cmp-demo-25", "handwerk", "Tischlerei & Innenausbau Brandt", "tischlerei-brandt-demo.de", "Maximilianstr. 40", "Neustadt a.d.W.", "67433", "Rheinland-Pfalz", 15, 7_600_000_000, 0, "Handwerk", "Basis 39 EUR/Mo", 73),
        ("cmp-demo-26", "handwerk", "Solar & Wärmepumpen Rhein-Pfalz GmbH", "solar-rheinpfalz-demo.de", "Speyerer Str. 9", "Mutterstadt", "67112", "Rheinland-Pfalz", 28, 21_300_000_000, 1, "Handwerk", "Pro 99 EUR/Mo", 95),
        ("cmp-demo-27", "handwerk", "Kfz-Meisterbetrieb Frank & Kollegen", "kfz-frank-demo.de", "Hockenheimer Str. 71", "Ketsch", "68775", "Baden-Württemberg", 10, 5_900_000_000, 0, "Dienstleistung", "Basis 39 EUR/Mo", 70),
        ("cmp-demo-28", "handwerk", "Gerüstbau & Fassaden Lorenz GmbH", "geruestbau-lorenz-demo.de", "Chemiestr. 11", "Lampertheim", "68623", "Hessen", 36, 16_400_000_000, 1, "Handwerk", "Wachstum 69 EUR/Mo", 87),
        ("cmp-demo-29", "handwerk", "Rohr- & Kanalservice Zimmer GmbH", "kanal-zimmer-demo.de", "Hardtwaldring 22", "Oftersheim", "68723", "Baden-Württemberg", 19, 11_200_000_000, 1, "Handwerk", "Wachstum 69 EUR/Mo", 86),
        ("cmp-demo-30", "handwerk", "Fensterbau & Rollladentechnik Seidel", "fenster-seidel-demo.de", "Bergstr. 53", "Dossenheim", "69221", "Baden-Württemberg", 17, 9_100_000_000, 0, "Handwerk", "Basis 39 EUR/Mo", 78),
    ];

    for (idx, (id, ws, name, dom, street, city, zip, state, emp, arr, icp, branche, paket, score)) in companies.iter().enumerate() {
        let cf = json!({ "branche": branche, "paket": paket, "kiScore": score }).to_string();
        tx.execute(
            "INSERT INTO twenty_companies
             (id, workspace, name, domain_label, domain_url, street1, city, postcode, state, country,
              employees, linkedin_url, x_url, arr_micros, arr_currency, icp, position, created_by_name, custom_fields_json, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 'Deutschland', ?10, '', '', ?11, 'EUR', ?12, ?13, 'Demo-Team', ?14, ?15, ?15)",
            params![id, ws, name, dom, dom, street, city, zip, state, emp, arr, icp, (idx + 1) as f64, cf, now],
        )?;
    }

    // 25 fiktive Ansprechpartner (15 in conrad-ki, 10 in handwerk)
    let people: [(&str, &str, &str, &str, &str, &str, &str, &str, &str); 25] = [
        ("per-demo-01", "conrad-ki", "Thomas", "Weber", "t.weber@elektro-weber-demo.de", "+49 6202 5550101", "Geschäftsführer", "Schwetzingen", "cmp-demo-01"),
        ("per-demo-02", "conrad-ki", "Julia", "Weber", "j.weber@elektro-weber-demo.de", "+49 6202 5550102", "Kaufmännische Leitung", "Schwetzingen", "cmp-demo-01"),
        ("per-demo-03", "conrad-ki", "Stefan", "Koch", "s.koch@shk-rhein-neckar-demo.de", "+49 621 5550103", "Technischer Leiter", "Mannheim", "cmp-demo-02"),
        ("per-demo-04", "conrad-ki", "Sabine", "Bergmann", "s.bergmann@kanzlei-bergmann-demo.de", "+49 6221 5550104", "Partnerin & Steuerberaterin", "Heidelberg", "cmp-demo-03"),
        ("per-demo-05", "conrad-ki", "Martin", "Hartmann", "m.hartmann@metallbau-hartmann-demo.de", "+49 6201 5550105", "Geschäftsführer", "Weinheim", "cmp-demo-04"),
        ("per-demo-06", "conrad-ki", "Katrin", "Lindemann", "praxis@praxis-marktplatz-demo.de", "+49 6222 5550106", "Praxismanagerin", "Wiesloch", "cmp-demo-05"),
        ("per-demo-07", "conrad-ki", "Christian", "Becker", "c.becker@schreinerei-becker-demo.de", "+49 6205 5550107", "Meister & Inhaber", "Hockenheim", "cmp-demo-06"),
        ("per-demo-08", "conrad-ki", "Andreas", "König", "a.koenig@steuer-koenig-demo.de", "+49 621 5550108", "Kanzleipartner", "Ludwigshafen", "cmp-demo-07"),
        ("per-demo-09", "conrad-ki", "Petra", "Schmitt", "p.schmitt@autohaus-kurpfalz-demo.de", "+49 6232 5550109", "Serviceleitung", "Speyer", "cmp-demo-08"),
        ("per-demo-10", "conrad-ki", "Frank", "Lindner", "f.lindner@baeckerei-lindner-demo.de", "+49 6203 5550110", "Inhaber", "Ladenburg", "cmp-demo-09"),
        ("per-demo-11", "conrad-ki", "Matthias", "Vogel", "m.vogel@spedition-vogel-demo.de", "+49 621 5550111", "Dispositionsleitung", "Mannheim", "cmp-demo-10"),
        ("per-demo-12", "conrad-ki", "Nina", "Bauer", "n.bauer@optik-rathaus-demo.de", "+49 6221 5550112", "Filialleiterin", "Heidelberg", "cmp-demo-11"),
        ("per-demo-13", "conrad-ki", "Daniel", "Krüger", "d.krueger@dachbau-krueger-demo.de", "+49 6204 5550113", "Dachdeckermeister", "Viernheim", "cmp-demo-13"),
        ("per-demo-14", "conrad-ki", "Sven", "Albrecht", "s.albrecht@neckarblick-it-demo.de", "+49 6221 5550114", "Leiter Managed Services", "Heidelberg", "cmp-demo-14"),
        ("per-demo-15", "conrad-ki", "Tobias", "Klose", "t.klose@ing-klose-demo.de", "+49 721 5550115", "Projektleiter TGA", "Karlsruhe", "cmp-demo-16"),
        ("per-demo-16", "handwerk", "Michael", "Wolf", "m.wolf@shk-wolf-demo.de", "+49 621 5550201", "SHK-Meister", "Mannheim", "cmp-demo-19"),
        ("per-demo-17", "handwerk", "Jan", "Braun", "j.braun@elektro-braun-demo.de", "+49 6221 5550202", "Elektromeister", "Eppelheim", "cmp-demo-20"),
        ("per-demo-18", "handwerk", "Holger", "Neumann", "h.neumann@stahlbau-neumann-demo.de", "+49 6223 5550203", "Geschäftsführer", "Neckargemünd", "cmp-demo-21"),
        ("per-demo-19", "handwerk", "Vanessa", "Albrecht", "v.albrecht@klima-albrecht-demo.de", "+49 6227 5550204", "Büroleitung", "Walldorf", "cmp-demo-22"),
        ("per-demo-20", "handwerk", "Ralf", "Lehner", "r.lehner@fliesen-lehner-demo.de", "+49 6224 5550205", "Inhaber", "Leimen", "cmp-demo-23"),
        ("per-demo-21", "handwerk", "Markus", "Kraft", "m.kraft@bau-kraft-demo.de", "+49 7251 5550206", "Bauleiter", "Bruchsal", "cmp-demo-24"),
        ("per-demo-22", "handwerk", "Lukas", "Brandt", "l.brandt@tischlerei-brandt-demo.de", "+49 6321 5550207", "Tischlermeister", "Neustadt a.d.W.", "cmp-demo-25"),
        ("per-demo-23", "handwerk", "Sarah", "Winter", "s.winter@solar-rheinpfalz-demo.de", "+49 6234 5550208", "Projektkoordinatorin", "Mutterstadt", "cmp-demo-26"),
        ("per-demo-24", "handwerk", "Bernd", "Lorenz", "b.lorenz@geruestbau-lorenz-demo.de", "+49 6206 5550209", "Geschäftsführer", "Lampertheim", "cmp-demo-28"),
        ("per-demo-25", "handwerk", "Patrick", "Zimmer", "p.zimmer@kanal-zimmer-demo.de", "+49 6202 5550210", "Einsatzleiter", "Oftersheim", "cmp-demo-29"),
    ];

    for (idx, (id, ws, fnm, lnm, email, phone, job, city, cid)) in people.iter().enumerate() {
        tx.execute(
            "INSERT INTO twenty_people
             (id, workspace, first_name, last_name, email, phone, phone_country_code, phone_calling_code,
              job_title, city, linkedin_url, company_id, position, created_by_name, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'DE', '+49', ?7, ?8, '', ?9, ?10, 'Demo-Team', ?11, ?11)",
            params![id, ws, fnm, lnm, email, phone, job, city, cid, (idx + 1) as f64, now],
        )?;
    }

    // 18 Verkaufschancen ueber alle 5 Phasen (NEW, SCREENING, MEETING, PROPOSAL, CUSTOMER)
    let opps: [(&str, &str, &str, i64, &str, &str, &str, &str, &str); 18] = [
        ("opp-demo-01", "conrad-ki", "Elektro Weber — KI-Angebots- & E-Mail-Triage", 4_800_000_000, "2026-11-30", "CUSTOMER", "cmp-demo-01", "per-demo-01", "mem-demo-1"),
        ("opp-demo-02", "conrad-ki", "SHK Rhein-Neckar — 24/7 Notdienst-Qualifizierung", 5_400_000_000, "2026-12-05", "PROPOSAL", "cmp-demo-02", "per-demo-03", "mem-demo-1"),
        ("opp-demo-03", "conrad-ki", "Kanzlei Bergmann — Mandanten-Onboarding & DMS", 6_900_000_000, "2026-12-10", "MEETING", "cmp-demo-03", "per-demo-04", "mem-demo-2"),
        ("opp-demo-04", "conrad-ki", "Metallbau Hartmann — Sprachmemo zu Angebot", 4_200_000_000, "2026-12-15", "SCREENING", "cmp-demo-04", "per-demo-05", "mem-demo-1"),
        ("opp-demo-05", "conrad-ki", "Zahnarztpraxis Marktplatz — Termin- & Recall-Bot", 3_600_000_000, "2026-11-28", "CUSTOMER", "cmp-demo-05", "per-demo-06", "mem-demo-2"),
        ("opp-demo-06", "conrad-ki", "Schreinerei Becker — Aufmaß-Fotodoku & CRM", 2_900_000_000, "2026-12-18", "NEW", "cmp-demo-06", "per-demo-07", "mem-demo-1"),
        ("opp-demo-07", "conrad-ki", "Steuerberatung König — GoBD Belegprüfung + DATEV", 7_500_000_000, "2026-12-02", "PROPOSAL", "cmp-demo-07", "per-demo-08", "mem-demo-2"),
        ("opp-demo-08", "conrad-ki", "Spedition Vogel — Frachtbrief-OCR (Lokal-KI)", 8_400_000_000, "2026-12-20", "MEETING", "cmp-demo-10", "per-demo-11", "mem-demo-1"),
        ("opp-demo-09", "conrad-ki", "Optik Rathaus — Online-Sehtest Vorqualifizierung", 2_400_000_000, "2026-12-22", "SCREENING", "cmp-demo-11", "per-demo-12", "mem-demo-2"),
        ("opp-demo-10", "conrad-ki", "Dachbau Krüger — Baustellen-Tagesbericht Automation", 3_900_000_000, "2026-12-28", "NEW", "cmp-demo-13", "per-demo-13", "mem-demo-1"),
        ("opp-demo-11", "conrad-ki", "Ingenieurbüro Klose — Ausschreibungs-Analyse", 6_100_000_000, "2026-12-12", "PROPOSAL", "cmp-demo-16", "per-demo-15", "mem-demo-2"),
        ("opp-demo-12", "handwerk", "SHK Wolf — Wartungsvertrags-Automatisierung", 3_800_000_000, "2026-11-29", "CUSTOMER", "cmp-demo-19", "per-demo-16", "mem-demo-3"),
        ("opp-demo-13", "handwerk", "Elektro Braun — Wallbox-Anfragen & Termin-Sync", 4_500_000_000, "2026-12-06", "PROPOSAL", "cmp-demo-20", "per-demo-17", "mem-demo-3"),
        ("opp-demo-14", "handwerk", "Stahlbau Neumann — CAD-Stücklisten Eingangs-Check", 5_100_000_000, "2026-12-14", "MEETING", "cmp-demo-21", "per-demo-18", "mem-demo-4"),
        ("opp-demo-15", "handwerk", "Klima Albrecht — Saison-Wartungsreminder (Draft)", 3_200_000_000, "2026-12-19", "SCREENING", "cmp-demo-22", "per-demo-19", "mem-demo-4"),
        ("opp-demo-16", "handwerk", "Bau Kraft — Subunternehmer-Rechnungsprüfung", 7_200_000_000, "2026-12-09", "PROPOSAL", "cmp-demo-24", "per-demo-21", "mem-demo-3"),
        ("opp-demo-17", "handwerk", "Solar Rhein-Pfalz — PV-Lead-Scoring + Entwürfe", 6_400_000_000, "2026-12-11", "CUSTOMER", "cmp-demo-26", "per-demo-23", "mem-demo-4"),
        ("opp-demo-18", "handwerk", "Gerüstbau Lorenz — Angebots-Nachfassung (7 Tage)", 2_700_000_000, "2026-12-27", "NEW", "cmp-demo-28", "per-demo-24", "mem-demo-3"),
    ];

    for (idx, (id, ws, name, amt, close_d, stage, cid, pid, oid)) in opps.iter().enumerate() {
        tx.execute(
            "INSERT INTO twenty_opportunities
             (id, workspace, name, amount_micros, currency_code, close_date, stage, company_id,
              point_of_contact_id, owner_id, position, created_by_name, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, 'EUR', ?5, ?6, ?7, ?8, ?9, ?10, 'Demo-Team', ?11, ?11)",
            params![id, ws, name, amt, close_d, stage, cid, pid, oid, (idx + 1) as f64, now],
        )?;
    }

    // 25 Notizen (15 in conrad-ki, 10 in handwerk)
    let notes: [(&str, &str, &str, &str, &str, Option<&str>, Option<&str>); 25] = [
        ("note-demo-01", "conrad-ki", "Erstgespräch Elektro Weber: Postfach-Triage", "Ziel: 100 % Entwurfspflicht (draftOnly=true) bei Kundenanfragen und Wallbox-Offerten.", "cmp-demo-01", Some("per-demo-01"), Some("opp-demo-01")),
        ("note-demo-02", "conrad-ki", "Go-Live Checkliste Elektro Weber", "Angebotsvorlagen + DATEV-Export erfolgreich im Testsystem geprüft.", "cmp-demo-01", Some("per-demo-02"), Some("opp-demo-01")),
        ("note-demo-03", "conrad-ki", "Anforderung SHK Rhein-Neckar Notdienst", "Pro Woche ca. 15 verpasste Anrufe während Baustelleneinsatz — Vorqualifizierung per Formular gewünscht.", "cmp-demo-02", Some("per-demo-03"), Some("opp-demo-02")),
        ("note-demo-04", "conrad-ki", "AVV & Datenschutz Kanzlei Bergmann", "Betrieb komplett lokal auf deutschem VPS ohne US-Cloud-Transfer bestätigt.", "cmp-demo-03", Some("per-demo-04"), Some("opp-demo-03")),
        ("note-demo-05", "conrad-ki", "Prozessanalyse Metallbau Hartmann", "Meister diktiert Aufmaß-Notizen im Transporter; Umwandlung in strukturiertes Angebot gewünscht.", "cmp-demo-04", Some("per-demo-05"), Some("opp-demo-04")),
        ("note-demo-06", "conrad-ki", "Praxis-Workflow Zahnarztpraxis Marktplatz", "Erinnerungs-Entwürfe für Prophylaxe 48h vor Termin getestet.", "cmp-demo-05", Some("per-demo-06"), Some("opp-demo-05")),
        ("note-demo-07", "conrad-ki", "Schreinerei Becker Rückfrage Holzpreise", "Preistabelle in Datentabelle hinterlegt für automatische Kalkulation.", "cmp-demo-06", Some("per-demo-07"), Some("opp-demo-06")),
        ("note-demo-08", "conrad-ki", "Steuerberatung König: DATEV-Prüfung", "Eingangsrechnungen automatisch auf USt-ID, Steuersatz und Belegdatum prüfen.", "cmp-demo-07", Some("per-demo-08"), Some("opp-demo-07")),
        ("note-demo-09", "conrad-ki", "Autohaus Kurpfalz Werkstatt-Slot", "Interesse an saisonaler Reifenwechsel-Terminbuchung im Frühjahr.", "cmp-demo-08", Some("per-demo-09"), None),
        ("note-demo-10", "conrad-ki", "Bäckerei Lindner Filialbestellung", "Tägliche Vorbestellliste per Webhook sammeln und als PDF zusammenfassen.", "cmp-demo-09", Some("per-demo-10"), None),
        ("note-demo-11", "conrad-ki", "Spedition Vogel Frachtbrief-Benchmark", "10 Test-Frachtbriefe anonymisiert eingelesen — Extraktionszeit unter 2 Sekunden.", "cmp-demo-10", Some("per-demo-11"), Some("opp-demo-08")),
        ("note-demo-12", "conrad-ki", "Optik Rathaus Beratungsstrecken", "Termin-Qualifizierung für Hörakustik & Gleitsichtberatung im CRM abgebildet.", "cmp-demo-11", Some("per-demo-12"), Some("opp-demo-09")),
        ("note-demo-13", "conrad-ki", "Dachbau Krüger Fotoprotokoll", "Baustellenfotos nach Projektnummer sortieren und tägliches Briefing erzeugen.", "cmp-demo-13", Some("per-demo-13"), Some("opp-demo-10")),
        ("note-demo-14", "conrad-ki", "IT-Systemhaus Neckarblick Partner-Call", "Mögliche Co-Vermarktung für regionale KMU im Raum Heidelberg besprochen.", "cmp-demo-14", Some("per-demo-14"), None),
        ("note-demo-15", "conrad-ki", "Ingenieurbüro Klose TGA-Prüfmatrix", "Automatische Zusammenfassung von 80-seitigen Leistungsverzeichnissen.", "cmp-demo-16", Some("per-demo-15"), Some("opp-demo-11")),
        ("note-demo-16", "handwerk", "SHK Wolf Wartungs-Intervalle", "Jährliche Heizungswartung automatisch 30 Tage vorher als Entwurf vorbereiten.", "cmp-demo-19", Some("per-demo-16"), Some("opp-demo-12")),
        ("note-demo-17", "handwerk", "Elektro Braun PV-Speicher-Checkliste", "Lead-Formular fragt Zählerschrank-Alter und Dachneigung ab.", "cmp-demo-20", Some("per-demo-17"), Some("opp-demo-13")),
        ("note-demo-18", "handwerk", "Stahlbau Neumann Lieferanten-Portal", "Lieferscheine gegen Bestellungen im CRM automatisch abgleichen.", "cmp-demo-21", Some("per-demo-18"), Some("opp-demo-14")),
        ("note-demo-19", "handwerk", "Klima Albrecht Störungs-Triage", "Kälteanlagen-Fehlermeldungen nach Dringlichkeit klassifizieren.", "cmp-demo-22", Some("per-demo-19"), Some("opp-demo-15")),
        ("note-demo-20", "handwerk", "Fliesen Lehner Aufmaß-Notiz", "Standard-Positionen für Badsanierung als Vorlage hinterlegt.", "cmp-demo-23", Some("per-demo-20"), None),
        ("note-demo-21", "handwerk", "Bau Kraft Rechnungsfreigabe", "Zweistufige Prüfung ab 2.500 EUR Nettobetrag vor DATEV-Übergabe.", "cmp-demo-24", Some("per-demo-21"), Some("opp-demo-16")),
        ("note-demo-22", "handwerk", "Tischlerei Brandt Musterkatalog", "Kunden erhalten PDF-Katalog-Link nach Beratungsbuchung.", "cmp-demo-25", Some("per-demo-22"), None),
        ("note-demo-23", "handwerk", "Solar Rhein-Pfalz Lead-Bewertung", "Score > 85 direkt an Vertriebs-Mitarbeiter inkl. Angebotsentwurf zuweisen.", "cmp-demo-26", Some("per-demo-23"), Some("opp-demo-17")),
        ("note-demo-24", "handwerk", "Gerüstbau Lorenz Nachfass-Regel", "Automatische Erinnerung 7 Tage nach Angebotsversand ohne Rückmeldung.", "cmp-demo-28", Some("per-demo-24"), Some("opp-demo-18")),
        ("note-demo-25", "handwerk", "Kanalservice Zimmer Einsatzplanung", "Notfälle sofort im Tagesbriefing markieren, Standardanfragen bündeln.", "cmp-demo-29", Some("per-demo-25"), None),
    ];

    for (idx, (id, ws, title, body, cid, pid, oid)) in notes.iter().enumerate() {
        tx.execute(
            "INSERT INTO twenty_notes
             (id, workspace, title, body_markdown, company_id, person_id, opportunity_id, position, created_by_name, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'Demo-Team', ?9, ?9)",
            params![id, ws, title, body, cid, pid, oid, (idx + 1) as f64, now],
        )?;
    }

    // 15 Aufgaben (9 in conrad-ki, 6 in handwerk) ueber TODO, IN_PROGRESS, DONE
    let tasks: [(&str, &str, &str, &str, &str, &str, &str, Option<&str>, Option<&str>, &str); 15] = [
        ("task-demo-01", "conrad-ki", "Quartals-Review Elektro Weber vorbereiten", "KPIs und gesparte Stunden pro Woche zusammenfassen.", "TODO", "2026-11-20T10:00:00Z", "cmp-demo-01", Some("per-demo-01"), Some("opp-demo-01"), "mem-demo-1"),
        ("task-demo-02", "conrad-ki", "Notdienst-Triage Workflow an SHK Rhein-Neckar vorstellen", "Live-Demo mit fiktivem Wasserschaden-Ticket durchspielen.", "IN_PROGRESS", "2026-11-18T14:00:00Z", "cmp-demo-02", Some("per-demo-03"), Some("opp-demo-02"), "mem-demo-1"),
        ("task-demo-03", "conrad-ki", "AV-Vertragsentwurf für Kanzlei Bergmann prüfen", "Freigabe durch Datenschutzbeauftragten einholen.", "TODO", "2026-11-21T09:30:00Z", "cmp-demo-03", Some("per-demo-04"), Some("opp-demo-03"), "mem-demo-2"),
        ("task-demo-04", "conrad-ki", "Sprachmemo-Parser für Metallbau Hartmann testen", "Aufmaß-Beispiel mit 4 Positionen als Testlauf ausführen.", "IN_PROGRESS", "2026-11-19T11:00:00Z", "cmp-demo-04", Some("per-demo-05"), Some("opp-demo-04"), "mem-demo-1"),
        ("task-demo-05", "conrad-ki", "Recall-Vorlagen Zahnarztpraxis Marktplatz abnehmen", "Nur E-Mail-Entwurf (draftOnly=true) verifiziert.", "DONE", "2026-11-14T16:00:00Z", "cmp-demo-05", Some("per-demo-06"), Some("opp-demo-05"), "mem-demo-2"),
        ("task-demo-06", "conrad-ki", "DATEV-Feldmapping mit Steuerberatung König abstimmen", "Kontenrahmen SKR04 in Datentabelle hinterlegen.", "TODO", "2026-11-22T15:00:00Z", "cmp-demo-07", Some("per-demo-08"), Some("opp-demo-07"), "mem-demo-2"),
        ("task-demo-07", "conrad-ki", "Frachtbrief-Benchmark mit Spedition Vogel auswerten", "Erkennungsrate und Latenz dokumentieren.", "DONE", "2026-11-13T13:00:00Z", "cmp-demo-10", Some("per-demo-11"), Some("opp-demo-08"), "mem-demo-1"),
        ("task-demo-08", "conrad-ki", "Erstgespräch-Agenda für Schreinerei Becker senden", "Kalender-Link und 3 Beispiel-Workflows beifügen (als Entwurf).", "TODO", "2026-11-24T08:30:00Z", "cmp-demo-06", Some("per-demo-07"), Some("opp-demo-06"), "mem-demo-1"),
        ("task-demo-09", "conrad-ki", "LV-Zusammenfassung für Ingenieurbüro Klose finalisieren", "Risikoklauseln im Output hervorheben.", "IN_PROGRESS", "2026-11-23T17:00:00Z", "cmp-demo-16", Some("per-demo-15"), Some("opp-demo-11"), "mem-demo-2"),
        ("task-demo-10", "handwerk", "Wartungs-Checkliste für SHK Wolf aktualisieren", "Heizungs- und Wärmepumpen-Prüfpunkte ergänzen.", "DONE", "2026-11-12T10:00:00Z", "cmp-demo-19", Some("per-demo-16"), Some("opp-demo-12"), "mem-demo-3"),
        ("task-demo-11", "handwerk", "Wallbox-Formular bei Elektro Braun freischalten", "Webhook-Pfad auf /webhook/lead-intake testen.", "IN_PROGRESS", "2026-11-19T14:30:00Z", "cmp-demo-20", Some("per-demo-17"), Some("opp-demo-13"), "mem-demo-3"),
        ("task-demo-12", "handwerk", "Lieferschein-Abgleich für Stahlbau Neumann demonstrieren", "Muster-PDF durch Eingangsrechnungs-Flow schicken.", "TODO", "2026-11-25T10:00:00Z", "cmp-demo-21", Some("per-demo-18"), Some("opp-demo-14"), "mem-demo-4"),
        ("task-demo-13", "handwerk", "Freigabe-Regel >2.500 EUR bei Bau Kraft einrichten", "GoBD-Prüfvermerk aktivieren.", "IN_PROGRESS", "2026-11-20T16:00:00Z", "cmp-demo-24", Some("per-demo-21"), Some("opp-demo-16"), "mem-demo-3"),
        ("task-demo-14", "handwerk", "Lead-Scoring Schwellenwerte mit Solar Rhein-Pfalz abgleichen", "Ab Score 85 hohe Priorität im CRM setzen.", "DONE", "2026-11-14T11:30:00Z", "cmp-demo-26", Some("per-demo-23"), Some("opp-demo-17"), "mem-demo-4"),
        ("task-demo-15", "handwerk", "Nachfass-Entwurf für Gerüstbau Lorenz testen", "7-Tage-Frist im Demo-Workflow verifizieren.", "TODO", "2026-11-26T09:00:00Z", "cmp-demo-28", Some("per-demo-24"), Some("opp-demo-18"), "mem-demo-3"),
    ];

    for (idx, (id, ws, title, body, status, due_at, cid, pid, oid, aid)) in tasks.iter().enumerate() {
        tx.execute(
            "INSERT INTO twenty_tasks
             (id, workspace, title, body_markdown, status, due_at, company_id, person_id, opportunity_id,
              assignee_id, position, created_by_name, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, 'Demo-Team', ?12, ?12)",
            params![id, ws, title, body, status, due_at, cid, pid, oid, aid, (idx + 1) as f64, now],
        )?;
    }

    // Gespeicherte Ansichten & Favoriten
    for (vid, ws, obj, name, layout, fjson, sjson, pos) in [
        ("view-demo-cmp-all", "conrad-ki", "company", "Alle KMU-Firmen", "table", "{}", r#"{"field":"name","dir":"asc"}"#, 0),
        ("view-demo-cmp-icp", "conrad-ki", "company", "Top-Zielkunden (ICP)", "table", r#"{"icp":true}"#, r#"{"field":"arr","dir":"desc"}"#, 1),
        ("view-demo-opp-kanban", "conrad-ki", "opportunity", "Vertriebs-Pipeline (Kanban)", "kanban", "{}", r#"{"field":"amount","dir":"desc"}"#, 0),
        ("view-demo-hw-cmp", "handwerk", "company", "Handwerksbetriebe", "table", "{}", r#"{"field":"name","dir":"asc"}"#, 0),
        ("view-demo-hw-kanban", "handwerk", "opportunity", "Handwerk Pipeline", "kanban", "{}", r#"{"field":"amount","dir":"desc"}"#, 0),
        ("view-demo-tasks-open", "conrad-ki", "task", "Offene Aufgaben", "table", r#"{"status":"TODO"}"#, r#"{"field":"dueAt","dir":"asc"}"#, 0),
    ] {
        tx.execute(
            "INSERT INTO twenty_views
             (id, workspace, object_name, name, layout, filter_json, sort_json, columns_json, position, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, '[]', ?8, ?9, ?9)",
            params![vid, ws, obj, name, layout, fjson, sjson, pos, now],
        )?;
    }

    for (fid, ws, label, ttype, tid, pos) in [
        ("fav-demo-1", "conrad-ki", "Elektro Weber GmbH", "company", "cmp-demo-01", 0),
        ("fav-demo-2", "conrad-ki", "SHK Rhein-Neckar", "company", "cmp-demo-02", 1),
        ("fav-demo-3", "conrad-ki", "Kanzlei Dr. Bergmann", "company", "cmp-demo-03", 2),
        ("fav-demo-4", "handwerk", "SHK Meisterbetrieb Wolf", "company", "cmp-demo-19", 0),
    ] {
        tx.execute(
            "INSERT INTO twenty_favorites (id, workspace, label, target_type, target_id, position, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![fid, ws, label, ttype, tid, pos, now],
        )?;
    }

    for (tid, ws, ev_name, cid, oid, summary) in [
        ("tl-demo-01", "conrad-ki", "company.created", Some("cmp-demo-01"), None, "Demo-Firma angelegt: Meisterbetrieb Elektro Weber GmbH"),
        ("tl-demo-02", "conrad-ki", "opportunity.updated", Some("cmp-demo-01"), Some("opp-demo-01"), "Phase auf CUSTOMER aktualisiert"),
        ("tl-demo-03", "conrad-ki", "note.created", Some("cmp-demo-02"), Some("opp-demo-02"), "Notiz: Anforderung SHK Rhein-Neckar Notdienst"),
        ("tl-demo-04", "handwerk", "company.created", Some("cmp-demo-19"), None, "Demo-Firma angelegt: Sanitär & Heizung Meisterbetrieb Wolf GmbH"),
        ("tl-demo-05", "handwerk", "opportunity.updated", Some("cmp-demo-26"), Some("opp-demo-17"), "Phase auf CUSTOMER aktualisiert"),
    ] {
        tx.execute(
            "INSERT INTO twenty_timeline (id, workspace, happens_at, name, company_id, person_id, opportunity_id, note_id, task_id, actor, details_json)
             VALUES (?1, ?2, ?3, ?4, ?5, NULL, ?6, NULL, NULL, 'Demo-Team', ?7)",
            params![tid, ws, now, ev_name, cid, oid, json!({"summary": summary}).to_string()],
        )?;
    }

    tx.commit()?;
    Ok(json!({
        "ok": true,
        "workspaces": ["conrad-ki", "handwerk"],
        "companies": 30,
        "people": 25,
        "opportunities": 18,
        "notes": 25,
        "tasks": 15,
        "members": 4,
        "views": 6,
        "favorites": 4
    }))
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
            "handwerk": {
                "companies": count_for("twenty_companies", "handwerk"),
                "people": count_for("twenty_people", "handwerk"),
                "opportunities": count_for("twenty_opportunities", "handwerk"),
                "notes": count_for("twenty_notes", "handwerk"),
                "tasks": count_for("twenty_tasks", "handwerk")
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
    let custom_fields_for = |obj_singular: &str| -> Vec<Value> {
        let mut out = Vec::new();
        if let Ok(mut st) = conn.prepare(
            "SELECT id, name, label, field_type, options_json, default_value FROM twenty_field_metadata WHERE object_name = ?1 ORDER BY created_at ASC",
        ) {
            if let Ok(mut rows) = st.query([obj_singular]) {
                while let Ok(Some(r)) = rows.next() {
                    let opts: Value = serde_json::from_str(&r.get::<_, String>(4).unwrap_or_else(|_| "[]".into())).unwrap_or_else(|_| json!([]));
                    out.push(json!({
                        "id": r.get::<_, String>(0).unwrap_or_default(),
                        "name": r.get::<_, String>(1).unwrap_or_default(),
                        "label": r.get::<_, String>(2).unwrap_or_default(),
                        "type": r.get::<_, String>(3).unwrap_or_else(|_| "TEXT".into()),
                        "options": opts,
                        "defaultValue": r.get::<_, String>(5).unwrap_or_default(),
                        "isCustom": true
                    }));
                }
            }
        }
        out
    };
    let merge_fields = |base: Vec<Value>, obj_singular: &str| -> Vec<Value> {
        let mut all = base;
        all.extend(custom_fields_for(obj_singular));
        all
    };

    Json(json!({
        "workspace": ws,
        "objects": [
            {
                "nameSingular": "company",
                "namePlural": "companies",
                "labelPlural": "Unternehmen",
                "icon": "IconBuildingSkyscraper",
                "recordCount": count_tbl("twenty_companies"),
                "fields": merge_fields(vec![
                    json!({"name": "name", "label": "Name", "type": "TEXT", "isCustom": false}),
                    json!({"name": "domainName", "label": "Domain", "type": "LINKS", "isCustom": false}),
                    json!({"name": "address", "label": "Adresse", "type": "ADDRESS", "isCustom": false}),
                    json!({"name": "employees", "label": "Mitarbeiter", "type": "NUMBER", "isCustom": false}),
                    json!({"name": "annualRecurringRevenue", "label": "ARR", "type": "CURRENCY", "isCustom": false}),
                    json!({"name": "idealCustomerProfile", "label": "ICP", "type": "BOOLEAN", "isCustom": false}),
                    json!({"name": "linkedinLink", "label": "LinkedIn", "type": "LINKS", "isCustom": false})
                ], "company")
            },
            {
                "nameSingular": "person",
                "namePlural": "people",
                "labelPlural": "Kontakte",
                "icon": "IconUser",
                "recordCount": count_tbl("twenty_people"),
                "fields": merge_fields(vec![
                    json!({"name": "name", "label": "Name", "type": "FULL_NAME", "isCustom": false}),
                    json!({"name": "emails", "label": "E-Mail", "type": "EMAILS", "isCustom": false}),
                    json!({"name": "phones", "label": "Telefon", "type": "PHONES", "isCustom": false}),
                    json!({"name": "jobTitle", "label": "Position", "type": "TEXT", "isCustom": false}),
                    json!({"name": "city", "label": "Stadt", "type": "TEXT", "isCustom": false}),
                    json!({"name": "company", "label": "Unternehmen", "type": "RELATION", "isCustom": false})
                ], "person")
            },
            {
                "nameSingular": "opportunity",
                "namePlural": "opportunities",
                "labelPlural": "Verkaufschancen",
                "icon": "IconTargetArrow",
                "recordCount": count_tbl("twenty_opportunities"),
                "stages": ["NEW", "SCREENING", "MEETING", "PROPOSAL", "CUSTOMER"],
                "fields": merge_fields(vec![
                    json!({"name": "name", "label": "Name", "type": "TEXT", "isCustom": false}),
                    json!({"name": "amount", "label": "Betrag", "type": "CURRENCY", "isCustom": false}),
                    json!({"name": "stage", "label": "Phase", "type": "SELECT", "options": ["NEW","SCREENING","MEETING","PROPOSAL","CUSTOMER"], "isCustom": false}),
                    json!({"name": "closeDate", "label": "Abschlussdatum", "type": "DATE_TIME", "isCustom": false}),
                    json!({"name": "company", "label": "Unternehmen", "type": "RELATION", "isCustom": false}),
                    json!({"name": "pointOfContact", "label": "Ansprechpartner", "type": "RELATION", "isCustom": false})
                ], "opportunity")
            },
            {
                "nameSingular": "note",
                "namePlural": "notes",
                "labelPlural": "Notizen",
                "icon": "IconNotes",
                "recordCount": count_tbl("twenty_notes"),
                "fields": merge_fields(vec![
                    json!({"name": "title", "label": "Titel", "type": "TEXT", "isCustom": false}),
                    json!({"name": "bodyV2", "label": "Inhalt", "type": "RICH_TEXT_V2", "isCustom": false}),
                    json!({"name": "noteTargets", "label": "Verknuepfungen", "type": "RELATION", "isCustom": false})
                ], "note")
            },
            {
                "nameSingular": "task",
                "namePlural": "tasks",
                "labelPlural": "Aufgaben",
                "icon": "IconCheckbox",
                "recordCount": count_tbl("twenty_tasks"),
                "statuses": ["TODO", "IN_PROGRESS", "DONE"],
                "fields": merge_fields(vec![
                    json!({"name": "title", "label": "Titel", "type": "TEXT", "isCustom": false}),
                    json!({"name": "status", "label": "Status", "type": "SELECT", "options": ["TODO","IN_PROGRESS","DONE"], "isCustom": false}),
                    json!({"name": "dueAt", "label": "Faelligkeitsdatum", "type": "DATE_TIME", "isCustom": false}),
                    json!({"name": "bodyV2", "label": "Beschreibung", "type": "RICH_TEXT_V2", "isCustom": false}),
                    json!({"name": "taskTargets", "label": "Verknuepfungen", "type": "RELATION", "isCustom": false})
                ], "task")
            }
        ]
    }))
}

pub async fn sync_sql_handler(State(state): State<AppState>) -> impl IntoResponse {
    match seed_twenty_demo_data(&state.db) {
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
    Ok(attach_custom_fields(json!({
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
    }), r))
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
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, '', ?9, ?10, ?11, '', ?12, 'EUR', ?13, 0, 'Demo-Team', ?14, ?14)",
        params![id, ws, name, domain_label, domain_url, street1, city, postcode, country, employees, linkedin, arr_micros, icp, now],
    )?;
    save_custom_fields_for_record(conn, "twenty_companies", "company", &id, v);
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
    save_custom_fields_for_record(&conn, "twenty_companies", "company", &id, &v);
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
    Ok(attach_custom_fields(json!({
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
    }), r))
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
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'DE', '+49', ?7, ?8, '', ?9, 0, 'Demo-Team', ?10, ?10)",
        params![id, ws, first, last, email, phone, job, city, company_id, now],
    )?;
    save_custom_fields_for_record(conn, "twenty_people", "person", &id, v);
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
    if let Some(cid) = v.get("companyId") {
        let cid_opt = cid.as_str().map(str::trim).filter(|s| !s.is_empty());
        let _ = conn.execute("UPDATE twenty_people SET company_id = ?1, updated_at = ?2 WHERE id = ?3", params![cid_opt, now, id]);
    }
    save_custom_fields_for_record(&conn, "twenty_people", "person", &id, &v);
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
    Ok(attach_custom_fields(json!({
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
    }), r))
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
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, NULL, 0, 'Demo-Team', ?10, ?10)",
        params![id, ws, name, amount_micros, curr, close_date, stage, company_id, poc_id, now],
    )?;
    save_custom_fields_for_record(conn, "twenty_opportunities", "opportunity", &id, v);
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
    if let Some(cid) = v.get("companyId") {
        let cid_opt = cid.as_str().map(str::trim).filter(|s| !s.is_empty());
        let _ = conn.execute("UPDATE twenty_opportunities SET company_id = ?1, updated_at = ?2 WHERE id = ?3", params![cid_opt, now, id]);
    }
    if let Some(pid) = v.get("pointOfContactId") {
        let pid_opt = pid.as_str().map(str::trim).filter(|s| !s.is_empty());
        let _ = conn.execute("UPDATE twenty_opportunities SET point_of_contact_id = ?1, updated_at = ?2 WHERE id = ?3", params![pid_opt, now, id]);
    }
    save_custom_fields_for_record(&conn, "twenty_opportunities", "opportunity", &id, &v);
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
    Ok(attach_custom_fields(json!({
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
    }), r))
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
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0, 'Demo-Team', ?8, ?8)",
        params![id, ws, title, body, company_id, person_id, opp_id, now],
    )?;
    save_custom_fields_for_record(conn, "twenty_notes", "note", &id, v);
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
    save_custom_fields_for_record(&conn, "twenty_notes", "note", &id, &v);
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
    Ok(attach_custom_fields(json!({
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
    }), r))
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
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, NULL, 0, 'Demo-Team', ?10, ?10)",
        params![id, ws, title, body, status, due_at, company_id, person_id, opp_id, now],
    )?;
    save_custom_fields_for_record(conn, "twenty_tasks", "task", &id, v);
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
    save_custom_fields_for_record(&conn, "twenty_tasks", "task", &id, &v);
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
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 'Demo-Team', ?10)",
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
            "errors": [{ "message": "Unrecognized GraphQL query on Conrad CRM engine" }]
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
                "description": "Startet einen lokalen Workflow in Conrad Automation.",
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
            "Du bist der integrierte Conrad CRM & Conrad Automation KI-Assistent fuer Workspace '{}' (Demo-Umgebung). \
             Heutiges Datum: {} (morgen: {}, morgen 10 Uhr = {}). \
             Antworte auf Deutsch, praezise und nutze die verfuegbaren Tools fuer CRM-Suchen, Kontakte, Deals, Notizen, Aufgaben, \
             Workflows und E-Mail-Entwuerfe (immer draftOnly=true, niemals E-Mails direkt versenden).",
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
                    "rustStack": "Conrad CRM (/twenty, bis zu 10× schneller, unbegrenzte Nutzer)",
                    "rustEur": 0
                },
                {
                    "category": "Workflow-Automatisierung & Integrationen",
                    "legacyStack": "Zapier Company / Make Enterprise / n8n Cloud",
                    "legacyEur": 850,
                    "rustStack": format!("Conrad Automation (/n8n, {} Demo-Workflows, Echtzeit-Engine)", n8n_workflows),
                    "rustEur": 0
                },
                {
                    "category": "KI-Assistent, Tool Calling & E-Mail-Triage",
                    "legacyStack": "OpenAI / Claude Team + KI-Add-ons",
                    "legacyEur": 1450,
                    "rustStack": "Conrad KI Engine (nachtrainiertes Spezial-Modell, 100 % lokal)",
                    "rustEur": 0
                },
                {
                    "category": "Cloud-Datenbank, Node/Docker-Cluster & Hosting",
                    "legacyStack": "Managed PostgreSQL, Redis, Kubernetes-Cluster & Monitoring",
                    "legacyEur": 900,
                    "rustStack": "1x deutscher Server (<20 €/Monat) · bis zu 100× kompakter",
                    "rustEur": 19
                }
            ]
        }
    }))
}

#[derive(Debug, Deserialize)]
pub struct MetaQueryParams {
    pub workspace: Option<String>,
    pub object: Option<String>,
    #[serde(rename = "objectName")]
    pub object_name: Option<String>,
    pub ids: Option<String>,
}

fn resolve_twenty_object(raw: &str) -> Option<(&'static str, &'static str)> {
    match raw.trim().to_lowercase().as_str() {
        "company" | "companies" | "unternehmen" => Some(("company", "twenty_companies")),
        "person" | "people" | "personen" => Some(("person", "twenty_people")),
        "opportunity" | "opportunities" | "verkaufschancen" => {
            Some(("opportunity", "twenty_opportunities"))
        }
        "note" | "notes" | "notizen" => Some(("note", "twenty_notes")),
        "task" | "tasks" | "aufgaben" => Some(("task", "twenty_tasks")),
        _ => None,
    }
}

fn sanitize_field_identifier(raw: &str) -> String {
    let mut out = String::new();
    let mut prev_us = false;
    for ch in raw.trim().chars() {
        let mapped = match ch {
            'ä' | 'Ä' => Some('a'),
            'ö' | 'Ö' => Some('o'),
            'ü' | 'Ü' => Some('u'),
            'ß' => Some('s'),
            c if c.is_ascii_alphanumeric() => Some(c.to_ascii_lowercase()),
            _ => None,
        };
        if let Some(c) = mapped {
            out.push(c);
            prev_us = false;
        } else if !out.is_empty() && !prev_us {
            out.push('_');
            prev_us = true;
        }
    }
    let trimmed = out.trim_matches('_').to_string();
    if trimmed.is_empty() {
        "benutzerdefiniertes_feld".to_string()
    } else if trimmed.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        format!("feld_{trimmed}")
    } else {
        trimmed
    }
}

pub async fn create_metadata_field_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let obj_raw = v
        .get("objectName")
        .or_else(|| v.get("object_name"))
        .or_else(|| v.get("object"))
        .and_then(|x| x.as_str())
        .unwrap_or("company");
    let Some((obj_singular, _)) = resolve_twenty_object(obj_raw) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Unbekanntes CRM-Objekt"})),
        );
    };
    let label = v
        .get("label")
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("Neues Feld")
        .to_string();
    let name_src = v
        .get("name")
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(&label);
    let name = sanitize_field_identifier(name_src);
    let field_type = v
        .get("type")
        .or_else(|| v.get("fieldType"))
        .or_else(|| v.get("field_type"))
        .and_then(|x| x.as_str())
        .unwrap_or("TEXT")
        .trim()
        .to_uppercase();
    let options_val = v.get("options").cloned().unwrap_or_else(|| json!([]));
    let options_json = if options_val.is_array() {
        options_val.to_string()
    } else {
        "[]".to_string()
    };
    let default_value = v
        .get("defaultValue")
        .or_else(|| v.get("default_value"))
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .to_string();
    let ws = v
        .get("workspace")
        .and_then(|x| x.as_str())
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| {
            if headers.get("host").is_some() {
                "all".to_string()
            } else {
                "all".to_string()
            }
        });

    let conn = state.db.lock_conn();
    let exists: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM twenty_field_metadata WHERE object_name = ?1 AND name = ?2",
            params![obj_singular, name],
            |r| r.get(0),
        )
        .unwrap_or(0);
    if exists > 0 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": format!("Feld '{}' existiert für '{}' bereits", name, obj_singular)})),
        );
    }

    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    if let Err(e) = conn.execute(
        "INSERT INTO twenty_field_metadata
         (id, workspace, object_name, name, label, field_type, options_json, default_value, is_custom, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 1, ?9)",
        params![id, ws, obj_singular, name, label, field_type, options_json, default_value, now],
    ) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        );
    }

    let opts: Value = serde_json::from_str(&options_json).unwrap_or_else(|_| json!([]));
    (
        StatusCode::CREATED,
        Json(json!({
            "id": id,
            "workspace": ws,
            "objectName": obj_singular,
            "name": name,
            "label": label,
            "type": field_type,
            "options": opts,
            "defaultValue": default_value,
            "isCustom": true,
            "createdAt": now
        })),
    )
}

pub async fn delete_metadata_field_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let n = conn
        .execute(
            "DELETE FROM twenty_field_metadata WHERE id = ?1 AND is_custom = 1",
            [&id],
        )
        .unwrap_or(0);
    Json(json!({"deleted": n > 0, "id": id}))
}

pub async fn list_views_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(p): Query<MetaQueryParams>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, p.workspace.as_deref());
    let obj_filter = p
        .object
        .as_deref()
        .or(p.object_name.as_deref())
        .and_then(resolve_twenty_object)
        .map(|(s, _)| s.to_string());
    let conn = state.db.lock_conn();
    let mut views = Vec::new();
    let sql = if obj_filter.is_some() {
        "SELECT id, workspace, object_name, name, layout, filter_json, sort_json, columns_json, position, created_at, updated_at
         FROM twenty_views WHERE (workspace = ?1 OR workspace = 'all' OR ?1 = 'all') AND object_name = ?2
         ORDER BY position ASC, created_at ASC"
    } else {
        "SELECT id, workspace, object_name, name, layout, filter_json, sort_json, columns_json, position, created_at, updated_at
         FROM twenty_views WHERE (workspace = ?1 OR workspace = 'all' OR ?1 = 'all')
         ORDER BY position ASC, created_at ASC"
    };
    if let Ok(mut st) = conn.prepare(sql) {
        let map_row = |r: &rusqlite::Row<'_>| -> rusqlite::Result<Value> {
            let filters: Value = serde_json::from_str(&r.get::<_, String>(5).unwrap_or_else(|_| "{}".into())).unwrap_or_else(|_| json!({}));
            let sort: Value = serde_json::from_str(&r.get::<_, String>(6).unwrap_or_else(|_| "{}".into())).unwrap_or_else(|_| json!({}));
            let columns: Value = serde_json::from_str(&r.get::<_, String>(7).unwrap_or_else(|_| "[]".into())).unwrap_or_else(|_| json!([]));
            Ok(json!({
                "id": r.get::<_, String>(0)?,
                "workspace": r.get::<_, String>(1)?,
                "objectName": r.get::<_, String>(2)?,
                "name": r.get::<_, String>(3)?,
                "layout": r.get::<_, String>(4)?,
                "filters": filters,
                "sort": sort,
                "columns": columns,
                "position": r.get::<_, i64>(8).unwrap_or(0),
                "createdAt": r.get::<_, String>(9)?,
                "updatedAt": r.get::<_, String>(10)?
            }))
        };
        if let Some(obj) = obj_filter.as_deref() {
            if let Ok(mut rows) = st.query(params![ws, obj]) {
                while let Ok(Some(r)) = rows.next() {
                    if let Ok(v) = map_row(r) {
                        views.push(v);
                    }
                }
            }
        } else if let Ok(mut rows) = st.query(params![ws]) {
            while let Ok(Some(r)) = rows.next() {
                if let Ok(v) = map_row(r) {
                    views.push(v);
                }
            }
        }
    }
    Json(json!({"views": views}))
}

pub async fn create_view_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, v.get("workspace").and_then(|x| x.as_str()));
    let obj_raw = v
        .get("objectName")
        .or_else(|| v.get("object"))
        .and_then(|x| x.as_str())
        .unwrap_or("company");
    let obj_name = resolve_twenty_object(obj_raw)
        .map(|(s, _)| s)
        .unwrap_or("company");
    let name = v
        .get("name")
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("Gespeicherte Ansicht")
        .to_string();
    let layout = v
        .get("layout")
        .and_then(|x| x.as_str())
        .unwrap_or("table")
        .to_lowercase();
    let filters = v.get("filters").cloned().unwrap_or_else(|| json!({}));
    let sort = v.get("sort").cloned().unwrap_or_else(|| json!({}));
    let columns = v.get("columns").cloned().unwrap_or_else(|| json!([]));
    let pos = v.get("position").and_then(|x| x.as_i64()).unwrap_or(0);
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let conn = state.db.lock_conn();
    if let Err(e) = conn.execute(
        "INSERT INTO twenty_views (id, workspace, object_name, name, layout, filter_json, sort_json, columns_json, position, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?10)",
        params![
            id,
            ws,
            obj_name,
            name,
            layout,
            filters.to_string(),
            sort.to_string(),
            columns.to_string(),
            pos,
            now
        ],
    ) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        );
    }
    (
        StatusCode::CREATED,
        Json(json!({
            "id": id,
            "workspace": ws,
            "objectName": obj_name,
            "name": name,
            "layout": layout,
            "filters": filters,
            "sort": sort,
            "columns": columns,
            "position": pos,
            "createdAt": now,
            "updatedAt": now
        })),
    )
}

pub async fn update_view_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let now = Utc::now().to_rfc3339();
    if let Some(name) = v.get("name").and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE twenty_views SET name = ?1, updated_at = ?2 WHERE id = ?3", params![name.trim(), now, id]);
    }
    if let Some(layout) = v.get("layout").and_then(|x| x.as_str()) {
        let _ = conn.execute("UPDATE twenty_views SET layout = ?1, updated_at = ?2 WHERE id = ?3", params![layout.to_lowercase(), now, id]);
    }
    if let Some(filters) = v.get("filters") {
        let _ = conn.execute("UPDATE twenty_views SET filter_json = ?1, updated_at = ?2 WHERE id = ?3", params![filters.to_string(), now, id]);
    }
    if let Some(sort) = v.get("sort") {
        let _ = conn.execute("UPDATE twenty_views SET sort_json = ?1, updated_at = ?2 WHERE id = ?3", params![sort.to_string(), now, id]);
    }
    if let Some(cols) = v.get("columns") {
        let _ = conn.execute("UPDATE twenty_views SET columns_json = ?1, updated_at = ?2 WHERE id = ?3", params![cols.to_string(), now, id]);
    }
    match conn.query_row(
        "SELECT id, workspace, object_name, name, layout, filter_json, sort_json, columns_json, position, created_at, updated_at
         FROM twenty_views WHERE id = ?1",
        [&id],
        |r| {
            let filters: Value = serde_json::from_str(&r.get::<_, String>(5).unwrap_or_else(|_| "{}".into())).unwrap_or_else(|_| json!({}));
            let sort: Value = serde_json::from_str(&r.get::<_, String>(6).unwrap_or_else(|_| "{}".into())).unwrap_or_else(|_| json!({}));
            let columns: Value = serde_json::from_str(&r.get::<_, String>(7).unwrap_or_else(|_| "[]".into())).unwrap_or_else(|_| json!([]));
            Ok(json!({
                "id": r.get::<_, String>(0)?,
                "workspace": r.get::<_, String>(1)?,
                "objectName": r.get::<_, String>(2)?,
                "name": r.get::<_, String>(3)?,
                "layout": r.get::<_, String>(4)?,
                "filters": filters,
                "sort": sort,
                "columns": columns,
                "position": r.get::<_, i64>(8).unwrap_or(0),
                "createdAt": r.get::<_, String>(9)?,
                "updatedAt": r.get::<_, String>(10)?
            }))
        },
    ).optional() {
        Ok(Some(row)) => (StatusCode::OK, Json(row)),
        Ok(None) => (StatusCode::NOT_FOUND, Json(json!({"error": "Ansicht nicht gefunden"}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))),
    }
}

pub async fn delete_view_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let n = conn.execute("DELETE FROM twenty_views WHERE id = ?1", [&id]).unwrap_or(0);
    Json(json!({"deleted": n > 0, "id": id}))
}

pub async fn list_favorites_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(p): Query<MetaQueryParams>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, p.workspace.as_deref());
    let conn = state.db.lock_conn();
    let mut favorites = Vec::new();
    if let Ok(mut st) = conn.prepare(
        "SELECT id, workspace, label, target_type, target_id, position, created_at
         FROM twenty_favorites WHERE (workspace = ?1 OR ?1 = 'all')
         ORDER BY position ASC, created_at ASC",
    ) {
        if let Ok(mut rows) = st.query([&ws]) {
            while let Ok(Some(r)) = rows.next() {
                favorites.push(json!({
                    "id": r.get::<_, String>(0).unwrap_or_default(),
                    "workspace": r.get::<_, String>(1).unwrap_or_default(),
                    "label": r.get::<_, String>(2).unwrap_or_default(),
                    "targetType": r.get::<_, String>(3).unwrap_or_else(|_| "object".into()),
                    "targetId": r.get::<_, String>(4).unwrap_or_default(),
                    "position": r.get::<_, i64>(5).unwrap_or(0),
                    "createdAt": r.get::<_, String>(6).unwrap_or_default()
                }));
            }
        }
    }
    Json(json!({"favorites": favorites}))
}

pub async fn create_favorite_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, v.get("workspace").and_then(|x| x.as_str()));
    let label = v.get("label").and_then(|x| x.as_str()).unwrap_or("Favorit").trim().to_string();
    let target_type = v.get("targetType").or_else(|| v.get("target_type")).and_then(|x| x.as_str()).unwrap_or("object").to_string();
    let target_id = v.get("targetId").or_else(|| v.get("target_id")).and_then(|x| x.as_str()).unwrap_or("").to_string();
    let pos = v.get("position").and_then(|x| x.as_i64()).unwrap_or(0);
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let conn = state.db.lock_conn();
    if let Err(e) = conn.execute(
        "INSERT INTO twenty_favorites (id, workspace, label, target_type, target_id, position, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![id, ws, label, target_type, target_id, pos, now],
    ) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})));
    }
    (
        StatusCode::CREATED,
        Json(json!({
            "id": id,
            "workspace": ws,
            "label": label,
            "targetType": target_type,
            "targetId": target_id,
            "position": pos,
            "createdAt": now
        })),
    )
}

pub async fn delete_favorite_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let n = conn.execute("DELETE FROM twenty_favorites WHERE id = ?1", [&id]).unwrap_or(0);
    Json(json!({"deleted": n > 0, "id": id}))
}

pub async fn list_webhooks_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(p): Query<MetaQueryParams>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, p.workspace.as_deref());
    let conn = state.db.lock_conn();
    let mut webhooks = Vec::new();
    if let Ok(mut st) = conn.prepare(
        "SELECT id, workspace, target_url, event_pattern, description, active, secret, created_at
         FROM twenty_webhooks WHERE (workspace = ?1 OR ?1 = 'all') ORDER BY created_at DESC",
    ) {
        if let Ok(mut rows) = st.query([&ws]) {
            while let Ok(Some(r)) = rows.next() {
                let sec = r.get::<_, String>(6).unwrap_or_default();
                webhooks.push(json!({
                    "id": r.get::<_, String>(0).unwrap_or_default(),
                    "workspace": r.get::<_, String>(1).unwrap_or_default(),
                    "targetUrl": r.get::<_, String>(2).unwrap_or_default(),
                    "eventPattern": r.get::<_, String>(3).unwrap_or_else(|_| "*".into()),
                    "description": r.get::<_, String>(4).unwrap_or_default(),
                    "active": r.get::<_, i64>(5).unwrap_or(1) != 0,
                    "hasSecret": !sec.is_empty(),
                    "createdAt": r.get::<_, String>(7).unwrap_or_default()
                }));
            }
        }
    }
    Json(json!({"webhooks": webhooks}))
}

pub async fn create_webhook_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, v.get("workspace").and_then(|x| x.as_str()));
    let target_url = v
        .get("targetUrl")
        .or_else(|| v.get("target_url"))
        .or_else(|| v.get("url"))
        .and_then(|x| x.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    if target_url.is_empty() || (!target_url.starts_with("http://") && !target_url.starts_with("https://")) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Gültige http(s):// Ziel-URL erforderlich"})),
        );
    }
    let event_pattern = v
        .get("eventPattern")
        .or_else(|| v.get("event_pattern"))
        .or_else(|| v.get("events"))
        .and_then(|x| x.as_str())
        .unwrap_or("*")
        .trim()
        .to_string();
    let description = v.get("description").and_then(|x| x.as_str()).unwrap_or("").trim().to_string();
    let active = v.get("active").and_then(|x| x.as_bool()).unwrap_or(true);
    let secret = v
        .get("secret")
        .and_then(|x| x.as_str())
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| format!("whsec_{}", Uuid::new_v4().simple()));
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let conn = state.db.lock_conn();
    if let Err(e) = conn.execute(
        "INSERT INTO twenty_webhooks (id, workspace, target_url, event_pattern, description, active, secret, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![id, ws, target_url, event_pattern, description, i64::from(active), secret, now],
    ) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})));
    }
    (
        StatusCode::CREATED,
        Json(json!({
            "id": id,
            "workspace": ws,
            "targetUrl": target_url,
            "eventPattern": event_pattern,
            "description": description,
            "active": active,
            "secretPreview": format!("{}...", &secret.chars().take(12).collect::<String>()),
            "createdAt": now
        })),
    )
}

pub async fn delete_webhook_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let n = conn.execute("DELETE FROM twenty_webhooks WHERE id = ?1", [&id]).unwrap_or(0);
    Json(json!({"deleted": n > 0, "id": id}))
}

pub async fn list_api_keys_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(p): Query<MetaQueryParams>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, p.workspace.as_deref());
    let conn = state.db.lock_conn();
    let mut keys = Vec::new();
    if let Ok(mut st) = conn.prepare(
        "SELECT id, workspace, name, key_prefix, expires_at, created_at
         FROM twenty_api_keys WHERE (workspace = ?1 OR ?1 = 'all') ORDER BY created_at DESC",
    ) {
        if let Ok(mut rows) = st.query([&ws]) {
            while let Ok(Some(r)) = rows.next() {
                keys.push(json!({
                    "id": r.get::<_, String>(0).unwrap_or_default(),
                    "workspace": r.get::<_, String>(1).unwrap_or_default(),
                    "name": r.get::<_, String>(2).unwrap_or_default(),
                    "keyPrefix": r.get::<_, String>(3).unwrap_or_default(),
                    "expiresAt": r.get::<_, Option<String>>(4).ok().flatten(),
                    "createdAt": r.get::<_, String>(5).unwrap_or_default()
                }));
            }
        }
    }
    Json(json!({"apiKeys": keys}))
}

pub async fn create_api_key_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    use sha2::{Digest, Sha256};
    let ws = resolve_workspace(&headers, v.get("workspace").and_then(|x| x.as_str()));
    let name = v
        .get("name")
        .and_then(|x| x.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("API-Schlüssel")
        .to_string();
    let expires_at = v
        .get("expiresAt")
        .or_else(|| v.get("expires_at"))
        .and_then(|x| x.as_str())
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string());
    let raw_token = format!("tw_{}_{}", ws, Uuid::new_v4().simple());
    let key_prefix = format!("{}...", &raw_token.chars().take(12).collect::<String>());
    let mut hasher = Sha256::new();
    hasher.update(raw_token.as_bytes());
    let key_hash = hex::encode(hasher.finalize());
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    let conn = state.db.lock_conn();
    if let Err(e) = conn.execute(
        "INSERT INTO twenty_api_keys (id, workspace, name, key_prefix, key_hash, expires_at, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![id, ws, name, key_prefix, key_hash, expires_at, now],
    ) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})));
    }
    (
        StatusCode::CREATED,
        Json(json!({
            "id": id,
            "workspace": ws,
            "name": name,
            "keyPrefix": key_prefix,
            "token": raw_token,
            "expiresAt": expires_at,
            "createdAt": now
        })),
    )
}

pub async fn delete_api_key_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let conn = state.db.lock_conn();
    let n = conn.execute("DELETE FROM twenty_api_keys WHERE id = ?1", [&id]).unwrap_or(0);
    Json(json!({"deleted": n > 0, "id": id}))
}

pub async fn bulk_delete_handler(
    State(state): State<AppState>,
    Path(object): Path<String>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let Some((_, table)) = resolve_twenty_object(&object) else {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": "Unbekanntes Objekt"})));
    };
    let ids: Vec<String> = v
        .get("ids")
        .and_then(|x| x.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|i| i.as_str().map(str::trim).filter(|s| !s.is_empty()).map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    if ids.is_empty() {
        return (StatusCode::OK, Json(json!({"deleted": 0, "ids": []})));
    }
    let conn = state.db.lock_conn();
    let sql = format!("DELETE FROM {table} WHERE id = ?1");
    let mut deleted = 0usize;
    for id in &ids {
        deleted += conn.execute(&sql, [id]).unwrap_or(0);
    }
    (StatusCode::OK, Json(json!({"deleted": deleted, "ids": ids})))
}

pub async fn bulk_update_handler(
    State(state): State<AppState>,
    Path(object): Path<String>,
    Json(v): Json<Value>,
) -> impl IntoResponse {
    let Some((singular, table)) = resolve_twenty_object(&object) else {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": "Unbekanntes Objekt"})));
    };
    let ids: Vec<String> = v
        .get("ids")
        .and_then(|x| x.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|i| i.as_str().map(str::trim).filter(|s| !s.is_empty()).map(str::to_string))
                .collect()
        })
        .unwrap_or_default();
    if ids.is_empty() {
        return (StatusCode::OK, Json(json!({"updated": 0})));
    }
    let patch = v.get("patch").cloned().unwrap_or_else(|| v.clone());
    let conn = state.db.lock_conn();
    let now = Utc::now().to_rfc3339();
    let mut updated = 0usize;
    for id in &ids {
        let mut touched = false;
        match singular {
            "company" => {
                if let Some(icp) = patch.get("icpTarget").or_else(|| patch.get("icp")).and_then(|x| x.as_bool()) {
                    touched |= conn.execute("UPDATE twenty_companies SET icp = ?1, updated_at = ?2 WHERE id = ?3", params![i64::from(icp), now, id]).unwrap_or(0) > 0;
                }
                if let Some(city) = patch.get("city").and_then(|x| x.as_str()) {
                    touched |= conn.execute("UPDATE twenty_companies SET city = ?1, updated_at = ?2 WHERE id = ?3", params![city, now, id]).unwrap_or(0) > 0;
                }
                if let Some(emp) = patch.get("employees").and_then(|x| x.as_i64()) {
                    touched |= conn.execute("UPDATE twenty_companies SET employees = ?1, updated_at = ?2 WHERE id = ?3", params![emp, now, id]).unwrap_or(0) > 0;
                }
            }
            "person" => {
                if let Some(city) = patch.get("city").and_then(|x| x.as_str()) {
                    touched |= conn.execute("UPDATE twenty_people SET city = ?1, updated_at = ?2 WHERE id = ?3", params![city, now, id]).unwrap_or(0) > 0;
                }
                if let Some(jt) = patch.get("jobTitle").and_then(|x| x.as_str()) {
                    touched |= conn.execute("UPDATE twenty_people SET job_title = ?1, updated_at = ?2 WHERE id = ?3", params![jt, now, id]).unwrap_or(0) > 0;
                }
            }
            "opportunity" => {
                if let Some(st) = patch.get("stage").and_then(|x| x.as_str()) {
                    touched |= conn.execute("UPDATE twenty_opportunities SET stage = ?1, updated_at = ?2 WHERE id = ?3", params![st.to_uppercase(), now, id]).unwrap_or(0) > 0;
                }
            }
            "task" => {
                if let Some(st) = patch.get("status").and_then(|x| x.as_str()) {
                    touched |= conn.execute("UPDATE twenty_tasks SET status = ?1, updated_at = ?2 WHERE id = ?3", params![st.to_uppercase(), now, id]).unwrap_or(0) > 0;
                }
            }
            "note" => {
                if let Some(t) = patch.get("title").and_then(|x| x.as_str()) {
                    touched |= conn.execute("UPDATE twenty_notes SET title = ?1, updated_at = ?2 WHERE id = ?3", params![t, now, id]).unwrap_or(0) > 0;
                }
            }
            _ => {}
        }
        save_custom_fields_for_record(&conn, table, singular, id, &patch);
        if touched || patch.get("customFields").is_some() {
            updated += 1;
        }
    }
    (StatusCode::OK, Json(json!({"updated": updated, "ids": ids})))
}

fn csv_escape_cell(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn parse_csv_lines(input: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut cell = String::new();
    let mut in_quotes = false;
    let mut chars = input.trim_start_matches('\u{FEFF}').chars().peekable();
    while let Some(c) = chars.next() {
        if in_quotes {
            if c == '"' {
                if chars.peek() == Some(&'"') {
                    chars.next();
                    cell.push('"');
                } else {
                    in_quotes = false;
                }
            } else {
                cell.push(c);
            }
        } else {
            match c {
                '"' => in_quotes = true,
                ',' | ';' => {
                    row.push(cell.trim().to_string());
                    cell.clear();
                }
                '\n' => {
                    row.push(cell.trim().to_string());
                    cell.clear();
                    if row.iter().any(|x| !x.is_empty()) {
                        rows.push(row);
                    }
                    row = Vec::new();
                }
                '\r' => {
                    if chars.peek() == Some(&'\n') {
                        chars.next();
                    }
                    row.push(cell.trim().to_string());
                    cell.clear();
                    if row.iter().any(|x| !x.is_empty()) {
                        rows.push(row);
                    }
                    row = Vec::new();
                }
                _ => cell.push(c),
            }
        }
    }
    if !cell.is_empty() || !row.is_empty() {
        row.push(cell.trim().to_string());
        if row.iter().any(|x| !x.is_empty()) {
            rows.push(row);
        }
    }
    rows
}

pub async fn export_csv_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(object): Path<String>,
    Query(p): Query<MetaQueryParams>,
) -> impl IntoResponse {
    let ws = resolve_workspace(&headers, p.workspace.as_deref());
    let Some((singular, _)) = resolve_twenty_object(&object) else {
        return (
            StatusCode::BAD_REQUEST,
            [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8".to_string())],
            "Unbekanntes Objekt".to_string(),
        )
            .into_response();
    };
    let selected_ids: Vec<String> = p
        .ids
        .as_deref()
        .map(|s| s.split(',').map(str::trim).filter(|x| !x.is_empty()).map(str::to_string).collect())
        .unwrap_or_default();

    let conn = state.db.lock_conn();
    let (clause, ws_param) = ws_filter_clause(&ws);
    let mut custom_fields: Vec<String> = Vec::new();
    if let Ok(mut st) = conn.prepare("SELECT name FROM twenty_field_metadata WHERE object_name = ?1 ORDER BY created_at ASC") {
        if let Ok(mut rows) = st.query([singular]) {
            while let Ok(Some(r)) = rows.next() {
                if let Ok(n) = r.get::<_, String>(0) {
                    custom_fields.push(n);
                }
            }
        }
    }

    let mut records: Vec<Value> = Vec::new();
    let push_if_selected = |records: &mut Vec<Value>, val: Value| {
        if selected_ids.is_empty()
            || val
                .get("id")
                .and_then(|x| x.as_str())
                .is_some_and(|id| selected_ids.iter().any(|s| s == id))
        {
            records.push(val);
        }
    };

    match singular {
        "company" => {
            let sql = format!("SELECT * FROM twenty_companies WHERE {clause} ORDER BY name ASC");
            if let Ok(mut st) = conn.prepare(&sql) {
                let mut rows = if ws == "all" { st.query([]) } else { st.query([&ws_param]) };
                if let Ok(ref mut rset) = rows {
                    while let Ok(Some(r)) = rset.next() {
                        if let Ok(v) = company_row_to_json(r) {
                            push_if_selected(&mut records, v);
                        }
                    }
                }
            }
        }
        "person" => {
            let sql = format!("SELECT p.*, c.name AS company_name FROM twenty_people p LEFT JOIN twenty_companies c ON p.company_id = c.id WHERE p.{} ORDER BY p.last_name ASC", clause.replace("workspace", "p.workspace"));
            if let Ok(mut st) = conn.prepare(&sql) {
                let mut rows = if ws == "all" { st.query([]) } else { st.query([&ws_param]) };
                if let Ok(ref mut rset) = rows {
                    while let Ok(Some(r)) = rset.next() {
                        if let Ok(v) = person_row_to_json(r) {
                            push_if_selected(&mut records, v);
                        }
                    }
                }
            }
        }
        "opportunity" => {
            let sql = format!("SELECT o.*, c.name AS company_name, TRIM(COALESCE(p.first_name,'') || ' ' || COALESCE(p.last_name,'')) AS contact_name FROM twenty_opportunities o LEFT JOIN twenty_companies c ON o.company_id = c.id LEFT JOIN twenty_people p ON o.point_of_contact_id = p.id WHERE {} ORDER BY o.created_at DESC", clause.replace("workspace", "o.workspace"));
            if let Ok(mut st) = conn.prepare(&sql) {
                let mut rows = if ws == "all" { st.query([]) } else { st.query([&ws_param]) };
                if let Ok(ref mut rset) = rows {
                    while let Ok(Some(r)) = rset.next() {
                        if let Ok(v) = opportunity_row_to_json(r) {
                            push_if_selected(&mut records, v);
                        }
                    }
                }
            }
        }
        "note" => {
            let sql = format!("SELECT n.*, c.name AS company_name FROM twenty_notes n LEFT JOIN twenty_companies c ON n.company_id = c.id WHERE {} ORDER BY n.created_at DESC", clause.replace("workspace", "n.workspace"));
            if let Ok(mut st) = conn.prepare(&sql) {
                let mut rows = if ws == "all" { st.query([]) } else { st.query([&ws_param]) };
                if let Ok(ref mut rset) = rows {
                    while let Ok(Some(r)) = rset.next() {
                        if let Ok(v) = note_row_to_json(r) {
                            push_if_selected(&mut records, v);
                        }
                    }
                }
            }
        }
        "task" => {
            let sql = format!("SELECT t.*, c.name AS company_name FROM twenty_tasks t LEFT JOIN twenty_companies c ON t.company_id = c.id WHERE {} ORDER BY t.created_at DESC", clause.replace("workspace", "t.workspace"));
            if let Ok(mut st) = conn.prepare(&sql) {
                let mut rows = if ws == "all" { st.query([]) } else { st.query([&ws_param]) };
                if let Ok(ref mut rset) = rows {
                    while let Ok(Some(r)) = rset.next() {
                        if let Ok(v) = task_row_to_json(r) {
                            push_if_selected(&mut records, v);
                        }
                    }
                }
            }
        }
        _ => {}
    }

    let mut cols: Vec<String> = match singular {
        "company" => vec!["id", "name", "domain", "city", "employees", "arrEur", "icpTarget"],
        "person" => vec!["id", "firstName", "lastName", "email", "phone", "jobTitle", "city", "companyName"],
        "opportunity" => vec!["id", "name", "stage", "amountEur", "closeDate", "companyName"],
        "note" => vec!["id", "title", "bodyMarkdown", "companyName", "createdAt"],
        "task" => vec!["id", "title", "status", "dueAt", "companyName", "bodyMarkdown"],
        _ => vec!["id"],
    }
    .into_iter()
    .map(String::from)
    .collect();
    for cf in custom_fields {
        if !cols.contains(&cf) {
            cols.push(cf);
        }
    }

    let mut csv_lines = Vec::with_capacity(records.len() + 1);
    csv_lines.push(cols.iter().map(|c| csv_escape_cell(c)).collect::<Vec<_>>().join(","));
    for rec in &records {
        let mut cells = Vec::with_capacity(cols.len());
        for col in &cols {
            let s = match col.as_str() {
                "domain" => rec.pointer("/domainName/primaryLinkUrl").and_then(|x| x.as_str()).unwrap_or("").to_string(),
                "city" => rec.get("city").or_else(|| rec.pointer("/address/addressCity")).and_then(|x| x.as_str()).unwrap_or("").to_string(),
                "firstName" => rec.pointer("/name/firstName").or_else(|| rec.get("firstName")).and_then(|x| x.as_str()).unwrap_or("").to_string(),
                "lastName" => rec.pointer("/name/lastName").or_else(|| rec.get("lastName")).and_then(|x| x.as_str()).unwrap_or("").to_string(),
                "email" => rec.pointer("/emails/primaryEmail").or_else(|| rec.get("email")).and_then(|x| x.as_str()).unwrap_or("").to_string(),
                "phone" => rec.pointer("/phones/primaryPhoneNumber").or_else(|| rec.get("phone")).and_then(|x| x.as_str()).unwrap_or("").to_string(),
                _ => match rec.get(col).or_else(|| rec.pointer(&format!("/customFields/{col}"))) {
                    Some(Value::String(s)) => s.clone(),
                    Some(Value::Number(n)) => n.to_string(),
                    Some(Value::Bool(b)) => b.to_string(),
                    Some(Value::Null) | None => String::new(),
                    Some(other) => other.to_string(),
                },
            };
            cells.push(csv_escape_cell(&s));
        }
        csv_lines.push(cells.join(","));
    }

    (
        StatusCode::OK,
        [
            (axum::http::header::CONTENT_TYPE, "text/csv; charset=utf-8".to_string()),
            (
                axum::http::header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{singular}.csv\""),
            ),
        ],
        csv_lines.join("\n"),
    )
        .into_response()
}

pub async fn import_csv_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(object): Path<String>,
    Query(p): Query<MetaQueryParams>,
    body: axum::body::Bytes,
) -> impl IntoResponse {
    let Some((singular, _)) = resolve_twenty_object(&object) else {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": "Unbekanntes Objekt"})));
    };
    let raw_str = String::from_utf8_lossy(&body).to_string();
    let mut ws = resolve_workspace(&headers, p.workspace.as_deref());
    let mut row_objs: Vec<Value> = Vec::new();

    if let Ok(json_body) = serde_json::from_str::<Value>(&raw_str) {
        if let Some(w) = json_body.get("workspace").and_then(|x| x.as_str()) {
            ws = resolve_workspace(&headers, Some(w));
        }
        if let Some(arr) = json_body.get("rows").and_then(|x| x.as_array()) {
            row_objs = arr.clone();
        } else if let Some(csv_text) = json_body.get("csv").and_then(|x| x.as_str()) {
            let parsed = parse_csv_lines(csv_text);
            if parsed.len() >= 2 {
                let hdr = &parsed[0];
                for line in &parsed[1..] {
                    let mut map = serde_json::Map::new();
                    for (idx, key) in hdr.iter().enumerate() {
                        if let Some(val) = line.get(idx) {
                            if !key.is_empty() && !val.is_empty() {
                                map.insert(key.clone(), Value::String(val.clone()));
                            }
                        }
                    }
                    row_objs.push(Value::Object(map));
                }
            }
        }
    } else {
        let parsed = parse_csv_lines(&raw_str);
        if parsed.len() >= 2 {
            let hdr = &parsed[0];
            for line in &parsed[1..] {
                let mut map = serde_json::Map::new();
                for (idx, key) in hdr.iter().enumerate() {
                    if let Some(val) = line.get(idx) {
                        if !key.is_empty() && !val.is_empty() {
                            map.insert(key.clone(), Value::String(val.clone()));
                        }
                    }
                }
                row_objs.push(Value::Object(map));
            }
        }
    }

    let conn = state.db.lock_conn();
    let mut imported = 0usize;
    let mut errors = Vec::new();
    for (idx, row) in row_objs.iter().enumerate() {
        let res = match singular {
            "company" => create_company_internal(&conn, &ws, row),
            "person" => create_person_internal(&conn, &ws, row),
            "opportunity" => create_opportunity_internal(&conn, &ws, row),
            "note" => create_note_internal(&conn, &ws, row),
            "task" => create_task_internal(&conn, &ws, row),
            _ => continue,
        };
        match res {
            Ok(_) => imported += 1,
            Err(e) => errors.push(json!({"row": idx + 1, "error": e.to_string()})),
        }
    }

    (
        StatusCode::OK,
        Json(json!({
            "object": singular,
            "workspace": ws,
            "imported": imported,
            "errors": errors
        })),
    )
}
