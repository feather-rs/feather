//! A simple thread pool implementation.

use std::thread;

use flume::Sender;

/// A simple thread pool.
///
/// This struct can be cloned to create
/// new handles to the pool (like an `Arc`).
#[derive(Clone)]
pub struct ThreadPool {
    sender: Sender<Task>,
}

impl ThreadPool {
    /// Creates a thread pool.
    ///
    /// `num_threads` threads are spawned. Each thread's name
    /// is computed from the given `name`.
    pub fn new(name: &str, num_threads: usize) -> Self {
        let (sender, receiver) = flume::unbounded::<Task>();
        for i in 0..num_threads {
            let receiver = receiver.clone();
            thread::Builder::new()
                .name(format!("{} #{}", name, i + 1))
                .spawn(move || {
                    for task in receiver {
                        (task)();
                    }
                })
                .expect("failed to spawn thread");
        }
        Self { sender }
    }

    /// Spawns a task on the thread pool.
    pub fn spawn(&self, task: impl FnOnce() + Send + 'static) {
        self.sender.send(Box::new(task)).ok();
    }
}

type Task = Box<dyn FnOnce() + Send>;
