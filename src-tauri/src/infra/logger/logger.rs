use log::LevelFilter;
use simplelog::{ColorChoice, Config, ConfigBuilder, LevelPadding, TermLogger, TerminalMode};
use time::macros::format_description;

// 解析日志级别字符串为 log 库的级别过滤器
fn parse_log_level(level_str: &str) -> Result<LevelFilter, Box<dyn std::error::Error>> {
    match level_str.to_lowercase().as_str() {
        "debug" => Ok(LevelFilter::Debug),
        "info" => Ok(LevelFilter::Info),
        "warn" => Ok(LevelFilter::Warn),
        "error" => Ok(LevelFilter::Error),
        _ => Err(format!("Invalid log level: {}", level_str).into()),
    }
}

// output config
fn build_log_config() -> Config {
    let time_format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

    ConfigBuilder::new()
        .set_time_format_custom(time_format)
        .set_level_padding(LevelPadding::Off)
        .build()
}

// init the log sys
pub fn init_logger(log_level: &str) -> Result<(), Box<dyn std::error::Error>> {
    let level = parse_log_level(log_level)?;
    let config = build_log_config();

    TermLogger::init(
        level,
        config,
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;

    log::info!("Logger initialized successfully: level={}", log_level);
    Ok(())
}

// info
#[macro_export]
macro_rules! app_info {
    ($($arg:tt)*) => {
        log::info!("{}", format_args!($($arg)*))
    };
}

// warning
#[macro_export]
macro_rules! app_warning {
    ($($arg:tt)*) => {
        log::warn!("{}", format_args!($($arg)*))
    };
}

// error
#[macro_export]
macro_rules! app_error {
    ($($arg:tt)*) => {
        log::error!("{}", format_args!($($arg)*))
    };
}

// debug
#[macro_export]
macro_rules! app_debug {
    ($($arg:tt)*) => {
        log::debug!("{}", format_args!($($arg)*))
    };
}
