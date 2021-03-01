use crate::{ClientId, Server};
use base::BlockPosition;
use common::Game;
use ecs::{Entity, EntityRef, SysResult};
use protocol::packets::client::{
    BlockFace, PlayerBlockPlacement, PlayerDigging, PlayerDiggingStatus,
};

/// Handles the player block placement packet. Currently just removes the block client side for the player.
pub fn handle_player_block_placement(
    game: &Game,
    server: &mut Server,
    packet: PlayerBlockPlacement,
    player: EntityRef,
) -> SysResult {
    let transform = match packet.face {
        BlockFace::Bottom => BlockPosition::new(0, -1, 0),
        BlockFace::Top => BlockPosition::new(0, 1, 0),
        BlockFace::North => BlockPosition::new(0, 0, -1),
        BlockFace::South => BlockPosition::new(0, 0, 1),
        BlockFace::West => BlockPosition::new(-1, 0, 0),
        BlockFace::East => BlockPosition::new(1, 0, 0),
    };
    let position = packet.position + transform;

    log::trace!("Got player block placement at {:?}", position);

    let client = server.clients.get(*player.get::<ClientId>()?).unwrap();
    let block_id = game.block(position).unwrap_or_default();

    client.send_block_change(position, block_id);
    Ok(())
}

/// Handles the Player Digging packet sent for the following
/// actions:
/// * Breaking blocks.
/// * Dropping items.
/// * Shooting arrows.
/// * Eating.
/// * Swapping items between the main and off hand.
pub fn handle_player_digging(game: &mut Game, packet: PlayerDigging, _player: Entity) -> SysResult {
    log::trace!("Got player digging with status {:?}", packet.status);
    match packet.status {
        PlayerDiggingStatus::StartDigging | PlayerDiggingStatus::CancelDigging => {
            game.break_block(packet.position);
            Ok(())
        }
        _ => Ok(()),
    }
}
