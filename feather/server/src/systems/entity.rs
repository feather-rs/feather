//! Sends entity-related packets to clients.
//! Spawn packets, position updates, equipment, animations, etc.

use base::Position;
use common::Game;
use ecs::{SysResult, SystemExecutor};
use quill_common::components::OnGround;

use crate::{entities::PreviousPosition, NetworkId, Server};

mod spawn_packet;

pub fn register(game: &mut Game, systems: &mut SystemExecutor<Game>) {
    spawn_packet::register(game, systems);
    systems.group::<Server>().add_system(send_entity_movement);
}

/// Sends entity movement packets.
fn send_entity_movement(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, (&position, prev_position, &on_ground, &network_id)) in game
        .ecs
        .query::<(&Position, &mut PreviousPosition, &OnGround, &NetworkId)>()
        .iter()
    {
        if position != prev_position.0 {
            server.broadcast_nearby_with(position, |client| {
                client.update_entity_position(network_id, position, on_ground);
            });
            prev_position.0 = position;
        }
    }
    Ok(())
}
