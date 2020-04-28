//! Handles saving of chunks and entities

use crate::{chunk_manager, ChunkWorkerHandle};
use feather_core::anvil::entity::BaseEntityData;
use feather_core::anvil::player::{InventorySlot, PlayerData};
use feather_core::inventory::Inventory;
use feather_core::util::{ChunkPosition, Gamemode, Position, Vec3d};
use feather_server_types::{
    ChunkLoadEvent, ChunkUnloadEvent, ComponentSerializer, Game, PlayerLeaveEvent, Uuid,
    TICK_LENGTH, TPS,
};
use fecs::{Entity, World};
use std::collections::VecDeque;
use std::path::Path;
use std::sync::Arc;

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
struct SaveQueue(VecDeque<SaveTask>);

/// On a chunk load, adds the chunk to the save queue.
#[fecs::event_handler]
pub fn on_chunk_load_queue_for_saving(
    event: &ChunkLoadEvent,
    game: &mut Game,
    #[default] save_queue: &mut SaveQueue,
) {
    queue_for_saving(game, save_queue, event.chunk);
}

/// On a chunk unload, saves the chunk first.
#[fecs::event_handler]
pub fn on_chunk_unload_save_chunk(
    event: &ChunkUnloadEvent,
    game: &mut Game,
    world: &mut World,
    chunk_worker_handle: &ChunkWorkerHandle,
) {
    save_chunk_at(game, world, event.chunk, chunk_worker_handle);
}

fn queue_for_saving(game: &mut Game, save_queue: &mut SaveQueue, chunk: ChunkPosition) {
    let tick_to_save_at =
        game.tick_count + (game.config.world.save_interval.as_millis() as u64) / TICK_LENGTH;

    let task = SaveTask {
        chunk,
        at: tick_to_save_at,
    };

    save_queue.0.push_back(task);
}

/// System which checks for chunks which have been queued for saving
/// and, if it is time, saves them.
#[fecs::system]
pub fn chunk_save(
    game: &mut Game,
    world: &mut World,
    save_queue: &mut SaveQueue,
    chunk_worker_handle: &ChunkWorkerHandle,
) {
    // no need to run this system every tick
    if game.tick_count % TPS != 0 {
        return;
    }

    loop {
        let task = match save_queue.0.front().copied() {
            Some(task) => task,
            None => return, // no save tasks to run
        };

        if game.chunk_map.chunk_at(task.chunk).is_none() {
            save_queue
                .0
                .pop_front()
                .expect("we just verified the front task exists");
            continue;
        }

        if task.at <= game.tick_count {
            // Save the chunk, then pop the task from the queue.
            save_chunk_at(game, world, task.chunk, chunk_worker_handle);

            save_queue
                .0
                .pop_front()
                .expect("we just verified the front task exists");

            // Requeue the chunk for saving again.
            queue_for_saving(game, save_queue, task.chunk);
        } else {
            return;
        }
    }
}

pub fn save_chunk_at(
    game: &Game,
    world: &World,
    pos: ChunkPosition,
    chunk_worker_handle: &ChunkWorkerHandle,
) {
    let chunk = game
        .chunk_map
        .chunk_handle_at(pos)
        .expect("chunk does not exist");

    if !chunk.write().check_modified() && game.chunk_entities.entities_in_chunk(pos).is_empty() {
        return;
    }

    // Serialize the entities in the chunk.
    let entities = game
        .chunk_entities
        .entities_in_chunk(pos)
        .iter()
        .filter_map(|entity| {
            if let Some(serializer) = world.try_get::<ComponentSerializer>(*entity) {
                let accessor = world.entity(*entity).expect("entity does not exist");

                Some(serializer.serialize(game, &accessor))
            } else {
                None
            }
        })
        .collect();

    log::trace!("Queuing chunk at {} for saving", pos);
    chunk_manager::save_chunk(
        chunk_worker_handle,
        game.chunk_map.chunk_handle_at(pos).unwrap(),
        entities,
    );
}

#[fecs::event_handler]
pub fn on_player_leave_save_data(event: &PlayerLeaveEvent, game: &Game, world: &mut World) {
    save_player_data(game, world, event.player);
}

pub fn save_player_data(game: &Game, world: &World, player: Entity) {
    let inventory = world
        .get::<Inventory>(player)
        .items()
        .iter()
        .enumerate()
        .filter_map(|(i, item)| item.map(|item| (i, item)))
        .map(|(slot, item)| InventorySlot {
            count: item.amount as i8,
            slot: slot as i8,
            item: item.ty.identifier().to_owned(),
        })
        .collect();

    let data = PlayerData {
        entity: BaseEntityData::new(*world.get::<Position>(player), Vec3d::broadcast(0.0)),
        gamemode: world.get::<Gamemode>(player).id() as i32,
        inventory,
    };

    let uuid = *world.get::<Uuid>(player);
    let config = Arc::clone(&game.config);

    game.running_tasks.schedule(async move {
        match feather_core::anvil::player::save_player_data(
            &Path::new(&config.world.name),
            uuid,
            &data,
        )
        .await
        {
            Ok(_) => (),
            Err(e) => log::error!("Failed to save player data for UUID {}: {}", uuid, e),
        }
    });
}
