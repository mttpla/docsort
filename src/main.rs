use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
struct Config {
    watch_path: PathBuf,
    root_path: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            watch_path: PathBuf::from("./bucket"),
            root_path: PathBuf::from("./docs"),
        }
    }
}

impl Config {
    fn load_config() -> Self {
        let exe = std::env::current_exe().expect("current_exe() failed");
        let exe_dir = exe.parent().unwrap_or_else(|| Path::new("."));
        let config_path = exe_dir.join("docsort.toml");

        confy::load_path(&config_path).unwrap_or_else(|err| {
            eprintln!(
                "Cannot read {:?} ({err}), falling back to built-in defaults",
                config_path
            );
            let default = Config::default();
            let _ = confy::store_path(&config_path, &default);
            default
        })
    }
}

fn main() {
    let cfg = Config::load_config();
    let version = env!("CARGO_PKG_VERSION");
    println!("DocDrop v{}", version);
    println!("{cfg:?}");
}
