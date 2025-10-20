use std::env;

use anyhow::{Result, anyhow};

pub fn load_config() -> Result<(String, u32, u64)> {
    let database_url =
        env::var("DATABASE_URL").map_err(|e| anyhow!("Missing DATABASE_URL: {}", e))?;

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

    Ok((
        database_url,
        database_max_connections,
        database_connection_timeout,
    ))
}
