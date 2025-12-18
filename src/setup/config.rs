use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub watch_path: PathBuf,
    pub root_path: PathBuf,
    pub log_file: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            watch_path: PathBuf::from("./bucket"),
            root_path: PathBuf::from("./docs"),
            log_file: PathBuf::from("./logs/docsort.log"),
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let exe = std::env::current_exe().expect("current_exe() failed");
        let exe_dir = exe.parent().unwrap_or_else(|| std::path::Path::new("."));
        let config_path = exe_dir.join("docsort.toml");

        confy::load_path(&config_path).unwrap_or_else(|err| {
            log::info!(
                "Cannot read {:?} ({err}), using built-in defaults",
                config_path
            );
            let default = Config::default();
            let _ = confy::store_path(&config_path, &default);
            default
        })
    }
}
