//! Handles saving of chunks and entities

use crate::entity::ComponentSerializer;
use crate::game::Game;
use crate::{chunk_logic, TICK_TIME, TPS};
use feather_core::ChunkPosition;
use fecs::World;
use std::collections::VecDeque;

/// A chunk to save + the tick count at which to do so.
#[derive(Clone, Copy, Debug)]
struct SaveTask {
    /// Chunk position to save.
    chunk: ChunkPosition,
    /// Tick count at which to save this chunk.
    at: u64,
}

/// Queue of chunks to save.
#[derive(Debug, Default)]
pub struct SaveQueue(VecDeque<SaveTask>);

/// On a chunk load, adds the chunk to the save queue.
pub fn on_chunk_load_queue_for_saving(game: &mut Game, chunk: ChunkPosition) {
    queue_for_saving(game, chunk);
}

/// On a chunk unload, saves the chunk first.
pub fn on_chunk_unload_save_chunk(game: &mut Game, world: &World, chunk: ChunkPosition) {
    save_chunk_at(game, world, chunk);
}

fn queue_for_saving(game: &mut Game, chunk: ChunkPosition) {
    let tick_to_save_at =
        game.tick_count + (game.config.world.save_interval.as_millis() as u64) / TICK_TIME;

    let task = SaveTask {
        chunk,
        at: tick_to_save_at,
    };

    game.save_queue.0.push_back(task);
}

/// System which checks for chunks which have been queued for saving
/// and, if it is time, saves them.
#[system]
pub fn chunk_save(game: &mut Game, world: &mut World) {
    // no need to run this system every tick
    if game.tick_count % TPS != 0 {
        return;
    }

    loop {
        let task = match game.save_queue.0.front().copied() {
            Some(task) => task,
            None => return, // no save tasks to run
        };

        if game.chunk_map.chunk_at(task.chunk).is_none() {
            game.save_queue
                .0
                .pop_front()
                .expect("we just verified the front task exists");
            continue;
        }

        if task.at <= game.tick_count {
            // Save the chunk, then pop the task from the queue.
            save_chunk_at(game, world, task.chunk);

            game.save_queue
                .0
                .pop_front()
                .expect("we just verified the front task exists");

            // Requeue the chunk for saving again.
            queue_for_saving(game, task.chunk);
        } else {
            return;
        }
    }
}

pub fn save_chunk_at(game: &Game, world: &World, pos: ChunkPosition) {
    let chunk = game
        .chunk_map
        .chunk_handle_at(pos)
        .expect("chunk does not exist");

    if !chunk.write().check_modified() {
        return;
    }

    // Serialize the entities in the chunk.
    let entities = game
        .chunk_entities
        .entities_in_chunk(pos)
        .into_iter()
        .filter_map(|entity| {
            if let Some(serializer) = world.try_get::<ComponentSerializer>(*entity) {
                let accessor = world.entity(*entity).expect("entity does not exist");

                Some(serializer.serialize(game, &accessor))
            } else {
                None
            }
        })
        .collect();

    trace!("Queuing chunk at {} for saving", pos);
    chunk_logic::save_chunk(
        &game.chunk_worker_handle,
        game.chunk_map.chunk_handle_at(pos).unwrap(),
        entities,
    );
}
