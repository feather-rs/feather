//! Logic for working with item entities.

use crate::entity::metadata::{self, Metadata};
use crate::entity::{EntityComponent, EntitySpawnEvent, EntityType, VelocityComponent};
use crate::player::PlayerItemDropEvent;
use rand::Rng;
use shrev::EventChannel;
use specs::{Entities, Read, ReaderId, System, SystemData, World, Write, WriteStorage};
use uuid::Uuid;

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
        WriteStorage<'a, EntityComponent>,
        WriteStorage<'a, VelocityComponent>,
        WriteStorage<'a, EntityType>,
        WriteStorage<'a, Metadata>,
        Write<'a, EventChannel<EntitySpawnEvent>>,
        Read<'a, EventChannel<PlayerItemDropEvent>>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut entity_comps,
            mut velocities,
            mut types,
            mut metadatas,
            mut spawn_events,
            item_drop_events,
            entities,
        ) = data;

        let mut rng = rand::thread_rng();

        for event in item_drop_events.read(self.reader.as_mut().unwrap()) {
            // Spawn item entity.
            let metadata = {
                let mut item = metadata::Item::default();
                item.set_item(Some(event.stack.clone()));
                Metadata::Item(item)
            };

            let entity = entities.create();

            let pos = {
                let player_pos = entity_comps.get(event.player).unwrap().position;
                player_pos - glm::vec3(0.0, 0.3, 0.0)
            };

            let entity_comp = EntityComponent {
                uuid: Uuid::new_v4(),
                display_name: String::with_capacity(0),
                position: pos,
                on_ground: false,
            };

            entity_comps.insert(entity, entity_comp).unwrap();
            types.insert(entity, EntityType::Item).unwrap();
            metadatas.insert(entity, metadata).unwrap();

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

                VelocityComponent(vel)
            };

            velocities.insert(entity, velocity).unwrap();

            // Trigger event
            let event = EntitySpawnEvent {
                entity,
                ty: EntityType::Item,
            };
            spawn_events.single_write(event);
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(world.fetch_mut::<EventChannel<_>>().register_reader());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testframework as t;
    use feather_core::world::Position;
    use feather_core::{Item, ItemStack};
    use specs::WorldExt;

    #[test]
    fn test_item_spawn_system() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);
        t::set_entity_pos(&w, player.entity, Position::new(0.0, 1.0, 0.0, 0.0, 0.0));

        let stack = ItemStack::new(Item::AcaciaBoat, 4);

        let mut entity_spawn_reader = t::reader(&w);

        let event = PlayerItemDropEvent {
            slot: None,
            stack,
            player: player.entity,
        };
        t::trigger_event(&mut w, event);

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
        assert_float_eq!(pos.y, 0.7);
        assert_float_eq!(pos.z, 0.0);

        // Confirm that velocity was created
        let _vel = t::entity_vel(&w, entity).unwrap();
    }
}
