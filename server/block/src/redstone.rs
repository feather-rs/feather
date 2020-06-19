use feather_core::{
    blocks::{BlockId, BlockKind, EastWire, FacingCubic, NorthWire, SouthWire, WestWire},
    util::BlockPosition,
};
use feather_server_types::{BlockUpdateCause, BlockUpdateEvent, Game};
use fecs::World;
use std::collections::{HashSet, VecDeque};

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

    // Keep track of the redstone blocks whoose power has been increased and decreased
    let mut rising_blocks = VecDeque::with_capacity(8);
    let mut falling_blocks = Vec::with_capacity(8);

    // check for the state of the redstone and correct it if neccessary
    check_positions.for_each(|block_pos| {
        if let Some(mut block) = game.block_at(block_pos) {
            if block.kind() == BlockKind::RedstoneWire {
                // Looks at the surrounding blocks and calculates what the state should be
                let correct_state = RedstoneState::calculate(block_pos, game);
                // returns what the current blockstate is
                let current_state = RedstoneState::from_block(block).unwrap();

                if correct_state != current_state {
                    correct_state.apply_to(&mut block);

                    game.set_block_at(world, block_pos, block, BlockUpdateCause::Redstone);
                }

                if correct_state.power > current_state.power {
                    rising_blocks.push_back(block_pos);
                } else if correct_state.power < current_state.power {
                    // For more efficiency, store the original powerlevel too
                    falling_blocks.push((current_state.power, block_pos));
                }
            }
        }
    });

    // // First handle blocks that loose power because of the update
    let new_rising_blocks = update_falling_blocks(falling_blocks, game, world);

    // Then handle rising blocks
    update_rising_blocks(rising_blocks, game, world);

    // Last, update those blocks that `update_falling_blocks` returned as rising blocks
    if !new_rising_blocks.is_empty() {
        for block in new_rising_blocks {
            let mut arg = VecDeque::new();
            arg.push_back(block);
            update_rising_blocks(arg, game, world);
        }
    }
}

/// Calculates increasing redstone power levels
/// I handle this differently for rising and falling power, because calculating the falling power levels
/// is more complicated than calculating the rising power levels
fn update_rising_blocks(
    mut rising_blocks: VecDeque<BlockPosition>,
    game: &mut Game,
    world: &mut World,
) {
    let mut updated_blocks = HashSet::new();

    // For each rising block check its adjacent redstone wires and if their power should increase add them to the vec aswell
    while let Some(block_pos) = rising_blocks.pop_front() {
        updated_blocks.insert(block_pos);

        for (mut block, pos, _, _) in get_redstone_connections(block_pos, game) {
            if block.kind() == BlockKind::RedstoneWire {
                if updated_blocks.contains(&pos) {
                    continue;
                }
                let block_state = RedstoneState::calculate(pos, game);
                let old_block_state = RedstoneState::from_block(block).unwrap();

                // println!("Block pos: {:?}", block_pos);
                // println!("old state: {:?}", old_block_state);
                // println!("new state: {:?}", block_state);

                if block_state.power > old_block_state.power {
                    block.set_power(block_state.power);

                    game.set_block_at(world, pos, block, BlockUpdateCause::Redstone);

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
    game: &mut Game,
    world: &mut World,
) -> VecDeque<BlockPosition> {
    // Keeps the blocks that were already updated
    let mut updated_blocks = HashSet::new();

    // Includes the blocks that are now at the border between power and no power
    // These must be updated again to prevent fragments
    let mut new_rising_blocks = VecDeque::new();

    while let Some((original_power, block_pos)) = falling_blocks.pop() {
        updated_blocks.insert(block_pos);
        // set the power level of this block to zero
        if let Some(mut block) = game.block_at(block_pos) {
            block.set_power(0);
            // Note: This step could be removed to avoid unnecessary packets
            // Then a internal or virtual state would be needed which stores the blocks power levels
            game.set_block_at(world, block_pos, block, BlockUpdateCause::Redstone);
        }

        for (block, pos, _, _) in get_redstone_connections(block_pos, game) {
            if block.kind() == BlockKind::RedstoneWire {
                // if the wire block is already detected, ignore it
                if updated_blocks.contains(&pos)
                    || falling_blocks
                        .iter()
                        .filter(|(_, other_pos)| pos == *other_pos)
                        .next()
                        .is_some()
                {
                    continue;
                }

                if let Some(power) = block.power() {
                    if power < original_power {
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
    }
    new_rising_blocks
}

#[derive(Debug, Clone)]
pub struct RedstoneState {
    north: NorthWire,
    east: EastWire,
    south: SouthWire,
    west: WestWire,
    power: i32,
}

impl PartialEq for RedstoneState {
    fn eq(&self, other: &Self) -> bool {
        other.power == self.power
            && other.north == self.north
            && other.east == self.east
            && other.south == self.south
            && other.west == self.west
    }
}

impl Eq for RedstoneState {}

impl RedstoneState {
    /// calculates the state of the redstone wire that would be at this position
    /// Has to check in all four directions one block down and one block up
    /// redstone at same y: side, redstone at y-1: side, redstone at y+1: up
    pub fn calculate(pos: BlockPosition, game: &Game) -> Self {
        let mut north = NorthWire::None;
        let mut east = EastWire::None;
        let mut south = SouthWire::None;
        let mut west = WestWire::None;

        // always choose the maximum available power level
        let mut power = 0;

        for (block, _, direction, vertical_offset) in get_redstone_connections(pos, game) {
            let block_weak_power = block.weak_redstone_power(direction.opposite());
            if let Some(block_weak_power) = block_weak_power {
                if block_weak_power > power {
                    power = block_weak_power;
                }
            }

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

pub enum VerticalOffset {
    None,
    Below,
    Above,
}

pub fn get_redstone_connections(
    pos: BlockPosition,
    game: &Game,
) -> Vec<(BlockId, BlockPosition, FacingCubic, VerticalOffset)> {
    let directions = vec![
        (pos + BlockPosition::new(0, 0, -1), FacingCubic::North),
        (pos + BlockPosition::new(1, 0, 0), FacingCubic::East),
        (pos + BlockPosition::new(0, 0, 1), FacingCubic::South),
        (pos + BlockPosition::new(-1, 0, 0), FacingCubic::West),
    ];

    let mut connected_redstone_components = Vec::with_capacity(8);

    let block_pos_up = pos + BlockPosition::new(0, 1, 0);
    let block_pos_down = pos + BlockPosition::new(0, -1, 0);
    let block_above = game.block_at(block_pos_up);
    let block_below = game.block_at(block_pos_down);

    for (position, direction) in directions {
        if let Some(block) = game.block_at(position) {
            if let Some(_) = block.weak_redstone_power(direction.opposite()) {
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

                if let Some(block) = game.block_at(up) {
                    if block.kind() == BlockKind::RedstoneWire
                        && !block_above
                            .map(|block| block.is_full_block())
                            .unwrap_or(false)
                    {
                        connected_redstone_components.push((
                            block,
                            up,
                            direction,
                            VerticalOffset::Above,
                        ));
                    }
                }

                if let Some(block) = game.block_at(down) {
                    if block.kind() == BlockKind::RedstoneWire
                        && !game
                            .block_at(down + BlockPosition::new(0, 1, 0))
                            .map(|block| block.is_full_block())
                            .unwrap_or(false)
                    {
                        connected_redstone_components.push((
                            block,
                            down,
                            direction,
                            VerticalOffset::Below,
                        ));
                    }
                }
            }
        }
    }

    // last, check for the block directly above/below

    if let Some(block_up) = block_above {
        if let Some(_) = block_up.weak_redstone_power(FacingCubic::Down) {
            connected_redstone_components.push((
                block_up,
                block_pos_up,
                FacingCubic::Up,
                VerticalOffset::Above,
            ));
        }
    }

    if let Some(block_down) = block_below {
        if let Some(_) = block_down.weak_redstone_power(FacingCubic::Up) {
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
