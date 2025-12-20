use crate::setup::config::Config;
use notify::{
    Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
    event::{MetadataKind, ModifyKind},
};

enum AppMsg {
    Notify(notify::Result<Event>),
    Shutdown,
}

pub fn run(cfg: &Config) -> notify::Result<()> {
    log::info!("app::run() watching {:?}", cfg.watch_path);

    let (event_tx, event_rx) = std::sync::mpsc::channel::<AppMsg>();

    let shutdown_tx = event_tx.clone();
    ctrlc::set_handler(move || {
        let _ = shutdown_tx.send(AppMsg::Shutdown);
    })
    .expect("failed to set Ctrl-C handler");

    let notify_tx = event_tx.clone();
    let mut watcher = RecommendedWatcher::new(
        move |res| {
            let _ = notify_tx.send(AppMsg::Notify(res));
        },
        notify::Config::default(),
    )?;
    watcher.watch(&cfg.watch_path, RecursiveMode::Recursive)?;

    for res in event_rx {
        match res {
            AppMsg::Shutdown => {
                log::info!("Shutdown requested (Ctrl-C).");
                break;
            }
            AppMsg::Notify(Ok(event)) => {
                log::info!("Change: {event:?}");
                if let Some(event) =
                    filter_supported_modify_metadata_extended_file_event(cfg, event)
                {
                    log::info!("Relevant change: {event:?}");
                }
            }
            AppMsg::Notify(Err(error)) => log::error!("Error: {error:?}"),
        }
    }
    Ok(())
}

fn filter_supported_modify_metadata_extended_file_event(
    cfg: &Config,
    event: Event,
) -> Option<Event> {
    let EventKind::Modify(ModifyKind::Metadata(MetadataKind::Extended)) = event.kind else {
        return None;
    };

    let has_supported_file = event
        .paths
        .iter()
        .any(|p| p.is_file() && is_supported_extension(cfg, p));

    if has_supported_file {
        Some(event)
    } else {
        None
    }
}

fn is_supported_extension(cfg: &Config, path: &std::path::Path) -> bool {
    let Some(ext) = path.extension().and_then(|e| e.to_str()) else {
        return false;
    };

    let ext = ext.to_ascii_lowercase();
    cfg.allowed_file_extensions
        .iter()
        .any(|allowed| allowed.eq_ignore_ascii_case(ext.as_str()))
}
