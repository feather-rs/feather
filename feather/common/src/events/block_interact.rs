use libcraft_core::{BlockFace, BlockPosition, Hand, Vec3f};

pub struct BlockInteractEvent {
    pub hand: Hand,
    pub location: BlockPosition,
    pub face: BlockFace,
    pub cursor_position: Vec3f,
    /// If the client thinks its inside a block when the interaction is fired.
    pub inside_block: bool,
}

pub struct BlockPlacementEvent {
    pub hand: Hand,
    pub location: BlockPosition,
    pub face: BlockFace,
    pub cursor_position: Vec3f,
    /// If the client thinks its inside a block when the interaction is fired.
    pub inside_block: bool,
}
