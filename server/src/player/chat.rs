use shrev::EventChannel;
use specs::SystemData;
use specs::{Entities, Entity, Read, ReadStorage, ReaderId, System, World, Write};

use feather_core::network::cast_packet;
use feather_core::network::packet::implementation::{
    ChatMessageClientbound, ChatMessageServerbound,
};
use feather_core::network::packet::PacketType;

use crate::entity::NamedComponent;
use crate::network::{send_packet_to_all_players, NetworkComponent, PacketQueue};

/// Event which is triggered when a player sends a chat message.
#[derive(Debug, Clone)]
pub struct PlayerChatEvent {
    pub player: Entity,
    pub message: String,
}

/// System for handling Chat Message Serverbound packets
/// and then triggering a `PlayerChatEvent`.
pub struct PlayerChatSystem;

impl<'a> System<'a> for PlayerChatSystem {
    type SystemData = (
        Write<'a, EventChannel<PlayerChatEvent>>,
        Read<'a, PacketQueue>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut events, packet_queue) = data;

        // Handle Animation Serverbound packets.
        let packets = packet_queue.for_packet(PacketType::ChatMessageServerbound);

        for (player, packet) in packets {
            let packet = cast_packet::<ChatMessageServerbound>(&*packet);
            let message = packet.message.clone();

            let event = PlayerChatEvent { player, message };
            events.single_write(event);
        }
    }
}

/// System for broadcasting chat messages.
/// This system listens to `PlayerChatSystem`s.
#[derive(Default)]
pub struct ChatBroadcastSystem {
    reader: Option<ReaderId<PlayerChatEvent>>,
}

impl<'a> System<'a> for ChatBroadcastSystem {
    type SystemData = (
        Read<'a, EventChannel<PlayerChatEvent>>,
        ReadStorage<'a, NamedComponent>,
        ReadStorage<'a, NetworkComponent>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (events, nameds, networks, entities) = data;

        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            let player_name = &nameds.get(event.player).unwrap().display_name;

            // Todo: could use a more robust chat-component library.
            let message_json = json!({
                "translate": "chat.type.text",
                "with": [
                    {"text": player_name},
                    {"text": event.message},
                ],
            })
            .to_string();

            // Broadcast chat message
            let packet = ChatMessageClientbound {
                json_data: message_json,
                position: 0,
            };

            send_packet_to_all_players(&networks, &entities, packet, None);

            // Log in the console
            info!("<{}> {}", player_name, event.message);
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerChatEvent>>()
                .register_reader(),
        );
    }
}
