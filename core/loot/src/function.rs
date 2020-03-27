use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::{ItemStack, Block, text::Text};
use super::ExactOrRandom;

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Zoom(usize);
impl Default for Zoom {
    fn default() -> Self {
        Zoom(2)
    }
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct SearchRadius(usize);
impl Default for SearchRadius {
    fn default() -> Self {
        SearchRadius(50)
    }
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct SkipExistingChunks(bool);
impl Default for SkipExistingChunks {
    fn default() -> Self {
        SkipExistingChunks(true)
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Operation {
    Addition,
    MultiplyBase,
    MultiplyTotal,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Slot {
    Mainhand,
    Offhand,
    Head,
    Chest,
    Legs,
    Feet,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "name", rename_all = "snake_case")]
pub struct Modifier {
    name: String,
    attribute: String,
    operation: Operation,
    amount: ExactOrRandom<f32>,
    id: Uuid,
    slot: Slot,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize)]
#[serde(tag = "function", rename_all = "snake_case")]
pub enum Function {
    #[serde(alias = "minecraft:enchant_randomly")]
    EnchantRandomly {
        enchantments: Option<Vec<String>>
    },
    EnchantWithLevels {
        treasure: bool,
        levels: ExactOrRandom<usize>,
    },
    #[serde(alias = "minecraft:exploration_map")]
    ExplorationMap {
        destination: String,
        decoration: String,
        #[serde(default, skip_serializing_if = "is_default")] 
        zoom: Zoom,
        #[serde(default, skip_serializing_if = "is_default")] 
        search_radius: SearchRadius,
        #[serde(default, skip_serializing_if = "is_default")] 
        skip_existing_chunks: SkipExistingChunks,
    },
    FurnaceSmelt {},
    LootingEnchant {
        count: ExactOrRandom<usize>,
    },
    SetAttributes {
        modifiers: Vec<Modifier>
    },
    #[serde(alias = "minecraft:set_count")]
    SetCount {
        count: ExactOrRandom<isize>,
    },
    SetDamage {
        damage: ExactOrRandom<f32>,
    },
    SetName {
        name: Text,
    },
    SetNbt {
        tag: String,
    }
}

pub trait Apply<T> {
    fn apply(&self, item: ItemStack, _: T) -> ItemStack;
}

impl Apply<Block> for Function {
    fn apply(&self, item: ItemStack, block: Block) -> ItemStack {
        match self {
            _ => item
        }
    }
}

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}