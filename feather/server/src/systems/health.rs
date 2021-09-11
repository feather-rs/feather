use crate::{client::ClientId, Game, Server};
use ecs::{SysResult, SystemExecutor};

use quill_common::components::Health;

pub fn register(_: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(update_health);
}

fn update_health(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, (client_id, health)) in game.ecs.query::<(&ClientId, &mut Health)>().iter() {
        if let Some(client) = server.clients.get(*client_id) {
            client.update_health(&health);
        }
    }

    Ok(())
}
