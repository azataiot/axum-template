// web/routes/v1/health.rs

use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct Health {
    ok: bool,
}

async fn health() -> Json<Health> {
    Json(Health { ok: true })
}

pub fn router() -> Router {
    Router::new().route("/health", get(health))
}
