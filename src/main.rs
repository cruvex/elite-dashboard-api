use log::{debug, error, info, trace, warn};
use crate::logger::setup_logger;

mod logger;

fn main() {
    setup_logger().expect("Failed to setup logger");

    trace!("Trace message");
    debug!("Debug message");
    info!("Info message");
    warn!("Warning message");
    error!("Error message");
}
