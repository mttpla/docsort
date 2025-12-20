#![cfg_attr(windows, windows_subsystem = "windows")]

use docsort::cli::{AutostartAction, Cli, Command};
use docsort::platform;
use docsort::setup;
use log::{error, info};

fn main() {

    let cli = Cli::parse_args();

    match cli.command() {
        Command::Start => {
            info!("starting...");
            let _instance = setup::single_instance::check_single_instance().unwrap_or_else(|| {
                eprintln!("DocSort is already running!");
                std::process::exit(1);
            });
            let cfg = setup::config::Config::load();

            let _logger_handle = setup::logging::init(&cfg.log_file).unwrap_or_else(|err| {
                panic!(
                    "Unable to initialize logger at {}: {err}",
                    cfg.log_file.display()
                )
            });
            let version = env!("CARGO_PKG_VERSION");
            info!("v{version} started with config: {:?}", cfg);
            setup::folder::check_or_create_folders(&cfg).unwrap_or_else(|err| {
                error!("Failed to create folders: {err}");
            });

            // Start the long-running app loop (folder watcher + processing pipeline).
            // This blocks until the process is terminated (logout/shutdown/kill).
            if let Err(err) = docsort::app::runner::run(&cfg) {
                error!("app terminated with error: {err:#}");
                std::process::exit(1);
            }
            info!("shutting down.");
        }

        Command::Autostart { action } => match action {
            AutostartAction::Enable => match platform::autostart::enable() {
                Ok(()) => println!("Autostart enabled for the current user"),
                Err(err) => {
                    eprintln!!("Failed to enable autostart: {err}");
                    std::process::exit(1);
                }
            },
            AutostartAction::Disable => match platform::autostart::disable() {
                Ok(()) => println!("Autostart disabled for the current user"),
                Err(err) => {
                    eprintln!("Failed to disable autostart: {err}");
                    std::process::exit(1);
                }
            },
        },
    }

}
