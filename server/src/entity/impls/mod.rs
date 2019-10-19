//! Entity implementations.
//!
//! Every entity implementation is expected to define
//! the following functions:
//!
//! * `create(&LazyUpdate, &EntitiesRes) -> LazyBuilder`. When a system spawns an entity
//! of a known type, it should call this function on the `LazyBuilder`
//! returned by `LazyUpdate::spawn_entity` to apply components, such as markers, metadata,
//! `SerializerComponent`, and `SpawnPacketComponent`. This function may
//! take parameters. This function should not apply generic components,
//! such as position and velocity; the callee is responsible for this.
//! * `create_from_data(&LazyUpdate, &EntitiesRes, &{Entity}Data) -> Option<Entity>`. Spawns an
//! entity loaded from the given entity data. If the entity creation failed, `None` is returned.
//!
//! These functions should be invoked in the form `name::function`, e.g.
//! `arrow::create` or `item::create_from_data`.
//!
//! Entity implementations should also define systems related to the entity: for
//! example, most entities will have an update system which updates an entity
//! on each tick.

pub mod arrow;
pub mod falling_block;
pub mod item;
#[cfg(test)]
pub mod test;
