use crate::blocks::RedstoneState;
use crate::{RedstoneBlock, RedstoneCache};
use feather_core::util::BlockPosition;
use feather_server_types::{BlockUpdateCause, BlockUpdateEvent, Game};
use fecs::World;
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};
use std::iter::IntoIterator;

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
        if let Some(block) = cache.block_at(block_pos, game) {
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
        if let Some(block) = cache.block_at(block_pos, game) {
            if let RedstoneBlock::RedstoneWire(mut state) = block.redstone_kind.clone() {
                state.power = 0;
                cache.update_block(block_pos, &state, block.block_id);
            } else {
                panic!(
                    "Unexpected block: Expected redstone wire but got, {:?}",
                    block.redstone_kind
                );
            }
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
