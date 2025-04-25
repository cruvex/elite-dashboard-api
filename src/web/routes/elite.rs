use crate::app::error::AppError;
use crate::app::state::AppState;
use crate::model::session::Session;
use crate::service::EliteService;
use crate::web::error::Error;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{Value, json};
use tracing::debug;

pub fn routes(state: AppState) -> Router {
    Router::new().route("/elites/@me", get(elites_me)).with_state(state)
}

async fn elites_me(session: Session, State(elite): State<EliteService>) -> Result<Json<Value>, AppError> {
    debug!("{}", session.user.id);

    let elite = elite.find_by_discord_id(&session.user.id).await?.ok_or(Error::NotInElite)?;

    Ok(Json(json!({
        "ign": elite.ign,
        "role": session.user.role.to_string(),
    })))
}
