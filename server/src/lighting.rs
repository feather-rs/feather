//! Calculation of block and sky light.
//!
//! # Algorithms: block light
//! For block light calculation, we define four types of block
//! updates for which to perform lighting:
//!
//! * Creation of a light-emitting block. We simply propagate
//! the light update using flood fill.
//!
//! * Removal of a light-emitting block. We first perform flood fill
//! and set any blocks which were previously affected by this block's
//! light to 0. Then, we recalculate lighting for light sources within
//! a range of 30 blocks based on algorithm #1.
//!
//! * Creation of an opaque, non-emitting block. We first set the created
//! block to air temporarily. We then query for nearby lights
//! within a range of 15 (the maximum distance travelled by light) and perform
//! algorithm #2 on them. Finally, we set the created block back to the correct
//! value and perform algorithm #1 on all lights.
//!
//! * Removal of an opaque, non-emitting block. In this case,
//! we set the new air block's light to the highest value of an
//! adjacent block minus 1. We then perform algorithm #1 on this new block.
//!
//! Each algorithm is implemented in a separate function, and `LightingSystem`
//! determines which to use based on the values of the block update event.
//!
//! If we are recalculating light for an entire chunk, e.g. when a chunk is generated,
//! we first zero out light, then find all light sources in the chunk and perform
//! algorithm #1 on them as if they had just been placed.

use crate::blocks::BlockUpdateEvent;
use crate::chunk_logic::ChunkLoadEvent;
use crate::physics::chunks_within_distance;
use crate::systems::LIGHTING;
use arrayvec::ArrayVec;
use failure::_core::marker::PhantomData;
use feather_blocks::{Block, BlockExt};
use feather_core::prelude::ChunkMap;
use feather_core::world::chunk_relative_pos;
use feather_core::{BlockPosition, Chunk, ChunkPosition};
use hashbrown::HashSet;
use multimap::MultiMap;
use shrev::{EventChannel, ReaderId};
use smallvec::SmallVec;
use specs::{DispatcherBuilder, Read, System, Write};
use std::collections::VecDeque;

const MAX_TRAVEL_DISTANCE: u8 = 15;

/// Lighter context, used to cache things during
/// a lighting iteration.
struct Context<'a> {
    /// Reference to the current cached chunk.
    /// This is used to avoid repetitive hashmap
    /// accesses in the chunk map when groups
    /// of clustered blocks are queried for.
    current_chunk: *mut Chunk,
    /// Chunk map. Raw pointers are used to bypass the borrow
    /// checker, since `current_chunk` refers to the chunk map,
    /// which isn't allowed.
    chunk_map: *mut ChunkMap,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> Context<'a> {
    fn new(chunk_map: &'a mut ChunkMap, start_chunk: ChunkPosition) -> Option<Self> {
        let chunk_map = chunk_map as *mut ChunkMap;

        // Safety: `chunk_map` is a valid pointer
        // made from a mutable reference.
        // It has not been modified since.
        let current_chunk = unsafe { (*chunk_map).chunk_at_mut(start_chunk)? as *mut Chunk };

        Some(Self {
            current_chunk,
            chunk_map,
            _phantom: PhantomData,
        })
    }

    fn chunk_at_mut(&mut self, pos: ChunkPosition) -> Option<&'a mut Chunk> {
        if pos == (unsafe { &*self.current_chunk }).position() {
            Some(unsafe { &mut *self.current_chunk })
        } else {
            // Safety: While `self.current_chunk` refers to the chunk map,
            // it is never accessed between mutations of the chunk
            // map itself, since `Context` holds a unique reference to the
            // map and never mutates it.
            self.current_chunk = unsafe { (*self.chunk_map).chunk_at_mut(pos)? };
            Some(unsafe { &mut *self.current_chunk })
        }
    }

    fn block_light_at(&mut self, pos: BlockPosition) -> u8 {
        match self.chunk_at_mut(pos.chunk_pos()) {
            Some(chunk) => {
                let (x, y, z) = chunk_relative_pos(pos);
                chunk.block_light_at(x, y, z)
            }
            None => 0,
        }
    }

    fn set_block_light_at(&mut self, pos: BlockPosition, value: u8) {
        if let Some(chunk) = self.chunk_at_mut(pos.chunk_pos()) {
            let (x, y, z) = chunk_relative_pos(pos);
            chunk.set_block_light_at(x, y, z, value);
        }
    }

    fn block_at(&mut self, pos: BlockPosition) -> Block {
        match self.chunk_at_mut(pos.chunk_pos()) {
            Some(chunk) => {
                let (x, y, z) = chunk_relative_pos(pos);
                chunk.block_at(x, y, z)
            }
            None => Block::Air,
        }
    }

    fn set_block_at(&mut self, pos: BlockPosition, block: Block) {
        if let Some(chunk) = self.chunk_at_mut(pos.chunk_pos()) {
            let (x, y, z) = chunk_relative_pos(pos);
            chunk.set_block_at(x, y, z, block);
        }
    }
}

/// Contains a map storing light sources for each chunk.
/// This is used to accelerate light calculation.
#[derive(Default)]
pub struct ChunkLights(MultiMap<ChunkPosition, BlockPosition>);

impl ChunkLights {
    fn lights_within_distance(&self, pos: BlockPosition, dist: u8) -> SmallVec<[BlockPosition; 9]> {
        let dist_f64 = f64::from(dist);
        let chunks =
            chunks_within_distance(pos.world_pos(), glm::vec3(dist_f64, dist_f64, dist_f64));

        chunks
            .into_iter()
            .map(|chunk| {
                self.0
                    .get_vec(&chunk)
                    .map(|vec| vec.as_slice())
                    .unwrap_or(&[])
                    .iter()
            })
            .flatten()
            .copied()
            .collect()
    }
}

/// System for handling all lighting tasks.
#[derive(Default)]
pub struct LightingSystem {
    update_reader: Option<ReaderId<BlockUpdateEvent>>,
    load_reader: Option<ReaderId<ChunkLoadEvent>>,
}

impl<'a> System<'a> for LightingSystem {
    type SystemData = (
        Write<'a, ChunkMap>,
        Write<'a, ChunkLights>,
        Read<'a, EventChannel<ChunkLoadEvent>>,
        Read<'a, EventChannel<BlockUpdateEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut chunk_map, mut chunk_lights, load_events, update_events) = data;

        // Update `ChunkLights` with newly loaded chunks
        for load in load_events.read(self.load_reader.as_mut().unwrap()) {
            // Find all lights within this chunk.
            if let Some(chunk) = chunk_map.chunk_at(load.pos) {
                let lights = find_lights_in_chunk(chunk);
                lights
                    .into_iter()
                    .for_each(|light| chunk_lights.0.insert(load.pos, light));
            }
        }

        // Perform lighting updates.
        for event in update_events.read(self.update_reader.as_mut().unwrap()) {
            let mut ctx = match Context::new(&mut chunk_map, event.pos.chunk_pos()) {
                Some(ctx) => ctx,
                None => continue, // Unloaded chunk
            };

            // Determine which algorithm to use.
            if event.old_block.light_emission() < event.new_block.light_emission() {
                ctx.set_block_light_at(event.pos, event.new_block.light_emission());
                emitting_creation(&mut ctx, event.pos);
            } else if event.new_block.light_emission() == 0 && event.old_block.light_emission() > 0
            {
                ctx.set_block_light_at(event.pos, 0);
                emitting_removal(&mut ctx, &chunk_lights, event.pos, event.old_block);
            } else if event.old_block.is_opaque() && !event.new_block.is_opaque() {
                opaque_non_emitting_removal(&mut ctx, event.pos);
            } else {
                opaque_non_emitting_creation(&mut ctx, &chunk_lights, event.pos, event.new_block);
            }

            // Update `ChunkLights`.
            if event.old_block.light_emission() != event.new_block.light_emission() {
                if event.new_block.light_emission() == 0 {
                    chunk_lights
                        .0
                        .get_vec_mut(&event.pos.chunk_pos())
                        .unwrap()
                        .retain(|pos| *pos != event.pos);
                } else if event.old_block.light_emission() == 0 {
                    chunk_lights.0.insert(event.pos.chunk_pos(), event.pos);
                }
            }
        }
    }

    setup_impl!(update_reader, load_reader);
}

pub fn init_logic(dispatcher: &mut DispatcherBuilder) {
    dispatcher.add(LightingSystem::default(), LIGHTING, &[]);
}

fn find_lights_in_chunk(chunk: &Chunk) -> Vec<BlockPosition> {
    let mut res = vec![];

    for x in 0..16 {
        for y in 0..256 {
            for z in 0..16 {
                let block = chunk.block_at(x, y, z);

                let emission = block.light_emission();
                if emission > 0 {
                    res.push(BlockPosition::new(x as i32, y as i32, z as i32));
                }
            }
        }
    }

    res
}

/// Algorithm #1, as described in the module-level docs.
fn emitting_creation(context: &mut Context, position: BlockPosition) {
    let emission = context.block_light_at(position);
    // Perform flood fill starting from `position`.
    // For each block, set the light value to the maximum light
    // value of any adjacent block minus 1.
    flood_fill(context, position, emission, |ctx, pos| {
        let light = light_value_for_block(ctx, pos);
        ctx.set_block_light_at(pos, light);
    });
}

/// Algorithm #2, as described in the module-level docs.
fn emitting_removal(
    context: &mut Context,
    chunk_lights: &ChunkLights,
    position: BlockPosition,
    old_block: Block,
) {
    // Perform flood fill and set all blocks affected by the old light to 0 light.
    flood_fill(context, position, old_block.light_emission(), |ctx, pos| {
        ctx.set_block_light_at(pos, 0);
    });

    // For all lights which could have affected the blocks we just set to 0,
    // recalculate lighting using algorithm #1.
    let nearby_lights = chunk_lights.lights_within_distance(position, MAX_TRAVEL_DISTANCE * 2);

    nearby_lights.into_iter().for_each(|light| {
        if light != position {
            emitting_creation(context, light);
        }
    });
}

/// Algorithm #3, as described in the module-level docs.
fn opaque_non_emitting_creation(
    context: &mut Context,
    chunk_lights: &ChunkLights,
    position: BlockPosition,
    new_block: Block,
) {
    // Re-calculate all lights that could have affected this block.
    // We ensure that all areas are correctly set to dark by first
    // faking that the block was never created.
    context.set_block_at(position, Block::Air);

    let nearby_lights = chunk_lights.lights_within_distance(position, MAX_TRAVEL_DISTANCE);

    nearby_lights.iter().for_each(|light| {
        let block = context.block_at(*light);
        emitting_removal(context, chunk_lights, *light, block);
    });

    // Set block back to correct value.
    context.set_block_at(position, new_block);

    // Recalculate nearby lights.
    nearby_lights.iter().for_each(|light| {
        emitting_creation(context, *light);
    });
}

/// Algorithm #4, as described in the module-level docs.
fn opaque_non_emitting_removal(context: &mut Context, position: BlockPosition) {
    let value = light_value_for_block(context, position);

    context.set_block_light_at(position, value);

    // Propagate new light value for this block, as if it were a new light source.
    if value > 0 {
        emitting_creation(context, position);
    }
}

/// Returns the light value for the block at `position`,
/// equivalent to the maximum light value of an adjacent block
/// minus 1.
fn light_value_for_block(context: &mut Context, position: BlockPosition) -> u8 {
    // Find highest light value of 6 adjacent blocks.
    let adjacent = adjacent_blocks(position);
    let mut value = adjacent
        .into_iter()
        .map(|pos| context.block_light_at(pos))
        .max()
        .unwrap();

    if value > 0 {
        value -= 1;
    }

    value
}

/// Performs flood fill starting at `start` and travelling up
/// to `max_dist` blocks.
///
/// For each block iterated over, the provided closure will be invoked.
/// No block will be iterated more than once.
fn flood_fill<F>(context: &mut Context, start: BlockPosition, max_dist: u8, mut func: F)
where
    F: FnMut(&mut Context, BlockPosition),
{
    // Don't iterate over same block more than once
    let mut touched = HashSet::with_capacity(64);
    touched.insert(start);

    // We use a queue-based algorithm rather than a recursive
    // one.
    let mut queue = VecDeque::with_capacity(64);

    queue.push_back(start);

    let mut finished = false;

    while let Some(pos) = queue.pop_front() {
        if finished {
            break;
        }

        let blocks = adjacent_blocks(pos);

        blocks.into_iter().for_each(|pos| {
            if pos.manhattan_distance(start) > max_dist as i32 {
                // Finished
                finished = true;
                return;
            }

            // Skip if we already went over this block
            if !touched.insert(pos) {
                return;
            }

            let block = context.block_at(pos);
            if block.is_opaque() {
                return; // Stop iterating
            }

            // Call closure
            func(context, pos);

            // Add block to queue
            queue.push_back(pos);
        });
    }
}

/// Returns the up to six adjacent blocks to a given block position.
fn adjacent_blocks(to: BlockPosition) -> ArrayVec<[BlockPosition; 6]> {
    let offsets = [
        (-1, 0, 0),
        (1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ];
    offsets
        .iter()
        .map(|(x, y, z)| BlockPosition::new(to.x + *x, to.y + *y, to.z + *z))
        .filter(|pos| pos.y >= 0 && pos.y <= 256)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context() {
        let mut chunk_map = ChunkMap::new();

        let pos = ChunkPosition::new(0, 0);
        chunk_map.set_chunk_at(pos, Chunk::new(pos));
        let pos2 = ChunkPosition::new(0, 1);
        chunk_map.set_chunk_at(pos2, Chunk::new(pos2));

        let mut ctx = Context::new(&mut chunk_map, pos).unwrap();

        assert_eq!(ctx.chunk_at_mut(pos).unwrap().position(), pos);
        assert_eq!(ctx.chunk_at_mut(pos2).unwrap().position(), pos2);
        assert_eq!(ctx.chunk_at_mut(pos).unwrap().position(), pos);
    }

    #[test]
    fn test_emitting_creation() {
        let mut chunk_map = chunk_map();
        let mut ctx = Context::new(&mut chunk_map, ChunkPosition::new(0, 0)).unwrap();

        let pos = BlockPosition::new(0, 100, 0);
        ctx.set_block_at(pos, Block::Glowstone);
        ctx.set_block_light_at(pos, Block::Glowstone.light_emission());

        emitting_creation(&mut ctx, pos);

        assert_eq!(ctx.block_light_at(BlockPosition::new(0, 99, 0)), 14);
        assert_eq!(ctx.block_light_at(BlockPosition::new(0, 99, 1)), 13);
    }

    #[test]
    fn test_opaque_non_emitting_removal() {
        let mut chunk_map = chunk_map();
        let mut ctx = Context::new(&mut chunk_map, ChunkPosition::new(0, 0)).unwrap();

        ctx.set_block_light_at(BlockPosition::new(0, 0, 0), 10);
        ctx.set_block_light_at(BlockPosition::new(0, 2, 0), 9);
        ctx.set_block_light_at(BlockPosition::new(1, 1, 0), 8);
        ctx.set_block_light_at(BlockPosition::new(-1, 1, 0), 11);
        ctx.set_block_light_at(BlockPosition::new(0, 1, 1), 0);
        ctx.set_block_light_at(BlockPosition::new(0, 1, -1), 12);
        ctx.set_block_light_at(BlockPosition::new(0, 1, 0), 15);

        opaque_non_emitting_removal(&mut ctx, BlockPosition::new(0, 1, 0));

        assert_eq!(ctx.block_light_at(BlockPosition::new(0, 1, 0)), 11);
        assert_eq!(ctx.block_light_at(BlockPosition::new(0, 1, 1)), 10);
        assert_eq!(ctx.block_light_at(BlockPosition::new(0, 1, 2)), 9);
        assert_eq!(ctx.block_light_at(BlockPosition::new(0, 1, 3)), 8);
        assert_eq!(ctx.block_light_at(BlockPosition::new(0, 1, 4)), 7);
        // ...
    }

    #[test]
    fn test_flood_fill() {
        let mut chunk_map = chunk_map();
        let mut ctx = Context::new(&mut chunk_map, ChunkPosition::new(0, 0)).unwrap();

        let mut count = 0;

        flood_fill(&mut ctx, BlockPosition::new(100, 100, 100), 1, |_, _| {
            count += 1
        });

        assert_eq!(count, 6);
    }

    #[test]
    fn test_chunk_lights() {
        let mut chunk_lights = ChunkLights::default();
        chunk_lights
            .0
            .insert(ChunkPosition::new(0, 0), BlockPosition::new(0, 0, 0));
        chunk_lights
            .0
            .insert(ChunkPosition::new(1, 0), BlockPosition::new(16, 0, 0));

        assert_eq!(
            chunk_lights
                .lights_within_distance(BlockPosition::new(0, 0, 0), 16)
                .as_slice(),
            &[BlockPosition::new(0, 0, 0), BlockPosition::new(16, 0, 0)]
        );
    }

    fn chunk_map() -> ChunkMap {
        let mut chunk_map = ChunkMap::new();

        for x in -1..=1 {
            for z in -1..=1 {
                let pos = ChunkPosition::new(x, z);
                chunk_map.set_chunk_at(pos, Chunk::new(pos));
            }
        }

        chunk_map
    }
}
