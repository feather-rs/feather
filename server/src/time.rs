//! Handles world time.

use crate::joinhandler::PlayerJoinEvent;
use crate::network::{send_packet_to_player, NetworkComponent};
use crate::systems::{TIME_INCREMENT, TIME_SEND};
use feather_core::level::LevelData;
use feather_core::packet::TimeUpdate;
use shrev::EventChannel;
use specs::{DispatcherBuilder, Read, ReadStorage, ReaderId, System, World, Write};

/// The current time of the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref, DerefMut, Default)]
pub struct Time(pub u64);

impl Time {
    /// Returns the time of day. This is calculated
    /// as `time.0 % 24_000`.
    pub fn time_of_day(self) -> u64 {
        self.0 % 24_000
    }

    /// Returns the age of the world in ticks. Equivalent to `time.0`.
    pub fn world_age(self) -> u64 {
        self.0
    }
}

/// Initializes systems for this module.
pub fn init_logic(dispatcher: &mut DispatcherBuilder) {
    dispatcher.add(TimeIncrementSystem, TIME_INCREMENT, &[]);
    dispatcher.add(TimeSendSystem::default(), TIME_SEND, &[]);
}

/// Initializes the time for the world, given the
/// level file.
pub fn init_time(world: &mut World, level: &LevelData) {
    world.insert(Time(level.time as u64))
}

/// System for incrementing time each tick.
pub struct TimeIncrementSystem;

impl<'a> System<'a> for TimeIncrementSystem {
    type SystemData = Write<'a, Time>;

    fn run(&mut self, mut time: Self::SystemData) {
        time.0 += 1;
    }
}

/// System for sending world time to players
/// upon joining.
///
/// This system listens to `PlayerJoinEvent`s.
#[derive(Default)]
pub struct TimeSendSystem {
    reader: Option<ReaderId<PlayerJoinEvent>>,
}

impl<'a> System<'a> for TimeSendSystem {
    type SystemData = (
        ReadStorage<'a, NetworkComponent>,
        Read<'a, EventChannel<PlayerJoinEvent>>,
        Read<'a, Time>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (networks, join_events, time) = data;

        for event in join_events.read(self.reader.as_mut().unwrap()) {
            let network = networks.get(event.player).unwrap();

            // Send time to player.
            let packet = TimeUpdate {
                world_age: time.world_age() as i64,
                time_of_day: time.time_of_day() as i64,
            };

            send_packet_to_player(network, packet);
        }
    }

    setup_impl!(reader);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testframework as t;
    use feather_core::{network::cast_packet, PacketType};
    use shrev::EventChannel;
    use specs::WorldExt;

    #[test]
    fn test_time_init() {
        let mut level = LevelData::default();
        let time = 29456;
        level.time = time as i64;

        let mut world = World::new();
        init_time(&mut world, &level);

        assert_eq!(*world.fetch::<Time>(), Time(time));
    }

    #[test]
    fn test_time_increment_system() {
        let (mut w, mut d) = t::builder().with(TimeIncrementSystem, "").build();

        d.dispatch(&w);
        w.maintain();

        assert_eq!(*w.fetch::<Time>(), Time(1));
    }

    #[test]
    fn test_time_send_system() {
        let (mut w, mut d) = t::builder().with(TimeSendSystem::default(), "").build();

        let player = t::add_player(&mut w);

        let event = PlayerJoinEvent {
            player: player.entity,
        };
        w.fetch_mut::<EventChannel<_>>().single_write(event);

        d.dispatch(&w);
        w.maintain();

        let packet = t::assert_packet_received(&player, PacketType::TimeUpdate);
        let packet = cast_packet::<TimeUpdate>(&*packet);

        assert_eq!(packet.time_of_day, 0);
        assert_eq!(packet.world_age, 0);
    }
}
