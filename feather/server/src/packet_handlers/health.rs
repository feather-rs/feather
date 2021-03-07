use protocol::packets::{
	client::ClientStatus,
	server::Respawn,
};
use base::Gamemode;
use ecs::{Entity, SysResult};
use common::Game;

use crate::{ClientId, Server};

pub fn handle_client_status(
	game: &mut Game,
    _server: &mut Server,
    player_id: Entity,
    packet: ClientStatus,
) -> SysResult {
	match packet {
		PerformRespawn => {
			let client_id = game.ecs.get::<ClientId>(player_id).unwrap();
            let client = _server.clients.get(*client_id).unwrap();
			
			client.respawn_player(Gamemode::Survival);

			let player = game.ecs.entity(player_id)?;
			game.reset_player(player);
		},
		RequestStats => {},
	}

	Ok(())
}