use feather_core::network::packets::ChangeGameState;
use feather_core::util::Gamemode;
use feather_server_types::{GamemodeUpdateEvent, Network};
use fecs::World;

/// Sends a Change Game State packet to update player's gamemode.
#[fecs::event_handler]
pub fn on_gamemode_update_send(event: &GamemodeUpdateEvent, world: &mut World) {
    let packet = ChangeGameState {
        reason: 3, // change gamemode
        value: match event.new {
            Gamemode::Survival => 0.0,
            Gamemode::Creative => 1.0,
            Gamemode::Adventure => 2.0,
            Gamemode::Spectator => 3.0,
        },
    };

    if let Some(network) = world.try_get::<Network>(event.player) {
        network.send(packet);
    }
}
