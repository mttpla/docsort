use notify::{Event, RecommendedWatcher, RecursiveMode};
use notify_debouncer_full::{DebounceEventResult, Debouncer, new_debouncer};
use std::time::Duration;

use super::processor::{RunnerMsg, process_messages};

pub fn run(cfg: &crate::setup::config::Config) -> notify::Result<()> {
    log::info!("app::run() watching {:?}", cfg.watch_path);

    let (event_tx, event_rx) = std::sync::mpsc::channel::<RunnerMsg>();

    install_ctrlc_shutdown_handler(event_tx.clone()).expect("failed to set Ctrl-C handler");

    let mut debouncer = create_notify_watcher(event_tx.clone())?;
    debouncer.watch(&cfg.watch_path, RecursiveMode::Recursive)?;

    process_messages(cfg, event_rx);

    Ok(())
}

fn install_ctrlc_shutdown_handler(
    tx: std::sync::mpsc::Sender<RunnerMsg>,
) -> std::result::Result<(), ctrlc::Error> {
    ctrlc::set_handler(move || {
        let _ = tx.send(RunnerMsg::Shutdown);
    })
}

fn create_notify_watcher(
    tx: std::sync::mpsc::Sender<RunnerMsg>,
) -> notify::Result<Debouncer<RecommendedWatcher, notify_debouncer_full::FileIdMap>> {
    let debouncer = new_debouncer(
        Duration::from_millis(1000),
        None,
        move |res: DebounceEventResult| match res {
            Ok(events) => {
                for debounced in events {
                    let event: Event = debounced.event;
                    let _ = tx.send(RunnerMsg::Notify(Ok(event)));
                }
            }
            Err(errors) => {
                for err in errors {
                    let _ = tx.send(RunnerMsg::Notify(Err(err)));
                }
            }
        },
    )?;
    Ok(debouncer)
}
