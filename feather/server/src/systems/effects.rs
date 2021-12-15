use base::{Particle, ParticleKind, TPS};
use common::Game;
use ecs::{Entity, SysResult, SystemExecutor};
use libcraft_core::Position;
use libcraft_effects::effects::Effect;
use quill_common::components_effects::{EffectApplication, SpeedEffect, WalkEffectModifier};
use quill_common::entity_init::EntityInit;
use std::collections::HashMap;

use crate::{Client, ClientId, NetworkId, Server};

pub fn register(_game: &mut Game, systems: &mut SystemExecutor<Game>) {
    systems
        .group::<Server>()
        .add_system(add_start_tick_to_speed_effects)
        .add_system(speed_effect)
        .add_system(walk_effect_modifier_cleaner)
        .add_system(effect_remover);
}

fn speed_effect(game: &mut Game, _server: &mut Server) -> SysResult {
    let mut new_walk_speed = HashMap::new();
    for (entity, speed) in game.ecs.query::<&mut SpeedEffect>().iter() {
        if speed.0.is_empty() {
            continue;
        }

        if let Some(effect_ref) = speed.active_effect() {
            let modifier = 20 * (effect_ref.amplifier + 1) as i32;
            new_walk_speed.insert(entity, modifier);
        };
    }

    for (entity, modifier) in new_walk_speed {
        change_modifier(game, entity, modifier)?;
    }

    Ok(())
}

fn effect_remover(game: &mut Game, server: &mut Server) -> SysResult {
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

            if speed.0.remove(effect) {
                log::debug!("speed effect was removed with params {:?}", effect)
            }
            new_walk_speed.insert(entity, 0);
        }

        if !end_effect.is_empty() {
            if let Some(active_effect) = speed.active_effect() {
                if let Some(client) = server.clients.get(client_id) {
                    let duration = active_effect.duration as u64
                        - (game.tick_count - active_effect.start_tick);

                    send_effect_to_client(
                        Effect::SpeedEffect.id() as u8,
                        network_id,
                        active_effect,
                        client,
                        duration,
                    );
                }
            }
        }
    }

    for (entity, modifier) in new_walk_speed {
        change_modifier(game, entity, modifier)?;
    }

    Ok(())
}

fn walk_effect_modifier_cleaner(game: &mut Game, _server: &mut Server) -> SysResult {
    let mut rem_comp = vec![];

    for (entity, wm) in game.ecs.query::<&mut WalkEffectModifier>().iter() {
        let mut rem_wm = vec![];

        for (effect, modifier) in wm.0.iter() {
            if *modifier == 0 {
                rem_wm.push(*effect);
            }
        }

        for effect in rem_wm {
            wm.0.remove(&effect);
        }

        if wm.0.is_empty() {
            rem_comp.push(entity);
        }
    }

    for entity in rem_comp {
        game.ecs.remove::<WalkEffectModifier>(entity)?;
    }

    Ok(())
}

fn change_modifier(game: &mut Game, entity: Entity, new_modifier: i32) -> SysResult {
    if game.ecs.get::<WalkEffectModifier>(entity).is_err() {
        game.ecs.insert(entity, WalkEffectModifier::new())?;
    }

    let mut walk_speed_modifier = game.ecs.get_mut::<WalkEffectModifier>(entity)?;
    if walk_speed_modifier.0.contains_key(&Effect::SpeedEffect)
        && new_modifier
            != *walk_speed_modifier
                .0
                .get(&Effect::SpeedEffect)
                .unwrap_or(&0)
    {
        walk_speed_modifier
            .0
            .insert(Effect::SpeedEffect, new_modifier);
    }
    Ok(())
}

fn send_effect_to_client(
    effect_id: u8,
    network_id: NetworkId,
    active_effect: &EffectApplication,
    client: &Client,
    duration: u64,
) {
    if duration == 0 {
        return;
    }

    client.send_entity_effect(
        network_id,
        effect_id,
        active_effect.amplifier as i8,
        duration as i32,
        active_effect.flags,
    );
}

// todo change particle color
fn add_particles(game: &mut Game, entity: Entity, _effect_kind: Effect) -> SysResult {
    if game.tick_count % (TPS * 2) as u64 == 0 {
        let position = *game.ecs.get::<Position>(entity)?;

        let mut entity_builder = game.create_entity_builder(position, EntityInit::AreaEffectCloud);

        entity_builder.add(position);
        entity_builder.add(Particle {
            kind: ParticleKind::Effect,
            offset_x: 0.0,
            offset_y: 0.0,
            offset_z: 0.0,
            count: 5,
        });
        game.spawn_entity(entity_builder);
    }
    Ok(())
}

/// Set start_tick to all effects in effects_bucket and spawn particles
macro_rules! add_start_tick_to_effects {
    ($fn_name:ident,$type:ident) => {
        fn $fn_name(game: &mut Game, server: &mut Server) -> SysResult {
            let mut entities = vec![];
            for (entity, (&client_id, effects_bucket, &network_id)) in game
                .ecs
                .query::<(&ClientId, &mut $type, &NetworkId)>()
                .iter()
            {
                if effects_bucket.0.is_empty() {
                    continue;
                }

                if let Some(active_effect) = effects_bucket.active_effect() {
                    if active_effect.flags.particle {
                        entities.push((entity, Effect::$type));
                    }
                }

                let not_started = effects_bucket.not_started();

                for mut effect in not_started {
                    effect.start_tick = game.tick_count;

                    if let Some(client) = server.clients.get(client_id) {
                        send_effect_to_client(
                            Effect::$type.id() as u8,
                            network_id,
                            &effect,
                            client,
                            effect.duration as u64,
                        );
                    }

                    effects_bucket.0.replace(effect);
                }
            }

            for (entity, effect_kind) in entities {
                add_particles(game, entity, effect_kind)?;
            }

            Ok(())
        }
    };
}

add_start_tick_to_effects!(add_start_tick_to_speed_effects, SpeedEffect);
