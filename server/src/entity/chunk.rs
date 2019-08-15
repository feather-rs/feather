//! Maintains a list of entities which are in each
//! chunk, which allows for more efficient nearby
//! entity queries and packet broadcasting.

use crate::entity::{EntityComponent, EntityDestroyEvent, EntityMoveEvent, EntitySpawnEvent};
use feather_core::world::ChunkPosition;
use fnv::FnvHashMap;
use shrev::EventChannel;
use specs::{Entity, Read, ReadStorage, ReaderId, System, Write};

/// Keeps track of which entities are in which chunk.
#[derive(Debug, Clone, Deref, DerefMut, Default)]
pub struct ChunkEntities(FnvHashMap<ChunkPosition, Vec<Entity>>);

lazy_static! {
    static ref EMPTY_VEC: Vec<Entity> = Vec::with_capacity(0);
}

impl ChunkEntities {
    /// Returns all entities in a given chunk.
    pub fn entities_in_chunk(&self, chunk: ChunkPosition) -> &Vec<Entity> {
        if let Some(entities) = self.0.get(&chunk) {
            entities
        } else {
            &EMPTY_VEC
        }
    }

    /// Adds an entity to a chunk.
    pub fn add_to_chunk(&mut self, chunk: ChunkPosition, entity: Entity) {
        self.0
            .entry(chunk)
            .and_modify(|vec| vec.push(entity))
            .or_insert_with(|| vec![entity]);
    }

    /// Removes an entity from a chunk.
    ///
    /// # Panics
    /// May panic in some cases if the entity is not contained
    /// within the given chunk.
    pub fn remove_from_chunk(&mut self, chunk: ChunkPosition, entity: Entity) {
        let vec = self.0.get_mut(&chunk).unwrap();

        let (index, _) = vec.iter().enumerate().find(|x| *x.1 == entity).unwrap();
        vec.swap_remove(index);

        if vec.is_empty() {
            self.0.remove(&chunk);
        }
    }
}

/// System for updating the `ChunkEntities`.
///
/// This system listens to `EntityMoveEvent`s, `EntitySpawnEvent`s,
/// and `EntityDestroyEvent`s.
#[derive(Default)]
pub struct ChunkEntityUpdateSystem {
    move_reader: Option<ReaderId<EntityMoveEvent>>,
    spawn_reader: Option<ReaderId<EntitySpawnEvent>>,
    destroy_reader: Option<ReaderId<EntityDestroyEvent>>,
}

impl<'a> System<'a> for ChunkEntityUpdateSystem {
    type SystemData = (
        ReadStorage<'a, EntityComponent>,
        Write<'a, ChunkEntities>,
        Read<'a, EventChannel<EntityMoveEvent>>,
        Read<'a, EventChannel<EntitySpawnEvent>>,
        Read<'a, EventChannel<EntityDestroyEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entity_comps, mut entity_chunks, move_events, spawn_events, destroy_events) = data;

        for event in move_events.read(self.move_reader.as_mut().unwrap()) {
            let new_pos = event.new_pos.chunk_pos();
            let old_pos = event.old_pos.chunk_pos();

            if new_pos != old_pos {
                entity_chunks.remove_from_chunk(old_pos, event.entity);
                entity_chunks.add_to_chunk(new_pos, event.entity);
            }
        }

        for event in spawn_events.read(self.spawn_reader.as_mut().unwrap()) {
            let pos = entity_comps.get(event.entity).unwrap().position;
            entity_chunks.add_to_chunk(pos.chunk_pos(), event.entity);
        }

        for event in destroy_events.read(self.destroy_reader.as_mut().unwrap()) {
            let pos = entity_comps.get(event.entity).unwrap().position;
            entity_chunks.remove_from_chunk(pos.chunk_pos(), event.entity);
        }
    }

    setup_impl!(move_reader, spawn_reader, destroy_reader);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::EntityType;
    use crate::testframework as t;
    use feather_core::Position;
    use specs::{Builder, World, WorldExt};

    #[test]
    fn test_entity_chunks() {
        let mut chunks = ChunkEntities::default();

        let mut world = World::new();
        let entity = world.create_entity().build();

        let pos = ChunkPosition::new(0, 0);

        chunks.add_to_chunk(pos, entity);
        assert_eq!(chunks.entities_in_chunk(pos).as_slice(), &[entity]);
    }

    #[test]
    fn test_new_entity() {
        let (mut w, mut d) = t::init_world();

        let pos = position!(1.0, 64.0, 1003.5);
        let entity = t::add_entity_with_pos(&mut w, EntityType::Player, pos, true);

        d.dispatch(&w);
        w.maintain();

        let chunk_entities = w.fetch::<ChunkEntities>();
        assert_eq!(
            chunk_entities.entities_in_chunk(pos.chunk_pos()).as_slice(),
            &[entity]
        );
    }

    #[test]
    fn test_moved_entity() {
        let (mut w, mut d) = t::init_world();

        let pos = position!(1.0, 64.0, -14.0);
        let old_pos = position!(1.0, 64.0, -18.0);

        let entity = t::add_entity_with_pos(&mut w, EntityType::Player, pos, false);

        let event = EntityMoveEvent {
            entity,
            new_pos: pos,
            old_pos,
        };
        t::trigger_event(&w, event);

        {
            let mut chunk_entities = w.fetch_mut::<ChunkEntities>();
            chunk_entities.add_to_chunk(old_pos.chunk_pos(), entity);
        }

        d.dispatch(&w);
        w.maintain();

        let chunk_entities = w.fetch::<ChunkEntities>();
        assert!(chunk_entities
            .entities_in_chunk(old_pos.chunk_pos())
            .is_empty());
        assert_eq!(
            chunk_entities.entities_in_chunk(pos.chunk_pos()).as_slice(),
            &[entity]
        );
    }

    #[test]
    fn test_destroyed_entity() {
        let (mut w, mut d) = t::init_world();

        let pos = position!(100.0, -100.0, -100.0);
        let entity = t::add_entity_with_pos(&mut w, EntityType::Player, pos, false);

        {
            let mut chunk_entities = w.fetch_mut::<ChunkEntities>();
            chunk_entities.add_to_chunk(pos.chunk_pos(), entity);
        }

        let event = EntityDestroyEvent { entity };
        t::trigger_event(&w, event);

        d.dispatch(&w);
        w.maintain();

        let chunk_entities = w.fetch::<ChunkEntities>();
        assert!(chunk_entities.entities_in_chunk(pos.chunk_pos()).is_empty());
    }
}
