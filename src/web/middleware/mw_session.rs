use axum::extract::FromRequestParts;
use axum::{
    body::Body,
    extract::State,
    http::{Request, request::Parts},
    middleware::Next,
    response::Response,
};
use tower_cookies::Cookies;
use tracing::{debug, trace};

use crate::web::error::Error;

use crate::app::constants::SESSION_COOKIE_NAME;
use crate::app::error::{AppError, Result};
use crate::model::session::Session;
use crate::service::SessionService;

pub async fn mw_session_require(
    cookies: Cookies,
    State(session_store): State<SessionService>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    trace!("{:<12} - mw_session_require", "MIDDLEWARE");

    let session = cookies.get(SESSION_COOKIE_NAME).ok_or(Error::SessionCookieNotFound)?;
    let session_id = session.value().to_string();

    let session = session_store.validate_session(&session_id).await?;

    debug!("{:<12} - Valid session", "MIDDLEWARE");

    req.extensions_mut().insert(session.clone());

    Ok(next.run(req).await)
}

impl<S: Send + Sync> FromRequestParts<S> for Session {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        trace!("{:<12} - Session", "EXTRACTOR");

        parts.extensions.get::<Session>().cloned().ok_or_else(|| Error::SessionNotFound.into())
    }
}
