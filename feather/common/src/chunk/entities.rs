use ahash::AHashMap;
use base::{ChunkPosition, Position};
use ecs::{Entity, SysResult, SystemExecutor};
use quill_common::events::{EntityCreateEvent, EntityRemoveEvent};
use utils::vec_remove_item;

use crate::{events::ChunkCrossEvent, Game};

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.add_system(update_chunk_entities);
}

/// A spatial index to look up entities within a given chunk.
#[derive(Default)]
pub struct ChunkEntities {
    entities: AHashMap<ChunkPosition, Vec<Entity>>,
}

impl ChunkEntities {
    /// Returns the entities in the given chunk.
    pub fn entities_in_chunk(&self, chunk: ChunkPosition) -> &[Entity] {
        self.entities
            .get(&chunk)
            .map(Vec::as_slice)
            .unwrap_or_default()
    }

    fn update(
        &mut self,
        entity: Entity,
        old_chunk: Option<ChunkPosition>,
        new_chunk: ChunkPosition,
    ) {
        if let Some(old_chunk) = old_chunk {
            if let Some(vec) = self.entities.get_mut(&old_chunk) {
                vec_remove_item(vec, &entity);
            }
        }

        self.entities.entry(new_chunk).or_default().push(entity);
    }

    fn remove_entity(&mut self, entity: Entity, chunk: ChunkPosition) {
        if let Some(vec) = self.entities.get_mut(&chunk) {
            vec_remove_item(vec, &entity);
        }
    }
}

fn update_chunk_entities(game: &mut Game) -> SysResult {
    // Entities that have crossed chunks
    let mut events = Vec::new();
    for (entity, (old_chunk, &position)) in
        game.ecs.query::<(&mut ChunkPosition, &Position)>().iter()
    {
        let new_chunk = position.chunk();
        if position.chunk() != *old_chunk {
            game.chunk_entities
                .update(entity, Some(*old_chunk), new_chunk);
            events.push((
                entity,
                ChunkCrossEvent {
                    old_chunk: *old_chunk,
                    new_chunk,
                },
            ));

            *old_chunk = new_chunk;
        }
    }
    for (entity, event) in events {
        game.ecs.insert_entity_event(entity, event)?;
    }

    // Entities that have been created
    let mut insertions = Vec::new();
    for (entity, (_event, &position)) in game.ecs.query::<(&EntityCreateEvent, &Position)>().iter()
    {
        let chunk = position.chunk();
        game.chunk_entities.update(entity, None, chunk);
        insertions.push((entity, chunk));
    }
    // Add ChunkPosition component to new entities
    for (entity, chunk) in insertions {
        game.ecs.insert(entity, chunk)?;
    }

    // Entities that have been destroyed
    for (entity, (_event, &chunk)) in game
        .ecs
        .query::<(&EntityRemoveEvent, &ChunkPosition)>()
        .iter()
    {
        game.chunk_entities.remove_entity(entity, chunk);
    }

    Ok(())
}
