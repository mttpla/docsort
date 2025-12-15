use log::{error, info};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

mod logging;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
struct Config {
    watch_path: PathBuf,
    root_path: PathBuf,
    log_file: PathBuf,
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
    fn load() -> Self {
        let exe = std::env::current_exe().expect("current_exe() failed");
        let exe_dir = exe.parent().unwrap_or_else(|| Path::new("."));
        let config_path = exe_dir.join("docsort.toml");

        confy::load_path(&config_path).unwrap_or_else(|err| {
            eprintln!(
                "Cannot read {:?} ({err}), using built-in defaults",
                config_path
            );
            let default = Config::default();
            let _ = confy::store_path(&config_path, &default);
            default
        })
    }
}

fn main() {
    let cfg = Config::load();

    let _logger_handle = match logging::init(&cfg.log_file) {
        Ok(handle) => Some(handle),
        Err(err) => {
            eprintln!(
                "Unable to initialize logger at {}: {err}",
                cfg.log_file.display()
            );
            None
        }
    };

    let version = env!("CARGO_PKG_VERSION");
    info!("DocSort v{version} started with config: {:?}", cfg);

    if !cfg.watch_path.exists() {
        error!(
            "Watch path {:?} does not exist; nothing will be processed",
            cfg.watch_path
        );
    }

    info!("DocSort v{version} shutting down.");
}
