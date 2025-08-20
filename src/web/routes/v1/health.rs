// web/routes/v1/health.rs

use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct Health {
    ok: bool,
}

async fn health() -> Json<Health> {
    Json(Health { ok: true })
}

pub fn routes() -> Router {
    Router::new().route("/health", get(health))
}
