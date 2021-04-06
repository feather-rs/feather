use crate::{ClientId, Game, Server};
use ecs::{SysResult, SystemExecutor};
use quill_common::{
    components::{Health, Hunger},
    events::{
        EntityDamageEventType, EntityHealthEvent, EntityRegenEventType, EntityResurrectionEvent,
        EntitySuicideEvent,
    },
};

pub fn register(_game: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems
        .group::<Server>()
        .add_system(entity_resurrection)
        .add_system(entity_suicide)
        .add_system(player_eating_regen)
        .add_system(player_hunger)
        .add_system(health_events_handler);
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

fn entity_resurrection(game: &mut Game, _server: &mut Server) -> SysResult {
    let mut events = Vec::new();

    for (entity, (_, health)) in game
        .ecs
        .query::<(&EntityResurrectionEvent, &mut Health)>()
        .iter()
    {
        events.push((entity, EntityHealthEvent::Regen(health.max_health)));
    }

    for (entity, event) in events {
        game.ecs.insert_entity_event(entity, event)?;
    }

    Ok(())
}

fn entity_suicide(game: &mut Game, _server: &mut Server) -> SysResult {
    let mut events = Vec::new();

    for (entity, (_, health)) in game
        .ecs
        .query::<(&EntitySuicideEvent, &mut Health)>()
        .iter()
    {
        events.push((entity, EntityHealthEvent::Damage(health.max_health)));
    }

    for (entity, event) in events {
        game.ecs.insert_entity_event(entity, event)?;
    }

    Ok(())
}

fn player_eating_regen(game: &mut Game, _server: &mut Server) -> SysResult {
    let mut events = Vec::new();

    for (entity, event) in game.ecs.query::<&EntityRegenEventType>().iter() {
        match event {
            EntityRegenEventType::Eating => events.push((entity, EntityHealthEvent::Regen(1))),
            _ => {}
        }
    }

    for (entity, event) in events {
        game.ecs.insert_entity_event(entity, event)?;
    }

    Ok(())
}

fn player_hunger(game: &mut Game, _server: &mut Server) -> SysResult {
    let mut events = Vec::new();

    for (entity, event) in game.ecs.query::<&EntityDamageEventType>().iter() {
        match event {
            EntityDamageEventType::Starvation => {
                events.push((entity, EntityHealthEvent::Damage(1)))
            }
            _ => {}
        }
    }

    for (entity, event) in events {
        game.ecs.insert_entity_event(entity, event)?;
    }

    Ok(())
}
