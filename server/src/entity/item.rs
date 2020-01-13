//! Handling of item entities.

use crate::entity::{EntityId, SpawnPacketCreator, Velocity};
use crate::lazy::EntityBuilder;
use crate::player::PLAYER_EYE_HEIGHT;
use crate::state::State;
use crate::util::{degrees_to_stops, protocol_velocity};
use crate::{entity, TickCount, TPS};
use feather_core::inventory::SlotIndex;
use feather_core::network::packet::implementation::SpawnObject;
use feather_core::{ItemStack, Packet, Position};
use legion::entity::Entity;
use legion::query::Read;
use rand::Rng;
use tonks::{EntityAccessor, PreparedWorld, Query};
use uuid::Uuid;

/// Event triggered when an item is dropped.
///
/// Before this event is triggered, the item
/// is removed from the player's inventory.
#[derive(Debug, Clone)]
pub struct ItemDropEvent {
    /// The slot from which the item was dropped,
    /// if known.
    pub slot: Option<SlotIndex>,
    /// The item stack which was dropped.
    pub stack: ItemStack,
    /// The player who dropped the item.
    pub player: Entity,
}

/// Component storing the tick at which an item becomes collectable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CollectableAt(pub u64);

// Item stack of an item entity is stored in `ItemStack` component

/// System for spawning an item entity when
/// an item is dropped.
#[event_handler]
pub fn item_spawn(
    event: &ItemDropEvent,
    state: &State,
    _query: &mut Query<Read<Position>>,
    world: &mut PreparedWorld,
    tick: &TickCount,
) {
    let mut rng = rand::thread_rng();

    // Spawn item entity.

    // Position is player's eye height minus 0.3
    let mut pos = {
        let player_pos = *world.get_component::<Position>(event.player).unwrap()
            + glm::vec3(0.0, PLAYER_EYE_HEIGHT, 0.0);
        player_pos - glm::vec3(0.0f64, 0.3, 0.0)
    };

    pos.on_ground = false;

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

    create(state, pos, event.stack.clone(), tick.0 + TPS)
        .with_component(velocity)
        .build();
}

/// Returns an entity builder to create an item entity
/// with the given stack and collectable tick.
pub fn create(
    state: &State,
    pos: Position,
    stack: ItemStack,
    collectable_at: u64,
) -> EntityBuilder {
    entity::base(state, pos)
        .with_component(stack)
        .with_component(CollectableAt(collectable_at))
        .with_component(SpawnPacketCreator(&create_spawn_packet))
}

fn create_spawn_packet(accessor: &EntityAccessor, world: &PreparedWorld) -> Box<dyn Packet> {
    let position = *accessor.get_component::<Position>(world).unwrap();
    let velocity = *accessor.get_component::<Velocity>(world).unwrap();
    let entity_id = accessor.get_component::<EntityId>(world).unwrap().0;

    let (velocity_x, velocity_y, velocity_z) = protocol_velocity(velocity.0);

    let packet = SpawnObject {
        entity_id,
        object_uuid: Uuid::new_v4(),
        ty: 2, // Type 2 for item stack
        x: position.x,
        y: position.y,
        z: position.z,
        pitch: degrees_to_stops(position.pitch),
        yaw: degrees_to_stops(position.yaw),
        data: 1, // Has velocity
        velocity_x,
        velocity_y,
        velocity_z,
    };

    Box::new(packet)
}
