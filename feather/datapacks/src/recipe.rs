use std::{collections::HashMap, fs::File, io::Read, path::Path};

use crate::{NamespacedId, SerdeItem, SerdeItemStack, TagRegistry};
use generated::{Item, ItemStack};
use serde::{Deserialize, Serialize};
use smartstring::{Compact, SmartString};
/// A registry for keeping track of recipes.
#[derive(Clone, Debug, Default)]
pub struct RecipeRegistry {
    pub blast: Vec<BlastingRecipe>,
    pub camp: Vec<CampfireRecipe>,
    pub shaped: Vec<ShapedRecipe>,
    pub shapeless: Vec<ShapelessRecipe>,
    pub smelt: Vec<SmeltingRecipe>,
    pub smith: Vec<SmithingRecipe>,
    pub smoke: Vec<SmokingRecipe>,
    pub stone: Vec<StonecuttingRecipe>,
}
impl RecipeRegistry {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn from_dir(path: &Path) -> Result<Self, crate::RecipeLoadError> {
        let mut this = Self::new();
        this.add_from_dir(path)?;
        Ok(this)
    }
    pub fn add_from_dir(&mut self, path: &Path) -> Result<(), crate::RecipeLoadError> {
        for file in std::fs::read_dir(path)? {
            let path = file?.path();
            log::trace!("{}", path.to_string_lossy());
            match Recipe::from_file(&path)? {
                Recipe::Blasting(recipe) => self.blast.push(recipe),
                Recipe::Campfire(recipe) => self.camp.push(recipe),
                Recipe::Shaped(recipe) => self.shaped.push(recipe),
                Recipe::Shapeless(recipe) => self.shapeless.push(recipe),
                Recipe::Smelting(recipe) => self.smelt.push(recipe),
                Recipe::Smithing(recipe) => self.smith.push(recipe),
                Recipe::Smoking(recipe) => self.smoke.push(recipe),
                Recipe::Stonecutting(recipe) => self.stone.push(recipe),
                Recipe::Special => {}
            }
        }
        Ok(())
    }
    pub fn match_blasting(&self, item: Item, tag_registry: &TagRegistry) -> Option<(Item, f32)> {
        self.blast
            .iter()
            .find_map(|r| r.match_self(item, tag_registry))
    }
    pub fn match_campfire_cooking(
        &self,
        item: Item,
        tag_registry: &TagRegistry,
    ) -> Option<(Item, f32)> {
        self.camp
            .iter()
            .find_map(|r| r.match_self(item, tag_registry))
    }
    pub fn match_shapeless<'a>(
        &self,
        items: impl Iterator<Item = &'a Item>,
        tag_registry: &TagRegistry,
    ) -> Option<ItemStack> {
        let items: Vec<Item> = items.copied().collect();
        self.shapeless
            .iter()
            .find_map(|r| r.match_self(items.iter(), tag_registry))
    }
    pub fn match_smelting(&self, item: Item, tag_registry: &TagRegistry) -> Option<(Item, f32)> {
        self.smelt
            .iter()
            .find_map(|r| r.match_self(item, tag_registry))
    }
    pub fn match_smithing(
        &self,
        base: Item,
        addition: Item,
        tag_registry: &TagRegistry,
    ) -> Option<Item> {
        self.smith
            .iter()
            .find_map(|r| r.match_self(base, addition, tag_registry))
    }
    pub fn match_smoking(&self, item: Item, tag_registry: &TagRegistry) -> Option<(Item, f32)> {
        self.smoke
            .iter()
            .find_map(|r| r.match_self(item, tag_registry))
    }
    pub fn match_stonecutting(&self, item: Item, tag_registry: &TagRegistry) -> Option<ItemStack> {
        self.stone
            .iter()
            .find_map(|r| r.match_self(item, tag_registry))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Recipe {
    #[serde(rename = "minecraft:blasting")]
    Blasting(BlastingRecipe),
    #[serde(rename = "minecraft:campfire_cooking")]
    Campfire(CampfireRecipe),
    #[serde(rename = "minecraft:crafting_shaped")]
    Shaped(ShapedRecipe),
    #[serde(rename = "minecraft:crafting_shapeless")]
    Shapeless(ShapelessRecipe),
    #[serde(rename = "minecraft:smelting")]
    Smelting(SmeltingRecipe),
    #[serde(rename = "minecraft:smithing")]
    Smithing(SmithingRecipe),
    #[serde(rename = "minecraft:smoking")]
    Smoking(SmokingRecipe),
    #[serde(rename = "minecraft:stonecutting")]
    Stonecutting(StonecuttingRecipe),
    #[serde(alias = "minecraft:crafting_special_armordye")]
    #[serde(alias = "minecraft:crafting_special_bannerduplicate")]
    #[serde(alias = "minecraft:crafting_special_bookcloning")]
    #[serde(alias = "minecraft:crafting_special_firework_rocket")]
    #[serde(alias = "minecraft:crafting_special_firework_star")]
    #[serde(alias = "minecraft:crafting_special_firework_star_fade")]
    #[serde(alias = "minecraft:crafting_special_mapcloning")]
    #[serde(alias = "minecraft:crafting_special_mapextending")]
    #[serde(alias = "minecraft:crafting_special_repairitem")]
    #[serde(alias = "minecraft:crafting_special_shielddecoration")]
    #[serde(alias = "minecraft:crafting_special_shulkerboxcoloring")]
    #[serde(alias = "minecraft:crafting_special_tippedarrow")]
    #[serde(alias = "minecraft:crafting_special_suspiciousstew")]
    Special,
}
impl Recipe {
    pub fn from_file(path: &Path) -> Result<Self, crate::RecipeLoadError> {
        let mut s = String::new();
        File::open(path)?.read_to_string(&mut s)?;
        let k = serde_json::from_str(&s)?;
        Ok(k)
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Single {
    item: Option<NamespacedId>,
    tag: Option<NamespacedId>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
enum Ingredient {
    One(Single),
    Array(Vec<Single>),
}
impl Single {
    pub fn matches(&self, item: Item, tag_registry: &TagRegistry) -> bool {
        self.item
            .as_ref()
            .map(|s| item.name() == s.name())
            .unwrap_or(false)
            | self
                .tag
                .as_ref()
                .map(|s| tag_registry.check_item_tag(item, s))
                .unwrap_or(false)
    }
}
impl Ingredient {
    pub fn matches(&self, item: Item, tag_registry: &TagRegistry) -> bool {
        match self {
            Ingredient::One(o) => o.matches(item, tag_registry),
            Ingredient::Array(vec) => vec.iter().any(|o| o.matches(item, tag_registry)),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmeltingRecipe {
    group: Option<SmartString<Compact>>,
    ingredient: Ingredient,
    result: SerdeItem,
    experience: f32,
    #[serde(default = "default_smelting_time")]
    cookingtime: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmokingRecipe {
    group: Option<SmartString<Compact>>,
    ingredient: Ingredient,
    result: SerdeItem,
    experience: f32,
    #[serde(default = "default_smoking_time")]
    cookingtime: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlastingRecipe {
    group: Option<SmartString<Compact>>,
    ingredient: Ingredient,
    result: SerdeItem,
    experience: f32,
    #[serde(default = "default_blasting_time")]
    cookingtime: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CampfireRecipe {
    group: Option<SmartString<Compact>>,
    ingredient: Ingredient,
    result: SerdeItem,
    experience: f32,
    #[serde(default = "default_campfire_time")]
    cookingtime: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapelessRecipe {
    group: Option<SmartString<Compact>>,
    ingredients: Vec<Ingredient>,
    result: SerdeItemStack,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapedRecipe {
    group: Option<SmartString<Compact>>,
    pattern: Vec<SmartString<Compact>>,
    key: HashMap<SmartString<Compact>, Ingredient>,
    result: SerdeItemStack,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmithingRecipe {
    group: Option<SmartString<Compact>>,
    base: Ingredient,
    addition: Ingredient,
    result: SerdeItemStack,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StonecuttingRecipe {
    group: Option<SmartString<Compact>>,
    ingredient: Ingredient,
    result: SerdeItem,
    count: u32,
}
impl SmeltingRecipe {
    pub fn matches(&self, item: Item, tag_registry: &TagRegistry) -> bool {
        self.ingredient.matches(item, tag_registry)
    }
    pub fn match_self(&self, item: Item, tag_registry: &TagRegistry) -> Option<(Item, f32)> {
        if self.matches(item, tag_registry) {
            Some((self.result.into(), self.experience))
        } else {
            None
        }
    }
}
impl SmokingRecipe {
    pub fn matches(&self, item: Item, tag_registry: &TagRegistry) -> bool {
        self.ingredient.matches(item, tag_registry)
    }
    pub fn match_self(&self, item: Item, tag_registry: &TagRegistry) -> Option<(Item, f32)> {
        if self.matches(item, tag_registry) {
            Some((self.result.into(), self.experience))
        } else {
            None
        }
    }
}
impl BlastingRecipe {
    pub fn matches(&self, item: Item, tag_registry: &TagRegistry) -> bool {
        self.ingredient.matches(item, tag_registry)
    }
    pub fn match_self(&self, item: Item, tag_registry: &TagRegistry) -> Option<(Item, f32)> {
        if self.matches(item, tag_registry) {
            Some((self.result.into(), self.experience))
        } else {
            None
        }
    }
}
impl CampfireRecipe {
    pub fn matches(&self, item: Item, tag_registry: &TagRegistry) -> bool {
        self.ingredient.matches(item, tag_registry)
    }
    pub fn match_self(&self, item: Item, tag_registry: &TagRegistry) -> Option<(Item, f32)> {
        if self.matches(item, tag_registry) {
            Some((self.result.into(), self.experience))
        } else {
            None
        }
    }
}
impl ShapelessRecipe {
    pub fn matches<'a>(
        &self,
        items: impl Iterator<Item = &'a Item>,
        tag_registry: &TagRegistry,
    ) -> bool {
        let mut counter = self.ingredients.clone();
        for i in items {
            match counter
                .iter()
                .enumerate()
                .find(|(_, ing)| ing.matches(*i, tag_registry))
            {
                Some((index, _)) => {
                    counter.remove(index);
                }
                None => return false,
            };
        }
        true
    }
    pub fn match_self<'a>(
        &self,
        items: impl Iterator<Item = &'a Item>,
        tag_registry: &TagRegistry,
    ) -> Option<ItemStack> {
        if self.matches(items, tag_registry) {
            Some(self.result.into())
        } else {
            None
        }
    }
}
impl ShapedRecipe {
    // TODO: Decide how to pass the crafting grid
}
impl SmithingRecipe {
    pub fn matches(&self, base: Item, addition: Item, tag_registry: &TagRegistry) -> bool {
        self.base.matches(base, tag_registry) && self.addition.matches(addition, tag_registry)
    }
    pub fn match_self(
        &self,
        base: Item,
        addition: Item,
        tag_registry: &TagRegistry,
    ) -> Option<Item> {
        if self.matches(base, addition, tag_registry) {
            Some(self.result.item.into())
        } else {
            None
        }
    }
}
impl StonecuttingRecipe {
    pub fn matches(&self, item: Item, tag_registry: &TagRegistry) -> bool {
        self.ingredient.matches(item, tag_registry)
    }
    pub fn match_self(&self, item: Item, tag_registry: &TagRegistry) -> Option<ItemStack> {
        if self.matches(item, tag_registry) {
            Some(ItemStack {
                item: self.result.into(),
                count: self.count,
                damage: None,
            })
        } else {
            None
        }
    }
}
pub fn default_smelting_time() -> u32 {
    200
}
pub fn default_smoking_time() -> u32 {
    100
}
pub fn default_blasting_time() -> u32 {
    100
}
pub fn default_campfire_time() -> u32 {
    100
}
