use axum::{Router, routing::get};

pub fn health_route() -> Router{
    Router::new().route("/api/health", get(|| async {"OK"}))
}