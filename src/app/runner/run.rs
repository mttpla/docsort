use crate::setup::config::Config;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

use super::processor::{RunnerMsg, process_messages};

pub fn run(cfg: &Config) -> notify::Result<()> {
    log::info!("app::run() watching {:?}", cfg.watch_path);

    let (event_tx, event_rx) = std::sync::mpsc::channel::<RunnerMsg>();

    install_ctrlc_shutdown_handler(event_tx.clone()).expect("failed to set Ctrl-C handler");

    let mut watcher = create_notify_watcher(event_tx.clone())?;
    watcher.watch(&cfg.watch_path, RecursiveMode::Recursive)?;

    process_messages(cfg, event_rx);

    Ok(())
}

fn install_ctrlc_shutdown_handler(
    tx: std::sync::mpsc::Sender<RunnerMsg>,
) -> Result<(), ctrlc::Error> {
    ctrlc::set_handler(move || {
        let _ = tx.send(RunnerMsg::Shutdown);
    })
}

fn create_notify_watcher(
    tx: std::sync::mpsc::Sender<RunnerMsg>,
) -> notify::Result<RecommendedWatcher> {
    RecommendedWatcher::new(
        move |res| {
            let _ = tx.send(RunnerMsg::Notify(res));
        },
        notify::Config::default(),
    )
}
