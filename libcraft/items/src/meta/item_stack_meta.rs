use crate::{Enchantment, ItemStack};
use std::convert::{Into, TryFrom};

use serde::{Deserialize, Serialize};

use libcraft_core::BlockPosition;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::utils::{bool_from_u8, bool_to_u8};

/// Represents the metadata of an `ItemStack`. Can contain
/// multiple `ItemMetaTagCompounds`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemStackMeta {
    /// All the tag compounds found in this enum.
    pub(crate) compounds: Vec<ItemMetaTagCompound>,
}

/// All the possible meta tag groups that an
/// `ItemStack` can have.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemMetaTagCompound {
    General(GeneralCompound),
    Block(BlockCompound),
    Enchantments(EnchantmentCompound),
    AttributeModifiers,
    PotionEffects(PotionEffectCompound),
    Crossbows(CrossbowCompound),
    DisplayProperties(DisplayPropertiesCompound),
    WrittenBooks(WrittenBookCompound),
    BooksAndQuills(BookAndQuillCompound),
    PlayerHeads,
    Fireworks(FireworkCompound),
    ArmorStandsSpawnEggs(ArmorStandSpawnEggCompound),
    FishBuckets(BucketOfFishCompound),
    Maps(MapCompound),
    SuspiciousStew(SuspiciousStewCompound),
    DebugSticks(DebugStickCompound),
    Compasses(CompassCompound),
}

/// Contains the general NBT tags.
/// * Damage to the item
/// * If the item is unbreakable
/// * A list of blocks that the item can destroy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GeneralCompound {
    /// The damage done to the item.
    damage: Option<u32>,

    /// If the item is unbreakable or not.
    unbreakable: Option<bool>,

    /// List of blocks that the item can destroy in
    /// adventure mode.
    can_destroy: Option<Vec<String>>,
    // TODO Investigate what CustomModelData is.
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockCompound {
    can_place_on: Option<Vec<String>>,
    // TODO BlockEntityTag.
    // TODO BlockStateTag. Connect with block_data?
}

/// Contains all data related to enchantments and the
/// `ItemStack`:
/// * Enchantments applied.
/// * Enchantments stored in the case of enchanted books.
/// * Repair cost of repairing the item. (Number of XP levels to add)
///   to the base level cost when repairing, combining or renaming.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnchantmentCompound {
    /// The applied enchantments to the `ItemStack`.
    enchantments: Option<Vec<Enchantment>>,

    /// The stored enchantments if this tag is assigned to
    /// enchanted books.
    stored_enchantments: Option<Vec<Enchantment>>,

    /// Repair cost of this item, if applicable.
    repair_cost: Option<u32>,
}

// TODO Generate potion effects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PotionEffectCompound {}

/// Contains data related to crossbows:
/// * If the crossbow is charged or not.
/// * A list of charged items on the crossbow, usually one entry,
///   but it may have multiple items charged if enchanted with multishot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrossbowCompound {
    /// True if the crossbow is charged.
    #[serde(serialize_with = "bool_to_u8", deserialize_with = "bool_from_u8")]
    charged: bool,

    /// A list of the charged projectiles if charged.
    charged_projectiles: Option<Vec<ItemStack>>,
}

/// Contains data related to `ItemStack` display.
/// * Display info: title, lore (list) and hex armor color codes.
/// * Bit flags to indicate which parts of the tooltip should be hidden.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisplayPropertiesCompound {
    /// Display info compound containing title, lore and
    /// hex armor colors.
    display: DisplayInfoCompound,

    /// Bit flags that indicate which tooltips should be hidden.
    #[serde(rename = "HideFlags")]
    hide_flags: Option<u8>,
}

bitflags! {
    struct HideToolTipFlags: u8 {
        const ENCHANTMENTS        = 0b00000001;
        const ATTRIBUTE_MODIFIERS = 0b00000010;
        const UNBREAKABLE         = 0b00000100;
        const CAN_DESTROY         = 0b00001000;
        const CAN_PLACE_ON        = 0b00010000;
        const OTHER               = 0b00100000;
        const DYE                 = 0b01000000;
    }
}

/// Contains info data to be displayed about the `ItemStack`:
/// * The title of the `ItemStack`.
/// * The lore of the `ItemStack`.
/// * The hex color code of the armor to be displayed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DisplayInfoCompound {
    /// The title of the `ItemStack`.
    name: String,

    /// The lore of the `ItemStack`.
    lore: Vec<String>,

    /// The hex color of the armor.
    /// Red << 16 + Green << 8 + Blue
    #[serde(rename = "color")]
    color: Option<u32>,
}

/// Contains data related to written books:
/// * If the book data has been resolved.
/// * The copy tier of the written book.
/// * The title of the written book.
/// * The author of the written book.
/// * The pages of the written book.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WrittenBookCompound {
    /// Set to 1 if the book has been opened.
    resolved: Option<u8>,

    /// The copy tier of the written book.
    generation: Option<BookCopyTier>,

    /// The title of the written book.
    title: String,

    /// The author of the written book.
    author: String,

    /// A list of all pages serialized as JSON.
    pages: Vec<String>,
}

/// Enum containing all copy tiers possible in a written
/// book.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromPrimitive)]
#[serde(try_from = "u32", into = "u32")]
pub enum BookCopyTier {
    Original = 0,
    CopyOfOriginal = 1,
    CopyOfCopy = 2,
    Tattered = 3,
}

impl Into<u32> for BookCopyTier {
    fn into(self) -> u32 {
        self as u32
    }
}

impl TryFrom<u32> for BookCopyTier {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if let Some(val) = FromPrimitive::from_u32(value) {
            Ok(val)
        } else {
            Err("Cannot find a book copy tier with the provided value!")
        }
    }
}

/// Contains related data to books and quills:
/// * The pages of the book and quill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookAndQuillCompound {
    /// A list of all pages serialized as JSON.
    pages: Vec<String>,
}

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

// TODO Implement entities in order to store Entity tags.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArmorStandSpawnEggCompound {}

// TODO Implement entities in order to store Entity tags.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BucketOfFishCompound {}

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

// TODO Implement effects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SuspiciousStewCompound {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DebugStickCompound {}

/// Contains metadata related to compasses:
/// * If the compass is tracking a lodestone.
/// * The dimension of the lodestone if applicable.
/// * The position of the lodestone if applicable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CompassCompound {
    /// True if the compass is tracking a lodestone.
    #[serde(serialize_with = "bool_to_u8", deserialize_with = "bool_from_u8")]
    lodestone_tracked: bool,

    // TODO Change from Option<String> to Option<Dimension> where Dimension is a custom enum.
    /// The dimension where the lodestone is found.
    /// Only Some if the compass is tracking a lodestone.
    lodestone_dimension: Option<String>,

    /// The position of the lodestone.
    lodestone_position: Option<BlockPosition>,
}
