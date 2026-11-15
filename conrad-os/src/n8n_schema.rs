//! Vollstaendige n8n Node- und Credential-Type-Registry (80+ Nodes, 45+ Credential-Typen).
//!
//! Bedient `GET /types/nodes.json`, `GET /types/credentials.json` sowie `GET /rest/node-types`
//! und liefert typisierte `properties`-Schemata mit `displayOptions` fuer den NDV-Editor.

use serde_json::{json, Value};

pub const BLANK_SENTINEL: &str = "__n8n_BLANK_VALUE_e5362baf-c777-4d57-a609-6eaf1f9e87f6";

struct NodeSpec {
    name: &'static str,
    display_name: &'static str,
    category: &'static str,
    icon: &'static str,
    group: &'static [&'static str],
    description: &'static str,
    inputs: &'static [&'static str],
    outputs: &'static [&'static str],
    credential: Option<&'static str>,
    usable_as_tool: bool,
    properties: fn() -> Vec<Value>,
}

fn p_str(name: &str, label: &str, default: &str, desc: &str) -> Value {
    json!({
        "displayName": label,
        "name": name,
        "type": "string",
        "default": default,
        "description": desc,
        "supportsExpression": true
    })
}

fn p_pw(name: &str, label: &str, desc: &str) -> Value {
    json!({
        "displayName": label,
        "name": name,
        "type": "string",
        "typeOptions": { "password": true },
        "default": "",
        "description": desc
    })
}

fn p_num(name: &str, label: &str, default: f64, desc: &str) -> Value {
    json!({
        "displayName": label,
        "name": name,
        "type": "number",
        "default": default,
        "description": desc,
        "supportsExpression": true
    })
}

fn p_bool(name: &str, label: &str, default: bool, desc: &str) -> Value {
    json!({
        "displayName": label,
        "name": name,
        "type": "boolean",
        "default": default,
        "description": desc
    })
}

fn p_opt(name: &str, label: &str, default: &str, opts: &[(&str, &str)], desc: &str) -> Value {
    let list: Vec<Value> = opts
        .iter()
        .map(|(n, v)| json!({ "name": n, "value": v }))
        .collect();
    json!({
        "displayName": label,
        "name": name,
        "type": "options",
        "default": default,
        "options": list,
        "description": desc
    })
}

fn p_json(name: &str, label: &str, default: &str, desc: &str) -> Value {
    json!({
        "displayName": label,
        "name": name,
        "type": "json",
        "default": default,
        "description": desc,
        "supportsExpression": true
    })
}

fn p_code(name: &str, label: &str, default: &str, desc: &str) -> Value {
    json!({
        "displayName": label,
        "name": name,
        "type": "string",
        "typeOptions": { "editor": "codeNodeEditor", "rows": 8 },
        "default": default,
        "description": desc
    })
}

fn with_show(mut prop: Value, show_key: &str, values: &[&str]) -> Value {
    let vals: Vec<Value> = values.iter().map(|s| json!(s)).collect();
    prop["displayOptions"] = json!({ "show": { show_key: vals } });
    prop
}

fn props_http_clean() -> Vec<Value> {
    vec![
        p_opt(
            "method",
            "HTTP-Methode",
            "GET",
            &[("GET", "GET"), ("POST", "POST"), ("PUT", "PUT"), ("PATCH", "PATCH"), ("DELETE", "DELETE"), ("HEAD", "HEAD")],
            "Auszufuehrende HTTP-Anfragemethode",
        ),
        p_str("url", "URL", "https://api.example.com/v1/resource", "Ziel-URL der Anfrage (unterstuetzt {{ }} Expressions)"),
        p_opt(
            "authentication",
            "Authentifizierung",
            "none",
            &[("Keine", "none"), ("Credential verwenden", "predefinedCredentialType"), ("Bearer Header", "genericCredentialType")],
            "Art der Authentifizierung",
        ),
        p_bool("sendHeaders", "Header senden", false, "Benutzerdefinierte HTTP-Header mitgeben"),
        with_show(p_json("headerParametersJson", "Header (JSON)", "{}", "Header als JSON-Objekt"), "sendHeaders", &["true"]),
        p_bool("sendBody", "Body senden", false, "Request-Body mitsenden"),
        with_show(p_json("jsonBody", "JSON-Body", "{}", "JSON-Nutzlast"), "sendBody", &["true"]),
        p_num("timeout", "Timeout (ms)", 10000.0, "Timeout in Millisekunden"),
    ]
}

fn props_code() -> Vec<Value> {
    vec![
        p_opt(
            "language",
            "Sprache",
            "javaScript",
            &[("JavaScript (QuickJS Sandbox)", "javaScript"), ("Python 3", "python")],
            "Ausfuehrungssprache des Code-Nodes",
        ),
        p_opt(
            "mode",
            "Modus",
            "runOnceForAllItems",
            &[("Einmal fuer alle Items", "runOnceForAllItems"), ("Fuer jedes Item ausfuehren", "runOnceForEachItem")],
            "Steuert ob items gebuendelt oder einzeln verarbeitet werden",
        ),
        with_show(
            p_code(
                "jsCode",
                "JavaScript Code",
                "for (const item of $input.all()) {\n  item.json.processed = true;\n}\nreturn $input.all();",
                "Zugriff auf $input, $json, $node, $(), $vars, $now, $jmespath",
            ),
            "language",
            &["javaScript"],
        ),
        with_show(
            p_code(
                "pythonCode",
                "Python Code",
                "for item in _items:\n  item['json']['processed'] = True\nreturn _items",
                "Python 3 Ausfuehrung ueber lokale Sandbox",
            ),
            "language",
            &["python"],
        ),
    ]
}

fn props_webhook() -> Vec<Value> {
    vec![
        p_opt("httpMethod", "HTTP-Methode", "POST", &[("GET", "GET"), ("POST", "POST"), ("PUT", "PUT"), ("PATCH", "PATCH"), ("DELETE", "DELETE")], "Erwartete HTTP-Methode"),
        p_str("path", "Webhook-Pfad", "lead-intake", "Relativer Pfad unter /webhook/<pfad>"),
        p_opt(
            "responseMode",
            "Antwortmodus",
            "onReceived",
            &[("Sofort beim Empfang", "onReceived"), ("Nach letztem Node", "lastNode"), ("Mit 'Respond to Webhook'-Node", "responseNode")],
            "Zeitpunkt und Quelle der HTTP-Antwort",
        ),
        with_show(p_num("responseCode", "Statuscode", 200.0, "HTTP-Statuscode der Antwort"), "responseMode", &["onReceived"]),
    ]
}

fn props_schedule() -> Vec<Value> {
    vec![
        p_opt(
            "intervalUnit",
            "Intervall-Einheit",
            "hours",
            &[("Sekunden", "seconds"), ("Minuten", "minutes"), ("Stunden", "hours"), ("Tage", "days"), ("Cron-Ausdruck", "cronExpression")],
            "Ausloese-Rhythmus",
        ),
        p_num("intervalValue", "Alle X Einheiten", 1.0, "Numerischer Intervallwert"),
        with_show(p_str("cronExpression", "Cron-Ausdruck", "0 8 * * 1-5", "Standard 5-Feld Cron"), "intervalUnit", &["cronExpression"]),
    ]
}

fn props_set() -> Vec<Value> {
    vec![
        p_opt("mode", "Modus", "manual", &[("Manuelle Felder", "manual"), ("Rohes JSON", "raw")], "Wie Felder gesetzt werden"),
        p_bool("includeOtherFields", "Andere Felder behalten", true, "Bestehende Eingabefelder mit uebernehmen"),
        with_show(
            p_json("assignments", "Zuweisungen (assignments)", r#"{"assignments":[{"name":"status","value":"neu","type":"string"}]}"#, "Assignments-Struktur fuer Feldbearbeitung"),
            "mode",
            &["manual"],
        ),
        with_show(p_json("jsonOutput", "JSON-Ausgabe", "{\n  \"status\": \"neu\"\n}", "Ausgabeobjekt als JSON"), "mode", &["raw"]),
    ]
}

fn props_if() -> Vec<Value> {
    vec![
        p_opt("combinator", "Verknuepfung", "and", &[("UND (alle erfuellt)", "and"), ("ODER (eine erfuellt)", "or")], "Logische Verknuepfung der Bedingungen"),
        p_json(
            "conditions",
            "Bedingungen (JSON)",
            r#"{"conditions":[{"leftValue":"={{ $json.score }}","operator":{"type":"number","operation":"gte"},"rightValue":70}],"combinator":"and"}"#,
            "Filter-/Vergleichsbedingungen",
        ),
        p_bool("ignoreCase", "Gross-/Kleinschreibung ignorieren", true, "Case-insensitiver String-Vergleich"),
    ]
}

fn props_switch() -> Vec<Value> {
    vec![
        p_opt("mode", "Routing-Modus", "rules", &[("Regeln", "rules"), ("Expression (Ausgangsindex)", "expression")], "Wie der Zielausgang bestimmt wird"),
        p_json("rules", "Regeln (JSON)", r#"{"values":[]}"#, "Verzweigungsregeln fuer Ausgaenge 0..N"),
        with_show(p_str("output", "Ausgangs-Expression", "={{ $json.routeIndex || 0 }}", "Ergibt den 0-basierten Ausgangsindex"), "mode", &["expression"]),
        p_opt("fallbackOutput", "Fallback-Ausgang", "none", &[("Keiner", "none"), ("Zusaetzlicher Ausgang", "extra")], "Wohin Items ohne Treffer gehen"),
    ]
}

fn props_merge() -> Vec<Value> {
    vec![
        p_opt(
            "mode",
            "Merge-Modus",
            "append",
            &[("Anhaengen (Append)", "append"), ("Nach Position kombinieren", "combineByPosition"), ("Nach Schluessel (SQL Join)", "combineByFields"), ("Multiplex", "multiplex")],
            "Strategie zum Zusammenfuehren von Zweigen",
        ),
        with_show(p_str("mergeByFields", "Join-Feld(er)", "id", "Kommagetrennte Feldnamen fuer SQL Join"), "mode", &["combineByFields"]),
        p_opt("joinMode", "Join-Typ", "keepMatches", &[("Nur Treffer (Inner Join)", "keepMatches"), ("Erster Zweig (Left Join)", "enrichInput1"), ("Alle behalten", "keepEverything")], "Join-Verhalten"),
    ]
}

fn props_split_batches() -> Vec<Value> {
    vec![
        p_num("batchSize", "Batch-Groesse", 10.0, "Anzahl Items pro Schleifendurchlauf"),
        p_bool("reset", "Zuruecksetzen", false, "Zaehler am Schleifenanfang zuruecksetzen"),
    ]
}

fn props_wait() -> Vec<Value> {
    vec![
        p_opt("resume", "Fortsetzen nach", "timeInterval", &[("Zeitintervall", "timeInterval"), ("Webhook-Aufruf (/webhook-waiting/:id)", "webhook"), ("Bestimmter Zeitpunkt", "specificTime")], "Wartebedingung"),
        p_num("amount", "Dauer", 1.0, "Wartezeit (wird bei Sync-Ausfuehrung gedeckelt)"),
        p_opt("unit", "Einheit", "seconds", &[("Millisekunden", "milliseconds"), ("Sekunden", "seconds"), ("Minuten", "minutes")], "Zeiteinheit"),
    ]
}

fn props_sort() -> Vec<Value> {
    vec![
        p_str("fieldName", "Sortierfeld", "createdAt", "Feldname nach dem sortiert wird"),
        p_opt("order", "Reihenfolge", "ascending", &[("Aufsteigend (A-Z / 0-9)", "ascending"), ("Absteigend (Z-A / 9-0)", "descending")], "Sortierrichtung"),
    ]
}

fn props_limit() -> Vec<Value> {
    vec![
        p_num("maxItems", "Maximale Items", 10.0, "Anzahl durchzulassender Items"),
        p_opt("keep", "Behalten", "firstItems", &[("Erste Items", "firstItems"), ("Letzte Items", "lastItems")], "Welche Seite behalten wird"),
    ]
}

fn props_remove_dupes() -> Vec<Value> {
    vec![
        p_opt("compare", "Vergleichen", "allFields", &[("Alle Felder", "allFields"), ("Ausgewaehlte Felder", "selectedFields")], "Vergleichsbasis fuer Duplikate"),
        p_str("fieldsToCompare", "Felder (kommagetrennt)", "email", "Schluesselfelder fuer Deduplizierung"),
    ]
}

fn props_aggregate() -> Vec<Value> {
    vec![
        p_opt("aggregate", "Aggregationsmodus", "aggregateAllItemData", &[("Alle Item-Daten als Liste", "aggregateAllItemData"), ("Einzelne Felder buendeln", "aggregateIndividualFields")], "Ausgabeformat"),
        p_str("destinationFieldName", "Zielfeldname", "data", "Name des Listenfeldes"),
    ]
}

fn props_split_out() -> Vec<Value> {
    vec![
        p_str("fieldToSplitOut", "Array-Feld", "items", "Feldname des Arrays das in Einzel-Items zerlegt wird"),
        p_bool("includeOtherFields", "Uebergeordnete Felder behalten", true, "Restfelder auf jedes neue Item kopieren"),
    ]
}

fn props_summarize() -> Vec<Value> {
    vec![
        p_opt("operation", "Berechnung", "count", &[("Anzahl (Count)", "count"), ("Summe (Sum)", "sum"), ("Durchschnitt (Avg)", "average"), ("Minimum", "min"), ("Maximum", "max")], "Aggregationsfunktion"),
        p_str("field", "Wertfeld", "amount", "Numerisches Zielfeld"),
        p_str("groupBy", "Gruppieren nach", "", "Optionales Gruppierungsfeld"),
    ]
}

fn props_datetime() -> Vec<Value> {
    vec![
        p_opt("operation", "Operation", "formatDate", &[("Datum formatieren", "formatDate"), ("Zeit hinzufuegen/abziehen", "addToDate"), ("Differenz berechnen", "getTimeBetweenDates"), ("Aktuelles Datum", "getCurrentDate")], "Datumsoperation"),
        p_str("date", "Eingabedatum", "={{ $now.toISO() }}", "ISO-Datum oder Timestamp"),
        p_str("format", "Ausgabeformat", "dd.MM.yyyy HH:mm", "Luxon Format-String"),
    ]
}

fn props_crypto() -> Vec<Value> {
    vec![
        p_opt("action", "Aktion", "hash", &[("Hash berechnen", "hash"), ("HMAC signieren", "hmac"), ("Zufallsstring (UUID/Hex)", "generate")], "Krypto-Operation"),
        p_opt("type", "Algorithmus", "SHA256", &[("MD5", "MD5"), ("SHA1", "SHA1"), ("SHA256", "SHA256"), ("SHA512", "SHA512")], "Hash-Algorithmus"),
        p_str("value", "Eingabetext", "={{ JSON.stringify($json) }}", "Zu verarbeitender Wert"),
        with_show(p_pw("secret", "HMAC Secret", "Geheimer Schluessel fuer HMAC"), "action", &["hmac"]),
        p_str("dataPropertyName", "Zielfeld", "hash", "Feldname im Ausgangs-JSON"),
    ]
}

fn props_jwt() -> Vec<Value> {
    vec![
        p_opt("operation", "Operation", "sign", &[("Token signieren (HS256)", "sign"), ("Token verifizieren", "verify"), ("Payload dekodieren", "decode")], "JWT-Aktion"),
        p_json("claimsJson", "Claims (JSON)", r#"{"sub":"user-1","role":"admin"}"#, "Nutzdaten des JWT"),
        p_str("token", "JWT Token", "={{ $json.token }}", "Vorhandener Token fuer Verify/Decode"),
    ]
}

fn props_data_table() -> Vec<Value> {
    vec![
        p_opt("operation", "Operation", "getRows", &[("Zeilen abfragen (Get Rows)", "getRows"), ("Zeile einfuegen (Insert)", "insert"), ("Upsert", "upsert"), ("Zeile aktualisieren", "update"), ("Zeile loeschen", "delete")], "Aktion auf lokaler Datentabelle"),
        p_str("tableName", "Tabellenname", "Lead-Scoring-Regeln", "Name oder ID der Datentabelle"),
        p_json("rowData", "Zeilendaten (JSON)", "{}", "Spaltenwerte fuer Insert/Upsert"),
        p_str("filterField", "Filter-Spalte", "", "Optionale WHERE-Spalte"),
        p_str("filterValue", "Filter-Wert", "", "Gesuchter Wert"),
    ]
}

fn props_agent() -> Vec<Value> {
    vec![
        p_str("text", "Eingabe-Prompt", "={{ $json.prompt || $json.text || $json.message }}", "Aufgabe oder Nutzeranfrage an den KI-Agenten"),
        p_str("systemMessage", "System-Prompt", "Du bist ein praeziser deutscher Unternehmens-Assistent von Conrad KI.", "Rollenanweisung fuer das Modell"),
        p_num("maxIterations", "Max. Tool-Schritte", 5.0, "Maximale Agenten-Iterationen"),
        p_bool("returnIntermediateSteps", "Zwischenschritte ausgeben", true, "Gedanken & Tool-Aufrufe protokollieren"),
    ]
}

fn props_llm() -> Vec<Value> {
    vec![
        p_str("model", "Modell", "conrad-n8n-agent:0.6b", "Lokales Conrad KI Engine Modell"),
        p_num("temperature", "Temperatur", 0.2, "Kreativitaet (0.0 deterministisch .. 1.0)"),
        p_num("maxTokens", "Max. Tokens", 1024.0, "Maximale Antwortlaenge"),
    ]
}

fn props_sticky() -> Vec<Value> {
    vec![
        p_code("content", "Notiztext (Markdown)", "## Hinweis\nKurze Erklaerung zu diesem Abschnitt.", "Inhalt der Haftnotiz"),
        p_num("color", "Farbe (1-7)", 1.0, "Farbvariante der Haftnotiz"),
        p_num("width", "Breite (px)", 240.0, "Breite auf dem Canvas"),
        p_num("height", "Hoehe (px)", 160.0, "Hoehe auf dem Canvas"),
    ]
}

fn props_email() -> Vec<Value> {
    vec![
        p_str("fromEmail", "Absender", "merlin@conrad-ki.com", "Absenderadresse"),
        p_str("toEmail", "Empfaenger", "={{ $json.email }}", "Ziel-E-Mail-Adresse"),
        p_str("subject", "Betreff", "={{ $json.subject || 'Nachricht von Conrad KI' }}", "Betreffzeile"),
        p_code("text", "Nachricht (Text/HTML)", "Guten Tag,\n\nvielen Dank fuer Ihre Anfrage.", "E-Mail-Inhalt"),
        p_bool("draftOnly", "Immer nur als Entwurf (Sicherheitsregel)", true, "Verhindert automatischen Versand ohne manuelle Freigabe"),
    ]
}

fn props_execute_workflow() -> Vec<Value> {
    vec![
        p_str("workflowId", "Ziel-Workflow ID", "", "ID des aufzurufenden Sub-Workflows"),
        p_opt("mode", "Aufrufmodus", "once", &[("Einmal mit allen Items", "once"), ("Pro Item einzeln", "each")], "Uebergabeverhalten"),
    ]
}

fn props_generic_op() -> Vec<Value> {
    vec![
        p_str("resource", "Ressource", "default", "Zielressource"),
        p_str("operation", "Operation", "get", "Auszufuehrende Aktion"),
        p_json("parametersJson", "Parameter (JSON)", "{}", "Erweiterte Node-Parameter"),
    ]
}

fn specs() -> Vec<NodeSpec> {
    let mut v = vec![
        // Triggers
        NodeSpec { name: "n8n-nodes-base.manualTrigger", display_name: "Manueller Trigger", category: "Trigger", icon: "play", group: &["trigger"], description: "Startet den Workflow manuell im Editor oder per API.", inputs: &[], outputs: &["main"], credential: None, usable_as_tool: false, properties: Vec::new },
        NodeSpec { name: "n8n-nodes-base.webhook", display_name: "Webhook", category: "Trigger", icon: "webhook", group: &["trigger"], description: "Empfaengt eingehende HTTP-Anfragen auf /webhook/* und /webhook-test/*.", inputs: &[], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_webhook },
        NodeSpec { name: "n8n-nodes-base.scheduleTrigger", display_name: "Zeitplan-Trigger (Schedule)", category: "Trigger", icon: "clock", group: &["trigger"], description: "Startet Workflows zyklisch per Cron oder Intervall.", inputs: &[], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_schedule },
        NodeSpec { name: "n8n-nodes-base.formTrigger", display_name: "Formular-Trigger", category: "Trigger", icon: "clipboard", group: &["trigger"], description: "Stellt ein HTML-Formular unter /form/:path bereit und startet bei Absendung.", inputs: &[], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.errorTrigger", display_name: "Fehler-Trigger", category: "Trigger", icon: "alert-triangle", group: &["trigger"], description: "Wird bei Fehlern in verknuepften Workflows ausgeloest.", inputs: &[], outputs: &["main"], credential: None, usable_as_tool: false, properties: Vec::new },
        NodeSpec { name: "n8n-nodes-base.executeWorkflowTrigger", display_name: "Sub-Workflow Trigger", category: "Trigger", icon: "log-in", group: &["trigger"], description: "Eintrittspunkt fuer Aufrufe ueber 'Execute Workflow'.", inputs: &[], outputs: &["main"], credential: None, usable_as_tool: false, properties: Vec::new },
        NodeSpec { name: "@n8n/n8n-nodes-langchain.chatTrigger", display_name: "Chat-Trigger", category: "Trigger", icon: "message-circle", group: &["trigger"], description: "Einstiegspunkt fuer interaktive KI-Chat-Sessions.", inputs: &[], outputs: &["main"], credential: None, usable_as_tool: false, properties: Vec::new },
        NodeSpec { name: "n8n-nodes-base.emailReadImap", display_name: "E-Mail Trigger (IMAP)", category: "Trigger", icon: "mail", group: &["trigger"], description: "Ueberwacht ein IMAP-Postfach auf neue E-Mails.", inputs: &[], outputs: &["main"], credential: Some("imap"), usable_as_tool: false, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.telegramTrigger", display_name: "Telegram Trigger", category: "Trigger", icon: "send", group: &["trigger"], description: "Startet bei neuen Telegram-Bot-Nachrichten.", inputs: &[], outputs: &["main"], credential: Some("telegramApi"), usable_as_tool: false, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.slackTrigger", display_name: "Slack Trigger", category: "Trigger", icon: "hash", group: &["trigger"], description: "Startet bei neuen Slack-Nachrichten oder Erwaehnungen.", inputs: &[], outputs: &["main"], credential: Some("slackApi"), usable_as_tool: false, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.githubTrigger", display_name: "GitHub Trigger", category: "Trigger", icon: "git-branch", group: &["trigger"], description: "Empfaengt Push-, PR- und Issue-Webhooks von GitHub.", inputs: &[], outputs: &["main"], credential: Some("githubApi"), usable_as_tool: false, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.stripeTrigger", display_name: "Stripe Trigger", category: "Trigger", icon: "credit-card", group: &["trigger"], description: "Reagiert auf Stripe-Zahlungs- und Abo-Events.", inputs: &[], outputs: &["main"], credential: Some("stripeApi"), usable_as_tool: false, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.rssFeedReadTrigger", display_name: "RSS-Feed Trigger", category: "Trigger", icon: "rss", group: &["trigger"], description: "Ueberwacht RSS/Atom-Feeds auf neue Eintraege.", inputs: &[], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_generic_op },

        // Kern & Transformation
        NodeSpec { name: "n8n-nodes-base.httpRequest", display_name: "HTTP Request", category: "Kern", icon: "globe", group: &["output"], description: "Sendet HTTP/REST/GraphQL-Anfragen (inkl. Auth, Retry, Expressions & SSRF-Schutz).", inputs: &["main"], outputs: &["main"], credential: Some("httpHeaderAuth"), usable_as_tool: true, properties: props_http_clean },
        NodeSpec { name: "n8n-nodes-base.code", display_name: "Code (QuickJS / Python)", category: "Kern", icon: "code", group: &["transform"], description: "Fuehrt echtes JavaScript (QuickJS mit Luxon & JMESPath) oder Python 3 aus.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: true, properties: props_code },
        NodeSpec { name: "n8n-nodes-base.set", display_name: "Edit Fields (Set)", category: "Datenverarbeitung", icon: "edit-3", group: &["input"], description: "Erstellt, aendert oder filtert JSON-Felder via Assignments & Expressions.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_set },
        NodeSpec { name: "n8n-nodes-base.if", display_name: "IF (Verzweigung)", category: "Datenfluss", icon: "git-branch", group: &["transform"], description: "Teilt Items anhand von Vergleichsregeln auf Ausgang 0 (Wahr) und 1 (Falsch).", inputs: &["main"], outputs: &["main", "main"], credential: None, usable_as_tool: false, properties: props_if },
        NodeSpec { name: "n8n-nodes-base.switch", display_name: "Switch", category: "Datenfluss", icon: "share-2", group: &["transform"], description: "Leitet Items dynamisch anhand von Regeln oder Index in 4+ Ausgaenge.", inputs: &["main"], outputs: &["main", "main", "main", "main"], credential: None, usable_as_tool: false, properties: props_switch },
        NodeSpec { name: "n8n-nodes-base.filter", display_name: "Filter", category: "Datenfluss", icon: "filter", group: &["transform"], description: "Laesst nur Items passieren, die alle Filterregeln erfuellen.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_if },
        NodeSpec { name: "n8n-nodes-base.merge", display_name: "Merge", category: "Datenfluss", icon: "git-merge", group: &["transform"], description: "Fuehrt zwei Eingaenge per Append, Position, SQL-Join oder Multiplex zusammen.", inputs: &["main", "main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_merge },
        NodeSpec { name: "n8n-nodes-base.splitInBatches", display_name: "Loop Over Items (Split in Batches)", category: "Datenfluss", icon: "repeat", group: &["transform"], description: "Verarbeitet grosse Item-Listen schrittweise in Batches (Output 0 = Done, Output 1 = Loop).", inputs: &["main"], outputs: &["main", "main"], credential: None, usable_as_tool: false, properties: props_split_batches },
        NodeSpec { name: "n8n-nodes-base.wait", display_name: "Wait", category: "Datenfluss", icon: "pause-circle", group: &["organization"], description: "Pausiert bis Zeitablauf oder /webhook-waiting/:executionId Aufruf.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_wait },
        NodeSpec { name: "n8n-nodes-base.respondToWebhook", display_name: "Respond to Webhook", category: "Kern", icon: "corner-up-left", group: &["output"], description: "Sendet eine definierte HTTP-Antwort an den aufrufenden Webhook zurueck.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.executeWorkflow", display_name: "Execute Sub-Workflow", category: "Datenfluss", icon: "layers", group: &["transform"], description: "Ruft einen anderen gespeicherten Workflow rekursiv auf.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: true, properties: props_execute_workflow },
        NodeSpec { name: "n8n-nodes-base.stickyNote", display_name: "Haftnotiz (Sticky Note)", category: "Kern", icon: "sticky-note", group: &["organization"], description: "Frei verschiebbare und skalierbare Notiz auf dem Canvas.", inputs: &[], outputs: &[], credential: None, usable_as_tool: false, properties: props_sticky },
        NodeSpec { name: "n8n-nodes-base.noOp", display_name: "No Operation (Durchreichen)", category: "Datenfluss", icon: "arrow-right", group: &["organization"], description: "Reicht eingehende Items unveraendert an den naechsten Node weiter.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: Vec::new },
        NodeSpec { name: "n8n-nodes-base.stopAndError", display_name: "Stop and Error", category: "Datenfluss", icon: "octagon", group: &["organization"], description: "Bricht die Workflow-Ausfuehrung kontrolliert mit einer Fehlermeldung ab.", inputs: &["main"], outputs: &[], credential: None, usable_as_tool: false, properties: props_generic_op },

        // Datenverarbeitung
        NodeSpec { name: "n8n-nodes-base.splitOut", display_name: "Split Out", category: "Datenverarbeitung", icon: "scissors", group: &["transform"], description: "Zerlegt ein Array-Feld in einzelne Items.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_split_out },
        NodeSpec { name: "n8n-nodes-base.aggregate", display_name: "Aggregate", category: "Datenverarbeitung", icon: "package", group: &["transform"], description: "Fasst mehrere Items wieder zu einem einzelnen Item/Array zusammen.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_aggregate },
        NodeSpec { name: "n8n-nodes-base.summarize", display_name: "Summarize", category: "Datenverarbeitung", icon: "bar-chart-2", group: &["transform"], description: "Gruppiert und berechnet Count, Sum, Avg, Min, Max.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: true, properties: props_summarize },
        NodeSpec { name: "n8n-nodes-base.sort", display_name: "Sort", category: "Datenverarbeitung", icon: "arrow-up-down", group: &["transform"], description: "Sortiert Items auf- oder absteigend nach Feldnamen.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_sort },
        NodeSpec { name: "n8n-nodes-base.limit", display_name: "Limit", category: "Datenverarbeitung", icon: "sliders", group: &["transform"], description: "Begrenzt auf die ersten oder letzten N Items.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_limit },
        NodeSpec { name: "n8n-nodes-base.removeDuplicates", display_name: "Remove Duplicates", category: "Datenverarbeitung", icon: "copy", group: &["transform"], description: "Entfernt doppelte Items anhand aller oder ausgewaehlter Felder.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_remove_dupes },
        NodeSpec { name: "n8n-nodes-base.compareDatasets", display_name: "Compare Datasets", category: "Datenverarbeitung", icon: "columns", group: &["transform"], description: "Vergleicht zwei Eingaenge (Nur A / Uebereinstimmend / Unterschiedlich / Nur B).", inputs: &["main", "main"], outputs: &["main", "main", "main", "main"], credential: None, usable_as_tool: false, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.renameKeys", display_name: "Rename Keys", category: "Datenverarbeitung", icon: "type", group: &["transform"], description: "Benennt JSON-Schluessel in allen Items um.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.itemLists", display_name: "Item Lists (Legacy)", category: "Datenverarbeitung", icon: "list", group: &["transform"], description: "Kompatibilitaets-Node fuer Split Out, Sort, Limit und Aggregate.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_split_out },
        NodeSpec { name: "n8n-nodes-base.dateTime", display_name: "Date & Time", category: "Datenverarbeitung", icon: "calendar", group: &["transform"], description: "Formatiert, konvertiert und berechnet Zeitstempel mit Zeitzonen.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: true, properties: props_datetime },
        NodeSpec { name: "n8n-nodes-base.dataTable", display_name: "Datentabelle", category: "Datenverarbeitung", icon: "table", group: &["transform"], description: "Liest, schreibt und sucht in persistenten lokalen SQLite-Datentabellen.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: true, properties: props_data_table },

        // Dateien & Krypto
        NodeSpec { name: "n8n-nodes-base.crypto", display_name: "Crypto", category: "Dateien & Krypto", icon: "lock", group: &["transform"], description: "Berechnet MD5/SHA/HMAC Hashes und generiert sichere Zufallswerte.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: true, properties: props_crypto },
        NodeSpec { name: "n8n-nodes-base.jwt", display_name: "JWT", category: "Dateien & Krypto", icon: "key", group: &["transform"], description: "Signiert, verifiziert und dekodiert JSON Web Tokens (HS256).", inputs: &["main"], outputs: &["main"], credential: Some("jwtAuth"), usable_as_tool: true, properties: props_jwt },
        NodeSpec { name: "n8n-nodes-base.html", display_name: "HTML", category: "Dateien & Krypto", icon: "file-code", group: &["transform"], description: "Extrahiert CSS-Selektoren oder erzeugt HTML-Tabellen und Templates.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.xml", display_name: "XML", category: "Dateien & Krypto", icon: "code", group: &["transform"], description: "Konvertiert zwischen XML und JSON.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.markdown", display_name: "Markdown", category: "Dateien & Krypto", icon: "file-text", group: &["transform"], description: "Wandelt Markdown in HTML und HTML in Markdown um.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.compression", display_name: "Compression", category: "Dateien & Krypto", icon: "archive", group: &["transform"], description: "Komprimiert oder entpackt Daten (Gzip/Deflate/Base64).", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.extractFromFile", display_name: "Extract from File", category: "Dateien & Krypto", icon: "file-plus", group: &["transform"], description: "Parst CSV, JSON, Text oder Base64-Dateien in strukturierte Items.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.convertToFile", display_name: "Convert to File", category: "Dateien & Krypto", icon: "download", group: &["transform"], description: "Exportiert Items als CSV, JSON oder Base64-Binaerdatei.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.rssFeedRead", display_name: "RSS Read", category: "Dateien & Krypto", icon: "rss", group: &["input"], description: "Liest und parst RSS/Atom-XML-Feeds.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: true, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.form", display_name: "Formular", category: "Kern", icon: "check-square", group: &["input"], description: "Zwischenformular oder Bestaetigungsseite im Workflow.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_generic_op },

        // KI & LangChain Cluster
        NodeSpec { name: "@n8n/n8n-nodes-langchain.agent", display_name: "AI Agent", category: "KI & LangChain", icon: "bot", group: &["transform"], description: "Autonomer ReAct/Tool-Calling Agent mit ChatModel, Memory und Sub-Tools.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: true, properties: props_agent },
        NodeSpec { name: "@n8n/n8n-nodes-langchain.chainLlm", display_name: "Basic LLM Chain", category: "KI & LangChain", icon: "message-square", group: &["transform"], description: "Fuehrt strukturierte Prompts direkt ueber ein verknuepftes Sprachmodell aus.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: true, properties: props_agent },
        NodeSpec { name: "@n8n/n8n-nodes-langchain.chainSummarization", display_name: "Summarization Chain", category: "KI & LangChain", icon: "align-left", group: &["transform"], description: "Fasst lange Dokumente oder Item-Listen ueber das LLM zusammen.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: true, properties: props_agent },
        NodeSpec { name: "@n8n/n8n-nodes-langchain.textClassifier", display_name: "Text Classifier", category: "KI & LangChain", icon: "tag", group: &["transform"], description: "Klassifiziert eingehende Texte per LLM in definierte Kategorien.", inputs: &["main"], outputs: &["main", "main"], credential: None, usable_as_tool: false, properties: props_agent },
        NodeSpec { name: "@n8n/n8n-nodes-langchain.sentimentAnalysis", display_name: "Sentiment Analysis", category: "KI & LangChain", icon: "smile", group: &["transform"], description: "Bewertet Stimmungslage (Positiv/Neutral/Negativ) von Kundenanfragen.", inputs: &["main"], outputs: &["main", "main", "main"], credential: None, usable_as_tool: false, properties: props_agent },
        NodeSpec { name: "@n8n/n8n-nodes-langchain.informationExtractor", display_name: "Information Extractor", category: "KI & LangChain", icon: "search", group: &["transform"], description: "Extrahiert strukturierte JSON-Schemata aus unstrukturiertem Text.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: true, properties: props_agent },
        NodeSpec { name: "@n8n/n8n-nodes-langchain.lmChatOllama", display_name: "Ollama Chat Model", category: "KI & LangChain", icon: "cpu", group: &["transform"], description: "Bindet lokales Ollama (z.B. Gemma/Llama/Mistral) datenschutzkonform ein.", inputs: &[], outputs: &["ai_languageModel"], credential: Some("ollamaApi"), usable_as_tool: false, properties: props_llm },
        NodeSpec { name: "@n8n/n8n-nodes-langchain.lmChatOpenAi", display_name: "OpenAI Chat Model", category: "KI & LangChain", icon: "sparkles", group: &["transform"], description: "OpenAI-kompatibles Chat-Modell (GPT / vLLM / OpenRouter).", inputs: &[], outputs: &["ai_languageModel"], credential: Some("openAiApi"), usable_as_tool: false, properties: props_llm },
        NodeSpec { name: "@n8n/n8n-nodes-langchain.lmChatAnthropic", display_name: "Anthropic Chat Model", category: "KI & LangChain", icon: "sparkles", group: &["transform"], description: "Claude Chat-Modell Sub-Node.", inputs: &[], outputs: &["ai_languageModel"], credential: Some("anthropicApi"), usable_as_tool: false, properties: props_llm },
        NodeSpec { name: "@n8n/n8n-nodes-langchain.lmChatGoogleGemini", display_name: "Google Gemini Chat Model", category: "KI & LangChain", icon: "sparkles", group: &["transform"], description: "Gemini Sprachmodell Sub-Node.", inputs: &[], outputs: &["ai_languageModel"], credential: Some("googlePalmApi"), usable_as_tool: false, properties: props_llm },
        NodeSpec { name: "@n8n/n8n-nodes-langchain.memoryBufferWindow", display_name: "Simple Memory", category: "KI & LangChain", icon: "database", group: &["transform"], description: "Speichert Konversationshistorie pro Session-ID.", inputs: &[], outputs: &["ai_memory"], credential: None, usable_as_tool: false, properties: props_generic_op },
        NodeSpec { name: "@n8n/n8n-nodes-langchain.toolHttpRequest", display_name: "HTTP Request Tool", category: "KI & LangChain", icon: "wrench", group: &["transform"], description: "Gibt dem AI Agent gezielten Zugriff auf definierte REST-APIs.", inputs: &[], outputs: &["ai_tool"], credential: None, usable_as_tool: false, properties: props_http_clean },
        NodeSpec { name: "@n8n/n8n-nodes-langchain.toolCode", display_name: "Custom Code Tool", category: "KI & LangChain", icon: "wrench", group: &["transform"], description: "Stellt QuickJS/Python-Code als aufrufbares Agent-Tool bereit.", inputs: &[], outputs: &["ai_tool"], credential: None, usable_as_tool: false, properties: props_code },
        NodeSpec { name: "@n8n/n8n-nodes-langchain.toolWorkflow", display_name: "Sub-Workflow Tool", category: "KI & LangChain", icon: "wrench", group: &["transform"], description: "Fuehrt einen Sub-Workflow als Tool des AI Agent aus.", inputs: &[], outputs: &["ai_tool"], credential: None, usable_as_tool: false, properties: props_execute_workflow },
        NodeSpec { name: "@n8n/n8n-nodes-langchain.toolCalculator", display_name: "Calculator Tool", category: "KI & LangChain", icon: "calculator", group: &["transform"], description: "Deterministischer Rechner als Tool fuer KI-Agenten.", inputs: &[], outputs: &["ai_tool"], credential: None, usable_as_tool: false, properties: Vec::new },
        NodeSpec { name: "@n8n/n8n-nodes-langchain.mcpClientTool", display_name: "MCP Client Tool", category: "KI & LangChain", icon: "plug", group: &["transform"], description: "Verbindet Conrad CRM & externe MCP-Tools direkt mit dem AI Agent.", inputs: &[], outputs: &["ai_tool"], credential: None, usable_as_tool: false, properties: props_generic_op },

        // Integrationen (mit Draft-Schutz fuer E-Mail/Messaging)
        NodeSpec { name: "n8n-nodes-base.emailSend", display_name: "E-Mail senden / Entwurf (SMTP)", category: "Integrationen", icon: "mail", group: &["output"], description: "Erstellt sichere E-Mail-Entwuerfe ueber das Conrad-CRM Entwurfsmodul (kein Blindversand).", inputs: &["main"], outputs: &["main"], credential: Some("smtp"), usable_as_tool: true, properties: props_email },
        NodeSpec { name: "n8n-nodes-base.gmail", display_name: "Gmail", category: "Integrationen", icon: "mail", group: &["output"], description: "Liest E-Mails und speichert Antwort-Entwuerfe in Gmail.", inputs: &["main"], outputs: &["main"], credential: Some("gmailOAuth2"), usable_as_tool: true, properties: props_email },
        NodeSpec { name: "n8n-nodes-base.microsoftOutlook", display_name: "Microsoft Outlook", category: "Integrationen", icon: "mail", group: &["output"], description: "Verwaltet Outlook-Nachrichten und speichert Entwuerfe.", inputs: &["main"], outputs: &["main"], credential: Some("microsoftOutlookOAuth2Api"), usable_as_tool: true, properties: props_email },
        NodeSpec { name: "n8n-nodes-base.slack", display_name: "Slack", category: "Integrationen", icon: "hash", group: &["output"], description: "Slack-Kanaele abfragen und Nachrichten vorbereiten.", inputs: &["main"], outputs: &["main"], credential: Some("slackApi"), usable_as_tool: true, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.telegram", display_name: "Telegram", category: "Integrationen", icon: "send", group: &["output"], description: "Telegram Bot API Integration (mit Draft-Schutz).", inputs: &["main"], outputs: &["main"], credential: Some("telegramApi"), usable_as_tool: true, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.discord", display_name: "Discord", category: "Integrationen", icon: "message-circle", group: &["output"], description: "Discord Bot & Webhook Integration.", inputs: &["main"], outputs: &["main"], credential: Some("discordWebhookApi"), usable_as_tool: true, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.postgres", display_name: "PostgreSQL", category: "Integrationen", icon: "database", group: &["input"], description: "SQL-Abfragen und Synchronisation.", inputs: &["main"], outputs: &["main"], credential: Some("postgres"), usable_as_tool: true, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.supabase", display_name: "Supabase", category: "Integrationen", icon: "database", group: &["input"], description: "Supabase Tabellen, Auth und Edge Integration.", inputs: &["main"], outputs: &["main"], credential: Some("supabaseApi"), usable_as_tool: true, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.googleSheets", display_name: "Google Sheets", category: "Integrationen", icon: "grid", group: &["input"], description: "Tabellenblaetter lesen, anhaengen und aktualisieren.", inputs: &["main"], outputs: &["main"], credential: Some("googleSheetsOAuth2Api"), usable_as_tool: true, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.googleDrive", display_name: "Google Drive", category: "Integrationen", icon: "hard-drive", group: &["input"], description: "Dateien und Ordner in Google Drive verwalten.", inputs: &["main"], outputs: &["main"], credential: Some("googleDriveOAuth2Api"), usable_as_tool: true, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.googleCalendar", display_name: "Google Calendar", category: "Integrationen", icon: "calendar", group: &["input"], description: "Termine und Verfuegbarkeiten lesen und anlegen.", inputs: &["main"], outputs: &["main"], credential: Some("googleCalendarOAuth2Api"), usable_as_tool: true, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.notion", display_name: "Notion", category: "Integrationen", icon: "book-open", group: &["input"], description: "Notion-Datenbanken und Seiten lesen/aktualisieren.", inputs: &["main"], outputs: &["main"], credential: Some("notionApi"), usable_as_tool: true, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.airtable", display_name: "Airtable", category: "Integrationen", icon: "table", group: &["input"], description: "Airtable Basen und Records verwalten.", inputs: &["main"], outputs: &["main"], credential: Some("airtableTokenApi"), usable_as_tool: true, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.hubspot", display_name: "HubSpot", category: "Integrationen", icon: "users", group: &["input"], description: "Kontakte, Firmen und Deals im CRM verwalten.", inputs: &["main"], outputs: &["main"], credential: Some("hubspotApi"), usable_as_tool: true, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.twentyCrm", display_name: "Conrad CRM", category: "Integrationen", icon: "briefcase", group: &["input"], description: "Direkter Zugriff auf lokale Conrad CRM Objekte (Firmen, Personen, Deals, Aufgaben).", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: true, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.github", display_name: "GitHub", category: "Integrationen", icon: "git-branch", group: &["output"], description: "Repositories, Issues und Pull Requests verwalten.", inputs: &["main"], outputs: &["main"], credential: Some("githubApi"), usable_as_tool: true, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.linear", display_name: "Linear", category: "Integrationen", icon: "check-circle", group: &["output"], description: "Linear Issues und Sprints steuern.", inputs: &["main"], outputs: &["main"], credential: Some("linearApi"), usable_as_tool: true, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.stripe", display_name: "Stripe", category: "Integrationen", icon: "credit-card", group: &["output"], description: "Kunden, Rechnungen und Abos abrufen (mit Schreibschutz).", inputs: &["main"], outputs: &["main"], credential: Some("stripeApi"), usable_as_tool: true, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.openai", display_name: "OpenAI", category: "KI & LangChain", icon: "sparkles", group: &["transform"], description: "Chat Completions, Embeddings und strukturierte Extraktion.", inputs: &["main"], outputs: &["main"], credential: Some("openAiApi"), usable_as_tool: true, properties: props_llm },
        NodeSpec { name: "n8n-nodes-base.ollama", display_name: "Ollama (Lokal)", category: "KI & LangChain", icon: "cpu", group: &["transform"], description: "Direkte Inferenz gegen lokales Ollama / Kolibri / Gemma.", inputs: &["main"], outputs: &["main"], credential: Some("ollamaApi"), usable_as_tool: true, properties: props_llm },
        NodeSpec { name: "n8n-nodes-base.executeCommand", display_name: "Execute Command (Eingeschraenkt)", category: "Kern", icon: "terminal", group: &["transform"], description: "Fuehrt freigegebene Systemkommandos kontrolliert aus.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_generic_op },
        NodeSpec { name: "n8n-nodes-base.readWriteFile", display_name: "Read/Write Files from Disk", category: "Dateien & Krypto", icon: "folder", group: &["transform"], description: "Liest und schreibt Dateien innerhalb des Arbeitsverzeichnisses.", inputs: &["main"], outputs: &["main"], credential: None, usable_as_tool: false, properties: props_generic_op },
    ];

    // Synthesize *Tool variants for usable_as_tool nodes (exact n8n convention!)
    let mut tool_variants = Vec::new();
    for s in &v {
        if s.usable_as_tool && !s.name.ends_with("Tool") && !s.name.starts_with("@n8n/n8n-nodes-langchain.tool") {
            let static_name: &'static str = Box::leak(format!("{}Tool", s.name).into_boxed_str());
            let static_disp: &'static str = Box::leak(format!("{} Tool", s.display_name).into_boxed_str());
            let static_desc: &'static str = Box::leak(format!("AI-Tool-Variante: {}", s.description).into_boxed_str());
            tool_variants.push(NodeSpec {
                name: static_name,
                display_name: static_disp,
                category: "KI & LangChain",
                icon: s.icon,
                group: &["transform"],
                description: static_desc,
                inputs: &[],
                outputs: &["ai_tool"],
                credential: s.credential,
                usable_as_tool: false,
                properties: s.properties,
            });
        }
    }
    v.extend(tool_variants);
    v
}

pub fn nodes_json() -> Value {
    let list: Vec<Value> = specs()
        .into_iter()
        .map(|s| {
            let mut obj = json!({
                "name": s.name,
                "displayName": s.display_name,
                "category": s.category,
                "icon": s.icon,
                "group": s.group,
                "version": 1,
                "description": s.description,
                "defaults": { "name": s.display_name },
                "inputs": s.inputs,
                "outputs": s.outputs,
                "usableAsTool": s.usable_as_tool,
                "properties": (s.properties)(),
            });
            if let Some(c) = s.credential {
                obj["credentials"] = json!([{ "name": c, "required": false }]);
            }
            obj
        })
        .collect();
    Value::Array(list)
}

#[allow(dead_code)]
pub fn node_summary(name: &str) -> Option<Value> {
    let all = nodes_json();
    all.as_array()?
        .iter()
        .find(|n| n.get("name").and_then(|v| v.as_str()) == Some(name))
        .cloned()
}

pub struct CredSpec {
    pub name: &'static str,
    pub display_name: &'static str,
    pub fields: &'static [(&'static str, &'static str, bool, &'static str)], // (key, label, is_secret, default)
}

pub fn credential_specs() -> &'static [CredSpec] {
    &[
        CredSpec { name: "httpHeaderAuth", display_name: "Header Auth", fields: &[("name", "Header-Name", false, "Authorization"), ("value", "Header-Wert / Token", true, "")] },
        CredSpec { name: "httpBearerAuth", display_name: "Bearer Auth", fields: &[("token", "Bearer Token", true, "")] },
        CredSpec { name: "httpBasicAuth", display_name: "Basic Auth", fields: &[("user", "Benutzername", false, ""), ("password", "Passwort", true, "")] },
        CredSpec { name: "httpQueryAuth", display_name: "Query Auth", fields: &[("name", "Parameter-Name", false, "api_key"), ("value", "Parameter-Wert", true, "")] },
        CredSpec { name: "httpCustomAuth", display_name: "Custom Auth (JSON)", fields: &[("json", "Auth JSON", true, "{}")] },
        CredSpec { name: "oAuth2Api", display_name: "OAuth2 API", fields: &[("authUrl", "Auth URL", false, ""), ("accessTokenUrl", "Token URL", false, ""), ("clientId", "Client ID", false, ""), ("clientSecret", "Client Secret", true, ""), ("accessToken", "Access Token", true, "")] },
        CredSpec { name: "oAuth1Api", display_name: "OAuth1 API", fields: &[("consumerKey", "Consumer Key", false, ""), ("consumerSecret", "Consumer Secret", true, "")] },
        CredSpec { name: "jwtAuth", display_name: "JWT Auth", fields: &[("keyType", "Schluesseltyp", false, "passphrase"), ("secret", "Secret (HS256)", true, ""), ("privateKey", "Private Key (PEM)", true, "")] },
        CredSpec { name: "openAiApi", display_name: "OpenAI API", fields: &[("apiKey", "API-Schluessel", true, ""), ("url", "Basis-URL", false, "https://api.openai.com/v1")] },
        CredSpec { name: "ollamaApi", display_name: "Ollama (Lokal)", fields: &[("baseUrl", "Ollama URL", false, "http://127.0.0.1:11434"), ("apiKey", "Optionaler API Key", true, "")] },
        CredSpec { name: "anthropicApi", display_name: "Anthropic API", fields: &[("apiKey", "API-Schluessel", true, ""), ("url", "Basis-URL", false, "https://api.anthropic.com")] },
        CredSpec { name: "googlePalmApi", display_name: "Google Gemini API", fields: &[("apiKey", "API-Schluessel", true, ""), ("host", "Host", false, "https://generativelanguage.googleapis.com")] },
        CredSpec { name: "groqApi", display_name: "Groq API", fields: &[("apiKey", "API-Schluessel", true, "")] },
        CredSpec { name: "mistralCloudApi", display_name: "Mistral Cloud API", fields: &[("apiKey", "API-Schluessel", true, "")] },
        CredSpec { name: "openRouterApi", display_name: "OpenRouter API", fields: &[("apiKey", "API-Schluessel", true, "")] },
        CredSpec { name: "smtp", display_name: "SMTP (E-Mail)", fields: &[("user", "Benutzer", false, ""), ("password", "Passwort", true, ""), ("host", "SMTP-Host", false, ""), ("port", "Port", false, "587")] },
        CredSpec { name: "imap", display_name: "IMAP (Postfach)", fields: &[("user", "Benutzer", false, ""), ("password", "Passwort", true, ""), ("host", "IMAP-Host", false, ""), ("port", "Port", false, "993")] },
        CredSpec { name: "gmailOAuth2", display_name: "Gmail OAuth2", fields: &[("clientId", "Client ID", false, ""), ("clientSecret", "Client Secret", true, ""), ("refreshToken", "Refresh Token", true, "")] },
        CredSpec { name: "microsoftOutlookOAuth2Api", display_name: "Microsoft Outlook OAuth2", fields: &[("clientId", "Client ID", false, ""), ("clientSecret", "Client Secret", true, ""), ("refreshToken", "Refresh Token", true, "")] },
        CredSpec { name: "slackApi", display_name: "Slack API", fields: &[("accessToken", "Bot User OAuth Token", true, "")] },
        CredSpec { name: "slackOAuth2Api", display_name: "Slack OAuth2 API", fields: &[("accessToken", "OAuth Access Token", true, "")] },
        CredSpec { name: "telegramApi", display_name: "Telegram Bot API", fields: &[("accessToken", "Bot Token", true, ""), ("baseUrl", "API URL", false, "https://api.telegram.org")] },
        CredSpec { name: "discordWebhookApi", display_name: "Discord Webhook", fields: &[("webhookUri", "Webhook URL", true, "")] },
        CredSpec { name: "discordBotApi", display_name: "Discord Bot API", fields: &[("botToken", "Bot Token", true, "")] },
        CredSpec { name: "githubApi", display_name: "GitHub Personal Access Token", fields: &[("user", "GitHub Benutzer", false, ""), ("accessToken", "Personal Access Token", true, "")] },
        CredSpec { name: "githubOAuth2Api", display_name: "GitHub OAuth2", fields: &[("accessToken", "OAuth Token", true, "")] },
        CredSpec { name: "gitlabApi", display_name: "GitLab API", fields: &[("apiServer", "GitLab URL", false, "https://gitlab.com"), ("accessToken", "Private Token", true, "")] },
        CredSpec { name: "linearApi", display_name: "Linear API", fields: &[("apiKey", "API Key", true, "")] },
        CredSpec { name: "jiraSoftwareCloudApi", display_name: "Jira Cloud API", fields: &[("email", "E-Mail", false, ""), ("apiToken", "API Token", true, ""), ("domain", "Domain", false, "")] },
        CredSpec { name: "notionApi", display_name: "Notion API", fields: &[("apiKey", "Internal Integration Token", true, "")] },
        CredSpec { name: "airtableTokenApi", display_name: "Airtable Personal Token", fields: &[("accessToken", "Personal Access Token", true, "")] },
        CredSpec { name: "googleSheetsOAuth2Api", display_name: "Google Sheets OAuth2", fields: &[("clientId", "Client ID", false, ""), ("clientSecret", "Client Secret", true, ""), ("refreshToken", "Refresh Token", true, "")] },
        CredSpec { name: "googleDriveOAuth2Api", display_name: "Google Drive OAuth2", fields: &[("clientId", "Client ID", false, ""), ("clientSecret", "Client Secret", true, ""), ("refreshToken", "Refresh Token", true, "")] },
        CredSpec { name: "googleCalendarOAuth2Api", display_name: "Google Calendar OAuth2", fields: &[("clientId", "Client ID", false, ""), ("clientSecret", "Client Secret", true, ""), ("refreshToken", "Refresh Token", true, "")] },
        CredSpec { name: "postgres", display_name: "PostgreSQL", fields: &[("host", "Host", false, "127.0.0.1"), ("database", "Datenbank", false, "postgres"), ("user", "Benutzer", false, "postgres"), ("password", "Passwort", true, ""), ("port", "Port", false, "5432")] },
        CredSpec { name: "mySql", display_name: "MySQL / MariaDB", fields: &[("host", "Host", false, "127.0.0.1"), ("database", "Datenbank", false, ""), ("user", "Benutzer", false, "root"), ("password", "Passwort", true, ""), ("port", "Port", false, "3306")] },
        CredSpec { name: "redis", display_name: "Redis", fields: &[("host", "Host", false, "127.0.0.1"), ("port", "Port", false, "6379"), ("password", "Passwort", true, "")] },
        CredSpec { name: "supabaseApi", display_name: "Supabase API", fields: &[("host", "Supabase URL", false, ""), ("serviceRole", "Service Role Secret", true, "")] },
        CredSpec { name: "hubspotApi", display_name: "HubSpot Private App", fields: &[("accessToken", "App Token", true, "")] },
        CredSpec { name: "pipedriveApi", display_name: "Pipedrive API", fields: &[("apiToken", "API Token", true, "")] },
        CredSpec { name: "stripeApi", display_name: "Stripe API", fields: &[("secretKey", "Secret Key", true, "")] },
        CredSpec { name: "brevoApi", display_name: "Brevo (Sendinblue) API", fields: &[("apiKey", "API Key", true, "")] },
        CredSpec { name: "resendApi", display_name: "Resend API", fields: &[("apiKey", "API Key", true, "")] },
        CredSpec { name: "twilioApi", display_name: "Twilio API", fields: &[("accountSid", "Account SID", false, ""), ("authToken", "Auth Token", true, "")] },
        CredSpec { name: "aws", display_name: "AWS IAM", fields: &[("accessKeyId", "Access Key ID", false, ""), ("secretAccessKey", "Secret Access Key", true, ""), ("region", "Region", false, "eu-central-1")] },
        CredSpec { name: "s3", display_name: "S3-kompatibler Speicher", fields: &[("endpoint", "Endpoint URL", false, ""), ("accessKeyId", "Access Key", false, ""), ("secretAccessKey", "Secret Key", true, "")] },
        CredSpec { name: "hetznerCloudApi", display_name: "Hetzner Cloud API", fields: &[("apiToken", "API Token", true, "")] },
    ]
}

pub fn credentials_json() -> Value {
    let arr: Vec<Value> = credential_specs()
        .iter()
        .map(|c| {
            let props: Vec<Value> = c
                .fields
                .iter()
                .map(|(k, lbl, secret, def)| {
                    json!({
                        "displayName": lbl,
                        "name": k,
                        "type": "string",
                        "typeOptions": { "password": secret },
                        "default": def
                    })
                })
                .collect();
            json!({
                "name": c.name,
                "displayName": c.display_name,
                "properties": props
            })
        })
        .collect();
    Value::Array(arr)
}

/// Maskiert geheime Felder mit `BLANK_SENTINEL` bevor Daten an den Client gesendet werden.
pub fn redact_credential_data(cred_type: &str, data: &Value) -> Value {
    let spec = credential_specs().iter().find(|c| c.name == cred_type);
    let Some(obj) = data.as_object() else {
        return json!({});
    };
    let mut masked = serde_json::Map::new();
    for (k, v) in obj {
        let is_secret_by_spec = spec
            .and_then(|s| s.fields.iter().find(|f| f.0 == k.as_str()).map(|f| f.2))
            .unwrap_or(false);
        let lower = k.to_ascii_lowercase();
        let is_secret_by_name = lower.contains("key")
            || lower.contains("secret")
            || lower.contains("token")
            || lower.contains("password")
            || lower.contains("passphrase")
            || lower == "value"
            || lower == "json";
        if (is_secret_by_spec || is_secret_by_name) && !v.as_str().unwrap_or("").is_empty() {
            masked.insert(k.clone(), Value::String(BLANK_SENTINEL.to_string()));
        } else {
            masked.insert(k.clone(), v.clone());
        }
    }
    Value::Object(masked)
}
