//! Various Specs components.

use feather_core::world::Position;
use feather_core::Gamemode;
use glm::Vec3;
use specs::storage::BTreeStorage;
use specs::{Component, DenseVecStorage, FlaggedStorage, VecStorage};
use uuid::Uuid;

pub struct PlayerComponent {
    pub profile_properties: Vec<mojang_api::ServerAuthProperty>,
    pub gamemode: Gamemode,
}

impl Component for PlayerComponent {
    type Storage = BTreeStorage<Self>;
}

#[derive(Deref, DerefMut, Debug, PartialEq, Eq)]
pub struct PositionComponent(pub Position);

impl Component for PositionComponent {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

/// An entity's velocity, in blocks per tick.
///
/// Entities without this component are assumed
/// to have a velocity of 0.
#[derive(Deref, DerefMut, Debug, PartialEq)]
pub struct VelocityComponent(pub Vec3);

impl Component for VelocityComponent {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

impl Default for VelocityComponent {
    fn default() -> Self {
        Self(glm::vec3(0.0, 0.0, 0.0))
    }
}

impl Default for VelocityComponent {
    fn default() -> Self {
        VelocityComponent(glm::vec3(0.0, 0.0, 0.0))
    }
}

#[derive(Clone, Debug)]
pub struct NamedComponent {
    pub display_name: String,
    pub uuid: Uuid,
}

impl Component for NamedComponent {
    type Storage = BTreeStorage<Self>;
}
