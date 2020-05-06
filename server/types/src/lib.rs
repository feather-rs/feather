#![forbid(unsafe_code)]

//! Defines components and resources so that subcrates can interact
//! in some ways without depending on each other.

extern crate nalgebra_glm as glm;

// MISC

pub type BumpVec<'bump, T> = bumpalo::collections::Vec<'bump, T>;

pub trait EntityLoaderFn:
    Fn(EntityData) -> anyhow::Result<EntityBuilder> + Send + Sync + 'static
{
}

impl<F> EntityLoaderFn for F where
    F: Fn(EntityData) -> anyhow::Result<EntityBuilder> + Send + Sync + 'static
{
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Weather {
    Clear,
    Rain,
    Thunder,
}

/// A registration for a function to convert an `EntityData`
/// to an `EntityBuilder` for spawning into the world. The
/// registration must provide the `EntityDataKind` it handles
/// to determine which `EntityData`s to pass to this function.
pub struct EntityLoaderRegistration {
    /// The loader function.
    pub f: &'static dyn EntityLoaderFn,
    /// The kind of `EntityData` which this loader
    /// function will accept.
    pub kind: EntityDataKind,
}

impl EntityLoaderRegistration {
    pub fn new(kind: EntityDataKind, f: &'static dyn EntityLoaderFn) -> Self {
        Self { f, kind }
    }
}

inventory::collect!(EntityLoaderRegistration);

// CONSTANTS
/// The number of ticks per second.
pub const TPS: u64 = 20;
/// The number of milliseconds per tick.
pub const TICK_LENGTH: u64 = 1000 / TPS;

/// Height from a player's position where the camera lies.
pub const PLAYER_EYE_HEIGHT: f64 = 1.62;

// COMPONENTS

mod network;
mod physics;
mod task;

pub use feather_core::inventory::Inventory;
pub use network::{Network, ServerToWorkerMessage, WorkerToServerMessage};
pub use physics::{AABBExt, Physics, PhysicsBuilder};
pub use uuid::Uuid;

use feather_core::inventory::SlotIndex;
use feather_core::util::{BlockPosition, ChunkPosition, ClientboundAnimation, Position};

/// The item an entity is currently holding.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct HeldItem(pub SlotIndex);

/// An entity's name.
#[derive(Debug, Clone, Default)]
pub struct Name(pub String);

/// Position of an entity on the previous tick.
#[derive(Copy, Clone, Debug)]
pub struct PreviousPosition(pub Position);

/// An entity's velocity.
#[derive(Copy, Clone, Debug)]
pub struct Velocity(pub glm::DVec3);

impl Default for Velocity {
    fn default() -> Self {
        Velocity(glm::vec3(0.0, 0.0, 0.0))
    }
}

/// Velocity of an entity on the previous tick.
#[derive(Copy, Clone, Debug)]
pub struct PreviousVelocity(pub glm::DVec3);

impl Default for PreviousVelocity {
    fn default() -> Self {
        PreviousVelocity(Velocity::default().0)
    }
}

/// Network ID of an entity.
#[derive(Copy, Clone, Debug)]
pub struct NetworkId(pub i32);

pub trait PacketCreatorFn: Fn(&EntityRef) -> Box<dyn Packet> + Send + Sync + 'static {}
impl<F> PacketCreatorFn for F where F: Fn(&EntityRef) -> Box<dyn Packet> + Send + Sync + 'static {}

/// Component which defines a function returning a packet to send
/// to clients when the entity comes within range. This packet
/// spawns the entity on the client.
pub struct SpawnPacketCreator(pub &'static dyn PacketCreatorFn);

impl SpawnPacketCreator {
    /// Returns the packet to send to clients when the entity is to be
    /// sent to the client.
    pub fn get(&self, accessor: &EntityRef) -> Box<dyn Packet> {
        let f = self.0;

        f(accessor)
    }
}

/// Component which defines a function returning a packet to send
/// to _all_ clients when the entity is created or the client joins.
/// This packet is sent before that returned by `SpawnPacketCreator`,
/// and it differs in that the packet is broadcasted globally
/// rather than to nearby clients.
///
/// Another difference is that the packet from `SpawnPacketCreator` is not sent
/// to its own entity, while that from `CreationPacketCreator` is.
///
/// An example of a use case for this packet is the `PlayerInfo` packet
/// sent when a player joins—it is sent to all players, not just those
/// that are able to see the player.
pub struct CreationPacketCreator(pub &'static dyn PacketCreatorFn);

impl CreationPacketCreator {
    /// Returns the packet to send to clients when the entity is created.
    pub fn get(&self, accessor: &EntityRef) -> Box<dyn Packet> {
        let f = self.0;

        f(accessor)
    }
}

pub trait ComponentSerializerFn:
    Fn(&Game, &EntityRef) -> EntityData + Send + Sync + 'static
{
}

impl<F> ComponentSerializerFn for F where
    F: Fn(&Game, &EntityRef) -> EntityData + Send + Sync + 'static
{
}

/// Component which stores a function needed to convert an entity's
/// components to the serializable `EntityData`.
pub struct ComponentSerializer(pub &'static dyn ComponentSerializerFn);

impl ComponentSerializer {
    pub fn serialize(&self, game: &Game, accessor: &EntityRef) -> EntityData {
        let f = self.0;

        f(game, accessor)
    }
}

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

impl ChunkHolder {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Component containing the last sent positions of all entities for a given client.
/// This component is used to determine
/// the relative movement for an entity.
#[derive(Default, Debug)]
pub struct LastKnownPositions(pub DashMap<Entity, Position>);

/// Profile properties of a player.
#[derive(Debug, Clone)]
pub struct ProfileProperties(pub Vec<mojang_api::ProfileProperty>);

/// Zero-sized marker component used to mark players.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Player;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct ParticleCount(pub u32);

// RESOURCES

use ahash::AHashSet;
use dashmap::DashMap;
use feather_core::anvil::entity::{EntityData, EntityDataKind};
use feather_core::blocks::BlockId;
use feather_core::items::ItemStack;
use feather_core::network::Packet;
use fecs::{Entity, EntityBuilder, EntityRef};
use smallvec::SmallVec;

mod game;
pub use feather_server_config::{Config, ProxyMode};
pub use feather_server_packet_buffer::{PacketBuffer, PacketBuffers};
pub use game::*;
pub use task::*;

// EVENTS

#[derive(Copy, Clone, Debug)]
pub struct BlockUpdateEvent {
    /// Position of the updated block
    pub pos: BlockPosition,
    /// Old block
    pub old: BlockId,
    /// New block
    pub new: BlockId,
    /// Cause of the block update.
    pub cause: BlockUpdateCause,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BlockUpdateCause {
    /// The update was caused by an entity performing
    /// a block break/placement. Usually a player.
    Entity(Entity),
    /// Unknown cause.
    Unknown,
}

/// Triggered directly _before_ an entity is removed from the world.
///
/// As such, components can still be accessed.
#[derive(Copy, Clone, Debug)]
pub struct EntityDespawnEvent {
    pub entity: Entity,
}

/// Triggered when a chunk is sent to a player.
#[derive(Copy, Clone, Debug)]
pub struct ChunkSendEvent {
    pub chunk: ChunkPosition,
    pub player: Entity,
}

/// Triggered when a player joins the server.
#[derive(Copy, Clone, Debug)]
pub struct PlayerJoinEvent {
    pub player: Entity,
}

/// Triggered when a player leaves.
#[derive(Copy, Clone, Debug)]
pub struct PlayerLeaveEvent {
    pub player: Entity,
}

/// Triggered when an entity lands on the ground.
#[derive(Copy, Clone, Debug)]
pub struct EntityLandEvent {
    pub entity: Entity,
    /// Position where the entity landed.
    pub pos: Position,
}

/// Event triggered when an item is dropped.
///
/// Before this event is triggered, the item
/// is removed from the player's inventory.
#[derive(Debug, Clone)]
pub struct ItemDropEvent {
    /// The slot from which the item was dropped,
    /// if known.
    pub slot: Option<SlotIndex>,
    /// The item stack which was dropped.
    pub stack: ItemStack,
    /// The player who dropped the item.
    pub player: Entity,
}

/// Event triggered when an item is collected into an entity's
/// inventory.
///
/// Triggered before the item is deleted from the world.
#[derive(Debug, Clone)]
pub struct ItemCollectEvent {
    /// The item which was collected.
    pub item: Entity,
    /// The entity which collected the item.
    pub collector: Entity,
    /// Number of items which was collected.
    pub amount: u8,
}

/// Event which is triggered when a player
/// updates their inventory.
///
/// This event could also be triggered when the player
/// changes their held item.
#[derive(Debug, Clone)]
pub struct InventoryUpdateEvent {
    /// The slot(s) affected by the update.
    ///
    /// Multiple slots could be affected when, for
    /// example, a player uses the "drag" inventory interaction.
    pub slots: SmallVec<[SlotIndex; 2]>,
    /// The player owning the updated inventory.
    pub player: Entity,
}

/// Event triggered when an entity is created.
#[derive(Copy, Clone, Debug)]
pub struct EntitySpawnEvent {
    pub entity: Entity,
}

/// Event triggered when a player performs an animation (hits with their hand).
#[derive(Copy, Clone, Debug)]
pub struct PlayerAnimationEvent {
    pub player: Entity,
    pub animation: ClientboundAnimation,
}

/// Event triggered when a chat message is sent out
#[derive(Debug, Clone)]
pub struct ChatEvent {
    /// The JSON-formatted message
    pub message: String,
    /// The position of the message
    pub position: ChatPosition,
}

/// Different positions a chat message can be displayed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatPosition {
    /// Simple message displayed in the chat box
    Chat,
    /// System message displayed in the chat box
    SystemMessage,
    /// A text displayed above the hotbar
    GameInfo,
}

/// Event triggered when an entity crosses into a new chunk.
#[derive(Copy, Clone, Debug)]
pub struct ChunkCrossEvent {
    pub entity: Entity,
    pub old: Option<ChunkPosition>,
    pub new: ChunkPosition,
}

/// Event triggered when an entity is sent to a client.
///
/// This can be used to send additional packets along with the Spawn *
/// packet, such as entity metadata.
#[derive(Copy, Clone, Debug)]
pub struct EntitySendEvent {
    /// The entity which was sent.
    pub entity: Entity,
    /// The client to which the entity was sent.
    pub client: Entity,
}

/// Event triggered when an entity is destroyed on a client.
///
/// This can be used to clean up data. For example, the movement
/// broadcast system stores the last known position of all visible
/// entities for each client. It uses this event to remove
/// entries in that map.
#[derive(Copy, Clone, Debug)]
pub struct EntityClientRemoveEvent {
    /// The entity which was destroyed on the client.
    pub entity: Entity,
    /// The client on which the entity was destroyed.
    pub client: Entity,
}

/// Event triggered when a chunk is loaded.
#[derive(Copy, Clone, Debug)]
pub struct ChunkLoadEvent {
    pub chunk: ChunkPosition,
}

/// Event which is triggered when a chunk fails to load.
#[derive(Debug)]
pub struct ChunkLoadFailEvent {
    pub pos: ChunkPosition,
    pub error: anyhow::Error,
}

/// Event triggeered when a chunk is unloaded.
#[derive(Copy, Clone, Debug)]
pub struct ChunkUnloadEvent {
    pub chunk: ChunkPosition,
}

/// Event triggered when a chunk holder releases their hold on a chunk.
#[derive(Copy, Clone, Debug)]
pub struct ChunkHolderReleaseEvent {
    /// Entity which released their hold.
    pub entity: Entity,
    /// The chunk which was released.
    pub chunk: ChunkPosition,
}

/// Triggered when the weather changes.
#[derive(Copy, Clone, Debug)]
pub struct WeatherChangeEvent {
    pub from: Weather,
    pub to: Weather,
    pub duration: i32,
}

/// Requests that a chunk be held for the given client.
///
/// This is a "request"-type event: it has one handler defined
/// in the `chunk` crate which executes the request.
#[derive(Copy, Clone, Debug)]
pub struct HoldChunkRequest {
    pub player: Entity,
    pub chunk: ChunkPosition,
}

/// Requests that a chunk hold be removed for the given client.
#[derive(Copy, Clone, Debug)]
pub struct ReleaseChunkRequest {
    pub player: Entity,
    pub chunk: ChunkPosition,
}

/// Requests that a chunk be queued for loading.
#[derive(Copy, Clone, Debug)]
pub struct LoadChunkRequest {
    pub chunk: ChunkPosition,
}
