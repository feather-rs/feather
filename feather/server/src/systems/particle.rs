use crate::Server;
use common::Game;
use quill::components::{EntityDimension, EntityWorld, EntityPosition, EntityParticle};
use vane::{SysResult, SystemExecutor};

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(send_particle_packets);
}

fn send_particle_packets(game: &mut Game, server: &mut Server) -> SysResult {
    let mut entities = Vec::new();

    for (entity, (particle, position, world, dimension)) in game
        .ecs
        .query::<(&EntityParticle, &EntityPosition, &EntityWorld, &EntityDimension)>()
        .iter()
    {
        server.broadcast_nearby_with(*world, &dimension, position.0, |client| {
            client.send_particle(&particle, false, &position);
        });

        entities.push(entity);
    }

    for entity in entities {
        game.ecs.despawn(entity)?;
    }

    Ok(())
}
