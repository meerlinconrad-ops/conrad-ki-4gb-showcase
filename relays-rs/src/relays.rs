use axum::{
    body::Bytes,
    extract::State,
    http::{header, HeaderMap, Method, StatusCode, Uri},
    response::IntoResponse,
    routing::any,
    Json, Router,
};
use reqwest::Client;
use serde_json::{json, Value};
use std::{collections::HashMap, sync::Arc};

use crate::smtp::{send_smtp_starttls, EmailPayload, SmtpConfig};
use crate::util::{cal_field_str, escape_html, format_cal_window, load_env_file, now_ddmmyyyy_hhmm};

fn cors_headers() -> HeaderMap {
    let mut h = HeaderMap::new();
    h.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    h.insert(header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, OPTIONS".parse().unwrap());
    h.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, "Content-Type".parse().unwrap());
    h
}

fn json_resp(status: StatusCode, body: Value) -> impl IntoResponse {
    (status, cors_headers(), Json(body))
}

fn env_from(map: &HashMap<String, String>, key: &str, default: &str) -> String {
    map.get(key)
        .filter(|s| !s.is_empty())
        .cloned()
        .or_else(|| std::env::var(key).ok().filter(|s| !s.is_empty()))
        .unwrap_or_else(|| default.to_string())
}

// ============================================================================
// 1. CONTACT RELAY (:3010)
// ============================================================================

#[derive(Clone)]
pub struct ContactState {
    pub smtp: SmtpConfig,
    pub to_email: String,
    pub n8n_internal: String,
    pub http: Client,
}

impl ContactState {
    pub fn from_env(http: Client) -> Self {
        let m = load_env_file("/etc/conrad-relays/contact.env");
        Self {
            smtp: SmtpConfig {
                host: env_from(&m, "SMTP_HOST", "smtp.hostinger.com"),
                port: env_from(&m, "SMTP_PORT", "587").parse().unwrap_or(587),
                user: env_from(&m, "SMTP_USER", "info@conrad-ki.com"),
                pass: env_from(&m, "SMTP_PASS", ""),
                timeout_secs: 15,
            },
            to_email: env_from(&m, "TO_EMAIL", "info@conrad-ki.com"),
            n8n_internal: env_from(&m, "N8N_INTERNAL", "https://n8n.conrad-ki.com"),
            http,
        }
    }
}

pub fn contact_router(state: Arc<ContactState>) -> Router {
    Router::new()
        .route("/", any(handle_contact))
        .route("/*rest", any(handle_contact))
        .with_state(state)
}

async fn handle_contact(
    State(st): State<Arc<ContactState>>,
    method: Method,
    uri: Uri,
    body: Bytes,
) -> impl IntoResponse {
    if method == Method::OPTIONS {
        return (StatusCode::OK, cors_headers(), String::new()).into_response();
    }
    if method == Method::GET {
        return json_resp(StatusCode::OK, json!({"ok": true, "service": "conrad-contact-relay"})).into_response();
    }
    if method != Method::POST {
        return json_resp(StatusCode::METHOD_NOT_ALLOWED, json!({"error": "Method not allowed"})).into_response();
    }

    let path = uri.path();
    let forward_paths = [
        "/webhook/booking_b-booking",
        "/webhook/calcom-booking_b-booking",
        "/webhook/booking_b-payment",
    ];

    if path == "/send-html" {
        let Ok(data) = serde_json::from_slice::<Value>(&body) else {
            return json_resp(StatusCode::BAD_REQUEST, json!({"error": "Invalid JSON"})).into_response();
        };
        let to_email = data.get("to").and_then(|v| v.as_str()).filter(|s| !s.is_empty()).unwrap_or(&st.to_email);
        let subject = data.get("subject").and_then(|v| v.as_str()).unwrap_or("Kein Betreff");
        let html = data.get("html").and_then(|v| v.as_str()).filter(|s| !s.is_empty());
        let text = data.get("text").and_then(|v| v.as_str()).filter(|s| !s.is_empty());
        let from_email = data.get("from").and_then(|v| v.as_str()).filter(|s| !s.is_empty()).unwrap_or(&st.smtp.user);
        let from_name = data.get("from_name").and_then(|v| v.as_str()).unwrap_or("Conrad KI");
        let from_hdr = format!("{from_name} <{from_email}>");

        return match send_smtp_starttls(
            &st.smtp,
            EmailPayload {
                from_header: &from_hdr,
                envelope_from: &st.smtp.user,
                to_header: to_email,
                envelope_to: to_email,
                subject,
                text,
                html,
                x_mailer: Some("Conrad-Relays-Rust"),
            },
        )
        .await
        {
            Ok(()) => {
                println!("[CONTACT-RELAY] HTML mail sent to {to_email}: {subject}");
                json_resp(StatusCode::OK, json!({"success": true, "sent_to": to_email})).into_response()
            }
            Err(e) => {
                eprintln!("[CONTACT-RELAY] HTML mail failed: {e}");
                json_resp(StatusCode::INTERNAL_SERVER_ERROR, json!({"success": false, "error": e})).into_response()
            }
        };
    }

    if forward_paths.contains(&path) {
        let target = format!("{}{}", st.n8n_internal.trim_end_matches('/'), path);
        match st
            .http
            .post(&target)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::USER_AGENT, "conrad-relays-rust")
            .body(body.to_vec())
            .send()
            .await
        {
            Ok(resp) => {
                let code = StatusCode::from_u16(resp.status().as_u16()).unwrap_or(StatusCode::OK);
                let val = resp.json::<Value>().await.unwrap_or_else(|_| json!({"success": true}));
                println!("[CONTACT-RELAY] Forward {path} -> n8n (HTTP {})", code.as_u16());
                return json_resp(code, val).into_response();
            }
            Err(e) => {
                eprintln!("[CONTACT-RELAY] Forward failed for {path}: {e}");
                return json_resp(StatusCode::OK, json!({"success": true, "note": "relay-forward-failed"})).into_response();
            }
        }
    }

    let Ok(data) = serde_json::from_slice::<Value>(&body) else {
        return json_resp(StatusCode::BAD_REQUEST, json!({"error": "Invalid JSON"})).into_response();
    };
    let name = data.get("name").and_then(|v| v.as_str()).unwrap_or("?");
    let email = data.get("email").and_then(|v| v.as_str()).unwrap_or("?");
    let telefon = data.get("telefon").and_then(|v| v.as_str()).unwrap_or("");
    let nachricht = data.get("nachricht").and_then(|v| v.as_str()).unwrap_or("?");
    let source = data.get("source").and_then(|v| v.as_str()).unwrap_or("conrad-ki.com");
    let text = format!(
        "Neue Kontaktformular-Anfrage\n\nName: {name}\nE-Mail: {email}\nTelefon: {telefon}\nNachricht: {nachricht}\nQuelle: {source}"
    );
    let subject = format!("Kontaktformular: {name}");

    match send_smtp_starttls(
        &st.smtp,
        EmailPayload {
            from_header: &st.smtp.user,
            envelope_from: &st.smtp.user,
            to_header: &st.to_email,
            envelope_to: &st.to_email,
            subject: &subject,
            text: Some(&text),
            html: None,
            x_mailer: Some("Conrad-Relays-Rust"),
        },
    )
    .await
    {
        Ok(()) => json_resp(StatusCode::OK, json!({"success": true})).into_response(),
        Err(e) => json_resp(StatusCode::INTERNAL_SERVER_ERROR, json!({"success": false, "error": e})).into_response(),
    }
}

// ============================================================================
// 2. LEAD WEBHOOK (:8080)
// ============================================================================

#[derive(Clone)]
pub struct LeadState {
    pub smtp: SmtpConfig,
    pub from_email: String,
    pub to_email: String,
}

impl LeadState {
    pub fn from_env() -> Self {
        let m = load_env_file("/etc/conrad-relays/lead.env");
        Self {
            smtp: SmtpConfig {
                host: env_from(&m, "SMTP_HOST", "smtp-relay.brevo.com"),
                port: env_from(&m, "SMTP_PORT", "587").parse().unwrap_or(587),
                user: env_from(&m, "SMTP_USER", "YOUR_SMTP_USER"),
                pass: env_from(&m, "SMTP_PASS", ""),
                timeout_secs: 15,
            },
            from_email: env_from(&m, "FROM_EMAIL", "info@conrad-ki.com"),
            to_email: env_from(&m, "TO_EMAIL", "info@conrad-ki.com"),
        }
    }
}

pub fn lead_router(state: Arc<LeadState>) -> Router {
    Router::new()
        .route("/", any(handle_lead))
        .route("/*rest", any(handle_lead))
        .with_state(state)
}

async fn handle_lead(State(st): State<Arc<LeadState>>, method: Method, body: Bytes) -> impl IntoResponse {
    if method == Method::GET {
        return (StatusCode::OK, "ok").into_response();
    }
    if method != Method::POST {
        return json_resp(StatusCode::METHOD_NOT_ALLOWED, json!({"error": "Method not allowed"})).into_response();
    }

    let raw_str = String::from_utf8_lossy(&body);
    let data: Value = serde_json::from_str(&raw_str).unwrap_or_else(|_| json!({"raw": raw_str}));
    let now = now_ddmmyyyy_hhmm();
    let get_field = |k: &str, default: &str| {
        data.get(k)
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .unwrap_or(default)
            .to_string()
    };

    let name = get_field("name", "Unbekannt");
    let email = get_field("email", "-");
    let phone = get_field("phone", "-");
    let address = get_field("address", "-");
    let roof = get_field("roof", "-");
    let area = get_field("area", "-");

    let html = format!(
        "<h2>Neue DachCheck-Anfrage</h2><table border=1 cellpadding=8>\
         <tr><td>Datum</td><td>{}</td></tr>\
         <tr><td>Name</td><td>{}</td></tr>\
         <tr><td>Email</td><td>{}</td></tr>\
         <tr><td>Telefon</td><td>{}</td></tr>\
         <tr><td>Adresse</td><td>{}</td></tr>\
         <tr><td>Dachart</td><td>{}</td></tr>\
         <tr><td>Flaeche</td><td>{}</td></tr></table>",
        escape_html(&now),
        escape_html(&name),
        escape_html(&email),
        escape_html(&phone),
        escape_html(&address),
        escape_html(&roof),
        escape_html(&area)
    );
    let text = format!("Neue Anfrage von {name}");
    let subject = format!("Neue Solar-Anfrage von {name}");

    match send_smtp_starttls(
        &st.smtp,
        EmailPayload {
            from_header: &st.from_email,
            envelope_from: &st.from_email,
            to_header: &st.to_email,
            envelope_to: &st.to_email,
            subject: &subject,
            text: Some(&text),
            html: Some(&html),
            x_mailer: Some("Conrad-Relays-Rust"),
        },
    )
    .await
    {
        Ok(()) => {
            println!("[{now}] [LEAD-WEBHOOK] Email sent for {name}");
            json_resp(StatusCode::OK, json!({"success": true})).into_response()
        }
        Err(e) => {
            eprintln!("[{now}] [LEAD-WEBHOOK] ERROR: {e}");
            json_resp(StatusCode::INTERNAL_SERVER_ERROR, json!({"error": e})).into_response()
        }
    }
}

// ============================================================================
// 3. BOOKING_A BOOKING RELAY (:3051)
// ============================================================================

#[derive(Clone)]
pub struct BookingAState {
    pub smtp: SmtpConfig,
    pub from_email: String,
    pub to_email: String,
}

impl BookingAState {
    pub fn from_env() -> Self {
        let m = load_env_file("/etc/conrad-relays/booking_a.env");
        Self {
            smtp: SmtpConfig {
                host: env_from(&m, "BREVO_HOST", "smtp-relay.brevo.com"),
                port: env_from(&m, "BREVO_PORT", "587").parse().unwrap_or(587),
                user: env_from(&m, "BREVO_USER", "YOUR_SMTP_USER"),
                pass: env_from(&m, "BREVO_PASS", ""),
                timeout_secs: 15,
            },
            from_email: env_from(&m, "FROM_EMAIL", "buchung@example.de"),
            to_email: env_from(&m, "TO_EMAIL", "info@example.de"),
        }
    }
}

pub fn booking_a_router(state: Arc<BookingAState>) -> Router {
    Router::new()
        .route("/", any(handle_booking_a))
        .route("/*rest", any(handle_booking_a))
        .with_state(state)
}

pub fn build_booking_a_emails(booking: &Value) -> (String, String, String, String, String) {
    let title = booking.get("title").and_then(|v| v.as_str()).unwrap_or("Neue Buchung");
    let start = booking.get("startTime").and_then(|v| v.as_str()).unwrap_or("");
    let end = booking.get("endTime").and_then(|v| v.as_str()).unwrap_or("");
    let attendee = booking
        .get("attendees")
        .and_then(|v| v.as_array())
        .and_then(|a| a.first());
    let name = attendee.and_then(|a| a.get("name")).and_then(|v| v.as_str()).unwrap_or("Unbekannt");
    let email = attendee.and_then(|a| a.get("email")).and_then(|v| v.as_str()).unwrap_or("");
    let guests: Vec<&str> = booking
        .get("guests")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|x| x.as_str()).collect())
        .unwrap_or_default();

    let (datum, uhrzeit) = format_cal_window(start, end, true);
    let guests_html = if guests.is_empty() {
        String::new()
    } else {
        format!(
            "<tr><td style=\"padding:8px 0;color:#475569\">Gäste</td><td style=\"padding:8px 0;color:#0F172A\">{}</td></tr>",
            escape_html(&guests.join(", "))
        )
    };

    let owner_html = format!(
        "<!DOCTYPE html><html lang=\"de\"><head><meta charset=\"utf-8\"><meta name=\"color-scheme\" content=\"light dark\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"></head>\
         <body style=\"margin:0;padding:0;background:#F8FAFC;font-family:'Inter','Helvetica Neue',Arial,sans-serif\">\
         <table width=\"100%\" cellpadding=\"0\" cellspacing=\"0\" style=\"background:#F8FAFC\"><tr><td align=\"center\" style=\"padding:30px 16px\">\
         <table width=\"100%\" cellpadding=\"0\" cellspacing=\"0\" style=\"max-width:520px;background:#ffffff;border-radius:20px;overflow:hidden;box-shadow:0 8px 32px rgba(31,38,135,0.06)\">\
         <tr><td style=\"background:#009E30;padding:24px;text-align:center\"><h1 style=\"margin:0;font-family:'Outfit','Helvetica Neue',Arial,sans-serif;font-size:22px;color:#ffffff;font-weight:700\">📅 Neue Terminbuchung</h1></td></tr>\
         <tr><td style=\"padding:28px 24px\"><p style=\"margin:0 0 20px 0;font-size:15px;color:#0F172A;line-height:1.5\">Guten Tag,<br><br>ein Kunde hat soeben einen Termin über die Website gebucht:</p>\
         <table width=\"100%\" cellpadding=\"0\" cellspacing=\"0\" style=\"background:#F8FAFC;border-radius:12px;padding:16px;border:1px solid rgba(0,158,48,0.15)\">\
         <tr><td style=\"padding:8px 0;color:#475569;font-size:13px;width:90px\">Kunde</td><td style=\"padding:8px 0;color:#0F172A;font-weight:600\">{}</td></tr>\
         <tr><td style=\"padding:8px 0;color:#475569;font-size:13px\">E-Mail</td><td style=\"padding:8px 0;color:#0F172A\"><a href=\"mailto:{}\" style=\"color:#009E30;text-decoration:none\">{}</a></td></tr>\
         <tr><td style=\"padding:8px 0;color:#475569;font-size:13px\">Termin</td><td style=\"padding:8px 0;color:#0F172A;font-weight:600\">{}</td></tr>\
         <tr><td style=\"padding:8px 0;color:#475569;font-size:13px\">Datum</td><td style=\"padding:8px 0;color:#0F172A;font-weight:600\">{}</td></tr>\
         <tr><td style=\"padding:8px 0;color:#475569;font-size:13px\">Uhrzeit</td><td style=\"padding:8px 0;color:#0F172A;font-weight:600\">{}</td></tr>\
         {guests_html}</table>\
         <table width=\"100%\" cellpadding=\"0\" cellspacing=\"0\" style=\"margin-top:20px\"><tr><td align=\"center\">\
         <a href=\"https://buchung.conrad-ki.com/bookings\" style=\"display:inline-block;background:#009E30;color:#ffffff;font-size:14px;font-weight:600;text-decoration:none;padding:12px 32px;border-radius:10px\">Im Dashboard ansehen →</a>\
         </td></tr></table></td></tr>\
         <tr><td style=\"border-top:1px solid rgba(0,158,48,0.15);padding:16px 24px;text-align:center\"><p style=\"margin:0;font-size:12px;color:#475569\">Diese automatische Benachrichtigung wurde von Ihrem Buchungssystem gesendet.<br><strong>Musterbetrieb GmbH</strong> · Musterstraße 1 · 12345 Musterstadt<br><a href=\"https://example.de\" style=\"color:#009E30;text-decoration:none\">www.example.de</a></p></td></tr>\
         </table></td></tr></table></body></html>",
        escape_html(name),
        escape_html(email),
        escape_html(email),
        escape_html(title),
        escape_html(&datum),
        escape_html(&uhrzeit)
    );

    let owner_plain = format!(
        "Neue Terminbuchung – Musterbetrieb GmbH\n\nEin Kunde hat soeben einen Termin gebucht:\n\nKunde:  {name}\nE-Mail: {email}\nTermin: {title}\nDatum:  {datum}\nUhrzeit:{uhrzeit}\n\nIm Dashboard: https://buchung.conrad-ki.com/bookings\n\n---\nMusterbetrieb GmbH · Musterstraße 1 · 12345 Musterstadt\nDiese E-Mail wurde automatisch von Ihrem Buchungssystem gesendet."
    );

    let first_name = name.split_whitespace().next().unwrap_or("");
    let greeting = if first_name.is_empty() || first_name == "Unbekannt" {
        String::new()
    } else {
        format!(", {first_name}")
    };
    let cust_title = booking.get("title").and_then(|v| v.as_str()).unwrap_or("Ihr Termin");

    let customer_html = format!(
        "<!DOCTYPE html><html lang=\"de\"><head><meta charset=\"utf-8\"><meta name=\"color-scheme\" content=\"light dark\"></head>\
         <body style=\"margin:0;padding:0;background:#F8FAFC;font-family:'Inter','Helvetica Neue',Arial,sans-serif\">\
         <table width=\"100%\" cellpadding=\"0\" cellspacing=\"0\"><tr><td align=\"center\" style=\"padding:30px 16px\">\
         <table width=\"100%\" cellpadding=\"0\" cellspacing=\"0\" style=\"max-width:520px;background:#ffffff;border-radius:20px;overflow:hidden;box-shadow:0 8px 32px rgba(31,38,135,0.06)\">\
         <tr><td style=\"background:#009E30;padding:24px;text-align:center\"><h1 style=\"margin:0;font-family:'Outfit',sans-serif;font-size:22px;color:#ffffff\">Vielen Dank{}!</h1></td></tr>\
         <tr><td style=\"padding:28px 24px\"><p style=\"font-size:15px;color:#0F172A;line-height:1.6;margin:0 0 16px 0\">Ihre Terminanfrage für <strong>{}</strong> ist bei uns eingegangen.</p>\
         <p style=\"font-size:14px;color:#475569;line-height:1.6;margin:0 0 16px 0\">Wir prüfen Ihre Anfrage und melden uns innerhalb der nächsten Stunden bei Ihnen.</p>\
         <p style=\"font-size:14px;color:#475569;line-height:1.6;margin:0\">Bei Fragen erreichen Sie uns unter <a href=\"tel:+491234567890\" style=\"color:#009E30\">01234 567890</a>.</p></td></tr>\
         <tr><td style=\"border-top:1px solid rgba(0,158,48,0.15);padding:16px 24px;text-align:center\"><p style=\"margin:0;font-size:11px;color:#94a3b8\"><strong>Musterbetrieb GmbH</strong> · Musterstraße 1 · 12345 Musterstadt<br><a href=\"https://example.de\" style=\"color:#009E30\">www.example.de</a></p></td></tr>\
         </table></td></tr></table></body></html>",
        escape_html(&greeting),
        escape_html(cust_title)
    );

    let customer_plain = format!(
        "Vielen Dank{greeting}!\n\nIhre Terminanfrage für \"{cust_title}\" ist bei uns eingegangen.\nWir prüfen Ihre Anfrage und melden uns innerhalb der nächsten Stunden.\n\nBei Fragen erreichen Sie uns unter 01234 567890.\n\nMusterbetrieb GmbH\nMusterstraße 1 · 12345 Musterstadt\nwww.example.de"
    );

    (email.to_string(), owner_plain, owner_html, customer_plain, customer_html)
}

async fn handle_booking_a(State(st): State<Arc<BookingAState>>, method: Method, body: Bytes) -> impl IntoResponse {
    if method == Method::GET {
        return json_resp(StatusCode::OK, json!({"ok": true, "service": "booking_a-booking-relay"})).into_response();
    }
    if method != Method::POST {
        return json_resp(StatusCode::METHOD_NOT_ALLOWED, json!({"error": "Method not allowed"})).into_response();
    }

    let Ok(data) = serde_json::from_slice::<Value>(&body) else {
        return json_resp(StatusCode::BAD_REQUEST, json!({"error": "Invalid JSON"})).into_response();
    };
    let booking = data.get("payload").unwrap_or(&data);
    if booking.is_null() || booking.as_object().is_some_and(|m| m.is_empty()) {
        return json_resp(StatusCode::OK, json!({"ok": false, "reason": "no payload"})).into_response();
    }

    let (customer_email, owner_plain, owner_html, cust_plain, cust_html) = build_booking_a_emails(booking);
    let owner_fut = send_smtp_starttls(
        &st.smtp,
        EmailPayload {
            from_header: &st.from_email,
            envelope_from: &st.from_email,
            to_header: &st.to_email,
            envelope_to: &st.to_email,
            subject: "📅 Neue Terminbuchung – Musterbetrieb GmbH",
            text: Some(&owner_plain),
            html: Some(&owner_html),
            x_mailer: Some("BookingA-Booking-Relay-Rust"),
        },
    );

    let (ok1, ok2) = if !customer_email.is_empty() {
        let cust_fut = send_smtp_starttls(
            &st.smtp,
            EmailPayload {
                from_header: &st.from_email,
                envelope_from: &st.from_email,
                to_header: &customer_email,
                envelope_to: &customer_email,
                subject: "📅 Ihre Terminanfrage – Musterbetrieb GmbH",
                text: Some(&cust_plain),
                html: Some(&cust_html),
                x_mailer: Some("BookingA-Booking-Relay-Rust"),
            },
        );
        let (r1, r2) = tokio::join!(owner_fut, cust_fut);
        if let Err(ref e) = r1 { eprintln!("[BOOKING_A-RELAY] Owner mail error: {e}"); }
        if let Err(ref e) = r2 { eprintln!("[BOOKING_A-RELAY] Customer mail error: {e}"); }
        (r1.is_ok(), r2.is_ok())
    } else {
        let r1 = owner_fut.await;
        if let Err(ref e) = r1 { eprintln!("[BOOKING_A-RELAY] Owner mail error: {e}"); }
        (r1.is_ok(), false)
    };

    let success = ok1 || ok2;
    let status = if success { StatusCode::OK } else { StatusCode::INTERNAL_SERVER_ERROR };
    json_resp(status, json!({"ok": success})).into_response()
}

// ============================================================================
// 4. BOOKING_B BOOKING RELAY (:3050)
// ============================================================================

#[derive(Clone)]
pub struct BookingBState {
    pub telegram_token: String,
    pub telegram_chat_id: String,
    pub whatsapp_url: String,
    pub whatsapp_chat_id: String,
    pub email_to: String,
    pub email_from: String,
    pub smtp: SmtpConfig,
    pub http: Client,
}

impl BookingBState {
    pub fn from_env(http: Client) -> Self {
        let m = load_env_file("/etc/conrad-relays/booking_b.env");
        let token_file = env_from(&m, "BOOKING_B_TOKEN_FILE", "");
        let telegram_token = m
            .get("TELEGRAM_BOT_TOKEN")
            .filter(|s| !s.is_empty())
            .cloned()
            .or_else(|| std::env::var("TELEGRAM_BOT_TOKEN").ok().filter(|s| !s.is_empty()))
            .or_else(|| {
                if token_file.is_empty() {
                    None
                } else {
                    std::fs::read_to_string(&token_file).ok().map(|s| s.trim().to_string())
                }
            })
            .unwrap_or_default();

        let brevo_file = env_from(&m, "BOOKING_B_BREVO_ENV", "");
        let brevo_map: HashMap<String, String> = if brevo_file.is_empty() {
            HashMap::new()
        } else {
            load_env_file(&brevo_file)
        };

        let smtp_user = brevo_map
            .get("BREVO_SMTP_LOGIN")
            .cloned()
            .or_else(|| std::env::var("BREVO_SMTP_LOGIN").ok())
            .unwrap_or_default();
        let smtp_pass = brevo_map
            .get("BREVO_SMTP_PASSWORD")
            .cloned()
            .or_else(|| std::env::var("BREVO_SMTP_PASSWORD").ok())
            .unwrap_or_default();

        Self {
            telegram_token,
            telegram_chat_id: env_from(&m, "TELEGRAM_CHAT_ID", "YOUR_TELEGRAM_CHAT_ID"),
            whatsapp_url: env_from(&m, "WHATSAPP_URL", "http://127.0.0.1:3052/send"),
            whatsapp_chat_id: env_from(&m, "WHATSAPP_CHAT_ID", "4917000000000@s.whatsapp.net"),
            email_to: env_from(&m, "EMAIL_TO", "booking-owner@example.de"),
            email_from: env_from(&m, "EMAIL_FROM", "booking-owner@example.de"),
            smtp: SmtpConfig {
                host: "smtp-relay.brevo.com".into(),
                port: 587,
                user: smtp_user,
                pass: smtp_pass,
                timeout_secs: 25,
            },
            http,
        }
    }
}

pub struct BookingBBooking {
    pub title: String,
    pub date: String,
    pub time: String,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub notes: String,
}

pub fn parse_booking_b_calcom(body: &Value) -> BookingBBooking {
    let d = body.get("payload").unwrap_or(body);
    let data = d.get("payload").unwrap_or(d);
    let evt_type = data.get("eventType");
    let responses = data.get("responses");
    let attendee0 = data.get("attendees").and_then(|a| a.as_array()).and_then(|a| a.first());

    let title = evt_type
        .and_then(|e| e.get("title"))
        .or_else(|| data.get("title"))
        .and_then(|v| v.as_str())
        .unwrap_or("Termin")
        .to_string();
    let start = data.get("startTime").and_then(|v| v.as_str()).unwrap_or("");
    let end = data.get("endTime").and_then(|v| v.as_str()).unwrap_or("");

    let resp_get = |k: &str| {
        let v = cal_field_str(responses.and_then(|r| r.get(k)));
        if v.is_empty() { None } else { Some(v) }
    };

    let name = resp_get("name")
        .or_else(|| attendee0.and_then(|a| a.get("name")).and_then(|v| v.as_str()).map(String::from))
        .unwrap_or_else(|| "Unbekannt".into());
    let email = resp_get("email")
        .or_else(|| attendee0.and_then(|a| a.get("email")).and_then(|v| v.as_str()).map(String::from))
        .unwrap_or_default();
    let phone = resp_get("phone")
        .or_else(|| resp_get("attendeePhoneNumber"))
        .or_else(|| resp_get("telefon"))
        .unwrap_or_default();
    let notes = resp_get("notes")
        .or_else(|| resp_get("message"))
        .or_else(|| resp_get("nachricht"))
        .or_else(|| resp_get("customRadio"))
        .unwrap_or_default();

    let (date, time) = format_cal_window(start, end, false);
    BookingBBooking { title, date, time, name, email, phone, notes }
}

pub fn build_booking_b_whatsapp_text(d: &BookingBBooking) -> String {
    let mut lines = vec![
        "🛞 *NEUE TERMINANFRAGE*".to_string(),
        String::new(),
        format!("*{}*", d.title),
        format!("📅 {}", d.date),
        format!("🕐 {}", d.time),
        String::new(),
        format!("👤 {}", d.name),
    ];
    if !d.phone.is_empty() {
        lines.push(format!("📞 {}", d.phone));
    }
    if !d.email.is_empty() {
        lines.push(format!("✉️ {}", d.email));
    }
    if !d.notes.is_empty() {
        lines.push(String::new());
        lines.push(format!("📝 {}", d.notes));
    }
    lines.push(String::new());
    lines.push("Bitte anrufen und Termin bestätigen.".to_string());
    lines.join("\n")
}

pub fn build_booking_b_email_html(d: &BookingBBooking) -> String {
    let name = if d.name.is_empty() { "Unbekannt" } else { &d.name };
    let phone = if d.phone.is_empty() { "—" } else { &d.phone };
    let email = if d.email.is_empty() { "—" } else { &d.email };
    let notes = if d.notes.is_empty() { "—" } else { &d.notes };

    format!(
        "<!DOCTYPE html><html><body style=\"margin:0;padding:0;background:#211B18;font-family:Arial,Helvetica,sans-serif;\">\
         <div style=\"max-width:600px;margin:0 auto;\">\
           <div style=\"background:#D05D26;padding:24px;text-align:center;\">\
             <div style=\"color:#FFFFFF;font-size:24px;font-weight:bold;letter-spacing:1px;\">BEISPIEL WERKSTATT</div>\
             <div style=\"color:#FFFFFF;font-size:13px;opacity:0.9;\">Neue Terminanfrage</div>\
           </div>\
           <div style=\"background:#FFFFFF;padding:28px;color:#211B18;\">\
             <h2 style=\"margin:0 0 16px;font-size:18px;\">🛞 {}</h2>\
             <table style=\"width:100%;border-collapse:collapse;font-size:15px;\">\
               <tr><td style=\"padding:8px 0;color:#888;\">Datum</td><td style=\"padding:8px 0;font-weight:bold;\">{}</td></tr>\
               <tr><td style=\"padding:8px 0;color:#888;\">Uhrzeit</td><td style=\"padding:8px 0;font-weight:bold;\">{}</td></tr>\
               <tr><td style=\"padding:8px 0;color:#888;\">Name</td><td style=\"padding:8px 0;font-weight:bold;\">{}</td></tr>\
               <tr><td style=\"padding:8px 0;color:#888;\">Telefon</td><td style=\"padding:8px 0;font-weight:bold;\">{}</td></tr>\
               <tr><td style=\"padding:8px 0;color:#888;\">E-Mail</td><td style=\"padding:8px 0;font-weight:bold;\">{}</td></tr>\
               <tr><td style=\"padding:8px 0;color:#888;vertical-align:top;\">Nachricht</td><td style=\"padding:8px 0;\">{}</td></tr>\
             </table>\
             <div style=\"margin-top:20px;padding:14px;background:#F5EDE2;border-left:4px solid #D05D26;\">\
               <strong>Bitte anrufen und Termin bestätigen.</strong>\
             </div>\
           </div>\
           <div style=\"text-align:center;padding:16px;color:#DECEAC;font-size:12px;\">Beispiel Werkstatt · Werkstattweg 1, 12345 Musterstadt</div>\
         </div></body></html>",
        escape_html(&d.title),
        escape_html(&d.date),
        escape_html(&d.time),
        escape_html(name),
        escape_html(phone),
        escape_html(email),
        escape_html(notes)
    )
}

pub fn booking_b_router(state: Arc<BookingBState>) -> Router {
    Router::new()
        .route("/", any(handle_booking_b))
        .route("/*rest", any(handle_booking_b))
        .with_state(state)
}

async fn handle_booking_b(State(st): State<Arc<BookingBState>>, method: Method, body: Bytes) -> impl IntoResponse {
    if method == Method::OPTIONS {
        return (StatusCode::OK, cors_headers(), String::new()).into_response();
    }
    if method == Method::GET {
        return json_resp(StatusCode::OK, json!({"status": "running", "service": "BookingB Booking Relay"})).into_response();
    }
    if method != Method::POST {
        return json_resp(StatusCode::METHOD_NOT_ALLOWED, json!({"error": "Method not allowed"})).into_response();
    }

    let Ok(payload) = serde_json::from_slice::<Value>(&body) else {
        return json_resp(StatusCode::BAD_REQUEST, json!({"error": "Invalid JSON"})).into_response();
    };

    let d = parse_booking_b_calcom(&payload);
    let wa_text = build_booking_b_whatsapp_text(&d);
    let email_html = build_booking_b_email_html(&d);
    let subject = format!("🛞 Neue Terminanfrage: {} ({})", d.name, d.date);

    let wa_fut = async {
        match st
            .http
            .post(&st.whatsapp_url)
            .json(&json!({"chatId": st.whatsapp_chat_id, "message": wa_text}))
            .send()
            .await
        {
            Ok(r) if r.status().is_success() => true,
            Ok(r) => {
                eprintln!("[BOOKING_B-RELAY] WhatsApp HTTP {}", r.status());
                false
            }
            Err(e) => {
                eprintln!("[BOOKING_B-RELAY] WhatsApp ERROR: {e}");
                false
            }
        }
    };

    let mail_fut = async {
        if st.smtp.user.is_empty() || st.smtp.pass.is_empty() {
            eprintln!("[BOOKING_B-RELAY] EMAIL SKIP: missing Brevo credentials");
            return false;
        }
        let from_hdr = format!("Beispiel Werkstatt <{}>", st.email_from);
        match send_smtp_starttls(
            &st.smtp,
            EmailPayload {
                from_header: &from_hdr,
                envelope_from: &st.email_from,
                to_header: &st.email_to,
                envelope_to: &st.email_to,
                subject: &subject,
                text: None,
                html: Some(&email_html),
                x_mailer: Some("BookingB-Booking-Relay-Rust"),
            },
        )
        .await
        {
            Ok(()) => true,
            Err(e) => {
                eprintln!("[BOOKING_B-RELAY] Email ERROR: {e}");
                false
            }
        }
    };

    let tg_fut = async {
        if st.telegram_token.is_empty() {
            return false;
        }
        let url = format!("https://api.telegram.org/bot{}/sendMessage", st.telegram_token);
        match st
            .http
            .post(&url)
            .json(&json!({"chat_id": st.telegram_chat_id, "text": wa_text, "parse_mode": ""}))
            .send()
            .await
        {
            Ok(r) => r
                .json::<Value>()
                .await
                .ok()
                .and_then(|v| v.get("ok").and_then(|b| b.as_bool()))
                .unwrap_or(false),
            Err(e) => {
                eprintln!("[BOOKING_B-RELAY] Telegram ERROR: {e}");
                false
            }
        }
    };

    // Execute all three notification channels concurrently instead of sequentially.
    let (ok_wa, ok_mail, ok_tg) = tokio::join!(wa_fut, mail_fut, tg_fut);
    let channels = json!({"whatsapp": ok_wa, "email": ok_mail, "telegram": ok_tg});
    if ok_wa || ok_mail || ok_tg {
        println!("[BOOKING_B-RELAY] OK: {} - {channels}", d.name);
        json_resp(StatusCode::OK, json!({"status": "sent", "channels": channels})).into_response()
    } else {
        eprintln!("[BOOKING_B-RELAY] ERROR: all channels failed - {channels}");
        json_resp(StatusCode::INTERNAL_SERVER_ERROR, json!({"error": "all channels failed", "channels": channels})).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_booking_b_calcom_nested_parsing_and_escape() {
        let payload = json!({
            "payload": {
                "payload": {
                    "eventType": {"title": "Reifenwechsel <VIP>"},
                    "startTime": "2026-11-15T10:00:00Z",
                    "endTime": "2026-11-15T10:30:00Z",
                    "responses": {
                        "name": {"value": "Max & Co"},
                        "phone": {"value": "+4917012345"},
                        "customRadio": {"label": "Winterräder"}
                    },
                    "attendees": [{"email": "max@example.de"}]
                }
            }
        });
        let b = parse_booking_b_calcom(&payload);
        assert_eq!(b.title, "Reifenwechsel <VIP>");
        assert_eq!(b.name, "Max & Co");
        assert_eq!(b.phone, "+4917012345");
        assert_eq!(b.email, "max@example.de");
        assert_eq!(b.notes, "Winterräder");
        assert_eq!(b.date, "15.11.2026");
        assert_eq!(b.time, "10:00 - 10:30");
        let html = build_booking_b_email_html(&b);
        assert!(html.contains("Reifenwechsel &lt;VIP&gt;"));
        assert!(html.contains("Max &amp; Co"));
    }

    #[test]
    fn test_booking_a_html_escaping() {
        let booking = json!({
            "title": "Hörtest",
            "startTime": "2026-11-15T09:00:00Z",
            "endTime": "2026-11-15T09:45:00Z",
            "attendees": [{"name": "Lisa <Test>", "email": "lisa@example.de"}],
            "guests": ["g1@example.de"]
        });
        let (email, _, owner_html, _, cust_html) = build_booking_a_emails(&booking);
        assert_eq!(email, "lisa@example.de");
        assert!(owner_html.contains("Lisa &lt;Test&gt;"));
        assert!(cust_html.contains("Vielen Dank, Lisa!"));
    }
}
