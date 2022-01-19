//! Systems linking a `Server` and a `Game`.

mod block;
mod chat;
mod entity;
mod gamemode;
mod particle;
mod player_join;
mod player_leave;
mod plugin_message;
mod tablist;
pub mod view;

use std::time::{Duration, Instant};

use common::Game;
use ecs::{SysResult, SystemExecutor};
use quill_common::components::Name;

use crate::{client::ClientId, Server};

/// Registers systems for a `Server` with a `Game`.
pub fn register(server: Server, game: &mut Game, systems: &mut SystemExecutor<Game>) {
    game.insert_resource(server);

    player_join::register(systems);
    systems
        .group::<Server>()
        .add_system(handle_packets)
        .add_system(send_keepalives);
    view::register(game, systems);
    crate::chunk_subscriptions::register(systems);
    player_leave::register(systems);
    tablist::register(systems);
    block::register(systems);
    entity::register(game, systems);
    chat::register(game, systems);
    particle::register(systems);
    plugin_message::register(systems);
    gamemode::register(systems);

    systems.group::<Server>().add_system(tick_clients);
}

/// Polls for packets received from clients
/// and handles them.
fn handle_packets(game: &mut Game, server: &mut Server) -> SysResult {
    let mut packets = Vec::new();

    for (player, &client_id) in game.ecs.query::<&ClientId>().iter() {
        if let Some(client) = server.clients.get(client_id) {
            for packet in client.received_packets() {
                packets.push((player, packet));
            }
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

/// Sends out keepalive packets at an interval.
fn send_keepalives(_game: &mut Game, server: &mut Server) -> SysResult {
    let interval = Duration::from_secs(5);
    if server.last_keepalive_time + interval < Instant::now() {
        server.broadcast_keepalive();
    }
    Ok(())
}

/// Ticks `Client`s.
fn tick_clients(_game: &mut Game, server: &mut Server) -> SysResult {
    for client in server.clients.iter() {
        client.tick();
    }

    Ok(())
}
