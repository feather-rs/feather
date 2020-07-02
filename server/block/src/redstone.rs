use feather_core::{
    blocks::{BlockId, BlockKind, EastWire, FacingCubic, NorthWire, SouthWire, WestWire},
    util::BlockPosition,
};
use feather_server_types::{BlockUpdateCause, BlockUpdateEvent, Game};
use fecs::World;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::{From, Into};
use std::iter::IntoIterator;
use std::rc::Rc;

/// Checks if the block update invalidates the current redstone state
/// If the case, recalculates the redstone behavior
#[fecs::event_handler]
pub fn on_block_update_redstone(event: &BlockUpdateEvent, game: &mut Game, world: &mut World) {
    // If this update occurs because of a redstone update, ignore this
    if event.cause == BlockUpdateCause::Redstone {
        return;
    }
    let check_positions = vec![
        event.pos,
        // One up and one down
        event.pos + BlockPosition::new(0, 1, 0),
        event.pos + BlockPosition::new(0, -1, 0),
        // All four directions
        event.pos + BlockPosition::new(0, 0, -1),
        event.pos + BlockPosition::new(1, 0, 0),
        event.pos + BlockPosition::new(0, 0, 1),
        event.pos + BlockPosition::new(-1, 0, 0),
        // All four directions + one up
        event.pos + BlockPosition::new(0, 1, -1),
        event.pos + BlockPosition::new(1, 1, 0),
        event.pos + BlockPosition::new(0, 1, 1),
        event.pos + BlockPosition::new(-1, 1, 0),
        // All four directions + one down
        event.pos + BlockPosition::new(0, -1, -1),
        event.pos + BlockPosition::new(1, -1, 0),
        event.pos + BlockPosition::new(0, -1, 1),
        event.pos + BlockPosition::new(-1, -1, 0),
    ]
    .into_iter();

    let mut cache = RedstoneCache::new();

    // Keep track of the redstone blocks whoose power has been increased and decreased
    let mut rising_blocks = VecDeque::with_capacity(8);
    let mut falling_blocks = Vec::with_capacity(8);

    // check for the state of the redstone and correct it if neccessary
    check_positions.for_each(|block_pos| {
        let block = cache.block_at(block_pos, game);
        if let RedstoneBlock::RedstoneWire(current_state) = &block.redstone_kind {
            // Looks at the surrounding blocks and calculates what the state should be
            let correct_state = RedstoneState::calculate(block_pos, &mut cache, game);

            match correct_state.power.cmp(&current_state.power) {
                Ordering::Greater => rising_blocks.push_back(block_pos),
                Ordering::Less => falling_blocks.push((current_state.power, block_pos)),
                Ordering::Equal => {}
            }

            if correct_state != *current_state {
                cache.update_block(block_pos, &correct_state, block.block_id);
            }
        }
    });

    // First handle blocks that loose power because of the update
    let new_rising_blocks = update_falling_blocks(falling_blocks, &mut cache, game);

    // Then handle rising blocks
    update_rising_blocks(rising_blocks, &mut cache, game);

    // Last, update those blocks that `update_falling_blocks` returned as rising blocks
    if !new_rising_blocks.is_empty() {
        for block in new_rising_blocks {
            let mut arg = VecDeque::new();
            arg.push_back(block);
            update_rising_blocks(arg, &mut cache, game);
        }
    }

    // And set all changed blocks from the cache in the actual world
    for (pos, block) in cache {
        game.set_block_at(world, pos, block.block_id, BlockUpdateCause::Redstone);
    }
}

/// Calculates increasing redstone power levels
/// I handle this differently for rising and falling power, because calculating the falling power levels
/// is more complicated than calculating the rising power levels
fn update_rising_blocks(
    mut rising_blocks: VecDeque<BlockPosition>,
    cache: &mut RedstoneCache,
    game: &mut Game,
) {
    let mut updated_blocks = HashSet::new();

    // For each rising block check its adjacent redstone wires and if their power should increase add them to the vec aswell
    while let Some(block_pos) = rising_blocks.pop_front() {
        updated_blocks.insert(block_pos);

        for (block, pos, _, _) in cache.get_redstone_connections(block_pos, game) {
            if let RedstoneBlock::RedstoneWire(old_block_state) = &block.redstone_kind {
                if updated_blocks.contains(&pos) {
                    continue;
                }
                let block_state = RedstoneState::calculate(pos, cache, game);

                // println!("Block pos: {:?}", pos);
                // println!("new state: {:?}", block_state);

                if block_state.power > old_block_state.power {
                    cache.update_block(pos, &block_state, block.block_id);

                    rising_blocks.push_back(pos);
                }
            }
        }
    }
}

/// From the original falling redstone wires, check the surrounding wires
/// if the surrounding wire has a lower power than the original power, set it to 0 and add the block to the falling blocks
fn update_falling_blocks(
    mut falling_blocks: Vec<(i32, BlockPosition)>,
    cache: &mut RedstoneCache,
    game: &mut Game,
) -> VecDeque<BlockPosition> {
    // Keeps the blocks that were already updated
    let mut updated_blocks = HashSet::new();

    // Includes the blocks that are now at the border between power and no power
    // These must be updated again to prevent fragments
    let mut new_rising_blocks = VecDeque::new();

    while let Some((original_power, block_pos)) = falling_blocks.pop() {
        updated_blocks.insert(block_pos);
        // set the power level of this block to zero
        let block = cache.block_at(block_pos, game);
        if let RedstoneBlock::RedstoneWire(mut state) = block.redstone_kind.clone() {
            state.power = 0;
            cache.update_block(block_pos, &state, block.block_id);
        } else {
            panic!(
                "Unexpected block: Expected redstone wire but got, {:?}",
                block.redstone_kind
            );
        }

        for (block, pos, _, _) in cache.get_redstone_connections(block_pos, game) {
            if let RedstoneBlock::RedstoneWire(state) = &block.redstone_kind {
                // if the wire block is already detected, ignore it
                if updated_blocks.contains(&pos)
                    || falling_blocks
                        .iter()
                        .any(|(_, other_pos)| pos == *other_pos)
                {
                    continue;
                }

                if state.power < original_power {
                    falling_blocks.push((original_power - 1, pos));
                } else {
                    // mark this block as rising so that in the next step the zeroed values can get calculated
                    if !new_rising_blocks.contains(&pos) {
                        new_rising_blocks.push_back(pos);
                    }
                }
            }
        }
    }
    new_rising_blocks
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct RedstoneState {
    north: NorthWire,
    east: EastWire,
    south: SouthWire,
    west: WestWire,
    power: i32,
}

impl RedstoneState {
    /// calculates the state of the redstone wire that would be at this position
    /// Has to check in all four directions one block down and one block up
    /// redstone at same y: side, redstone at y-1: side, redstone at y+1: up
    pub fn calculate(pos: BlockPosition, cache: &mut RedstoneCache, game: &Game) -> Self {
        // println!("Calculating for block at {:?}", pos);
        // println!("{:?}", get_redstone_connections(pos, game));
        let mut north = NorthWire::None;
        let mut east = EastWire::None;
        let mut south = SouthWire::None;
        let mut west = WestWire::None;

        // always choose the maximum available power level
        let mut power = 0;

        for (block, _, direction, vertical_offset) in cache.get_redstone_connections(pos, game) {
            let block_weak_power = block.block_id.weak_redstone_power(direction.opposite());
            if let Some(block_weak_power) = block_weak_power {
                if block_weak_power > power {
                    power = block_weak_power;
                }
            }

            // println!(
            //     "Block: {:?} - power: {:?} - position: {:?}",
            //     block, block_weak_power, block_pos
            // );

            // If a redstone wire exist one block above and one block below for a direction,
            // The Wire::Side state is preferred (because of the order of get_redstone_connections)
            match direction {
                FacingCubic::North => {
                    north = match vertical_offset {
                        VerticalOffset::None | VerticalOffset::Below => NorthWire::Side,
                        VerticalOffset::Above => NorthWire::Up,
                    }
                }
                FacingCubic::East => {
                    east = match vertical_offset {
                        VerticalOffset::None | VerticalOffset::Below => EastWire::Side,
                        VerticalOffset::Above => EastWire::Up,
                    }
                }
                FacingCubic::South => {
                    south = match vertical_offset {
                        VerticalOffset::None | VerticalOffset::Below => SouthWire::Side,
                        VerticalOffset::Above => SouthWire::Up,
                    }
                }
                FacingCubic::West => {
                    west = match vertical_offset {
                        VerticalOffset::None | VerticalOffset::Below => WestWire::Side,
                        VerticalOffset::Above => WestWire::Up,
                    }
                }
                _ => (),
            }
        }
        // println!("Actual values: power - {:?}", power);
        Self {
            north,
            east,
            south,
            west,
            power,
        }
    }

    pub fn from_block(block: BlockId) -> Option<Self> {
        if block.kind() != BlockKind::RedstoneWire {
            return None;
        }

        Some(Self {
            north: block.north_wire().unwrap(),
            east: block.east_wire().unwrap(),
            south: block.south_wire().unwrap(),
            west: block.west_wire().unwrap(),
            power: block.power().unwrap(),
        })
    }

    /// Sets the redstone wire to this state
    pub fn apply_to(&self, block: &mut BlockId) {
        block.set_north_wire(self.north);
        block.set_east_wire(self.east);
        block.set_south_wire(self.south);
        block.set_west_wire(self.west);
        block.set_power(self.power);
    }
}

#[derive(Debug)]
pub enum VerticalOffset {
    None,
    Below,
    Above,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum RedstoneBlock {
    RedstoneWire(RedstoneState),
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
                RedstoneState::from_block(block).expect("This block is a redstone wire"),
            ),

            _ if block.is_full_block() => RedstoneBlock::FullBlock,
            _ => RedstoneBlock::None,
        }
    }
}

#[derive(Debug)]
pub struct CachedRedstoneBlock {
    block_id: BlockId,
    redstone_kind: RedstoneBlock,
}

#[derive(Debug, Default)]
pub struct RedstoneCache {
    cache: HashMap<BlockPosition, Rc<CachedRedstoneBlock>>,
}

impl RedstoneCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn block_at(&mut self, pos: BlockPosition, game: &Game) -> Rc<CachedRedstoneBlock> {
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

        self.cache.get(&pos).map(|block| block.clone()).unwrap()
    }

    pub fn update_block(&mut self, pos: BlockPosition, state: &RedstoneState, mut block: BlockId) {
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
            if block
                .block_id
                .weak_redstone_power(direction.opposite())
                .is_some()
            {
                connected_redstone_components.push((
                    block,
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
                let up_block = self.block_at(up, game);
                if up_block.block_id.kind() == BlockKind::RedstoneWire
                    && !self
                        .block_at(block_pos_up, game)
                        .redstone_kind
                        .is_full_block()
                {
                    connected_redstone_components.push((
                        up_block,
                        up,
                        direction,
                        VerticalOffset::Above,
                    ));
                }

                let down_block = self.block_at(down, game);
                // Condition: Connect to a down block if not solid block is above that down block
                if down_block.block_id.kind() == BlockKind::RedstoneWire
                    && !block.redstone_kind.is_full_block()
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

        // last, check for the block directly above/below
        let block_up = self.block_at(block_pos_up, game);
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

        let block_down = self.block_at(block_pos_down, game);
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
    use super::*;
    use feather_test_framework::Test;

    fn init_blocks(test: &mut Test, blocks: Vec<&str>) {
        for (z, block_row) in blocks.into_iter().enumerate() {
            for (x, block) in block_row.chars().enumerate() {
                let block_id = match block {
                    'r' => BlockId::redstone_wire(),
                    'b' => BlockId::redstone_block(),
                    ' ' => continue,
                    _ => panic!("Invalid block specifier: {}!", block),
                };
                test.game.set_block_at(
                    &mut test.world,
                    BlockPosition::new(x as i32, 0, z as i32),
                    BlockId::stone(),
                    BlockUpdateCause::Unknown,
                );
                let pos = BlockPosition::new(x as i32, 1, z as i32);
                test.game
                    .set_block_at(&mut test.world, pos, block_id, BlockUpdateCause::Unknown);
            }
        }
    }

    fn assert_power(x: i32, z: i32, game: &Game, power: i32) {
        assert_eq!(
            game.block_at(BlockPosition::new(x, 1, z))
                .unwrap()
                .power()
                .unwrap(),
            power
        );
    }

    #[test]
    fn test_power_redstone() {
        let mut test = Test::new();

        init_blocks(&mut test, vec!["rb"]);
        assert_power(0, 0, &test.game, 15);
    }

    #[test]
    fn test_power_redstone_range() {
        let mut test = Test::new();

        init_blocks(&mut test, vec!["rrrrrrrrrrrrrrrrb"]);
        assert_power(0, 0, &test.game, 0);
        assert_power(1, 0, &test.game, 1);
    }

    #[test]
    fn test_power_falling() {
        let mut test = Test::new();

        init_blocks(&mut test, vec!["r ", "rb"]);
        assert_power(0, 0, &test.game, 14);

        test.game.set_block_at(
            &mut test.world,
            BlockPosition::new(1, 1, 1),
            BlockId::air(),
            BlockUpdateCause::Unknown,
        );

        assert_power(0, 0, &test.game, 0);
    }

    #[test]
    fn test_multiple_power_sources() {
        let mut test = Test::new();

        init_blocks(&mut test, vec!["brrrb"]);

        assert_power(1, 0, &test.game, 15);
        assert_power(2, 0, &test.game, 14);
        assert_power(3, 0, &test.game, 15);

        test.game.set_block_at(
            &mut test.world,
            BlockPosition::new(0, 1, 0),
            BlockId::redstone_wire(),
            BlockUpdateCause::Unknown,
        );

        assert_power(0, 0, &test.game, 12);
        assert_power(1, 0, &test.game, 13);
        assert_power(2, 0, &test.game, 14);
        assert_power(3, 0, &test.game, 15);
    }

    #[test]
    fn test_redstone_square() {
        let mut test = Test::new();

        init_blocks(
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

        assert_power(0, 0, &test.game, 0);
        assert_power(1, 0, &test.game, 1);
        assert_power(0, 1, &test.game, 1);

        test.game.set_block_at(
            &mut test.world,
            BlockPosition::new(8, 1, 8),
            BlockId::air(),
            BlockUpdateCause::Unknown,
        );

        assert_power(0, 0, &test.game, 0);
        assert_power(1, 0, &test.game, 0);
        assert_power(0, 1, &test.game, 0);
    }

    #[test]
    fn test_redstone_square_multiple_sources() {
        let mut test = Test::new();

        init_blocks(
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

        assert_power(4, 4, &test.game, 8);
        assert_power(3, 3, &test.game, 10);

        test.game.set_block_at(
            &mut test.world,
            BlockPosition::new(0, 1, 0),
            BlockId::air(),
            BlockUpdateCause::Unknown,
        );

        assert_power(4, 4, &test.game, 8);
        assert_power(3, 3, &test.game, 6);
    }
}
