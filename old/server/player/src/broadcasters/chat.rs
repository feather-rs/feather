//! Broadcasting of chat messages

use feather_core::network::packets::ChatMessageClientbound;
use feather_server_types::{ChatEvent, ChatPosition, Game, MessageReceiver, Network, Player};
use fecs::{component, IntoQuery, Read, World, Write};

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

/// System to flush a players `MessageReceiver` component and send the messages.
#[fecs::system]
pub fn flush_player_message_receiver(world: &mut World) {
    <(Write<MessageReceiver>, Read<Network>)>::query()
        .filter(component::<Player>())
        .par_for_each_mut(world.inner_mut(), |(mut receiver, network)| {
            for message in receiver.flush() {
                network.send(ChatMessageClientbound {
                    json_data: message.to_string(),
                    position: 0,
                });
            }
        });
}
