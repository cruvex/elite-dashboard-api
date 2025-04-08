use crate::config::RedisConfig;
use redis::aio::ConnectionManager;
use redis::{Client, RedisResult};

pub async fn init_redis(redis_config: &RedisConfig) -> RedisResult<ConnectionManager> {
    let client = Client::open(redis_config.url.clone())?;

    ConnectionManager::new(client).await
}
