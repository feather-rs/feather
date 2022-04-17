//! The game events exposed to plugins.

pub use block_interact::{BlockInteractEvent, BlockPlacementEvent};
pub use change::{
    BuildingAbilityEvent, CreativeFlyingEvent, FlyingAbilityEvent, GamemodeEvent,
    HeldItemChangeEvent, InstabreakEvent, InventorySlotUpdateEvent, InvulnerabilityEvent,
    SneakEvent, SprintEvent,
};
pub use entity::{EntityCreateEvent, EntityRemoveEvent, PlayerJoinEvent};
pub use interact_entity::InteractEntityEvent;
use libcraft::ChunkPosition;
use vane::Component;

use crate::world::WorldId;

mod block_interact;
mod change;
mod entity;
mod interact_entity;

/// Event triggered when a chunk is loaded.
#[derive(Debug)]
pub struct ChunkLoadEvent {
    pub pos: ChunkPosition,
    pub world: WorldId,
}
impl Component for ChunkLoadEvent {}
