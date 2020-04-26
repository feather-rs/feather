use crate::entity::item::ItemCollectEvent;
use crate::entity::EntityId;
use crate::game::Game;
use feather_core::network::packets::CollectItem;
use fecs::World;

/// Sends `CollectItem` packet when an item is collected.
pub fn on_item_collect_broadcast(game: &Game, world: &World, event: &ItemCollectEvent) {
    let packet = CollectItem {
        collected: world.get::<EntityId>(event.item).0,
        collector: world.get::<EntityId>(event.collector).0,
        count: event.amount as i32,
    };

    game.broadcast_entity_update(world, packet, event.item, None);
}
