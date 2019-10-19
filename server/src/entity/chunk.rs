//! Maintains a list of entities which are in each
//! chunk, which allows for more efficient nearby
//! entity queries and packet broadcasting.

use crate::chunk_logic::ChunkLoadEvent;
use crate::entity::{arrow, item, EntityDestroyEvent, EntitySpawnEvent, PositionComponent};
use crate::TickCount;
use feather_core::entity::EntityData;
use feather_core::world::ChunkPosition;
use hashbrown::{HashMap, HashSet};
use shrev::EventChannel;
use specs::storage::ComponentEvent;
use specs::{
    BitSet, Entities, Entity, Join, LazyUpdate, Read, ReadStorage, ReaderId, System, World,
    WorldExt, Write,
};
use std::sync::atomic::{AtomicBool, Ordering};

/// Keeps track of which entities are in which chunk.
/// Also has a boolean for each chunk which indicates
/// whether its entities have been updated recently.
#[derive(Debug, Deref, DerefMut, Default)]
pub struct ChunkEntities(HashMap<ChunkPosition, (AtomicBool, Vec<Entity>)>);

lazy_static! {
    static ref EMPTY_VEC: Vec<Entity> = Vec::with_capacity(0);
}

impl ChunkEntities {
    /// Returns all entities in a given chunk.
    pub fn entities_in_chunk(&self, chunk: ChunkPosition) -> &Vec<Entity> {
        if let Some((_, entities)) = self.0.get(&chunk) {
            entities
        } else {
            &EMPTY_VEC
        }
    }

    /// Returns all entities in the chunk, in addition to
    /// a boolean indicating whether the entities have been
    /// updated since the last call to this function.
    pub fn entities_in_chunk_and_modified(&self, chunk: ChunkPosition) -> (bool, &[Entity]) {
        if let Some((dirty, entities)) = self.0.get(&chunk) {
            let d = dirty.load(Ordering::SeqCst);
            dirty.store(false, Ordering::SeqCst);
            (d, entities)
        } else {
            (false, &[])
        }
    }

    /// Adds an entity to a chunk.
    pub fn add_to_chunk(&mut self, chunk: ChunkPosition, entity: Entity) {
        self.0
            .entry(chunk)
            .and_modify(|(dirty, vec)| {
                dirty.store(true, Ordering::SeqCst);
                vec.push(entity)
            })
            .or_insert_with(|| (AtomicBool::new(true), vec![entity]));
    }

    /// Removes an entity from a chunk.
    ///
    /// # Panics
    /// May panic in some cases if the entity is not contained
    /// within the given chunk.
    pub fn remove_from_chunk(&mut self, chunk: ChunkPosition, entity: Entity) {
        let (dirty, vec) = match self.0.get_mut(&chunk) {
            Some(vec) => vec,
            _ => return,
        };

        let (index, _) = match vec.iter().enumerate().find(|x| *x.1 == entity) {
            Some(index) => index,
            None => return,
        };
        vec.swap_remove(index);

        dirty.store(true, Ordering::SeqCst);

        if vec.is_empty() {
            self.0.remove(&chunk);
        }
    }

    /// Returns a vector of all entities in all chunks
    /// within the given view distance of another chunk.
    pub fn entites_within_view_distance(
        &self,
        chunk: ChunkPosition,
        view_distance: u8,
    ) -> HashSet<Entity> {
        let mut result = HashSet::new();

        // 1 is subtracted from the view distance because of some odd
        // client-side glitch (or maybe it's our fault?) where the last chunk within the view distance
        // is not loaded correctly.
        let view_distance = i32::from(view_distance) - 1;

        for x_offset in -view_distance..=view_distance {
            for z_offset in -view_distance..=view_distance {
                let chunk = ChunkPosition::new(chunk.x + x_offset, chunk.z + z_offset);

                result.extend(self.entities_in_chunk(chunk));
            }
        }

        result
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
            if let ComponentEvent::Modified(id) = event {
                self.dirty.add(*id);
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
            if let Some(pos) = positions.get(event.entity) {
                entity_chunks.add_to_chunk(pos.current.chunk_pos(), event.entity);
            }
        }

        for event in destroy_events.read(self.destroy_reader.as_mut().unwrap()) {
            if let Some(pos) = positions.get(event.entity) {
                entity_chunks.remove_from_chunk(pos.current.chunk_pos(), event.entity);
            }
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

/// System for spawning entities inside newly-loaded chunks.
///
/// This system listens to `ChunkLoadEvent`s.
#[derive(Default)]
pub struct EntityChunkLoadSystem {
    reader: Option<ReaderId<ChunkLoadEvent>>,
}

impl<'a> System<'a> for EntityChunkLoadSystem {
    type SystemData = (
        Read<'a, EventChannel<ChunkLoadEvent>>,
        Read<'a, LazyUpdate>,
        Entities<'a>,
        Read<'a, TickCount>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (load_events, lazy, entities, tick) = data;

        for event in load_events.read(self.reader.as_mut().unwrap()) {
            for entity in &event.entities {
                match entity {
                    EntityData::Item(item_data) => {
                        if item::create_from_data(&lazy, &entities, item_data, &tick).is_none() {
                            debug!("Error while loading item entity");
                        }
                    }
                    EntityData::Arrow(arrow_data) => {
                        if arrow::create_from_data(&lazy, &entities, arrow_data).is_none() {
                            debug!("Error while loading arrow entity");
                        }
                    }
                    // TODO: Spawn remaining entity types here.
                    EntityData::Unknown => {
                        trace!("Chunk {:?} contains an unknown entity type", event.pos);
                    }
                    _ => todo!(),
                }
            }
        }
    }

    setup_impl!(reader);
}

// Tests here cannot use the `testframework::add_entity` function
// because it automatically adds a ChunkEntities entry for the entity.
#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{test, ArrowComponent, ItemComponent};
    use crate::testframework as t;
    use feather_core::entity::{ArrowEntityData, ItemEntityData};
    use specs::{Builder, World, WorldExt};

    #[test]
    fn test_chunk_entities() {
        let mut chunks = ChunkEntities::default();

        let mut world = World::new();
        let entity = world.create_entity().build();

        let pos = ChunkPosition::new(0, 0);

        chunks.add_to_chunk(pos, entity);
        assert_eq!(chunks.entities_in_chunk(pos).as_slice(), &[entity]);
        assert!(chunks.entities_in_chunk_and_modified(pos).0);
        assert!(!chunks.entities_in_chunk_and_modified(pos).0);
    }

    #[test]
    fn test_new_entity() {
        let (mut w, mut d) = t::builder()
            .with(ChunkEntityUpdateSystem::default(), "")
            .build();

        let pos = position!(1.0, 64.0, 1003.5);
        let entity = w
            .create_entity()
            .with(PositionComponent {
                current: pos,
                previous: pos,
            })
            .build();

        let event = EntitySpawnEvent { entity };
        t::trigger_event(&w, event);

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
        let (mut w, mut d) = t::builder()
            .with(ChunkEntityUpdateSystem::default(), "")
            .build();

        let pos = position!(1.0, 64.0, -14.0);
        let old_pos = position!(1.0, 64.0, -18.0);

        let entity = w
            .create_entity()
            .with(PositionComponent {
                current: old_pos,
                previous: old_pos,
            })
            .build();

        // Trigger flagged storage event.
        w.write_component::<PositionComponent>()
            .get_mut(entity)
            .unwrap()
            .current = pos;

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
        let (mut w, mut d) = t::builder()
            .with(ChunkEntityUpdateSystem::default(), "")
            .build();

        let pos = position!(100.0, -100.0, -100.0);
        let entity = test::create(&mut w, pos).build();

        let event = EntityDestroyEvent { entity };
        t::trigger_event(&w, event);

        d.dispatch(&w);
        w.maintain();

        let chunk_entities = w.fetch::<ChunkEntities>();
        assert!(chunk_entities.entities_in_chunk(pos.chunk_pos()).is_empty());
    }

    #[test]
    fn test_entities_within_view_distance() {
        let mut chunk_entities = ChunkEntities::default();

        let mut world = World::new();
        let entity1 = world.create_entity().build();
        let entity2 = world.create_entity().build();
        let entity3 = world.create_entity().build();
        let entity4 = world.create_entity().build();

        let chunk1 = ChunkPosition::new(0, 0);
        let chunk2 = ChunkPosition::new(0, 3);
        let chunk3 = ChunkPosition::new(0, 4);
        let chunk4 = ChunkPosition::new(-3, -3);

        chunk_entities.add_to_chunk(chunk1, entity1);
        chunk_entities.add_to_chunk(chunk2, entity2);
        chunk_entities.add_to_chunk(chunk3, entity3);
        chunk_entities.add_to_chunk(chunk4, entity4);

        let view_distance = 4;
        let entities = chunk_entities.entites_within_view_distance(chunk1, view_distance);

        assert!(entities.contains(&entity1));
        assert!(entities.contains(&entity2));
        assert!(!entities.contains(&entity3));
        assert!(entities.contains(&entity4));
    }

    #[test]
    fn test_entities_loaded_in_chunk() {
        let (mut w, mut d) = t::builder()
            .with(EntityChunkLoadSystem::default(), "")
            .build();

        let entities = vec![
            EntityData::Item(ItemEntityData::default()),
            EntityData::Arrow(ArrowEntityData::default()),
        ];
        let pos = ChunkPosition::new(1, 2);

        let mut entity_spawn_reader = t::reader(&w);
        let load_event = ChunkLoadEvent { pos, entities };
        t::trigger_event(&w, load_event);

        d.dispatch(&w);
        w.maintain();
        d.dispatch(&w);
        w.maintain();

        // Confirm two entities were created: one arrow, one item
        let mut events = t::triggered_events::<EntitySpawnEvent>(&w, &mut entity_spawn_reader);

        let first = events.remove(0).entity;
        let second = events.remove(0).entity;
        assert!(w.read_component::<ItemComponent>().contains(first));
        assert!(w.read_component::<ArrowComponent>().contains(second));
    }
}
