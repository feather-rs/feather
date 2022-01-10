use itertools::Itertools;
use std::collections::HashMap;
use std::sync::atomic::Ordering;

use indexmap::IndexMap;
use libcraft_blocks::BlockKind;
use serde::{Deserialize, Serialize};

use crate::chunk::paletted_container::BIOMES_COUNT;
use libcraft_core::EntityKind;

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    derive_more::Deref,
    Serialize,
    Deserialize,
)]
pub struct BiomeId(usize);

impl From<usize> for BiomeId {
    fn from(id: usize) -> Self {
        BiomeId(id)
    }
}

impl Default for BiomeId {
    fn default() -> Self {
        0.into()
    }
}

#[derive(Default, derive_more::Deref)]
pub struct BiomeList(IndexMap<String, BiomeGeneratorInfo>);

impl BiomeList {
    pub fn insert(&mut self, biome: String, info: BiomeGeneratorInfo) {
        BIOMES_COUNT.fetch_add(1, Ordering::Relaxed);
        self.0.insert(biome, info);
    }

    pub fn get_by_id(&self, id: &BiomeId) -> Option<(&String, &BiomeGeneratorInfo)> {
        self.0.get_index(**id)
    }

    pub fn get_id(&self, identifier: &str) -> Option<BiomeId> {
        self.0.get_index_of(identifier).map(BiomeId)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BiomeGeneratorInfo {
    pub carvers: HashMap<BlockKind, Vec<String>>,
    pub features: Vec<Vec<String>>,
    pub spawners: BiomeSpawners,
    #[serde(with = "spawn_costs")]
    pub spawn_costs: HashMap<EntityKind, BiomeSpawnCost>,
    #[serde(flatten)]
    pub info: BiomeInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BiomeInfo {
    pub effects: BiomeEffects,
    pub precipitation: String,
    pub temperature: f32,
    pub downfall: f32,
    pub temperature_modifier: Option<BiomeTemperatureModifier>,
    pub category: BiomeCategory,
    pub particle: Option<BiomeParticle>,
}

pub mod spawn_costs {
    use serde::de::Error;
    use serde::ser::SerializeMap;
    use serde::{Deserializer, Serializer};

    use super::*;

    pub fn serialize<S>(
        value: &HashMap<EntityKind, BiomeSpawnCost>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(value.len()))?;
        for (key, value) in value.iter() {
            map.serialize_entry(key.namespaced_id(), value)?;
        }
        map.end()
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<HashMap<EntityKind, BiomeSpawnCost>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut map = HashMap::new();
        for (key, value) in HashMap::<String, BiomeSpawnCost>::deserialize(deserializer)? {
            map.insert(
                EntityKind::from_namespaced_id(&key).ok_or_else(|| {
                    D::Error::custom(format_args!(
                        "unknown field `{}`, expected one of {}",
                        key,
                        EntityKind::values()
                            .iter()
                            .map(|kind| format!("`{}`", kind.namespaced_id()))
                            .join(", ")
                    ))
                })?,
                value,
            );
        }
        Ok(map)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BiomeSpawnCost {
    energy_budget: f32,
    charge: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BiomeParticle {
    pub probability: f32,
    pub options: BiomeParticleOptions,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BiomeParticleOptions {
    #[serde(rename = "type")]
    pub particle_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BiomeTemperatureModifier {
    Frozen,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BiomeGrassColorModifier {
    Swamp,
    DarkForest,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(from = "String", into = "String")] // quartz_nbt serialized enum variants by their index, not name
pub enum BiomeCategory {
    Ocean,
    Plains,
    Desert,
    Forest,
    ExtremeHills,
    Taiga,
    Swamp,
    River,
    Nether,
    TheEnd,
    Icy,
    Mushroom,
    Beach,
    Jungle,
    Mesa,
    Savanna,
    Mountain,
    Underground,
    None,
}

impl From<BiomeCategory> for String {
    fn from(category: BiomeCategory) -> Self {
        match category {
            BiomeCategory::Ocean => "ocean",
            BiomeCategory::Plains => "plains",
            BiomeCategory::Desert => "desert",
            BiomeCategory::Forest => "forest",
            BiomeCategory::ExtremeHills => "extreme_hills",
            BiomeCategory::Taiga => "taiga",
            BiomeCategory::Swamp => "swamp",
            BiomeCategory::River => "river",
            BiomeCategory::Nether => "nether",
            BiomeCategory::TheEnd => "the_end",
            BiomeCategory::Icy => "icy",
            BiomeCategory::Mushroom => "mushroom",
            BiomeCategory::Beach => "beach",
            BiomeCategory::Jungle => "jungle",
            BiomeCategory::Mesa => "mesa",
            BiomeCategory::Savanna => "savanna",
            BiomeCategory::Mountain => "mountain",
            BiomeCategory::Underground => "underground",
            BiomeCategory::None => "none",
        }
        .to_owned()
    }
}

impl From<String> for BiomeCategory {
    fn from(s: String) -> Self {
        match &s[..] {
            "ocean" => BiomeCategory::Ocean,
            "plains" => BiomeCategory::Plains,
            "desert" => BiomeCategory::Desert,
            "forest" => BiomeCategory::Forest,
            "extreme_hills" => BiomeCategory::ExtremeHills,
            "taiga" => BiomeCategory::Taiga,
            "swamp" => BiomeCategory::Swamp,
            "river" => BiomeCategory::River,
            "nether" => BiomeCategory::Nether,
            "the_end" => BiomeCategory::TheEnd,
            "icy" => BiomeCategory::Icy,
            "mushroom" => BiomeCategory::Mushroom,
            "beach" => BiomeCategory::Beach,
            "jungle" => BiomeCategory::Jungle,
            "mesa" => BiomeCategory::Mesa,
            "savanna" => BiomeCategory::Savanna,
            "mountain" => BiomeCategory::Mountain,
            "underground" => BiomeCategory::Underground,
            _ => BiomeCategory::None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BiomeEffects {
    pub mood_sound: Option<BiomeMoodSound>,
    pub music: Option<BiomeMusic>,
    pub ambient_sound: Option<String>,
    pub additions_sound: Option<BiomeAdditionsSound>,
    pub grass_color_modifier: Option<BiomeGrassColorModifier>,
    pub sky_color: BiomeColor,
    pub foliage_color: Option<BiomeColor>,
    pub grass_color: Option<BiomeColor>,
    pub fog_color: BiomeColor,
    pub water_color: BiomeColor,
    pub water_fog_color: BiomeColor,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BiomeAdditionsSound {
    pub sound: String,
    pub tick_chance: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BiomeMusic {
    pub sound: String,
    pub min_delay: i32,
    pub max_delay: i32,
    #[serde(deserialize_with = "super::world::deserialize_bool")]
    pub replace_current_music: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(from = "i32", into = "i32")]
pub struct BiomeColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<i32> for BiomeColor {
    #[allow(clippy::identity_op)]
    #[allow(clippy::erasing_op)]
    fn from(i: i32) -> Self {
        let r = ((i >> (8 * 2)) & 0xFF) as u8;
        let g = ((i >> (8 * 1)) & 0xFF) as u8;
        let b = ((i >> (8 * 0)) & 0xFF) as u8;
        BiomeColor { r, g, b }
    }
}

impl From<BiomeColor> for i32 {
    fn from(color: BiomeColor) -> Self {
        let mut i = 0i32;
        i += color.r as i32;
        i <<= 8;
        i += color.g as i32;
        i <<= 8;
        i += color.b as i32;
        i
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BiomeMoodSound {
    pub sound: String,
    pub tick_delay: i32,
    pub block_search_extent: i32,
    pub offset: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BiomeSpawners {
    pub monster: Vec<BiomeSpawner>,
    pub creature: Vec<BiomeSpawner>,
    pub ambient: Vec<BiomeSpawner>,
    #[serde(default)]
    pub axolotls: Vec<BiomeSpawner>,
    pub underground_water_creature: Vec<BiomeSpawner>,
    pub water_creature: Vec<BiomeSpawner>,
    pub water_ambient: Vec<BiomeSpawner>,
    pub misc: Vec<BiomeSpawner>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BiomeSpawner {
    #[serde(rename = "type")]
    pub entity_type: String,
    pub weight: usize,
    pub min_count: usize,
    pub max_count: usize,
}
