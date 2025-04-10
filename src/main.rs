mod app;
mod db;
mod error;
mod model;
mod service;
mod web;

use app::config::AppConfig;
use axum::http::Method;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::routing::get;
use axum::{Router, middleware};
use tokio::signal;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::app::state::AppState;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_target(false).with_env_filter(EnvFilter::from_default_env()).init();

    let config = AppConfig::from_env().expect("Failed to load configuration");

    let db_pool = db::init_db(&config.database).await.expect("Failed to initialize database pool");
    db::run_migrations(db_pool.clone()).await.expect("Failed to run database migrations");

    let redis = db::init_redis(&config.redis).await.expect("Failed to initialize Redis");

    let state = AppState::initialize(db_pool.clone(), redis, &config).await.expect("Failed to initialize app state");

    let routes_api =
        web::routes_discord_api::routes(state.clone())
            .merge(web::routes_elite::routes(state.clone()))
            .layer(middleware::from_fn_with_state(
                state.clone(),
                web::middleware::mw_session::mw_session_require,
            ));

    // let routes_elite = web::routes_elite::routes(state.clone());

    let routes_auth = Router::new()
        // .merge(web::routes_auth::routes(state.clone()))
        .merge(web::routes_auth_discord::routes(state.clone()));

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(["http://localhost:5173".parse().unwrap(), "http://localhost:3000".parse().unwrap()])
        .allow_headers([CONTENT_TYPE, AUTHORIZATION])
        .allow_credentials(true);

    let routes_all = Router::new()
        .merge(routes_api)
        .merge(routes_auth)
        .route("/health", get(|| async { "Hello, World!" }))
        .layer(middleware::from_fn(web::middleware::mw_req_log::mw_req_log))
        .layer(middleware::map_response(web::middleware::mw_response_map::mw_response_map))
        .layer(CookieManagerLayer::new())
        .layer(cors);

    let listener_url = format!("{}:{}", &config.server.address, &config.server.port);
    let listener = tokio::net::TcpListener::bind(&listener_url).await.unwrap();

    info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());

    axum::serve(listener, routes_all.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("Failed to start server");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate()).expect("failed to install signal handler").recv().await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
