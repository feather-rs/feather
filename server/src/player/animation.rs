use crate::network::PacketQueue;
use crate::util::Util;
use feather_core::network::cast_packet;
use feather_core::network::packet::implementation::{AnimationClientbound, AnimationServerbound};
use feather_core::network::packet::PacketType;
use feather_core::{ClientboundAnimation, Hand};
use shrev::EventChannel;
use specs::SystemData;
use specs::{Entity, Read, ReaderId, System, World, Write};

/// Event which is triggered when a player causes
/// an animation.
#[derive(Debug, Clone)]
pub struct PlayerAnimationEvent {
    pub player: Entity,
    pub animation: ClientboundAnimation,
}

/// System for handling Animation Serverbound packets
/// and then triggering a `PlayerAnimationEvent`.
pub struct PlayerAnimationSystem;

impl<'a> System<'a> for PlayerAnimationSystem {
    type SystemData = (
        Write<'a, EventChannel<PlayerAnimationEvent>>,
        Read<'a, PacketQueue>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut events, packet_queue) = data;

        // Handle Animation Serverbound packets.
        let packets = packet_queue.for_packet(PacketType::AnimationServerbound);

        for (player, packet) in packets {
            let packet = cast_packet::<AnimationServerbound>(&*packet);

            let animation = match packet.hand {
                Hand::Main => ClientboundAnimation::SwingMainArm,
                Hand::Off => ClientboundAnimation::SwingOffhand,
            };

            let event = PlayerAnimationEvent { player, animation };
            events.single_write(event);
        }
    }
}

/// System for broadcasting when a player causes an animation.
/// This system listens to `PlayerAnimationEvent`s.
#[derive(Default)]
pub struct AnimationBroadcastSystem {
    reader: Option<ReaderId<PlayerAnimationEvent>>,
}

impl<'a> System<'a> for AnimationBroadcastSystem {
    type SystemData = (Read<'a, EventChannel<PlayerAnimationEvent>>, Read<'a, Util>);

    fn run(&mut self, data: Self::SystemData) {
        let (events, util) = data;

        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            // Broadcast animation
            let packet = AnimationClientbound::new(event.player.id() as i32, event.animation);

            util.broadcast_entity_update(event.player, packet, Some(event.player))
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader = Some(
            world
                .fetch_mut::<EventChannel<PlayerAnimationEvent>>()
                .register_reader(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testframework as t;
    use feather_core::network::packet::implementation::{
        AnimationClientbound, AnimationServerbound,
    };
    use specs::WorldExt;

    #[test]
    fn test_animation_system() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);

        let packet = AnimationServerbound::new(Hand::Main);
        t::receive_packet(&player, &w, packet);

        let mut event_reader = t::reader::<PlayerAnimationEvent>(&w);

        d.dispatch(&w);
        w.maintain();

        let channel = w.fetch::<EventChannel<PlayerAnimationEvent>>();

        let events = channel.read(&mut event_reader).collect::<Vec<_>>();
        assert_eq!(events.len(), 1);
        let first = events.first().unwrap();

        assert_eq!(first.player, player.entity);
        assert_eq!(first.animation, ClientboundAnimation::SwingMainArm);
    }

    #[test]
    fn test_animation_broadcast_system() {
        let (mut w, mut d) = t::init_world();

        let player = t::add_player(&mut w);
        let player2 = t::add_player(&mut w);

        let event = PlayerAnimationEvent {
            player: player.entity,
            animation: ClientboundAnimation::SwingMainArm,
        };

        t::trigger_event(&w, event.clone());

        d.dispatch(&w);
        w.maintain();

        // Make sure animation wasn't sent to the player itself
        t::assert_packet_not_received(&player, PacketType::AnimationClientbound);

        let packet = t::assert_packet_received(&player2, PacketType::AnimationClientbound);
        let packet = cast_packet::<AnimationClientbound>(&*packet);

        assert_eq!(packet.entity_id, event.player.id() as i32);
        assert_eq!(packet.animation, event.animation);
    }
}
