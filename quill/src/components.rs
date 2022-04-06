//! Components not associated with a specific type of entity.
//!
//! See the [entities module](crate::entities) for entity-specific
//! components.

use std::fmt::Display;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use smartstring::{LazyCompact, SmartString};

#[doc(inline)]
pub use libcraft_core::{Gamemode, Position, ChunkPosition};

/// Whether an entity is touching the ground.
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
pub struct OnGround(pub bool);

/// A player's username.
///
/// This component is immutable. Do not
/// attempt to change it.
///
/// Non-player entities cannot have this component. See [`CustomName`]
/// if you need to name an entity.
#[derive(Clone, Debug, Serialize, Deserialize, derive_more::Deref)]
pub struct Name(SmartString<LazyCompact>);

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
impl Sprinting {
    pub fn new(value: bool) -> Self {
        Sprinting(value)
    }
}

#[derive(
    Clone,
    PartialEq,
    Eq,
    Hash,
    Debug,
    derive_more::Deref,
    derive_more::DerefMut,
    Serialize,
    Deserialize,
)]
pub struct EntityDimension(pub String);

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug, derive_more::Deref, derive_more::DerefMut)]
pub struct EntityWorld(pub vane::Entity);
