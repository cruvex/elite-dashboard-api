use std::time::SystemTime;
use log::LevelFilter;

pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] {:5} - {}",
                humantime::format_rfc3339_micros(SystemTime::now()),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Trace)
        .level_for("axum", LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}