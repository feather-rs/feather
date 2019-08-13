//! Assorted utilities for use in Feather's codebase.
use glm::Vec3;

/// Asserts that a floating-point value is within
/// a certain range of the expected value.
#[cfg(test)]
macro_rules! assert_float_eq {
    ($left:expr, $right:expr) => {
        assert_float_eq!($left, $right, 0.001);
    };
    ($left:expr, $right:expr, $range:expr) => {
        let range = ($left - $range)..($left + $range);
        assert!(range.contains(&$right));
    };
}

/// Converts float-based velocity in blocks per tick
/// to the obnoxious format used by the protocol.
pub fn protocol_velocity(vel: Vec3) -> (i16, i16, i16) {
    // Apparently, these are in units of 1/8000 block per tick.
    (
        (vel.x / 8000.0) as i16,
        (vel.y / 8000.0) as i16,
        (vel.z / 8000.0) as i16,
    )
}
