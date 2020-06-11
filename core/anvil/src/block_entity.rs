use crate::player::InventorySlot;
use nbt::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A block entity loaded or saved to the Anvil format.
/// Should be serialized using NBT.
///
/// https://minecraft.gamepedia.com/Chunk_format#Block_entity_format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockEntityData {
    #[serde(flatten)]
    pub base: BlockEntityBase,
    #[serde(flatten)]
    pub kind: BlockEntityKind,
}

impl BlockEntityData {
    pub fn into_nbt_value(self) -> Value {
        let mut compound = HashMap::new();

        compound.insert(String::from("x"), Value::Int(self.base.x));
        compound.insert(String::from("y"), Value::Int(self.base.y));
        compound.insert(String::from("z"), Value::Int(self.base.z));

        let id = match self.kind {
            BlockEntityKind::Chest { items, .. } => {
                compound.insert(
                    String::from("Items"),
                    Value::List(
                        items
                            .into_iter()
                            .map(InventorySlot::into_nbt_value)
                            .collect(),
                    ),
                );
                "minecraft:chest"
            }
            _ => todo!("implement block entity into_nbt_value"),
        };
        compound.insert(String::from("id"), Value::String(String::from(id)));

        Value::Compound(compound)
    }
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
#[serde(rename_all = "PascalCase")]
pub enum BlockEntityKind {
    #[serde(rename = "minecraft:beacon")]
    Beacon {
        levels: i32,
        primary: i32,
        secondary: i32,
    },
    #[serde(rename = "minecraft:bed")]
    Bed, // empty in JE
    #[serde(rename = "minecraft:brewing_stand")]
    BrewingStand {
        items: Vec<InventorySlot>,
        brew_time: i16,
        fuel: i8,
    },
    #[serde(rename = "minecraft:cauldron")]
    Cauldron {
        items: Vec<InventorySlot>,
        potion_id: i16,
        splash_potion: bool,
        is_movable: bool,
    },
    #[serde(rename = "minecraft:chest")]
    Chest {
        #[serde(rename = "Items")]
        items: Vec<InventorySlot>,
        loot_table: Option<String>,
        loot_table_seed: Option<i64>,
    },
    #[serde(rename = "minecraft:comparator")]
    Comparator { output_signal: i32 },
    #[serde(rename = "minecraft:command_block")]
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
    DaylightDetector, // empty
    #[serde(rename = "minecraft:dispenser")]
    Dispenser { items: Vec<InventorySlot> },
    #[serde(rename = "minecraft:dropper")]
    Dropper { items: Vec<InventorySlot> },
    #[serde(rename = "minecraft:enchanting_table")]
    EnchantingTable,
    #[serde(rename = "minecraft:ender_chest")]
    EnderChest,
    #[serde(rename = "minecraft:end_gateway")]
    EndGateway { age: i64, exact_teleport: bool },
    #[serde(rename = "minecraft:end_portal")]
    EndPortal,
    #[serde(rename = "minecraft:furnace")]
    Furnace {
        items: Vec<InventorySlot>,
        burn_time: i16,
        cook_time: i16,
        cook_time_total: i16,
    },
    #[serde(rename = "minecraft:hopper")]
    Hopper {
        items: Vec<InventorySlot>,
        transfer_cooldown: i32,
    },
    #[serde(rename = "minecraft:jigsaw")]
    Jigsaw {
        target_pool: String,
        final_state: String,
        /// spelled "attachement" on the wiki,
        /// but mispelling is probably a mistake?
        attachment_type: String,
    },
    #[serde(rename = "minecraft:jukebox")]
    Jukebox { record_item: InventorySlot },
    // TODO: a few more
}

impl BlockEntityKind {
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
}
