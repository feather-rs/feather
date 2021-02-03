use base::{Gamemode, Position};
use common::{view::View, Game, Name};
use ecs::{SysResult, SystemExecutor};

use crate::{ClientId, Server};

pub fn register(systems: &mut SystemExecutor<Game>) {
    systems.group::<Server>().add_system(poll_new_players);
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
    client.update_own_position(Position::default());

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
        .add(Gamemode::Creative)
        .add(Name(client.username().into()))
        .add(client.uuid())
        .add(client.profile().to_vec());
    game.spawn_entity(builder);

    Ok(())
}
