use crate::app::error::AppError;
use axum::response::Response;
use tracing::{debug, trace};
use crate::web::middleware::mw_req_log::ReqStamp;

pub async fn mw_response_map(res: Response) -> Response {
    trace!("{:<12} - mw_response_map", "RES_MAPPER");
    let web_error = res.extensions().get::<AppError>();

    if let Some(err) = web_error {
        debug!("{:<12} - web_error: {:?}", "RES_MAPPER", err);
    }

    let req_stamp = res.extensions().get::<ReqStamp>();
    debug!("{:<12} - Request info: {:?}", "RES_MAPPER", req_stamp);

    // TODO: Implement proper response mapping

    // Print empty line for better readability
    debug!("\n");

    res
}
