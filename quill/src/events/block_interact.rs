use libcraft_core::{BlockFace, BlockPosition, Hand, Vec3f};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockInteractEvent {
    pub hand: Hand,
    pub location: BlockPosition,
    pub face: BlockFace,
    pub cursor_position: Vec3f,
    /// If the client thinks its inside a block when the interaction is fired.
    pub inside_block: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BlockPlacementEvent {
    pub hand: Hand,
    pub location: BlockPosition,
    pub face: BlockFace,
    pub cursor_position: Vec3f,
    /// If the client thinks its inside a block when the interaction is fired.
    pub inside_block: bool,
}
