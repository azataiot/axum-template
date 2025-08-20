// src/config.rs

#[derive(Debug, Clone)]
pub struct Config {
    pub bind_addr: String,
}

impl Config {
    pub fn from_env() -> Self {
        // Load .env if present (dose nothing if the .env file is missing)
        let _ = dotenv::dotenv();

        let bind_addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:8000".to_string());

        Self { bind_addr }
    }
}
