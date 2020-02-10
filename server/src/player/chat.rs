use legion::entity::Entity;
use crate::entity::Name;
use legion::query::Read;
use tonks::{PreparedWorld, Query, Trigger};

/// Event triggered when a player sends a chat message
#[derive(Debug, Clone)]
pub struct PlayerChatEvent {
    /// The player that sent the chat message
    pub player: Entity,

    /// The raw message that was sent
    pub message: String
}

/// Event that will result in a chat message being broadcasted
pub struct ChatBroadcastEvent {
    // TODO: Use composable chat component here
    /// A JSON string representing the Chat component to sent
    pub json_data: String,

    /// The position 
    pub position: ChatPosition
}

/// Different positions a chat message can be displayed
pub enum ChatPosition {
    /// Simple message displayed in the chat box
    Chat,

    /// System message displayed in the chat box
    SystemMessage,

    /// A text displayed above the hotbar
    GameInfo
}

/// System that broadcasts chat messages to all players
#[event_handler]
fn broadcast_chat(event: &PlayerChatEvent, world: &mut PreparedWorld, _query: &mut Query<Read<Name>>, trigger: &mut Trigger<ChatBroadcastEvent>) {
    let player_name = &world.get_component::<Name>(event.player)
        .unwrap()
        .0;

    let json_data = json!({
        "translate": "chat.type.text",
        "with": [
            {"text": player_name},
            {"text": event.message}
        ]
    }).to_string();
    
    trigger.trigger(ChatBroadcastEvent {
        json_data,
        position: ChatPosition::Chat
    });

    info!("<{}> {}", player_name, event.message);
}
