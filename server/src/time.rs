//! Handles world time.

use crate::network::Network;
use crate::player::PlayerJoinEvent;
use feather_core::packet::TimeUpdate;
use legion::query::Read;
use tonks::{PreparedWorld, Query};

/// The current time of the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref, DerefMut, Default, Resource)]
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

/// System for incrementing time each tick.
#[system]
pub fn time_increment(time: &mut Time) {
    time.0 += 1;
}

/// Event handler for sending world time to players.
#[event_handler]
pub fn time_send(
    time: &Time,
    event: &PlayerJoinEvent,
    _query: &mut Query<Read<Network>>,
    world: &mut PreparedWorld,
) {
    let network = world.get_component::<Network>(event.player).unwrap();

    // Send time to player.
    let packet = TimeUpdate {
        world_age: time.world_age() as i64,
        time_of_day: time.time_of_day() as i64,
    };

    network.send(packet);
}
