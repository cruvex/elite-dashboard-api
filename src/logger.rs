use tracing_subscriber::{fmt, EnvFilter};

pub fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
    let format = fmt::format()
        .with_timer(fmt::time::SystemTime::default())
        .with_level(true)
        .with_target(false)
        .compact(); // Compact output

    tracing_subscriber::fmt()
        .event_format(format)
        .with_env_filter(
            EnvFilter::new("trace") // Global default log level
                .add_directive("axum=info".parse()?) // Per-crate log level
                .add_directive("reqwest=info".parse()?)
                .add_directive("hyper_util=info".parse()?)
        )
        .with_writer(std::io::stdout)
        .init();

    Ok(())
}