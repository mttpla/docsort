use flexi_logger::{
    Cleanup, Criterion, Duplicate, FileSpec, FlexiLoggerError, Logger, LoggerHandle, Naming,
    WriteMode,
};
use log::info;
use std::{
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;

/// Describes all recoverable errors that can happen while preparing the logger.
#[derive(Debug, Error)]
pub enum LoggingError {
    #[error("unable to create log directory {0:?}: {1}")]
    DirectoryCreation(PathBuf, #[source] std::io::Error),

    #[error("unable to initialize flexi_logger: {0}")]
    Logger(#[from] FlexiLoggerError),
}

/// Initialize flexi_logger using the provided file path.
///
/// The function ensures the log directory exists, configures rotation,
/// and emits an informational message once logging is ready.
pub fn init(log_file: &Path) -> Result<LoggerHandle, LoggingError> {
    let log_dir = log_file
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));

    if let Err(err) = fs::create_dir_all(&log_dir) {
        return Err(LoggingError::DirectoryCreation(log_dir, err));
    }

    let file_spec = FileSpec::try_from(log_file)?;
    let handle = Logger::try_with_env_or_str("info")?
        .log_to_file(file_spec)
        .write_mode(WriteMode::BufferAndFlush)
        .duplicate_to_stdout(Duplicate::Info)
        .format(flexi_logger::detailed_format)
        .rotate(
            Criterion::Size(10 * 1024 * 1024),
            Naming::Numbers,
            Cleanup::KeepLogFiles(7),
        )
        .start()?;

    info!(
        "Logging initialized; writing to {}",
        log_file.to_string_lossy()
    );

    Ok(handle)
}
