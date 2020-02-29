use crate::entity::item::ItemCollectEvent;
use crate::entity::EntityId;
use crate::state::State;
use feather_core::network::packet::implementation::CollectItem;
use legion::query::Read;
use tonks::{PreparedWorld, Query};

/// Sends `CollectItem` packet when an item is collected.
#[event_handler]
pub fn broadcast_item_collect(
    event: &ItemCollectEvent,
    state: &State,
    _query: &mut Query<Read<EntityId>>,
    world: &mut PreparedWorld,
) {
    let packet = CollectItem {
        collected: world.get_component::<EntityId>(event.item).unwrap().0,
        collector: world.get_component::<EntityId>(event.item).unwrap().0,
        count: event.amount as i32,
    };

    // TODO: broadcast for item instead
    state.broadcast_entity_update(event.collector, packet, None);
}
