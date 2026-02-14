// src/logger/config.rs
use simplelog::{Config, ConfigBuilder};
use time::macros::format_description;

pub(crate) fn build_log_config() -> Config {
    let time_format = format_description!("[year]-[month]-[day] [hour]:[minute]:[second]");

    ConfigBuilder::new()
        .set_time_format_custom(time_format)
        .set_level_padding(simplelog::LevelPadding::Off)
        .build()
}