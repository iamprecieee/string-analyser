use std::time::Duration;

use sqlx::{postgres::PgPoolOptions, Error, PgPool, Result};

pub type DbPool = PgPool;

pub async fn create_pool(
    database_url: &str,
    database_max_connections: u32,
    database_connection_timeout: u64,
) -> Result<DbPool, Error> {
    PgPoolOptions::new()
        .max_connections(database_max_connections)
        .acquire_timeout(Duration::from_secs(database_connection_timeout))
        .connect(database_url)
        .await
}
