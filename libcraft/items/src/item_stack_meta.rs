use crate::{Enchantment, ItemStack};
use std::convert::TryFrom;

/// All the possible meta tag groups that an
/// `ItemStack` can have.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemMetaTagCompound {
    General(GeneralCompound),
    Block(BlockCompound),
    Enchantments(EnchantmentCompound),
    AttributeModifiers,
    PotionEffects,
    Crossbows(CrossbowCompound),
    DisplayProperties(DisplayPropertiesCompound),
    WrittenBooks,
    BooksAndQuills,
    PlayerHeads,
    Fireworks,
    ArmorStandsSpawnEggs,
    FishBuckets,
    Maps,
    SuspiciousStew,
    DebugSticks,
    Compasses,
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

    // TODO BlockEntityTag. Needs implementing a bunch of Block tags.

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

/// Contains data related to crossbows:
/// * If the crossbow is charged or not.
/// * A list of charged items on the crossbow, usually one entry,
///   but it may have multiple items charged if enchanted with multishot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrossbowCompound {

    /// True if the crossbow is charged.
    charged: bool,

    /// A list of the charged projectiles if charged.
    charged_projectiles: Option<Vec<ItemStack>>

}

/// Contains data related to `ItemStack` display.
/// * Display info: title, lore (list) and hex armor color codes.
/// * Bit flags to indicate which parts of the tooltip should be hidden.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DisplayPropertiesCompound {

    /// Display info compound containing title, lore and
    /// hex armor colors.
    display: DisplayInfoCompound,

    /// Bit flags that indicate which tooltips should be hidden.
    hide_flags: Option<u8>

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
    title: String,

    /// The lore of the `ItemStack`.
    lore: Vec<String>,

    /// The hex color of the armor.
    /// Red << 16 + Green << 8 + Blue
    color: Option<u32>,

}