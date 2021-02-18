use common::Game;
use ecs::{Entity, SysResult};
use protocol::packets::client::{PlayerDigging, PlayerDiggingStatus};

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
            game.break_block_at(packet.position);
            Ok(())
        }
        _ => Ok(()),
    }
}
