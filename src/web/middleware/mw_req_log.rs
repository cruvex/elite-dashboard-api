use crate::app::error::{AppError, Result};
use crate::web::error::Error;
use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{Method, Request, Uri};
use axum::middleware::Next;
use axum::response::Response;
use time::OffsetDateTime;
use tracing::{debug, trace};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ReqStamp {
    pub method: Method,
    pub uri: Uri,
    pub uuid: Uuid,
    pub time_in: OffsetDateTime,
}

pub async fn mw_req_log(uri: Uri, req_method: Method, mut req: Request<Body>, next: Next) -> Result<Response> {
    trace!("{:<12} - mw_req_log", "MIDDLEWARE");

    let time_in = OffsetDateTime::now_utc();
    let uuid = Uuid::new_v4();

    let req_stamp = ReqStamp {
        method: req_method.clone(),
        uri: uri.clone(),
        uuid,
        time_in,
    };

    debug!("{:<12} - Incoming request: {req_stamp:?}", "MIDDLEWARE");

    let res = next.run(req).await;

    Ok(res)
}
