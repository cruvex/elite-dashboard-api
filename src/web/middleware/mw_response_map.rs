use crate::app::error::AppError;
use axum::response::Response;
use tracing::{debug, trace};

pub async fn mw_response_map(res: Response) -> Response {
    trace!("{:<12} - mw_response_map", "RES_MAPPER");
    let web_error = res.extensions().get::<AppError>();

    if let Some(err) = web_error {
        debug!("{:<12} - web_error: {:?}", "RES_MAPPER", err);
    }

    // TODO: Implement proper response mapping

    // if let Some(err) = web_error {
    //     let response = match err {
    //         Error::SessionCookieNotFound => (StatusCode::UNAUTHORIZED, ""),
    //         Error::NoCodeInDiscordCallbackPath | Error::NoStateInDiscordCallbackPath => (StatusCode::BAD_REQUEST, ""),
    //         _ => (StatusCode::INTERNAL_SERVER_ERROR, ""),
    //     };
    //     return response.into_response();
    // };

    // Print empty line for better readability
    debug!("\n");

    res
}
