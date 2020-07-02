#![forbid(unsafe_code)]
use feather_core::{
    blocks::{BlockId, BlockKind, FacingCubic},
    util::BlockPosition,
};
use feather_server_types::Game;
use std::collections::HashMap;
use std::convert::{From, Into};
use std::iter::IntoIterator;
use std::rc::Rc;

pub mod blocks;

mod event_handlers;
pub use event_handlers::on_block_update_redstone;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum RedstoneBlock {
    RedstoneWire(blocks::RedstoneState),
    FullBlock,
    None,
}

impl RedstoneBlock {
    fn is_full_block(&self) -> bool {
        matches!(self, RedstoneBlock::FullBlock)
    }

    // fn is_none(&self) -> bool {
    //     matches!(self, RedstoneBlock::None)
    // }
}

impl From<BlockId> for RedstoneBlock {
    fn from(block: BlockId) -> Self {
        match block.kind() {
            BlockKind::RedstoneWire => RedstoneBlock::RedstoneWire(
                blocks::RedstoneState::from_block(block).expect("This block is a redstone wire"),
            ),

            _ if block.is_full_block() => RedstoneBlock::FullBlock,
            _ => RedstoneBlock::None,
        }
    }
}

#[derive(Debug)]
pub enum VerticalOffset {
    None,
    Below,
    Above,
}

#[derive(Debug)]
pub struct CachedRedstoneBlock {
    block_id: BlockId,
    redstone_kind: RedstoneBlock,
}

// Cache for blocks
#[derive(Debug, Default)]
pub struct RedstoneCache {
    cache: HashMap<BlockPosition, Rc<CachedRedstoneBlock>>,
}

impl RedstoneCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn block_at(&mut self, pos: BlockPosition, game: &Game) -> Option<Rc<CachedRedstoneBlock>> {
        if !self.cache.contains_key(&pos) {
            if let Some(block) = game.block_at(pos) {
                self.cache.insert(
                    pos,
                    Rc::new(CachedRedstoneBlock {
                        block_id: block,
                        redstone_kind: block.into(),
                    }),
                );
            }
        }

        self.cache.get(&pos).map(|block| block.clone())
    }

    pub fn update_block(
        &mut self,
        pos: BlockPosition,
        state: &blocks::RedstoneState,
        mut block: BlockId,
    ) {
        state.apply_to(&mut block);

        self.cache.insert(
            pos,
            Rc::new(CachedRedstoneBlock {
                block_id: block,
                redstone_kind: block.into(),
            }),
        );
    }

    /// Finds all neighbouring blocks that directly power this block
    pub fn get_redstone_connections(
        &mut self,
        pos: BlockPosition,
        game: &Game,
    ) -> Vec<(
        Rc<CachedRedstoneBlock>,
        BlockPosition,
        FacingCubic,
        VerticalOffset,
    )> {
        let directions = vec![
            (pos + BlockPosition::new(0, 0, -1), FacingCubic::North),
            (pos + BlockPosition::new(1, 0, 0), FacingCubic::East),
            (pos + BlockPosition::new(0, 0, 1), FacingCubic::South),
            (pos + BlockPosition::new(-1, 0, 0), FacingCubic::West),
        ];

        // println!("Checking redstone connections!");
        let mut connected_redstone_components = Vec::with_capacity(8);

        let block_pos_up = pos + BlockPosition::new(0, 1, 0);
        let block_pos_down = pos + BlockPosition::new(0, -1, 0);

        for (position, direction) in directions {
            let block = self.block_at(position, game);
            if block.clone().map_or(false, |block| {
                block
                    .block_id
                    .weak_redstone_power(direction.opposite())
                    .is_some()
            }) {
                connected_redstone_components.push((
                    block.unwrap(),
                    position,
                    direction,
                    VerticalOffset::None,
                ));
            } else {
                // If no block at the same y, check one block below AND one block up
                // Also, only check for redstone wires here, because other components should not connect
                let up = position + BlockPosition::new(0, 1, 0);
                let down = position + BlockPosition::new(0, -1, 0);

                // Condition: Connect to an up block if no soid block obstructs the center
                if let Some(up_block) = self.block_at(up, game) {
                    if up_block.block_id.kind() == BlockKind::RedstoneWire
                        && !self
                            .block_at(block_pos_up, game)
                            .map_or(false, |block| block.redstone_kind.is_full_block())
                    {
                        connected_redstone_components.push((
                            up_block,
                            up,
                            direction,
                            VerticalOffset::Above,
                        ));
                    }
                }

                if let Some(down_block) = self.block_at(down, game) {
                    // Condition: Connect to a down block if not solid block is above that down block
                    if down_block.block_id.kind() == BlockKind::RedstoneWire
                        && !block.map_or(false, |block| block.redstone_kind.is_full_block())
                    {
                        connected_redstone_components.push((
                            down_block,
                            down,
                            direction,
                            VerticalOffset::Below,
                        ));
                    }
                }
            }
        }

        // last, check for the block directly above/below
        if let Some(block_up) = self.block_at(block_pos_up, game) {
            if block_up
                .block_id
                .weak_redstone_power(FacingCubic::Down)
                .is_some()
            {
                connected_redstone_components.push((
                    block_up,
                    block_pos_up,
                    FacingCubic::Up,
                    VerticalOffset::Above,
                ));
            }
        }

        if let Some(block_down) = self.block_at(block_pos_down, game) {
            if block_down
                .block_id
                .weak_redstone_power(FacingCubic::Up)
                .is_some()
            {
                connected_redstone_components.push((
                    block_down,
                    block_pos_up,
                    FacingCubic::Down,
                    VerticalOffset::Below,
                ));
            }
        }

        connected_redstone_components
    }
}

impl IntoIterator for RedstoneCache {
    type Item = (BlockPosition, Rc<CachedRedstoneBlock>);
    type IntoIter = std::collections::hash_map::IntoIter<BlockPosition, Rc<CachedRedstoneBlock>>;

    fn into_iter(self) -> Self::IntoIter {
        self.cache.into_iter()
    }
}

#[cfg(test)]
mod tets {
    use feather_core::{blocks::BlockId, util::BlockPosition};
    use feather_server_types::{BlockUpdateCause, Game};
    use feather_test_framework::Test;
    use std::iter::IntoIterator;

    fn init_blocks(test: &mut Test, blocks: Vec<Vec<&str>>) {
        for (y, block_map) in blocks.into_iter().enumerate() {
            for (z, block_row) in block_map.into_iter().enumerate() {
                for (x, block) in block_row.chars().enumerate() {
                    let block_id = match block {
                        'r' => BlockId::redstone_wire(),
                        'b' => BlockId::redstone_block(),
                        's' => BlockId::stone(),
                        ' ' => continue,
                        _ => panic!("Invalid block specifier: {}!", block),
                    };
                    test.game.set_block_at(
                        &mut test.world,
                        BlockPosition::new(x as i32, y as i32, z as i32),
                        BlockId::stone(),
                        BlockUpdateCause::Unknown,
                    );
                    let pos = BlockPosition::new(x as i32, y as i32 + 1, z as i32);
                    test.game.set_block_at(
                        &mut test.world,
                        pos,
                        block_id,
                        BlockUpdateCause::Unknown,
                    );
                }
            }
        }
    }

    fn init_blocks_2d(test: &mut Test, blocks: Vec<&str>) {
        init_blocks(test, vec![blocks])
    }

    fn assert_power(x: i32, y: i32, z: i32, game: &Game, power: i32) {
        assert_eq!(
            game.block_at(BlockPosition::new(x, y, z))
                .unwrap()
                .power()
                .unwrap(),
            power
        );
    }

    fn assert_power_2d(x: i32, z: i32, game: &Game, power: i32) {
        assert_power(x, 1, z, game, power)
    }

    #[test]
    fn test_power_redstone() {
        let mut test = Test::new();

        init_blocks_2d(&mut test, vec!["rb"]);
        assert_power_2d(0, 0, &test.game, 15);
    }

    #[test]
    fn test_power_redstone_range() {
        let mut test = Test::new();

        init_blocks_2d(&mut test, vec!["rrrrrrrrrrrrrrrrb"]);
        assert_power_2d(0, 0, &test.game, 0);
        assert_power_2d(1, 0, &test.game, 1);
    }

    #[test]
    fn test_power_falling() {
        let mut test = Test::new();

        init_blocks_2d(&mut test, vec!["r ", "rb"]);
        assert_power_2d(0, 0, &test.game, 14);

        test.game.set_block_at(
            &mut test.world,
            BlockPosition::new(1, 1, 1),
            BlockId::air(),
            BlockUpdateCause::Unknown,
        );

        assert_power_2d(0, 0, &test.game, 0);
    }

    #[test]
    fn test_multiple_power_sources() {
        let mut test = Test::new();

        init_blocks_2d(&mut test, vec!["brrrb"]);

        assert_power_2d(1, 0, &test.game, 15);
        assert_power_2d(2, 0, &test.game, 14);
        assert_power_2d(3, 0, &test.game, 15);

        test.game.set_block_at(
            &mut test.world,
            BlockPosition::new(0, 1, 0),
            BlockId::redstone_wire(),
            BlockUpdateCause::Unknown,
        );

        assert_power_2d(0, 0, &test.game, 12);
        assert_power_2d(1, 0, &test.game, 13);
        assert_power_2d(2, 0, &test.game, 14);
        assert_power_2d(3, 0, &test.game, 15);
    }

    #[test]
    fn test_redstone_square() {
        let mut test = Test::new();

        init_blocks_2d(
            &mut test,
            vec![
                "rrrrrrrrr",
                "rrrrrrrrr",
                "rrrrrrrrr",
                "rrrrrrrrr",
                "rrrrrrrrr",
                "rrrrrrrrr",
                "rrrrrrrrr",
                "rrrrrrrrr",
                "rrrrrrrrb",
            ],
        );

        assert_power_2d(0, 0, &test.game, 0);
        assert_power_2d(1, 0, &test.game, 1);
        assert_power_2d(0, 1, &test.game, 1);

        test.game.set_block_at(
            &mut test.world,
            BlockPosition::new(8, 1, 8),
            BlockId::air(),
            BlockUpdateCause::Unknown,
        );

        assert_power_2d(0, 0, &test.game, 0);
        assert_power_2d(1, 0, &test.game, 0);
        assert_power_2d(0, 1, &test.game, 0);
    }

    #[test]
    fn test_redstone_square_multiple_sources() {
        let mut test = Test::new();

        init_blocks_2d(
            &mut test,
            vec![
                "brrrrrrrr",
                "rrrrrrrrr",
                "rrrrrrrrr",
                "rrrrrrrrr",
                "rrrrrrrrr",
                "rrrrrrrrr",
                "rrrrrrrrr",
                "rrrrrrrrr",
                "rrrrrrrrb",
            ],
        );

        assert_power_2d(4, 4, &test.game, 8);
        assert_power_2d(3, 3, &test.game, 10);

        test.game.set_block_at(
            &mut test.world,
            BlockPosition::new(0, 1, 0),
            BlockId::air(),
            BlockUpdateCause::Unknown,
        );

        assert_power_2d(4, 4, &test.game, 8);
        assert_power_2d(3, 3, &test.game, 6);
    }

    #[test]
    fn test_wire_up() {
        let mut test = Test::new();

        init_blocks(&mut test, vec![vec!["br  "], vec!["  rr"]]);

        assert_power_2d(1, 0, &test.game, 15);
        assert_power(2, 2, 0, &test.game, 14);
        assert_power(3, 2, 0, &test.game, 13);
    }

    #[test]
    fn test_blocked_wire_up() {
        let mut test = Test::new();

        init_blocks(&mut test, vec![vec!["br  "], vec!["  rr"], vec![" s  "]]);

        assert_power_2d(1, 0, &test.game, 15);
        assert_power(2, 2, 0, &test.game, 0);
        assert_power(3, 2, 0, &test.game, 0);
    }
}
