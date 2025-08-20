// web/routes/v1/mod.rs

use axum::Router;

pub mod health;

pub fn router() -> Router {
    Router::new().merge(health::router())
}
