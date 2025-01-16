pub mod routes_auth;
mod error;
mod handlers;

pub use self::error::Result;
pub use handlers::handlers_auth_discord;