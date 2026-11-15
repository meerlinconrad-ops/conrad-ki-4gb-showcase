use axum::{
    body::Bytes,
    extract::State,
    http::{header, HeaderMap, Method, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use rustls::pki_types::ServerName;
use serde_json::{json, Value};
use std::{collections::HashMap, sync::Arc, time::Duration};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::timeout;

use crate::smtp::tls_connector;
use crate::util::load_env_file;

const IMAP_TIMEOUT_SECS: u64 = 15;
const SNIPPET_MAX_CHARS: usize = 500;
const MAX_MIME_DEPTH: usize = 16;

/// Shared state for the `:5005` Pure-Rust IMAP relay.
#[derive(Clone)]
pub struct ImapState {
    pub token: String,
}

impl ImapState {
    /// Loads `IMAP_TOKEN` from environment variables or `/etc/conrad-relays/imap.env`.
    pub fn from_env() -> Self {
        let env_map = load_env_file("/etc/conrad-relays/imap.env");
        let token = std::env::var("IMAP_TOKEN")
            .ok()
            .filter(|s| !s.is_empty())
            .or_else(|| env_map.get("IMAP_TOKEN").filter(|s| !s.is_empty()).cloned())
            .unwrap_or_default();
        if token.is_empty() {
            eprintln!("[imap-relay] WARNING: IMAP_TOKEN not set; all POST requests will return 401");
        }
        Self { token }
    }
}

fn cors_headers() -> HeaderMap {
    let mut h = HeaderMap::new();
    h.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    h.insert(header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, OPTIONS".parse().unwrap());
    h.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        "Content-Type, X-IMAP-Token".parse().unwrap(),
    );
    h
}

fn json_resp(status: StatusCode, body: Value) -> impl IntoResponse {
    (status, cors_headers(), Json(body))
}

fn ct_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.iter().zip(b.iter()) {
        diff |= x ^ y;
    }
    diff == 0
}

/// Resolves upstream IMAP host (`993/TLS`) from the mailbox email address.
pub fn get_imap_server(username: &str) -> &'static str {
    let lower = username.to_ascii_lowercase();
    if lower.contains("@online.de") || lower.contains("@1und1.de") {
        "imap.1und1.de"
    } else if lower.contains("@ionos.") {
        "imap.ionos.de"
    } else {
        "imap.hostinger.com"
    }
}

/// Builds the Axum router for the `:5005` IMAP relay endpoints.
pub fn imap_router(state: Arc<ImapState>) -> Router {
    Router::new()
        .route("/", get(health_handler))
        .route("/healthz", get(health_handler))
        .route("/fetch", post(fetch_handler))
        .route("/move", post(move_handler))
        .route("/mark-read", post(mark_read_handler))
        .fallback(fallback_handler)
        .with_state(state)
}

async fn health_handler(State(state): State<Arc<ImapState>>) -> impl IntoResponse {
    json_resp(
        StatusCode::OK,
        json!({
            "status": "ok",
            "service": "conrad-imap-relay",
            "configured": !state.token.is_empty(),
        }),
    )
}

async fn fallback_handler(method: Method) -> impl IntoResponse {
    if method == Method::OPTIONS {
        return (StatusCode::NO_CONTENT, cors_headers(), Json(json!({})));
    }
    (
        StatusCode::NOT_FOUND,
        cors_headers(),
        Json(json!({"success": false, "error": "Not Found"})),
    )
}

struct ParsedRequest {
    imap_server: &'static str,
    username: String,
    password: String,
    data: Value,
}

fn verify_and_parse(
    state: &ImapState,
    headers: &HeaderMap,
    body: &Bytes,
) -> Result<ParsedRequest, (StatusCode, Value)> {
    let provided = headers
        .get("x-imap-token")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    if state.token.is_empty() || !ct_eq(provided.as_bytes(), state.token.as_bytes()) {
        return Err((
            StatusCode::UNAUTHORIZED,
            json!({"success": false, "error": "Unauthorized"}),
        ));
    }

    let data: Value = if body.is_empty() {
        json!({})
    } else {
        serde_json::from_slice(body).map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                json!({"success": false, "error": format!("Invalid JSON: {e}")}),
            )
        })?
    };

    let username = data
        .get("username")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let password = data
        .get("password")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    if username.is_empty() || password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            json!({"success": false, "error": "Missing credentials (username, password)"}),
        ));
    }

    let imap_server = get_imap_server(&username);
    Ok(ParsedRequest {
        imap_server,
        username,
        password,
        data,
    })
}

async fn fetch_handler(
    State(state): State<Arc<ImapState>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let req = match verify_and_parse(&state, &headers, &body) {
        Ok(r) => r,
        Err((status, val)) => return json_resp(status, val).into_response(),
    };

    let limit = req
        .data
        .get("limit")
        .and_then(|v| v.as_i64().or_else(|| v.as_str().and_then(|s| s.parse().ok())))
        .unwrap_or(10)
        .clamp(0, 200) as usize;

    println!(
        "[imap-relay] Fetching up to {} unseen emails for {}...",
        limit, req.username
    );

    match imap_fetch(req.imap_server, &req.username, &req.password, limit).await {
        Ok(emails) => {
            println!("[imap-relay] Returning {} emails", emails.len());
            json_resp(StatusCode::OK, Value::Array(emails)).into_response()
        }
        Err(e) => {
            eprintln!("[imap-relay] Error fetching: {e}");
            json_resp(
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "error": e}),
            )
            .into_response()
        }
    }
}

async fn move_handler(
    State(state): State<Arc<ImapState>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let req = match verify_and_parse(&state, &headers, &body) {
        Ok(r) => r,
        Err((status, val)) => return json_resp(status, val).into_response(),
    };

    let msg_id = req
        .data
        .get("message_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let target_folder = req
        .data
        .get("target_folder")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    if msg_id.is_empty() || target_folder.is_empty() {
        return json_resp(
            StatusCode::BAD_REQUEST,
            json!({"success": false, "error": "Missing parameters (message_id, target_folder)"}),
        )
        .into_response();
    }

    match imap_move(req.imap_server, &req.username, &req.password, &msg_id, &target_folder).await {
        Ok(Some(moved)) => {
            println!("[imap-relay] Moved {moved} messages to {target_folder}");
            json_resp(StatusCode::OK, json!({"success": true, "moved": moved})).into_response()
        }
        Ok(None) => json_resp(
            StatusCode::NOT_FOUND,
            json!({"success": false, "error": "Message not found in INBOX"}),
        )
        .into_response(),
        Err(e) => {
            eprintln!("[imap-relay] Error moving: {e}");
            json_resp(
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "error": e}),
            )
            .into_response()
        }
    }
}

async fn mark_read_handler(
    State(state): State<Arc<ImapState>>,
    headers: HeaderMap,
    body: Bytes,
) -> impl IntoResponse {
    let req = match verify_and_parse(&state, &headers, &body) {
        Ok(r) => r,
        Err((status, val)) => return json_resp(status, val).into_response(),
    };

    let msg_id = req
        .data
        .get("message_id")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    if msg_id.is_empty() {
        return json_resp(
            StatusCode::BAD_REQUEST,
            json!({"success": false, "error": "Missing parameter: message_id"}),
        )
        .into_response();
    }

    match imap_mark_read(req.imap_server, &req.username, &req.password, &msg_id).await {
        Ok(0) => json_resp(
            StatusCode::OK,
            json!({"success": true, "message": "Already processed or not found"}),
        )
        .into_response(),
        Ok(marked) => {
            println!("[imap-relay] Marked {marked} as read");
            json_resp(StatusCode::OK, json!({"success": true, "marked": marked})).into_response()
        }
        Err(e) => {
            eprintln!("[imap-relay] Error marking read: {e}");
            json_resp(
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"success": false, "error": e}),
            )
            .into_response()
        }
    }
}

// ============================================================================
// Minimal async IMAP4rev1 over TLS (port 993) client
// ============================================================================

struct ImapResp {
    lines: Vec<String>,
    literals: Vec<Vec<u8>>,
}

struct ImapClient<S> {
    reader: BufReader<S>,
    tag_seq: u32,
}

/// Formats an IMAP quoted string, rejecting control characters that could split commands.
pub fn imap_quote(s: &str) -> Result<String, String> {
    if s.chars().any(|c| c == '\r' || c == '\n' || c == '\0') {
        return Err("Invalid control character in IMAP argument".into());
    }
    let mut out = String::with_capacity(s.len() + 4);
    out.push('"');
    for ch in s.chars() {
        if ch == '\\' || ch == '"' {
            out.push('\\');
        }
        out.push(ch);
    }
    out.push('"');
    Ok(out)
}

/// Checks if a trimmed IMAP line ends with `{<digits>}` or `{<digits>+}` literal marker.
pub fn parse_imap_literal_size(line_no_crlf: &str) -> Option<usize> {
    let trimmed = line_no_crlf.trim_end_matches(['\r', '\n']);
    if !trimmed.ends_with('}') {
        return None;
    }
    let open = trimmed.rfind('{')?;
    let inner = &trimmed[open + 1..trimmed.len() - 1];
    let digits = inner.strip_suffix('+').unwrap_or(inner);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    digits.parse::<usize>().ok()
}

impl<S: AsyncRead + AsyncWrite + Unpin> ImapClient<S> {
    async fn connect_handshake(stream: S) -> Result<Self, String> {
        let mut client = Self {
            reader: BufReader::new(stream),
            tag_seq: 0,
        };
        let greeting = client.read_line_lossy().await?;
        let upper = greeting.to_ascii_uppercase();
        if !upper.starts_with("* OK") && !upper.starts_with("* PREAUTH") {
            return Err(format!("Unexpected IMAP greeting: {}", greeting.trim()));
        }
        Ok(client)
    }

    async fn read_line_lossy(&mut self) -> Result<String, String> {
        let mut buf = Vec::with_capacity(256);
        let n = timeout(Duration::from_secs(IMAP_TIMEOUT_SECS), self.reader.read_until(b'\n', &mut buf))
            .await
            .map_err(|_| "IMAP read timeout".to_string())?
            .map_err(|e| format!("IMAP read error: {e}"))?;
        if n == 0 {
            return Err("IMAP server closed connection unexpectedly".into());
        }
        Ok(String::from_utf8_lossy(&buf).into_owned())
    }

    async fn cmd(&mut self, command: &str) -> Result<ImapResp, String> {
        self.tag_seq += 1;
        let tag = format!("A{:04}", self.tag_seq);
        let wire = format!("{tag} {command}\r\n");

        timeout(
            Duration::from_secs(IMAP_TIMEOUT_SECS),
            self.reader.get_mut().write_all(wire.as_bytes()),
        )
        .await
        .map_err(|_| "IMAP write timeout".to_string())?
        .map_err(|e| format!("IMAP write error: {e}"))?;

        timeout(Duration::from_secs(IMAP_TIMEOUT_SECS), self.reader.get_mut().flush())
            .await
            .map_err(|_| "IMAP flush timeout".to_string())?
            .map_err(|e| format!("IMAP flush error: {e}"))?;

        let mut lines = Vec::new();
        let mut literals = Vec::new();
        let tag_prefix = format!("{tag} ");

        loop {
            let line = self.read_line_lossy().await?;
            if let Some(lit_len) = parse_imap_literal_size(&line) {
                if lit_len > 25 * 1024 * 1024 {
                    return Err(format!("IMAP literal too large: {lit_len} bytes"));
                }
                let mut data = vec![0u8; lit_len];
                timeout(Duration::from_secs(IMAP_TIMEOUT_SECS), self.reader.read_exact(&mut data))
                    .await
                    .map_err(|_| "IMAP literal read timeout".to_string())?
                    .map_err(|e| format!("IMAP literal read error: {e}"))?;
                literals.push(data);
                lines.push(line);
                // Consume remainder of the untagged response line up to CRLF (e.g. ")\r\n")
                let rest = self.read_line_lossy().await?;
                lines.push(rest);
                continue;
            }

            if line.starts_with(&tag_prefix) {
                let status_rest = line[tag_prefix.len()..].trim_start();
                let upper = status_rest.to_ascii_uppercase();
                if upper.starts_with("OK") {
                    return Ok(ImapResp { lines, literals });
                }
                return Err(format!("IMAP command failed: {}", line.trim()));
            }

            lines.push(line);
        }
    }

    async fn login(&mut self, user: &str, pass: &str) -> Result<(), String> {
        let u = imap_quote(user)?;
        let p = imap_quote(pass)?;
        self.cmd(&format!("LOGIN {u} {p}")).await.map(|_| ())
    }

    async fn select_inbox(&mut self) -> Result<(), String> {
        self.cmd("SELECT \"INBOX\"").await.map(|_| ())
    }

    async fn search_ids(&mut self, query: &str) -> Result<Vec<String>, String> {
        let resp = self.cmd(&format!("SEARCH {query}")).await?;
        let mut ids = Vec::new();
        for line in resp.lines {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("* SEARCH") {
                for tok in rest.split_whitespace() {
                    if tok.bytes().all(|b| b.is_ascii_digit()) {
                        ids.push(tok.to_string());
                    }
                }
            }
        }
        Ok(ids)
    }

    async fn logout_quiet(mut self) {
        let _ = self.cmd("LOGOUT").await;
    }
}

async fn connect_imap(host: &str) -> Result<ImapClient<tokio_rustls::client::TlsStream<TcpStream>>, String> {
    let tcp = timeout(
        Duration::from_secs(IMAP_TIMEOUT_SECS),
        TcpStream::connect((host, 993)),
    )
    .await
    .map_err(|_| format!("TCP timeout connecting to {host}:993"))?
    .map_err(|e| format!("TCP connect to {host}:993 failed: {e}"))?;

    let server_name = ServerName::try_from(host.to_string())
        .map_err(|e| format!("Invalid IMAP server name '{host}': {e}"))?;

    let tls = timeout(
        Duration::from_secs(IMAP_TIMEOUT_SECS),
        tls_connector().connect(server_name, tcp),
    )
    .await
    .map_err(|_| format!("TLS handshake timeout with {host}:993"))?
    .map_err(|e| format!("TLS handshake failed with {host}:993: {e}"))?;

    ImapClient::connect_handshake(tls).await
}

async fn imap_fetch(
    host: &str,
    user: &str,
    pass: &str,
    limit: usize,
) -> Result<Vec<Value>, String> {
    let mut client = connect_imap(host).await?;
    client.login(user, pass).await?;
    client.select_inbox().await?;

    let ids = client.search_ids("UNSEEN").await?;
    println!("[imap-relay] Total unseen: {}", ids.len());

    let mut emails = Vec::new();
    for seq in ids.into_iter().take(limit) {
        match client.cmd(&format!("FETCH {seq} (RFC822)")).await {
            Ok(resp) => {
                if let Some(raw) = resp.literals.into_iter().next() {
                    let parsed = parse_email_message(&raw);
                    emails.push(json!({
                        "messageId": parsed.message_id,
                        "from": parsed.from,
                        "subject": parsed.subject,
                        "text": parsed.text,
                    }));
                }
            }
            Err(e) => {
                eprintln!("[imap-relay] Error processing msg {seq}: {e}");
            }
        }
    }

    client.logout_quiet().await;
    Ok(emails)
}

async fn imap_move(
    host: &str,
    user: &str,
    pass: &str,
    msg_id: &str,
    target_folder: &str,
) -> Result<Option<usize>, String> {
    let mut client = connect_imap(host).await?;
    client.login(user, pass).await?;
    client.select_inbox().await?;

    let q_id = imap_quote(msg_id)?;
    let ids = client.search_ids(&format!("HEADER Message-ID {q_id}")).await?;
    if ids.is_empty() {
        client.logout_quiet().await;
        return Ok(None);
    }

    let q_folder = imap_quote(target_folder)?;
    let mut moved = 0usize;
    for seq in &ids {
        if client.cmd(&format!("COPY {seq} {q_folder}")).await.is_ok() {
            if client.cmd(&format!("STORE {seq} +FLAGS (\\Deleted)")).await.is_ok() {
                moved += 1;
            }
        } else {
            eprintln!("[imap-relay] Copy failed for {seq} -> {target_folder}");
        }
    }

    let _ = client.cmd("EXPUNGE").await;
    client.logout_quiet().await;
    Ok(Some(moved))
}

async fn imap_mark_read(
    host: &str,
    user: &str,
    pass: &str,
    msg_id: &str,
) -> Result<usize, String> {
    let mut client = connect_imap(host).await?;
    client.login(user, pass).await?;
    client.select_inbox().await?;

    let q_id = imap_quote(msg_id)?;
    let ids = client.search_ids(&format!("HEADER Message-ID {q_id}")).await?;
    if ids.is_empty() {
        client.logout_quiet().await;
        return Ok(0);
    }

    let mut marked = 0usize;
    for seq in &ids {
        if client.cmd(&format!("STORE {seq} +FLAGS (\\Seen)")).await.is_ok() {
            marked += 1;
        }
    }

    client.logout_quiet().await;
    Ok(marked)
}

// ============================================================================
// Lightweight RFC 5322 / MIME / RFC 2047 Email Parser (zero extra crates)
// ============================================================================

#[derive(Debug, PartialEq, Eq)]
pub struct ParsedEmail {
    pub message_id: String,
    pub from: String,
    pub subject: String,
    pub text: String,
}

/// Parses raw RFC 5322/MIME bytes into `messageId`, `from`, `subject`, and 500-char `text` snippet.
pub fn parse_email_message(raw: &[u8]) -> ParsedEmail {
    let (headers, body) = split_headers_body(raw);
    let from = decode_mime_words(headers.get("from").map(String::as_str).unwrap_or(""));
    let subject = decode_mime_words(headers.get("subject").map(String::as_str).unwrap_or(""));
    let message_id = headers
        .get("message-id")
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    let mut plain_parts = Vec::new();
    let mut html_parts = Vec::new();
    collect_text_parts(&headers, body, &mut plain_parts, &mut html_parts, 0);

    let combined = if !plain_parts.is_empty() {
        plain_parts.join("")
    } else {
        strip_html_tags(&html_parts.join(""))
    };

    let normalized = combined
        .trim()
        .replace("\r\n", " ")
        .replace(['\r', '\n'], " ");
    let snippet: String = normalized.chars().take(SNIPPET_MAX_CHARS).collect();

    ParsedEmail {
        message_id,
        from,
        subject,
        text: snippet,
    }
}

fn split_headers_body(raw: &[u8]) -> (HashMap<String, String>, &[u8]) {
    let mut sep_pos = None;
    let mut sep_len = 0;

    let mut i = 0;
    while i < raw.len() {
        if i + 3 < raw.len() && &raw[i..i + 4] == b"\r\n\r\n" {
            sep_pos = Some(i);
            sep_len = 4;
            break;
        }
        if i + 1 < raw.len() && &raw[i..i + 2] == b"\n\n" {
            sep_pos = Some(i);
            sep_len = 2;
            break;
        }
        i += 1;
    }

    let (hdr_bytes, body) = match sep_pos {
        Some(pos) => (&raw[..pos], &raw[pos + sep_len..]),
        None => (raw, &b""[..]),
    };

    let hdr_str = String::from_utf8_lossy(hdr_bytes);
    let mut map: HashMap<String, String> = HashMap::new();
    let mut current_key: Option<String> = None;

    for raw_line in hdr_str.lines() {
        let line = raw_line.trim_end_matches('\r');
        if (line.starts_with(' ') || line.starts_with('\t')) && current_key.is_some() {
            if let Some(k) = &current_key {
                if let Some(val) = map.get_mut(k) {
                    val.push(' ');
                    val.push_str(line.trim());
                }
            }
        } else if let Some((name, val)) = line.split_once(':') {
            let key = name.trim().to_ascii_lowercase();
            let v = val.trim().to_string();
            current_key = Some(key.clone());
            map.entry(key).or_insert(v);
        } else {
            current_key = None;
        }
    }

    (map, body)
}

fn header_param(header_val: &str, param_name: &str) -> Option<String> {
    let target = param_name.to_ascii_lowercase();
    for seg in header_val.split(';').skip(1) {
        if let Some((k, v)) = seg.split_once('=') {
            if k.trim().to_ascii_lowercase() == target {
                let clean = v.trim().trim_matches('"').trim_matches('\'').to_string();
                if !clean.is_empty() {
                    return Some(clean);
                }
            }
        }
    }
    None
}

fn content_type_base(headers: &HashMap<String, String>) -> String {
    headers
        .get("content-type")
        .and_then(|v| v.split(';').next())
        .map(|s| s.trim().to_ascii_lowercase())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "text/plain".to_string())
}

fn collect_text_parts(
    headers: &HashMap<String, String>,
    body: &[u8],
    plain_parts: &mut Vec<String>,
    html_parts: &mut Vec<String>,
    depth: usize,
) {
    if depth >= MAX_MIME_DEPTH {
        return;
    }

    if let Some(disp) = headers.get("content-disposition") {
        if disp.to_ascii_lowercase().contains("attachment") {
            return;
        }
    }

    let ctype_full = headers.get("content-type").map(String::as_str).unwrap_or("");
    let ctype = content_type_base(headers);

    if ctype.starts_with("multipart/") {
        if let Some(boundary) = header_param(ctype_full, "boundary") {
            for part_bytes in split_multipart(body, &boundary) {
                let (p_hdr, p_body) = split_headers_body(part_bytes);
                collect_text_parts(&p_hdr, p_body, plain_parts, html_parts, depth + 1);
            }
            return;
        }
    }

    if ctype != "text/plain" && ctype != "text/html" {
        return;
    }

    let cte = headers
        .get("content-transfer-encoding")
        .map(|s| s.trim().to_ascii_lowercase())
        .unwrap_or_default();

    let decoded = match cte.as_str() {
        "base64" => {
            let compact: Vec<u8> = body.iter().copied().filter(|b| !b.is_ascii_whitespace()).collect();
            B64.decode(&compact).unwrap_or_else(|_| body.to_vec())
        }
        "quoted-printable" => decode_quoted_printable(body),
        _ => body.to_vec(),
    };

    let charset = header_param(ctype_full, "charset").unwrap_or_else(|| "utf-8".into());
    let text = decode_bytes_with_charset(&decoded, &charset);

    if ctype == "text/html" {
        html_parts.push(text);
    } else {
        plain_parts.push(text);
    }
}

fn split_multipart<'a>(body: &'a [u8], boundary: &str) -> Vec<&'a [u8]> {
    let delim = format!("--{boundary}");
    let delim_bytes = delim.as_bytes();
    let mut parts = Vec::new();
    let mut idx = 0;
    let mut part_start: Option<usize> = None;

    while let Some(pos) = find_subslice(&body[idx..], delim_bytes) {
        let abs = idx + pos;
        let after = abs + delim_bytes.len();
        let is_close = body.get(after..after + 2) == Some(b"--");

        if let Some(start) = part_start {
            let mut end = abs;
            if end >= 2 && &body[end - 2..end] == b"\r\n" {
                end -= 2;
            } else if end >= 1 && body[end - 1] == b'\n' {
                end -= 1;
            }
            if end >= start {
                parts.push(&body[start..end]);
            }
        }

        if is_close {
            break;
        }

        // Advance past delimiter + optional trailing spaces + CRLF/LF
        let mut next = after;
        while next < body.len() && body[next] != b'\n' {
            next += 1;
        }
        if next < body.len() && body[next] == b'\n' {
            next += 1;
        }
        part_start = Some(next);
        idx = next;
    }

    parts
}

fn find_subslice(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    if needle.is_empty() || haystack.len() < needle.len() {
        return None;
    }
    haystack.windows(needle.len()).position(|w| w == needle)
}

/// Decodes `Content-Transfer-Encoding: quoted-printable` bytes.
pub fn decode_quoted_printable(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        if input[i] == b'=' {
            if i + 2 < input.len() && input[i + 1] == b'\r' && input[i + 2] == b'\n' {
                i += 3;
                continue;
            }
            if i + 1 < input.len() && input[i + 1] == b'\n' {
                i += 2;
                continue;
            }
            if i + 2 < input.len() {
                if let (Some(hi), Some(lo)) = (hex_val(input[i + 1]), hex_val(input[i + 2])) {
                    out.push((hi << 4) | lo);
                    i += 3;
                    continue;
                }
            }
        }
        out.push(input[i]);
        i += 1;
    }
    out
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

fn decode_bytes_with_charset(bytes: &[u8], charset: &str) -> String {
    let lower = charset.trim().to_ascii_lowercase();
    if lower.is_empty() || lower == "utf-8" || lower == "utf8" || lower == "us-ascii" {
        return String::from_utf8_lossy(bytes).into_owned();
    }
    if lower == "iso-8859-1" || lower == "latin1" || lower == "latin-1" {
        return bytes.iter().map(|&b| b as char).collect();
    }
    if lower == "iso-8859-15" || lower == "latin9" {
        return bytes
            .iter()
            .map(|&b| match b {
                0xA4 => '€',
                0xA6 => 'Š',
                0xA8 => 'š',
                0xB4 => 'Ž',
                0xB8 => 'ž',
                0xBC => 'Œ',
                0xBD => 'œ',
                0xBE => 'Ÿ',
                _ => b as char,
            })
            .collect();
    }
    if lower == "windows-1252" || lower == "cp1252" {
        return bytes.iter().map(|&b| decode_cp1252(b)).collect();
    }
    String::from_utf8_lossy(bytes).into_owned()
}

fn decode_cp1252(b: u8) -> char {
    match b {
        0x80 => '€',
        0x82 => '‚',
        0x83 => 'ƒ',
        0x84 => '„',
        0x85 => '…',
        0x86 => '†',
        0x87 => '‡',
        0x88 => 'ˆ',
        0x89 => '‰',
        0x8A => 'Š',
        0x8B => '‹',
        0x8C => 'Œ',
        0x8E => 'Ž',
        0x91 => '‘',
        0x92 => '’',
        0x93 => '“',
        0x94 => '”',
        0x95 => '•',
        0x96 => '–',
        0x97 => '—',
        0x98 => '˜',
        0x99 => '™',
        0x9A => 'š',
        0x9B => '›',
        0x9C => 'œ',
        0x9E => 'ž',
        0x9F => 'Ÿ',
        _ => b as char,
    }
}

/// Decodes RFC 2047 encoded words (`=?UTF-8?B?...?=` / `=?ISO-8859-1?Q?...?=`).
pub fn decode_mime_words(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }
    let mut out = String::with_capacity(input.len());
    let mut rest = input;
    let mut prev_was_encoded = false;

    while let Some(start) = rest.find("=?") {
        let prefix = &rest[..start];
        if !(prev_was_encoded && prefix.trim().is_empty()) {
            out.push_str(prefix);
        }

        let after_start = &rest[start + 2..];
        let Some(q1) = after_start.find('?') else {
            out.push_str("=?");
            rest = after_start;
            prev_was_encoded = false;
            continue;
        };
        let charset = &after_start[..q1];
        let after_q1 = &after_start[q1 + 1..];
        let Some(q2) = after_q1.find('?') else {
            out.push_str(&rest[start..start + 2 + q1 + 1]);
            rest = after_q1;
            prev_was_encoded = false;
            continue;
        };
        let enc = &after_q1[..q2];
        let after_q2 = &after_q1[q2 + 1..];
        let Some(end) = after_q2.find("?=") else {
            out.push_str(&rest[start..]);
            return out;
        };
        let payload = &after_q2[..end];

        let decoded_bytes = if enc.eq_ignore_ascii_case("B") {
            let compact: Vec<u8> = payload
                .bytes()
                .filter(|b| !b.is_ascii_whitespace())
                .collect();
            B64.decode(&compact).ok()
        } else if enc.eq_ignore_ascii_case("Q") {
            Some(decode_rfc2047_q(payload.as_bytes()))
        } else {
            None
        };

        if let Some(bytes) = decoded_bytes {
            out.push_str(&decode_bytes_with_charset(&bytes, charset));
            prev_was_encoded = true;
        } else {
            out.push_str(&rest[start..start + 2 + q1 + 1 + q2 + 1 + end + 2]);
            prev_was_encoded = false;
        }
        rest = &after_q2[end + 2..];
    }

    out.push_str(rest);
    out
}

fn decode_rfc2047_q(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len());
    let mut i = 0;
    while i < input.len() {
        if input[i] == b'_' {
            out.push(b' ');
            i += 1;
        } else if input[i] == b'=' && i + 2 < input.len() {
            if let (Some(hi), Some(lo)) = (hex_val(input[i + 1]), hex_val(input[i + 2])) {
                out.push((hi << 4) | lo);
                i += 3;
                continue;
            }
            out.push(input[i]);
            i += 1;
        } else {
            out.push(input[i]);
            i += 1;
        }
    }
    out
}

/// Strips `<script>`, `<style>`, HTML tags, and collapses whitespace into clean text.
pub fn strip_html_tags(html: &str) -> String {
    let without_scripts = strip_tag_block(html, "script");
    let without_styles = strip_tag_block(&without_scripts, "style");

    let mut plain = String::with_capacity(without_styles.len());
    let mut in_tag = false;
    for ch in without_styles.chars() {
        match ch {
            '<' => in_tag = true,
            '>' if in_tag => {
                in_tag = false;
                plain.push(' ');
            }
            _ if !in_tag => plain.push(ch),
            _ => {}
        }
    }

    let decoded = decode_basic_entities(&plain);
    decoded.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn strip_tag_block(input: &str, tag: &str) -> String {
    let lower = input.to_ascii_lowercase();
    let open_pat = format!("<{tag}");
    let close_pat = format!("</{tag}>");
    let mut out = String::with_capacity(input.len());
    let mut pos = 0;

    while let Some(rel_start) = lower[pos..].find(&open_pat) {
        let start = pos + rel_start;
        let next_ch = lower.as_bytes().get(start + open_pat.len()).copied();
        let valid_boundary = matches!(next_ch, Some(b'>') | Some(b' ') | Some(b'\t') | Some(b'\n') | Some(b'\r'));
        if !valid_boundary {
            out.push_str(&input[pos..start + open_pat.len()]);
            pos = start + open_pat.len();
            continue;
        }
        out.push_str(&input[pos..start]);
        if let Some(rel_end) = lower[start..].find(&close_pat) {
            out.push(' ');
            pos = start + rel_end + close_pat.len();
        } else {
            // Unclosed tag: drop remaining tail safely
            pos = input.len();
            break;
        }
    }
    if pos < input.len() {
        out.push_str(&input[pos..]);
    }
    out
}

fn decode_basic_entities(s: &str) -> String {
    s.replace("&nbsp;", " ")
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&auml;", "ä")
        .replace("&ouml;", "ö")
        .replace("&uuml;", "ü")
        .replace("&Auml;", "Ä")
        .replace("&Ouml;", "Ö")
        .replace("&Uuml;", "Ü")
        .replace("&szlig;", "ß")
        .replace("&euro;", "€")
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpListener;

    #[test]
    fn test_get_imap_server() {
        assert_eq!(get_imap_server("user@online.de"), "imap.1und1.de");
        assert_eq!(get_imap_server("USER@1UND1.DE"), "imap.1und1.de");
        assert_eq!(get_imap_server("info@ionos.de"), "imap.ionos.de");
        assert_eq!(get_imap_server("hello@conrad-ki.com"), "imap.hostinger.com");
    }

    #[test]
    fn test_decode_mime_words() {
        let b64 = format!("=?UTF-8?B?{}?=", B64.encode("Rückruf gewünscht".as_bytes()));
        assert_eq!(decode_mime_words(&b64), "Rückruf gewünscht");

        let qp = "=?ISO-8859-1?Q?Gr=FC=DFe_aus?= =?UTF-8?Q?_Schwetzingen?=";
        assert_eq!(decode_mime_words(qp), "Grüße aus Schwetzingen");
    }

    #[test]
    fn test_strip_html_tags() {
        let raw = "<html><head><style>body{color:red}</style><script>alert(1)</script></head><body><p>Hallo&nbsp;<b>Welt</b></p></body></html>";
        assert_eq!(strip_html_tags(raw), "Hallo Welt");
    }

    #[test]
    fn test_parse_multipart_email() {
        let raw = concat!(
            "From: =?UTF-8?Q?Max_M=C3=BCller?= <max@example.de>\r\n",
            "Subject: =?UTF-8?B?VGVzdCBCZXRyZWZm?=\r\n",
            "Message-ID: <abc123@example.de>\r\n",
            "Content-Type: multipart/alternative; boundary=\"b1\"\r\n",
            "\r\n",
            "--b1\r\n",
            "Content-Type: text/plain; charset=utf-8\r\n",
            "Content-Transfer-Encoding: quoted-printable\r\n",
            "\r\n",
            "Hallo Welt=2C\r\ndas ist ein Test=21\r\n",
            "--b1\r\n",
            "Content-Type: text/html; charset=utf-8\r\n",
            "\r\n",
            "<p>HTML sollte ignoriert werden wenn text/plain da ist</p>\r\n",
            "--b1\r\n",
            "Content-Type: text/plain; charset=utf-8\r\n",
            "Content-Disposition: attachment; filename=\"note.txt\"\r\n",
            "\r\n",
            "Anhang darf nicht erscheinen\r\n",
            "--b1--\r\n"
        );
        let parsed = parse_email_message(raw.as_bytes());
        assert_eq!(parsed.message_id, "<abc123@example.de>");
        assert_eq!(parsed.from, "Max Müller <max@example.de>");
        assert_eq!(parsed.subject, "Test Betreff");
        assert_eq!(parsed.text, "Hallo Welt, das ist ein Test!");
    }

    #[test]
    fn test_parse_imap_literal_and_quote() {
        assert_eq!(parse_imap_literal_size("* 12 FETCH (RFC822 {4812}\r\n"), Some(4812));
        assert_eq!(parse_imap_literal_size("* OK ready\r\n"), None);
        assert_eq!(imap_quote(r#"a"b\c"#).unwrap(), r#""a\"b\\c""#);
        assert!(imap_quote("bad\nline").is_err());
    }

    #[tokio::test]
    async fn test_imap_http_auth_and_validation() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let state = Arc::new(ImapState {
            token: "test-secret-token".into(),
        });
        let router = imap_router(state);
        tokio::spawn(async move {
            let _ = axum::serve(listener, router).await;
        });

        let client = reqwest::Client::new();
        let base = format!("http://{addr}");

        let health = client.get(format!("{base}/")).send().await.unwrap();
        assert_eq!(health.status(), reqwest::StatusCode::OK);

        let unauth = client
            .post(format!("{base}/fetch"))
            .json(&json!({}))
            .send()
            .await
            .unwrap();
        assert_eq!(unauth.status(), reqwest::StatusCode::UNAUTHORIZED);

        let missing_creds = client
            .post(format!("{base}/fetch"))
            .header("X-IMAP-Token", "test-secret-token")
            .json(&json!({"limit": 5}))
            .send()
            .await
            .unwrap();
        assert_eq!(missing_creds.status(), reqwest::StatusCode::BAD_REQUEST);

        let missing_move = client
            .post(format!("{base}/move"))
            .header("X-IMAP-Token", "test-secret-token")
            .json(&json!({"username": "u@ionos.de", "password": "pw"}))
            .send()
            .await
            .unwrap();
        assert_eq!(missing_move.status(), reqwest::StatusCode::BAD_REQUEST);
    }
}
