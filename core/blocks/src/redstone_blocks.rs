use crate::{BlockId, BlockKind, EastWire, FacingCubic, NorthWire, SouthWire, WestWire};

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

    /// If a block emits strong redstone power in one direction, it also has to emit weak power there
    pub fn strong_redstone_power(&self, direction: FacingCubic) -> Option<i32> {
        match self.kind() {
            BlockKind::RedstoneWire => {
                match (
                    direction,
                    self.north_wire().unwrap(),
                    self.east_wire().unwrap(),
                    self.south_wire().unwrap(),
                    self.west_wire().unwrap(),
                ) {
                    (
                        direction,
                        NorthWire::None,
                        EastWire::None,
                        SouthWire::None,
                        WestWire::None,
                    ) if direction != FacingCubic::Up => Some(15),
                    (FacingCubic::North, _, _, SouthWire::Side, _)
                    | (FacingCubic::East, _, _, _, WestWire::Side)
                    | (FacingCubic::South, NorthWire::Side, _, _, _)
                    | (FacingCubic::West, _, EastWire::Side, _, _)
                    | (FacingCubic::Down, _, _, _, _) => Some(15),
                    _ => None,
                }
            }
            BlockKind::RedstoneTorch | BlockKind::RedstoneWallTorch => {
                if self.powered().unwrap() && direction == FacingCubic::Up {
                    Some(15)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
