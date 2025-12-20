use crate::setup::config::Config;
use notify::Event;

#[derive(Debug)]
pub enum RunnerMsg {
    Notify(notify::Result<Event>),
    Shutdown,
}

pub fn process_messages(cfg: &Config, rx: std::sync::mpsc::Receiver<RunnerMsg>) {
    for msg in rx {
        match msg {
            RunnerMsg::Shutdown => {
                log::info!("Shutdown requested (Ctrl-C).");
                break;
            }
            RunnerMsg::Notify(Ok(event)) => {
                log::info!("Change: {event:?}");
                if let Some(event) =
                    super::filter::filter_supported_modify_file_event(cfg, event)
                {
                    log::info!("Relevant change: {event:?}");
                }
            }
            RunnerMsg::Notify(Err(error)) => log::error!("Error: {error:?}"),
        }
    }
}
