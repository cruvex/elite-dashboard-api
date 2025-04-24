use crate::app::constants::RAILWAY_REQUEST_ID_HEADER;
use crate::app::error::AppError;
use axum::body::Body;
use axum::http::{Method, Request, Uri};
use axum::middleware::Next;
use axum::response::Response;
use chrono::Utc;
use serde::Serialize;
use strum_macros::{Display, EnumString};
use tracing::trace;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)] // FIXME
pub struct ReqStamp {
    pub method: String,
    pub uri: String,
    pub id: String,
    pub platform: RequestPlatform,
    pub time_in: String,
    pub time_out: String,
}

#[derive(Clone, Debug, Serialize, Display, PartialEq, EnumString)]
pub enum RequestPlatform {
    #[strum(serialize = "local")]
    Local,
    #[strum(serialize = "railway")]
    Railway,
}

pub async fn mw_req_log(uri: Uri, req_method: Method, req: Request<Body>, next: Next) -> Result<Response, AppError> {
    trace!("{:<12} - mw_req_log", "MIDDLEWARE");

    let req_id_header = req.headers().get(RAILWAY_REQUEST_ID_HEADER);
    let (req_id, platform) = match req_id_header {
        Some(id) => (id.to_str().unwrap().to_string(), RequestPlatform::Railway),
        None => (Uuid::new_v4().to_string(), RequestPlatform::Local),
    };

    let time_in = Utc::now();

    let mut res = next.run(req).await;

    let time_out = Utc::now();

    let req_stamp = ReqStamp {
        method: req_method.to_string(),
        uri: uri.to_string(),
        id: req_id.clone(),
        platform,
        time_in: time_in.to_rfc3339(),
        time_out: time_out.to_rfc3339(),
    };

    res.extensions_mut().insert(req_stamp.clone());

    Ok(res)
}
