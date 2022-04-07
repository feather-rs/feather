use serde::ser::Error;
use serde::{Deserialize, Serialize, Serializer};

use super::player::InventorySlot;

/// A block entity loaded or saved to the Anvil format.
/// Should be serialized using NBT.
///
/// <https://minecraft.gamepedia.com/Chunk_format#Block_entity_format>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockEntityData {
    #[serde(flatten)]
    pub base: BlockEntityBase,
    #[serde(flatten)]
    pub kind: BlockEntityKind,
}

/// Data common to all block entities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockEntityBase {
    /// X coordinate in global coordinate space.
    pub x: i32,
    /// Y coordinate in global space.
    pub y: i32,
    /// Z coordinate in global space.
    pub z: i32,
}

/// Kind of a block entity.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "id")]
pub enum BlockEntityKind {
    #[serde(rename = "minecraft:beacon")]
    #[serde(rename_all = "PascalCase")]
    Beacon {
        levels: i32,
        primary: i32,
        secondary: i32,
    },
    #[serde(rename = "minecraft:bed")]
    #[serde(rename_all = "PascalCase")]
    Bed, // empty in JE
    #[serde(rename = "minecraft:brewing_stand")]
    #[serde(rename_all = "PascalCase")]
    BrewingStand {
        #[serde(default)]
        items: Vec<InventorySlot>,
        brew_time: i16,
        fuel: i8,
    },
    #[serde(rename = "minecraft:cauldron")]
    #[serde(rename_all = "PascalCase")]
    Cauldron {
        #[serde(default)]
        items: Vec<InventorySlot>,
        potion_id: i16,
        splash_potion: bool,
        is_movable: bool,
    },
    #[serde(rename = "minecraft:chest")]
    #[serde(rename_all = "PascalCase")]
    Chest {
        #[serde(default)]
        items: Vec<InventorySlot>,
        loot_table: Option<String>,
        loot_table_seed: Option<i64>,
    },
    #[serde(rename = "minecraft:comparator")]
    #[serde(rename_all = "PascalCase")]
    Comparator { output_signal: i32 },
    #[serde(rename = "minecraft:command_block")]
    #[serde(rename_all = "PascalCase")]
    CommandBlock {
        custom_name: Option<String>,
        command: String,
        success_count: i32,
        last_output: String,
        track_output: bool,
        powered: bool,
        auto: bool,
        condition_met: bool,
        update_last_execution: bool,
        last_execution: i64,
    },
    #[serde(rename = "minecraft:daylight_detector")]
    #[serde(rename_all = "PascalCase")]
    DaylightDetector, // empty
    #[serde(rename = "minecraft:dispenser")]
    #[serde(rename_all = "PascalCase")]
    Dispenser {
        #[serde(default)]
        items: Vec<InventorySlot>,
    },
    #[serde(rename = "minecraft:dropper")]
    #[serde(rename_all = "PascalCase")]
    Dropper {
        #[serde(default)]
        items: Vec<InventorySlot>,
    },
    #[serde(rename = "minecraft:enchanting_table")]
    #[serde(rename_all = "PascalCase")]
    EnchantingTable,
    #[serde(rename = "minecraft:ender_chest")]
    #[serde(rename_all = "PascalCase")]
    EnderChest,
    #[serde(rename = "minecraft:end_gateway")]
    #[serde(rename_all = "PascalCase")]
    EndGateway { age: i64, exact_teleport: bool },
    #[serde(rename = "minecraft:end_portal")]
    #[serde(rename_all = "PascalCase")]
    EndPortal,
    #[serde(rename = "minecraft:furnace")]
    #[serde(rename_all = "PascalCase")]
    Furnace {
        #[serde(default)]
        items: Vec<InventorySlot>,
        burn_time: i16,
        cook_time: i16,
        cook_time_total: i16,
    },
    #[serde(rename = "minecraft:hopper")]
    #[serde(rename_all = "PascalCase")]
    Hopper {
        #[serde(default)]
        items: Vec<InventorySlot>,
        transfer_cooldown: i32,
    },
    #[serde(rename = "minecraft:jigsaw")]
    #[serde(rename_all = "PascalCase")]
    Jigsaw {
        target_pool: String,
        final_state: String,
        /// spelled "attachement" on the wiki,
        /// but mispelling is probably a mistake?
        attachment_type: String,
    },
    #[serde(rename = "minecraft:jukebox")]
    #[serde(rename_all = "PascalCase")]
    Jukebox {
        #[serde(default)]
        record_item: InventorySlot,
    },
    // TODO: a few more
    /// Fallback type for unknown block entities
    #[serde(other, serialize_with = "BlockEntityKind::serialize_unknown")]
    Unknown,
}

impl BlockEntityKind {
    pub(crate) fn serialize_unknown<S: Serializer>(_serializer: S) -> Result<S::Ok, S::Error> {
        Err(S::Error::custom("cannot serialize unknown block entities"))
    }

    pub fn variant(&self) -> BlockEntityVariant {
        match self {
            BlockEntityKind::Beacon { .. } => BlockEntityVariant::Beacon,
            BlockEntityKind::Bed { .. } => BlockEntityVariant::Bed,
            BlockEntityKind::BrewingStand { .. } => BlockEntityVariant::BrewingStand,
            BlockEntityKind::Cauldron { .. } => BlockEntityVariant::Cauldron,
            BlockEntityKind::Comparator { .. } => BlockEntityVariant::Comparator,
            BlockEntityKind::CommandBlock { .. } => BlockEntityVariant::CommandBlock,
            BlockEntityKind::Chest { .. } => BlockEntityVariant::Chest,
            BlockEntityKind::DaylightDetector { .. } => BlockEntityVariant::DaylightDetector,
            BlockEntityKind::Dispenser { .. } => BlockEntityVariant::Dispenser,
            BlockEntityKind::Dropper { .. } => BlockEntityVariant::Dropper,
            BlockEntityKind::EnchantingTable { .. } => BlockEntityVariant::EnchantingTable,
            BlockEntityKind::EnderChest { .. } => BlockEntityVariant::EnderChest,
            BlockEntityKind::EndGateway { .. } => BlockEntityVariant::EndGateway,
            BlockEntityKind::EndPortal { .. } => BlockEntityVariant::EndPortal,
            BlockEntityKind::Furnace { .. } => BlockEntityVariant::Furnace,
            BlockEntityKind::Hopper { .. } => BlockEntityVariant::Hopper,
            BlockEntityKind::Jigsaw { .. } => BlockEntityVariant::Jigsaw,
            BlockEntityKind::Jukebox { .. } => BlockEntityVariant::Jukebox,
            BlockEntityKind::Unknown { .. } => BlockEntityVariant::Unknown,
        }
    }
}

/// Variant of a `BlockEntityKind`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BlockEntityVariant {
    Beacon,
    Bed,
    BrewingStand,
    Cauldron,
    Chest,
    Comparator,
    CommandBlock,
    DaylightDetector,
    Dispenser,
    Dropper,
    EnchantingTable,
    EnderChest,
    EndGateway,
    EndPortal,
    Furnace,
    Hopper,
    Jigsaw,
    Jukebox,
    Unknown,
}
