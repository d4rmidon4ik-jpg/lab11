use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use std::env;

async fn ping() -> Json<Value> {
    Json(json!({
        "message": "pong",
        "service": "rust"
    }))
}

async fn health() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "8081".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Rust service starting on {}", addr);
    axum::serve(listener, app).await.unwrap();
}