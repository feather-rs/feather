//! Provides several useful components, including `EntityComponent`
//! and `PlayerComponent`. In the future, will also
//! provide entity-specific components and systems.

use feather_core::world::Position;
use feather_core::Gamemode;
use specs::storage::BTreeStorage;
use specs::{Component, VecStorage};
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
