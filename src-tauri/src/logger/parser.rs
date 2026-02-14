use log::LevelFilter;

pub(crate) fn parse_log_level(level_str: &str) -> Result<LevelFilter, Box<dyn std::error::Error>> {
    match level_str.to_lowercase().as_str() {
        "debug" => Ok(LevelFilter::Debug),
        "info" => Ok(LevelFilter::Info),
        "warn" => Ok(LevelFilter::Warn),
        "error" => Ok(LevelFilter::Error),
        _ => Err(format!("Invalid log level: {}", level_str).into()),
    }
}