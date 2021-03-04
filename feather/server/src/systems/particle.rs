use crate::Server;
use base::{Particle, Position};
use common::Game;
use ecs::{SysResult, SystemExecutor};

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(send_particle_packets);
}

fn send_particle_packets(game: &mut Game, server: &mut Server) -> SysResult {
    let mut entities = Vec::new();

    for (entity, (&particle, &position)) in game.ecs.query::<(&Particle, &Position)>().iter() {
        server.broadcast_nearby_with(position, |client| {
            client.send_particle(&particle, &position);
        });

        entities.push(entity);
    }

    for entity in entities {
        game.ecs.despawn(entity)?;
    }

    Ok(())
}
