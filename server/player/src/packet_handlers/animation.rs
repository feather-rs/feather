use crate::IteratorExt;
use feather_core::network::packets::AnimationServerbound;
use feather_core::util::{ClientboundAnimation, Hand};
use feather_server_types::{Game, PacketBuffers, PlayerAnimationEvent};
use fecs::World;
use std::sync::Arc;

/// Handles animation packets.
#[fecs::system]
pub fn handle_animation(game: &mut Game, world: &mut World, packet_buffers: &Arc<PacketBuffers>) {
    packet_buffers
        .received::<AnimationServerbound>()
        .for_each_valid(world, |world, (player, packet)| {
            let animation = match packet.hand {
                Hand::Main => ClientboundAnimation::SwingMainArm,
                Hand::Off => ClientboundAnimation::SwingOffhand,
            };

            game.handle(world, PlayerAnimationEvent { player, animation });
        });
}
