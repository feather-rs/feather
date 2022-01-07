//! Sends tablist info to clients via the Player Info packet.

use base::{Gamemode, ProfileProperty};
use common::{
    events::{EntityRemoveEvent, PlayerJoinEvent, TablistExtrasUpdateEvent},
    Game,
};
use ecs::{SysResult, SystemExecutor};
use quill_common::{components::Name, entities::Player};
use uuid::Uuid;

use crate::{ClientId, Server};

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems
        .group::<Server>()
        .add_system(remove_tablist_players)
        .add_system(add_tablist_players)
        .add_system(update_tablist_header);
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

fn update_tablist_header(game: &mut Game, server: &mut Server) -> SysResult {
    game.ecs.insert_event(TablistExtrasUpdateEvent{ header: Some("{\"text\":\"xDDDDD\"}".to_string()), footer: Some("{\"text\":\"xDDDDDDD\"}".to_string()) });
    let default = "{\"text\":\"\"}";
    for (_,event) in game
        .ecs
        .query::<&TablistExtrasUpdateEvent>()
        .iter() 
    {
        server.broadcast_with(|client| client.send_tablist_header_footer(event.header.as_deref().unwrap_or(default), event.footer.as_deref().unwrap_or(default)))    
    }
    Ok(())
}
