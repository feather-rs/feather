use crate::chat::{ChatEvent, ChatPosition};
use crate::entity::Name;
use crate::game::Game;
use crate::packet_buffer::PacketBuffers;
use feather_core::network::packet::implementation::ChatMessageServerbound;
use feather_core::text::{TextRoot, Translate};
use fecs::World;
use std::sync::Arc;

/// Handles chat packets.
#[system]
pub fn handle_chat(game: &mut Game, world: &mut World, packet_buffers: &Arc<PacketBuffers>) {
    packet_buffers
        .received::<ChatMessageServerbound>()
        .for_each(|(player, packet)| {
            let player_name = world.get::<Name>(player);
            let message: String = TextRoot::from(
                Translate::ChatTypeText
                    * vec![player_name.0.to_string(), packet.message.to_string()],
            )
            .into();

            info!("<{}> {}", player_name.0, packet.message);
            drop(player_name);

            game.on_chat(
                world,
                ChatEvent {
                    message,
                    position: ChatPosition::Chat,
                },
            );
        });
}
