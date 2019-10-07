//! Saving of entity data (and chunk data along with it).

use crate::chunk_logic;
use crate::chunk_logic::{ChunkUnloadEvent, ChunkWorkerHandle};
use crate::config::Config;
use crate::entity::{ChunkEntities, SerializerComponent};
use feather_core::world::ChunkMap;
use rayon::prelude::*;
use shrev::{EventChannel, ReaderId};
use specs::{Entity, LazyUpdate, Read, ReadExpect, System, WorldExt, Write};
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
        Read<'a, LazyUpdate>,
        ReadExpect<'a, ChunkWorkerHandle>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut prev_save_time,
            mut chunk_map,
            chunk_entities,
            unload_events,
            config,
            lazy,
            worker_handle,
        ) = data;

        for event in unload_events.read(self.reader.as_mut().unwrap()) {
            let entities = vec![]; // TODO
            chunk_logic::save_chunk(&worker_handle, Arc::clone(&event.chunk), entities);
        }

        if prev_save_time.0.elapsed() >= config.world.save_interval {
            // Save chunks
            save_chunks(&mut chunk_map, &chunk_entities, &lazy);
            prev_save_time.0 = Instant::now();
        }
    }

    setup_impl!(reader);
}

/// Saves all modified chunks.
///
/// The saves themselves are performed lazily and asynchronously.
///
/// Returns the number of chunks queued for saving.
pub fn save_chunks(
    chunk_map: &mut ChunkMap,
    chunk_entities: &ChunkEntities,
    lazy: &LazyUpdate,
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

            // World access is required for entity serialization,
            // so we perform the saving itself asynchronously.
            let chunk = Arc::new(chunk.clone());
            let entities: Vec<Entity> = entities.to_vec();
            lazy.exec(move |world| {
                // Compute entity data.
                let entity_data = entities
                    .into_iter()
                    .filter_map(|entity| {
                        let serializers = world.read_component::<SerializerComponent>();
                        let serializer = match serializers.get(entity) {
                            Some(serializer) => serializer,
                            None => return None, // Entity not serialized
                        };

                        let serialize = serializer.0;
                        Some(serialize(world, entity))
                    })
                    .collect();

                let handle = world.fetch::<ChunkWorkerHandle>();
                chunk_logic::save_chunk(&handle, chunk, entity_data);
            });

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

        let last_save_time = PreviousSaveTime(Instant::now() - Duration::from_secs(120));
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
