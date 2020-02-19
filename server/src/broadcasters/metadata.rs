//! Sending of entity metadata.

use crate::entity::{EntityId, EntitySendEvent};
use crate::metadata::Metadata;
use crate::network::Network;
use feather_core::network::packet::implementation::PacketEntityMetadata;
use legion::query::Read;
use tonks::{PreparedWorld, Query};

/// System which sends entity metadata when an entity
/// is sent to a player.
#[event_handler]
fn send_entity_metadata(
    event: &EntitySendEvent,
    _query: &mut Query<(Read<EntityId>, Read<Network>, Read<Metadata>)>,
    world: &mut PreparedWorld,
) {
    if let Some(meta) = world.get_component::<Metadata>(event.entity) {
        if let Some(network) = world.get_component::<Network>(event.to) {
            let entity_id = world.get_component::<EntityId>(event.entity).unwrap().0;
            let packet = PacketEntityMetadata {
                entity_id,
                metadata: meta.to_full_raw_metadata(),
            };
            network.send(packet);
        }
    }
}
