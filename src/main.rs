// main.rs

mod config;
use axum_template::app;

#[tokio::main]
async fn main() {
    // get the config
    let cfg = config::Config::from_env();

    let app = app();

    // run our app with hyper, listen globally on port 8000
    let listener = tokio::net::TcpListener::bind(&cfg.bind_addr).await.unwrap();

    axum::serve(listener, app).await.unwrap()
}
