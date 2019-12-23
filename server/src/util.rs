//! Assorted utility functions.

use feather_core::Position;

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
