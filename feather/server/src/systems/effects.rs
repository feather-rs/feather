use common::Game;
use ecs::{SysResult, SystemExecutor};
use libcraft_effects::effects::Effect;
use quill_common::components_effects::{SpeedEffect, SpeedEffectModifier};
use std::collections::HashMap;

use crate::{ClientId, NetworkId, Server};

pub fn register(_game: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems
        .group::<Server>()
        .add_system(add_start_tick_to_speed_effects)
        .add_system(speed_effect);
}

fn speed_effect(game: &mut Game, server: &mut Server) -> SysResult {
    let mut new_walk_speed = HashMap::new();
    for (entity, (&client_id, speed, &network_id)) in game
        .ecs
        .query::<(&ClientId, &mut SpeedEffect, &NetworkId)>()
        .iter()
    {
        if speed.0.is_empty() {
            continue;
        }

        let end_effect = speed.ended_on_tick(game.tick_count);

        for effect in end_effect.iter() {
            if let Some(client) = server.clients.get(client_id) {
                client.send_remove_entity_effect(network_id, Effect::SpeedEffect.id() as u8);
            }
            let ok = speed.0.remove(effect);
            if ok {
                log::debug!("speed effect was removed with params {:?}", effect)
            }
            new_walk_speed.insert(entity, 0);
        }

        if !end_effect.is_empty() {
            for active_effect in speed.0.iter() {
                if let Some(client) = server.clients.get(client_id) {
                    let duration = active_effect.duration as u64
                        - (game.tick_count - active_effect.start_tick);

                    if duration == 0 {
                        continue;
                    }

                    client.send_entity_effect(
                        network_id,
                        Effect::SpeedEffect.id() as u8,
                        active_effect.amplifier as i8,
                        duration as i32,
                        0x02,
                    );
                }
            }
        }

        if let Some(effect_ref) = speed.active_effect() {
            let modifier = 20 * (effect_ref.amplifier + 1) as i32;
            new_walk_speed.insert(entity, modifier);
        };
    }

    for (entity, modifier) in new_walk_speed {
        if game.ecs.get::<SpeedEffectModifier>(entity).is_err() {
            game.ecs.insert(entity, SpeedEffectModifier(0))?;
        }

        let walk_speed_modifier = game.ecs.get::<SpeedEffectModifier>(entity)?.0;

        if modifier == walk_speed_modifier {
            continue;
        }

        game.ecs
            .insert(entity, SpeedEffectModifier { 0: modifier })?;
    }

    let mut rem_wm = vec![];

    for (entity, &wm) in game.ecs.query::<&SpeedEffectModifier>().iter() {
        if wm.0 == 0 {
            rem_wm.push(entity);
        }
    }

    for entity in rem_wm {
        game.ecs.remove::<SpeedEffectModifier>(entity)?;
    }

    Ok(())
}

/// Set start_tick to all effects in effects_bucket
macro_rules! add_start_tick_to_effects {
    ($fn_name:ident,$type:ident) => {
        fn $fn_name(game: &mut Game, server: &mut Server) -> SysResult {
            for (_entity, (&client_id, effects_bucket, &network_id)) in game
                .ecs
                .query::<(&ClientId, &mut $type, &NetworkId)>()
                .iter()
            {
                if effects_bucket.0.is_empty() {
                    continue;
                }

                let not_started = effects_bucket.not_started();

                for mut effect in not_started {
                    effect.start_tick = game.tick_count;

                    if let Some(client) = server.clients.get(client_id) {
                        client.send_entity_effect(
                            network_id,
                            Effect::$type.id() as u8,
                            effect.amplifier as i8,
                            effect.duration as i32,
                            0x02,
                        );
                    }

                    effects_bucket.0.replace(effect);
                }
            }
            Ok(())
        }
    };
}

add_start_tick_to_effects!(add_start_tick_to_speed_effects, SpeedEffect);
