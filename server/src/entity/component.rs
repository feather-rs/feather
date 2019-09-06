//! Various Specs components.

use feather_core::world::Position;
use feather_core::Gamemode;
use glm::DVec3;
use specs::storage::BTreeStorage;
use specs::{Component, FlaggedStorage, Join, System, VecStorage, WriteStorage};
use uuid::Uuid;

pub struct PlayerComponent {
    pub profile_properties: Vec<mojang_api::ServerAuthProperty>,
    pub gamemode: Gamemode,
}

impl Component for PlayerComponent {
    type Storage = BTreeStorage<Self>;
}

#[derive(Default, Debug, PartialEq)]
pub struct PositionComponent {
    /// The current position of this entity.
    pub current: Position,
    /// The position of this entity on the previous
    /// tick. At the end of each tick, `reset` should
    /// be called.
    pub previous: Position,
}

impl PositionComponent {
    /// Resets the current and previous position.
    /// Should be called at the end of every tick.
    pub fn reset(&mut self) {
        self.previous = self.current;
    }
}

impl Component for PositionComponent {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

/// An entity's velocity, in blocks per tick.
///
/// Entities without this component are assumed
/// to have a velocity of 0.
#[derive(Deref, DerefMut, Debug, PartialEq, Clone)]
pub struct VelocityComponent(pub DVec3);

impl Component for VelocityComponent {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

impl Default for VelocityComponent {
    fn default() -> Self {
        Self(glm::vec3(0.0, 0.0, 0.0))
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

/// System for resetting an entity's components
/// at the end of the tick.
pub struct ComponentResetSystem;

impl<'a> System<'a> for ComponentResetSystem {
    type SystemData = WriteStorage<'a, PositionComponent>;

    fn run(&mut self, mut positions: Self::SystemData) {
        // Ensure that position update events are not triggered
        // for this. See #81
        positions.set_event_emission(false);

        for position in (&mut positions).join() {
            position.reset();
        }

        positions.set_event_emission(true);
    }
}
