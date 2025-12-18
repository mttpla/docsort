use crate::setup::config::Config;
use log::error;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn check_or_create_folders(cfg: &Config) -> io::Result<()> {
    for path in [&cfg.watch_path, &cfg.root_path] {
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
    }

    // Safety: avoid infinite recursion by ensuring `watch_path` is not within
    // `root_path` (or any of its subfolders).
    //
    // If `watch_path` is inside `root_path`, moving files into `root_path`
    // can generate new events that are again watched, creating loops.
    let watch_abs = canonicalize_or_fallback(&cfg.watch_path);
    let root_abs = canonicalize_or_fallback(&cfg.root_path);

    if is_within_dir(&watch_abs, &root_abs) {
        error!(
            "Invalid configuration: watch_path ({}) must not be inside root_path ({})",
            cfg.watch_path.display(),
            cfg.root_path.display()
        );
        panic!(
            "Invalid configuration: watch_path ({}) must not be inside root_path ({})",
            cfg.watch_path.display(),
            cfg.root_path.display()
        );
    }

    Ok(())
}

fn canonicalize_or_fallback(path: &Path) -> PathBuf {
    // `canonicalize` can fail on some platforms / edge-cases (permissions, etc.).
    // In that case, we fallback to the original path to avoid masking other errors.
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

fn is_within_dir(candidate: &Path, ancestor: &Path) -> bool {
    // True if `candidate` == `ancestor` or `candidate` is under `ancestor`.
    // We consider equality invalid too, because watching the same folder we write to
    // is also a recipe for loops.
    candidate.starts_with(ancestor)
}
