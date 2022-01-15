use base::{BlockId, BlockPosition, FacingCardinal, SimplifiedBlockKind};
use libcraft_core::BlockFace;

use crate::{block::util::AdjacentBlockHelper, World};

use super::util::Nlt;

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
fn gate_connects_to_face(block: BlockId, face: BlockFace) -> bool {
    use BlockFace::*;
    match face {
        East | West => gate_connects_ew(block),
        North | South => gate_connects_ns(block),
        Top | Bottom => false,
    }
}

fn set_face_connected(block: &mut BlockId, face: BlockFace, connected: bool) -> bool {
    use BlockFace::*;
    match face {
        Bottom | Top => false,
        East => block.set_east_connected(connected),
        West => block.set_west_connected(connected),
        North => block.set_north_connected(connected),
        South => block.set_south_connected(connected),
    }
}
fn set_face_nlt(block: &mut BlockId, face: BlockFace, nlt: Nlt) -> bool {
    use BlockFace::*;
    match face {
        Bottom | Top => false,
        North => block.set_north_nlt(nlt.into()),
        South => block.set_south_nlt(nlt.into()),
        West => block.set_west_nlt(nlt.into()),
        East => block.set_east_nlt(nlt.into()),
    }
}

pub fn connect_neighbours_and_up(world: &mut World, pos: BlockPosition) -> Option<()> {
    use base::SimplifiedBlockKind::*;
    let mut block = world.block_at(pos)?;
    let (mut east_connected, mut west_connected, mut north_connected, mut south_connected) =
        (false, false, false, false);
    for (block_face, connected_flag) in [
        (BlockFace::East, &mut east_connected),
        (BlockFace::West, &mut west_connected),
        (BlockFace::North, &mut north_connected),
        (BlockFace::South, &mut south_connected),
    ] {
        let facing = adjacent_or_default(world, pos, block_face, BlockId::air());
        // Walls and fences connect to opaque blocks.
        *connected_flag |= is_wall_compatible(block) && facing.is_opaque();
        *connected_flag |= block.simplified_kind() == Fence && facing.is_opaque();
        // Walls, glass panes and iron bars all connect to one another.
        *connected_flag |= is_wall_compatible(block) && is_wall_compatible(facing);
        // Walls and fences connect to fence gates.
        *connected_flag |= is_wall(block) && gate_connects_to_face(facing, block_face);
        *connected_flag |=
            block.simplified_kind() == Fence && gate_connects_to_face(facing, block_face);
        // Blocks connect to those of the same kind. This handles tripwires and fences.
        *connected_flag |= facing.simplified_kind() == block.simplified_kind();

        if is_wall(block) {
            // TODO: walls are tall when stacked
            set_face_nlt(
                &mut block,
                block_face,
                if *connected_flag { Nlt::Low } else { Nlt::None },
            );
        } else {
            set_face_connected(&mut block, block_face, *connected_flag);
        }
    }
    if is_wall(block) {
        block.set_up((east_connected ^ west_connected) || (north_connected ^ south_connected));
    }
    world.set_block_at(pos, block);

    Some(())
}
