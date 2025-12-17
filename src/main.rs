use docsort::cli::{AutostartAction, Cli, Command};
use docsort::platform;
use docsort::setup;
use log::{error, info};

fn main() {
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

    setup::folder::check_or_create_folders(&cfg).unwrap_or_else(|err| {
        error!("Failed to create folders: {err}");
    });

    let cli = Cli::parse_args();

    match cli.command() {
        Command::Run => {
            let version = env!("CARGO_PKG_VERSION");
            info!("v{version} started with config: {:?}", cfg);
            info!("starting...");
        }
        Command::Autostart { action } => match action {
            AutostartAction::Enable => match platform::autostart::enable() {
                Ok(()) => info!("Autostart enabled for the current user"),
                Err(err) => {
                    error!("Failed to enable autostart: {err}");
                    std::process::exit(1);
                }
            },
            AutostartAction::Disable => match platform::autostart::disable() {
                Ok(()) => info!("Autostart disabled for the current user"),
                Err(err) => {
                    error!("Failed to disable autostart: {err}");
                    std::process::exit(1);
                }
            },
        },
    }
    info!("shutting down.");
}
