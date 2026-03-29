use super::parser::parse_log_level;
use super::config::build_log_config;
use simplelog::{TermLogger, TerminalMode, ColorChoice};

// 对外暴露的初始化函数
pub fn init_logger(log_level: &str) -> Result<(), Box<dyn std::error::Error>> {
    let level = parse_log_level(log_level)?;
    let config = build_log_config();

    // init TermLogger
    TermLogger::init(
        level,
        config,
        TerminalMode::Stderr,
        ColorChoice::Auto,
    )?;

    log::info!("Logger initialized successfully: level={}", log_level);
    Ok(())
}

#[macro_export]
macro_rules! app_info {
    ($($arg:tt)*) => {
        log::info!("{}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! app_warning {
    ($($arg:tt)*) => {
        log::warn!("{}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! app_error {
    ($($arg:tt)*) => {
        log::error!("{}", format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! app_debug {
    ($($arg:tt)*) => {
        log::debug!("{}", format_args!($($arg)*));
    };
}