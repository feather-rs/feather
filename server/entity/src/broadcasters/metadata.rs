//! Sending of entity metadata.

use feather_core::entitymeta::EntityMetadata;
use feather_core::network::packets::PacketEntityMetadata;
use feather_server_types::{EntitySendEvent, Network, NetworkId};
use fecs::World;

/// System which sends entity metadata when an entity
/// is sent to a player.
#[fecs::event_handler]
pub fn on_entity_send_send_metadata(event: &EntitySendEvent, world: &mut World) {
    if let Some(metadata) = world.try_get::<EntityMetadata>(event.entity) {
        if let Some(network) = world.try_get::<Network>(event.client) {
            let entity_id = world.get::<NetworkId>(event.entity).0;
            let packet = PacketEntityMetadata {
                entity_id,
                metadata: (&*metadata).clone(),
            };
            network.send(packet);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item;
    use feather_core::entitymeta::{MetaEntry, META_INDEX_ITEM_SLOT};
    use feather_core::items::{Item, ItemStack};
    use feather_core::util::Position;
    use feather_test_framework::Test;

    #[test]
    fn send_metadata() {
        let mut test = Test::new();

        let player1 = test.player("", position!(0.0, 64.0, 0.0));
        let player2 = test.player("", position!(0.0, 100.0, 0.0));

        let stack = ItemStack::new(Item::String, 4);
        let item = test.entity(item::create(stack, Default::default()).with(Position::default()));

        test.handle(
            EntitySendEvent {
                client: player1,
                entity: item,
            },
            on_entity_send_send_metadata,
        );

        let packet = test.sent::<PacketEntityMetadata>(player1).unwrap();
        assert_eq!(packet.entity_id, test.id(item));
        assert_eq!(
            packet.metadata.get(META_INDEX_ITEM_SLOT),
            Some(MetaEntry::Slot(Some(stack)))
        );

        assert!(test.sent::<PacketEntityMetadata>(player2).is_none());
    }
}
