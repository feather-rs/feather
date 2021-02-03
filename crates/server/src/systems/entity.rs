//! Sends entity-related packets to clients.
//! Spawn packets, position updates, equipment, animations, etc.

use base::Position;
use common::Game;
use ecs::{SysResult, SystemExecutor};

use crate::{NetworkId, Server};

mod spawn_packet;

pub fn register(game: &mut Game, systems: &mut SystemExecutor<Game>) {
    spawn_packet::register(game, systems);
    systems.group::<Server>().add_system(send_entity_movement);
}

/// Sends entity movement packets.
///
/// Currently entities move every tick, which may not be the
/// case. Optimization possible.
fn send_entity_movement(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, (&position, &network_id)) in game.ecs.query::<(&Position, &NetworkId)>().iter() {
        server.broadcast_nearby_with(position, |client| {
            client.update_entity_position(network_id, position)
        });
    }
    Ok(())
}
