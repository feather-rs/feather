//! Logic for working with item entities.
use crate::entity::metadata::{self, Metadata};
use crate::entity::{ChunkEntities, EntityDestroyEvent, PositionComponent};
use crate::physics::nearby_entities;
use crate::player::{PlayerItemDropEvent, PLAYER_EYE_HEIGHT};
use crate::util::Util;
use feather_core::ItemStack;
use rand::Rng;
use shrev::EventChannel;
use smallvec::SmallVec;
use specs::storage::ComponentEvent;
use specs::{
    BitSet, Component, Entities, Entity, Join, NullStorage, Read, ReadStorage, ReaderId, System,
    SystemData, World, Write, WriteStorage,
};

/// Marker component, used to mark entities as items.
#[derive(Default)]
pub struct ItemMarker;

impl Component for ItemMarker {
    type Storage = NullStorage<Self>;
}

/// System for spawning an item entity when
/// an item is dropped.
///
/// This system listens to `PlayerItemDropEvent`s.
#[derive(Default)]
pub struct ItemSpawnSystem {
    reader: Option<ReaderId<PlayerItemDropEvent>>,
}

impl<'a> System<'a> for ItemSpawnSystem {
    type SystemData = (
        ReadStorage<'a, PositionComponent>,
        Read<'a, Util>,
        Read<'a, EventChannel<PlayerItemDropEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, util, item_drop_events) = data;

        let mut rng = rand::thread_rng();

        for event in item_drop_events.read(self.reader.as_mut().unwrap()) {
            // Spawn item entity.

            // Position is player's eye height minus 0.3
            let pos = {
                let player_pos = positions.get(event.player).unwrap().current
                    + glm::vec3(0.0, PLAYER_EYE_HEIGHT, 0.0);
                player_pos - glm::vec3(0.0, 0.3, 0.0)
            };

            // This velocity calculation was sourced from Glowstone's
            // work. See https://github.com/GlowstoneMC/Glowstone/blob/dev/src/main/java/net/glowstone/entity/GlowHumanEntity.java
            // (method drop(ItemStack stack)) for their code.
            let velocity = {
                let mut vel = pos.direction() * 0.3;
                let rand_offset = 0.02;

                let x = rng.gen_range(0.0, rand_offset) - rand_offset / 2.0;
                let y = rng.gen_range(0.0, 0.12);
                let z = rng.gen_range(0.0, rand_offset) - rand_offset / 2.0;

                vel += glm::vec3(x, y, z);

                vel
            };

            util.spawn_item(pos, velocity, event.stack.clone());
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(world.fetch_mut::<EventChannel<_>>().register_reader());
    }
}

/// System for merging item entities of the same
/// type.
#[derive(Default)]
pub struct ItemMergeSystem {
    dirty: BitSet,
    reader: Option<ReaderId<ComponentEvent>>,
}

impl<'a> System<'a> for ItemMergeSystem {
    type SystemData = (
        ReadStorage<'a, PositionComponent>,
        ReadStorage<'a, ItemMarker>,
        WriteStorage<'a, Metadata>,
        Write<'a, EventChannel<EntityDestroyEvent>>,
        Read<'a, ChunkEntities>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, item_markers, mut metadatas, mut destroy_events, chunk_entities, entities) =
            data;

        self.dirty.clear();

        for event in positions.channel().read(self.reader.as_mut().unwrap()) {
            match event {
                ComponentEvent::Modified(id) | ComponentEvent::Inserted(id) => {
                    self.dirty.add(*id);
                }
                _ => (),
            }
        }

        let mut metadatas_to_update: SmallVec<[(Entity, Metadata); 2]> = smallvec![];
        // Used to not destroy both entities
        let mut destroyed: SmallVec<[Entity; 2]> = smallvec![];

        for (position, entity, _, _) in (&positions, &entities, &item_markers, &self.dirty).join() {
            if !entities.is_alive(entity) {
                continue;
            }

            if destroyed.iter().any(|x| *x == entity) {
                continue;
            }

            let mut stack = item_stack_from_meta(metadatas.get(entity).unwrap());

            // Find nearby entities and check if they are of the same item
            // type. If so, merge the two item stacks.
            let nearby = nearby_entities(
                &chunk_entities,
                &positions,
                position.current,
                glm::vec3(1.0, 0.5, 1.0),
            );

            for other in nearby {
                // Skip entity if it's dead.
                if !entities.is_alive(other) {
                    continue;
                }

                if other == entity {
                    continue;
                }

                // Skip if it's not an item.
                if item_markers.get(other).is_none() {
                    continue;
                }

                let other_stack = item_stack_from_meta(metadatas.get(other).unwrap());

                if other_stack.ty != stack.ty {
                    continue;
                }

                // Merge two stacks.
                // This works by deleting `other` and adding
                // together the amounts of the two item stacks.
                entities.delete(other).unwrap();

                let event = EntityDestroyEvent { entity: other };
                destroy_events.single_write(event);

                // TODO this could overflow...
                stack.amount += other_stack.amount;

                metadatas_to_update.push((entity, item_meta(stack.clone())));
                destroyed.push(other);
            }
        }

        metadatas_to_update.into_iter().for_each(|(entity, meta)| {
            metadatas.insert(entity, meta).unwrap();
        });
    }

    flagged_setup_impl!(PositionComponent, reader);
}

pub fn item_stack_from_meta(meta: &Metadata) -> ItemStack {
    match meta {
        Metadata::Item(item) => item.item().unwrap().clone(),
        _ => panic!(),
    }
}

pub fn item_meta(stack: ItemStack) -> Metadata {
    let mut item = metadata::Item::default();
    item.set_item(Some(stack));
    Metadata::Item(item)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::chunk::ChunkEntityUpdateSystem;
    use crate::entity::EntitySpawnEvent;
    use crate::entity::EntityType;
    use crate::testframework as t;
    use feather_core::world::Position;
    use feather_core::{Item, ItemStack};
    use specs::WorldExt;

    #[test]
    fn test_item_spawn_system() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);
        t::set_entity_pos(&w, player.entity, position!(0.0, 1.0, 0.0));

        let stack = ItemStack::new(Item::AcaciaBoat, 4);

        let mut entity_spawn_reader = t::reader(&w);

        let event = PlayerItemDropEvent {
            slot: None,
            stack,
            player: player.entity,
        };
        t::trigger_event(&w, event);

        d.dispatch(&w);
        w.maintain();

        // Confirm event was triggered
        let events = t::triggered_events::<EntitySpawnEvent>(&w, &mut entity_spawn_reader);
        assert_eq!(events.len(), 1);
        let first = events.first().unwrap();
        let entity = first.entity;
        assert_eq!(first.ty, EntityType::Item);

        // Check position
        let pos = t::entity_pos(&w, entity);
        assert_float_eq!(pos.x, 0.0);
        assert_float_eq!(pos.z, 0.0);

        // Confirm that velocity was created
        let _vel = t::entity_vel(&w, entity).unwrap();
    }

    #[test]
    fn test_item_merge_system() {
        let (mut w, mut d) = t::builder()
            .with(ChunkEntityUpdateSystem::default(), "chunk_entity_update")
            .with_dep(
                ItemMergeSystem::default(),
                "item_merge",
                &["chunk_entity_update"],
            ) // Required so nearby_entities() works
            .build();

        let item1 =
            t::add_entity_with_pos(&mut w, EntityType::Item, position!(0.0, 0.0, 0.0), true);
        let item2 =
            t::add_entity_with_pos(&mut w, EntityType::Item, position!(1.0, 0.4, 1.0), true);

        {
            let mut metadatas = w.write_component::<Metadata>();

            metadatas
                .insert(item1, item_meta(ItemStack::new(Item::EnderPearl, 4)))
                .unwrap();
            metadatas
                .insert(item2, item_meta(ItemStack::new(Item::EnderPearl, 7)))
                .unwrap();
        }

        d.dispatch(&w);
        w.maintain();

        assert!(!w.is_alive(item2));
        assert!(w.is_alive(item1));

        let metadatas = w.read_component::<Metadata>();
        let metadata = metadatas.get(item1).unwrap();

        let stack = item_stack_from_meta(&metadata);
        assert_eq!(stack.ty, Item::EnderPearl);
        assert_eq!(stack.amount, 11);
    }
}
