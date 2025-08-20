// main.rs

use axum_template::app;
use axum_template::{config, db};

#[tokio::main]
async fn main() {
    // get the config
    let cfg = config::Config::from_env();

    // connect to the database
    let pool = db::connect(&cfg.database_url).await.unwrap();

    // run migrations (embedded at compile time)
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let app = app();

    let listener = tokio::net::TcpListener::bind(&cfg.bind_addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
