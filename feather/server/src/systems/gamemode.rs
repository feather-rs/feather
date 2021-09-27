use std::collections::HashMap;

use base::Gamemode;
use common::Game;
use ecs::{SysResult, SystemExecutor};
use quill_common::components::{
    CanBuild, CanCreativeFly, ClientId, CreativeFlying, CreativeFlyingSpeed, Instabreak,
    Invulnerable, WalkSpeed,
};
use quill_common::events::{CreativeFlyingEvent, GamemodeUpdateEvent, InstabreakChangeEvent};

use crate::Server;

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(gamemode_change);
}

fn gamemode_change(game: &mut Game, server: &mut Server) -> SysResult {
    let mut may_fly_changes = HashMap::new();
    let mut fly_changes = HashMap::new();
    let mut instabreak_changes = HashMap::new();
    let mut build_changes = HashMap::new();
    let mut invulnerability_changes = HashMap::new();
    for (
        entity,
        (
            event,
            &client_id,
            &_walk_speed,
            &_fly_speed,
            mut may_fly,
            mut is_flying,
            mut instabreak,
            mut may_build,
            mut invulnerable,
        ),
    ) in game
        .ecs
        .query::<(
            &GamemodeUpdateEvent,
            &ClientId,
            &WalkSpeed,
            &CreativeFlyingSpeed,
            &mut CanCreativeFly,
            &mut CreativeFlying,
            &mut Instabreak,
            &mut CanBuild,
            &mut Invulnerable,
        )>()
        .iter()
    {
        match event.new {
            Gamemode::Creative => {
                if !**instabreak {
                    instabreak_changes.insert(entity, true);
                    instabreak.0 = true;
                }
                if !**may_fly {
                    may_fly_changes.insert(entity, true);
                    may_fly.0 = true;
                }
                if !**may_build {
                    build_changes.insert(entity, true);
                    may_build.0 = true;
                }
                if !**invulnerable {
                    invulnerability_changes.insert(entity, true);
                    invulnerable.0 = true;
                }
            }
            Gamemode::Spectator => {
                if !**is_flying {
                    fly_changes.insert(entity, true);
                    is_flying.0 = true;
                }
                if **instabreak {
                    instabreak_changes.insert(entity, false);
                    instabreak.0 = false;
                }
                if !**may_fly {
                    may_fly_changes.insert(entity, true);
                    may_fly.0 = true;
                }
                if **may_build {
                    build_changes.insert(entity, false);
                    may_build.0 = false;
                }
                if !**invulnerable {
                    invulnerability_changes.insert(entity, true);
                    invulnerable.0 = true;
                }
            }
            Gamemode::Survival => {
                if **is_flying {
                    fly_changes.insert(entity, false);
                    is_flying.0 = false;
                }
                if **instabreak {
                    instabreak_changes.insert(entity, false);
                    instabreak.0 = false;
                }
                if **may_fly {
                    may_fly_changes.insert(entity, false);
                    may_fly.0 = false;
                }
                if !**may_build {
                    build_changes.insert(entity, true);
                    may_build.0 = true;
                }
                if **invulnerable {
                    invulnerability_changes.insert(entity, false);
                    invulnerable.0 = false;
                }
            }
            Gamemode::Adventure => {
                if **is_flying {
                    fly_changes.insert(entity, false);
                    is_flying.0 = false;
                }
                if **instabreak {
                    instabreak_changes.insert(entity, false);
                    instabreak.0 = false;
                }
                if **may_fly {
                    may_fly_changes.insert(entity, false);
                    may_fly.0 = false;
                }
                if **may_build {
                    build_changes.insert(entity, false);
                    may_build.0 = false;
                }
                if **invulnerable {
                    invulnerability_changes.insert(entity, false);
                    invulnerable.0 = false;
                }
            }
        }
        server
            .clients
            .get(client_id)
            .unwrap()
            .change_gamemode(event.new);
        // TODO is this needed?
        //server
        //    .clients
        //    .get(client_id)
        //    .unwrap()
        //    .send_abilities(&PlayerAbilities {
        //        walk_speed,
        //        fly_speed,
        //        may_fly: *may_fly,
        //        is_flying: *is_flying,
        //        may_build: *may_build,
        //        instabreak: *instabreak,
        //        invulnerable: *invulnerable,
        //    });
    }
    for (entity, flying) in fly_changes {
        if flying {
            game.ecs
                .insert_entity_event(entity, CreativeFlyingEvent::new(true))
                .unwrap();
        } else {
            game.ecs
                .insert_entity_event(entity, CreativeFlyingEvent::new(false))
                .unwrap();
        }
    }
    for (entity, instabreak) in instabreak_changes {
        if instabreak {
            game.ecs
                .insert_entity_event(entity, InstabreakChangeEvent(true))
                .unwrap();
        } else {
            game.ecs
                .insert_entity_event(entity, InstabreakChangeEvent(false))
                .unwrap();
        }
    }
    Ok(())
}
