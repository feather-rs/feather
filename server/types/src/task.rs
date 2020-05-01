//! Implements a task-running system which allows asynchronous
//! tasks to run without worry of preemption during server shutdown.

use futures::Future;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::runtime;
use tokio::sync::Notify;

/// Stores tasks which must complete before the server may shut down.
pub struct RunningTasks(Arc<RunningTasksInner>);

struct RunningTasksInner {
    count: AtomicUsize,
    /// Notify handle which should be notified
    /// when a task completes. Allows the server
    /// to wait for tasks to complete.
    notify: Notify,
    /// Runtime to spawn tasks on.
    runtime: runtime::Handle,
}

impl RunningTasks {
    pub fn new(runtime: runtime::Handle) -> Self {
        Self(Arc::new(RunningTasksInner {
            count: Default::default(),
            notify: Notify::new(),
            runtime,
        }))
    }

    /// Schedules an asynchronous task to run.
    ///
    /// The task is guaranteed to finish, even if the server
    /// starts shutting down during its execution.
    pub fn schedule(&self, task: impl Future + Send + 'static) {
        self.0.count.fetch_add(1, Ordering::Relaxed);

        let running_tasks = Arc::clone(&self.0);

        self.0.runtime.enter(move || {
            tokio::spawn(async move {
                task.await;

                running_tasks.count.fetch_sub(1, Ordering::Relaxed);
                running_tasks.notify.notify();
            });
        });
    }

    /// Waits for all tasks to complete. Should be called before the program exits.
    pub async fn wait(&self) {
        while self.0.count.load(Ordering::Relaxed) > 0 {
            self.0.notify.notified().await;
        }
    }
}
