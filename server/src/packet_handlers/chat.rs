use crate::network::PacketQueue;
use crate::player::chat::PlayerChatEvent;
use feather_core::network::packet::implementation::ChatMessageServerbound;
use tonks::Trigger;

/// Handles animation packets.
#[system]
fn handle_chat(queue: &PacketQueue, trigger: &mut Trigger<PlayerChatEvent>) {
    queue
        .received::<ChatMessageServerbound>()
        .for_each(|(player, packet)| {
            let message = packet.message;

            trigger.trigger(PlayerChatEvent { player, message });
        });
}
