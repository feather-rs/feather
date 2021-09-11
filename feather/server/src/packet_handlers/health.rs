use crate::{ClientId, Server};
use base::Gamemode;
use common::Game;
use ecs::{Entity, SysResult};
use protocol::packets::client::ClientStatus;

pub fn handle_client_status(
    game: &mut Game,
    server: &mut Server,
    player_id: Entity,
    packet: ClientStatus,
) -> SysResult {
    match packet {
        ClientStatus::PerformRespawn => {
            let client_id = game.ecs.get::<ClientId>(player_id).unwrap();
            let client = server.clients.get(*client_id).unwrap();

            client.respawn_player(Gamemode::Survival);

            let player = game.ecs.entity(player_id)?;
            game.reset_player(player);
        }
        ClientStatus::RequestStats => {}
    }

    Ok(())
}
