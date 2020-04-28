use feather_core::util::Position;
use feather_server_types::{ChunkCrossEvent, EntityDespawnEvent, EntitySpawnEvent, Game};
use fecs::World;
use itertools::Itertools;

/// System to update ChunkEntities when entities move into new chunks.
#[fecs::event_handler]
pub fn on_chunk_cross_update_chunk_entities(event: &ChunkCrossEvent, game: &mut Game) {
    if let Some(old) = event.old {
        if let Some(vec) = game.chunk_entities.0.get_mut(&old) {
            let index = vec
                .iter()
                .find_position(|e| **e == event.entity)
                .map(|(index, _)| index);
            if let Some(index) = index {
                vec.swap_remove(index);
            }
        }

        game.chunk_entities
            .0
            .entry(event.new)
            .or_default()
            .push(event.entity);
    }
}

#[fecs::event_handler]
pub fn on_entity_despawn_update_chunk_entities(
    event: &EntityDespawnEvent,
    game: &mut Game,
    world: &mut World,
) {
    if let Some(pos) = world.try_get::<Position>(event.entity) {
        if let Some(vec) = game.chunk_entities.0.get_mut(&pos.chunk()) {
            let index = vec
                .iter()
                .find_position(|e| **e == event.entity)
                .map(|(index, _)| index);
            if let Some(index) = index {
                vec.swap_remove(index);
            }
        }
    }
}

#[fecs::event_handler]
pub fn on_entity_spawn_update_chunk_entities(
    event: &EntitySpawnEvent,
    game: &mut Game,
    world: &mut World,
) {
    if let Some(chunk) = world
        .try_get::<Position>(event.entity)
        .map(|pos| pos.chunk())
    {
        game.chunk_entities
            .0
            .entry(chunk)
            .or_default()
            .push(event.entity);
    }
}
