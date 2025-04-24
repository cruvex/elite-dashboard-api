use crate::app::error::AppError;
use crate::app::state::AppState;
use crate::model::recent_change::RecentChange;
use crate::service::IgnTrackerService;
use crate::web::middleware::mw_staff_only::mw_staff_only;
use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router, middleware};
use serde::Deserialize;
use tracing::debug;

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/ign-history/latest", get(history_latest).layer(middleware::from_fn(mw_staff_only)))
        .with_state(state)
}

#[derive(Debug, Deserialize)]
pub struct PageQueryParams {
    limit: Option<i64>,
    offset: Option<i64>,
}

async fn history_latest(
    State(ign_tracker): State<IgnTrackerService>,
    Query(params): Query<PageQueryParams>,
) -> Result<Json<Vec<RecentChange>>, AppError> {
    debug!("{:<12} - {}", "HANDLER", "history_latest");

    let limit = params.limit.unwrap_or(10);
    let offset = params.offset.unwrap_or(0);

    let latest_changes = ign_tracker.get_latest_changes(limit, offset).await?;

    Ok(Json(latest_changes))
}
