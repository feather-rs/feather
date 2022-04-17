use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::Chunk;
use anyhow::bail;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};

pub type ChunkHandle = Arc<ChunkLock>;

/// A wrapper around a RwLock. Cannot be locked for writing when unloaded.
/// This structure exists so that a chunk can be read from even after being unloaded without accidentaly writing to it.
#[derive(Debug)]
pub struct ChunkLock {
    loaded: AtomicBool,
    lock: RwLock<Chunk>,
}

impl ChunkLock {
    pub fn new(chunk: Chunk) -> Self {
        Self {
            loaded: AtomicBool::new(true),
            lock: RwLock::new(chunk),
        }
    }

    /// Returns whether the chunk is loaded.
    pub fn is_loaded(&self) -> bool {
        self.loaded.load(Ordering::SeqCst)
    }

    /// Attempts to set the chunk as unloaded. Returns an error if the chunk is locked as writable.
    pub fn set_unloaded(&self) -> anyhow::Result<()> {
        if self.loaded.swap(false, Ordering::SeqCst) {
            // FIXME: Decide what to do when unloading an unloaded chunk
        }
        if self.lock.try_read().is_none() {
            // Locking fails when someone else already owns a write lock
            bail!("Cannot unload chunk because it is locked as writable!")
        }
        Ok(())
    }

    /// Sets the chunk as loaded and returns the previous state.
    pub fn set_loaded(&self) -> bool {
        self.loaded.swap(true, Ordering::SeqCst)
    }

    /// Locks this chunk with read access. Doesn't block.
    /// Returns None if the chunk is unloaded or locked for writing, Some otherwise.
    pub fn try_read(&self) -> Option<RwLockReadGuard<Chunk>> {
        self.lock.try_read()
    }

    /// Locks this chunk with read access, blocking the current thread until it can be acquired.
    pub fn read(&self) -> RwLockReadGuard<Chunk> {
        self.lock.read()
    }

    /// Locks this chunk with exclusive write access. Doesn't block.
    /// Returns None if the chunk is unloaded or locked already, Some otherwise.
    pub fn try_write(&self) -> Option<RwLockWriteGuard<Chunk>> {
        if self.is_loaded() {
            self.lock.try_write()
        } else {
            None
        }
    }

    /// Locks this chunk with exclusive write acccess, blocking the current thread until it can be acquired.
    /// Returns None if the chunk is unloaded, Some otherwise.
    pub fn write(&self) -> Option<RwLockWriteGuard<Chunk>> {
        if self.is_loaded() {
            Some(self.lock.write())
        } else {
            None
        }
    }

    pub fn is_locked(&self) -> bool {
        self.lock.is_locked()
    }
}

#[cfg(test)]
mod tests {
    use crate::world::Sections;
    use crate::ChunkPosition;
    use std::{
        thread::{sleep, spawn, JoinHandle},
        time::Duration,
    };

    use super::*;

    fn empty_lock(x: i32, z: i32) -> ChunkLock {
        ChunkLock::new(Chunk::new(ChunkPosition::new(x, z), Sections(16), 0))
    }

    #[test]
    fn normal_function() {
        let lock = empty_lock(0, 0);
        for _ in 0..100 {
            // It should be possible to lock in any way
            if rand::random::<bool>() {
                let _guard = lock.try_read().unwrap();
            } else {
                let _guard = lock.try_write().unwrap();
            }
        }
    }

    #[test]
    fn cannot_write_unloaded() {
        let lock = empty_lock(0, 0);
        lock.set_unloaded();
        assert!(lock.try_write().is_none())
    }

    #[test]
    fn can_read_unloaded() {
        let lock = empty_lock(0, 0);
        lock.set_unloaded();
        assert!(lock.try_read().is_some())
    }

    #[test]
    fn multithreaded() {
        let lock = Arc::new(empty_lock(0, 0));
        let mut handles: Vec<JoinHandle<()>> = vec![];
        for _ in 0..20 {
            let l = lock.clone();
            handles.push(spawn(move || {
                while let Some(guard) = l.write() {
                    sleep(Duration::from_millis(10));
                    drop(guard)
                }
            }))
        }
        sleep(Duration::from_millis(1000));
        lock.set_unloaded().unwrap_or(()); // Discard error
        for h in handles {
            h.join().unwrap() // Wait for all threads to stop
        }
    }
}
