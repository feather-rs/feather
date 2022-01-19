use base::anvil::player::PlayerAbilities;
use base::Gamemode;
use common::Game;
use ecs::{SysResult, SystemExecutor};
use quill_common::components::{
    CanBuild, CanCreativeFly, CreativeFlying, CreativeFlyingSpeed, Instabreak, Invulnerable,
    PreviousGamemode, WalkSpeed,
};
use quill_common::events::{
    BuildingAbilityEvent, CreativeFlyingEvent, FlyingAbilityEvent, GamemodeEvent, InstabreakEvent,
    InvulnerabilityEvent,
};

use crate::{ClientId, Server};

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(gamemode_change);
}

fn gamemode_change(game: &mut Game, server: &mut Server) -> SysResult {
    let mut may_fly_changes = Vec::new();
    let mut fly_changes = Vec::new();
    let mut instabreak_changes = Vec::new();
    let mut build_changes = Vec::new();
    let mut invulnerability_changes = Vec::new();
    for (
        entity,
        (
            event,
            &client_id,
            &walk_speed,
            &fly_speed,
            mut may_fly,
            mut is_flying,
            mut instabreak,
            mut may_build,
            mut invulnerable,
            gamemode,
            prev_gamemode,
        ),
    ) in game
        .ecs
        .query::<(
            &GamemodeEvent,
            &ClientId,
            &WalkSpeed,
            &CreativeFlyingSpeed,
            &mut CanCreativeFly,
            &mut CreativeFlying,
            &mut Instabreak,
            &mut CanBuild,
            &mut Invulnerable,
            &mut Gamemode,
            &mut PreviousGamemode,
        )>()
        .iter()
    {
        if **event == *gamemode {
            continue;
        }
        *prev_gamemode = PreviousGamemode(Some(*gamemode));
        *gamemode = **event;
        match gamemode {
            Gamemode::Creative => {
                if !**instabreak {
                    instabreak_changes.push((entity, true));
                    instabreak.0 = true;
                }
                if !**may_fly {
                    may_fly_changes.push((entity, true));
                    may_fly.0 = true;
                }
                if !**may_build {
                    build_changes.push((entity, true));
                    may_build.0 = true;
                }
                if !**invulnerable {
                    invulnerability_changes.push((entity, true));
                    invulnerable.0 = true;
                }
            }
            Gamemode::Spectator => {
                if !**is_flying {
                    fly_changes.push((entity, true));
                    is_flying.0 = true;
                }
                if **instabreak {
                    instabreak_changes.push((entity, false));
                    instabreak.0 = false;
                }
                if !**may_fly {
                    may_fly_changes.push((entity, true));
                    may_fly.0 = true;
                }
                if **may_build {
                    build_changes.push((entity, false));
                    may_build.0 = false;
                }
                if !**invulnerable {
                    invulnerability_changes.push((entity, true));
                    invulnerable.0 = true;
                }
            }
            Gamemode::Survival => {
                if **is_flying {
                    fly_changes.push((entity, false));
                    is_flying.0 = false;
                }
                if **instabreak {
                    instabreak_changes.push((entity, false));
                    instabreak.0 = false;
                }
                if **may_fly {
                    may_fly_changes.push((entity, false));
                    may_fly.0 = false;
                }
                if !**may_build {
                    build_changes.push((entity, true));
                    may_build.0 = true;
                }
                if **invulnerable {
                    invulnerability_changes.push((entity, false));
                    invulnerable.0 = false;
                }
            }
            Gamemode::Adventure => {
                if **is_flying {
                    fly_changes.push((entity, false));
                    is_flying.0 = false;
                }
                if **instabreak {
                    instabreak_changes.push((entity, false));
                    instabreak.0 = false;
                }
                if **may_fly {
                    may_fly_changes.push((entity, false));
                    may_fly.0 = false;
                }
                if **may_build {
                    build_changes.push((entity, false));
                    may_build.0 = false;
                }
                if **invulnerable {
                    invulnerability_changes.push((entity, false));
                    invulnerable.0 = false;
                }
            }
        }
        server
            .clients
            .get(client_id)
            .unwrap()
            .change_gamemode(**event);
        server
            .clients
            .get(client_id)
            .unwrap()
            .send_abilities(&PlayerAbilities {
                walk_speed,
                fly_speed,
                may_fly: *may_fly,
                is_flying: *is_flying,
                may_build: *may_build,
                instabreak: *instabreak,
                invulnerable: *invulnerable,
            });
    }
    for (entity, flying) in fly_changes {
        game.ecs
            .insert_entity_event(entity, CreativeFlyingEvent::new(flying))
            .unwrap();
    }
    for (entity, instabreak) in instabreak_changes {
        game.ecs
            .insert_entity_event(entity, InstabreakEvent(instabreak))
            .unwrap();
    }
    for (entity, may_fly) in may_fly_changes {
        game.ecs
            .insert_entity_event(entity, FlyingAbilityEvent(may_fly))
            .unwrap();
    }
    for (entity, build) in build_changes {
        game.ecs
            .insert_entity_event(entity, BuildingAbilityEvent(build))
            .unwrap();
    }
    for (entity, invulnerable) in invulnerability_changes {
        game.ecs
            .insert_entity_event(entity, InvulnerabilityEvent(invulnerable))
            .unwrap();
    }
    Ok(())
}
