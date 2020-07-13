use feather_core::network::packets::CollectItem;
use feather_server_types::{Game, ItemCollectEvent, NetworkId};
use fecs::World;

/// Sends `CollectItem` packet when an item is collected.
#[fecs::event_handler]
pub fn on_item_collect_broadcast(event: &ItemCollectEvent, game: &Game, world: &mut World) {
    let packet = CollectItem {
        collected: world.get::<NetworkId>(event.item).0,
        collector: world.get::<NetworkId>(event.collector).0,
        count: event.amount as i32,
    };

    game.broadcast_entity_update(world, packet, event.item, None);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item;
    use feather_core::items::{Item, ItemStack};
    use feather_core::util::Position;
    use feather_test_framework::Test;

    #[test]
    fn broadcast_item_collect() {
        let mut test = Test::new();

        let stack = ItemStack::new(Item::Anvil, 2);
        let item = test.entity(item::create(stack, Default::default()).with(Position::default()));
        let (packet, player) = test.broadcast_routine::<CollectItem, _, _, _>(
            |_test, player1, _player2| ItemCollectEvent {
                item,
                collector: player1,
                amount: 1,
            },
            on_item_collect_broadcast,
            true,
        );

        assert_eq!(packet.collector, test.id(player));
        assert_eq!(packet.collected, test.id(item));
        assert_eq!(packet.count, 1);
    }
}
