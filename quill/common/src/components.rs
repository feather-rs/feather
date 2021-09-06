//! Components not associated with a specific type of entity.
//!
//! See the [entities module](crate::entities) for entity-specific
//! components.

use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};
use smartstring::{LazyCompact, SmartString};

/// Whether an entity is touching the ground.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OnGround(pub bool);

bincode_component_impl!(OnGround);

/// A player's username.
///
/// This component is immutable. Do not
/// attempt to change it.
///
/// Non-player entities cannot have this component. See [`CustomName`]
/// if you need to name an entity.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Name(SmartString<LazyCompact>);

bincode_component_impl!(Name);

impl Name {
    pub fn new(string: &str) -> Self {
        Self(string.into())
    }

    pub fn as_str(&self) -> &str {
        &*self
    }
}

impl Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
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
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomName(SmartString<LazyCompact>);

bincode_component_impl!(CustomName);

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

impl Deref for CustomName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CustomName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for CustomName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Whether an entity is flying (like in creative mode, so it does not reflect if the player is flying by other means)
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CreativeFlying(pub bool);

bincode_component_impl!(CreativeFlying);

/// Wheather an entity is sneaking, like in pressing shift.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Sneaking(pub bool);
bincode_component_impl!(Sneaking);

/// A component on players that tracks if they are sprinting or not.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Sprinting(pub bool);
impl Sprinting {
    pub fn new(value: bool) -> Self {
        Sprinting(value)
    }
}
bincode_component_impl!(Sprinting);
