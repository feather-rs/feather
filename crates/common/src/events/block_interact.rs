use libcraft_core::{BlockPosition, InteractBlockFace, InteractHand, Vec3f};

pub struct BlockInteractEvent {
    pub hand: InteractHand,
    pub location: BlockPosition,
    pub face: InteractBlockFace,
    pub cursor_position: Vec3f,
    /// If the client thinks its inside a block when the interaction is fired.
    pub inside_block: bool,
    /// Should be set to `true` if this event was used to trigger a block interaction.
    pub interaction_handled: bool
}
