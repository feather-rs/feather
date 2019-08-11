//! Provides several useful components, including `EntityComponent`
//! and `PlayerComponent`. In the future, will also
//! provide entity-specific components and systems.

mod component;
mod destroy;
mod movement;

pub use component::{EntityComponent, PlayerComponent};
pub use destroy::{EntityDestroyBroadcastSystem, EntityDestroyEvent, EntityDestroySystem};

pub use movement::broadcast_entity_movement;
