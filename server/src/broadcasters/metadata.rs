//! Sending of entity metadata.

use crate::entity::EntityId;
use crate::network::Network;
use feather_core::network::packet::implementation::PacketEntityMetadata;
use feather_core::EntityMetadata;
use fecs::{Entity, World};

/// System which sends entity metadata when an entity
/// is sent to a player.
pub fn on_entity_send_send_metadata(world: &World, entity: Entity, client: Entity) {
    if let Some(metadata) = world.try_get::<EntityMetadata>(entity) {
        if let Some(network) = world.try_get::<Network>(client) {
            let entity_id = world.get::<EntityId>(entity).0;
            let packet = PacketEntityMetadata {
                entity_id,
                metadata: (&*metadata).clone(),
            };
            network.send(packet);
        }
    }
}
