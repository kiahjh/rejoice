use std::time::Duration;

// Re-export sqlx types and query functions/macros
pub use sqlx::{Pool, Sqlite, query, query_as, FromRow};

pub struct PoolConfig {
    pub db_url: String,
    pub max_connections: u32,
    pub acquire_timeout: Duration,
    pub idle_timeout: Duration,
    pub max_lifetime: Duration,
}

pub async fn create_pool(config: PoolConfig) -> Pool<Sqlite> {
    sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(config.acquire_timeout)
        .idle_timeout(config.idle_timeout)
        .max_lifetime(config.max_lifetime)
        .connect(&config.db_url)
        .await
        .expect("Failed to create db pool")
}
