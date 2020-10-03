use std::{
    sync::atomic::AtomicUsize, sync::atomic::Ordering, sync::Arc, thread, time::Duration,
    time::Instant,
};

use async_executor::{Executor, Task};
use async_io::Timer;
use event_listener::Event;
use futures_lite::{future, future::block_on, Future, FutureExt};
use thread::sleep;

#[derive(Debug)]
struct ShutdownSignal;

/// A pool of threads for running compute tasks.
///
/// Compute tasks can be arbitrary `Future`s. Compute tasks
/// are allowed to do long-running compute operations but
/// should not block on IO or channel operations unless
/// they use `async`/`await` to do so.
///
/// This pool can be cheaply cloned as if it were an `Arc`.
#[derive(Clone)]
pub struct ComputePool {
    inner: Arc<AbstractPool>,
}

impl ComputePool {
    /// Creates a new `ComputePool` with the specified number of threads.
    pub fn new(threads: usize) -> Self {
        let pool = ComputePool {
            inner: Arc::new(AbstractPool::new()),
        };

        for i in 0..threads {
            pool.inner
                .spawn_thread(None, |_| true, format!("compute thread #{}", i));
        }

        pool
    }

    /// Spawns a task on the compute pool.
    ///
    /// Note that this function accepts a `Future`. That means
    /// you have to use an `async` block instead of a normal closure
    /// to pass the task.
    ///
    /// Compute tasks are allowed to do long-running compute operations but
    /// should not block on IO or channel operations unless
    /// they use `async`/`await` to do so.
    pub fn spawn<T: Send + 'static>(
        &self,
        task: impl Future<Output = T> + Send + 'static,
    ) -> Task<T> {
        self.inner.spawn_task(task)
    }

    /// Shuts down the pool, waiting for threads to stop.
    pub fn shut_down(&self) {
        log::debug!("Shutting down compute pool");
        self.inner.shut_down();
    }
}

/// A pool of threads for running blocking IO operations.
/// For example, file IO and blocking network requests
/// should run on this pool.
///
/// This pool will dynamically resize its thread count, up
/// to a fixed maximum number of threads.
///
/// This pool can be cheaply cloned as if it were an `Arc`.
#[derive(Clone)]
pub struct BlockingPool {
    inner: Arc<AbstractPool>,
    thread_counter: Arc<AtomicUsize>,
}

/// Minimum number of threads to keep alive in the blocking pool.
const MIN_THREADS: usize = 2;
/// Max number of threads that may be alive in the blocking pool.
const MAX_THREADS: usize = 512;
/// After this duration of idling, a thread will attempt to exit.
const MAX_IDLE_TIME: Duration = Duration::from_secs(30);

impl BlockingPool {
    /// Creates a new `BlockingPool`.
    pub fn new() -> Self {
        let pool = BlockingPool {
            inner: Arc::new(AbstractPool::new()),
            thread_counter: Arc::new(AtomicUsize::new(1)),
        };

        for _ in 0..MIN_THREADS {
            pool.spawn_thread();
        }

        pool
    }

    fn spawn_thread(&self) {
        self.inner.spawn_thread(
            Some(MAX_IDLE_TIME),
            |pool| pool.num_threads() > MIN_THREADS,
            format!(
                "blocking thread #{}",
                self.thread_counter.fetch_add(1, Ordering::AcqRel)
            ),
        );
    }

    /// Spawns a task on the blocking pool.
    ///
    /// The task is allowed to block on IO operations, even
    /// if it doesn't use `async`/`await` to do so (so you
    /// may run e.g. file operations on this pool.)
    ///
    /// Blocking tasks should not, however, do expensive compute
    /// operations.
    pub fn spawn<T: Send + 'static>(
        &self,
        task: impl Future<Output = T> + Send + 'static,
    ) -> Task<T> {
        // Spawn another thread if current threads are occupied.
        if self.inner.num_tasks() > self.inner.num_threads()
            && self.inner.num_threads() < MAX_THREADS
        {
            self.spawn_thread();
        }

        self.inner.spawn_task(task)
    }

    /// Shuts down the pool, waiting for all tasks to complete.
    pub fn shut_down(&self) {
        log::debug!("Shutting down blocking pool");
        self.inner.shut_down();
    }
}

/// A pool of threads running async tasks.
#[derive(Default)]
struct AbstractPool {
    /// Executor which runs tasks.
    executor: Executor<'static>,
    /// The number of threads currently active.
    num_threads: AtomicUsize,
    /// The number of tasks currently running.
    num_tasks: AtomicUsize,
    /// Event which is notified when the pool should shut down.
    shutdown: Event,
}

impl AbstractPool {
    pub fn new() -> Self {
        Self::default()
    }

    /// Spawns a new task on this pool.
    pub fn spawn_task<T: Send + 'static>(
        self: &Arc<Self>,
        task: impl Future<Output = T> + Send + 'static,
    ) -> Task<T> {
        self.num_tasks.fetch_add(1, Ordering::Release);
        let this = Arc::clone(self);
        self.executor.spawn(async move {
            let ret = task.await;
            this.num_tasks.fetch_sub(1, Ordering::Release);
            ret
        })
    }

    /// Shuts down this pool, closing all threads.
    pub fn shut_down(&self) {
        self.shutdown.notify(usize::MAX);

        // Finish executing remaining tasks.
        let start = Instant::now();
        while self.num_tasks() > 0 {
            self.executor.try_tick();

            sleep(Duration::from_millis(50));

            if start.elapsed().as_secs() > 10 {
                log::error!("Tasks failed to complete within 10 seconds");
                break;
            }
        }
    }

    /// Returns the current number of active threads.
    pub fn num_threads(&self) -> usize {
        self.num_threads.load(Ordering::Acquire)
    }

    /// Returns the current number of running tasks.
    pub fn num_tasks(&self) -> usize {
        self.num_tasks.load(Ordering::Acquire)
    }

    /// Spawns a new thread on this pool.
    pub fn spawn_thread(
        self: &Arc<Self>,
        idle_timeout: Option<Duration>,
        mut idle_condition: impl FnMut(&Self) -> bool + Send + 'static,
        name: String,
    ) {
        self.num_threads.fetch_add(1, Ordering::Release);
        let this = Arc::clone(self);
        thread::Builder::new()
            .name(name.clone())
            .spawn(move || {
                log::debug!("Started {}", name);
                let shutdown = async {
                    this.shutdown.listen().await;
                    true
                };
                futures_lite::pin!(shutdown);

                loop {
                    let should_exit = block_on(
                        async {
                            this.executor.tick().await;
                            false
                        }
                        .or(&mut shutdown)
                        .or(async {
                            if let Some(timeout) = idle_timeout {
                                Timer::after(timeout).await;
                                idle_condition(&this)
                            } else {
                                future::pending::<bool>().await
                            }
                        }),
                    );

                    if should_exit {
                        log::debug!("Closing {}", name);
                        this.num_threads.fetch_sub(1, Ordering::Release);
                        break;
                    }
                }
            })
            .expect("failed to spawn pool thread");
    }
}
