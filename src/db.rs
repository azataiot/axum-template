// src/db.rs

pub type Db = sqlx::PgPool;

pub async fn connect(url: &str) -> anyhow::Result<Db> {
    Ok(sqlx::PgPool::connect(url).await?)
}
