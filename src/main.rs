mod application;
mod domain;
mod infrastructure;
mod presentation;

use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use presentation::handlers::{compile_handler, validate_handler};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Mock allowed symbols for MVP (Ideally fetched from DB via SQLx)
    let allowed_symbols = vec![
        "NVDAc".to_string(), "MSFTc".to_string(), "GOOGLc".to_string(), 
        "AMZNc".to_string(), "METAc".to_string()
    ];

    let app = Router::new()
        .route("/api/health", get(|| async { "StockPilot API MVP live." }))
        .route("/api/strategy/compile", post(compile_handler))
        .route("/api/strategy/validate", post(validate_handler));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handlers would be implemented here, parsing JSON and passing to the domain logic.