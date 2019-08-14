//! Various Specs components.

use feather_core::world::Position;
use feather_core::Gamemode;
use glm::Vec3;
use specs::storage::BTreeStorage;
use specs::{Component, DenseVecStorage, VecStorage};
use uuid::Uuid;

pub struct PlayerComponent {
    pub profile_properties: Vec<mojang_api::ServerAuthProperty>,
    pub gamemode: Gamemode,
}

impl Component for PlayerComponent {
    type Storage = BTreeStorage<Self>;
}

pub struct EntityComponent {
    pub uuid: Uuid,
    pub display_name: String,
    pub position: Position,
    pub on_ground: bool,
}

impl Component for EntityComponent {
    type Storage = VecStorage<Self>;
}

/// An entity's velocity, in blocks per tick.
///
/// Entities without this component are assumed
/// to have a velocity of 0.
#[derive(Deref, DerefMut, Clone, Debug, PartialEq)]
pub struct VelocityComponent(pub Vec3);

impl Component for VelocityComponent {
    type Storage = DenseVecStorage<Self>;
}

impl Default for VelocityComponent {
    fn default() -> Self {
        VelocityComponent(glm::vec3(0.0, 0.0, 0.0))
    }
}
