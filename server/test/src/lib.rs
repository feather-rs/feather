//! A testing framework for `feather-server`. Provides
//! functions for both unit and integration testing.

#![forbid(unsafe_code)]

mod unit;

pub use unit::Test;
