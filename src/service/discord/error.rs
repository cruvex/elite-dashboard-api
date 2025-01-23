use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Deserialize;
use std::fmt::{Debug, Formatter, Display};
use tracing::debug;

#[derive(Clone, Debug)]
pub enum Error {
    DiscordApiRequestError(String),
}

// impl IntoResponse for Error {
//     fn into_response(self) -> Response {
//         debug!("{:<12} - {self:?}", "INTO_RES");
//
//         let mut response = (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response();
//
//         response.extensions_mut().insert(self);
//
//         response
//     }
// }
impl Display for Error {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
