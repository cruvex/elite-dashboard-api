use axum::Router;
use axum::routing::get;
use crate::AppState;
use crate::web::handlers_auth_discord;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/auth/discord/oauth-url", get(handlers_auth_discord::redirect_url))
        .route("/auth/discord/callback", get(handlers_auth_discord::callback))
        .with_state(state)
}