use std::env;

use anyhow::{anyhow, Result};

pub fn load_config() -> Result<(String, String, u32, u64, String, u32)> {
    let database_url =
        env::var("DATABASE_URL").map_err(|e| anyhow!("Missing DATABASE_URL: {}", e))?;

    let redis_url = env::var("REDIS_URL").map_err(|e| anyhow!("Missing REDIS_URL: {}", e))?;

    let max_conn_str = env::var("DATABASE_MAX_CONNECTIONS")
        .map_err(|e| anyhow!("Missing DATABASE_MAX_CONNECTIONS: {}", e))?;

    let database_max_connections = max_conn_str
        .parse::<u32>()
        .map_err(|e| anyhow!("Invalid DATABASE_MAX_CONNECTIONS '{}': {}", max_conn_str, e))?;

    let timeout_str = env::var("DATABASE_CONNECTION_TIMEOUT")
        .map_err(|e| anyhow!("Missing DATABASE_CONNECTION_TIMEOUT: {}", e))?;

    let database_connection_timeout = timeout_str.parse::<u64>().map_err(|e| {
        anyhow!(
            "Invalid DATABASE_CONNECTION_TIMEOUT '{}': {}",
            timeout_str,
            e
        )
    })?;

    let host = env::var("SERVER_HOST").unwrap_or(String::from("0.0.0.0"));

    let port = env::var("REDIS_URL")
        .unwrap_or_else(|_| String::from("8000"))
        .parse()
        .unwrap_or(8000);

    Ok((
        database_url,
        redis_url,
        database_max_connections,
        database_connection_timeout,
        host,
        port,
    ))
}
