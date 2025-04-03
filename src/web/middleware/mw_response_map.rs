use crate::web;
use crate::web::error::Error;
use crate::web::middleware::mw_req_log::ReqStamp;
use axum::http::{Method, Uri};
use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use tracing::{debug, trace};

pub async fn mw_response_map(res: Response) -> Response {
    trace!("{:<12} - mw_reponse_map", "RES_MAPPER");
    let web_error = res.extensions().get::<Error>();
    debug!("{:<12} - web_error: {:?}", "RES_MAPPER", web_error);

    // TODO: Implement proper response mapping

    if let Some(err) = web_error {
        let response = match err {
            Error::RefreshCookieNotFound | Error::AuthCookieNotFound => (StatusCode::UNAUTHORIZED, ""),
            Error::NoDiscordCodeInPath => (StatusCode::BAD_REQUEST, ""),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ""),
        };
        return response.into_response();
    };

    // Print empty line for better readability
    debug!("\n");

    res
}
