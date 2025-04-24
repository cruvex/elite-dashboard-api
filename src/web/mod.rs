use crate::app::state::AppState;
use axum::Router;
use axum::http::header::CONTENT_TYPE;
use axum::http::{Method, StatusCode};
use axum::routing::get;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;

pub mod error;
pub mod middleware;
pub mod routes;

pub fn app_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::OPTIONS])
        .allow_origin(["http://192.168.1.38:5173".parse().unwrap()])
        .allow_headers([CONTENT_TYPE])
        .allow_credentials(true);

    let authenticated_routes = Router::new()
        .merge(routes::elite::routes(state.clone()))
        .merge(routes::discord::routes(state.clone()))
        .merge(routes::ign_history::routes(state.clone()))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            middleware::mw_session::mw_session_require,
        ));

    Router::new()
        .merge(authenticated_routes)
        .merge(routes::auth::routes(state.clone()))
        .route("/healthz", get(|| async { StatusCode::OK }))
        .layer(axum::middleware::from_fn(middleware::mw_req_log::mw_req_log))
        .layer(axum::middleware::map_response(middleware::mw_response_map::mw_response_map))
        .layer(CookieManagerLayer::new())
        .layer(cors)
}
