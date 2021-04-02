use base::Gamemode;
use common::Game;
use ecs::{Entity, SysResult};
use protocol::packets::client::ClientStatus;

use crate::{ClientId, Server};

pub fn handle_client_status(
    game: &mut Game,
    server: &mut Server,
    player_id: Entity,
    packet: ClientStatus,
) -> SysResult {
    match packet {
        ClientStatus::PerformRespawn => {
            let client_id = game.ecs.get::<ClientId>(player_id)?;
            if let Some(client) = server.clients.get(*client_id) {
                let gamemode = game.ecs.get::<Gamemode>(player_id)?;
                client.send_respawn(*gamemode, false);

                // Temporary, will be replaced with an event.
                let player = game.ecs.entity(player_id)?;
                game.reset_player(player);
            }
        }

        ClientStatus::RequestStats => {}
    }

    Ok(())
}
