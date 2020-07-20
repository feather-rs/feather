use crate::{RedstoneBlock, RedstoneCache};
use feather_core::blocks::{BlockId, BlockKind};
use feather_core::util::BlockPosition;
use feather_server_types::Game;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RedstoneTorchState {
    pub powered: bool,
    pub connected_to: BlockPosition,
}

impl RedstoneTorchState {
    pub fn calculate(
        pos: BlockPosition,
        redstone_block: &RedstoneBlock,
        cache: &mut RedstoneCache,
        game: &Game,
    ) -> Self {
        let block = cache.block_at(pos, game);
        println!("{:?}", block);
        let connected_to = match redstone_block {
            RedstoneBlock::RedstoneTorch(state) => state.connected_to,
            _ => panic!("Expected a redstone torch at this position"),
        };

        let powered = false;

        Self {
            powered,
            connected_to,
        }
    }

    pub fn from_block(block: BlockId) -> Option<Self> {
        match block.kind() {
            BlockKind::RedstoneTorch => Some(Self {
                powered: block.lit().unwrap(),
                connected_to: BlockPosition::new(0, -1, 0),
            }),
            BlockKind::RedstoneWallTorch => Some(Self {
                powered: block.lit().unwrap(),
                connected_to: block.facing_cardinal().unwrap().offset(),
            }),
            _ => None,
        }
    }
}
