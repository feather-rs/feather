//! Logic for working with item entities.

use crate::entity::spawn::Spawner;
use crate::entity::PositionComponent;
use crate::player::{PlayerItemDropEvent, PLAYER_EYE_HEIGHT};
use rand::Rng;
use shrev::EventChannel;
use specs::{Component, NullStorage, Read, ReadStorage, ReaderId, System, SystemData, World};

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
        Read<'a, Spawner>,
        Read<'a, EventChannel<PlayerItemDropEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, spawner, item_drop_events) = data;

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

            spawner.spawn_item(pos, velocity, event.stack.clone());
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
}
