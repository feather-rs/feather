//! This module provides systems and components
//! relating to players, including player movement
//! and inventory handling.

/// Module for handling player animation broadcasting
/// (e.g. when a player swings their arm).
mod animation;
/// Module for broadcasting when a player joins and leaves.
mod broadcast;
/// Module for handling the Player Digging packet.
mod digging;
/// Module for initializing the necessary components
/// when a player joins.
mod init;
/// Module for handling player inventory.
mod inventory;
/// Module for handling player movement packets.
/// Also handles loading/unloading chunks when necessary.
mod movement;
mod resource_pack;

pub use init::PlayerInitSystem;

pub use broadcast::{DisconnectBroadcastSystem, JoinBroadcastSystem, PlayerDisconnectEvent};

pub use movement::{
    send_chunk_to_player, ChunkCrossSystem, ChunkPendingComponent, ChunkSendSystem,
    ClientChunkUnloadSystem, LoadedChunksComponent, PlayerMovementSystem,
};

pub use animation::{AnimationBroadcastSystem, PlayerAnimationEvent, PlayerAnimationSystem};

pub use inventory::{
    CreativeInventorySystem, EquipmentSendSystem, HeldItemBroadcastSystem, HeldItemChangeSystem,
    InventoryComponent, InventoryUpdateEvent,
};

pub use digging::{
    BlockUpdateBroadcastSystem, BlockUpdateCause, BlockUpdateEvent, PlayerDiggingSystem,
    PlayerItemDropEvent,
};

pub use resource_pack::ResourcePackSendSystem;

pub const PLAYER_EYE_HEIGHT: f32 = 1.62;
pub const PLAYER_EYE_HEIGHT_WHILE_SNEAKING: f32 = 1.54;
