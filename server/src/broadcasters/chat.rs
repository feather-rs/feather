//! Broadcasting of chat messages

use crate::chat::{ChatEvent, ChatPosition};
use crate::game::Game;
use feather_core::network::packet::implementation::ChatMessageClientbound;
use feather_core::text::{Text, TextComponent};
use fecs::World;

/// System that broadcasts chat messages to all players
pub fn on_chat_broadcast(game: &Game, world: &World, event: &ChatEvent) {
    let packet = ChatMessageClientbound {
        json_data: String::from(Text::from(TextComponent::from(event.message.clone()))),
        position: match event.position {
            ChatPosition::Chat => 0,
            ChatPosition::SystemMessage => 1,
            ChatPosition::GameInfo => 2,
        },
    };
    game.broadcast_global(world, packet, None);
}
