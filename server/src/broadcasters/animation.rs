use crate::entity::EntityId;
use crate::game::Game;
use feather_core::network::packet::implementation::AnimationClientbound;
use feather_core::ClientboundAnimation;
use fecs::{Entity, World};

/// Broadcasts animations.
pub fn on_player_animation_broadcast_animation(
    game: &mut Game,
    world: &World,
    player: Entity,
    animation: ClientboundAnimation,
) {
    let packet = AnimationClientbound {
        entity_id: world.get::<EntityId>(player).0,
        animation,
    };
    game.broadcast_entity_update(world, packet, player, Some(player));
}
