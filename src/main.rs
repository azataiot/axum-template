// main.rs

mod config;
mod web;

use axum::Router;

#[tokio::main]
async fn main() {
    // get the config
    let cfg = config::Config::from_env();

    let app = Router::new().nest("/v1", web::routes::v1::router());

    // run our app with hyper, listen globally on port 8000
    let listener = tokio::net::TcpListener::bind(&cfg.bind_addr).await.unwrap();

    axum::serve(listener, app).await.unwrap()
}
