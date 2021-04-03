use crate::{Game, Server};
use ecs::{SysResult, SystemExecutor};
use quill_common::{components::Hunger, events::EntityHealthEvent};

pub fn register(_game: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(hunger_health_events);
}

fn hunger_health_events(game: &mut Game, _server: &mut Server) -> SysResult {
    let mut events = Vec::new();

    for (player, hunger) in game.ecs.query::<&mut Hunger>().iter() {
        match hunger.food {
            20 if hunger.saturation > 0 => {
                if game.tick_count % 10 == 0 {
                    events.push((player, EntityHealthEvent::Regen(1)));
                }
            }
            18..=20 => {
                if game.tick_count % 80 == 0 {
                    events.push((player, EntityHealthEvent::Regen(1)));
                }
            }
            0 => {
                if game.tick_count % 80 == 0 {
                    events.push((player, EntityHealthEvent::Damage(1)));
                }
            }
            _ => {}
        }
    }

    for (entity, event) in events {
        game.ecs.insert_entity_event(entity, event)?;
    }

    Ok(())
}
