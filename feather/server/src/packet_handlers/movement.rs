use base::Position;
use ecs::{EntityRef, SysResult};
use protocol::packets::client::{
    PlayerMovement, PlayerPosition, PlayerPositionAndRotation, PlayerRotation,
};
use quill_common::components::OnGround;

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
    let pos_copy = *pos;
    drop(pos);
    update_client_position(server, player, pos_copy)?;
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
    let pos_copy = *pos;
    drop(pos);
    update_client_position(server, player, pos_copy)?;
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
    let pos_copy = *pos;
    drop(pos);
    update_client_position(server, player, pos_copy)?;
    Ok(())
}

fn update_client_position(server: &Server, player: EntityRef, pos: Position) -> SysResult {
    if let Some(client) = server.clients.get(*player.get::<ClientId>()?) {
        client.set_client_known_position(pos);
    }
    Ok(())
}
