use std::{
    collections::hash_map::Entry,
    path::PathBuf,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use ahash::AHashMap;
use feather_world_format::RegionHandle;
use flume::{Receiver, RecvTimeoutError, Sender};
use parking_lot::RwLock;
use quill::{
    libcraft::{biome::BiomeList, RegionPosition, Sections},
    WorldId,
};

struct ThreadHandle {
    num_open_regions: AtomicUsize,
    sender: Sender<Task>,
}

/// Manages threads used for region IO.
///
/// The thread pool is shared between all worlds using the `FregWorldSource`.
pub struct RegionThreadPool {
    threads: Vec<Arc<ThreadHandle>>,
    threads_by_key: AHashMap<RegionKey, usize>,
    shared: Arc<Shared>,
}

impl RegionThreadPool {
    pub fn new(shared: Shared, num_threads: usize) -> Self {
        let mut threads = Vec::new();
        let shared = Arc::new(shared);

        for i in 0..num_threads {
            let (sender, receiver) = flume::unbounded();
            let handle = Arc::new(ThreadHandle {
                sender,
                num_open_regions: AtomicUsize::new(0),
            });

            let thread = WorkerThread::new(Arc::clone(&shared), Arc::clone(&handle));
            std::thread::Builder::new()
                .name(format!("Region IO Thread #{}", i))
                .spawn(move || {
                    thread.run(receiver);
                })
                .expect("failed to create region IO thread");

            threads.push(handle);
        }

        Self {
            threads,
            threads_by_key: AHashMap::new(),
            shared,
        }
    }

    pub fn register_world(&self, id: WorldId, info: WorldInfo) {
        self.shared.worlds.write().insert(id, info);
    }

    pub fn spawn(
        &mut self,
        key: RegionKey,
        task: impl FnOnce(anyhow::Result<&mut RegionHandle>) + Send + 'static,
    ) {
        let thread = self.threads_by_key.entry(key).or_insert_with(|| {
            self.threads
                .iter()
                .enumerate()
                .min_by_key(|(_, thread)| thread.num_open_regions.load(Ordering::Relaxed))
                .expect("no threads available")
                .0
        });

        self.threads[*thread]
            .num_open_regions
            .fetch_add(1, Ordering::Relaxed);

        self.threads[*thread]
            .sender
            .send(Task {
                key,
                op: Box::new(task),
            })
            .expect("thread shut down?");
    }
}

pub struct WorldInfo {
    pub sections: Sections,
    pub min_y: i32,
    pub dir: PathBuf,
}

pub struct Shared {
    pub biomes: Arc<BiomeList>,
    worlds: RwLock<AHashMap<WorldId, WorldInfo>>,
}

impl Shared {
    pub fn new(biomes: Arc<BiomeList>) -> Self {
        Self {
            biomes,
            worlds: RwLock::new(AHashMap::new()),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RegionKey {
    pub region: RegionPosition,
    pub world: WorldId,
}

struct Task {
    key: RegionKey,
    op: Box<dyn FnOnce(anyhow::Result<&mut RegionHandle>) + Send>,
}

struct WorkerThread {
    shared: Arc<Shared>,
    open_regions: AHashMap<RegionKey, OpenRegion>,
    handle: Arc<ThreadHandle>,
}

struct OpenRegion {
    handle: RegionHandle,
    last_used: Instant,
}

const FLUSH_INTERVAL: Duration = Duration::from_secs(60);
const REGION_CACHE_TIME: Duration = Duration::from_secs(60);

impl WorkerThread {
    pub fn new(shared: Arc<Shared>, handle: Arc<ThreadHandle>) -> Self {
        Self {
            shared,
            open_regions: AHashMap::new(),
            handle,
        }
    }

    pub fn run(mut self, receiver: Receiver<Task>) {
        let mut next_flush = Instant::now() + FLUSH_INTERVAL;
        log::debug!("Region worker thread started");
        loop {
            match receiver.recv_deadline(next_flush) {
                Ok(task) => self.do_task(task),
                Err(RecvTimeoutError::Timeout) => {
                    self.flush_unused_regions();
                    next_flush = Instant::now() + FLUSH_INTERVAL;
                }
                Err(RecvTimeoutError::Disconnected) => {
                    log::debug!("Region worker thread shutting down");
                    return;
                }
            }

            self.handle
                .num_open_regions
                .store(self.open_regions.len(), Ordering::Relaxed);
        }
    }

    fn do_task(&mut self, task: Task) {
        let mut region = match self.open_regions.entry(task.key) {
            Entry::Occupied(e) => Ok(e.into_mut()),
            Entry::Vacant(v) => {
                log::debug!("Opening region with key {:?}", v.key());
                let worlds = self.shared.worlds.read();
                let world = worlds
                    .get(&v.key().world)
                    .expect("unregistered world on region threadpool");
                let region = RegionHandle::open(
                    &world.dir,
                    v.key().region,
                    true,
                    Arc::clone(&self.shared.biomes),
                    world.sections,
                    world.min_y,
                );
                let region = region.map(|handle| OpenRegion {
                    handle,
                    last_used: Instant::now(),
                });
                region.map(|r| v.insert(r))
            }
        };

        if let Ok(region) = &mut region {
            region.last_used = Instant::now();
        }

        (task.op)(region.map(|r| &mut r.handle));
    }

    fn flush_unused_regions(&mut self) {
        let now = Instant::now();
        let initial_len = self.open_regions.len();
        self.open_regions
            .retain(|_, region| now - region.last_used < REGION_CACHE_TIME);
        if initial_len != self.open_regions.len() {
            let closed = initial_len - self.open_regions.len();
            log::debug!(
                "Closed {} region files ({} left open on this thread)",
                closed,
                self.open_regions.len()
            );
        }
    }
}
