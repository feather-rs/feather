use libcraft_core::{BlockPosition, InteractBlockFace, InteractHand, Vec3f};

pub struct BlockPlacementEvent {
    pub hand: InteractHand,
    pub location: BlockPosition,
    pub face: InteractBlockFace,
    pub cursor_position: Vec3f,
    pub inside_block: bool,
}
