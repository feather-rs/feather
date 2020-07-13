use feather_core::network::packets::AnimationClientbound;
use feather_server_types::{Game, NetworkId, PlayerAnimationEvent};
use fecs::World;

/// Broadcasts animations.
#[fecs::event_handler]
pub fn on_player_animation_broadcast_animation(
    event: &PlayerAnimationEvent,
    game: &mut Game,
    world: &mut World,
) {
    let packet = AnimationClientbound {
        entity_id: world.get::<NetworkId>(event.player).0,
        animation: event.animation,
    };
    game.broadcast_entity_update(world, packet, event.player, Some(event.player));
}
