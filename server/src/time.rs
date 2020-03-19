//! Handles world time.

use crate::game::Game;
use crate::network::Network;
use feather_core::packet::TimeUpdate;
use fecs::{Entity, World};
use std::ops::{Deref, DerefMut};

/// The current time of the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Time(pub u64);

impl Deref for Time {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Time {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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
pub fn increment_time(game: &mut Game) {
    game.time.0 += 1;
}

/// Event handler for sending world time to players.
pub fn on_player_join_send_time(game: &Game, world: &World, player: Entity) {
    let network = world.get::<Network>(player);

    // Send time to player.
    let packet = TimeUpdate {
        world_age: game.time.world_age() as i64,
        time_of_day: game.time.time_of_day() as i64,
    };

    network.send(packet);
}
