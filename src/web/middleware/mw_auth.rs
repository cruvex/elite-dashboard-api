use axum::extract::FromRequestParts;
use axum::{
    body::Body,
    extract::State,
    http::{Request, request::Parts},
    middleware::Next,
    response::Response,
};
use tower_cookies::Cookies;
use tracing::debug;

use crate::{
    ctx::Ctx,
    service::JwtService,
    web::{ACCESS_TOKEN_COOKIE, error::Error},
};

use crate::app::error::{AppError, Result};

pub async fn mw_ctx_require(
    //	ctx: Result<Ctx>,
    cookies: Cookies,
    State(jwt): State<JwtService>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    debug!("{:<12} - mw_ctx_require", "MIDDLEWARE");

    let access_token = cookies.get(ACCESS_TOKEN_COOKIE).ok_or(Error::RefreshCookieNotFound)?.value().to_string();

    let claims = jwt.validate_access_token(&access_token)?;

    req.extensions_mut().insert(Ctx::new(&claims.sub));

    debug!("Claims: {:?}", claims);

    Ok(next.run(req).await)
}

impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        debug!("{:<12} - Ctx", "EXTRACTOR");

        parts.extensions.get::<Ctx>().cloned().ok_or(Error::CtxNotinReqExt.into())
    }
}
