use feather_core::network::packets::KeepAliveClientbound;
use feather_server_types::{Game, TPS};
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
