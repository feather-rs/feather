//! Provides several useful components, including `EntityComponent`
//! and `PlayerComponent`. In the future, will also
//! provide entity-specific components and systems.

mod arrow;
mod broadcast;
mod chunk;
mod component;
mod destroy;
mod item;
pub mod metadata;
mod movement;
mod types;

use crate::systems::{
    CHUNK_CROSS, CHUNK_ENTITIES_LOAD, CHUNK_ENTITIES_UPDATE, ENTITY_DESTROY,
    ENTITY_DESTROY_BROADCAST, ENTITY_METADATA_BROADCAST, ENTITY_MOVE_BROADCAST, ENTITY_SEND,
    ENTITY_SPAWN_BROADCAST, ENTITY_VELOCITY_BROADCAST, ITEM_COLLECT, ITEM_MERGE, ITEM_SPAWN,
    JOIN_BROADCAST, SHOOT_ARROW,
};
pub use arrow::{ArrowComponent, ShootArrowEvent};
pub use broadcast::EntitySendSystem;
pub use broadcast::EntitySender;
pub use broadcast::EntitySpawnEvent;
pub use chunk::ChunkEntities;
pub use chunk::ChunkEntityUpdateSystem;
pub use component::{NamedComponent, PlayerComponent, PositionComponent, VelocityComponent};
pub use destroy::EntityDestroyEvent;
pub use item::ItemComponent;
pub use metadata::{EntityBitMask, Metadata};
pub use movement::broadcast_entity_movement;
pub use types::EntityType;

use crate::entity::arrow::ShootArrowSystem;
use crate::entity::chunk::EntityChunkLoadSystem;
use crate::entity::destroy::EntityDestroyBroadcastSystem;
use crate::entity::item::ItemCollectSystem;
use crate::entity::metadata::MetadataBroadcastSystem;
use broadcast::EntityBroadcastSystem;
use component::ComponentResetSystem;
use destroy::EntityDestroySystem;
use item::{ItemMergeSystem, ItemSpawnSystem};
use movement::{EntityMoveBroadcastSystem, EntityVelocityBroadcastSystem};
use specs::DispatcherBuilder;

pub fn init_logic(dispatcher: &mut DispatcherBuilder) {
    dispatcher.add(ItemCollectSystem::default(), ITEM_COLLECT, &[]);
}

pub fn init_handlers(dispatcher: &mut DispatcherBuilder) {
    dispatcher.add(
        ChunkEntityUpdateSystem::default(),
        CHUNK_ENTITIES_UPDATE,
        &[],
    );
    dispatcher.add(EntityChunkLoadSystem::default(), CHUNK_ENTITIES_LOAD, &[]);
    dispatcher.add(EntityDestroySystem::default(), ENTITY_DESTROY, &[]);
    dispatcher.add(ItemSpawnSystem::default(), ITEM_SPAWN, &[]);
    dispatcher.add(ItemMergeSystem::default(), ITEM_MERGE, &[]);
    dispatcher.add(
        MetadataBroadcastSystem::default(),
        ENTITY_METADATA_BROADCAST,
        &[],
    );
    dispatcher.add(ShootArrowSystem::default(), SHOOT_ARROW, &[]);
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
        &[JOIN_BROADCAST, CHUNK_CROSS],
    );
    dispatcher.add(EntitySendSystem, ENTITY_SEND, &[ENTITY_SPAWN_BROADCAST]);
    dispatcher.add(
        EntityVelocityBroadcastSystem::default(),
        ENTITY_VELOCITY_BROADCAST,
        &[],
    );
    dispatcher.add(
        EntityDestroyBroadcastSystem::default(),
        ENTITY_DESTROY_BROADCAST,
        &[],
    );
    dispatcher.add_thread_local(ComponentResetSystem);
}
