// main.rs

mod web;

use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new().nest("/v1", web::routes::v1::router());

    // run our app with hyper, listen globally on port 8000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    axum::serve(listener, app).await.unwrap()
}
