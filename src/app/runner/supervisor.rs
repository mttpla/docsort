use crate::setup::config::Config;
use std::collections::VecDeque;
use std::sync::mpsc::{Receiver, RecvTimeoutError, Sender};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use super::worker::{self, WorkerCmd, WorkerEvent};

/// A unit of work derived from filesystem notifications.
/// Keep it small, serializable-ish, and stable.
#[derive(Debug, Clone)]
pub struct Task {
    pub id: u64,
    pub created_at: Instant,
    pub kind: TaskKind,
}

#[derive(Debug, Clone)]
pub enum TaskKind {
    /// A relevant change happened to this path; downstream can decide what to do.
    FileChanged { path: std::path::PathBuf },
}

/// Commands sent to the supervisor from the ingest loop (`process_messages`).
#[derive(Debug)]
pub enum SupervisorCmd {
    Enqueue(Task),
    Shutdown,
}

/// Start a single supervisor thread and return a channel to send it commands.
///
/// Design goals:
/// - `process_messages` never blocks on worker work
/// - tasks are executed sequentially by one worker
/// - supervisor can replace a stuck worker (best-effort; threads can't be force-killed safely)
pub fn start_supervisor(cfg: Config) -> (Sender<SupervisorCmd>, JoinHandle<()>) {
    let (sup_tx, sup_rx) = std::sync::mpsc::channel::<SupervisorCmd>();

    let handle = thread::Builder::new()
        .name("runner-supervisor".to_string())
        .spawn(move || supervisor_main(cfg, sup_rx))
        .expect("failed to spawn supervisor thread");

    (sup_tx, handle)
}

struct SupervisorState {
    queue: VecDeque<Task>,
    in_flight: Option<Task>,
    shutdown_deadline: Option<Instant>,

    worker_tx: Sender<WorkerCmd>,
    worker_rx: Receiver<WorkerEvent>,
    last_heartbeat: Instant,
}

fn supervisor_main(cfg: Config, sup_rx: Receiver<SupervisorCmd>) {
    log::info!("Supervisor started.");

    // Tuning knobs (can be moved to Config later)
    let poll_tick = Duration::from_millis(200);
    let stuck_timeout = Duration::from_secs(120);

    let (worker_tx, worker_rx) = worker::spawn_worker(cfg.clone());

    let mut st = SupervisorState {
        queue: VecDeque::new(),
        in_flight: None,
        shutdown_deadline: None,
        worker_tx,
        worker_rx,
        last_heartbeat: Instant::now(),
    };

    loop {
        // If completely idle (no pending work, no in-flight work, and not shutting down),
        // block on recv() to avoid periodic wakeups.
        if st.shutdown_deadline.is_none() && st.in_flight.is_none() && st.queue.is_empty() {
            match sup_rx.recv() {
                Ok(cmd) => handle_supervisor_cmd(&mut st, cmd),
                Err(_) => {
                    // Channel disconnected means no more commands will arrive. Trigger shutdown.
                    if st.shutdown_deadline.is_none() {
                        log::info!("Supervisor command channel disconnected; shutting down.");
                        st.shutdown_deadline = Some(Instant::now() + Duration::from_secs(2));
                    }
                }
            }
        } else {
            // Active (work in-flight/queued or shutdown in progress): use a timeout tick to
            // keep draining worker events and performing deadline/health checks.
            match sup_rx.recv_timeout(poll_tick) {
                Ok(cmd) => handle_supervisor_cmd(&mut st, cmd),
                Err(RecvTimeoutError::Timeout) => {
                    // periodic tick: fallthrough to worker polling + health checks
                }
                Err(RecvTimeoutError::Disconnected) => {
                    // Channel disconnected means no more commands will arrive. Trigger shutdown.
                    if st.shutdown_deadline.is_none() {
                        log::info!("Supervisor command channel disconnected; shutting down.");
                        st.shutdown_deadline = Some(Instant::now() + Duration::from_secs(2));
                    }
                }
            }
        }

        // 2) Drain worker events (non-blocking-ish with small timeout via try_recv loop).
        while let Ok(ev) = st.worker_rx.try_recv() {
            handle_worker_event(&mut st, ev);
        }

        // 3) Health check (best-effort).
        if st.shutdown_deadline.is_none() && st.in_flight.is_some() {
            let since = Instant::now().saturating_duration_since(st.last_heartbeat);
            if since > stuck_timeout {
                log::error!(
                    "Worker considered stuck (no heartbeat for {:?}). Replacing worker.",
                    since
                );
                replace_worker(&mut st, cfg.clone());
            }
        }

        // 4) Dispatch next task if possible.
        if st.shutdown_deadline.is_none() && st.in_flight.is_none() {
            if let Some(next) = st.queue.pop_front() {
                if let Err(err) = st.worker_tx.send(WorkerCmd::Run(next.clone())) {
                    log::error!("Failed to send task to worker ({err:?}); replacing worker.");
                    // Put it back and replace worker.
                    st.queue.push_front(next);
                    replace_worker(&mut st, cfg.clone());
                } else {
                    st.last_heartbeat = Instant::now();
                    st.in_flight = Some(next);
                }
            }
        }

        // 5) Shutdown condition: if shutdown has been requested and nothing is in flight and queue is empty.
        if st.shutdown_deadline.is_some() && st.in_flight.is_none() && st.queue.is_empty() {
            let _ = st.worker_tx.send(WorkerCmd::Stop);
            log::info!("Supervisor exiting.");
            break;
        }

        // 6) Forced shutdown: once shutdown is requested, wait up to 2 seconds for in-flight work
        // to finish, then exit anyway. (deadline is set at the moment shutdown is requested)
        if let Some(deadline) = st.shutdown_deadline {
            if Instant::now() >= deadline {
                let _ = st.worker_tx.send(WorkerCmd::Stop);
                log::info!(
                    "Supervisor forced exit after shutdown deadline (queue_len={}, in_flight={}).",
                    st.queue.len(),
                    st.in_flight.as_ref().map(|t| t.id).unwrap_or(0)
                );
                break;
            }
        }
    }
}

fn handle_supervisor_cmd(st: &mut SupervisorState, cmd: SupervisorCmd) {
    match cmd {
        SupervisorCmd::Enqueue(task) => {
            if st.shutdown_deadline.is_some() {
                log::info!("Ignoring task enqueue during shutdown: {task:?}");
                return;
            }
            st.queue.push_back(task);
        }
        SupervisorCmd::Shutdown => {
            if st.shutdown_deadline.is_none() {
                log::info!("Supervisor shutdown requested.");
                st.shutdown_deadline = Some(Instant::now() + Duration::from_secs(2));
            }
        }
    }
}

fn handle_worker_event(st: &mut SupervisorState, ev: WorkerEvent) {
    match ev {
        WorkerEvent::Heartbeat { task_id } => {
            st.last_heartbeat = Instant::now();
            log::debug!("Worker heartbeat for task_id={task_id}");
        }
        WorkerEvent::Started { task_id } => {
            st.last_heartbeat = Instant::now();
            log::info!("Worker started task_id={task_id}");
        }
        WorkerEvent::Finished { task_id } => {
            st.last_heartbeat = Instant::now();
            log::info!("Worker finished task_id={task_id}");
            // Clear in-flight if it matches.
            if st.in_flight.as_ref().is_some_and(|t| t.id == task_id) {
                st.in_flight = None;
            } else {
                log::warn!(
                    "Received Finished for task_id={task_id} but supervisor had different in_flight={:?}",
                    st.in_flight.as_ref().map(|t| t.id)
                );
            }
        }
        WorkerEvent::Failed { task_id, error } => {
            st.last_heartbeat = Instant::now();
            log::error!("Worker failed task_id={task_id}: {error}");
            // For now: drop the task (or later: retry/backoff).
            if st.in_flight.as_ref().is_some_and(|t| t.id == task_id) {
                st.in_flight = None;
            }
        }
        WorkerEvent::Stopped => {
            log::info!("Worker stopped.");
        }
    }
}

fn replace_worker(st: &mut SupervisorState, cfg: Config) {
    // Best-effort: ask old worker to stop, but it might be stuck.
    let _ = st.worker_tx.send(WorkerCmd::Stop);

    // Re-enqueue in-flight task (at-least-once semantics).
    if let Some(task) = st.in_flight.take() {
        log::warn!(
            "Re-enqueueing in-flight task after worker replacement: {:?}",
            task.id
        );
        st.queue.push_front(task);
    }

    let (worker_tx, worker_rx) = worker::spawn_worker(cfg);
    st.worker_tx = worker_tx;
    st.worker_rx = worker_rx;
    st.last_heartbeat = Instant::now();
}
