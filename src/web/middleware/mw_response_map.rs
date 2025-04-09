use crate::app::constants::LOCAL_REQUEST_ID_HEADER;
use crate::app::error::AppError;
use crate::web::middleware::mw_req_log::{ReqStamp, RequestPlatform};
use axum::response::Response;
use tracing::{debug, trace};

pub async fn mw_response_map(mut res: Response) -> Response {
    trace!("{:<12} - mw_response_map", "RES_MAPPER");
    let web_error = res.extensions().get::<AppError>();

    if let Some(err) = web_error {
        debug!("{:<12} - web_error: {:?}", "RES_MAPPER", err);
    }

    // There should always be a request stamp in the response extensions
    let req_stamp = res.extensions().get::<ReqStamp>().unwrap().clone();
    debug!("{:<12} - Request info: {:?}", "RES_MAPPER", req_stamp);

    // insert request id into response headers if the request was local
    if req_stamp.platform == RequestPlatform::Local {
        res.headers_mut().insert(LOCAL_REQUEST_ID_HEADER, req_stamp.id.parse().unwrap());
    }

    // TODO: Implement proper response mapping

    // Print empty line for better readability
    debug!("\n");

    res
}
