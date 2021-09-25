use base::Position;
use common::Game;
use ecs::{Entity, EntityRef, SysResult};
use protocol::packets::client::{
    PlayerAbilities, PlayerMovement, PlayerPosition, PlayerPositionAndRotation, PlayerRotation,
};
use quill_common::{
    components::{CreativeFlying, OnGround},
    events::CreativeFlyingEvent,
};

use crate::{ClientId, Server};

/// If a player has been teleported by the server,
/// we don't want to override their position if
/// we receive a movement packet before the client
/// is aware of the position update.
fn should_skip_movement(server: &Server, player: &EntityRef) -> SysResult<bool> {
    if let Some(client) = server.clients.get(*player.get::<ClientId>()?) {
        let server_position = *player.get::<Position>()?;
        let client_position = client.client_known_position();
        if let Some(client_position) = client_position {
            if client_position != server_position {
                // Player has been teleported by the server.
                // Don't override.
                return Ok(true);
            }
        }
    }
    Ok(false)
}

pub fn handle_player_movement(player: EntityRef, packet: PlayerMovement) -> SysResult {
    player.get_mut::<OnGround>()?.0 = packet.on_ground;
    Ok(())
}

pub fn handle_player_position(
    server: &Server,
    player: EntityRef,
    packet: PlayerPosition,
) -> SysResult {
    if should_skip_movement(server, &player)? {
        return Ok(());
    }
    let mut pos = player.get_mut::<Position>()?;
    pos.x = packet.x;
    pos.y = packet.feet_y;
    pos.z = packet.z;
    player.get_mut::<OnGround>()?.0 = packet.on_ground;
    update_client_position(server, player, *pos)?;
    Ok(())
}

pub fn handle_player_position_and_rotation(
    server: &Server,
    player: EntityRef,
    packet: PlayerPositionAndRotation,
) -> SysResult {
    if should_skip_movement(server, &player)? {
        return Ok(());
    }
    let mut pos = player.get_mut::<Position>()?;
    pos.x = packet.x;
    pos.y = packet.feet_y;
    pos.z = packet.z;
    pos.yaw = packet.yaw;
    pos.pitch = packet.pitch;
    player.get_mut::<OnGround>()?.0 = packet.on_ground;
    update_client_position(server, player, *pos)?;
    Ok(())
}

pub fn handle_player_rotation(
    server: &Server,
    player: EntityRef,
    packet: PlayerRotation,
) -> SysResult {
    if should_skip_movement(server, &player)? {
        return Ok(());
    }
    let mut pos = player.get_mut::<Position>()?;
    pos.yaw = packet.yaw;
    pos.pitch = packet.pitch;
    player.get_mut::<OnGround>()?.0 = packet.on_ground;
    update_client_position(server, player, *pos)?;
    Ok(())
}

fn update_client_position(server: &Server, player: EntityRef, pos: Position) -> SysResult {
    if let Some(client) = server.clients.get(*player.get::<ClientId>()?) {
        client.set_client_known_position(pos);
    }
    Ok(())
}

/// Handles the PlayerAbilities packet that signals that the client wants to
/// start/stop flying (like in creative mode).
pub fn handle_player_abilities(
    game: &mut Game,
    player: Entity,
    packet: PlayerAbilities,
) -> SysResult {
    let flying = game.ecs.get_mut::<CreativeFlying>(player)?.0;

    match packet.flags {
        0 => {
            // Flying stopped
            if flying {
                // Then it used to fly, therefor we need to trigger a event
                // The vanilla client is actually quite good at keeping track of sending
                // this packet only when there is a change, so this if should basically
                // always trigger.
                game.ecs
                    .insert_entity_event(player, CreativeFlyingEvent::new(false))?;

                game.ecs.get_mut::<CreativeFlying>(player)?.0 = false;
            }
        }
        2 => {
            // Flying started
            if !flying {
                // Then it used to not fly, therefor we need to trigger a event.
                // The vanilla client is actually quite good at keeping track of sending
                // this packet only when there is a change, so this if should basically
                // always trigger.
                game.ecs
                    .insert_entity_event(player, CreativeFlyingEvent::new(true))?;
                game.ecs.get_mut::<CreativeFlying>(player)?.0 = true;
            }
        }
        err => {
            log::error!("Got a unexpected flag in the PlayerAbilities packet. The value was: {} and not 0 or 2.", err)
        }
    }

    Ok(())
}
