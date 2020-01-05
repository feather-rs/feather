use crate::entity::EntityId;
use crate::player::PlayerAnimationEvent;
use crate::state::State;
use feather_core::network::packet::implementation::AnimationClientbound;
use legion::query::Read;
use tonks::{PreparedWorld, Query};

/// Broadcasts animations.
#[event_handler]
fn broadcast_animation(
    event: &PlayerAnimationEvent,
    state: &State,
    _query: &mut Query<Read<EntityId>>,
    world: &mut PreparedWorld,
) {
    let packet = AnimationClientbound {
        entity_id: world.get_component::<EntityId>(event.player).unwrap().0,
        animation: event.animation,
    };
    state.broadcast_entity_update(event.player, packet, Some(event.player));
}
