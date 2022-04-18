use std::{collections::VecDeque, sync::Arc, time::Instant};

use ahash::AHashSet;
use libcraft::{
    dimension::DimensionInfo, BlockPosition, BlockState, Chunk, ChunkPosition, WorldHeight,
};
use quill::{
    events::ChunkLoadEvent,
    saveload::{ChunkLoadError, StoredChunk, WorldSource},
    world::{ChunkNotLoaded, ChunkTicket, WorldDescriptor, WorldSaveStrategy, WorldSettings},
    ChunkHandle, ChunkLock, World as _, WorldId,
};

use crate::Game;

use self::{chunk_cache::ChunkCache, chunk_map::ChunkMap, tickets::ChunkTickets};

mod chunk_cache;
mod chunk_map;
pub mod systems;
mod tickets;

pub struct World {
    id: WorldId,
    name: Option<String>,

    settings: WorldSettings,

    dimension_info: DimensionInfo,

    chunk_map: ChunkMap,
    chunk_cache: ChunkCache,
    tickets: ChunkTickets,
    unload_queue: UnloadQueue,

    source: Box<dyn WorldSource>,
    loading_chunks: AHashSet<ChunkPosition>,
    canceled_chunk_loads: AHashSet<ChunkPosition>,

    /// If true, has a different void fog and horizon at y=min
    flat: bool,
}

impl World {
    pub fn new(desc: WorldDescriptor, _game: &Game) -> Self {
        assert_eq!(
            desc.dimension_info.info.height % 16,
            0,
            "world height must be a multiple of the chunk section height (16)"
        );

        if matches!(
            &desc.settings.save_strategy,
            &WorldSaveStrategy::SaveIncrementally { .. }
        ) {
            assert!(
                desc.source.supports_saving(),
                "world is configured to save incrementally, but its source doesn't support saving!"
            );
        }

        Self {
            name: desc.name,
            id: desc.id,
            settings: desc.settings,
            chunk_map: ChunkMap::new(
                WorldHeight(desc.dimension_info.info.height as usize),
                desc.dimension_info.info.min_y,
            ),
            chunk_cache: ChunkCache::new(),
            tickets: ChunkTickets::new(),
            unload_queue: UnloadQueue::default(),
            loading_chunks: AHashSet::new(),
            canceled_chunk_loads: AHashSet::new(),
            source: desc.source,
            dimension_info: desc.dimension_info,
            flat: desc.flat,
        }
    }

    /// Should be called on each tick.
    pub fn update_chunks(&mut self) -> Vec<ChunkLoadEvent> {
        // For chunks that have lost all tickets, put them into the unload
        // queue.
        while let Some(chunk_to_unload) = self.tickets.poll_chunk_with_no_tickets() {
            self.unload_queue.push(chunk_to_unload, &self.settings);
        }

        // Unload chunks that have made their way through the unload queue.
        if !matches!(&self.settings.save_strategy, &WorldSaveStrategy::KeepLoaded) {
            let mut num_unloaded = 0;
            while let Some(chunk) = self.unload_queue.pop() {
                self.unload_chunk(chunk);
                num_unloaded += 1;
            }
            if num_unloaded != 0 {
                log::debug!(
                    "Unloaded {} chunks for world {} (total loaded chunks: {} + {} cached)",
                    num_unloaded,
                    self.display_name(),
                    self.chunk_map.len(),
                    self.chunk_cache.len()
                );
            }
        }

        self.chunk_cache.purge_old_unused();

        self.load_chunks()
    }

    /// Queues the given chunk to be loaded. If the chunk was cached, it is loaded immediately.
    fn queue_chunk_load(&mut self, pos: ChunkPosition) {
        if self.is_chunk_loaded(pos) || self.is_chunk_loading(pos) {
            return;
        }

        if let Some(chunk) = self.chunk_cache.remove(pos) {
            // Load immediately.
            // Move the chunk from the cache to the map
            chunk.set_loaded();
            self.chunk_map.insert_chunk(chunk);
        } else if self.loading_chunks.insert(pos) {
            self.source.queue_load_chunk(pos);
        }
    }

    /// Loads any chunks that have been loaded asynchronously
    /// after a call to [`World::queue_chunk_load`].
    fn load_chunks(&mut self) -> Vec<ChunkLoadEvent> {
        let mut events = Vec::new();
        while let Some(loaded) = self.source.poll_loaded_chunk() {
            self.loading_chunks.remove(&loaded.pos);
            if self.canceled_chunk_loads.remove(&loaded.pos) {
                continue;
            }

            let chunk = match loaded.result {
                Ok(chunk) => chunk.chunk,
                Err(ChunkLoadError::Missing) => {
                    log::debug!(
                        "Chunk at {:?} is missing from the world source. Using an empty chunk.",
                        loaded.pos
                    );
                    ChunkHandle::new(ChunkLock::new(Chunk::new(
                        loaded.pos,
                        self.height().into(),
                        self.min_y(),
                    )))
                }
                Err(e) => {
                    log::error!("Failed to load chunk at {:?}: {:?}", loaded.pos, e);
                    continue;
                }
            };

            self.chunk_map.insert_chunk(chunk);

            events.push(ChunkLoadEvent {
                pos: loaded.pos,
                world: self.id,
            });
            log::trace!("Loaded chunk {:?}", loaded.pos);
        }

        if !events.is_empty() {
            log::debug!(
                "Loaded {} chunks for world {} (total loaded chunks: {} + {} cached)",
                events.len(),
                self.display_name(),
                self.chunk_map.len(),
                self.chunk_cache.len()
            );
        }

        events
    }

    /// Unloads the given chunk.
    fn unload_chunk(&mut self, pos: ChunkPosition) {
        if matches!(&self.settings.save_strategy, &WorldSaveStrategy::KeepLoaded) {
            // Don't ever unload chunks.
            return;
        }

        if let Some(handle) = self.chunk_map.remove_chunk(pos) {
            // TODO: handle case where chunk is being written on a separate
            // thread
            handle.set_unloaded().ok();

            if !matches!(
                &self.settings.save_strategy,
                &WorldSaveStrategy::DropChanges
            ) {
                self.source.queue_save_chunk(StoredChunk {
                    pos,
                    chunk: Arc::clone(&handle),
                });
            }

            self.chunk_cache.insert(pos, handle);
        }

        if self.is_chunk_loading(pos) {
            self.canceled_chunk_loads.insert(pos);
        }
    }

    fn is_chunk_loading(&self, pos: ChunkPosition) -> bool {
        self.loading_chunks.contains(&pos)
    }
}

impl quill::World for World {
    fn id(&self) -> WorldId {
        self.id
    }

    fn name(&self) -> Option<&str> {
        self.name.as_ref().map(String::as_str)
    }

    fn block_at(&self, pos: BlockPosition) -> Result<BlockState, ChunkNotLoaded> {
        self.chunk_map
            .block_at(pos)
            .ok_or_else(|| ChunkNotLoaded(pos.chunk()))
    }

    fn set_block_at(&self, pos: BlockPosition, block: BlockState) -> Result<(), ChunkNotLoaded> {
        if self.chunk_map.set_block_at(pos, block) {
            Ok(())
        } else {
            Err(ChunkNotLoaded(pos.chunk()))
        }
    }

    fn chunk_handle_at(&self, pos: ChunkPosition) -> Result<ChunkHandle, ChunkNotLoaded> {
        self.chunk_map
            .chunk_handle_at(pos)
            .ok_or_else(|| ChunkNotLoaded(pos))
    }

    fn is_chunk_loaded(&self, pos: ChunkPosition) -> bool {
        self.chunk_map.contains(pos)
    }

    fn dimension_info(&self) -> &DimensionInfo {
        &self.dimension_info
    }

    fn create_chunk_ticket(&mut self, chunk: ChunkPosition) -> ChunkTicket {
        let ticket = self.tickets.create_ticket(chunk);
        self.queue_chunk_load(chunk);
        self.unload_queue.remove(chunk);
        ticket
    }

    fn is_persistent(&self) -> bool {
        self.source.supports_saving()
    }

    fn is_flat(&self) -> bool {
        self.flat
    }
}

#[derive(Debug, Copy, Clone)]
struct QueuedChunkUnload {
    pos: ChunkPosition,
    time: Instant,
}

#[derive(Default)]
struct UnloadQueue {
    entries: VecDeque<QueuedChunkUnload>,
}

impl UnloadQueue {
    pub fn push(&mut self, chunk: ChunkPosition, settings: &WorldSettings) {
        self.remove(chunk);
        let time = Instant::now() + settings.unload_delay;
        self.entries
            .push_back(QueuedChunkUnload { pos: chunk, time });
    }

    pub fn remove(&mut self, chunk: ChunkPosition) {
        self.entries.retain(|e| e.pos != chunk)
    }

    pub fn pop(&mut self) -> Option<ChunkPosition> {
        match self.entries.get(0).copied() {
            Some(entry) => {
                if entry.time < Instant::now() {
                    self.entries.remove(0);
                    Some(entry.pos)
                } else {
                    None
                }
            }
            None => None,
        }
    }
}
