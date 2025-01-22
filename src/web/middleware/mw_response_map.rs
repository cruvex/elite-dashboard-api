use crate::web;
use crate::web::middleware::mw_req_stamp::ReqStamp;
use axum::http::{Method, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use tracing::debug;
use uuid::Uuid;

pub async fn mw_response_map(uri: Uri, req_method: Method, req_stamp: ReqStamp, res: Response) -> Response {
    let web_error = res.extensions().get::<web::error::Error>();
    debug!("{:<12} - {:?}", "RES_MAPPER", web_error);

    debug!("{:<12} - mw_reponse_map", "RES_MAPPER");
    debug!("Uri: {}", uri);
    debug!("Method: {}", req_method);
    debug!("ReqStamp: {:?}", req_stamp);

    debug!("\n");

    // TODO: Implement response mapping

    res
}
