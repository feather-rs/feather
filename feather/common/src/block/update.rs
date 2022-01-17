use base::BlockPosition;
use ecs::SysResult;
use libcraft_core::BlockFace;
use utils::continue_on_none;

use crate::{events::BlockChangeEvent, Game};

use super::wall::update_wall_connections;

/// TODO: send updated blocks to player
pub fn block_update(game: &mut Game) -> SysResult {
    for (_, event) in game.ecs.query::<&BlockChangeEvent>().iter() {
        for pos in event.iter_changed_blocks().map(Into::<BlockPosition>::into) {
            for adjacent in [
                BlockFace::East,
                BlockFace::West,
                BlockFace::North,
                BlockFace::South,
                BlockFace::Bottom,
            ]
            .iter()
            .map(|&d| [pos.adjacent(d), pos.adjacent(BlockFace::Bottom).adjacent(d)])
            .flatten()
            {
                continue_on_none!(update_wall_connections(&mut game.world, adjacent));
            }
        }
    }
    Ok(())
}
