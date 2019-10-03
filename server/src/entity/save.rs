//! Saving of entity data (and chunk data along with it).

use crate::chunk_logic;
use crate::chunk_logic::{ChunkUnloadEvent, ChunkWorkerHandle};
use crate::config::Config;
use feather_core::world::ChunkMap;
use shrev::{EventChannel, ReaderId};
use specs::{Read, ReadExpect, System, Write};
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
        Read<'a, EventChannel<ChunkUnloadEvent>>,
        Read<'a, Arc<Config>>,
        ReadExpect<'a, ChunkWorkerHandle>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut prev_save_time, mut chunk_map, unload_events, config, worker_handle) = data;

        // TODO: entities

        for event in unload_events.read(self.reader.as_mut().unwrap()) {
            let entities = vec![]; // TODO
            chunk_logic::save_chunk(&worker_handle, Arc::clone(&event.chunk), entities);
        }

        if prev_save_time.0.elapsed() >= config.world.save_interval {
            // Save chunks
            let mut count = 0;
            for (_, chunk) in chunk_map.chunks_mut() {
                if chunk.check_modified() {
                    let entities = vec![]; // TODO
                    chunk_logic::save_chunk(&worker_handle, Arc::new(chunk.clone()), entities);
                    count += 1;
                }
            }

            prev_save_time.0 = Instant::now();

            debug!("Saving {} chunks", count);
        }
    }

    setup_impl!(reader);
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
