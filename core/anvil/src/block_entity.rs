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
pub enum BlockEntityKind {
    #[serde(rename = "minecraft:banner")]
    #[serde(rename_all = "PascalCase")]
    Banner {
        // TODO
    },
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
        items: Vec<InventorySlot>,
        brew_time: i16,
        fuel: i8,
    },
    #[serde(rename = "minecraft:cauldron")]
    #[serde(rename_all = "PascalCase")]
    Cauldron {
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
    Dispenser { items: Vec<InventorySlot> },
    #[serde(rename = "minecraft:dropper")]
    #[serde(rename_all = "PascalCase")]
    Dropper { items: Vec<InventorySlot> },
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
        items: Vec<InventorySlot>,
        burn_time: i16,
        cook_time: i16,
        cook_time_total: i16,
    },
    #[serde(rename = "minecraft:hopper")]
    #[serde(rename_all = "PascalCase")]
    Hopper {
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
    Jukebox { record_item: InventorySlot },
    #[serde(rename = "minecraft:mob_spawner")]
    #[serde(rename_all = "PascalCase")]
    MobSpawner {
        // TODO
    },
    // TODO: a few more
}

impl BlockEntityKind {
    pub fn variant(&self) -> BlockEntityVariant {
        match self {
            BlockEntityKind::Banner { .. } => BlockEntityVariant::Banner,
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
            BlockEntityKind::MobSpawner { .. } => BlockEntityVariant::MobSpawner,
        }
    }
}

/// Variant of a `BlockEntityKind`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum BlockEntityVariant {
    Banner,
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
    MobSpawner,
}
