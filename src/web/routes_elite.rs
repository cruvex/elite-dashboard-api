use crate::app::state::AppState;
use crate::model::session::Session;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use tracing::debug;

pub fn routes(state: AppState) -> Router {
    Router::new().route("/elite/@me", get(elite_me)).with_state(state)
}

async fn elite_me(session: Session) -> impl IntoResponse {
    debug!("{}", session.user.id);

    (
        StatusCode::OK,
        Json(json!({
            "user_id": session.user.id
        })),
    )
}
