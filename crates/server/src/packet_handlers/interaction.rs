use crate::{ClientId, Server};
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
    let position = match packet.face {
        BlockFace::Bottom => packet.position.down(),
        BlockFace::Top => packet.position.up(),
        BlockFace::North => packet.position.north(),
        BlockFace::South => packet.position.south(),
        BlockFace::West => packet.position.west(),
        BlockFace::East => packet.position.east(),
    };

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
