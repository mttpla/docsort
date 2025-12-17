use flexi_logger::{
    Age, Cleanup, Criterion, Duplicate, FileSpec, FlexiLoggerError, Logger, LoggerHandle, Naming,
    WriteMode,
};
use std::{
    fs,
    path::{Path, PathBuf},
};
use thiserror::Error;

const RETENTION_DAYS: usize = 365;

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
        .format_for_stdout(flexi_logger::default_format)
        .format_for_files(flexi_logger::detailed_format)
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(RETENTION_DAYS),
        )
        .start()?;
    Ok(handle)
}
