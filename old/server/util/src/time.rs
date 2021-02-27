//! Handles world time.

use feather_core::network::packets::TimeUpdate;
use feather_server_types::{Game, Network, PlayerPreJoinEvent};
use fecs::World;

/// System for incrementing time each tick.
#[fecs::system]
pub fn increment_time(game: &mut Game) {
    game.time.0 += 1;
}

/// Event handler for sending world time to players.
#[fecs::event_handler]
pub fn on_player_join_send_time(event: &PlayerPreJoinEvent, game: &Game, world: &mut World) {
    let network = world.get::<Network>(event.player);

    // Send time to player.
    let packet = TimeUpdate {
        world_age: game.time.world_age() as i64,
        time_of_day: game.time.time_of_day() as i64,
    };

    network.send(packet);
}
