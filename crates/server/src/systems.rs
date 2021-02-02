//! Systems linking a `Server` and a `Game`.

mod view;

use anyhow::anyhow;
use base::{Gamemode, Position};
use common::{view::View, Game, Name};
use ecs::{SysResult, SystemExecutor};

use crate::{client::ClientId, Server};

/// Registers systems for a `Server` with a `Game`.
pub fn register(server: Server, game: &mut Game, systems: &mut SystemExecutor<Game>) {
    game.insert_resource(server);
    systems
        .group::<Server>()
        .add_system(poll_new_players)
        .add_system(handle_packets);
    view::register(game, systems);
}

/// Polls for new clients and sends them the necessary packets
/// to join the game.
fn poll_new_players(game: &mut Game, server: &mut Server) -> SysResult {
    for client_id in server.accept_new_players() {
        accept_new_player(game, server, client_id)?;
    }
    Ok(())
}

fn accept_new_player(game: &mut Game, server: &mut Server, client_id: ClientId) -> SysResult {
    let client = server.clients.get(client_id).unwrap();
    client.send_join_game(Gamemode::Creative);
    client.send_brand();

    let mut builder = game.create_entity_builder();
    common::entity::player::build(&mut builder);
    builder
        .add(Position::default())
        .add(client.network_id())
        .add(client_id)
        .add(View::new(
            Position::default().chunk(),
            server.options.view_distance,
        ))
        .add(Name(client.username().into()));
    game.spawn_entity(builder);

    Ok(())
}

/// Polls for packets received from clients
/// and handles them.
fn handle_packets(game: &mut Game, server: &mut Server) -> SysResult {
    let mut packets = Vec::new();

    for (player, &client_id) in game.ecs.query::<&ClientId>().iter() {
        let client = server
            .clients
            .get(client_id)
            .ok_or_else(|| anyhow!("missing Client"))?;
        for packet in client.received_packets() {
            packets.push((player, packet));
        }
    }

    for (player, packet) in packets {
        if let Err(e) = crate::packet_handlers::handle_packet(game, server, player, packet) {
            log::warn!(
                "Failed to handle packet from '{}': {:?}",
                &**game.ecs.get::<Name>(player)?,
                e
            );
        }
    }

    Ok(())
}
