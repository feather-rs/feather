use std::convert::TryFrom;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use serde::{Deserialize, Serialize};

/// Contains all data related to Maps:
/// * The map number.
/// * The map scale direction.
/// * The map decorations.
/// * The display compound of the map.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapCompound {
    /// The map number.
    map: u32,

    /// The map scale direction. Usually 1.
    map_scale_direction: u32,

    /// The map decorations.
    #[serde(rename = "Decorations")]
    decorations: Vec<MapDecorationCompound>,

    /// The display tag compound of the map.
    display: MapDisplayCompound,
}

/// Contains all data related to Map decorations:
/// * The unique id of the decoration.
/// * The type of the decoration.
/// * The position of the decoration in the map.
/// * The rotation of the decoration. (0 displays the icon
///   upside down).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapDecorationCompound {
    /// The unique id of the decoration.
    id: String,

    /// The type of the decoration.
    #[serde(rename = "type")]
    typ: MapDecorationIcon,

    /// The x coordinate of the decoration in the world.
    x: f64,

    /// The z coordinate of the decoration in the world.
    z: f64,

    /// The rotation in degrees of the decoration (clockwise).
    rot: f32,
}

impl Eq for MapDecorationCompound {}

/// All possible decoration items in Java Edition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromPrimitive)]
#[serde(try_from = "u8", into = "u8")]
pub enum MapDecorationIcon {
    WhiteMarker = 0,
    GreenMarker = 1,
    RedMarker = 2,
    BlueMarker = 3,
    TargetX = 4,
    TargetPoint = 5,
    LargeWhiteDot = 6,
    SmallWhiteDot = 7,
    Mansion = 8,
    Monument = 9,
    BannerWhite = 10,
    BannerOrange = 11,
    BannerMagenta = 12,
    BannerLightBlue = 13,
    BannerYellow = 14,
    BannerLime = 15,
    BannerPink = 16,
    BannerGray = 17,
    BannerLightGray = 18,
    BannerCyan = 19,
    BannerPurple = 20,
    BannerBlue = 21,
    BannerBrown = 22,
    BannerGreen = 23,
    BannerRed = 24,
    BannerBlack = 25,
    RedX = 26,
}

impl Into<u8> for MapDecorationIcon {
    fn into(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for MapDecorationIcon {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if let Some(val) = FromPrimitive::from_u8(value) {
            Ok(val)
        } else {
            Err("Cannot find a map decoration icon with the provided id!")
        }
    }
}

/// Contains map display info:
/// * The color of the markings on the item's texture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MapDisplayCompound {
    /// The color of the markings on the item's texture.
    map_color: u32,
}
