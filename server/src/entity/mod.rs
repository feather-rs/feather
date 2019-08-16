//! Provides several useful components, including `EntityComponent`
//! and `PlayerComponent`. In the future, will also
//! provide entity-specific components and systems.

mod broadcast;
mod chunk;
mod component;
mod destroy;
mod item;
pub mod metadata;
mod movement;
mod types;

use crate::systems::{
    CHUNK_ENTITIES_UPDATE, ENTITY_DESTROY, ENTITY_MOVE_BROADCAST, ENTITY_SPAWN_BROADCAST,
    ENTITY_VELOCITY_BROADCAST, ITEM_SPAWN,
};
pub use broadcast::{EntityBroadcastSystem, EntitySpawnEvent};
pub use chunk::{ChunkEntities, ChunkEntityUpdateSystem};
pub use component::{EntityComponent, PlayerComponent, VelocityComponent};
pub use destroy::{EntityDestroyBroadcastSystem, EntityDestroyEvent, EntityDestroySystem};
pub use item::ItemSpawnSystem;
pub use metadata::{EntityBitMask, Metadata};
pub use movement::{
    broadcast_entity_movement, EntityMoveBroadcastSystem, EntityMoveEvent,
    EntityVelocityBroadcastSystem,
};
use specs::DispatcherBuilder;
pub use types::EntityType;

pub fn init_logic(_: &mut DispatcherBuilder) {}

pub fn init_handlers(dispatcher: &mut DispatcherBuilder) {
    dispatcher.add(
        ChunkEntityUpdateSystem::default(),
        CHUNK_ENTITIES_UPDATE,
        &[],
    );
    dispatcher.add(EntityDestroySystem::default(), ENTITY_DESTROY, &[]);
    dispatcher.add(ItemSpawnSystem::default(), ITEM_SPAWN, &[]);
}

pub fn init_broadcast(dispatcher: &mut DispatcherBuilder) {
    dispatcher.add(
        EntityMoveBroadcastSystem::default(),
        ENTITY_MOVE_BROADCAST,
        &[],
    );
    dispatcher.add(
        EntityBroadcastSystem::default(),
        ENTITY_SPAWN_BROADCAST,
        &[],
    );
    dispatcher.add(
        EntityVelocityBroadcastSystem::default(),
        ENTITY_VELOCITY_BROADCAST,
        &[],
    );
    dispatcher.add(EntityDestroySystem::default(), ENTITY_DESTROY, &[]);
}
