use crate::app::error::Result;
use axum::body::Body;
use axum::http::{Method, Request, Uri};
use axum::middleware::Next;
use axum::response::Response;
use chrono::Utc;
use std::time::Instant;
use time::OffsetDateTime;
use tracing::{debug, trace};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ReqStamp {
    pub method: Method,
    pub uri: Uri,
    pub uuid: Uuid,
    pub origin: String,
    pub time_in: String,
    pub time_out: String,
}

pub async fn mw_req_log(uri: Uri, req_method: Method, req: Request<Body>, next: Next) -> Result<Response> {
    trace!("{:<12} - mw_req_log", "MIDDLEWARE");

    let req_id_header = req.headers().get("x-railway-request-id");
    let (req_id, origin) = match req_id_header {
        Some(id) => (Uuid::parse_str(id.to_str().unwrap()).unwrap(), "railway"),
        None => (Uuid::new_v4(), "local"),
    };

    let time_in = Utc::now();

    let mut res = next.run(req).await;

    let time_out = Utc::now();

    let req_stamp = ReqStamp {
        method: req_method.clone(),
        uri: uri.clone(),
        uuid: req_id.clone(),
        origin: origin.to_string(),
        time_in: time_in.to_rfc3339(),
        time_out: time_out.to_rfc3339(),
    };

    res.extensions_mut().insert(req_stamp.clone());

    Ok(res)
}
