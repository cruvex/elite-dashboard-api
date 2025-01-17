use std::fmt::Error;
use std::sync::Arc;
use ::redis::aio::MultiplexedConnection;
use ::redis::Client;
use axum::Router;
use axum::routing::get;
use redis_pool::RedisPool;
use tracing::{debug, info};
use crate::config::Config;
use crate::logger::setup_logger;
use crate::web::routes_auth;

mod logger;
mod config;
mod web;
pub mod discord;
mod redis;
mod jwt;

pub use discord::Discord;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_logger().expect("Failed to setup logger");

    // Load config
    let config = Config::from_env().expect("Failed to load configuration");

    let redis = redis::init_redis(&config.redis);

    let discord = Discord::new(&config.discord);

    let state = AppState {
        discord,
        redis: Arc::new(redis)
    };

    let routes_all = Router::new()
        .merge(routes_auth::routes(state.clone()))
        .route("/", get(|| async { "Hello, World!" }));

    let listener_url = format!("{}:{}", &config.server.address, &config.server.port);

    let listener = tokio::net::TcpListener::bind(&listener_url).await.unwrap();
    info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[derive(Clone)]
struct AppState {
    discord: Discord,
    redis: Arc<RedisPool<Client, MultiplexedConnection>>
}
