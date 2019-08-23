use shrev::EventChannel;
use specs::SystemData;
use specs::{Entities, Read, ReadStorage, ReaderId, System, World, Write};

use feather_core::network::cast_packet;
use feather_core::network::packet::implementation::{
    ChatMessageClientbound, ChatMessageServerbound,
};
use feather_core::network::packet::PacketType;

use crate::entity::NamedComponent;
use crate::network::{send_packet_to_all_players, NetworkComponent, PacketQueue};

/// Event which is triggered when a new chat message is to be broadcasted to the whole server.
#[derive(Debug, Clone)]
pub struct ChatBroadcastEvent {
    pub message: String,
}

/// System for handling Chat Message Serverbound packets
/// and then triggering a `ChatBroadcastEvent`.
pub struct PlayerChatSystem;

impl<'a> System<'a> for PlayerChatSystem {
    type SystemData = (
        Write<'a, EventChannel<ChatBroadcastEvent>>,
        ReadStorage<'a, NamedComponent>,
        Read<'a, PacketQueue>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut events, nameds, packet_queue) = data;

        // Handle Chat Message Serverbound packets.
        let packets = packet_queue.for_packet(PacketType::ChatMessageServerbound);

        for (player, packet) in packets {
            let packet = cast_packet::<ChatMessageServerbound>(&*packet);
            let message = packet.message.clone();
            let player_name = &nameds.get(player).unwrap().display_name;

            // TODO: could use a more robust chat-component library.
            let message_json = json!({
                "translate": "chat.type.text",
                "with": [
                    {"text": player_name},
                    {"text": message},
                ],
            })
            .to_string();

            let event = ChatBroadcastEvent {
                message: message_json,
            };
            events.single_write(event);

            // Log in the console
            info!("<{}> {}", player_name, message);
        }
    }
}

/// System for broadcasting chat messages.
/// This system listens to `ChatBroadcastEvent`s.
#[derive(Default)]
pub struct ChatBroadcastSystem {
    reader: Option<ReaderId<ChatBroadcastEvent>>,
}

impl<'a> System<'a> for ChatBroadcastSystem {
    type SystemData = (
        Read<'a, EventChannel<ChatBroadcastEvent>>,
        ReadStorage<'a, NetworkComponent>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (events, networks, entities) = data;

        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            let message = event.message.clone();

            // Broadcast chat message
            let packet = ChatMessageClientbound {
                json_data: message,
                position: 0,
            };

            send_packet_to_all_players(&networks, &entities, packet, None);
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(
            world
                .fetch_mut::<EventChannel<ChatBroadcastEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testframework as t;
    use feather_core::network::packet::implementation::ChatMessageServerbound;
    use specs::WorldExt;

    #[test]
    fn test_chat_system() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        let packet = ChatMessageServerbound {
            message: String::from("test"),
        };
        t::receive_packet(&player, &w, packet);

        let mut event_reader = t::reader::<ChatBroadcastEvent>(&w);

        d.dispatch(&w);
        w.maintain();

        let channel = w.fetch::<EventChannel<ChatBroadcastEvent>>();

        let events = channel.read(&mut event_reader).collect::<Vec<_>>();
        assert_eq!(events.len(), 1);
    }

    #[test]
    fn test_chat_broadcast_system() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);
        let player2 = t::add_player(&mut w);

        let event = ChatBroadcastEvent {
            message: String::from("test"),
        };

        t::trigger_event(&w, event.clone());

        d.dispatch(&w);
        w.maintain();

        t::assert_packet_received(&player, PacketType::ChatMessageClientbound);
        let packet = t::assert_packet_received(&player2, PacketType::ChatMessageClientbound);
        let packet = cast_packet::<ChatMessageClientbound>(&*packet);
        assert_eq!(packet.json_data, String::from("test"));
    }
}
