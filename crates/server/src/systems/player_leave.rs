use common::Game;
use ecs::{SysResult, SystemExecutor};

use crate::{ClientId, Server};

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems
        .group::<Server>()
        .add_system(remove_disconnected_clients);
}

fn remove_disconnected_clients(game: &mut Game, server: &mut Server) -> SysResult {
    let mut entities_to_remove = Vec::new();
    for (player, &client_id) in game.ecs.query::<&ClientId>().iter() {
        let client = server.clients.get(client_id).unwrap();
        if client.is_disconnected() {
            server.remove_client(client_id);
            entities_to_remove.push(player);
        }
    }

    for player in entities_to_remove {
        game.remove_entity(player)?;
    }

    Ok(())
}
