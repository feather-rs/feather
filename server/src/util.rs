//! Assorted utilities for use in Feather's codebase.

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
