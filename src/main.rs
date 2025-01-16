use std::fmt::Error;
use axum::Router;
use axum::routing::get;
use log::info;
use crate::config::Config;
use crate::logger::setup_logger;


mod logger;
mod config;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_logger().expect("Failed to setup logger");

    let config = Config::from_env().expect("Failed to load configuration");

    let routes_all = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let listener_url = format!("{}:{}", &config.server.address, &config.server.port);

    let listener = tokio::net::TcpListener::bind(&listener_url).await.unwrap();
    info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}
