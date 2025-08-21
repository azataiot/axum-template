// main.rs

use axum::extract::{Json, Path, Query};
use axum::routing::get;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // build the root router
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/bar", get(foo_bar));
    // run the app with hyper, listening globally on port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// the router calls one of the handlers for each request
async fn root() -> &'static str {
    "Hello, world!"
}

async fn get_foo() -> &'static str {
    "foo"
}

async fn post_foo() -> &'static str {
    "foo"
}

async fn foo_bar() -> &'static str {
    "foo/bar"
}

// Extractors
// `Path` gives you the path parameters and deserializes them.
async fn path(Path(user_id): Path<u32>) {}

// `Query` gives you the query parameters and deserializes them.
async fn query(Query(params): Query<HashMap<String, String>>) {}

// Buffer the request body and deserialize it as JSON into a
// `serde_json::Value`. `Json` supports any type that implements
// `serde::Deserialize`.
async fn json(Json(payload): Json<serde_json::Value>) {}
