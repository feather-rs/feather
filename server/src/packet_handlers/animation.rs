use crate::game::Game;
use crate::packet_buffer::PacketBuffers;
use feather_core::network::packet::implementation::AnimationServerbound;
use feather_core::{ClientboundAnimation, Hand};
use fecs::World;
use std::sync::Arc;

/// Handles animation packets.
#[system]
pub fn handle_animation(game: &mut Game, world: &mut World, packet_buffers: &Arc<PacketBuffers>) {
    packet_buffers
        .received::<AnimationServerbound>()
        .for_each(|(player, packet)| {
            let animation = match packet.hand {
                Hand::Main => ClientboundAnimation::SwingMainArm,
                Hand::Off => ClientboundAnimation::SwingOffhand,
            };

            game.on_player_animation(world, player, animation);
        });
}
