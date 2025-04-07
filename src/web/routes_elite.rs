use crate::app::error::AppError;
use crate::app::state::AppState;
use crate::model::session::Session;
use crate::service::EliteService;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{Value, json};
use std::time::Instant;
use tracing::debug;

pub fn routes(state: AppState) -> Router {
    Router::new().route("/elite/@me", get(elite_me)).with_state(state)
}

async fn elite_me(session: Session, State(elite): State<EliteService>) -> Result<Json<Value>, AppError> {
    debug!("{}", session.user.id);

    let start = Instant::now();

    let elite = elite.find_by_discord_id(&session.user.id).await?;

    let elapsed = start.elapsed().as_millis();

    Ok(Json(json!({
        "elite": &elite,
        "elapsed": format!("{}ms", elapsed),
    })))
}
