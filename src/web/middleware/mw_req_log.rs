use crate::app::error::Result;
use crate::service::constant::RAILWAY_REQUEST_ID_HEADER;
use axum::body::Body;
use axum::http::{Method, Request, Uri};
use axum::middleware::Next;
use axum::response::Response;
use chrono::Utc;
use tracing::trace;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ReqStamp {
    pub method: Method,
    pub uri: Uri,
    pub id: String,
    pub origin: String,
    pub time_in: String,
    pub time_out: String,
}

pub async fn mw_req_log(uri: Uri, req_method: Method, req: Request<Body>, next: Next) -> Result<Response> {
    trace!("{:<12} - mw_req_log", "MIDDLEWARE");

    let req_id_header = req.headers().get(RAILWAY_REQUEST_ID_HEADER);
    let (req_id, origin) = match req_id_header {
        Some(id) => (id.to_str().unwrap().to_string(), "railway"),
        None => (Uuid::new_v4().to_string(), "local"),
    };

    let time_in = Utc::now();

    let mut res = next.run(req).await;

    let time_out = Utc::now();

    let req_stamp = ReqStamp {
        method: req_method.clone(),
        uri: uri.clone(),
        id: req_id.clone(),
        origin: origin.to_string(),
        time_in: time_in.to_rfc3339(),
        time_out: time_out.to_rfc3339(),
    };

    res.extensions_mut().insert(req_stamp.clone());

    Ok(res)
}
