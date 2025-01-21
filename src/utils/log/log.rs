use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::LevelFilter;
use std::path::Path;

#[derive(Debug)]
pub enum LogError {
    ConfigError(String),
    IoError(std::io::Error),
}

pub fn init_logger(log_path: &str) -> Result<(), LogError> {
    let log_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} {l} - {m}{n}")))
        .build(Path::new(log_path))
        .map_err(|e| LogError::IoError(e))?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(log_file)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .map_err(|e| LogError::ConfigError(e.to_string()))?;

    log4rs::init_config(config).map_err(|e| LogError::ConfigError(e.to_string()))?;

    Ok(())
}
