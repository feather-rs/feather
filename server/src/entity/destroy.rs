//! Module for broadcasting and handling entity destroy
//! events.

use crate::network::{send_packet_to_player, NetworkComponent};
use feather_core::network::packet::implementation::DestroyEntities;
use shrev::{EventChannel, ReaderId};
use specs::SystemData;
use specs::{Entities, Entity, Join, Read, ReadStorage, System, World, Write};

/// Event triggered when an entity
/// of any type is destroyed.
pub struct EntityDestroyEvent {
    /// Note that when this event is triggered,
    /// the entity isn't actually removed from the world
    /// yet. This allows systems to access the entity's
    /// data before it is destroyed.
    ///
    /// `EntityDestroySystem` is responsible for removing
    /// entities once the `EntityDestroyEvent` has been
    /// handled by all readers.
    pub entity: Entity,
}

/// System for removing entities from the world when they
/// are destroyed.
#[derive(Default)]
pub struct EntityDestroySystem {
    reader: Option<ReaderId<EntityDestroyEvent>>,
}

impl EntityDestroySystem {
    pub fn new() -> Self {
        Self { reader: None }
    }
}

impl<'a> System<'a> for EntityDestroySystem {
    type SystemData = (Write<'a, EventChannel<EntityDestroyEvent>>, Entities<'a>);

    fn run(&mut self, data: Self::SystemData) {
        let (events, entities) = data;

        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            entities.delete(event.entity).unwrap();
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(
            world
                .fetch_mut::<EventChannel<EntityDestroyEvent>>()
                .register_reader(),
        );
    }
}

/// System for broadcasting when an entity is destroyed.
#[derive(Default)]
pub struct EntityDestroyBroadcastSystem {
    reader: Option<ReaderId<EntityDestroyEvent>>,
}

impl EntityDestroyBroadcastSystem {
    pub fn new() -> Self {
        Self { reader: None }
    }
}

impl<'a> System<'a> for EntityDestroyBroadcastSystem {
    type SystemData = (
        ReadStorage<'a, NetworkComponent>,
        Read<'a, EventChannel<EntityDestroyEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (net_comps, events) = data;

        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            let destroy_entities = DestroyEntities::new(vec![event.entity.id() as i32]);

            // TODO only broadcast to nearby
            for net in net_comps.join() {
                send_packet_to_player(net, destroy_entities.clone());
            }
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(
            world
                .fetch_mut::<EventChannel<EntityDestroyEvent>>()
                .register_reader(),
        );
    }
}
