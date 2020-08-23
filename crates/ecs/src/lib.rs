//! A lightweight ECS wrapper tailored to Feather's needs.
//!
//! This is implemented as a wrapper around the Bevy Engine's fork of the
//!  `hecs` crate, but we've made some interface changes:
//! * A system framework has been implemented, with systems written as plain functions and
//! executed sequentialy.
//! * `World` is renamed to `Ecs` so as to avoid conflict with Minecraft's concept of worlds.
//!
//! This wrapper library exists in case we need additional features in the ECS. If necessary,
//! we can change the backend crate or fork it as needed, without refactoring the rest of the codebase.

/// Use as a type in a query to filter by entities whose component
/// of type `T` was newly added in the last tick.
pub use hecs::Added;
/// Use as a type in a query to filter by entities whose component
/// of type `T` has been changed since the last tick.
pub use hecs::Changed;
pub use hecs::World as Ecs;
pub use hecs::{
    BuiltEntity, ComponentError, Entity, EntityBuilder, EntityRef, MissingComponent, NoSuchEntity,
};

mod system;
pub use system::{Stage, SysResult, SystemExecutor, SystemFn};
