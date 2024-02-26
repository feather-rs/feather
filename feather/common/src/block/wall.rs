use base::{BlockId, BlockPosition, FacingCardinal, SimplifiedBlockKind};
use libcraft_core::BlockFace;

use crate::{
    block::util::{is_wall, AdjacentBlockHelper},
    World,
};

use super::util::Nlt;

/// General function that connects all walls, fences, glass panes, iron bars and tripwires and then lowers fence gates to fit into walls.
pub fn update_wall_connections(world: &mut World, pos: BlockPosition) -> Option<()> {
    lower_fence_gate(world, pos)?;
    connect_neighbours_and_up(world, pos)
}

/// Check if this block is a wall/iron bars/glass pane. If true, the block can connect to any other block that satisfies this predicate.
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
/// Checks if this block is a `FenceGate` and has one of its connecting side on the given `BlockFace`
fn gate_connects_to_face(block: BlockId, face: BlockFace) -> bool {
    use BlockFace::*;
    block.simplified_kind() == SimplifiedBlockKind::FenceGate
        && matches!(
            (face, block.facing_cardinal().unwrap()),
            (East | West, FacingCardinal::North | FacingCardinal::South)
                | (North | South, FacingCardinal::East | FacingCardinal::West)
        )
}

/// Uses the appropriate `BlockId::set_#####_connected` function, depending on the given `BlockFace`
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
/// Uses the appropriate `BlockId::set_#####_nlt` function, depending on the given `BlockFace`. The given `Nlt` is automatically converted.
fn set_face_nlt(block: &mut BlockId, face: BlockFace, nlt: Nlt) -> bool {
    use BlockFace::*;
    match face {
        Bottom | Top => false,
        East => block.set_east_nlt(nlt.into()),
        West => block.set_west_nlt(nlt.into()),
        North => block.set_north_nlt(nlt.into()),
        South => block.set_south_nlt(nlt.into()),
    }
}
/// Checks whether the block is a wall and connected to the given `BlockFace`
fn is_nlt_connected(block: BlockId, face: BlockFace) -> Option<bool> {
    use BlockFace::*;
    let f = |n: Nlt| matches!(n, Nlt::Low | Nlt::Tall);
    match face {
        Bottom | Top => None,
        East => block.east_nlt().map(|n| f(n.into())),
        West => block.west_nlt().map(|n| f(n.into())),
        North => block.north_nlt().map(|n| f(n.into())),
        South => block.south_nlt().map(|n| f(n.into())),
    }
}
/// Checks if the block is connected to the given `BlockFace`
fn is_face_connected(block: BlockId, face: BlockFace) -> Option<bool> {
    use BlockFace::*;
    match face {
        Bottom | Top => Some(false),
        East => block.east_connected(),
        West => block.west_connected(),
        North => block.north_connected(),
        South => block.south_connected(),
    }
}

/// If called on a position containing a `FenceGate`, checks whether the block connects to a wall. If true, this lowers the gate by setting its `in_wall` property to true.
pub fn lower_fence_gate(world: &mut World, pos: BlockPosition) -> Option<()> {
    let mut block = world.block_at(pos)?;
    if block.simplified_kind() != SimplifiedBlockKind::FenceGate {
        return Some(());
    }
    let left = block.facing_cardinal().unwrap().left();
    let right = left.opposite();
    // Check if neighbouring block is a wall
    let predicate = |fc| {
        is_wall(
            world
                .adjacent_block_cardinal(pos, fc)
                .unwrap_or_else(BlockId::air),
        )
    };
    // One wall is enough to lower the fence gate
    block.set_in_wall(predicate(left) || predicate(right));
    world.set_block_at(pos, block);
    Some(())
}

pub fn connect_neighbours_and_up(world: &mut World, pos: BlockPosition) -> Option<()> {
    use base::SimplifiedBlockKind::*;
    let mut block = world.block_at(pos)?;
    let up = adjacent_or_default(world, pos, BlockFace::Top, BlockId::air());
    let (mut east_connected, mut west_connected, mut north_connected, mut south_connected) =
        (false, false, false, false);
    // Iterate over cardinal directions
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
            // Wall connections are `Tall` when the block above then is opaque or has a connection in the same direction
            let tall_wall_connection = is_face_connected(up, block_face).unwrap_or(false)
                || is_nlt_connected(up, block_face).unwrap_or(false)
                || gate_connects_to_face(up, block_face)
                || up.is_opaque();
            let connection_type = if *connected_flag && tall_wall_connection {
                Nlt::Tall
            } else if *connected_flag {
                Nlt::Low
            } else {
                Nlt::None
            };
            set_face_nlt(&mut block, block_face, connection_type);
        } else {
            set_face_connected(&mut block, block_face, *connected_flag);
        }
    }
    if is_wall(block) {
        // Wall crossings always have a tall post
        let up_from_wall_cross = (east_connected ^ west_connected)
            || (north_connected ^ south_connected)
            || !(east_connected || west_connected || north_connected || south_connected);
        // Tall wall posts propagate downward
        let up_has_up = up.up().unwrap_or(false);
        // Walls always have a tall post when ending
        let this_ends = [
            (BlockFace::East, east_connected),
            (BlockFace::West, west_connected),
            (BlockFace::North, north_connected),
            (BlockFace::South, south_connected),
        ]
        .iter()
        .any(|&(f, con)| {
            con && !is_nlt_connected(block, f)
                .or_else(|| is_face_connected(block, f))
                .unwrap_or(true)
        });

        // Check if there is a wall/fence/etc ending above
        let up_ends = [
            (BlockFace::East, east_connected),
            (BlockFace::West, west_connected),
            (BlockFace::North, north_connected),
            (BlockFace::South, south_connected),
        ]
        .iter()
        .any(|&(f, con)| {
            con && !is_nlt_connected(up, f)
                .or_else(|| is_face_connected(up, f))
                .unwrap_or(true)
        });
        // TODO: Query this property at runtime because it is a tag.
        let wall_post_override = false;
        block.set_up(up_from_wall_cross || up_has_up || this_ends || up_ends || wall_post_override);
    }
    world.set_block_at(pos, block);
    Some(())
}
