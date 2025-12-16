use docsort::setup;
use log::{error, info};

fn main() {
    let cfg = setup::config::Config::load();
    let _logger_handle = setup::logging::init(&cfg.log_file).unwrap_or_else(|err| {
        panic!(
            "Unable to initialize logger at {}: {err}",
            cfg.log_file.display()
        )
    });

    let version = env!("CARGO_PKG_VERSION");
    info!("DocSort v{version} started with config: {:?}", cfg);

    setup::folder::check_or_create_folders(&cfg).unwrap_or_else(|err| {
        error!("Failed to create folders: {err}");
    });

    info!("DocSort v{version} shutting down.");
}
