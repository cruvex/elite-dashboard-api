use crate::web;
use crate::web::error::Error;
use crate::web::middleware::mw_req_stamp::ReqStamp;
use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use tracing::debug;

pub async fn mw_response_map(uri: Uri, req_method: Method, req_stamp: ReqStamp, res: Response) -> Response {
    let web_error = res.extensions().get::<web::error::Error>();
    debug!("{:<12} - {:?}", "RES_MAPPER", web_error);

    debug!("{:<12} - mw_reponse_map", "RES_MAPPER");
    debug!("Uri: {}", uri);
    debug!("Method: {}", req_method);
    debug!("ReqStamp: {:?}", req_stamp);

    debug!("\n");

    // TODO: Implement proper response mapping

    if let Some(err) = web_error {
        let response = match err {
            Error::RefreshCookieNotFound | Error::AuthCookieNotFound => (StatusCode::UNAUTHORIZED, ""),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ""),
        };
        return response.into_response();
    };

    res
}
