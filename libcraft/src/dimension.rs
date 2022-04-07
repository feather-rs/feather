use serde::{Deserialize, Serialize};

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
    #[serde(deserialize_with = "libcraft_core::deserialize_bool")]
    pub respawn_anchor_works: bool,
    #[serde(deserialize_with = "libcraft_core::deserialize_bool")]
    pub has_raids: bool,
    pub min_y: i32,
    pub height: i32,
    #[serde(deserialize_with = "libcraft_core::deserialize_bool")]
    pub natural: bool,
    pub coordinate_scale: f32,
    #[serde(deserialize_with = "libcraft_core::deserialize_bool")]
    pub piglin_safe: bool,
    #[serde(deserialize_with = "libcraft_core::deserialize_bool")]
    pub bed_works: bool,
    #[serde(deserialize_with = "libcraft_core::deserialize_bool")]
    pub has_skylight: bool,
    #[serde(deserialize_with = "libcraft_core::deserialize_bool")]
    pub has_ceiling: bool,
    #[serde(deserialize_with = "libcraft_core::deserialize_bool")]
    pub ultrawarm: bool,
    pub fixed_time: Option<i32>,
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
