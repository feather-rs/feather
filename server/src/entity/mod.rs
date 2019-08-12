//! Provides several useful components, including `EntityComponent`
//! and `PlayerComponent`. In the future, will also
//! provide entity-specific components and systems.

mod broadcast;
mod component;
mod destroy;
mod movement;
mod types;

pub use broadcast::{EntityBroadcastSystem, EntitySpawnEvent};
pub use component::{EntityComponent, PlayerComponent, VelocityComponent};
pub use destroy::{EntityDestroyBroadcastSystem, EntityDestroyEvent, EntityDestroySystem};
pub use movement::broadcast_entity_movement;
pub use types::EntityType;
