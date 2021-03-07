use crate::{client::ClientId, Server};
use ecs::{Entity, SysResult, SystemExecutor};
use crate::{Game};

use quill_common::components::Health;

pub fn register(game: &mut Game, systems: &mut SystemExecutor<Game>) {
	systems.group::<Server>()
		.add_system(lose_health);
}

fn lose_health(game: &mut Game, server: &mut Server) -> SysResult {
	if game.tick_count % 8 == 0 {
		for (player, (client_id, health)) in game.ecs.query::<(&ClientId, &mut Health)>().iter() {
			if let Some(client) = server.clients.get(*client_id) {
				health.deal_damage(1);
				client.update_health(&health);
			}
		}
	}
	
	Ok(())
}