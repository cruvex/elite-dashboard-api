mod app;
mod db;
mod error;
mod model;
mod service;
mod web;

use app::config::AppConfig;
use app::state::AppState;
use tokio::signal;
use tracing::info;
use tracing::log::debug;

#[tokio::main]
async fn main() {
    app::tracing::init_tracing().expect("Failed to initialize tracing");

    let config = AppConfig::from_env().expect("Failed to load configuration");

    let db_pool = db::init_db(&config.database).await.expect("Failed to initialize database pool");
    db::run_migrations(db_pool.clone()).await.expect("Failed to run database migrations");

    let redis = db::init_redis(&config.redis).await.expect("Failed to initialize Redis");

    let state = AppState::initialize(db_pool.clone(), redis, &config).await.expect("Failed to initialize app state");

    let listener_url = format!("{}:{}", &config.server.address, &config.server.port);
    let listener = tokio::net::TcpListener::bind(&listener_url).await.unwrap();

    info!("{:<12} - {:?}", "LISTENING", listener.local_addr());
    debug!("");

    axum::serve(listener, web::app_router(state.clone()))
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
