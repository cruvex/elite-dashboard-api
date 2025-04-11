use crate::app::error::AppError;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::JsonFields;

pub fn init_tracing() -> Result<(), AppError> {
    let is_railway = std::env::var("RAILWAY_ENVIRONMENT_NAME").is_ok();
    let env_filter = EnvFilter::from_default_env();

    if is_railway {
        tracing_subscriber::fmt().with_env_filter(env_filter).json().fmt_fields(JsonFields::new()).flatten_event(true).init();
    } else {
        tracing_subscriber::fmt().with_env_filter(env_filter).with_target(false).with_file(false).init();
    }

    Ok(())
}
