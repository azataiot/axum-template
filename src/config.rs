// src/config.rs

#[derive(Debug, Clone)]
pub struct Config {
    pub bind_addr: String,
    pub database_url: String,
}

fn get_env(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

impl Config {
    pub fn from_env() -> Self {
        // Load .env if present (dose nothing if the .env file is missing)
        let _ = dotenv::dotenv();

        Self {
            bind_addr: get_env("BIND_ADDR", "127.0.0.1:8000"),
            database_url: get_env(
                "DATABASE_URL",
                "postgres://postgres:postgres@localhost:5432/app",
            ),
        }
    }
}
