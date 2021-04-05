use base::Gamemode;
use common::Game;
use ecs::{Entity, SysResult};
use protocol::packets::client::ClientStatus;
use quill_common::events::EntityResurrectionEvent;

use crate::{ClientId, Server};

pub fn handle_client_status(
    game: &mut Game,
    server: &mut Server,
    player: Entity,
    packet: ClientStatus,
) -> SysResult {
    match packet {
        ClientStatus::PerformRespawn => {
            let mut send_event = false;
            {
                let client_id = game.ecs.get::<ClientId>(player)?;
                if let Some(client) = server.clients.get(*client_id) {
                    let gamemode = game.ecs.get::<Gamemode>(player)?;
                    client.send_respawn(*gamemode, false);

                    send_event = true;
                }
            }

            if send_event {
                game.ecs
                    .insert_entity_event(player, EntityResurrectionEvent)?;
            }
        }

        ClientStatus::RequestStats => {}
    }

    Ok(())
}
