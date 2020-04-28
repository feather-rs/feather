use feather_core::network::packets::CollectItem;
use feather_server_types::{EntityId, Game, ItemCollectEvent};
use fecs::World;

/// Sends `CollectItem` packet when an item is collected.
#[fecs::event_handler]
pub fn on_item_collect_broadcast(event: &ItemCollectEvent, game: &Game, world: &mut World) {
    let packet = CollectItem {
        collected: world.get::<EntityId>(event.item).0,
        collector: world.get::<EntityId>(event.collector).0,
        count: event.amount as i32,
    };

    game.broadcast_entity_update(world, packet, event.item, None);
}
