use crate::{
    entities::{FallDistance, PreviousPosition},
    Game, Server,
};
use base::{Gamemode, Position};
use ecs::{SysResult, SystemExecutor};
use quill_common::components::{Health, OnGround};

pub fn register(_: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(calculate_falldamage);
}

fn calculate_falldamage(game: &mut Game, _: &mut Server) -> SysResult {
    for (entity, (position, prev_position, on_ground, fall_distance, health)) in game
        .ecs
        .query::<(
            &Position,
            &PreviousPosition,
            &OnGround,
            &mut FallDistance,
            &mut Health,
        )>()
        .iter()
    {
        match game.ecs.get::<Gamemode>(entity) {
            // Entity is player
            Ok(gamemode) => match *gamemode {
                // Only Survival/Adventure mode affected by fall damage.
                // Fall damage for players is based on velocity (WIP).
                Gamemode::Survival | Gamemode::Adventure => match on_ground.0 {
                    false => {
                        let new_distance = prev_position.0.y - position.y;
                        fall_distance.0 += new_distance;
                    }

                    true => {
                        let damage = (fall_distance.0.ceil() as u32).saturating_sub(3);
                        health.deal_damage(damage);

                        #[cfg(debug_assertions)]
                        if fall_distance.0 > 0.0 {
                            log::debug!("Entity fell {:?} ({:?} damage)", fall_distance, damage);
                        }

                        // Reset fall distance.
                        fall_distance.0 = 0.0;
                    }
                },
                _ => (),
            },
            // Other entities have fall damage based on distance. Moving upwards (+Y) does
            // not reduce the accumulated fall damage.
            Err(_) => match on_ground.0 {
                false => fall_distance.0 += (prev_position.0.y - position.y).max(0.0),
                true => {
                    let damage = (fall_distance.0.ceil() as u32).saturating_sub(3);
                    health.deal_damage(damage);

                    #[cfg(debug_assertions)]
                    if fall_distance.0 > 0.0 {
                        log::debug!("Entity fell {:?} ({:?} damage)", fall_distance, damage);
                    }

                    // Reset fall distance.
                    fall_distance.0 = 0.0;
                }
            },
        }
    }

    Ok(())
}
