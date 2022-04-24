use anyhow::Context;
use libcraft::biome::BiomeList;
use libcraft::chunk::LightStore;
use libcraft::{
    chunk::{
        paletted_container::{Paletteable, PalettedContainer},
        PackedArray,
    },
    BlockState, ChunkPosition,
};
use libcraft::{Chunk, ChunkSection, Sections};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkModel {
    pub sections: Vec<ChunkSectionModel>,
}

impl ChunkModel {
    pub fn from_chunk(chunk: &Chunk, biomes: &BiomeList) -> Self {
        Self {
            sections: chunk
                .sections()
                .iter()
                .map(|section| ChunkSectionModel::from_chunk_section(section, biomes))
                .collect(),
        }
    }

    pub fn to_chunk(
        &self,
        biomes: &BiomeList,
        pos: ChunkPosition,
        sections: Sections,
        min_y: i32,
    ) -> Result<Chunk, anyhow::Error> {
        let mut chunk = Chunk::new(pos, sections, min_y);

        for (model, section) in self.sections.iter().zip(chunk.sections_mut()) {
            *section = model.to_chunk_section(biomes)?;
        }

        Ok(chunk)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkSectionModel {
    pub sky_light: Option<PackedArrayModel>,
    pub block_light: Option<PackedArrayModel>,
    pub blocks: PalettedContainerModel<BlockStateModel>,
    pub biomes: PalettedContainerModel<String>,
    pub air_block_count: u32,
}

impl ChunkSectionModel {
    pub fn from_chunk_section(section: &ChunkSection, biomes: &BiomeList) -> Self {
        Self {
            sky_light: section.light().sky_light().map(Into::into),
            block_light: section.light().block_light().map(Into::into),
            blocks: PalettedContainerModel::from(section.blocks(), |state| {
                BlockStateModel::from(*state)
            }),
            biomes: PalettedContainerModel::from(section.biomes(), |biome_id| {
                biomes.get_by_id(biome_id).unwrap().0.clone()
            }),
            air_block_count: section.air_blocks(),
        }
    }

    pub fn to_chunk_section(&self, biomes: &BiomeList) -> Result<ChunkSection, anyhow::Error> {
        Ok(ChunkSection::new(
            self.blocks
                .clone()
                .into(|block| (&block).try_into().unwrap()),
            self.biomes
                .clone()
                .into(|biome| biomes.get_id(&biome).unwrap()),
            self.air_block_count,
            LightStore::from_packed_arrays(
                self.sky_light.clone().map(Into::into),
                self.block_light.clone().map(Into::into),
            )
            .unwrap(),
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockStateModel {
    pub kind: String,
    pub properties: Vec<(String, String)>,
}

impl From<BlockState> for BlockStateModel {
    fn from(state: BlockState) -> Self {
        Self {
            kind: state.namespaced_id().to_owned(),
            properties: state
                .property_values()
                .map(|(a, b)| (a.to_owned(), b.to_owned()))
                .collect(),
        }
    }
}

impl TryFrom<&BlockStateModel> for BlockState {
    type Error = anyhow::Error;

    fn try_from(model: &BlockStateModel) -> Result<Self, anyhow::Error> {
        BlockState::from_namespaced_id_and_property_values(
            &model.kind,
            model
                .properties
                .iter()
                .map(|(a, b)| (a.as_str(), b.as_str())),
        )
        .context("invalid block state")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PalettedContainerModel<T> {
    SingleValue(T),
    MultipleValues {
        data: PackedArrayModel,
        palette: Vec<T>,
    },
    GlobalPalette {
        data: PackedArrayModel,
    },
}

impl<T> PalettedContainerModel<T> {
    pub fn from<U>(container: &PalettedContainer<U>, convert: impl Fn(&U) -> T) -> Self
    where
        U: Paletteable,
    {
        match container {
            PalettedContainer::SingleValue(val) => {
                PalettedContainerModel::SingleValue(convert(val))
            }
            PalettedContainer::MultipleValues { data, palette } => {
                PalettedContainerModel::MultipleValues {
                    data: data.into(),
                    palette: palette.iter().map(convert).collect(),
                }
            }
            PalettedContainer::GlobalPalette { data } => {
                PalettedContainerModel::GlobalPalette { data: data.into() }
            }
        }
    }

    pub fn into<U>(self, convert: impl Fn(T) -> U) -> PalettedContainer<U>
    where
        U: Paletteable,
    {
        match self {
            PalettedContainerModel::SingleValue(val) => {
                PalettedContainer::SingleValue(convert(val))
            }
            PalettedContainerModel::MultipleValues { data, palette } => {
                PalettedContainer::MultipleValues {
                    data: data.into(),
                    palette: palette.into_iter().map(convert).collect(),
                }
            }
            PalettedContainerModel::GlobalPalette { data } => {
                PalettedContainer::GlobalPalette { data: data.into() }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackedArrayModel {
    pub length: usize,
    pub bits: Vec<u64>,
}

impl<'a> From<&'a PackedArray> for PackedArrayModel {
    fn from(packed: &'a PackedArray) -> Self {
        Self {
            length: packed.len(),
            bits: packed.as_u64_slice().to_vec(),
        }
    }
}

impl From<PackedArrayModel> for PackedArray {
    fn from(model: PackedArrayModel) -> Self {
        Self::from_u64_vec(model.bits, model.length)
    }
}
