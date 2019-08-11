//! Module with various types representing entity types
//! and metadata.

use specs::{Component, VecStorage};

/// The type of an entity.
///
/// This is primarily used to determine
/// which packet to send to spawn the entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityType {
    Player,
    Item,
    ExperienceOrb,
    Thunderbolt,
    // TODO more...
}

impl Component for EntityType {
    type Storage = VecStorage<Self>;
}
