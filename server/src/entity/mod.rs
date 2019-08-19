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
    ENTITY_VELOCITY_BROADCAST, ITEM_MERGE, ITEM_SPAWN, JOIN_BROADCAST,
};
pub use broadcast::EntitySpawnEvent;
pub use chunk::ChunkEntities;
pub use component::{NamedComponent, PlayerComponent, PositionComponent, VelocityComponent};
pub use destroy::EntityDestroyEvent;
pub use item::ItemMarker;
pub use metadata::{EntityBitMask, Metadata};
pub use movement::broadcast_entity_movement;
pub use types::EntityType;

use broadcast::EntityBroadcastSystem;
use chunk::ChunkEntityUpdateSystem;
use component::ComponentResetSystem;
use destroy::EntityDestroySystem;
use item::{ItemMergeSystem, ItemSpawnSystem};
use movement::{EntityMoveBroadcastSystem, EntityVelocityBroadcastSystem};
use specs::DispatcherBuilder;

pub fn init_logic(_: &mut DispatcherBuilder) {}

pub fn init_handlers(dispatcher: &mut DispatcherBuilder) {
    dispatcher.add(
        ChunkEntityUpdateSystem::default(),
        CHUNK_ENTITIES_UPDATE,
        &[],
    );
    dispatcher.add(EntityDestroySystem::default(), ENTITY_DESTROY, &[]);
    dispatcher.add(ItemSpawnSystem::default(), ITEM_SPAWN, &[]);
    dispatcher.add(ItemMergeSystem::default(), ITEM_MERGE, &[]);
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
        &[JOIN_BROADCAST],
    );
    dispatcher.add(
        EntityVelocityBroadcastSystem::default(),
        ENTITY_VELOCITY_BROADCAST,
        &[],
    );
    dispatcher.add_thread_local(ComponentResetSystem);
}
