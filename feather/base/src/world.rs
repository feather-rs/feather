use crate::chunk::SECTION_HEIGHT;
use serde::de::Visitor;
use serde::Deserializer;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionInfo {
    pub r#type: String,
    #[serde(default)]
    pub info: DimensionTypeInfo,
    pub generator: DimensionGeneratorInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DimensionTypeInfo {
    pub logical_height: i32,
    pub infiniburn: String,
    pub effects: String,
    pub ambient_light: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub respawn_anchor_works: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub has_raids: bool,
    pub min_y: i32,
    pub height: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub natural: bool,
    pub coordinate_scale: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub piglin_safe: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub bed_works: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub has_skylight: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub has_ceiling: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ultrawarm: bool,
    pub fixed_time: Option<i32>,
}

pub(crate) fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct BoolI8Visitor;

    impl Visitor<'_> for BoolI8Visitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("a bool")
        }

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v)
        }

        fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v != 0)
        }

        fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v != 0)
        }
    }

    deserializer.deserialize_any(BoolI8Visitor)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionGeneratorInfo {
    pub biome_source: BiomeSource,
    pub seed: u64,
    pub settings: DimensionSettings,
    pub r#type: GeneratorRandomSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BiomeSource {
    #[serde(rename = "minecraft:multi_noise")]
    MultiNoise { biomes: Vec<Biome> },
    #[serde(rename = "minecraft:the_end")]
    TheEnd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Biome {
    pub parameters: BiomeParams,
    pub biome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiomeParams {
    pub erosion: ValueOrRange,
    pub depth: ValueOrRange,
    pub weirdness: ValueOrRange,
    pub offset: ValueOrRange,
    pub temperature: ValueOrRange,
    pub humidity: ValueOrRange,
    pub continentalness: ValueOrRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ValueOrRange {
    Value(f64),
    Range([f64; 2]),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeneratorRandomSource {
    #[serde(rename = "minecraft:noise")]
    Noise,
    #[serde(rename = "minecraft:multi_noise")]
    MultiNoise,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DimensionSettings {
    #[serde(rename = "minecraft:overworld")]
    Overworld,
    #[serde(rename = "minecraft:nether")]
    Nether,
    #[serde(rename = "minecraft:end")]
    End,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug, derive_more::Deref)]
pub struct WorldHeight(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, Debug, derive_more::Deref)]
pub struct Sections(pub usize);

impl From<Sections> for WorldHeight {
    fn from(sections: Sections) -> Self {
        WorldHeight(sections.0 * SECTION_HEIGHT)
    }
}

impl From<WorldHeight> for Sections {
    fn from(sections: WorldHeight) -> Self {
        Sections(sections.0 / SECTION_HEIGHT)
    }
}
