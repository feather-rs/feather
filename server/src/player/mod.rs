//! This module provides systems and components
//! relating to players, including player movement
//! and inventory handling.

/// Module for handling player animation broadcasting
/// (e.g. when a player swings their arm).
mod animation;
/// Module for broadcasting when a player joins and leaves.
mod broadcast;
/// Module for initializing the necessary components
/// when a player joins.
mod init;
/// Module for handling player movement packets.
/// Also handles loading/unloading chunks when necessary.
mod movement;

pub use init::PlayerInitSystem;

pub use broadcast::{DisconnectBroadcastSystem, JoinBroadcastSystem, PlayerDisconnectEvent};

pub use movement::{
    send_chunk_to_player, ChunkCrossSystem, ChunkPendingComponent, ChunkSendSystem,
    ClientChunkUnloadSystem, LoadedChunksComponent, PlayerMovementSystem,
};

pub use animation::{AnimationBroadcastSystem, PlayerAnimationEvent, PlayerAnimationSystem};
