use crate::{ClientId, Game, Server};
use common::events::{DamageEvent, DamageType};
use ecs::{EntityRef, SysResult, SystemExecutor};
use quill_common::components::Health;

pub fn register(_game: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems
        .group::<Server>()
        .add_system(damage_handler)
        .add_system(health_regen);
}

fn damage_handler(game: &mut Game, server: &mut Server) -> SysResult {
    // for (entity, (event, health)) in game.ecs.query::<(&DamageEvent, &mut Health)>().iter() {
    // 	match event.damage_type {
    // 		DamageType::FallDamage(_) => {},
    // 	}
    // }

    if game.tick_count % 8 == 0 {
        for (player, (client_id, health)) in game.ecs.query::<(&ClientId, &mut Health)>().iter() {
            if let Some(client) = server.clients.get(*client_id) {
                // health.deal_damage(1);
                // client.update_health(&health);
            }
        }
    }

    Ok(())
}

fn health_regen(game: &mut Game, server: &mut Server) -> SysResult {
    if game.tick_count % 80 == 0 {
        for (player, (client_id, health)) in game.ecs.query::<(&ClientId, &mut Health)>().iter() {
            if let Some(client) = server.clients.get(*client_id) {
                health.regenerate(1);
                client.update_health(&health);
            }
        }
    }

    Ok(())
}
