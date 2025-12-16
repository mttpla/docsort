use crate::setup::config::Config;
use std::fs;
use std::io;

pub fn check_or_create_folders(cfg: &Config) -> io::Result<()> {
    for path in [&cfg.watch_path, &cfg.root_path] {
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
    }
    Ok(())
}
