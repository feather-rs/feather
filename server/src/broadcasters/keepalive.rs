use crate::game::Game;
use crate::TPS;
use feather_core::network::packet::implementation::KeepAliveClientbound;
use fecs::World;

/// Broadcasts keepalives every second.
#[fecs::system]
pub fn broadcast_keepalive(game: &Game, world: &mut World) {
    if game.tick_count % TPS == 0 {
        let packet = KeepAliveClientbound {
            keep_alive_id: game.tick_count,
        };
        game.broadcast_global(world, packet, None);
    }
}
