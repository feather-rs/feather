use crate::{client::ClientId, Game, NetworkId, Server};
use base::Position;
use ecs::{SysResult, SystemExecutor};
use quill_common::components::Health;

pub fn register(_: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(update_health);
}

fn update_health(game: &mut Game, server: &mut Server) -> SysResult {
    let mut drop_entities = Vec::new();

    for (entity, (health, &position, &network_id)) in
        game.ecs.query::<(&Health, &Position, &NetworkId)>().iter()
    {
        match game.ecs.get::<ClientId>(entity) {
            // Entity is player
            Ok(client_id) => {
                if let Some(client) = server.clients.get(*client_id) {
                    client.update_health(health);
                }

                if health.health == 0 {
                    // Unload/hide the player from all other clients.
                    server.broadcast_nearby_with(position, |client| {
                        if client.is_entity_loaded(network_id) {
                            client.unload_entity(network_id);
                        }
                    });
                }
            }
            Err(_) => {
                if health.health == 0 {
                    drop_entities.push(entity);
                }
            }
        }
    }

    // Destroy all entities with 0 health (that are not players).
    for entity in drop_entities {
        game.remove_entity(entity)?;
    }

    Ok(())
}
