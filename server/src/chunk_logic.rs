//! Module for interacting with the chunk worker thread
//! from the server threads.
//!
//! Also handles unloading chunks when unused.
use crossbeam::channel::{Receiver, Sender};
use std::sync::atomic::{AtomicU32, Ordering};

use feather_core::world::ChunkPosition;

use rayon::prelude::*;

use crate::game::Game;
use crate::{chunk_worker, current_time_in_millis, TPS};
use feather_core::entity::EntityData;
use feather_core::Chunk;
use fecs::{Entity, World};
use hashbrown::HashSet;
use multimap::MultiMap;
use std::collections::VecDeque;
use std::sync::Arc;

/// A handle for interacting with the chunk
/// worker thread.
#[derive(Debug, Clone)]
pub struct ChunkWorkerHandle {
    pub sender: Sender<chunk_worker::Request>,
    pub receiver: Receiver<chunk_worker::Reply>,
}

/// Event which is triggered when a chunk is loaded.
#[derive(Debug, Clone)]
pub struct ChunkLoadEvent {
    pub pos: ChunkPosition,
    pub entities: Vec<EntityData>,
}

/// Event which is triggered when a chunk fails to load.
#[derive(Debug, Clone, Copy)]
pub struct ChunkLoadFailEvent {
    pub pos: ChunkPosition,
}

/// System for receiving loaded chunks from the chunk worker thread.
#[system]
pub fn chunk_load(game: &mut Game, world: &mut World) {
    while let Ok(reply) = game.chunk_worker_handle.receiver.try_recv() {
        if let chunk_worker::Reply::LoadedChunk(pos, result) = reply {
            match result {
                Ok((chunk, _entities)) => {
                    game.chunk_map.insert(chunk);

                    game.on_chunk_load(world, pos);

                    // TODO: entities

                    trace!("Loaded chunk at {:?}", pos);
                }
                Err(err) => {
                    warn!("Failed to load chunk at {:?}: {}", pos, err);
                    game.on_chunk_load_fail(world, pos);
                }
            }
        }
    }
}

/// The chunk holder map contains a mapping
/// of chunk positions to any number of entities, called "holders."
/// When a chunk position has no holders, it will be queued
/// for unloading.
///
/// In addition, the chunk holders map can be used to select
/// which players to broadcast an entity movement to: a player
/// who has a chunk hold on the entity's chunk would be able to see
/// the movement, while other players would be outside of the view
/// distance. This technique allows for higher performance and
/// avoids constant nearby entity queries.
#[derive(Default, Clone, Debug)]
pub struct ChunkHolders {
    inner: MultiMap<ChunkPosition, Entity>,
}

impl ChunkHolders {
    pub fn holders_for(&self, chunk: ChunkPosition) -> &[Entity] {
        self.inner
            .get_vec(&chunk)
            .map(|holders| holders.as_slice())
            .unwrap_or(&[])
    }

    pub fn chunk_has_holders(&self, chunk: ChunkPosition) -> bool {
        let holders = self.holders_for(chunk);

        !holders.is_empty()
    }

    pub fn insert_holder(&mut self, chunk: ChunkPosition, holder: Entity) {
        self.inner.insert(chunk, holder);
    }
}
pub fn remove_chunk_holder(game: &mut Game, chunk: ChunkPosition, holder: Entity) {
    if let Some(vec) = game.chunk_holders.inner.get_vec_mut(&chunk) {
        let index = vec.iter().position(|e| *e == holder);
        if let Some(index) = index {
            vec.remove(index);

            game.on_chunk_holder_release(chunk, holder);
        }
    }
}

/// The queue of chunks to be unloaded.
/// See `chunk_unload` for details.
#[derive(Clone, Debug, Default)]
pub struct ChunkUnloadQueue {
    /// The internal queue.
    queue: VecDeque<ChunkUnload>,
}

/// A chunk to be unloaded.
#[derive(Clone, Debug, Default)]
struct ChunkUnload {
    /// The position of this chunk.
    chunk: ChunkPosition,
    /// The tick count at which to unload the chunk.
    time: u64,
}

/// The amount of time, in ticks, between the time
/// a chunk is queued for unloading and when it is unloaded.
const CHUNK_UNLOAD_TIME: u64 = TPS * 5; // 5 seconds - TODO make this configurable

/// System for unloading chunks when they have no holders.
/// This system through chunks which are currently
/// queued to be loaded and unloads them if the
/// period of time has elapsed.
///
/// Chunks are not unloaded immediately after having
/// no holders because doing so could open up
/// opportunities for exploits. For example, a player
/// could quickly move between chunk boundaries, causing
/// chunks at the edge of their view distance
/// to be loaded and unloaded at an alarming rate.
#[system]
pub fn chunk_unload(game: &mut Game) {
    // Unload chunks which are finished in the queue.

    // Since chunks are queued in the back and taken out
    // from the front, the chunks in the front of the vector
    // were queued the longest time ago. Because of this,
    // we go through the unloads in the front of the queue
    // to find which chunks to unload.
    while let Some(unload) = game.chunk_unload_queue.queue.front() {
        if game.tick_count >= unload.time {
            // Don't unload if new chunk holders have appeared.
            if game.chunk_holders.chunk_has_holders(unload.chunk) {
                game.chunk_unload_queue.queue.pop_front();
                continue;
            }

            // Unload chunk and pop from queue.
            game.chunk_map.remove(unload.chunk);
            game.chunk_unload_queue.queue.pop_front();
        } else {
            // We're done - all chunks farther up in
            // the queue were queued before this one,
            // so it isn't time to unload any of those.
            break;
        }
    }
}

/// Event handler which handles holder release events. If
/// a chunk has no more holders, then a chunk unload is queued.
pub fn on_chunk_holder_release_unload_chunk(game: &mut Game, chunk: ChunkPosition) {
    // Handle holder release events.
    // If the chunk now has zero holders, queue it for unloading.
    if !game.chunk_holders.chunk_has_holders(chunk) {
        let unload = ChunkUnload {
            chunk,
            time: game.tick_count + CHUNK_UNLOAD_TIME,
        };
        game.chunk_unload_queue.queue.push_back(unload);
    }
}

/// Component which stores which
/// chunks a given entity has a holder
/// on.
///
/// Although this information is also
/// stored in the `ChunkHolders` resource,
/// using this component allows for efficiently
/// finding which chunks a given entity has
/// a hold on, rather than having
/// to linear search all chunks (obviously ridiculous).
#[derive(Default)]
pub struct ChunkHolder {
    pub holds: HashSet<ChunkPosition>,
}

impl ChunkHolder {
    pub fn new() -> Self {
        Self::default()
    }
}

/// System for removing an entity's chunk holds
/// once it is destroyed.
pub fn on_entity_despawn_remove_chunk_holder(game: &mut Game, world: &mut World, entity: Entity) {
    // If entity had chunk holds, remove them all
    if let Some(holder_comp) = world.try_get::<ChunkHolder>(entity) {
        debug!("Removing chunk holds for entity {:?}", entity);
        holder_comp.holds.iter().for_each(|chunk| {
            remove_chunk_holder(game, *chunk, entity);
        });
    }
}

/// The interval, in ticks, at which
/// chunks will be optimized.
const CHUNK_OPTIMIZE_INTERVAL: u64 = TPS * 60 * 5; // 5 minutes

/// System which optimizes chunks periodically.
/// This allows for more efficient memory use
/// at the cost of the occasional CPU spike
/// when optimization happens.
///
/// For optimal performance, this system is fully
/// concurrent - each chunk optimization is split
/// into a separate job and fed into `rayon`.
#[system]
pub fn chunk_optimize(game: &mut Game) {
    // Only run every CHUNK_OPTIMIZE_INTERVAL ticks
    if game.tick_count % CHUNK_OPTIMIZE_INTERVAL != 0 {
        return;
    }

    debug!("Optimizing chunks");

    let start_time = current_time_in_millis();
    let count = AtomicU32::new(0);

    game.chunk_map.par_iter_chunks().for_each(|chunk| {
        count.fetch_add(chunk.write().optimize(), Ordering::Relaxed);
    });

    let end_time = current_time_in_millis();
    let elapsed = end_time - start_time;

    debug!(
        "Optimized {} chunk sections (took {}ms - {:.2}ms/section)",
        count.load(Ordering::Relaxed),
        elapsed,
        elapsed as f64 / f64::from(count.load(Ordering::Relaxed))
    );
}

/// Adds a hold for a chunk for the given entity.
pub fn hold_chunk(game: &mut Game, holder: &mut ChunkHolder, chunk: ChunkPosition, entity: Entity) {
    holder.holds.insert(chunk);
    game.chunk_holders.inner.insert(chunk, entity);
}

/// Releases a hold for a chunk for the given entity.
pub fn release_chunk(game: &mut Game, world: &mut World, chunk: ChunkPosition, entity: Entity) {
    let mut holder = world.get_mut::<ChunkHolder>(entity);
    holder.holds.remove(&chunk);
    if let Some(vec) = game.chunk_holders.inner.get_vec_mut(&chunk) {
        let mut index = None;
        for (i, e) in vec.iter().enumerate() {
            if *e == entity {
                index = Some(i);
            }
        }

        if let Some(index) = index {
            vec.swap_remove(index);
        }
    }
    game.on_chunk_holder_release(chunk, entity);
}

/// Asynchronously loads the chunk at the given position.
/// At some point in time after this function is called,
/// the chunk will appear in the chunk map.
///
/// In the event that the requested chunk does not exist
/// in the world save, it will be generated asynchronously.
pub fn load_chunk(handle: &ChunkWorkerHandle, pos: ChunkPosition) {
    // Send request to chunk worker thread
    handle
        .sender
        .send(chunk_worker::Request::LoadChunk(pos))
        .unwrap();
}

/// Asynchronously saves the chunk at the given position.
pub fn save_chunk(handle: &ChunkWorkerHandle, chunk: Arc<Chunk>, entities: Vec<EntityData>) {
    handle
        .sender
        .send(chunk_worker::Request::SaveChunk(chunk, entities))
        .unwrap();
}
