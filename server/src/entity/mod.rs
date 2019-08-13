//! Provides several useful components, including `EntityComponent`
//! and `PlayerComponent`. In the future, will also
//! provide entity-specific components and systems.

mod broadcast;
mod component;
mod destroy;
pub mod metadata;
mod movement;
mod types;

pub use broadcast::{EntityBroadcastSystem, EntitySpawnEvent};
pub use component::{EntityComponent, PlayerComponent, VelocityComponent};
pub use destroy::{EntityDestroyBroadcastSystem, EntityDestroyEvent, EntityDestroySystem};
pub use metadata::Metadata;
pub use movement::{broadcast_entity_movement, EntityMoveBroadcastSystem, EntityMoveEvent};
pub use types::EntityType;
