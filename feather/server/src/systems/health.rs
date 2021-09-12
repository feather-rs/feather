use crate::{client::ClientId, Game, Server};
use ecs::{SysResult, SystemExecutor};

use quill_common::components::Health;

pub fn register(_: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(update_health);
}

fn update_health(game: &mut Game, server: &mut Server) -> SysResult {
    let mut entities = Vec::new();

    for (entity, health) in game.ecs.query::<&Health>().iter() {
        match game.ecs.get::<ClientId>(entity) {
            // Entity is player
            Ok(client_id) => {
                if let Some(client) = server.clients.get(*client_id) {
                    client.update_health(health);
                }
            }
            Err(_) => {
                if health.health == 0 {
                    entities.push(entity);
                }
            }
        }
    }

    // Destroy all entities with 0 health (that are not players).
    for entity in entities {
        game.remove_entity(entity)?;
    }

    Ok(())
}
