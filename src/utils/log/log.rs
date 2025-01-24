use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::path::Path;

/// CN: 日志错误类型枚举
/// EN: Log error type enumeration
#[derive(Debug)]
pub enum LogError {
    /// CN: 配置错误
    /// EN: Configuration error
    ConfigError(String),
    /// CN: IO错误
    /// EN: IO error
    IoError(std::io::Error),
}

/// CN: 初始化日志记录器
/// CN: 参数:
/// CN: - log_path: 日志文件路径
/// CN: 返回:
/// CN: - Ok(()):  初始化成功
/// CN: - Err(LogError): 初始化过程中的错误
///
/// EN: Initialize logger
/// EN: Parameters:
/// EN: - log_path: Log file path
/// EN: Returns:
/// EN: - Ok(()): Initialization successful
/// EN: - Err(LogError): Errors during initialization
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
