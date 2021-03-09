use crate::{ClientId, Game, Server};
use common::events::{DamageEvent, DamageType};
use ecs::{Entity, SysResult, SystemExecutor};
use quill_common::components::{Health, Hunger};

pub fn register(_game: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems
        .group::<Server>()
        .add_system(damage_handler)
        .add_system(player_health_manager);
}

fn damage_handler(game: &mut Game, server: &mut Server) -> SysResult {
    for (entity, (event, health)) in game.ecs.query::<(&DamageEvent, &mut Health)>().iter() {
        let player_client = {
            let client_id = game.ecs.get::<ClientId>(entity)?;
            server.clients.get(*client_id)
        };

        match event.damage_type {
            DamageType::FallDamage(_) => {}
            DamageType::Hunger => {
                if let Some(client) = player_client {
                    health.deal_damage(1);
                    client.update_health(&health);
                }
            }
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

fn player_health_manager(game: &mut Game, server: &mut Server) -> SysResult {
    let mut events: Vec<(Entity, DamageEvent)> = Vec::new();

    for (player, (health, hunger)) in game.ecs.query::<(&mut Health, &mut Hunger)>().iter() {
        let client_id = game.ecs.get::<ClientId>(player)?;
        if let Some(client) = server.clients.get(*client_id) {
            match hunger.food {
                20 if hunger.saturation > 0 => {
                    if game.tick_count % 10 == 0 {
                        health.regenerate(1);
                    }
                }
                18..=20 => {
                    if game.tick_count % 80 == 0 {
                        health.regenerate(1);
                    }
                }
                0 => {
                    if game.tick_count % 80 == 0 {
                        events.push((player, DamageEvent::new(DamageType::Hunger)));
                    }
                }
                _ => {}
            }

            client.update_health(&health);
        }
    }

    for (entity, event) in events {
        game.ecs.insert_entity_event(entity, event)?;
    }

    Ok(())
}
