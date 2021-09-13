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
        // Only calculate fall damage for players that are Survival/Adventure mode.
        if let Ok(gamemode) = game.ecs.get::<Gamemode>(entity) {
            if *gamemode != Gamemode::Survival && *gamemode != Gamemode::Adventure {
                continue;
            }
        }

        match on_ground.0 {
            false => {
                let new_distance = prev_position.0.y - position.y;

                match new_distance < 0.0 {
                    // Reset fall distance when the player moves upwards.
                    true => fall_distance.0 = 0.0,
                    false => fall_distance.0 += new_distance,
                }
            }
            // Apply fall damage.
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
        }
    }

    Ok(())
}
