use base::{
    BlockId, BlockPosition, EastNlt, FacingCardinal, NorthNlt, SimplifiedBlockKind, SouthNlt,
    WestNlt,
};
use libcraft_core::BlockFace;

use crate::{block::util::AdjacentBlockHelper, World};

pub fn is_wall(block: BlockId) -> bool {
    use base::SimplifiedBlockKind::*;
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
}
pub fn is_wall_compatible(block: BlockId) -> bool {
    use base::SimplifiedBlockKind::*;
    is_wall(block) || matches!(block.simplified_kind(), GlassPane | IronBars)
}
fn adjacent_or_default(
    world: &World,
    pos: BlockPosition,
    face: BlockFace,
    default: BlockId,
) -> BlockId {
    world.adjacent_block(pos, face).unwrap_or(default)
}
fn gate_connects_ew(block: BlockId) -> bool {
    block.simplified_kind() == SimplifiedBlockKind::FenceGate
        && matches!(
            block.facing_cardinal(),
            Some(FacingCardinal::North | FacingCardinal::South)
        )
}
fn gate_connects_ns(block: BlockId) -> bool {
    block.simplified_kind() == SimplifiedBlockKind::FenceGate
        && matches!(
            block.facing_cardinal(),
            Some(FacingCardinal::East | FacingCardinal::West)
        )
}

pub fn connect_neighbours_and_up(world: &mut World, pos: BlockPosition) -> Option<()> {
    use base::SimplifiedBlockKind::*;
    let mut block = world.block_at(pos)?;
    let east = adjacent_or_default(world, pos, BlockFace::East, BlockId::air());
    let west = adjacent_or_default(world, pos, BlockFace::West, BlockId::air());
    let north = adjacent_or_default(world, pos, BlockFace::North, BlockId::air());
    let south = adjacent_or_default(world, pos, BlockFace::South, BlockId::air());
    let mut east_connected = east.simplified_kind() == block.simplified_kind();
    let mut west_connected = west.simplified_kind() == block.simplified_kind();
    let mut north_connected = north.simplified_kind() == block.simplified_kind();
    let mut south_connected = south.simplified_kind() == block.simplified_kind();
    if is_wall_compatible(block) || matches!(block.simplified_kind(), Fence) {
        east_connected |= east.is_opaque();
        west_connected |= west.is_opaque();
        north_connected |= north.is_opaque();
        south_connected |= south.is_opaque();
    }
    if is_wall_compatible(block) {
        east_connected |= is_wall_compatible(east);
        west_connected |= is_wall_compatible(west);
        north_connected |= is_wall_compatible(north);
        south_connected |= is_wall_compatible(south);
    }
    if is_wall(block) {
        east_connected |= gate_connects_ew(east);
        west_connected |= gate_connects_ew(west);
        north_connected |= gate_connects_ns(north);
        south_connected |= gate_connects_ns(south);
    }
    // TODO: walls are tall when stacked
    if is_wall(block) {
        block.set_up((east_connected ^ west_connected) || (north_connected ^ south_connected));
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
