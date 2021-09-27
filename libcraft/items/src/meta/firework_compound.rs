use std::convert::TryFrom;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use serde::{Deserialize, Serialize};

use crate::utils::{bool_from_u8, bool_to_u8};

/// Contains related data to fireworks:
/// * Single explosion on firework stars.
/// * Multiple explosions and flight time on firework rockets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FireworkCompound {
    /// A single explosion, found only for firework stars.
    explosion: Option<FireworkExplosion>,

    /// A compound containing all firework rocket data.
    /// Used only for firework rockets.
    fireworks: Option<FireworkRocketData>,
}

/// Contains data related to firework rockets:
/// * List of explosions.
/// * Flight time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FireworkRocketData {
    /// The flight of the rocket (equivalent to the amount of
    /// gunpowder used to craft the rocket).
    flight: i8,

    /// List of compounds representing all the explosions the
    /// firework will cause.
    explosions: Vec<FireworkExplosion>,
}

/// Contains all data related to firework explosions:
/// * Flicker.
/// * Trail.
/// * Type.
/// * Colors.
/// * Fade colors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FireworkExplosion {
    /// True if the explosion flickers.
    #[serde(serialize_with = "bool_to_u8", deserialize_with = "bool_from_u8")]
    flicker: bool,

    /// True of the explosion has a trail.
    #[serde(serialize_with = "bool_to_u8", deserialize_with = "bool_from_u8")]
    trail: bool,

    #[serde(rename = "type")]
    typ: FireworkExplosionType,

    /// The hex color list of the primary colors of the explosion.
    /// Color computing: Red << 16 + Green << 8 + Blue
    colors: Vec<u32>,

    /// The hex color list of the primary colors of the explosion.
    /// Color computing: Red << 16 + Green << 8 + Blue
    fade_colors: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromPrimitive)]
#[serde(try_from = "u8", into = "u8")]
pub enum FireworkExplosionType {
    SmallBall = 0,
    LargeBall = 1,
    StarShaped = 2,
    CreeperShaped = 3,
    Burst = 4,
}

impl Into<u8> for FireworkExplosionType {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for FireworkExplosionType {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if let Some(val) = FromPrimitive::from_u8(value) {
            Ok(val)
        } else {
            Err("Cannot find firework explosion with the provided id!")
        }
    }
}
