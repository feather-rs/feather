//! Components for entities.
//!
//! See the [entities module](crate::entities) for entity-specific
//! components.

use std::fmt::Display;

use derive_more::{Deref, DerefMut};
use libcraft::{ChunkPosition, EntityKind, Gamemode, Inventory, Particle, Position};
use serde::{Deserialize, Serialize};
use smartstring::{LazyCompact, SmartString};
use uuid::Uuid;
use vane::Component;

use crate::{events::InventorySlotUpdateEvent, WorldId};

/// The kind of an entity.
#[derive(Copy, Clone, Debug, Deref, DerefMut)]
pub struct EntityKindComponent(pub EntityKind);
impl Component for EntityKindComponent {}

/// Stores the position of an entity.
#[derive(Copy, Clone, Debug, Deref, DerefMut)]
pub struct EntityPosition(pub Position);
impl Component for EntityPosition {}

/// Stores the current chunk of an entity.
#[derive(Copy, Clone, Debug, Deref, DerefMut)]
pub struct EntityChunk(pub ChunkPosition);
impl Component for EntityChunk {}

/// Stores a player's gamemode.
#[derive(Copy, Clone, Debug, Deref, DerefMut)]
pub struct PlayerGamemode(pub Gamemode);
impl Component for PlayerGamemode {}

/// An entity's UUID.
///
/// For players, this is the UUID of the player account when
/// the server is in online mode.
#[derive(Copy, Clone, Debug, Deref, DerefMut)]
pub struct EntityUuid(pub Uuid);
impl Component for EntityUuid {}

/// Whether an entity is touching the ground.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Deref, DerefMut)]
pub struct OnGround(pub bool);
impl Component for OnGround {}

/// A player's username.
///
/// This component is immutable. Do not
/// attempt to change it.
///
/// Non-player entities cannot have this component. See [`CustomName`]
/// if you need to name an entity.
#[derive(Clone, Debug, Serialize, Deserialize, derive_more::Deref)]
pub struct Name(SmartString<LazyCompact>);
impl Component for Name {}

impl Name {
    pub fn new(string: &str) -> Self {
        Self(string.into())
    }

    pub fn as_str(&self) -> &str {
        &*self
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// An entity's custom name.
///
/// Adding this component to an entity
/// will give it a custom name, visible on the client.
///
/// Giving a player a custom name has no effect.
#[derive(Clone, Debug, Serialize, Deserialize, derive_more::Deref, derive_more::DerefMut)]
pub struct CustomName(SmartString<LazyCompact>);
impl Component for CustomName {}

impl CustomName {
    /// Creates a custom name from a string.
    pub fn new(string: &str) -> Self {
        Self(string.into())
    }

    pub fn as_str(&self) -> &str {
        &*self
    }

    pub fn as_mut_str(&mut self) -> &mut str {
        &mut *self
    }
}

impl Display for CustomName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// A player's walk speed
#[derive(
    Copy, Clone, Debug, PartialEq, Serialize, Deserialize, derive_more::Deref, derive_more::DerefMut,
)]
pub struct WalkSpeed(pub f32);
impl Component for WalkSpeed {}

impl Default for WalkSpeed {
    fn default() -> Self {
        WalkSpeed(0.1)
    }
}

/// A player's fly speed
#[derive(
    Copy, Clone, Debug, PartialEq, Serialize, Deserialize, derive_more::Deref, derive_more::DerefMut,
)]
pub struct CreativeFlyingSpeed(pub f32);
impl Component for CreativeFlyingSpeed {}

impl Default for CreativeFlyingSpeed {
    fn default() -> Self {
        CreativeFlyingSpeed(0.05)
    }
}

/// Whether a player can fly like in creative mode
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct CanCreativeFly(pub bool);
impl Component for CanCreativeFly {}

/// Whether a player is flying (like in creative mode, so it does not reflect if the player is flying by other means)
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct CreativeFlying(pub bool);
impl Component for CreativeFlying {}

/// Whether a player can place and destroy blocks
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct CanBuild(pub bool);
impl Component for CanBuild {}

/// Whether a player breaks blocks instantly (like in creative mode)
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct Instabreak(pub bool);
impl Component for Instabreak {}

/// Whether a player is immune to damage
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct Invulnerable(pub bool);
impl Component for Invulnerable {}

/// Whether an entity is sneaking, like in pressing shift.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct Sneaking(pub bool);
impl Component for Sneaking {}

/// A player's previous gamemode
#[derive(
    Copy,
    Clone,
    Debug,
    Default,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct PreviousGamemode(pub Option<Gamemode>);
impl Component for PreviousGamemode {}

impl PreviousGamemode {
    /// Gets a previous gamemode from its ID.
    pub fn from_id(id: i8) -> Self {
        PreviousGamemode(match id {
            0 => Some(Gamemode::Survival),
            1 => Some(Gamemode::Creative),
            2 => Some(Gamemode::Adventure),
            3 => Some(Gamemode::Spectator),
            _ => None,
        })
    }

    /// Gets this gamemode's id
    pub fn id(&self) -> i8 {
        match self.0 {
            Some(Gamemode::Survival) => 0,
            Some(Gamemode::Creative) => 1,
            Some(Gamemode::Adventure) => 2,
            Some(Gamemode::Spectator) => 3,
            None => -1,
        }
    }
}

/// Represents an entity's health
#[derive(
    Copy, Clone, Debug, PartialEq, Serialize, Deserialize, derive_more::Deref, derive_more::DerefMut,
)]
pub struct Health(pub f32);
impl Component for Health {}

/// A component on players that tracks if they are sprinting or not.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct Sprinting(pub bool);
impl Component for Sprinting {}

impl Sprinting {
    pub fn new(value: bool) -> Self {
        Sprinting(value)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, derive_more::Deref, derive_more::DerefMut)]
pub struct EntityWorld(pub WorldId);
impl Component for EntityWorld {}

/// An entity's inventory.
#[derive(Deref, DerefMut, Debug)]
pub struct EntityInventory(pub Inventory);

impl EntityInventory {
    pub fn new(inventory: Inventory) -> Self {
        Self(inventory)
    }
}

impl Component for EntityInventory {
    fn on_inserted(&mut self, ecs: &vane::Entities, owner: vane::Entity) {
        let bus = ecs.bus();
        self.0.set_slot_mutated_callback(move |area, slot| {
            let event = InventorySlotUpdateEvent {
                entity: owner,
                area,
                slot,
            };
            bus.defer(|ecs| ecs.insert_event(event));
        });
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct EntityParticle(pub Particle);
impl Component for EntityParticle {}
