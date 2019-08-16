//! Maintains a list of entities which are in each
//! chunk, which allows for more efficient nearby
//! entity queries and packet broadcasting.

use crate::entity::{EntityDestroyEvent, EntitySpawnEvent, PositionComponent};
use feather_core::world::ChunkPosition;
use fnv::FnvHashMap;
use shrev::EventChannel;
use specs::storage::ComponentEvent;
use specs::{
    BitSet, Entities, Entity, Join, Read, ReadStorage, ReaderId, System, World, WorldExt, Write,
};

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
    dirty: BitSet,
    move_reader: Option<ReaderId<ComponentEvent>>,
    spawn_reader: Option<ReaderId<EntitySpawnEvent>>,
    destroy_reader: Option<ReaderId<EntityDestroyEvent>>,
}

impl<'a> System<'a> for ChunkEntityUpdateSystem {
    type SystemData = (
        ReadStorage<'a, PositionComponent>,
        Write<'a, ChunkEntities>,
        Read<'a, EventChannel<EntitySpawnEvent>>,
        Read<'a, EventChannel<EntityDestroyEvent>>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, mut entity_chunks, spawn_events, destroy_events, entities) = data;

        self.dirty.clear();
        for event in positions.channel().read(self.move_reader.as_mut().unwrap()) {
            match event {
                ComponentEvent::Inserted(id) | ComponentEvent::Modified(id) => {
                    self.dirty.add(*id);
                }
                _ => (),
            }
        }

        for (position, entity, _) in (&positions, &entities, &self.dirty).join() {
            let new_pos = position.current.chunk_pos();
            let old_pos = position.previous.chunk_pos();

            if new_pos != old_pos {
                entity_chunks.remove_from_chunk(old_pos, entity);
                entity_chunks.add_to_chunk(new_pos, entity);
            }
        }

        for event in spawn_events.read(self.spawn_reader.as_mut().unwrap()) {
            let pos = positions.get(event.entity).unwrap().current;
            entity_chunks.add_to_chunk(pos.chunk_pos(), event.entity);
        }

        for event in destroy_events.read(self.destroy_reader.as_mut().unwrap()) {
            let pos = positions.get(event.entity).unwrap().current;
            entity_chunks.remove_from_chunk(pos.chunk_pos(), event.entity);
        }
    }

    fn setup(&mut self, world: &mut World) {
        use specs::SystemData;

        Self::SystemData::setup(world);

        self.move_reader = Some(
            world
                .write_component::<PositionComponent>()
                .register_reader(),
        );
        self.destroy_reader = Some(world.fetch_mut::<EventChannel<_>>().register_reader());
        self.spawn_reader = Some(world.fetch_mut::<EventChannel<_>>().register_reader());
    }
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
