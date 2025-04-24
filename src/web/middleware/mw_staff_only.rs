use crate::app::error::AppError;
use crate::model::session::Session;
use crate::web::error::Error::{SessionNotFound, StaffOnly};
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tracing::trace;

pub async fn mw_staff_only(req: Request<Body>, next: Next) -> Result<Response, AppError> {
    trace!("{:<12} - mw_staff_only", "MIDDLEWARE");
    let session = req.extensions().get::<Session>().ok_or(SessionNotFound)?.clone();

    if !session.user.is_staff() {
        return Err(StaffOnly.into());
    }

    Ok(next.run(req).await)
}
