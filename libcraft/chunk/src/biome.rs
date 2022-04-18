use itertools::Itertools;
use std::sync::atomic::Ordering;
use std::{collections::HashMap, io::Cursor};

use bincode::{Decode, Encode};
use libcraft_blocks::BlockKind;
use serde::{Deserialize, Serialize};

use crate::paletted_container::BIOMES_COUNT;
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
    Encode,
    Decode,
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

#[derive(Default, Serialize, Deserialize, Encode, Decode)]
pub struct BiomeList(Vec<(String, BiomeGeneratorInfo)>);

impl BiomeList {
    /// Returns the default set of biomes used in vanilla.
    pub fn vanilla() -> Self {
        static DATA: &[u8] = include_bytes!("../../assets/vanilla_biomes.bc.gz");

        let mut decoder = flate2::read::GzDecoder::new(Cursor::new(DATA));
        bincode::decode_from_std_read(&mut decoder, bincode::config::standard())
            .expect("malformed vanilla biomes data")
    }

    pub fn insert(&mut self, biome: String, info: BiomeGeneratorInfo) {
        BIOMES_COUNT.fetch_add(1, Ordering::Relaxed);
        self.0.push((biome, info));
    }

    pub fn get_by_id(&self, id: &BiomeId) -> Option<(&String, &BiomeGeneratorInfo)> {
        self.0.get(id.0).map(|(a, b)| (a, b))
    }

    pub fn get_id(&self, identifier: &str) -> Option<BiomeId> {
        self.0
            .iter()
            .position(|(name, _)| name == identifier)
            .map(BiomeId)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &BiomeGeneratorInfo)> + '_ {
        self.0.iter().map(|(a, b)| (a, b))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Encode, Decode)]
pub struct BiomeGeneratorInfo {
    pub carvers: HashMap<BlockKind, Vec<String>>,
    pub features: Vec<Vec<String>>,
    pub spawners: BiomeSpawners,
    #[serde(with = "spawn_costs")]
    pub spawn_costs: HashMap<EntityKind, BiomeSpawnCost>,
    #[serde(flatten)]
    pub info: BiomeInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone, Encode, Decode)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Encode, Decode)]
pub struct BiomeSpawnCost {
    energy_budget: f32,
    charge: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Encode, Decode)]
pub struct BiomeParticle {
    pub probability: f32,
    pub options: BiomeParticleOptions,
}

#[derive(Serialize, Deserialize, Debug, Clone, Encode, Decode)]
pub struct BiomeParticleOptions {
    #[serde(rename = "type")]
    pub particle_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Encode, Decode)]
#[serde(rename_all = "snake_case")]
pub enum BiomeTemperatureModifier {
    Frozen,
}

#[derive(Serialize, Deserialize, Debug, Clone, Encode, Decode)]
#[serde(rename_all = "snake_case")]
pub enum BiomeGrassColorModifier {
    Swamp,
    DarkForest,
}

#[derive(Serialize, Deserialize, Debug, Clone, Encode, Decode)]
#[serde(rename_all = "snake_case")]
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

#[derive(Serialize, Deserialize, Debug, Clone, Encode, Decode)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Encode, Decode)]
pub struct BiomeAdditionsSound {
    pub sound: String,
    pub tick_chance: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Encode, Decode)]
pub struct BiomeMusic {
    pub sound: String,
    pub min_delay: i32,
    pub max_delay: i32,
    #[serde(deserialize_with = "libcraft_core::deserialize_bool")]
    pub replace_current_music: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Encode, Decode)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Encode, Decode)]
pub struct BiomeMoodSound {
    pub sound: String,
    pub tick_delay: i32,
    pub block_search_extent: i32,
    pub offset: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Encode, Decode)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Encode, Decode)]
#[serde(rename_all = "camelCase")]
pub struct BiomeSpawner {
    #[serde(rename = "type")]
    pub entity_type: String,
    pub weight: usize,
    pub min_count: usize,
    pub max_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vanilla_biomes_load() {
        let biomes = BiomeList::vanilla();
        assert!(!biomes.0.is_empty());
        assert!(biomes.get_id("minecraft:plains").is_some());
    }
}
