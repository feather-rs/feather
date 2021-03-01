use base::{BlockId, BlockPosition};
use common::Game;
use ecs::{Entity, EntityRef, SysResult};
use protocol::packets::client::{PlayerBlockPlacement, PlayerDigging, PlayerDiggingStatus, BlockFace};
use crate::{ClientId, Server};

/// Handles the player block placement packet. Currently just removes the block client side for the player.
pub fn handle_player_block_placement(server: &mut Server, packet: PlayerBlockPlacement, player: EntityRef) -> SysResult {
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

    client.send_block_change(position, BlockId::air());
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
