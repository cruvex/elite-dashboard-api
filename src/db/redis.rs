use crate::config::RedisConfig;
use redis::aio::MultiplexedConnection;
use redis::{Client, RedisResult};

pub type RedisConnection = MultiplexedConnection;

pub async fn init_redis(redis_config: &RedisConfig) -> RedisResult<RedisConnection> {
    let client = Client::open(redis_config.url.clone());
    client?.get_multiplexed_tokio_connection().await
}
