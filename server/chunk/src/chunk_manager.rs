//! Module for interacting with the chunk worker thread
//! from the server threads.
//!
//! Also handles unloading chunks when unused.
use crossbeam::channel::{Receiver, Sender};
use std::sync::atomic::{AtomicU32, Ordering};

use crate::chunk_worker;
use ahash::AHashSet;
use chunk_worker::ChunkSave;
use feather_core::anvil::{block_entity::BlockEntityData, entity::EntityData};
use feather_core::chunk::Chunk;
use feather_core::util::ChunkPosition;
use feather_server_types::{
    ChunkHolder, ChunkHolderReleaseEvent, ChunkLoadEvent, ChunkLoadFailEvent, ChunkUnloadEvent,
    EntityDespawnEvent, EntitySpawnEvent, Game, HoldChunkRequest, LoadChunkRequest,
    ReleaseChunkRequest, TPS,
};
use feather_server_util::current_time_in_millis;
use fecs::{Entity, World};
use parking_lot::RwLock;
use rayon::prelude::*;
use smallvec::SmallVec;
use std::collections::VecDeque;
use std::sync::Arc;

/// Set of chunks which are currently being loaded.
#[derive(Debug, Clone, Default)]
pub struct LoadingChunks(pub AHashSet<ChunkPosition>);

/// A handle for interacting with the chunk
/// worker thread.
#[derive(Debug, Clone)]
pub struct ChunkWorkerHandle {
    pub sender: Sender<chunk_worker::Request>,
    pub receiver: Receiver<chunk_worker::Reply>,
}

/// System for handling replies from the chunk worker thread.
#[fecs::system]
pub fn handle_chunk_worker_replies(
    game: &mut Game,
    world: &mut World,
    chunk_worker_handle: &ChunkWorkerHandle,
    #[default] loading_chunks: &mut LoadingChunks,
) {
    while let Ok(reply) = chunk_worker_handle.receiver.try_recv() {
        match reply {
            chunk_worker::Reply::LoadedChunk(pos, result) => {
                loading_chunks.0.remove(&pos);
                match result {
                    Ok(loaded) => {
                        game.chunk_map.insert(loaded.chunk);

                        loaded.entities.into_iter().for_each(|builder| {
                            let entity = builder.build().spawn_in(world);
                            game.handle(world, EntitySpawnEvent { entity });
                        });

                        game.handle(world, ChunkLoadEvent { chunk: pos });

                        log::trace!("Loaded chunk at {:?}", pos);
                    }
                    Err(error) => {
                        log::warn!("Failed to load chunk at {:?}: {}", pos, error);
                        game.handle(world, ChunkLoadFailEvent { pos, error });
                    }
                }
            }
            chunk_worker::Reply::SavedChunk(pos, result) => match result {
                Ok(()) => log::trace!("Saved chunk at {:?}", pos),
                Err(error) => log::warn!("Failed to save chunk at {:?}: {}", pos, error),
            },
        }
    }
}

pub fn remove_chunk_holder(
    game: &mut Game,
    world: &mut World,
    chunk: ChunkPosition,
    holder: Entity,
) {
    if let Some(vec) = game.chunk_holders.inner.get_mut(&chunk) {
        let index = vec.iter().position(|e| *e == holder);
        if let Some(index) = index {
            vec.remove(index);

            game.handle(
                world,
                ChunkHolderReleaseEvent {
                    chunk,
                    entity: holder,
                },
            );
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
#[derive(Clone, Copy, Debug, Default)]
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
#[fecs::system]
pub fn chunk_unload(
    game: &mut Game,
    world: &mut World,
    #[default] chunk_unload_queue: &mut ChunkUnloadQueue,
) {
    // Unload chunks which are finished in the queue.

    // Since chunks are queued in the back and taken out
    // from the front, the chunks in the front of the vector
    // were queued the longest time ago. Because of this,
    // we go through the unloads in the front of the queue
    // to find which chunks to unload.
    while let Some(unload) = chunk_unload_queue.queue.front().copied() {
        if game.tick_count >= unload.time {
            // Don't unload if new chunk holders have appeared.
            if game.chunk_holders.chunk_has_holders(unload.chunk) {
                chunk_unload_queue.queue.pop_front();
                continue;
            }

            // Unload chunk and pop from queue.
            if game.chunk_map.chunk_at(unload.chunk).is_some() {
                game.handle(
                    world,
                    ChunkUnloadEvent {
                        chunk: unload.chunk,
                    },
                );
                game.chunk_map.remove(unload.chunk);
                log::trace!("Unloaded chunk at {}", unload.chunk);
            }
            chunk_unload_queue.queue.pop_front();
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
#[fecs::event_handler]
pub fn on_chunk_holder_release_unload_chunk(
    event: &ChunkHolderReleaseEvent,
    game: &mut Game,
    chunk_unload_queue: &mut ChunkUnloadQueue,
) {
    // Handle holder release events.
    // If the chunk now has zero holders, queue it for unloading.
    if !game.chunk_holders.chunk_has_holders(event.chunk) {
        let unload = ChunkUnload {
            chunk: event.chunk,
            time: game.tick_count + CHUNK_UNLOAD_TIME,
        };
        chunk_unload_queue.queue.push_back(unload);
    }
}

/// System for removing an entity's chunk holds
/// once it is destroyed.
#[fecs::event_handler]
pub fn on_entity_despawn_remove_chunk_holder(
    event: &EntityDespawnEvent,
    game: &mut Game,
    world: &mut World,
) {
    // If entity had chunk holds, remove them all
    let holds = if let Some(holds) = world.try_get::<ChunkHolder>(event.entity) {
        log::debug!("Removing chunk holds for entity {:?}", event.entity);
        holds.holds.iter().copied().collect::<Vec<_>>() // todo: remove allocation
    } else {
        Vec::new()
    };

    for hold in holds {
        remove_chunk_holder(game, world, hold, event.entity);
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
#[fecs::system]
pub fn chunk_optimize(game: &mut Game) {
    // Only run every CHUNK_OPTIMIZE_INTERVAL ticks
    if game.tick_count % CHUNK_OPTIMIZE_INTERVAL != 0 {
        return;
    }

    log::debug!("Optimizing chunks");

    let start_time = current_time_in_millis();
    let count = AtomicU32::new(0);

    game.chunk_map.0.par_values().for_each(|chunk| {
        count.fetch_add(chunk.write().optimize(), Ordering::Relaxed);
    });

    let end_time = current_time_in_millis();
    let elapsed = end_time - start_time;

    let num_sections = count.load(Ordering::Relaxed);
    log::debug!(
        "Optimized {} chunk sections (took {}ms{})",
        num_sections,
        elapsed,
        if num_sections == 0 {
            String::new()
        } else {
            format!(" - {:.2}ms/section", elapsed as f64 / num_sections as f64)
        }
    );
}

/// Adds a hold for a chunk for the given entity.
pub fn hold_chunk(game: &mut Game, holder: &mut ChunkHolder, chunk: ChunkPosition, entity: Entity) {
    holder.holds.insert(chunk);
    game.chunk_holders
        .inner
        .entry(chunk)
        .or_default()
        .push(entity);
    log::trace!("Obtained chunk hold on {} for player {:?}", chunk, entity);
}

/// Releases a hold for a chunk for the given entity.
pub fn release_chunk(game: &mut Game, world: &mut World, chunk: ChunkPosition, entity: Entity) {
    let mut holder = world.get_mut::<ChunkHolder>(entity);
    holder.holds.remove(&chunk);
    if let Some(vec) = game.chunk_holders.inner.get_mut(&chunk) {
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
    log::trace!("Released chunk hold on {} for player {:?}", chunk, entity);
    drop(holder);
    game.handle(world, ChunkHolderReleaseEvent { chunk, entity });
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
pub fn save_chunk(
    handle: &ChunkWorkerHandle,
    chunk: Arc<RwLock<Chunk>>,
    entities: SmallVec<[EntityData; 4]>,
    block_entities: SmallVec<[BlockEntityData; 4]>,
) {
    let save = ChunkSave {
        chunk,
        entities,
        block_entities,
    };
    handle
        .sender
        .send(chunk_worker::Request::SaveChunk(save))
        .unwrap();
}

#[fecs::event_handler]
pub fn release_chunk_request(event: &ReleaseChunkRequest, game: &mut Game, world: &mut World) {
    release_chunk(game, world, event.chunk, event.player);
}

#[fecs::event_handler]
pub fn hold_chunk_request(event: &HoldChunkRequest, game: &mut Game, world: &mut World) {
    hold_chunk(
        game,
        &mut *world.get_mut::<ChunkHolder>(event.player),
        event.chunk,
        event.player,
    );
}

#[fecs::event_handler]
pub fn load_chunk_request(
    event: &LoadChunkRequest,
    handle: &ChunkWorkerHandle,
    loading_chunks: &mut LoadingChunks,
    game: &mut Game,
) {
    // Don't load chunk if it's already loading or already loaded.
    if !loading_chunks.0.insert(event.chunk) || game.chunk_map.0.contains_key(&event.chunk) {
        return;
    }

    load_chunk(handle, event.chunk);
}
