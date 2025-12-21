use crate::setup::config::Config;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

use super::supervisor::{Task, TaskKind};

#[derive(Debug)]
pub enum WorkerCmd {
    Run(Task),
    Stop,
}

#[derive(Debug)]
pub enum WorkerEvent {
    Started { task_id: u64 },
    Heartbeat { task_id: u64 },
    Finished { task_id: u64 },
    Failed { task_id: u64, error: String },
    Stopped,
}

pub fn spawn_worker(cfg: Config) -> (Sender<WorkerCmd>, Receiver<WorkerEvent>) {
    let (cmd_tx, cmd_rx) = std::sync::mpsc::channel::<WorkerCmd>();
    let (ev_tx, ev_rx) = std::sync::mpsc::channel::<WorkerEvent>();

    thread::Builder::new()
        .name("runner-worker".to_string())
        .spawn(move || worker_main(cfg, cmd_rx, ev_tx))
        .expect("failed to spawn worker thread");

    (cmd_tx, ev_rx)
}

fn worker_main(_cfg: Config, cmd_rx: Receiver<WorkerCmd>, ev_tx: Sender<WorkerEvent>) {
    log::info!("Worker started.");

    for cmd in cmd_rx {
        match cmd {
            WorkerCmd::Stop => {
                let _ = ev_tx.send(WorkerEvent::Stopped);
                log::info!("Worker stopping.");
                break;
            }
            WorkerCmd::Run(task) => {
                let _ = ev_tx.send(WorkerEvent::Started { task_id: task.id });

                // Emit heartbeats while executing this task.
                // This does NOT prevent a hard block inside the task body,
                // but it helps detect "alive and progressing" for long operations.
                let (hb_stop_tx, hb_stop_rx) = std::sync::mpsc::channel::<()>();
                let hb_ev_tx = ev_tx.clone();
                let task_id = task.id;

                let hb_handle = thread::Builder::new()
                    .name(format!("runner-worker-hb-{task_id}"))
                    .spawn(move || heartbeat_loop(task_id, hb_ev_tx, hb_stop_rx))
                    .ok();

                let result = execute_task(&task);

                // Stop heartbeats and wait briefly for the heartbeat thread to exit.
                let _ = hb_stop_tx.send(());
                if let Some(h) = hb_handle {
                    let _ = h.join();
                }

                match result {
                    Ok(()) => {
                        let _ = ev_tx.send(WorkerEvent::Finished { task_id: task.id });
                    }
                    Err(err) => {
                        let _ = ev_tx.send(WorkerEvent::Failed {
                            task_id: task.id,
                            error: err,
                        });
                    }
                }
            }
        }
    }

    log::info!("Worker exited.");
}

fn heartbeat_loop(task_id: u64, ev_tx: Sender<WorkerEvent>, stop_rx: Receiver<()>) {
    let interval = Duration::from_secs(2);

    loop {
        // Wait for stop signal with timeout; otherwise emit heartbeat.
        log::info!("heartbeat_loop");
        match stop_rx.recv_timeout(interval) {
            Ok(()) => break,
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                let _ = ev_tx.send(WorkerEvent::Heartbeat { task_id });
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }
}

fn execute_task(task: &Task) -> Result<(), String> {
    match &task.kind {
        TaskKind::FileChanged { path } => {
            log::info!(
                "Executing task_id={} created_at={:?} kind=FileChanged path={:?}",
                task.id,
                task.created_at,
                path
            );

            // Simulate short work.
            let _started = Instant::now();
            log::info!("Simulating short work");
            Ok(())
        }
    }
}
