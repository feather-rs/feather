//! Sends tablist info to clients via the Player Info packet.

use uuid::Uuid;

use base::{Gamemode, ProfileProperty};
use common::Game;
use ecs::{SysResult, SystemExecutor};
use quill_common::events::{EntityRemoveEvent, GamemodeEvent, PlayerJoinEvent};
use quill_common::{components::Name, entities::Player};

use crate::{ClientId, Server};

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems
        .group::<Server>()
        .add_system(remove_tablist_players)
        .add_system(add_tablist_players)
        .add_system(change_tablist_player_gamemode);
}

fn remove_tablist_players(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, (_event, _player, &uuid)) in game
        .ecs
        .query::<(&EntityRemoveEvent, &Player, &Uuid)>()
        .iter()
    {
        server.broadcast_with(|client| client.remove_tablist_player(uuid));
    }
    Ok(())
}

fn add_tablist_players(game: &mut Game, server: &mut Server) -> SysResult {
    for (player, (_, &client_id, &uuid, name, &gamemode, profile)) in game
        .ecs
        .query::<(
            &PlayerJoinEvent,
            &ClientId,
            &Uuid,
            &Name,
            &Gamemode,
            &Vec<ProfileProperty>,
        )>()
        .iter()
    {
        // Add this player to other players' tablists
        server.broadcast_with(|client| {
            client.add_tablist_player(uuid, name.to_string(), profile, gamemode)
        });

        // Add other players to this player's tablist
        for (other_player, (&uuid, name, &gamemode, profile)) in game
            .ecs
            .query::<(&Uuid, &Name, &Gamemode, &Vec<ProfileProperty>)>()
            .iter()
        {
            if let Some(client) = server.clients.get(client_id) {
                if other_player != player {
                    client.add_tablist_player(uuid, name.to_string(), profile, gamemode);
                }
            }
        }
    }
    Ok(())
}

fn change_tablist_player_gamemode(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, (event, &uuid)) in game.ecs.query::<(&GamemodeEvent, &Uuid)>().iter() {
        // Change this player's gamemode in players' tablists
        server.broadcast_with(|client| client.change_player_tablist_gamemode(uuid, **event));
    }
    Ok(())
}
