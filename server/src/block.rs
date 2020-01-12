use feather_core::{Block, BlockPosition};
use legion::entity::Entity;

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
}
