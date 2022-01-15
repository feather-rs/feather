use base::BlockPosition;
use ecs::SysResult;
use libcraft_core::BlockFace;

use crate::{events::BlockChangeEvent, Game};

use super::wall::connect_neighbours_and_up;

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
                if connect_neighbours_and_up(&mut game.world, adjacent).is_none() {
                    continue;
                }
            }
        }
    }
    Ok(())
}
