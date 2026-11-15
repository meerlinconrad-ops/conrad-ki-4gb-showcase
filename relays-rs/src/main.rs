mod relays;
mod smtp;
mod util;

use axum::Router;
use relays::{
    contact_router, lead_router, booking_a_router, booking_b_router, ContactState, LeadState,
    BookingAState, BookingBState,
};
use reqwest::Client;
use std::{sync::Arc, time::Duration};
use tokio::net::TcpListener;

async fn serve(addr: String, router: Router, label: &'static str) {
    let listener = TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| panic!("Failed to bind {label} on {addr}: {e}"));
    println!("[{label}] listening on {addr}");
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap_or_else(|e| eprintln!("[{label}] server error: {e}"));
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

#[tokio::main]
async fn main() {
    let mut mode = String::from("all");
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--mode" {
            if let Some(m) = args.next() {
                mode = m;
            }
        }
    }

    let shared_http = Client::builder()
        .timeout(Duration::from_secs(15))
        .build()
        .expect("reqwest client");

    match mode.as_str() {
        "contact" => {
            let port = std::env::var("PORT").unwrap_or_else(|_| "3010".into());
            serve(
                format!("0.0.0.0:{port}"),
                contact_router(Arc::new(ContactState::from_env(shared_http))),
                "contact-relay",
            )
            .await;
        }
        "lead" => {
            let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
            serve(
                format!("0.0.0.0:{port}"),
                lead_router(Arc::new(LeadState::from_env())),
                "lead-webhook",
            )
            .await;
        }
        "booking_a" => {
            let port = std::env::var("PORT").unwrap_or_else(|_| "3051".into());
            serve(
                format!("0.0.0.0:{port}"),
                booking_a_router(Arc::new(BookingAState::from_env())),
                "booking_a-relay",
            )
            .await;
        }
        "booking_b" => {
            let port = std::env::var("PORT").unwrap_or_else(|_| "3050".into());
            serve(
                format!("0.0.0.0:{port}"),
                booking_b_router(Arc::new(BookingBState::from_env(shared_http))),
                "booking_b-booking-relay",
            )
            .await;
        }
        "all" => {
            tokio::join!(
                serve("0.0.0.0:3010".into(), contact_router(Arc::new(ContactState::from_env(shared_http.clone()))), "contact-relay"),
                serve("0.0.0.0:8080".into(), lead_router(Arc::new(LeadState::from_env())), "lead-webhook"),
                serve("0.0.0.0:3051".into(), booking_a_router(Arc::new(BookingAState::from_env())), "booking_a-relay"),
                serve("0.0.0.0:3050".into(), booking_b_router(Arc::new(BookingBState::from_env(shared_http))), "booking_b-booking-relay"),
            );
        }
        other => {
            eprintln!("Unknown --mode '{other}'. Expected: contact | lead | booking_a | booking_b | all");
            std::process::exit(2);
        }
    }
}
