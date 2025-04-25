use crate::app::error::AppError;
use crate::app::state::AppState;
use crate::model::elite::{Elite, EliteStatus};
use crate::model::session::Session;
use crate::service::EliteService;
use crate::web::error::Error;
use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{Value, json};
use tracing::debug;

pub fn routes(state: AppState) -> Router {
    Router::new().route("/elites/@me", get(elites_me)).route("/elites", get(elites)).with_state(state)
}

async fn elites_me(session: Session, State(elite): State<EliteService>) -> Result<Json<Value>, AppError> {
    debug!("{:<12} - {}", "HANDLER", "elite_me");
    debug!("{}", session.user.id);

    let elite = elite.find_by_discord_id(&session.user.id).await?.ok_or(Error::NotInElite)?;

    Ok(Json(json!({
        "ign": elite.ign,
        "role": session.user.role.to_string(),
    })))
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
struct ElitesQueryParams {
    includeExElites: Option<bool>,
}

async fn elites(session: Session, State(elite): State<EliteService>, Query(params): Query<ElitesQueryParams>) -> Result<Json<Vec<Elite>>, AppError> {
    let include_ex_elites = params.includeExElites.unwrap_or(false);
    debug!("{:<12} - {} | includeExElites={}", "HANDLER", "elites", include_ex_elites);

    let statuses = if include_ex_elites && session.user.is_staff() {
        vec![
            EliteStatus::Staff,
            EliteStatus::Veteran,
            EliteStatus::Elite,
            EliteStatus::Trial,
            EliteStatus::None,
        ]
    } else {
        vec![EliteStatus::Staff, EliteStatus::Veteran, EliteStatus::Elite, EliteStatus::Trial]
    };

    let elites = elite.elites_all(&statuses).await?;

    Ok(Json(elites))
}
