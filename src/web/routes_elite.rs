use crate::app::error::AppError;
use crate::app::state::AppState;
use crate::model::elite::Elite;
use crate::model::session::Session;
use crate::service::EliteService;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{Value, json};
use tracing::debug;

pub fn routes(state: AppState) -> Router {
    Router::new().route("/elite/@me", get(elite_me)).with_state(state)
}

async fn elite_me(session: Session, State(elite): State<EliteService>) -> Result<Json<Value>, AppError> {
    debug!("{}", session.user.id);

    let elite = elite.find_by_discord_id(&session.user.id).await?;

    Ok(Json(json!(&elite)))
}
