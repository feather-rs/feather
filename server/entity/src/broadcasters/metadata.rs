//! Sending of entity metadata.

use feather_core::entitymeta::EntityMetadata;
use feather_core::network::packets::PacketEntityMetadata;
use feather_server_types::{EntityId, EntitySendEvent, Network};
use fecs::World;

/// System which sends entity metadata when an entity
/// is sent to a player.
#[fecs::event_handler]
pub fn on_entity_send_send_metadata(event: &EntitySendEvent, world: &mut World) {
    if let Some(metadata) = world.try_get::<EntityMetadata>(event.entity) {
        if let Some(network) = world.try_get::<Network>(event.client) {
            let entity_id = world.get::<EntityId>(event.entity).0;
            let packet = PacketEntityMetadata {
                entity_id,
                metadata: (&*metadata).clone(),
            };
            network.send(packet);
        }
    }
}
