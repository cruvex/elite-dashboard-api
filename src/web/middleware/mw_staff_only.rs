use crate::app::error::AppError;
use crate::model::session::Session;
use crate::web::error::Error::StaffOnly;
use axum::body::Body;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use tracing::trace;

pub async fn mw_staff_only(session: Session, req: Request<Body>, next: Next) -> Result<Response, AppError> {
    trace!("{:<12} - mw_staff_only", "MIDDLEWARE");

    if !session.user.is_staff() {
        return Err(StaffOnly.into());
    }

    Ok(next.run(req).await)
}
