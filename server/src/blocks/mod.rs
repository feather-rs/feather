mod falling;
mod supported;

pub use falling::FallingBlockCreationSystem;

use shrev::{EventChannel, ReaderId};
use specs::{DispatcherBuilder, Entity, Read, System, Write};

use feather_blocks::{Block, BlockExt};
use feather_core::world::{BlockPosition, ChunkMap};

use crate::blocks::supported::SupportBlockBreakSystem;
use crate::systems::{BLOCK_FALLING_CREATION, BLOCK_SUPPORTED_BREAK, BLOCK_UPDATE_PROPAGATE};
use hashbrown::HashSet;

lazy_static! {
    /// List of block types that need to be notified
    /// of adjacent block updates.
    static ref BLOCKS_TO_NOTIFY: HashSet<Block> = {
        let mut set = HashSet::new();
        // Falling blocks
        set.insert(Block::Sand);
        set.insert(Block::RedSand);
        set.insert(Block::Gravel);
        set
    };
}

/// Event triggered when a block is updated.
///
/// This event is triggered *after* the block is updated
/// in the chunk map.
#[derive(Debug, Clone)]
pub struct BlockUpdateEvent {
    /// The cause of this block update event.
    pub cause: BlockUpdateCause,
    /// The location of the block which was updated.
    pub pos: BlockPosition,
    /// The block which was previously at the position.
    pub old_block: Block,
    /// The new block at the position.
    pub new_block: Block,
}

/// The possible causes of a block update event.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BlockUpdateCause {
    /// Indicates that a player updated the block.
    Player(Entity),
    /// Indicates that a falling block updated the block.
    FallingBlock,
    /// Indicates that a block was broken because the support
    /// underneath was destroyed.
    SupportDestroyed,
    /// A test block update caused, used for unit testing.
    Test,
}

#[derive(Debug, Clone)]
pub struct BlockNotifyEvent {
    pub block: Block,
    pub pos: BlockPosition,
    pub notified_by: BlockPosition,
}

/// System for propagating block update
/// events to surrounding blocks.
///
/// This system listens to `BlockUpdateEvent`s.
#[derive(Default)]
pub struct BlockUpdatePropagateSystem {
    reader: Option<ReaderId<BlockUpdateEvent>>,
}

impl<'a> System<'a> for BlockUpdatePropagateSystem {
    type SystemData = (
        Read<'a, EventChannel<BlockUpdateEvent>>,
        Read<'a, ChunkMap>,
        Write<'a, EventChannel<BlockNotifyEvent>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (events, chunk_map, mut notify) = data;

        // Process events
        for event in events.read(&mut self.reader.as_mut().unwrap()) {
            let mut notify_events: Vec<BlockNotifyEvent> = Vec::new();

            for x in -1..=1 {
                for y in -1..=1 {
                    for z in -1..=1 {
                        let mut adjacent = event.pos;
                        adjacent.x += x;
                        adjacent.y += y;
                        adjacent.z += z;
                        let block = chunk_map.block_at(adjacent);
                        if let Some(block) = block {
                            // TODO: clean up block.is_solid() check, which is
                            // needed for supported block destruction
                            if BLOCKS_TO_NOTIFY.contains(&block)
                                || (block != Block::Air && !block.is_solid())
                            {
                                notify_events.push(BlockNotifyEvent {
                                    block,
                                    pos: adjacent,
                                    notified_by: event.pos,
                                });
                            }
                        }
                    }
                }
            }

            // Notify all adjacent blocks at once
            notify.drain_vec_write(&mut notify_events);
        }
    }

    setup_impl!(reader);
}

pub fn init_logic(_dispatcher: &mut DispatcherBuilder) {
    // TODO
}

pub fn init_handlers(dispatcher: &mut DispatcherBuilder) {
    dispatcher.add(
        BlockUpdatePropagateSystem::default(),
        BLOCK_UPDATE_PROPAGATE,
        &[],
    );
    dispatcher.add(
        FallingBlockCreationSystem::default(),
        BLOCK_FALLING_CREATION,
        &[BLOCK_UPDATE_PROPAGATE],
    );
    dispatcher.add(
        SupportBlockBreakSystem::default(),
        BLOCK_SUPPORTED_BREAK,
        &[BLOCK_UPDATE_PROPAGATE],
    );
}
