use crate::{ClientId, NetworkId, Position, Server};
use common::Game;
use ecs::{Entity, SysResult};
use protocol::packets::client::ClientStatus;
use uuid::Uuid;

pub fn handle_client_status(
    game: &mut Game,
    server: &mut Server,
    player: Entity,
    packet: ClientStatus,
) -> SysResult {
    match packet {
        ClientStatus::PerformRespawn => {
            let client_id = game.ecs.get::<ClientId>(player)?;
            let client = server.clients.get(*client_id).unwrap();

            client.respawn_player(server.options.default_gamemode);

            let player_entity = game.ecs.entity(player)?;
            game.reset_player(player_entity)?;

            let network_id = game.ecs.get::<NetworkId>(player)?;
            let position = game.ecs.get::<Position>(player)?;
            let uuid = game.ecs.get::<Uuid>(player)?;

            // Recreate the player for all clients.
            server.broadcast_nearby_with(*position, |client| {
                if let Some(client_network_id) = client.network_id() {
                    if client_network_id != *network_id {
                        client.send_player(*network_id, *uuid, *position);
                    }
                }
            });
        }
        ClientStatus::RequestStats => {}
    }

    Ok(())
}
