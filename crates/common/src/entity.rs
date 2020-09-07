//! Entity implementations.
//!
//! Each entity should be implemented in a submodule of this module.
//! It should export at least the following:
//! * A `build(builder)` function which fills an `EntityBuilder` with components for that entity.
//! * A marker component for that entity kind.

pub mod player;
