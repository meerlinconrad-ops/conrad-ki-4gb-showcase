mod imap;
mod relays;
mod smtp;
mod util;

use axum::Router;
use imap::{imap_router, ImapState};
use relays::{
    booking_a_router, booking_b_router, contact_router, lead_router, BookingAState,
    BookingBState, ContactState, LeadState,
};
use reqwest::Client;
use std::{sync::Arc, time::Duration};
use tokio::net::TcpListener;
use tokio::task::JoinSet;

/// Parses a comma-separated IP list (e.g. `127.0.0.1,172.17.0.1,172.19.0.1`) while preserving order.
fn parse_bind_addrs(raw: &str) -> Vec<String> {
    let mut addrs = Vec::new();
    for part in raw.split(',') {
        let trimmed = part.trim();
        if !trimmed.is_empty() && !addrs.iter().any(|existing| existing == trimmed) {
            addrs.push(trimmed.to_string());
        }
    }
    if addrs.is_empty() {
        addrs.push("0.0.0.0".to_string());
    }
    addrs
}

async fn serve(bind_addrs: &[String], port: u16, router: Router, label: &'static str) {
    let mut tasks = JoinSet::new();
    for ip in bind_addrs {
        let addr = format!("{ip}:{port}");
        let listener = TcpListener::bind(&addr)
            .await
            .unwrap_or_else(|e| panic!("Failed to bind {label} on {addr}: {e}"));
        println!("[{label}] listening on {addr}");
        let r = router.clone();
        tasks.spawn(async move {
            axum::serve(listener, r)
                .with_graceful_shutdown(shutdown_signal())
                .await
                .unwrap_or_else(|e| eprintln!("[{label}] server error on {addr}: {e}"));
        });
    }
    while let Some(res) = tasks.join_next().await {
        if let Err(e) = res {
            eprintln!("[{label}] listener task panicked: {e}");
        }
    }
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        let mut sigterm = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("SIGTERM handler");
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {},
            _ = sigterm.recv() => {},
        }
    }
    #[cfg(not(unix))]
    let _ = tokio::signal::ctrl_c().await;
}

fn parse_port(default_port: u16) -> u16 {
    std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(default_port)
}

#[tokio::main]
async fn main() {
    let mut mode = String::from("all");
    let mut bind_arg = std::env::var("BIND_ADDRS").unwrap_or_else(|_| "0.0.0.0".into());

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--mode" => {
                if let Some(m) = args.next() {
                    mode = m;
                }
            }
            "--bind-addrs" => {
                if let Some(b) = args.next() {
                    bind_arg = b;
                }
            }
            _ => {}
        }
    }

    let bind_addrs = parse_bind_addrs(&bind_arg);

    let shared_http = Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .expect("reqwest client");

    match mode.as_str() {
        "contact" => {
            serve(
                &bind_addrs,
                parse_port(3010),
                contact_router(Arc::new(ContactState::from_env(shared_http))),
                "contact-relay",
            )
            .await;
        }
        "lead" => {
            serve(
                &bind_addrs,
                parse_port(8080),
                lead_router(Arc::new(LeadState::from_env())),
                "lead-webhook",
            )
            .await;
        }
        "booking_a" => {
            serve(
                &bind_addrs,
                parse_port(3051),
                booking_a_router(Arc::new(BookingAState::from_env())),
                "booking_a-relay",
            )
            .await;
        }
        "booking_b" => {
            serve(
                &bind_addrs,
                parse_port(3050),
                booking_b_router(Arc::new(BookingBState::from_env(shared_http))),
                "booking_b-booking-relay",
            )
            .await;
        }
        "imap" => {
            serve(
                &bind_addrs,
                parse_port(5005),
                imap_router(Arc::new(ImapState::from_env())),
                "imap-relay",
            )
            .await;
        }
        "all" => {
            tokio::join!(
                serve(
                    &bind_addrs,
                    3010,
                    contact_router(Arc::new(ContactState::from_env(shared_http.clone()))),
                    "contact-relay",
                ),
                serve(
                    &bind_addrs,
                    8080,
                    lead_router(Arc::new(LeadState::from_env())),
                    "lead-webhook",
                ),
                serve(
                    &bind_addrs,
                    3051,
                    booking_a_router(Arc::new(BookingAState::from_env())),
                    "booking_a-relay",
                ),
                serve(
                    &bind_addrs,
                    3050,
                    booking_b_router(Arc::new(BookingBState::from_env(shared_http))),
                    "booking_b-booking-relay",
                ),
                serve(
                    &bind_addrs,
                    5005,
                    imap_router(Arc::new(ImapState::from_env())),
                    "imap-relay",
                ),
            );
        }
        other => {
            eprintln!(
                "Unknown --mode '{other}'. Expected: contact | lead | booking_a | booking_b | imap | all"
            );
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bind_addrs() {
        assert_eq!(
            parse_bind_addrs("127.0.0.1, 172.17.0.1,172.19.0.1,127.0.0.1"),
            vec!["127.0.0.1", "172.17.0.1", "172.19.0.1"]
        );
        assert_eq!(parse_bind_addrs(""), vec!["0.0.0.0"]);
    }
}
