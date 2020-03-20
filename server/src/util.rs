//! Assorted utility functions.

use arrayvec::ArrayVec;
use feather_core::{BlockPosition, Position};
use glm::DVec3;

/// Calculates the relative move fields
/// as used in the Entity Relative Move packets.
pub fn calculate_relative_move(old: Position, current: Position) -> (i16, i16, i16) {
    let x = ((current.x * 32.0 - old.x * 32.0) * 128.0) as i16;
    let y = ((current.y * 32.0 - old.y * 32.0) * 128.0) as i16;
    let z = ((current.z * 32.0 - old.z * 32.0) * 128.0) as i16;
    (x, y, z)
}

/// Converts degrees to stops as used in the protocol.
pub fn degrees_to_stops(degs: f32) -> u8 {
    ((degs / 360.0) * 256.0) as u8
}

/// Converts float-based velocity in blocks per tick
/// to the format used by the protocol.
pub fn protocol_velocity(vel: DVec3) -> (i16, i16, i16) {
    // These are in units of 1/8000 block per tick.
    (
        (vel.x * 8000.0) as i16,
        (vel.y * 8000.0) as i16,
        (vel.z * 8000.0) as i16,
    )
}

/// Returns the set of block positions adjacent to a given position.
pub fn adjacent_blocks(pos: BlockPosition) -> ArrayVec<[BlockPosition; 6]> {
    ArrayVec::from([
        pos + BlockPosition::new(1, 0, 0),
        pos + BlockPosition::new(0, 1, 0),
        pos + BlockPosition::new(0, 0, 1),
        pos + BlockPosition::new(-1, 0, 0),
        pos + BlockPosition::new(0, -1, 0),
        pos + BlockPosition::new(0, 0, -1),
    ])
}
