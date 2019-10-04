//! Saving of entity data (and chunk data along with it).

use crate::chunk_logic;
use crate::chunk_logic::{ChunkUnloadEvent, ChunkWorkerHandle};
use crate::config::Config;
use crate::entity::{
    ArrowComponent, ChunkEntities, ItemComponent, PositionComponent, VelocityComponent,
};
use feather_core::entity::{ArrowEntityData, BaseEntityData, EntityData, ItemData, ItemEntityData};
use feather_core::world::ChunkMap;
use rayon::prelude::*;
use shrev::{EventChannel, ReaderId};
use specs::{Read, ReadExpect, ReadStorage, System, Write};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;

/// System to save chunk and entity data upon a chunk unload
/// and periodically.
///
/// This system listens to `ChunkUnloadEvent`s.
#[derive(Default)]
pub struct ChunkSaveSystem {
    reader: Option<ReaderId<ChunkUnloadEvent>>,
}

/// Previous time at which chunks were saved.
pub struct PreviousSaveTime(Instant);

impl Default for PreviousSaveTime {
    fn default() -> Self {
        Self(Instant::now())
    }
}

impl<'a> System<'a> for ChunkSaveSystem {
    type SystemData = (
        Write<'a, PreviousSaveTime>,
        Write<'a, ChunkMap>,
        Read<'a, ChunkEntities>,
        Read<'a, EventChannel<ChunkUnloadEvent>>,
        Read<'a, Arc<Config>>,
        ReadExpect<'a, ChunkWorkerHandle>,
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, VelocityComponent>,
        ReadStorage<'a, ItemComponent>,
        ReadStorage<'a, ArrowComponent>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut prev_save_time,
            mut chunk_map,
            chunk_entities,
            unload_events,
            config,
            worker_handle,
            positions,
            velocities,
            items,
            arrows,
        ) = data;

        // TODO: entities

        for event in unload_events.read(self.reader.as_mut().unwrap()) {
            let entities = vec![]; // TODO
            chunk_logic::save_chunk(&worker_handle, Arc::clone(&event.chunk), entities);
        }

        if prev_save_time.0.elapsed() >= config.world.save_interval {
            // Save chunks
            save_chunks(
                &mut chunk_map,
                &worker_handle,
                &chunk_entities,
                &positions,
                &velocities,
                &items,
                &arrows,
            );
            prev_save_time.0 = Instant::now();
        }
    }

    setup_impl!(reader);
}

/// Saves all modified chunks.
pub fn save_chunks(
    chunk_map: &mut ChunkMap,
    handle: &ChunkWorkerHandle,
    chunk_entities: &ChunkEntities,
    positions: &ReadStorage<PositionComponent>,
    velocities: &ReadStorage<VelocityComponent>,
    items: &ReadStorage<ItemComponent>,
    arrows: &ReadStorage<ArrowComponent>,
) -> u32 {
    let count = AtomicUsize::new(0);
    chunk_map
        .chunks_mut()
        .par_iter_mut()
        .map(|(_, chunk)| {
            let (dirty, entities) = chunk_entities.entities_in_chunk_and_modified(chunk.position());
            (chunk, entities, dirty)
        })
        .for_each(|(chunk, entities, dirty)| {
            // If all of the following are true, don't save the chunk:
            // * The chunk has not been modified since the last save.
            // * The entities in the chunk are empty (if they weren't, it is likely they were modified)
            // * The entities in the chunk haven't changed.
            if !chunk.check_modified() && (entities.is_empty() && !dirty) {
                return;
            }

            let entity_data: Vec<_> = entities
                .iter()
                .filter_map(|entity| {
                    // Convert entity to entity data.
                    // All entities have positions, but some don't have velocities
                    // (e.g. players). Those without velocities are not saved,
                    // so we can just skip them.
                    let pos = positions.get(*entity).unwrap();
                    let vel = match velocities.get(*entity) {
                        Some(vel) => vel,
                        None => return None,
                    };
                    let item = items.get(*entity);
                    let arrow = arrows.get(*entity);

                    let base = BaseEntityData {
                        position: vec![pos.current.x, pos.current.y, pos.current.z],
                        velocity: vec![vel.x, vel.y, vel.z],
                        rotation: vec![pos.current.pitch, pos.current.yaw],
                    };
                    if arrow.is_some() {
                        Some(EntityData::Arrow(ArrowEntityData {
                            entity: base,
                            critical: 0,
                        }))
                    } else if let Some(item) = item {
                        Some(EntityData::Item(ItemEntityData {
                            entity: base,
                            age: 0,          // TODO
                            pickup_delay: 0, // TODO
                            item: ItemData {
                                item: item.stack.ty.identifier().to_string(),
                                count: item.stack.amount,
                            },
                        }))
                    } else {
                        None
                    }
                })
                .collect();

            chunk_logic::save_chunk(&handle, Arc::new(chunk.clone()), entity_data);
            count.fetch_add(1, Ordering::Release);
        });

    let count = count.load(Ordering::Acquire);
    debug!("Saving {} chunks", count);
    count as u32
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{chunkworker, testframework as t};
    use failure::_core::time::Duration;
    use feather_core::{Chunk, ChunkPosition};

    #[test]
    fn test_chunk_unload() {
        let (mut world, mut dispatcher) = t::builder().with(ChunkSaveSystem::default(), "").build();

        let (tx, rx) = crossbeam::unbounded();
        let (_tx2, rx2) = crossbeam::unbounded();
        world.insert(ChunkWorkerHandle {
            sender: tx,
            receiver: rx2,
        });

        let event = ChunkUnloadEvent {
            chunk: Arc::new(Chunk::default()),
        };

        t::trigger_event(&world, event);

        dispatcher.dispatch(&world);

        let msg = rx.try_recv().unwrap();

        match msg {
            chunkworker::Request::SaveChunk(chunk, entities) => {
                assert_eq!(chunk.position(), ChunkPosition::new(0, 0));
                assert!(entities.is_empty()); // TODO
            }
            _ => panic!(),
        }
    }

    #[test]
    fn test_periodic() {
        let (mut world, mut dispatcher) = t::builder().with(ChunkSaveSystem::default(), "").build();

        let (tx, rx) = crossbeam::unbounded();
        let (_tx2, rx2) = crossbeam::unbounded();
        world.insert(ChunkWorkerHandle {
            sender: tx,
            receiver: rx2,
        });

        let last_save_time = PreviousSaveTime(Instant::now() - Duration::from_secs(1_000_000));
        world.insert(last_save_time);

        let pos = ChunkPosition::new(0, 0);
        world
            .fetch_mut::<ChunkMap>()
            .set_chunk_at(pos, Chunk::new(pos));

        dispatcher.dispatch(&world);

        let msg = rx.try_recv().unwrap();

        match msg {
            chunkworker::Request::SaveChunk(chunk, entities) => {
                assert_eq!(chunk.position(), pos);
                assert!(entities.is_empty()); // TODO
            }
            _ => panic!(),
        }
    }
}
