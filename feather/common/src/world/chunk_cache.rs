use std::{collections::VecDeque, sync::Arc};

use ahash::AHashMap;
use libcraft::ChunkPosition;
use quill::ChunkHandle;

/// This struct contains chunks that were unloaded but remain in memory, either
/// because there are still handles to them (`Arc`s) or because they have not
/// yet completed saving.
#[derive(Default)]
pub struct ChunkCache {
    map: AHashMap<ChunkPosition, (ChunkHandle, bool)>, // handle + was saved
    unload_queue: VecDeque<ChunkPosition>,
}

impl ChunkCache {
    pub fn new() -> Self {
        Self {
            map: AHashMap::new(),
            unload_queue: VecDeque::new(),
        }
    }

    fn ref_count(&self, pos: &ChunkPosition) -> Option<usize> {
        self.map.get(pos).map(|(arc, _)| Arc::strong_count(arc))
    }

    /// Purges all chunks in the cache that have no remaining references
    /// and have been saved.
    pub fn purge_old_unused(&mut self) -> usize {
        let mut processed = 0;
        let mut purged = 0;
        let total = self.unload_queue.len();

        while let Some(pos) = self.unload_queue.pop_front() {
            if !self.map.contains_key(&pos) {
                processed += 1;
                continue;
            }
            let (_handle, was_saved) = self.map.get(&pos).unwrap();

            if self.ref_count(&pos).unwrap() > 1 || !*was_saved {
                // Another copy of this handle still exists, or
                // the chunk has not yet been saved.
                self.unload_queue.push_back(pos);
            } else {
                self.map.remove_entry(&pos);
                purged += 1;
            }

            processed += 1;
            if processed >= total {
                break;
            }
        }

        purged
    }

    #[allow(unused)]
    pub fn get(&self, pos: ChunkPosition) -> Option<ChunkHandle> {
        self.map.get(&pos).map(|(c, _)| Arc::clone(c))
    }

    /// Returns whether the chunk at the position is cached.
    #[allow(unused)]
    pub fn contains(&self, pos: &ChunkPosition) -> bool {
        self.map.contains_key(pos)
    }

    /// Inserts a chunk handle into the cache, returning the previous handle if there was one.
    pub fn insert(
        &mut self,
        pos: ChunkPosition,
        handle: ChunkHandle,
        was_saved: bool,
    ) -> Option<ChunkHandle> {
        if was_saved && Arc::strong_count(&handle) == 1 {
            // No need to cache the chunk, as it's been saved
            // and has no other handles alive.
            return None;
        }

        self.unload_queue.push_back(pos);
        self.map
            .insert(pos, (handle, was_saved))
            .map(|(handle, _)| handle)
    }

    /// Marks the given chunk as saved.
    pub fn mark_saved(&mut self, pos: ChunkPosition) {
        if let Some((_, saved)) = self.map.get_mut(&pos) {
            *saved = true;
        }
    }

    /// Removes the chunk handle at the given position, returning the handle if it was cached.
    pub fn remove(&mut self, pos: ChunkPosition) -> Option<ChunkHandle> {
        self.map.remove(&pos).map(|(handle, _)| handle)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
