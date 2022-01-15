use base::{BlockId, BlockPosition, EastNlt, FacingCardinal, NorthNlt, SouthNlt, WestNlt};
use libcraft_core::BlockFace;

use crate::{block::util::AdjacentBlockHelper, World};

pub fn connect_neighbours_and_up(world: &mut World, pos: BlockPosition) -> Option<()> {
    use base::SimplifiedBlockKind::*;
    let mut block = world.block_at(pos)?;
    let east = world
        .adjacent_block(pos, BlockFace::East)
        .unwrap_or_else(BlockId::air);
    let west = world
        .adjacent_block(pos, BlockFace::West)
        .unwrap_or_else(BlockId::air);
    let north = world
        .adjacent_block(pos, BlockFace::North)
        .unwrap_or_else(BlockId::air);
    let south = world
        .adjacent_block(pos, BlockFace::South)
        .unwrap_or_else(BlockId::air);
    let mut east_connected = east.simplified_kind() == block.simplified_kind();
    let mut west_connected = west.simplified_kind() == block.simplified_kind();
    let mut north_connected = north.simplified_kind() == block.simplified_kind();
    let mut south_connected = south.simplified_kind() == block.simplified_kind();
    let is_wall = |block: BlockId| {
        matches!(
            block.simplified_kind(),
            BrickWall
                | PrismarineWall
                | RedSandstoneWall
                | MossyStoneBrickWall
                | GraniteWall
                | StoneBrickWall
                | NetherBrickWall
                | AndesiteWall
                | RedNetherBrickWall
                | SandstoneWall
                | EndStoneBrickWall
                | DioriteWall
                | CobblestoneWall
                | MossyCobblestoneWall
                | BlackstoneWall
                | PolishedBlackstoneBrickWall
                | PolishedBlackstoneWall
        )
    };
    let is_wall_compatible =
        |block| is_wall(block) || matches!(block.simplified_kind(), GlassPane | IronBars);
    if is_wall_compatible(block) {
        east_connected |= east.is_opaque() || is_wall_compatible(east);
        west_connected |= west.is_opaque() || is_wall_compatible(west);
        north_connected |= north.is_opaque() || is_wall_compatible(north);
        south_connected |= south.is_opaque() || is_wall_compatible(south);
        if block.has_up() {
            block.set_up((east_connected ^ west_connected) || (north_connected ^ south_connected));
        }
    }
    if is_wall(block) || matches!(block.simplified_kind(), Fence) {
        let gate_ew = |block: BlockId| {
            matches!(
                block.facing_cardinal(),
                Some(FacingCardinal::North | FacingCardinal::South)
            )
        };
        let gate_ns = |block: BlockId| {
            matches!(
                block.facing_cardinal(),
                Some(FacingCardinal::West | FacingCardinal::East)
            )
        };
        east_connected |=
            east.is_opaque() || (east.simplified_kind() == FenceGate && gate_ew(east));
        west_connected |=
            west.is_opaque() || (west.simplified_kind() == FenceGate && gate_ew(west));
        north_connected |=
            north.is_opaque() || (north.simplified_kind() == FenceGate && gate_ns(north));
        south_connected |=
            south.is_opaque() || (south.simplified_kind() == FenceGate && gate_ns(south));
    }
    // TODO: walls are tall when stacked
    if is_wall(block) {
        block.set_east_nlt(if east_connected {
            EastNlt::Low
        } else {
            EastNlt::None
        });
        block.set_west_nlt(if west_connected {
            WestNlt::Low
        } else {
            WestNlt::None
        });
        block.set_north_nlt(if north_connected {
            NorthNlt::Low
        } else {
            NorthNlt::None
        });
        block.set_south_nlt(if south_connected {
            SouthNlt::Low
        } else {
            SouthNlt::None
        });
    } else {
        block.set_east_connected(east_connected);
        block.set_west_connected(west_connected);
        block.set_north_connected(north_connected);
        block.set_south_connected(south_connected);
    }
    world.set_block_at(pos, block);

    Some(())
}
