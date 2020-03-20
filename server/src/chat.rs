use crate::entity::Name;
use crate::game::Game;
use fecs::{Entity, World};

/// Event triggered when a chat message is sent out
#[derive(Debug, Clone)]
pub struct ChatEvent {
    /// The JSON-formatted message
    pub message: String,

    /// The position of the message
    pub position: ChatPosition,
}

/// Different positions a chat message can be displayed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatPosition {
    /// Simple message displayed in the chat box
    Chat,

    /// System message displayed in the chat box
    SystemMessage,

    /// A text displayed above the hotbar
    GameInfo,
}

pub fn on_player_join_broadcast_join_message(game: &mut Game, world: &mut World, player: Entity) {
    let message = {
        let name = world.get::<Name>(player);
        json!({
            "translate": "multiplayer.player.joined",
            "color": "yellow",
            "with": [
                { "text": name.0 },
            ],
        })
        .to_string()
    };

    game.on_chat(
        world,
        ChatEvent {
            message,
            position: ChatPosition::Chat,
        },
    );
}
