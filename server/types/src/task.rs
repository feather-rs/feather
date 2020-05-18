//! Implements a global task scheduler as a wrapper over Tokio.
//!
//! A few guarantees are made:
//! * The server will not shut down until all tasks complete.
//! * All scheduled tasks will complete at some point.

use crate::Game;
use fecs::World;
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::future::Future;
use std::mem::{transmute, MaybeUninit};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::runtime;
use tokio::sync::{oneshot, Notify};
use tokio::task::JoinHandle;

/// TODO: reduce allocation
enum SyncFn {
    Owned(Box<dyn FnOnce(&mut Game, &mut World) + Send + 'static>),
    Scoped(*mut (dyn FnMut(&mut Game, &mut World))),
}

unsafe impl Send for SyncFn {}

/// Fake wrapper which causes a value to become `Send` and `Sync`.
struct UnsafeSendSync<T>(T);

unsafe impl<T> Send for UnsafeSendSync<T> {}
unsafe impl<T> Sync for UnsafeSendSync<T> {}

/// Global task manager.
static TASK_MANAGER: OnceCell<TaskManager> = OnceCell::new();

/// Returns a reference to the global task manager.
pub fn tasks() -> &'static TaskManager {
    TASK_MANAGER
        .get()
        .expect("task manager not initialized (call task::init() first)")
}

/// Intializes the global task manager.
pub fn init(runtime: runtime::Handle) {
    TASK_MANAGER
        .set(TaskManager::new(runtime))
        .ok()
        .expect("task manager already initialized");
}

pub struct TaskManager {
    /// Number of currently running tasks.
    running: Arc<AtomicUsize>,
    /// Notify handle, woken every time an _async_ task stops.
    notify: Arc<Notify>,
    /// Handle to the Tokio runtime.
    runtime: runtime::Handle,
    /// Queue of tasks to run on the ticking thread.
    sync_tx: flume::Sender<SyncFn>,
    sync_rx: Mutex<flume::Receiver<SyncFn>>,
}

impl TaskManager {
    fn new(runtime: runtime::Handle) -> Self {
        let (sync_tx, sync_rx) = flume::bounded(16);
        let sync_rx = Mutex::new(sync_rx);
        Self {
            running: Arc::new(AtomicUsize::new(0)),
            notify: Arc::new(Notify::new()),
            runtime,
            sync_tx,
            sync_rx,
        }
    }

    /// Spawns an asynchronous task. The task is guaranteed
    /// to finish before the server shuts down.
    pub fn spawn<F>(&self, f: F) -> JoinHandle<F::Output>
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.running.fetch_add(1, Ordering::AcqRel);
        let notify = Arc::clone(&self.notify);
        let running = Arc::clone(&self.running);
        self.runtime.spawn(async move {
            let ret = f.await;
            running.fetch_sub(1, Ordering::AcqRel);
            notify.notify();
            ret
        })
    }

    /// Spawns a synchronous task with access to game state. The task
    /// will run on the next tick cycle.
    pub fn sync<F, R>(&self, f: F) -> oneshot::Receiver<R>
    where
        F: FnOnce(&mut Game, &mut World) -> R + Send + 'static,
        R: Send + 'static,
    {
        let (tx, rx) = oneshot::channel();

        self.sync_tx
            .send(SyncFn::Owned(Box::new(move |game, world| {
                let _ = tx.send(f(game, world));
            })))
            .ok()
            .unwrap();

        rx
    }

    /// Runs a "scoped" synchronous task with access to game state.
    ///
    /// The task will run on the next tick cycle. Afterward, the
    /// returned future will complete with the return value.
    #[allow(clippy::let_and_return)] // weird rustc bug causes compile error
    pub async fn scoped<'a, F, R>(&self, mut f: F) -> R
    where
        F: FnMut(&mut Game, &mut World) -> R + Send + 'a,
        R: Send + 'a,
    {
        // EXTREMELY UNSAFE IMPLEMENTATION.
        // Please audit.
        let notify = Notify::new();
        let notify_ptr = UnsafeSendSync(&notify as *const Notify);

        let mut return_value = MaybeUninit::<R>::uninit();

        let return_value_ptr = UnsafeSendSync(return_value.as_mut_ptr());

        // Dummy closure used to write the return value into `return_value`.
        let mut dummy = move |game: &mut Game, world: &mut World| {
            let ret = f(game, world);
            unsafe {
                return_value_ptr.0.write(ret);
                (&*notify_ptr.0).notify();
            }
        };

        // Erase the lifetime of `dummy`.
        // This is legal because we ensure it
        // is not dropped until it completes, and any
        // references remain valid because this stack
        // frame remains intact.
        let dummy = UnsafeSendSync((&mut dummy) as *mut (dyn FnMut(&mut Game, &mut World) + 'a));
        let dummy = unsafe {
            transmute::<
                UnsafeSendSync<*mut (dyn FnMut(&mut Game, &mut World) + 'a)>,
                UnsafeSendSync<*mut (dyn FnMut(&mut Game, &mut World) + 'static)>,
            >(dummy)
        };

        // Submit the dummy function to the queue.
        self.sync_tx.send(SyncFn::Scoped(dummy.0)).ok().unwrap();

        // Wait for the task to complete.
        // This ensures that all the pointers above remain
        // valid until `dummy` is called.
        notify.notified().await;

        // Return value was written by `dummy`.
        unsafe { return_value.assume_init() }
    }

    /// Executes all queued sync tasks.
    pub fn flush_sync(&self, game: &mut Game, world: &mut World) {
        let sync_rx = self.sync_rx.lock();
        while let Ok(task) = sync_rx.try_recv() {
            match task {
                SyncFn::Owned(f) => f(game, world),
                SyncFn::Scoped(f) => {
                    let f = unsafe { &mut *f };
                    f(game, world);
                }
            }
        }
    }

    /// Waits until all tasks have completed.
    pub async fn wait(&self) {
        loop {
            // TODO: less naive approach?
            if self.running.load(Ordering::Acquire) == 0 {
                return;
            }

            self.notify.notified().await;
        }
    }
}

/// System to run sync-queued tasks.
#[fecs::system]
pub fn run_sync_tasks(game: &mut Game, world: &mut World) {
    tasks().flush_sync(game, world);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn task_spawning() {
        init(runtime::Handle::current());

        assert_eq!(tasks().spawn(async { 1 }).await.ok(), Some(1));
    }
}
