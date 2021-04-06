use crate::{Game, Server};
use ecs::{SysResult, SystemExecutor};
use quill_common::{
    components::Hunger,
    events::{EntityDamageEventType, EntityRegenEventType},
};

pub fn register(_game: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(hunger_events);
}

fn hunger_events(game: &mut Game, _server: &mut Server) -> SysResult {
    let mut eating_events = Vec::new();
    let mut starving_events = Vec::new();

    for (player, hunger) in game.ecs.query::<&mut Hunger>().iter() {
        match hunger.food {
            20 if hunger.saturation > 0 => {
                if game.tick_count % 10 == 0 {
                    eating_events.push((player, EntityRegenEventType::Eating));
                }
            }
            18..=20 => {
                if game.tick_count % 80 == 0 {
                    eating_events.push((player, EntityRegenEventType::Eating));
                }
            }
            0 => {
                if game.tick_count % 80 == 0 {
                    starving_events.push((player, EntityDamageEventType::Starvation));
                }
            }
            _ => {}
        }
    }

    for (entity, event) in eating_events {
        game.ecs.insert_entity_event(entity, event)?;
    }

    for (entity, event) in starving_events {
        game.ecs.insert_entity_event(entity, event)?;
    }

    Ok(())
}
