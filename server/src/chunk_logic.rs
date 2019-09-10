//! Module for interacting with the chunk worker thread
//! from the server threads.
//!
//! Also handles unloading chunks when unused.
use crossbeam::channel::{Receiver, Sender};
use shrev::{EventChannel, ReaderId};
use specs::{
    Component, DispatcherBuilder, Entity, Read, ReadExpect, ReadStorage, System, World, Write,
};
use std::sync::atomic::{AtomicU32, Ordering};

use feather_core::world::{ChunkMap, ChunkPosition};

use rayon::prelude::*;

use crate::entity::EntityDestroyEvent;
use crate::systems::{CHUNK_HOLD_REMOVE, CHUNK_LOAD, CHUNK_OPTIMIZE, CHUNK_UNLOAD};
use crate::worldgen::WorldGenerator;
use crate::{chunkworker, current_time_in_millis, TickCount, TPS};
use feather_core::entity::EntityData;
use hashbrown::HashSet;
use multimap::MultiMap;
use specs::storage::BTreeStorage;
use std::collections::VecDeque;
use std::sync::Arc;

/// A handle for interacting with the chunk
/// worker thread.
#[derive(Debug, Clone)]
pub struct ChunkWorkerHandle {
    sender: Sender<chunkworker::Request>,
    receiver: Receiver<chunkworker::Reply>,
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
pub struct ChunkLoadSystem;

impl<'a> System<'a> for ChunkLoadSystem {
    type SystemData = (
        Write<'a, ChunkMap>,
        Write<'a, EventChannel<ChunkLoadEvent>>,
        Write<'a, EventChannel<ChunkLoadFailEvent>>,
        ReadExpect<'a, ChunkWorkerHandle>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut chunk_map, mut load_events, mut fail_events, handle) = data;

        while let Ok((pos, result)) = handle.receiver.try_recv() {
            match result {
                Ok((chunk, entities)) => {
                    chunk_map.set_chunk_at(pos, chunk);

                    // Trigger event
                    let event = ChunkLoadEvent { pos, entities };
                    load_events.single_write(event);

                    trace!("Loaded chunk at {:?}", pos);
                }
                Err(err) => {
                    // TODO generate chunk if it didn't exist
                    warn!("Failed to load chunk at {:?}: {}", pos, err);
                    let event = ChunkLoadFailEvent { pos };
                    fail_events.single_write(event);
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        use specs::prelude::SystemData;

        let generator = world.fetch_mut::<Arc<dyn WorldGenerator>>().clone();

        info!("Starting chunk worker thread");
        let (sender, receiver) = chunkworker::start("world", generator);
        world.insert(ChunkWorkerHandle { sender, receiver });

        Self::SystemData::setup(world);
    }
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
        .send(chunkworker::Request::LoadChunk(pos))
        .unwrap();
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
    pub fn holders_for(&self, chunk: ChunkPosition) -> Option<&Vec<Entity>> {
        self.inner.get_vec(&chunk)
    }

    pub fn chunk_has_holders(&self, chunk: ChunkPosition) -> bool {
        let holders = self.holders_for(chunk);

        !(holders.is_none() || holders.unwrap().is_empty())
    }

    pub fn insert_holder(&mut self, chunk: ChunkPosition, holder: Entity) {
        self.inner.insert(chunk, holder);
    }

    pub fn remove_holder(
        &mut self,
        chunk: ChunkPosition,
        holder: Entity,
        events: &mut EventChannel<ChunkHolderReleaseEvent>,
    ) {
        if let Some(vec) = self.inner.get_vec_mut(&chunk) {
            let index = vec.iter().position(|e| *e == holder);
            if let Some(index) = index {
                vec.remove(index);

                // Trigger event
                let event = ChunkHolderReleaseEvent {
                    entity: holder,
                    chunk,
                };
                events.single_write(event);
            }
        }
    }
}

/// Event triggered when a chunk holder is released.
#[derive(Clone, Debug)]
pub struct ChunkHolderReleaseEvent {
    /// The entity which previously held the chunk.
    pub entity: Entity,
    /// The chunk which the holder was released from.
    pub chunk: ChunkPosition,
}

/// The queue of chunks to be unloaded.
/// See `ChunkUnloadSystem` for details.
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
/// This system performs multiple actions:
///
/// * It listens to `ChunkHolderReleaseEvent` and
/// checks if a chunk has no holders. If so, it queues
/// the chunk to be unloaded after some period of time
/// (defined by a constant).
/// * It goes through chunks which are currently
/// queued to be loaded and unloads them if the
/// period of time has elapsed.
///
/// Chunks are not unloaded immediately after having
/// no holders because doing so could open up
/// opportunities for exploits. For example, a player
/// could quickly move between chunk boundaries, causing
/// chunks at the edge of their view distance
/// to be loaded and unloaded at an alarming rate.
#[derive(Default)]
pub struct ChunkUnloadSystem {
    reader: Option<ReaderId<ChunkHolderReleaseEvent>>,
}

impl ChunkUnloadSystem {
    pub fn new() -> Self {
        Self { reader: None }
    }
}

impl<'a> System<'a> for ChunkUnloadSystem {
    type SystemData = (
        Write<'a, ChunkMap>,
        Read<'a, EventChannel<ChunkHolderReleaseEvent>>,
        Write<'a, ChunkUnloadQueue>,
        Read<'a, ChunkHolders>,
        Read<'a, TickCount>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut chunk_map, events, mut unload_queue, holders, tick_count) = data;

        // Handle holder release events.
        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            // If the chunk now has zero holders, queue it for unloading.
            if !holders.chunk_has_holders(event.chunk) {
                let unload = ChunkUnload {
                    chunk: event.chunk,
                    time: tick_count.0 + CHUNK_UNLOAD_TIME,
                };
                unload_queue.queue.push_back(unload);
            }
        }

        // Unload chunks which are finished in the queue.

        // Since chunks are queued in the back and taken out
        // from the front, the chunks in the front of the vector
        // were queued the longest time ago. Because of this,
        // we go through the unloads in the front of the queue
        // to find which chunks to unload.
        while let Some(unload) = unload_queue.queue.front() {
            if tick_count.0 >= unload.time {
                // Don't unload if new chunk holders have appeared.
                if holders.chunk_has_holders(unload.chunk) {
                    unload_queue.queue.pop_front();
                    continue;
                }

                trace!("Unloading chunk at {:?}", unload.chunk);

                // Unload chunk and pop from queue.
                chunk_map.unload_chunk_at(unload.chunk);
                unload_queue.queue.pop_front();
            } else {
                // We're done - all chunks farther up in
                // the queue were queued before this one,
                // so it isn't time to unload any of those.
                break;
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        use specs::SystemData;
        Self::SystemData::setup(world);

        self.reader = Some(
            world
                .fetch_mut::<EventChannel<ChunkHolderReleaseEvent>>()
                .register_reader(),
        );
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
/// a hold on.
#[derive(Default)]
pub struct ChunkHolderComponent {
    pub holds: HashSet<ChunkPosition>,
}

impl ChunkHolderComponent {
    pub fn new() -> Self {
        Self {
            holds: HashSet::new(),
        }
    }
}

impl Component for ChunkHolderComponent {
    type Storage = BTreeStorage<Self>;
}

/// System for removing an entity's chunk holds
/// once it is destroyed.
#[derive(Default)]
pub struct ChunkHoldRemoveSystem {
    reader: Option<ReaderId<EntityDestroyEvent>>,
}

impl ChunkHoldRemoveSystem {
    pub fn new() -> Self {
        Self { reader: None }
    }
}

impl<'a> System<'a> for ChunkHoldRemoveSystem {
    type SystemData = (
        Read<'a, EventChannel<EntityDestroyEvent>>,
        Write<'a, ChunkHolders>,
        ReadStorage<'a, ChunkHolderComponent>,
        Write<'a, EventChannel<ChunkHolderReleaseEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (events, mut holders, holder_comps, mut release_events) = data;

        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            // If entity had chunk holds, remove them all
            if let Some(holder_comp) = holder_comps.get(event.entity) {
                debug!("Removing chunk holds for entity {:?}", event.entity);
                holder_comp.holds.iter().for_each(|chunk| {
                    holders.remove_holder(*chunk, event.entity, &mut release_events);
                });
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        use specs::SystemData;

        Self::SystemData::setup(world);

        self.reader = Some(
            world
                .fetch_mut::<EventChannel<EntityDestroyEvent>>()
                .register_reader(),
        );
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
pub struct ChunkOptimizeSystem;

impl<'a> System<'a> for ChunkOptimizeSystem {
    type SystemData = (Write<'a, ChunkMap>, Read<'a, TickCount>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut chunk_map, tick_count) = data;

        // Only run every CHUNK_OPTIMIZE_INTERVAL ticks
        if tick_count.0 % CHUNK_OPTIMIZE_INTERVAL != 0 {
            return;
        }

        let chunks = chunk_map.chunks_mut();

        // Don't run if there aren't any chunks loaded
        if chunks.is_empty() {
            return;
        }

        debug!("Optimizing chunks");

        let start_time = current_time_in_millis();
        let count = AtomicU32::new(0);

        chunks.par_iter_mut().for_each(|(_, chunk)| {
            count.fetch_add(chunk.optimize(), Ordering::SeqCst);
        });

        let end_time = current_time_in_millis();
        let elapsed = end_time - start_time;

        debug!(
            "Optimized {} chunk sections (took {}ms - {:.2}ms/section)",
            count.load(Ordering::SeqCst),
            elapsed,
            elapsed as f64 / f64::from(count.load(Ordering::SeqCst))
        );
    }
}

pub fn init_logic(dispatcher: &mut DispatcherBuilder) {
    dispatcher.add(ChunkLoadSystem, CHUNK_LOAD, &[]);
    dispatcher.add(ChunkOptimizeSystem, CHUNK_OPTIMIZE, &[]);
}

pub fn init_handlers(dispatcher: &mut DispatcherBuilder) {
    dispatcher.add(ChunkUnloadSystem::default(), CHUNK_UNLOAD, &[]);
    dispatcher.add(ChunkHoldRemoveSystem::default(), CHUNK_HOLD_REMOVE, &[]);
}

#[cfg(test)]
mod tests {
    use specs::{RunNow, World, WorldExt};

    use feather_core::world::chunk::Chunk;
    use feather_core::world::ChunkPosition;

    use super::*;

    #[test]
    fn test_chunk_system() {
        let (send1, _recv1) = crossbeam::channel::unbounded();
        let (send2, recv2) = crossbeam::channel::unbounded();
        let handle = ChunkWorkerHandle {
            sender: send1,
            receiver: recv2,
        };

        let chunk_map = ChunkMap::new();
        let pos = ChunkPosition::new(0, 0);
        send2.send((pos, Ok((Chunk::new(pos), vec![])))).unwrap();

        let load_event_channel = EventChannel::<ChunkLoadEvent>::new();
        let fail_event_channel = EventChannel::<ChunkLoadFailEvent>::new();

        let mut system = ChunkLoadSystem;
        let mut world = World::new();
        world.insert(chunk_map);
        world.insert(handle);
        world.insert(load_event_channel);
        world.insert(fail_event_channel);

        system.run_now(&world);

        // Confirm that chunk was loaded
        let chunk_map = world.read_resource::<ChunkMap>();
        let chunk = chunk_map.chunk_at(pos);

        assert!(chunk.is_some());
        assert!(chunk.unwrap().position() == pos);
    }

    #[test]
    fn test_load_chunk() {
        let (send1, recv1) = crossbeam::channel::unbounded();
        let (_send2, recv2) = crossbeam::channel::unbounded();
        let handle = ChunkWorkerHandle {
            sender: send1,
            receiver: recv2,
        };

        let pos = ChunkPosition::new(0, 0);

        load_chunk(&handle, pos);

        let recv = recv1.try_recv().unwrap();
        assert_eq!(recv, chunkworker::Request::LoadChunk(pos));
    }
}
