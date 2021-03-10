use crate::{ClientId, Game, Server};
use common::events::{HealthEventType, UpdateHealthEvent};
use ecs::{Entity, SysResult, SystemExecutor};
use quill_common::components::{Health, Hunger};

pub fn register(_game: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems
        .group::<Server>()
        .add_system(damage_handler)
        .add_system(entity_regeneration);
}

fn damage_handler(game: &mut Game, server: &mut Server) -> SysResult {
    for (entity, (event, health)) in game.ecs.query::<(&UpdateHealthEvent, &mut Health)>().iter() {
        match event.event_type {
            HealthEventType::Regen(hearts) => health.regenerate(hearts),

            HealthEventType::FallDamage(_) => {}
            HealthEventType::Hunger => health.deal_damage(1),
        }

        let client_id = game.ecs.get::<ClientId>(entity)?;
        if let Some(client) = server.clients.get(*client_id) {
            client.update_health(&health);
        }
    }

    // if game.tick_count % 8 == 0 {
    //     for (player, (client_id, health)) in game.ecs.query::<(&ClientId, &mut Health)>().iter() {
    //         if let Some(client) = server.clients.get(*client_id) {
    //             health.deal_damage(1);
    //             client.update_health(&health);
    //         }
    //     }
    // }

    Ok(())
}

fn entity_regeneration(game: &mut Game, server: &mut Server) -> SysResult {
    let mut events: Vec<(Entity, UpdateHealthEvent)> = Vec::new();

    for (player, hunger) in game.ecs.query::<&mut Hunger>().iter() {
        let client_id = game.ecs.get::<ClientId>(player)?;
        if let Some(_) = server.clients.get(*client_id) {
            match hunger.food {
                20 if hunger.saturation > 0 => {
                    if game.tick_count % 10 == 0 {
                        events.push((player, UpdateHealthEvent::new(HealthEventType::Regen(1))));
                    }
                }
                18..=20 => {
                    if game.tick_count % 80 == 0 {
                        events.push((player, UpdateHealthEvent::new(HealthEventType::Regen(1))));
                    }
                }
                0 => {
                    if game.tick_count % 80 == 0 {
                        events.push((player, UpdateHealthEvent::new(HealthEventType::Hunger)));
                    }
                }
                _ => {}
            }
        }
    }

    for (entity, event) in events {
        game.ecs.insert_entity_event(entity, event)?;
    }

    Ok(())
}
