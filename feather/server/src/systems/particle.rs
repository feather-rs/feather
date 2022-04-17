use crate::Server;
use common::Game;
use quill::components::{EntityParticle, EntityPosition, EntityWorld};
use vane::{SysResult, SystemExecutor};

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(send_particle_packets);
}

fn send_particle_packets(game: &mut Game, server: &mut Server) -> SysResult {
    let mut entities = Vec::new();

    for (entity, (particle, position, world)) in game
        .ecs
        .query::<(&EntityParticle, &EntityPosition, &EntityWorld)>()
        .iter()
    {
        server.broadcast_nearby_with(world.0, position.0, |client| {
            client.send_particle(&particle, false, &position);
        });

        entities.push(entity);
    }

    for entity in entities {
        game.ecs.despawn(entity)?;
    }

    Ok(())
}
