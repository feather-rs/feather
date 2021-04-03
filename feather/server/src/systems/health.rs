use crate::{ClientId, Game, Server};
use ecs::{SysResult, SystemExecutor};
use quill_common::{
    components::{Health, Hunger},
    events::EntityHealthEvent,
};

pub fn register(_game: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(health_events_handler);
}

// TODO: Implement for entities besides players.
fn health_events_handler(game: &mut Game, server: &mut Server) -> SysResult {
    for (entity, (event, health)) in game.ecs.query::<(&EntityHealthEvent, &mut Health)>().iter() {
        match event {
            EntityHealthEvent::Regen(amount) => health.heal(*amount),
            EntityHealthEvent::Damage(amount) => health.harm(*amount),
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
