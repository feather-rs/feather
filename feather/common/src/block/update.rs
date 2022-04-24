use libcraft::BlockPosition;
use vane::SysResult;
use libcraft::BlockDirection;
use utils::continue_on_none;

use crate::{events::BlockChangeEvent, Game};

use super::wall::update_wall_connections;

pub fn block_update(game: &mut Game) -> SysResult {
    for (_, event) in game.ecs.query::<&BlockChangeEvent>().iter() {
        let mut world = game.world_mut(event.world())?;
        for pos in event.iter_changed_blocks().map(Into::<BlockPosition>::into) {
            for adjacent in [
                BlockDirection::East,
                BlockDirection::West,
                BlockDirection::North,
                BlockDirection::South,
                BlockDirection::Bottom,
            ]
            .iter()
            .map(|&d| [pos.adjacent(d), pos.adjacent(BlockDirection::Bottom).adjacent(d)])
            .flatten()
            {
                continue_on_none!(update_wall_connections(&mut world, adjacent));
            }
        }
    }
    Ok(())
}
