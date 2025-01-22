mod config;
mod error;
mod model;
mod service;
mod web;

use crate::config::AppConfig;
use axum::routing::get;
use axum::{middleware, Router};
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::config::app::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = AppConfig::from_env().expect("Failed to load configuration");

    let state = AppState::initialize(&config).await.expect("Failed to initialize app state");

    let routes_all = Router::new()
        .merge(web::routes_auth::routes(state.clone()))
        .merge(web::routes_auth_discord::routes(state.clone()))
        .layer(middleware::map_response(web::middleware::mw_response_map::mw_response_map))
        .layer(CookieManagerLayer::new())
        .layer(middleware::from_fn(web::middleware::mw_req_stamp::mw_req_stamp_resolver))
        .route("/health", get(|| async { "Hello, World!" }));

    let listener_url = format!("{}:{}", &config.server.address, &config.server.port);

    let listener = tokio::net::TcpListener::bind(&listener_url).await.unwrap();

    info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());

    axum::serve(listener, routes_all.into_make_service())
        .await
        .expect("Failed to start server");
}
