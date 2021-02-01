use base::Position;
use ecs::{EntityRef, SysResult};
use protocol::packets::client::{
    PlayerMovement, PlayerPosition, PlayerPositionAndRotation, PlayerRotation,
};

pub fn handle_player_movement(player: EntityRef, packet: PlayerMovement) -> SysResult {
    player.get_mut::<Position>()?.on_ground = packet.on_ground;
    Ok(())
}

pub fn handle_player_position(player: EntityRef, packet: PlayerPosition) -> SysResult {
    let mut pos = player.get_mut::<Position>()?;
    pos.x = packet.x;
    pos.y = packet.feet_y;
    pos.z = packet.z;
    pos.on_ground = packet.on_ground;
    Ok(())
}

pub fn handle_player_position_and_rotation(
    player: EntityRef,
    packet: PlayerPositionAndRotation,
) -> SysResult {
    let mut pos = player.get_mut::<Position>()?;
    pos.x = packet.x;
    pos.y = packet.feet_y;
    pos.z = packet.z;
    pos.yaw = packet.yaw;
    pos.pitch = packet.pitch;
    pos.on_ground = packet.on_ground;
    Ok(())
}

pub fn handle_player_rotation(player: EntityRef, packet: PlayerRotation) -> SysResult {
    let mut pos = player.get_mut::<Position>()?;
    pos.yaw = packet.yaw;
    pos.pitch = packet.pitch;
    pos.on_ground = packet.on_ground;
    Ok(())
}
