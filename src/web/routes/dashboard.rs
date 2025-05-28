use crate::app::error::AppError;
use crate::app::state::AppState;
use crate::model::elite::{Elite, EliteStatus};
use crate::model::recent_change::RecentChange;
use crate::service::{EliteService, IgnTrackerService};
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{Value, json};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/elites", get(dashboard_elites))
        .route("/ign-tracker", get(dashboard_ign_tracker))
        .with_state(state)
}

pub async fn dashboard_elites(State(elites): State<EliteService>) -> Result<Json<Value>, AppError> {
    let elites = elites.elites_all(&vec![EliteStatus::Staff, EliteStatus::Veteran, EliteStatus::Elite, EliteStatus::Trial]).await?;

    let elite_count = elites.len();
    let trial_elites = elites.iter().filter(|elite| elite.status == EliteStatus::Trial).collect::<Vec<&Elite>>();
    let veterans = elites.iter().filter(|elite| elite.status == EliteStatus::Veteran).collect::<Vec<&Elite>>();

    Ok(Json(json!({
        "elite_count": elite_count,
        "trial_elites": trial_elites,
        "veterans": veterans,
    })))
}

pub async fn dashboard_ign_tracker(State(ign_tracker): State<IgnTrackerService>) -> Result<Json<Vec<RecentChange>>, AppError> {
    let latest_changes = ign_tracker.get_latest_changes(7, 0).await?;

    Ok(Json(latest_changes))
}
