//! Maintains a list of entities which are in each
//! chunk, which allows for more efficient nearby
//! entity queries and packet broadcasting.

use crate::chunk_logic::ChunkLoadEvent;
use crate::entity::{EntityDestroyEvent, EntitySpawnEvent, PositionComponent};
use crate::util::Util;
use feather_core::entity::EntityData;
use feather_core::world::ChunkPosition;
use feather_core::{Item, ItemStack};
use fnv::FnvHashMap;
use hashbrown::HashSet;
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
        let vec = match self.0.get_mut(&chunk) {
            Some(vec) => vec,
            _ => return,
        };

        let (index, _) = match vec.iter().enumerate().find(|x| *x.1 == entity) {
            Some(index) => index,
            None => return,
        };
        vec.swap_remove(index);

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

#[derive(Default)]
pub struct EntityChunkLoadSystem {
    reader: Option<ReaderId<ChunkLoadEvent>>,
}

impl<'a> System<'a> for EntityChunkLoadSystem {
    type SystemData = (Read<'a, EventChannel<ChunkLoadEvent>>, Read<'a, Util>);

    fn run(&mut self, data: Self::SystemData) {
        let (load_events, util) = data;

        for event in load_events.read(self.reader.as_mut().unwrap()) {
            for entity in &event.entities {
                match entity {
                    EntityData::Item(item_data) => {
                        debug!("Found an item entity: {:?}", item_data);
                        let velocity = {
                            let vel = item_data.entity.velocity.clone();
                            glm::vec3(vel[0], vel[1], vel[2])
                        };
                        util.spawn_item(
                            item_data.entity.read_position().unwrap(),
                            velocity,
                            ItemStack::new(
                                Item::from_identifier(item_data.item.item.as_str())
                                    .unwrap_or(Item::Stone),
                                item_data.item.count,
                            ),
                        )
                    }
                    EntityData::Arrow(arrow_data) => {
                        debug!("Found an arrow entity: {:?}", arrow_data);
                        let velocity = {
                            let vel = arrow_data.entity.velocity.clone();
                            glm::vec3(vel[0], vel[1], vel[2])
                        };
                        util.spawn_arrow(
                            arrow_data.entity.read_position().unwrap(),
                            velocity,
                            arrow_data.critical > 0,
                            None,
                        );
                    }
                    EntityData::Unknown => {
                        trace!("Chunk {:?} contains an unknown entity type", event.pos);
                    }
                }
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        use specs::SystemData;
        Self::SystemData::setup(world);

        self.reader = Some(world.fetch_mut::<EventChannel<_>>().register_reader());
    }
}

// Tests here cannot use the `testframework::add_entity` function
// because it automatically adds a ChunkEntities entry for the entity.
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

        let event = EntitySpawnEvent {
            entity,
            ty: EntityType::Player,
        };
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
        let entity = t::add_entity_with_pos(&mut w, EntityType::Player, pos, false);

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
}
