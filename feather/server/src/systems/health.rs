use crate::{ClientId, Game, Server};
use common::events::{HealthEventType, UpdateHealthEvent};
use ecs::{SysResult, SystemExecutor};
use quill_common::components::{Health, Hunger};

pub fn register(_game: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(health_events_handler);
}

// TODO: Implement for entities besides players.
fn health_events_handler(game: &mut Game, server: &mut Server) -> SysResult {
    for (entity, (event, health)) in game.ecs.query::<(&UpdateHealthEvent, &mut Health)>().iter() {
        match event.event_type {
            HealthEventType::Heal(half_hearts) => health.heal(half_hearts),
            HealthEventType::Harm(half_hearts) => health.harm(half_hearts),
        }

        if let Ok(client_id) = game.ecs.get::<ClientId>(entity) {
            if let Some(client) = server.clients.get(*client_id) {
                let hunger = game.ecs.entity(entity)?.get::<Hunger>()?;
                client.update_status(&health, &hunger);
            }
        }
    }

    Ok(())
}
