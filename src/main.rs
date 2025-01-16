use std::fmt::Error;
use axum::Router;
use axum::routing::get;
use log::info;
use crate::logger::setup_logger;


mod logger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    setup_logger().expect("Failed to setup logger");

    let routes_all = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
}
