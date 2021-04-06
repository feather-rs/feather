use crate::{ClientId, Game, Server};
use ecs::{SysResult, SystemExecutor};
use quill_common::{
    components::{Health, Hunger},
    events::{
        entity_damage_event_type, entity_regen_event_type, entity_special_event_type,
        EntityHealthEvent,
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
        .query::<(
            &entity_special_event_type::EntityResurrectionEvent,
            &mut Health,
        )>()
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
        .query::<(&entity_special_event_type::EntitySuicideEvent, &mut Health)>()
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

    for (entity, _) in game.ecs.query::<&entity_regen_event_type::Eating>().iter() {
        events.push((entity, EntityHealthEvent::Regen(1)));
    }

    for (entity, event) in events {
        game.ecs.insert_entity_event(entity, event)?;
    }

    Ok(())
}

fn player_hunger(game: &mut Game, _server: &mut Server) -> SysResult {
    let mut events = Vec::new();

    for (entity, _) in game
        .ecs
        .query::<&entity_damage_event_type::Starvation>()
        .iter()
    {
        events.push((entity, EntityHealthEvent::Damage(1)));
    }

    for (entity, event) in events {
        game.ecs.insert_entity_event(entity, event)?;
    }

    Ok(())
}
