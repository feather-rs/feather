use std::collections::HashMap;

use crate::Server;
use base::Gamemode;
use common::Game;
use ecs::{SysResult, SystemExecutor};
use quill_common::components::{ClientId, CreativeFlying};
use quill_common::events::{CreativeFlyingEvent, GamemodeUpdateEvent};

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(gamemode_change);
}

fn gamemode_change(game: &mut Game, server: &mut Server) -> SysResult {
    let mut changes = HashMap::new();
    for (entity, (_, &client_id, &gamemode, mut flying)) in game
        .ecs
        .query::<(
            &GamemodeUpdateEvent,
            &ClientId,
            &Gamemode,
            &mut CreativeFlying,
        )>()
        .iter()
    {
        match gamemode {
            Gamemode::Spectator => {
                if !flying.0 {
                    changes.insert(entity, true);
                    flying.0 = true;
                }
            }
            Gamemode::Survival | Gamemode::Adventure => {
                if flying.0 {
                    changes.insert(entity, false);
                    flying.0 = false;
                }
            }
            _ => (),
        }
        server
            .clients
            .get(client_id)
            .unwrap()
            .change_gamemode(gamemode);
    }
    for (entity, flying) in changes {
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
    Ok(())
}
