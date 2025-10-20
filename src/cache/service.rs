use redis::{AsyncCommands, RedisError, aio::MultiplexedConnection};

use crate::models::properties::AnalysedString;

#[derive(Clone)]
pub struct CacheService {
    redis: MultiplexedConnection,
}

impl CacheService {
    pub fn new(redis: MultiplexedConnection) -> Self {
        Self { redis }
    }

    pub fn clone_redis(&self) -> MultiplexedConnection {
        self.redis.clone()
    }

    pub async fn get(&self, id: &str) -> Result<Option<AnalysedString>, RedisError> {
        let cache_key = format!("string:{}", id);
        let mut conn = self.redis.clone();

        let result: Option<String> = conn.get(&cache_key).await?;

        match result {
            Some(json_data) => {
                let analysed_string: AnalysedString = serde_json::from_str(&json_data).unwrap();
                Ok(Some(analysed_string))
            }
            None => Ok(None),
        }
    }

    pub async fn set(&self, analysed_data: &AnalysedString) -> Result<(), RedisError> {
        let cache_key = format!("string:{}", analysed_data.id);
        let json_data = serde_json::to_string(analysed_data).unwrap();
        let mut conn = self.redis.clone();

        let _: () = conn.set_ex(&cache_key, json_data, 3600).await?;
        Ok(())
    }

    pub async fn delete(&self, id: &str) -> Result<(), RedisError> {
        let cache_key = format!("string:{}", id);
        let mut conn = self.redis.clone();

        let _: () = conn.del(&cache_key).await?;
        Ok(())
    }

    pub async fn invalidate(&self) -> Result<(), redis::RedisError> {
        let mut conn = self.redis.clone();
        let pattern = "query:*";

        let cache_keys: Vec<String> = redis::cmd("KEYS")
            .arg(pattern)
            .query_async(&mut conn)
            .await?;

        if !cache_keys.is_empty() {
            let _: () = conn.del(&cache_keys).await?;
        }

        Ok(())
    }
}
