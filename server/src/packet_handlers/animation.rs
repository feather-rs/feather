use crate::network::PacketQueue;
use crate::player::PlayerAnimationEvent;
use feather_core::network::packet::implementation::AnimationServerbound;
use feather_core::{ClientboundAnimation, Hand};
use tonks::Trigger;

/// Handles animation packets.
#[system]
fn handle_animation(queue: &PacketQueue, trigger: &mut Trigger<PlayerAnimationEvent>) {
    queue
        .received::<AnimationServerbound>()
        .for_each(|(player, packet)| {
            let animation = match packet.hand {
                Hand::Main => ClientboundAnimation::SwingMainArm,
                Hand::Off => ClientboundAnimation::SwingOffhand,
            };

            trigger.trigger(PlayerAnimationEvent { player, animation });
        });
}
