use crate::{BlockId, BlockKind, FacingCubic};

impl BlockId {
    pub fn weak_redstone_power(&self, _direction: FacingCubic) -> Option<i32> {
        match self.kind() {
            BlockKind::RedstoneBlock | BlockKind::RedstoneTorch | BlockKind::RedstoneWallTorch => {
                Some(15)
            }
            BlockKind::RedstoneWire => self.power().map(|power| (power - 1).max(0)),
            _ => None,
        }
    }
}
