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
use std::cmp::Ordering;
use std::collections::BTreeSet;

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

#[derive(Copy, Clone, Debug, Hash, Serialize, PartialEq, Eq, Deserialize)]
pub struct PotionApplication {
    pub amplifier: u8,
    pub duration: u32,
    pub particle: bool,
    pub ambient: bool, // given from beacon or not.
    pub icon: bool,    // show effect icon.
}

impl Ord for PotionApplication {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.amplifier > other.amplifier || self.duration > other.duration {
            Ordering::Greater
        } else if self.amplifier == self.amplifier || self.duration == other.duration {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}
impl PartialOrd for PotionApplication {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

macro_rules! impl_effect {
    ($ident:ident) => {
        #[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
        pub struct $ident(BTreeSet<PotionApplication>);
        impl $ident {}
        bincode_component_impl!($ident);
    };
}

impl_effect!(Speed);
impl_effect!(Slowness);
impl_effect!(Haste);
impl_effect!(MiningFatigue);
impl_effect!(Strength);
impl_effect!(InstantHealth);
impl_effect!(InstantDamage);
impl_effect!(JumpBoost);
impl_effect!(Nausea);
impl_effect!(Regeneration);
impl_effect!(Resistance);
impl_effect!(FireResistance);
impl_effect!(WaterBreathing);
impl_effect!(Invisibility);
impl_effect!(Blindness);
impl_effect!(NightVision);
impl_effect!(Hunger);
impl_effect!(Weakness);
impl_effect!(Poison);
impl_effect!(WitherEffect);
impl_effect!(HealthBoost);
impl_effect!(Absorption);
impl_effect!(Saturation);
impl_effect!(Glowing);
impl_effect!(Levitation);
impl_effect!(Luck);
impl_effect!(BadLuck);
impl_effect!(SlowFalling);
impl_effect!(ConduitPower);
impl_effect!(DolphinsGrace);
impl_effect!(BadOmen);
impl_effect!(HeroOfTheVillage);

/// Wheather an entity is sneaking, like in pressing shift.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Sneaking(pub bool);
bincode_component_impl!(Sneaking);
