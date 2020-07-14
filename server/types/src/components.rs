mod marker;
mod network;
mod physics;
mod serialize;

pub use marker::*;
pub use serialize::*;

pub use feather_core::inventory::Inventory;
pub use network::{Network, ServerToWorkerMessage, WorkerToServerMessage};
pub use physics::{AABBExt, Physics, PhysicsBuilder};
pub use uuid::Uuid;

use ahash::AHashSet;
use dashmap::DashMap;
use feather_core::text::Text;
use feather_core::util::{ChunkPosition, Position};
use fecs::Entity;

/// The item an entity is currently holding.
///
/// This is the index inside the `Hotbar` area
/// of the inventory.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct HeldItem(pub usize);

/// An entity's name.
#[derive(Debug, Clone, Default)]
pub struct Name(pub String);

/// Position of an entity on the previous tick.
#[derive(Copy, Clone, Debug, Default)]
pub struct PreviousPosition(pub Option<Position>);

/// An entity's velocity.
#[derive(Copy, Clone, Debug)]
pub struct Velocity(pub glm::DVec3);

impl Default for Velocity {
    fn default() -> Self {
        Velocity(glm::vec3(0.0, 0.0, 0.0))
    }
}

/// Velocity of an entity on the previous tick.
#[derive(Copy, Clone, Debug, Default)]
pub struct PreviousVelocity(pub Option<glm::DVec3>);

/// Network ID of an entity.
#[derive(Copy, Clone, Debug)]
pub struct NetworkId(pub i32);

/// Component which stores which
/// chunks a given entity has a holder
/// on.
///
/// Although this information is also
/// stored in the `ChunkHolders` resource,
/// using this component allows for efficiently
/// finding which chunks a given entity has
/// a hold on, rather than having
/// to linear search all chunks (obviously ridiculous).
#[derive(Default)]
pub struct ChunkHolder {
    pub holds: AHashSet<ChunkPosition>,
}

/// Component containing the last sent positions of all entities for a given client.
/// This component is used to determine
/// the relative movement for an entity.
#[derive(Default, Debug)]
pub struct LastKnownPositions(pub DashMap<Entity, Position>);

/// Profile properties of a player.
#[derive(Debug, Clone)]
pub struct ProfileProperties(pub Vec<mojang_api::ProfileProperty>);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ParticleCount(pub u32);

/// Component added to entities to which messages can be sent.
#[derive(Default, Debug)]
pub struct MessageReceiver {
    /// Internal buffer of messages to send
    buffer: Vec<Text>,
}

impl MessageReceiver {
    /// Sends a message to the entity.
    pub fn send(&mut self, message: impl Into<Text>) {
        self.buffer.push(message.into());
    }

    /// Flushes the message buffer, returning an iterator
    /// over messages.
    pub fn flush<'a>(&'a mut self) -> impl Iterator<Item = Text> + 'a {
        self.buffer.drain(..)
    }
}

/// Component which stores a number which used for OpenWindow packets, incremented on every access
#[derive(Debug)]
pub struct OpenWindowCount {
    count: u8,
}

impl OpenWindowCount {
    pub fn get_increment(&mut self) -> u8 {
        self.count += 1;
        self.count - 1
    }
}

impl Default for OpenWindowCount {
    fn default() -> Self {
        OpenWindowCount { count: 1 }
    }
}
/// Health of an entity. Measured in "half-hearts."
#[derive(Copy, Clone, Debug)]
pub struct Health(pub u32);

/// Maximum health of an entity under normal conditions (i.e. excluding potion effects
/// such as absorption.)
#[derive(Copy, Clone, Debug)]
pub struct MaxHealth(pub u32);

/// Stores the number of blocks fallen by an entity
/// since the last time they were on_ground.
#[derive(Default, Copy, Clone, Debug)]
pub struct BlocksFallen(pub f64);
