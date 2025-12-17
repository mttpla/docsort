use thiserror::Error;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
use self::windows::WindowsAutostartError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutostartStatus {
    Enabled,
    Disabled,
}

#[derive(Debug, Error)]
pub enum AutostartError {
    #[error("autostart is not supported on this platform")]
    Unsupported,

    #[cfg(target_os = "windows")]
    #[error(transparent)]
    Windows(#[from] WindowsAutostartError),
}

pub fn enable() -> Result<(), AutostartError> {
    platform_enable()
}

pub fn disable() -> Result<(), AutostartError> {
    platform_disable()
}

pub fn status() -> Result<AutostartStatus, AutostartError> {
    platform_status()
}

#[cfg(target_os = "windows")]
fn platform_enable() -> Result<(), AutostartError> {
    windows::set_entry(true).map_err(AutostartError::from)
}

#[cfg(not(target_os = "windows"))]
fn platform_enable() -> Result<(), AutostartError> {
    Err(AutostartError::Unsupported)
}

#[cfg(target_os = "windows")]
fn platform_disable() -> Result<(), AutostartError> {
    windows::set_entry(false).map_err(AutostartError::from)
}

#[cfg(not(target_os = "windows"))]
fn platform_disable() -> Result<(), AutostartError> {
    Err(AutostartError::Unsupported)
}

#[cfg(target_os = "windows")]
fn platform_status() -> Result<AutostartStatus, AutostartError> {
    windows::is_enabled().map_err(AutostartError::from)
}

#[cfg(not(target_os = "windows"))]
fn platform_status() -> Result<AutostartStatus, AutostartError> {
    Err(AutostartError::Unsupported)
}
