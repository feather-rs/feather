//! Sends entity-related packets to clients.
//! Spawn packets, position updates, equipment, animations, etc.

use base::{
    metadata::{EntityBitMask, Pose, META_INDEX_ENTITY_BITMASK, META_INDEX_POSE},
    EntityMetadata, Position,
};
use common::Game;
use ecs::{SysResult, SystemExecutor};
use quill_common::{
    components::{OnGround, Sprinting},
    events::{SneakEvent, SprintEvent},
};

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
        .add_system(send_entity_sprint_metadata);
}

/// Sends entity movement packets.
fn send_entity_movement(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, (&position, prev_position, &on_ground, &network_id, prev_on_ground)) in game
        .ecs
        .query::<(
            &Position,
            &mut PreviousPosition,
            &OnGround,
            &NetworkId,
            &mut PreviousOnGround,
        )>()
        .iter()
    {
        if position != prev_position.0 {
            server.broadcast_nearby_with(position, |client| {
                client.update_entity_position(
                    network_id,
                    position,
                    *prev_position,
                    on_ground,
                    *prev_on_ground,
                );
            });
            prev_position.0 = position;
        }
        if on_ground != prev_on_ground.0 {
            prev_on_ground.0 = on_ground;
        }
    }
    Ok(())
}

/// Sends [SendEntityMetadata](protocol::packets::server::play::SendEntityMetadata) packet for when an entity is sneaking.
fn send_entity_sneak_metadata(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, (&position, &SneakEvent { is_sneaking }, is_sprinting, &network_id)) in game
        .ecs
        .query::<(&Position, &SneakEvent, &Sprinting, &NetworkId)>()
        .iter()
    {
        let mut metadata = EntityMetadata::entity_base();
        let mut bit_mask = EntityBitMask::empty();

        // The Entity can sneak and sprint at the same time, what happens is that when it stops sneaking you immediately start running again.
        bit_mask.set(EntityBitMask::CROUCHED, is_sneaking);
        bit_mask.set(EntityBitMask::SPRINTING, is_sprinting.0);
        metadata.set(META_INDEX_ENTITY_BITMASK, bit_mask.bits());

        if is_sneaking {
            metadata.set(META_INDEX_POSE, Pose::Sneaking);
        } else {
            metadata.set(META_INDEX_POSE, Pose::Standing);
        }

        server.broadcast_nearby_with(position, |client| {
            client.send_entity_metadata(network_id, metadata.clone());
        });
    }
    Ok(())
}

/// Sends [SendEntityMetadata](protocol::packets::server::play::SendEntityMetadata) packet for when an entity is sprinting.
fn send_entity_sprint_metadata(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, (&position, &SprintEvent { is_sprinting }, &network_id)) in game
        .ecs
        .query::<(&Position, &SprintEvent, &NetworkId)>()
        .iter()
    {
        let mut metadata = EntityMetadata::entity_base();
        let mut bit_mask = EntityBitMask::empty();

        bit_mask.set(EntityBitMask::SPRINTING, is_sprinting);
        metadata.set(META_INDEX_ENTITY_BITMASK, bit_mask.bits());

        server.broadcast_nearby_with(position, |client| {
            client.send_entity_metadata(network_id, metadata.clone());
        });
    }
    Ok(())
}
