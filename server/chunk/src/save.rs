//! Handles saving of chunks and entities

use crate::{chunk_manager, ChunkWorkerHandle};
use feather_core::anvil::entity::{AnimalData, BaseEntityData, EntityData};
use feather_core::anvil::{
    block_entity::BlockEntityData,
    player::{InventorySlot, PlayerData},
};
use feather_core::inventory::{Inventory, Window};
use feather_core::util::{ChunkPosition, Gamemode, Position, Vec3d};
use feather_server_types::{
    tasks, BlockSerializer, ChunkLoadEvent, ChunkUnloadEvent, ComponentSerializer, Game, Health,
    HeldItem, PlayerLeaveEvent, Uuid, TICK_LENGTH, TPS,
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
    let (entities, block_entities) = serialize_entities(game, world, pos);

    log::trace!("Queuing chunk at {} for saving", pos);
    chunk_manager::save_chunk(
        chunk_worker_handle,
        game.chunk_map.chunk_handle_at(pos).unwrap(),
        entities.collect(),
        block_entities.collect(),
    );
}

fn serialize_entities<'a>(
    game: &'a Game,
    world: &'a World,
    pos: ChunkPosition,
) -> (
    impl Iterator<Item = EntityData> + 'a,
    impl Iterator<Item = BlockEntityData> + 'a,
) {
    let entities = game
        .chunk_entities
        .entities_in_chunk(pos)
        .iter()
        .filter_map(move |entity| {
            if let Some(serializer) = world.try_get::<ComponentSerializer>(*entity) {
                let accessor = world.entity(*entity).expect("entity does not exist");

                Some(serializer.serialize(game, &accessor))
            } else {
                None
            }
        });

    let block_entities = game
        .chunk_entities
        .entities_in_chunk(pos)
        .iter()
        .filter_map(move |entity| {
            if let Some(serializer) = world.try_get::<BlockSerializer>(*entity) {
                let accessor = world.entity(*entity).expect("entity does not exist");

                Some(serializer.serialize(game, &accessor))
            } else {
                None
            }
        });

    (entities, block_entities)
}

#[fecs::event_handler]
pub fn on_player_leave_save_data(event: &PlayerLeaveEvent, game: &Game, world: &mut World) {
    save_player_data(game, world, event.player);
}

pub fn save_player_data(game: &Game, world: &World, player: Entity) {
    let inventory = world
        .get::<Inventory>(player)
        .enumerate()
        .filter_map(|(index, slot)| slot.map(move |slot| (index, slot)))
        .filter_map(|(index, slot)| {
            InventorySlot::from_network_index(
                Window::player(player).convert_slot(index, player).unwrap(),
                slot,
            )
        })
        .collect();

    let health = world
        .try_get::<Health>(player)
        .map(|health| health.0 as f32)
        .unwrap_or(1.0);
    let data = PlayerData {
        animal: AnimalData::new(
            BaseEntityData::new(*world.get::<Position>(player), Vec3d::broadcast(0.0)),
            health,
        ),
        gamemode: world.get::<Gamemode>(player).id() as i32,
        inventory,
        held_item: world.get::<HeldItem>(player).0 as i32,
    };

    let uuid = *world.get::<Uuid>(player);
    let config = Arc::clone(&game.config);

    tasks().spawn(async move {
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
