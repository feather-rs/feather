//! Handling of item entities.

use crate::entity;
use crate::entity::{EntityId, SpawnPacketCreator, Velocity};
use crate::game::Game;
use crate::physics::PhysicsBuilder;
use crate::player::PLAYER_EYE_HEIGHT;
use crate::util::{degrees_to_stops, protocol_velocity};
use feather_core::inventory::SlotIndex;
use feather_core::network::packet::implementation::SpawnObject;
use feather_core::{EntityMetadata, ItemStack, Packet, Position, META_INDEX_ITEM_SLOT};
use fecs::{Entity, EntityBuilder, EntityRef, World};
use rand::Rng;
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

/*
/// Event triggered when an item is collected.
#[derive(Debug, Clone)]
pub struct ItemCollectEvent {
    /// Item entity which was collected.
    pub item: Entity,
    /// Entity which collected the item.
    pub collector: Entity,
    /// Number of the item which was picked up.
    pub amount: u8,
}

/// Component storing the tick at which an item becomes collectable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CollectableAt(pub u64);

/// Component storing if an item stack has been collected and queued for removal.
pub struct IsRemoved(AtomicBool);
*/

// Item stack of an item entity is stored in `ItemStack` component

/// System for spawning an item entity when
/// an item is dropped.
pub fn on_item_drop_spawn_item_entity(game: &mut Game, world: &mut World, event: &ItemDropEvent) {
    // Spawn item entity.

    // Position is player's eye height minus 0.3
    let mut pos = {
        let player_pos =
            *world.get::<Position>(event.player) + glm::vec3(0.0, PLAYER_EYE_HEIGHT, 0.0);
        player_pos - glm::vec3(0.0f64, 0.3, 0.0)
    };

    pos.on_ground = false;

    let mut rng = game.rng();

    // This velocity calculation was sourced from Glowstone's
    // work. See https://github.com/GlowstoneMC/Glowstone/blob/dev/src/main/java/net/glowstone/entity/GlowHumanEntity.java
    // (method drop(ItemStack stack)) for their code.
    let velocity = {
        let mut vel = glm::DVec3::from_column_slice(&(pos.direction() * 0.3).into_array());
        let rand_offset = 0.02;

        let x = rng.gen_range(0.0, rand_offset) - rand_offset / 2.0;
        let y = rng.gen_range(0.0, 0.12);
        let z = rng.gen_range(0.0, rand_offset) - rand_offset / 2.0;

        vel += glm::vec3(x, y, z);

        vel
    };

    drop(rng);

    let entity = create(game, pos, event.stack)
        .with(Velocity(velocity))
        .build()
        .spawn_in(world);
    game.on_entity_spawn(world, entity);
}

/// Returns an entity builder to create an item entity
/// with the given stack and collectable tick.
pub fn create(_game: &mut Game, pos: Position, stack: ItemStack) -> EntityBuilder {
    let meta = EntityMetadata::entity_base().with(META_INDEX_ITEM_SLOT, Some(stack));

    entity::base(pos)
        .with(stack)
        //.with(CollectableAt(collectable_at))
        .with(SpawnPacketCreator(&create_spawn_packet))
        .with(meta)
        //.with(IsRemoved(AtomicBool::new(false)))
        .with(
            PhysicsBuilder::new()
                .bbox(0.25, 0.25, 0.25)
                .drag(0.98)
                .gravity(-0.04)
                .build(),
        )
}

fn create_spawn_packet(accessor: &EntityRef) -> Box<dyn Packet> {
    let position = *accessor.get::<Position>();
    let velocity = *accessor.get::<Velocity>();
    let entity_id = accessor.get::<EntityId>().0;

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
