use redis::{aio::MultiplexedConnection, Client, RedisError};

pub async fn create_redis_client(redis_url: &str) -> Result<MultiplexedConnection, RedisError> {
    let client = Client::open(redis_url)?;
    let conn = client.get_multiplexed_async_connection().await?;
    Ok(conn)
}
