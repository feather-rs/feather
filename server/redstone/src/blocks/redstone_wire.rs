use crate::{RedstoneCache, VerticalOffset};
use feather_core::{
    blocks::{BlockId, BlockKind, EastWire, FacingCubic, NorthWire, SouthWire, WestWire},
    util::BlockPosition,
};
use feather_server_types::Game;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct RedstoneWireState {
    pub north: NorthWire,
    pub east: EastWire,
    pub south: SouthWire,
    pub west: WestWire,
    pub power: i32,
}

impl RedstoneWireState {
    /// calculates the state of the redstone wire that would be at this position
    /// Has to check in all four directions one block down and one block up
    /// redstone at same y: side, redstone at y-1: side, redstone at y+1: up
    pub fn calculate(pos: BlockPosition, cache: &mut RedstoneCache, game: &Game) -> Self {
        // println!("Calculating for block at {:?}", pos);
        // println!("{:?}", get_redstone_connections(pos, game));
        let mut north = NorthWire::None;
        let mut east = EastWire::None;
        let mut south = SouthWire::None;
        let mut west = WestWire::None;

        // always choose the maximum available power level
        let mut power = 0;

        for (block, _, direction, vertical_offset) in cache.get_redstone_connections(pos, game) {
            let block_weak_power = block.block_id.weak_redstone_power(direction.opposite());
            if let Some(block_weak_power) = block_weak_power {
                if block_weak_power > power {
                    power = block_weak_power;
                }
            }

            // println!(
            //     "Block: {:?} - power: {:?} - position: {:?}",
            //     block, block_weak_power, block_pos
            // );

            // If a redstone wire exist one block above and one block below for a direction,
            // The Wire::Side state is preferred (because of the order of get_redstone_connections)
            match direction {
                FacingCubic::North => {
                    north = match vertical_offset {
                        VerticalOffset::None | VerticalOffset::Below => NorthWire::Side,
                        VerticalOffset::Above => NorthWire::Up,
                    }
                }
                FacingCubic::East => {
                    east = match vertical_offset {
                        VerticalOffset::None | VerticalOffset::Below => EastWire::Side,
                        VerticalOffset::Above => EastWire::Up,
                    }
                }
                FacingCubic::South => {
                    south = match vertical_offset {
                        VerticalOffset::None | VerticalOffset::Below => SouthWire::Side,
                        VerticalOffset::Above => SouthWire::Up,
                    }
                }
                FacingCubic::West => {
                    west = match vertical_offset {
                        VerticalOffset::None | VerticalOffset::Below => WestWire::Side,
                        VerticalOffset::Above => WestWire::Up,
                    }
                }
                _ => (),
            }
        }
        // println!("Actual values: power - {:?}", power);
        Self {
            north,
            east,
            south,
            west,
            power,
        }
    }

    pub fn from_block(block: BlockId) -> Option<Self> {
        if block.kind() != BlockKind::RedstoneWire {
            return None;
        }

        Some(Self {
            north: block.north_wire().unwrap(),
            east: block.east_wire().unwrap(),
            south: block.south_wire().unwrap(),
            west: block.west_wire().unwrap(),
            power: block.power().unwrap(),
        })
    }

    /// Sets the redstone wire to this state
    pub fn apply_to(&self, block: &mut BlockId) {
        block.set_north_wire(self.north);
        block.set_east_wire(self.east);
        block.set_south_wire(self.south);
        block.set_west_wire(self.west);
        block.set_power(self.power);
    }
}
