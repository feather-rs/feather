//! Broadcasting of chat messages

use crate::player::chat::{ChatBroadcastEvent, ChatPosition};
use crate::state::State;
use feather_core::network::packet::implementation::ChatMessageClientbound;

/// System that broadcasts chat messages to all players
#[event_handler]
fn broadcast_chat(event: &ChatBroadcastEvent, state: &State) {
    let packet = ChatMessageClientbound {
        json_data: event.json_data.clone(),
        position: match event.position {
            ChatPosition::Chat => 0,
            ChatPosition::SystemMessage => 1,
            ChatPosition::GameInfo => 2,
        },
    };
    state.broadcast_global(packet, None);
}
