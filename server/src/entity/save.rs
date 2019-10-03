//! Saving of entity data (and chunk data along with it).

use crate::chunk_logic;
use crate::chunk_logic::{ChunkUnloadEvent, ChunkWorkerHandle};
use shrev::{EventChannel, ReaderId};
use specs::{Read, ReadExpect, System};
use std::sync::Arc;

/// System to save chunk and entity data upon a chunk unload.
///
/// This system listens to `ChunkUnloadEvent`s.
#[derive(Default)]
pub struct ChunkSaveSystem {
    reader: Option<ReaderId<ChunkUnloadEvent>>,
}

impl<'a> System<'a> for ChunkSaveSystem {
    type SystemData = (
        Read<'a, EventChannel<ChunkUnloadEvent>>,
        ReadExpect<'a, ChunkWorkerHandle>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (unload_events, worker_handle) = data;

        for event in unload_events.read(self.reader.as_mut().unwrap()) {
            let entities = vec![]; // TODO
            chunk_logic::save_chunk(&worker_handle, Arc::clone(&event.chunk), entities);
        }
    }

    setup_impl!(reader);
}
