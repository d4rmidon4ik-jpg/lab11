use axum::{routing::get, Json, Router};
use axum::body::Body;
use axum::http::{Request, StatusCode};
use serde_json::{json, Value};
use std::env;
use tower::ServiceExt; // для .oneshot()

async fn ping() -> Json<Value> {
    Json(json!({
        "message": "pong",
        "service": "rust"
    }))
}

async fn health() -> Json<Value> {
    Json(json!({"status": "ok"}))
}

fn app() -> Router {
    Router::new()
        .route("/ping", get(ping))
        .route("/health", get(health))
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "8081".to_string());
    let addr = format!("0.0.0.0:{}", port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Rust service starting on {}", addr);
    axum::serve(listener, app()).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;

    #[tokio::test]
    async fn test_ping() {
        let router = app();
        let req = Request::builder()
            .uri("/ping")
            .body(Body::empty())
            .unwrap();

        let resp = router.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["message"], "pong");
        assert_eq!(json["service"], "rust");
    }

    #[tokio::test]
    async fn test_health() {
        let router = app();
        let req = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let resp = router.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let body = to_bytes(resp.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(json["status"], "ok");
    }

    #[tokio::test]
    async fn test_ping_returns_json_content_type() {
        let router = app();
        let req = Request::builder()
            .uri("/ping")
            .body(Body::empty())
            .unwrap();

        let resp = router.oneshot(req).await.unwrap();
        let content_type = resp.headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap();
        assert!(content_type.contains("application/json"));
    }

    #[tokio::test]
    async fn test_unknown_route_returns_404() {
        let router = app();
        let req = Request::builder()
            .uri("/unknown")
            .body(Body::empty())
            .unwrap();

        let resp = router.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }
}