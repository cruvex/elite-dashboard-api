mod config;
mod ctx;
mod error;
mod model;
mod service;
mod web;
mod app;

use crate::config::AppConfig;
use axum::routing::get;
use axum::{middleware, Router};
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::app::state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = AppConfig::from_env().expect("Failed to load configuration");

    let state = AppState::initialize(&config).await.expect("Failed to initialize app state");

    let routes_api = web::routes_discord_api::routes(state.clone()).layer(middleware::from_fn_with_state(
        state.clone(),
        web::middleware::mw_auth::mw_ctx_require,
    ));

    let routes_auth = Router::new()
        .merge(web::routes_auth::routes(state.clone()))
        .merge(web::routes_auth_discord::routes(state));

    let routes_all = Router::new()
        .nest("/api", routes_api)
        .merge(routes_auth)
        .route("/health", get(|| async { "Hello, World!" }))
        .layer(middleware::map_response(web::middleware::mw_response_map::mw_response_map))
        .layer(CookieManagerLayer::new())
        .layer(middleware::from_fn(web::middleware::mw_req_stamp::mw_req_stamp_resolver));

    let listener_url = format!("{}:{}", &config.server.address, &config.server.port);

    let listener = tokio::net::TcpListener::bind(&listener_url).await.unwrap();

    info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());

    axum::serve(listener, routes_all.into_make_service())
        .await
        .expect("Failed to start server");
}
