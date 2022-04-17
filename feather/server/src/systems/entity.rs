//! Sends entity-related packets to clients.
//! Spawn packets, position updates, equipment, animations, etc.

use common::entities::player::HotbarSlot;
use common::Game;
use libcraft::{
    entity_metadata::{EntityBitMask, Pose, META_INDEX_ENTITY_BITMASK, META_INDEX_POSE},
    Area, EntityMetadata,
};
use quill::events::HeldItemChangeEvent;
use quill::{
    components::{EntityInventory, EntityPosition, EntityWorld, PlayerGamemode, PreviousGamemode},
    events::InventorySlotUpdateEvent,
};
use quill::{
    components::{OnGround, Sprinting},
    events::{SneakEvent, SprintEvent},
};
use vane::{SysResult, SystemExecutor};

use crate::{
    entities::{PreviousOnGround, PreviousPosition},
    NetworkId, Server,
};

mod spawn_packet;

pub fn register(game: &mut Game, systems: &mut SystemExecutor<Game>) {
    spawn_packet::register(game, systems);
    systems
        .group::<Server>()
        .add_system(send_entity_movement)
        .add_system(send_entity_sneak_metadata)
        .add_system(send_entity_sprint_metadata)
        .add_system(send_entity_equipment);
}

/// Sends entity movement packets.
fn send_entity_movement(game: &mut Game, server: &mut Server) -> SysResult {
    for (entity, (position, mut prev_position, on_ground, network_id, mut prev_on_ground, world)) in
        game.ecs
            .query::<(
                &EntityPosition,
                &mut PreviousPosition,
                &OnGround,
                &NetworkId,
                &mut PreviousOnGround,
                &EntityWorld,
            )>()
            .iter()
    {
        if position.0 != prev_position.0 {
            let world = game.world(world.0)?;
            server.broadcast_nearby_with_mut(world.id(), position.0, |client| {
                client.update_entity_position(
                    *network_id,
                    position.0,
                    *prev_position,
                    *on_ground,
                    *prev_on_ground,
                    &*world,
                    game.ecs.get::<PlayerGamemode>(entity).ok().map(|g| g.0),
                    game.ecs.get::<PreviousGamemode>(entity).ok().map(|g| *g),
                );
            });
            prev_position.0 = position.0;
        }
        if *on_ground != prev_on_ground.0 {
            prev_on_ground.0 = *on_ground;
        }
    }
    Ok(())
}

/// Sends [SendEntityMetadata](protocol::packets::server::play::SendEntityMetadata) packet for when an entity is sneaking.
fn send_entity_sneak_metadata(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, (position, sneak_event, is_sprinting, network_id, world)) in game
        .ecs
        .query::<(
            &EntityPosition,
            &SneakEvent,
            &Sprinting,
            &NetworkId,
            &EntityWorld,
        )>()
        .iter()
    {
        let mut metadata = EntityMetadata::entity_base();
        let mut bit_mask = EntityBitMask::empty();

        // The Entity can sneak and sprint at the same time, what happens is that when it stops sneaking you immediately start running again.
        bit_mask.set(EntityBitMask::CROUCHED, sneak_event.is_sneaking);
        bit_mask.set(EntityBitMask::SPRINTING, is_sprinting.0);
        metadata.set(META_INDEX_ENTITY_BITMASK, bit_mask.bits());

        if sneak_event.is_sneaking {
            metadata.set(META_INDEX_POSE, Pose::Sneaking);
        } else {
            metadata.set(META_INDEX_POSE, Pose::Standing);
        }

        server.broadcast_nearby_with(world.0, position.0, |client| {
            client.send_entity_metadata(*network_id, metadata.clone());
        });
    }
    Ok(())
}

/// Sends [SendEntityMetadata](protocol::packets::server::play::SendEntityMetadata) packet for when an entity is sprinting.
fn send_entity_sprint_metadata(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, (position, sprint_event, network_id, world)) in game
        .ecs
        .query::<(&EntityPosition, &SprintEvent, &NetworkId, &EntityWorld)>()
        .iter()
    {
        let mut metadata = EntityMetadata::entity_base();
        let mut bit_mask = EntityBitMask::empty();

        bit_mask.set(EntityBitMask::SPRINTING, sprint_event.is_sprinting);
        metadata.set(META_INDEX_ENTITY_BITMASK, bit_mask.bits());

        server.broadcast_nearby_with(world.0, position.0, |client| {
            client.send_entity_metadata(*network_id, metadata.clone());
        });
    }
    Ok(())
}

/// Resends entity equipment when inventory slots change.
fn send_entity_equipment(game: &mut Game, server: &mut Server) -> SysResult {
    let mut updated_entities = Vec::new();
    for (_, event) in game.ecs.query::<&InventorySlotUpdateEvent>().iter() {
        if !game.ecs.contains(event.entity) {
            continue;
        }

        if is_equipment_area(event.area) {
            updated_entities.push(event.entity);
        }
    }

    for (entity, _event) in game.ecs.query::<&HeldItemChangeEvent>().iter() {
        updated_entities.push(entity);
    }

    for entity in updated_entities {
        let network_id = *game.ecs.get::<NetworkId>(entity)?;
        let inventory = game.ecs.get::<EntityInventory>(entity)?;
        let hotbar_slot = game
            .ecs
            .get::<HotbarSlot>(entity)
            .map(|r| *r)
            .unwrap_or_else(|_| HotbarSlot::new(0));
        server.broadcast_nearby_with_mut(
            game.ecs.get::<EntityWorld>(entity)?.0,
            game.ecs.get::<EntityPosition>(entity)?.0,
            |client| client.send_entity_equipment(network_id, &inventory, &hotbar_slot),
        );
    }

    Ok(())
}

fn is_equipment_area(area: Area) -> bool {
    matches!(
        area,
        Area::Boots
            | Area::Leggings
            | Area::Chestplate
            | Area::Helmet
            | Area::Offhand
            | Area::Hotbar
    )
}
