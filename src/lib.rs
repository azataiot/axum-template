// src/lib.rs

// Integration tests runs as a separate crate, so they can't import our binary (main.rs).
// We'll add a small src/lib.rs here that builds the router and reuse it in main.rs.

pub mod web;
use axum::Router;
pub fn app() -> Router {
    Router::new().nest("/v1", web::routes::v1::router())
}
