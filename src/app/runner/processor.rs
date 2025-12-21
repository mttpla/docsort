use crate::setup::config::Config;
use notify::Event;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use super::supervisor::{SupervisorCmd, Task, TaskKind};

#[derive(Debug)]
pub enum RunnerMsg {
    Notify(notify::Result<Event>),
    Shutdown,
}

pub fn process_messages(cfg: &Config, rx: std::sync::mpsc::Receiver<RunnerMsg>) {
    // Supervisor owns the worker and guarantees tasks are executed sequentially without
    // blocking this ingest loop.
    let (supervisor_tx, _supervisor_handle) = super::supervisor::start_supervisor(cfg.clone());

    static TASK_ID: AtomicU64 = AtomicU64::new(1);

    for msg in rx {
        match msg {
            RunnerMsg::Shutdown => {
                log::info!("Shutdown requested (Ctrl-C).");
                let _ = supervisor_tx.send(SupervisorCmd::Shutdown);
                break;
            }
            RunnerMsg::Notify(Ok(event)) => {
                log::info!("Change: {event:?}");
                if let Some(event) = super::filter::filter_supported_modify_file_event(cfg, event) {
                    log::info!("Relevant change: {event:?}");

                    // For now we enqueue one Task per relevant notify event.
                    // Later we can:
                    // - coalesce multiple paths
                    // - dedupe bursts
                    // - enrich task kind based on event details
                    let task_id = TASK_ID.fetch_add(1, Ordering::Relaxed);

                    // notify::Event can contain multiple paths; we enqueue one task per path.
                    for path in event.paths {
                        let task = Task {
                            id: task_id,
                            created_at: Instant::now(),
                            kind: TaskKind::FileChanged { path },
                        };
                        let _ = supervisor_tx.send(SupervisorCmd::Enqueue(task));
                    }
                }
            }
            RunnerMsg::Notify(Err(error)) => log::error!("Error: {error:?}"),
        }
    }
}
