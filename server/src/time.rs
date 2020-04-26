//! Handles world time.

use crate::game::Game;
use crate::network::Network;
use feather_core::packet::TimeUpdate;
use fecs::{Entity, World};
use std::ops::{Deref, DerefMut};

/// System for incrementing time each tick.
#[fecs::system]
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
