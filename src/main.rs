// main.rs

use axum::Router;
use axum::routing::get;

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
