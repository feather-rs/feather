//! Broadcasting of chat messages

use feather_core::network::packets::ChatMessageClientbound;
use feather_server_types::{ChatEvent, ChatPosition, Game};
use fecs::World;

/// System that broadcasts chat messages to all players
#[fecs::event_handler]
pub fn on_chat_broadcast(event: &ChatEvent, game: &Game, world: &mut World) {
    let packet = ChatMessageClientbound {
        json_data: event.message.clone(),
        position: match event.position {
            ChatPosition::Chat => 0,
            ChatPosition::SystemMessage => 1,
            ChatPosition::GameInfo => 2,
        },
    };
    game.broadcast_global(world, packet, None);
}
