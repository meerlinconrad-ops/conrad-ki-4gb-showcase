use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use rustls::pki_types::ServerName;
use rustls::{ClientConfig, RootCertStore};
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_rustls::TlsConnector;

use crate::util::now_rfc2822;

#[derive(Clone, Debug)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub pass: String,
    pub timeout_secs: u64,
}

pub struct EmailPayload<'a> {
    pub from_header: &'a str,
    pub envelope_from: &'a str,
    pub to_header: &'a str,
    pub envelope_to: &'a str,
    pub subject: &'a str,
    pub text: Option<&'a str>,
    pub html: Option<&'a str>,
    pub x_mailer: Option<&'a str>,
}

pub(crate) fn tls_connector() -> TlsConnector {
    static CFG: OnceLock<Arc<ClientConfig>> = OnceLock::new();
    let cfg = CFG.get_or_init(|| {
        let mut roots = RootCertStore::empty();
        roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
        let c = ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth();
        Arc::new(c)
    });
    TlsConnector::from(cfg.clone())
}

fn sanitize_header_val(s: &str) -> String {
    s.replace(['\r', '\n'], " ").trim().to_string()
}

/// Extracts bare email address from `"Name <a@b.de>"` or `"a@b.de"`.
pub fn extract_addr(raw: &str) -> String {
    let clean = sanitize_header_val(raw);
    if let (Some(l), Some(r)) = (clean.rfind('<'), clean.rfind('>')) {
        if r > l + 1 {
            return clean[l + 1..r].trim().to_string();
        }
    }
    clean
}

/// Encodes non-ASCII subject using RFC 2047 UTF-8 Base64, otherwise returns sanitized ASCII.
pub fn encode_subject(subject: &str) -> String {
    let clean = sanitize_header_val(subject);
    if clean.is_ascii() {
        clean
    } else {
        format!("=?UTF-8?B?{}?=", B64.encode(clean.as_bytes()))
    }
}

fn wrap_b64_76(data: &[u8]) -> String {
    let encoded = B64.encode(data);
    let mut out = String::with_capacity(encoded.len() + encoded.len() / 76 * 2);
    for (i, chunk) in encoded.as_bytes().chunks(76).enumerate() {
        if i > 0 {
            out.push_str("\r\n");
        }
        out.push_str(std::str::from_utf8(chunk).unwrap_or(""));
    }
    out
}

/// Builds an RFC 5322 + MIME message (dot-stuffed lines for SMTP DATA).
pub fn build_rfc5322_message(mail: &EmailPayload<'_>) -> String {
    let mut headers = format!(
        "From: {}\r\nTo: {}\r\nSubject: {}\r\nDate: {}\r\nMIME-Version: 1.0\r\n",
        sanitize_header_val(mail.from_header),
        sanitize_header_val(mail.to_header),
        encode_subject(mail.subject),
        now_rfc2822()
    );
    if let Some(xm) = mail.x_mailer {
        headers.push_str(&format!("X-Mailer: {}\r\n", sanitize_header_val(xm)));
    }

    let body = match (mail.text.filter(|t| !t.is_empty()), mail.html.filter(|h| !h.is_empty())) {
        (Some(text), Some(html)) => {
            let boundary = format!("----conrad-relays-{:x}", chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0));
            format!(
                "Content-Type: multipart/alternative; boundary=\"{boundary}\"\r\n\r\n\
                 --{boundary}\r\n\
                 Content-Type: text/plain; charset=utf-8\r\n\
                 Content-Transfer-Encoding: base64\r\n\r\n\
                 {}\r\n\r\n\
                 --{boundary}\r\n\
                 Content-Type: text/html; charset=utf-8\r\n\
                 Content-Transfer-Encoding: base64\r\n\r\n\
                 {}\r\n\r\n\
                 --{boundary}--\r\n",
                wrap_b64_76(text.as_bytes()),
                wrap_b64_76(html.as_bytes())
            )
        }
        (None, Some(html)) => format!(
            "Content-Type: text/html; charset=utf-8\r\nContent-Transfer-Encoding: base64\r\n\r\n{}\r\n",
            wrap_b64_76(html.as_bytes())
        ),
        (Some(text), None) => format!(
            "Content-Type: text/plain; charset=utf-8\r\nContent-Transfer-Encoding: base64\r\n\r\n{}\r\n",
            wrap_b64_76(text.as_bytes())
        ),
        (None, None) => "Content-Type: text/plain; charset=utf-8\r\n\r\n".to_string(),
    };

    // Dot-stuffing not strictly needed on base64, still safe.
    format!("{headers}{body}")
}

async fn read_smtp_reply<R: AsyncRead + Unpin>(reader: &mut BufReader<R>) -> Result<(u16, String), String> {
    let mut full = String::new();
    loop {
        let mut line = String::new();
        let n = reader.read_line(&mut line).await.map_err(|e| format!("SMTP read error: {e}"))?;
        if n == 0 {
            return Err("SMTP unexpected EOF".to_string());
        }
        full.push_str(&line);
        let bytes = line.as_bytes();
        if bytes.len() >= 4 && bytes[0..3].iter().all(|b| b.is_ascii_digit()) {
            let sep = bytes[3];
            if sep == b' ' || sep == b'\r' || sep == b'\n' {
                let code: u16 = line[0..3].parse().unwrap_or(0);
                return Ok((code, full));
            }
        }
    }
}

async fn send_cmd<IO: AsyncRead + AsyncWrite + Unpin>(
    reader: &mut BufReader<IO>,
    cmd: &str,
    expected: u16,
) -> Result<String, String> {
    reader.get_mut().write_all(cmd.as_bytes()).await.map_err(|e| format!("SMTP write error: {e}"))?;
    reader.get_mut().write_all(b"\r\n").await.map_err(|e| format!("SMTP write error: {e}"))?;
    reader.get_mut().flush().await.map_err(|e| format!("SMTP flush error: {e}"))?;
    let (code, reply) = read_smtp_reply(reader).await?;
    if code != expected {
        return Err(format!("SMTP expected {expected}, got {code}: {}", reply.trim()));
    }
    Ok(reply)
}

/// Sends an email over STARTTLS SMTP (`port 587`) using `AUTH LOGIN`.
pub async fn send_smtp_starttls(cfg: &SmtpConfig, mail: EmailPayload<'_>) -> Result<(), String> {
    let dur = Duration::from_secs(cfg.timeout_secs.max(5));
    timeout(dur, async {
        let addr = format!("{}:{}", cfg.host, cfg.port);
        let stream = TcpStream::connect(&addr)
            .await
            .map_err(|e| format!("SMTP connect {addr} failed: {e}"))?;

        let mut plain = BufReader::new(stream);
        let (code, greeting) = read_smtp_reply(&mut plain).await?;
        if code != 220 {
            return Err(format!("SMTP greeting {code}: {}", greeting.trim()));
        }

        send_cmd(&mut plain, "EHLO relay.conrad-ki.com", 250).await?;
        send_cmd(&mut plain, "STARTTLS", 220).await?;

        let tcp = plain.into_inner();
        let server_name = ServerName::try_from(cfg.host.clone())
            .map_err(|e| format!("Invalid SMTP host {}: {e}", cfg.host))?;
        let tls_stream = tls_connector()
            .connect(server_name, tcp)
            .await
            .map_err(|e| format!("SMTP STARTTLS handshake failed: {e}"))?;

        let mut tls = BufReader::new(tls_stream);
        send_cmd(&mut tls, "EHLO relay.conrad-ki.com", 250).await?;
        send_cmd(&mut tls, "AUTH LOGIN", 334).await?;
        send_cmd(&mut tls, &B64.encode(cfg.user.as_bytes()), 334).await?;
        send_cmd(&mut tls, &B64.encode(cfg.pass.as_bytes()), 235).await?;

        let env_from = extract_addr(mail.envelope_from);
        let env_to = extract_addr(mail.envelope_to);
        send_cmd(&mut tls, &format!("MAIL FROM:<{env_from}>"), 250).await?;
        send_cmd(&mut tls, &format!("RCPT TO:<{env_to}>"), 250).await?;
        send_cmd(&mut tls, "DATA", 354).await?;

        let msg_data = build_rfc5322_message(&mail);
        tls.get_mut()
            .write_all(msg_data.as_bytes())
            .await
            .map_err(|e| format!("SMTP DATA write failed: {e}"))?;
        tls.get_mut()
            .write_all(b"\r\n.\r\n")
            .await
            .map_err(|e| format!("SMTP DATA end failed: {e}"))?;
        tls.get_mut()
            .flush()
            .await
            .map_err(|e| format!("SMTP flush failed: {e}"))?;

        let (done_code, done_reply) = read_smtp_reply(&mut tls).await?;
        if done_code != 250 {
            return Err(format!("SMTP DATA rejected ({done_code}): {}", done_reply.trim()));
        }
        let _ = send_cmd(&mut tls, "QUIT", 221).await;
        Ok(())
    })
    .await
    .map_err(|_| format!("SMTP timeout after {}s", cfg.timeout_secs))?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_addr_and_subject() {
        assert_eq!(extract_addr("Beispiel Werkstatt <booking-owner@example.de>"), "booking-owner@example.de");
        assert_eq!(extract_addr("info@example.de"), "info@example.de");
        assert_eq!(encode_subject("Hello"), "Hello");
        assert!(encode_subject("🛞 Neue Terminanfrage").starts_with("=?UTF-8?B?"));
    }

    #[test]
    fn test_build_multipart_message() {
        let msg = build_rfc5322_message(&EmailPayload {
            from_header: "Conrad KI <info@conrad-ki.com>",
            envelope_from: "info@conrad-ki.com",
            to_header: "info@conrad-ki.com",
            envelope_to: "info@conrad-ki.com",
            subject: "Test Ümlaut\r\nBcc: evil@attacker",
            text: Some("Hallo Welt"),
            html: Some("<p>Hallo Welt</p>"),
            x_mailer: Some("Conrad-Relays"),
        });
        assert!(!msg.contains("Bcc: evil@attacker\r\n"));
        assert!(msg.contains("Content-Type: multipart/alternative"));
        assert!(msg.contains("Content-Transfer-Encoding: base64"));
    }
}
