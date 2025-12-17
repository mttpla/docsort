use super::AutostartStatus;
use std::io::{self, ErrorKind};
use thiserror::Error;
use winreg::{
    RegKey,
    enums::{HKEY_CURRENT_USER, KEY_READ, KEY_SET_VALUE},
};

const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
const VALUE_NAME: &str = "DocSort";

#[derive(Debug, Error)]
pub enum WindowsAutostartError {
    #[error("unable to resolve DocSort executable path: {0}")]
    CurrentExe(#[source] io::Error),

    #[error("Windows registry operation failed: {0}")]
    Registry(#[source] io::Error),
}

pub fn set_entry(enable: bool) -> Result<(), WindowsAutostartError> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    if enable {
        let exe = std::env::current_exe().map_err(WindowsAutostartError::CurrentExe)?;
        let command = format!("\"{}\"", exe.display());
        let (key, _) = hkcu
            .create_subkey(RUN_KEY)
            .map_err(WindowsAutostartError::Registry)?;
        key.set_value(VALUE_NAME, &command)
            .map_err(WindowsAutostartError::Registry)
    } else {
        match hkcu.open_subkey_with_flags(RUN_KEY, KEY_SET_VALUE) {
            Ok(key) => match key.delete_value(VALUE_NAME) {
                Ok(()) => Ok(()),
                Err(err) if err.kind() == ErrorKind::NotFound => Ok(()),
                Err(err) => Err(WindowsAutostartError::Registry(err)),
            },
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(()),
            Err(err) => Err(WindowsAutostartError::Registry(err)),
        }
    }
}

pub fn is_enabled() -> Result<AutostartStatus, WindowsAutostartError> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    match hkcu.open_subkey_with_flags(RUN_KEY, KEY_READ) {
        Ok(key) => match key.get_value::<String, _>(VALUE_NAME) {
            Ok(value) if !value.trim().is_empty() => Ok(AutostartStatus::Enabled),
            Ok(_) => Ok(AutostartStatus::Disabled),
            Err(err) if err.kind() == ErrorKind::NotFound => Ok(AutostartStatus::Disabled),
            Err(err) => Err(WindowsAutostartError::Registry(err)),
        },
        Err(err) if err.kind() == ErrorKind::NotFound => Ok(AutostartStatus::Disabled),
        Err(err) => Err(WindowsAutostartError::Registry(err)),
    }
}
