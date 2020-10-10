use crate::IteratorExt;
use feather_core::network::packets::ChatMessageServerbound;
use feather_core::text::{TextRoot, Translate};
use feather_server_commands::CommandState;
use feather_server_types::{ChatEvent, ChatPosition, Game, Name, PacketBuffers};
use fecs::World;
use std::sync::Arc;

/// Handles chat packets.
#[fecs::system]
pub fn handle_chat(
    game: &mut Game,
    world: &mut World,
    packet_buffers: &Arc<PacketBuffers>,
    #[default] commands: &CommandState,
) {
    packet_buffers
        .received::<ChatMessageServerbound>()
        .for_each_valid(world, |world, (player, packet)| {
            if packet.message.starts_with('/') {
                log::info!(
                    "Player `{}` executed command `{}`",
                    world.get::<Name>(player).0,
                    packet.message
                );
                commands.dispatch(game, world, player, &packet.message[1..]);
            } else {
                let player_name = world.get::<Name>(player);
                let message: String = TextRoot::from(
                    Translate::ChatTypeText
                        * vec![player_name.0.to_string(), packet.message.to_string()],
                )
                .into();

                log::info!("<{}> {}", player_name.0, packet.message);
                drop(player_name);

                game.handle(
                    world,
                    ChatEvent {
                        message,
                        position: ChatPosition::Chat,
                    },
                );
            }
        });
}
