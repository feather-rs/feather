//! This module provides systems and components
//! relating to players, including player movement
//! and inventory handling.

/// Module for handling player animation broadcasting
/// (e.g. when a player swings their arm).
mod animation;
/// Module for broadcasting when a player joins and leaves.
mod broadcast;
/// Module for handling and broadcasting chat messages.
mod chat;
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
/// Module for handling player block placements.
mod placement;
mod resource_pack;

pub use broadcast::PlayerDisconnectEvent;

pub use movement::{send_chunk_to_player, ChunkPendingComponent, LoadedChunksComponent};

pub use animation::PlayerAnimationEvent;

pub use inventory::{InventoryComponent, InventoryUpdateEvent};

pub use digging::{BlockUpdateCause, BlockUpdateEvent, PlayerItemDropEvent};

use crate::player::inventory::SetSlotSystem;
use crate::player::placement::BlockPlacementSystem;
use crate::systems::{
    ANIMATION_BROADCAST, BLOCK_BREAK_BROADCAST, BLOCK_PLACEMENT, CHAT_BROADCAST, CHUNK_CROSS,
    CHUNK_SEND, CLIENT_CHUNK_UNLOAD, CREATIVE_INVENTORY, DISCONNECT_BROADCAST, EQUIPMENT_SEND,
    HELD_ITEM_BROADCAST, HELD_ITEM_CHANGE, JOIN_BROADCAST, NETWORK, PLAYER_ANIMATION, PLAYER_CHAT,
    PLAYER_DIGGING, PLAYER_INIT, PLAYER_MOVEMENT, RESOURCE_PACK_SEND, SET_SLOT,
};
use animation::{AnimationBroadcastSystem, PlayerAnimationSystem};
use broadcast::{DisconnectBroadcastSystem, JoinBroadcastSystem};
use chat::{ChatBroadcastSystem, PlayerChatSystem};
use digging::BlockUpdateBroadcastSystem;
use digging::PlayerDiggingSystem;
use init::PlayerInitSystem;
use inventory::{
    CreativeInventorySystem, EquipmentSendSystem, HeldItemBroadcastSystem, HeldItemChangeSystem,
};
use movement::{ChunkCrossSystem, ChunkSendSystem, ClientChunkUnloadSystem, PlayerMovementSystem};
pub use resource_pack::ResourcePackSendSystem;
use specs::DispatcherBuilder;

pub const PLAYER_EYE_HEIGHT: f64 = 1.62;
pub const PLAYER_EYE_HEIGHT_WHILE_SNEAKING: f64 = 1.54;

pub fn init_logic(dispatcher: &mut DispatcherBuilder) {
    dispatcher.add(PlayerDiggingSystem, PLAYER_DIGGING, &[NETWORK]);
    dispatcher.add(PlayerAnimationSystem, PLAYER_ANIMATION, &[NETWORK]);
    dispatcher.add(CreativeInventorySystem, CREATIVE_INVENTORY, &[NETWORK]);
    dispatcher.add(HeldItemChangeSystem, HELD_ITEM_CHANGE, &[NETWORK]);
    dispatcher.add(PlayerMovementSystem, PLAYER_MOVEMENT, &[NETWORK]);
    dispatcher.add(PlayerChatSystem, PLAYER_CHAT, &[NETWORK]);
    dispatcher.add(BlockPlacementSystem, BLOCK_PLACEMENT, &[NETWORK]);
}

pub fn init_handlers(dispatcher: &mut DispatcherBuilder) {
    dispatcher.add(ChunkCrossSystem::default(), CHUNK_CROSS, &[]);
    dispatcher.add(ClientChunkUnloadSystem, CLIENT_CHUNK_UNLOAD, &[]);
    dispatcher.add(PlayerInitSystem::default(), PLAYER_INIT, &[]);
}

pub fn init_broadcast(dispatcher: &mut DispatcherBuilder) {
    dispatcher.add(HeldItemBroadcastSystem::default(), HELD_ITEM_BROADCAST, &[]);
    dispatcher.add(JoinBroadcastSystem::default(), JOIN_BROADCAST, &[]);
    dispatcher.add(
        DisconnectBroadcastSystem::default(),
        DISCONNECT_BROADCAST,
        &[],
    );
    dispatcher.add(
        AnimationBroadcastSystem::default(),
        ANIMATION_BROADCAST,
        &[],
    );
    dispatcher.add(EquipmentSendSystem::default(), EQUIPMENT_SEND, &[]);
    dispatcher.add(ResourcePackSendSystem::default(), RESOURCE_PACK_SEND, &[]);
    dispatcher.add(ChunkSendSystem::default(), CHUNK_SEND, &[]);
    dispatcher.add(
        BlockUpdateBroadcastSystem::default(),
        BLOCK_BREAK_BROADCAST,
        &[],
    );
    dispatcher.add(SetSlotSystem::default(), SET_SLOT, &[]);
    dispatcher.add(ChatBroadcastSystem::default(), CHAT_BROADCAST, &[]);
}
