//! Entity implementations.
//!
//! Every entity implementation is expected to define
//! the following functions:
//!
//! * `create(LazyBuilder) -> LazyBuilder`. When a system spawns an entity
//! of a known type, it should call this function on the `LazyBuilder`
//! returned by `LazyUpdate::spawn_entity` to apply components, such as markers,
//! `SerializerComponent`, and `SpawnPacketComponent`. This function may
//! take parameters. This function should not apply generic components,
//! such as position and velocity: the callee is responsible for this.
//! * TODO: more?
//!
//! These functions should be invoked in the form `name::function`, e.g.
//! `arrow::apply_components` or `item::apply_components`.
//!
//! Entity implementations should also define systems related to the entity: for
//! example, most entities will have an update system which updates an entity
//! on each tick.

pub mod arrow;
pub mod falling_block;
pub mod item;
