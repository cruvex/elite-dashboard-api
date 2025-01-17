use redis::aio::MultiplexedConnection;
use redis::Client;
use redis_pool::RedisPool;
use crate::config::RedisConfig;

pub fn init_redis(config: &RedisConfig) -> RedisPool<Client, MultiplexedConnection> {
    let redis = Client::open(config.url.as_str()).expect("Failed to create Redis client");
    RedisPool::from(redis)
}