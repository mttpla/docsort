use docsort::setup::config;
use docsort::setup::logging;
use log::{error, info};

fn main() {
    let cfg = config::Config::load();
    let _logger_handle = logging::init(&cfg.log_file).unwrap_or_else(|err| {
        panic!(
            "Unable to initialize logger at {}: {err}",
            cfg.log_file.display()
        )
    });

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
