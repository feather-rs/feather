use crate::game::Game;
use ahash::AHashMap;
use feather_core::{ChunkPosition, Position};
use fecs::{Entity, World};
use itertools::Itertools;
use smallvec::SmallVec;

/// System to update ChunkEntities when entities move into new chunks.
pub fn on_chunk_cross_update_chunk_entities(
    game: &mut Game,
    entity: Entity,
    old: Option<ChunkPosition>,
    new: ChunkPosition,
) {
    if let Some(old) = old {
        if let Some(vec) = game.chunk_entities.0.get_mut(&old) {
            let index = vec
                .iter()
                .find_position(|e| **e == entity)
                .map(|(index, _)| index);
            if let Some(index) = index {
                vec.swap_remove(index);
            }
        }

        game.chunk_entities.0.entry(new).or_default().push(entity);
    }
}

pub fn on_entity_despawn_update_chunk_entities(game: &mut Game, world: &World, entity: Entity) {
    if let Some(pos) = world.try_get::<Position>(entity) {
        if let Some(vec) = game.chunk_entities.0.get_mut(&pos.chunk()) {
            let index = vec
                .iter()
                .find_position(|e| **e == entity)
                .map(|(index, _)| index);
            if let Some(index) = index {
                vec.swap_remove(index);
            }
        }
    }
}

pub fn on_entity_spawn_update_chunk_entities(game: &mut Game, world: &World, entity: Entity) {
    if let Some(chunk) = world.try_get::<Position>(entity).map(|pos| pos.chunk()) {
        game.chunk_entities.0.entry(chunk).or_default().push(entity);
    }
}
