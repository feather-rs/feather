//! Sends tablist info to clients via the Player Info packet.

use base::{Gamemode, ProfileProperty};
use common::{
    events::{EntityRemoveEvent, PlayerJoinEvent, TablistExtrasUpdateEvent, TablistHeaderFooter},
    Game,
};
use ecs::{SysResult, SystemExecutor};
use quill_common::{components::Name, entities::Player};
use uuid::Uuid;

use crate::{ClientId, Server};

pub fn register(game: &mut Game, systems: &mut SystemExecutor<Game>) {
    game.insert_resource(TablistHeaderFooter {
        header: "{\"text\":\"\"}".to_string(),
        footer: "{\"text\":\"\"}".to_string(),
    });
    systems
        .group::<Server>()
        .add_system(remove_tablist_players)
        .add_system(add_tablist_players)
        .add_system(update_tablist_header)
        .add_system(send_tablist_header_on_join);
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
    for _ in game.ecs.query::<&TablistExtrasUpdateEvent>().iter() {
        let header_footer = game.resources.get::<TablistHeaderFooter>()?;
        server.broadcast_with(|client| {
            client.send_tablist_header_footer(&header_footer.header, &header_footer.footer)
        });
    }
    Ok(())
}

fn send_tablist_header_on_join(game: &mut Game, server: &mut Server) -> SysResult {
    for (_, (_, &client_id)) in game.ecs.query::<(&PlayerJoinEvent, &ClientId)>().iter() {
        let header_footer = game.resources.get::<TablistHeaderFooter>()?;
        server
            .clients
            .get(client_id)
            .unwrap()
            .send_tablist_header_footer(&header_footer.header, &header_footer.footer);
    }
    Ok(())
}
