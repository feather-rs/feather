//! This module implements the loading and saving
//! of Anvil region files.

use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt::{self, Display, Formatter};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::{fs, io, iter};

use bitvec::vec::BitVec;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use libcraft_blocks::{BlockKind, BlockState};
use libcraft_chunk::biome::{BiomeId, BiomeList};
use libcraft_chunk::paletted_container::{Paletteable, PalettedContainer};
use libcraft_chunk::{
    Chunk, ChunkSection, LightStore, PackedArray, BIOMES_PER_CHUNK_SECTION, SECTION_VOLUME,
};
use libcraft_core::REGION_SIZE;
use libcraft_core::{ChunkPosition, WorldHeight, ANVIL_VERSION_RANGE};
use serde::{Deserialize, Serialize};

use super::{block_entity::BlockEntityData, entity::EntityData};

pub use libcraft_core::RegionPosition;

/// Length, in bytes, of a sector.
const SECTOR_BYTES: usize = 4096;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataChunk {
    #[serde(rename = "DataVersion")]
    data_version: i32,
    #[serde(rename = "xPos")]
    x_pos: i32,
    #[serde(rename = "zPos")]
    z_pos: i32,
    #[serde(rename = "yPos")]
    min_y_section: i32,
    #[serde(rename = "LastUpdate")]
    last_update: i64,
    #[serde(rename = "InhabitedTime")]
    #[serde(default)]
    inhabited_time: i64,
    sections: Vec<LevelSection>,
    #[serde(default)]
    entities: Vec<EntityData>,
    #[serde(default)]
    block_entities: Vec<BlockEntityData>,
    #[serde(rename = "PostProcessing")]
    #[serde(default)]
    post_processing: Vec<Vec<i16>>,
    #[serde(rename = "Status")]
    worldgen_status: Cow<'static, str>,
}

/// Represents a chunk section in a region file.
#[derive(Serialize, Deserialize, Debug)]
pub struct LevelSection {
    #[serde(rename = "Y")]
    y: i8,
    #[serde(default)]
    block_states: PaletteAndData<SerializedBlockState>,
    #[serde(default = "default_biomes")]
    biomes: PaletteAndData<String>,
    #[serde(rename = "SkyLight")]
    sky_light: Option<Vec<i8>>,
    #[serde(rename = "BlockLight")]
    block_light: Option<Vec<i8>>,
}

fn default_biomes() -> PaletteAndData<String> {
    PaletteAndData {
        palette: vec!["minecraft:the_void".to_owned()],
        data: None,
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct PaletteAndData<T> {
    palette: Vec<T>,
    data: Option<Vec<i64>>,
}

impl<T> Default for PaletteAndData<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            palette: vec![T::default()],
            data: None,
        }
    }
}

/// Represents a palette entry in a region file.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SerializedBlockState {
    /// The identifier of the type of this block
    name: Cow<'static, str>,
    /// Optional properties for this block
    properties: Option<LevelProperties>,
}

impl Default for SerializedBlockState {
    fn default() -> Self {
        Self {
            name: Cow::Borrowed("minecraft:air"),
            properties: None,
        }
    }
}

/// Represents the properties for a palette entry.
#[derive(Serialize, Deserialize, Debug)]
pub struct LevelProperties {
    /// Map containing a list of property names to values.
    #[serde(flatten)]
    props: BTreeMap<Cow<'static, str>, Cow<'static, str>>,
}

/// Represents a block update scheduled for a specific time.
#[derive(Serialize, Deserialize, Debug)]
pub struct ScheduledBlockUpdate {
    /// The identifier of the type of this block
    #[serde(rename = "i")]
    name: Cow<'static, str>,
    // TODO are these global or chunk coordinates?
    /// X coordinate
    pub x: i32,
    /// Y coordinate
    pub y: i32,
    /// Z coordinate
    pub z: i32,
}

/// A block of sectors in a region file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SectorBlock {
    /// Offset, in sectors, from the start of the file (beginning of the header.)
    offset: u32,
    /// Number of sectors in this block. Each sector is 4KiB.
    count: u32,
}

/// A region file handle.
pub struct RegionHandle {
    /// The region file.
    file: File,
    /// The region file's header, pre-loaded into memory.
    header: RegionHeader,
    /// Sector allocator to allocate sectors where we can store chunks.
    allocator: SectorAllocator,
    world_height: WorldHeight,
}

impl RegionHandle {
    /// Loads the chunk at the given position (global, not region-relative).
    ///
    /// The specified chunk is expected to be contained within this region.
    ///
    /// # Panics
    /// Panics if the specified chunk position is not within this
    /// region file.
    pub fn load_chunk(
        &mut self,
        mut pos: ChunkPosition,
        biomes: &BiomeList,
    ) -> Result<(Chunk, Vec<EntityData>, Vec<BlockEntityData>), Error> {
        // Get a copy of the original position before clipping
        let original_pos = pos;
        // Clip chunk position to region-local coordinates.
        pos.x %= 32;
        pos.z %= 32;

        // Get the offset of the chunk within the file
        // so that it can be read.
        let offset = self.header.location_for_chunk(pos).0.offset;

        // If the chunk doesn't exist, return early
        if !self.header.location_for_chunk(pos).exists() {
            return Err(Error::ChunkNotExist);
        }

        // Seek to the offset position. Note that since the offset in the header
        // is in "sectors" of 4KiB each, the value needs to be multiplied by SECTOR_BYTES
        // to get the offset in bytes.
        self.file
            .seek(SeekFrom::Start(u64::from(offset) * SECTOR_BYTES as u64))
            .map_err(Error::Io)?;

        // A chunk begins with a four-byte, big-endian value
        // indicating the exact length of the chunk's data
        // in bytes.
        let len = self.file.read_u32::<BigEndian>().map_err(Error::Io)?;

        // Avoid DoS attacks
        if len > 1_048_576 {
            return Err(Error::ChunkTooLarge(len as usize));
        }

        if len == 0 {
            return Err(Error::ChunkTooLarge(0));
        }

        // Read `len` bytes into memory.
        let mut buf = vec![0u8; len as usize];
        self.file.read_exact(&mut buf).map_err(Error::Io)?;

        // The compression type is indicated by a byte.
        // 1 corresponds to gzip compression, while 2
        // corresponds to zlib.
        let compression_type = buf[0];

        // Parse NBT data
        let bytes = &buf[1..];
        let mut data_chunk: DataChunk = match compression_type {
            1 => nbt::from_gzip_reader(bytes).map_err(Error::Nbt)?,
            2 => nbt::from_zlib_reader(bytes).map_err(Error::Nbt)?,
            _ => return Err(Error::InvalidCompression(compression_type)),
        };

        // Check data version
        if !ANVIL_VERSION_RANGE.contains(&data_chunk.data_version) {
            return Err(Error::UnsupportedDataVersion(data_chunk.data_version));
        }

        let mut chunk = Chunk::new(
            original_pos,
            self.world_height.into(),
            data_chunk.min_y_section,
        );

        // Read sections
        for section in std::mem::take(&mut data_chunk.sections) {
            read_section_into_chunk(section, &mut chunk, data_chunk.min_y_section, biomes)?;
        }

        chunk.recalculate_heightmaps();

        Ok((chunk, data_chunk.entities, data_chunk.block_entities))
    }

    /// Checks if the specified chunk position is generated in this region.
    /// # Panics
    /// Panics if the specified chunk position is not within this
    /// region file.
    pub fn check_chunk_existence(&self, pos: ChunkPosition) -> bool {
        self.header.location_for_chunk(pos).exists()
    }

    /// Saves the given chunk to this region file. The header will be updated
    /// accordingly and saved as well.
    ///
    /// Behavior may be unexpected if this region file does not contain the given
    /// chunk position.
    pub fn save_chunk(
        &mut self,
        chunk: &Chunk,
        entities: &[EntityData],
        block_entities: &[BlockEntityData],
        biomes: &BiomeList,
    ) -> Result<(), Error> {
        let chunk_pos = chunk.position();

        let (local_x, local_z) = (chunk_pos.x % 32, chunk_pos.z % 32);

        // Find position in header and deallocate it if it currently exists.
        let location = self
            .header
            .location_for_chunk(ChunkPosition::new(local_x, local_z));
        if location.exists() {
            self.allocator.free(location.0);
        }

        // Write chunk to `ChunkData` tag.
        let data_chunk = chunk_to_data_chunk(chunk, entities, block_entities, biomes);

        // Write to intermediate buffer, because we need to know the length.
        let mut buf = Vec::with_capacity(4096);
        buf.write_u8(2).map_err(Error::Io)?; // Compression type: zlib

        nbt::to_zlib_writer(&mut buf, &data_chunk, None).map_err(Error::Nbt)?;

        let total_len = buf.len() + 4; // 4 bytes for length header

        let sectors = (total_len + SECTOR_BYTES - 1) / SECTOR_BYTES;

        let block = self.allocator.allocate(sectors as u32);

        // Write to file
        self.file
            .seek(SeekFrom::Start(block.offset as u64 * SECTOR_BYTES as u64))
            .map_err(Error::Io)?;

        self.file
            .write_u32::<BigEndian>(buf.len() as u32)
            .map_err(Error::Io)?;
        self.file.write_all(&buf).map_err(Error::Io)?;

        // Write padding to align to sector count
        let padding_count = SECTOR_BYTES - total_len % SECTOR_BYTES;

        for _ in 0..padding_count {
            self.file.write_u8(0).map_err(Error::Io)?;
        }

        // Update header
        self.header
            .set_location_for_chunk(ChunkPosition::new(local_x, local_z), ChunkLocation(block));
        self.save_header().map_err(Error::Io)?;

        Ok(())
    }

    fn save_header(&mut self) -> Result<(), io::Error> {
        self.file.seek(SeekFrom::Start(0))?;

        self.header.write_to(&mut self.file)
    }
}

fn read_section_into_chunk(
    mut section: LevelSection,
    chunk: &mut Chunk,
    min_y_section: i32,
    biome_list: &BiomeList,
) -> Result<(), Error> {
    // Create palettes
    let mut block_palette = Vec::new();
    for entry in &section.block_states.palette {
        // Construct properties map
        let mut props = BTreeMap::new();
        if let Some(entry_props) = entry.properties.as_ref() {
            props.extend(
                entry_props
                    .props
                    .iter()
                    .map(|(k, v)| (k.clone().to_owned(), v.clone().to_owned())),
            );
        }

        // Attempt to get block from the given values
        let block = match &entry.properties {
            Some(properties) => BlockState::from_namespaced_id_and_property_values(
                &entry.name,
                properties.props.iter().map(|(k, v)| (&**k, &**v)),
            ),
            None => BlockKind::from_namespaced_id(&entry.name).map(BlockState::new),
        }
        .ok_or_else(|| {
            Error::InvalidBlockType(format!(
                "{} with properties {:?}",
                entry.name, entry.properties
            ))
        })?;

        block_palette.push(block);
    }

    let mut block_palette_iter = block_palette.iter();
    let mut blocks = if let Some(block) = block_palette_iter.next() {
        let mut blocks = PalettedContainer::SingleValue(*block);
        for block in block_palette_iter {
            blocks.index_or_insert(*block);
        }
        blocks
    } else {
        PalettedContainer::new()
    };

    let mut biome_palette = Vec::new();
    for biome in section.biomes.palette {
        let biome = biome_list
            .get_id(&biome)
            .unwrap_or_else(|| panic!("Biome not found: {}", biome))
            .into();
        biome_palette.push(biome);
    }

    let mut biome_palette_iter = biome_palette.iter();
    let mut biomes = if let Some(biome) = biome_palette_iter.next() {
        let mut biomes = PalettedContainer::SingleValue(*biome);
        for biome in biome_palette_iter {
            biomes.index_or_insert(*biome);
        }
        biomes
    } else {
        PalettedContainer::new()
    };

    if let Some(mut blocks_data) = section
        .block_states
        .data
        .map(|data| PackedArray::from_i64_vec(data, SECTION_VOLUME))
    {
        // Correct the palette.
        // For some reason, vanilla seems to write
        // out-of-bounds palette indexes into the data array. We
        // set these to 0.

        if let Some(palette) = blocks.palette() {
            for i in 0..SECTION_VOLUME {
                let block = blocks_data.get(i).unwrap();

                if block as usize >= palette.len() {
                    blocks_data.set(i, 0);
                }
            }
        }

        blocks.set_data(
            if blocks_data.bits_per_value() > <BlockState as Paletteable>::MAX_BITS_PER_ENTRY {
                // Convert to GlobalPalette
                let mut data = blocks_data
                    .resized(PalettedContainer::<BlockState>::global_palette_bits_per_value());
                PalettedContainer::<BlockState>::map_to_global_palette(
                    blocks.len(),
                    &block_palette,
                    &mut data,
                );
                data
            } else {
                blocks_data
            },
        );
    }
    if let Some(biomes_data) = section
        .biomes
        .data
        .map(|data| PackedArray::from_i64_vec(data, BIOMES_PER_CHUNK_SECTION))
    {
        biomes.set_data(
            if biomes_data.bits_per_value() > <BlockState as Paletteable>::MAX_BITS_PER_ENTRY {
                // Convert to GlobalPalette
                let mut data = biomes_data
                    .resized(PalettedContainer::<BiomeId>::global_palette_bits_per_value());
                PalettedContainer::<BiomeId>::map_to_global_palette(
                    biomes.len(),
                    &biome_palette,
                    &mut data,
                );
                data
            } else {
                biomes_data
            },
        );
    }

    // Light
    // convert raw lighting data (4bits / block) into a BitArray
    fn convert_light_data(light_data: &[i8]) -> PackedArray {
        let data: Vec<u64> = light_data
            .chunks(8)
            .map(|chunk| {
                // not sure if there's a better (safe) way of doing this..
                let chunk: [u8; 8] = [
                    chunk[0] as u8,
                    chunk[1] as u8,
                    chunk[2] as u8,
                    chunk[3] as u8,
                    chunk[4] as u8,
                    chunk[5] as u8,
                    chunk[6] as u8,
                    chunk[7] as u8,
                ];
                u64::from_le_bytes(chunk)
            })
            .collect();

        PackedArray::from_u64_vec(data, SECTION_VOLUME)
    }

    if section.sky_light.is_some() && section.sky_light.as_ref().unwrap().is_empty() {
        section.sky_light = Some(vec![0; 2048]);
    }
    if section.block_light.is_some() && section.block_light.as_ref().unwrap().is_empty() {
        section.block_light = Some(vec![0; 2048]);
    }

    if section.sky_light.is_some() && section.sky_light.as_ref().unwrap().len() != 2048
        || section.block_light.is_some() && section.block_light.as_ref().unwrap().len() != 2048
    {
        return Err(Error::IndexOutOfBounds);
    }

    let sky_light = section
        .sky_light
        .as_ref()
        .map(|light| convert_light_data(light));
    let block_light = section
        .block_light
        .as_ref()
        .map(|light| convert_light_data(light));

    let light =
        LightStore::from_packed_arrays(sky_light, block_light).ok_or(Error::IndexOutOfBounds)?;

    let air_blocks = ChunkSection::count_air_blocks(&blocks);
    let chunk_section = ChunkSection::new(blocks, biomes, air_blocks, light);

    chunk.set_section_at(section.y as isize - min_y_section as isize, chunk_section);

    Ok(())
}

fn chunk_to_data_chunk(
    chunk: &Chunk,
    entities: &[EntityData],
    block_entities: &[BlockEntityData],
    biome_list: &BiomeList,
) -> DataChunk {
    DataChunk {
        data_version: *ANVIL_VERSION_RANGE.end(),
        x_pos: chunk.position().x,
        z_pos: chunk.position().z,
        min_y_section: chunk.min_y_section(),
        last_update: 0,    // TODO
        inhabited_time: 0, // TODO
        block_entities: block_entities.into(),
        sections: chunk
            .sections()
            .iter()
            .enumerate()
            .map(|(y, section)| {
                LevelSection {
                    y: (y as i8) - 1,
                    block_states: match section.blocks() {
                        PalettedContainer::SingleValue(block) => PaletteAndData {
                            palette: vec![serialize_block_state(block)],
                            data: None,
                        },
                        PalettedContainer::MultipleValues { data, palette } => PaletteAndData {
                            palette: palette.iter().map(serialize_block_state).collect(),
                            data: Some(bytemuck::cast_slice(data.as_u64_slice()).to_vec()),
                        },
                        PalettedContainer::GlobalPalette { data } => {
                            // Convert to MultipleValues palette
                            let mut data = data.clone();
                            let mut palette = Vec::new();
                            data.iter().for_each(|value| {
                                let block = BlockState::from_default_palette(value as u32).unwrap();
                                if !palette.contains(&block) {
                                    palette.push(block);
                                }
                            });
                            PalettedContainer::<BlockState>::map_from_global_palette(
                                section.blocks().len(),
                                &palette,
                                &mut data,
                            );
                            let data = data.resized(
                                PalettedContainer::<BlockState>::palette_bits_per_value(
                                    palette.len(),
                                ),
                            );
                            PaletteAndData {
                                palette: palette.iter().map(serialize_block_state).collect(),
                                data: Some(bytemuck::cast_slice(data.as_u64_slice()).to_vec()),
                            }
                        }
                    },
                    biomes: match section.biomes() {
                        PalettedContainer::SingleValue(biome) => PaletteAndData {
                            palette: vec![biome_list.get_by_id(biome).unwrap().0.into()],
                            data: None,
                        },
                        PalettedContainer::MultipleValues { data, palette } => PaletteAndData {
                            palette: palette
                                .iter()
                                .map(|biome| biome_list.get_by_id(biome).unwrap().0.into())
                                .collect(),
                            data: Some(bytemuck::cast_slice(data.as_u64_slice()).to_vec()),
                        },
                        PalettedContainer::GlobalPalette { data } => {
                            // Convert to MultipleValues palette
                            let mut data = data.clone();
                            let mut palette = Vec::new();
                            data.iter().for_each(|value| {
                                let block = BiomeId::from_default_palette(value as u32).unwrap();
                                if !palette.contains(&block) {
                                    palette.push(block);
                                }
                            });
                            PalettedContainer::<BiomeId>::map_from_global_palette(
                                section.biomes().len(),
                                &palette,
                                &mut data,
                            );
                            let data = data.resized(
                                PalettedContainer::<BiomeId>::palette_bits_per_value(palette.len()),
                            );
                            PaletteAndData {
                                palette: palette
                                    .iter()
                                    .map(|biome| biome_list.get_by_id(biome).unwrap().0.into())
                                    .collect(),
                                data: Some(bytemuck::cast_slice(data.as_u64_slice()).to_vec()),
                            }
                        }
                    },
                    sky_light: section
                        .light()
                        .sky_light()
                        .map(|light| slice_u64_to_i8(light.as_u64_slice()).to_vec()),
                    block_light: section
                        .light()
                        .block_light()
                        .map(|light| slice_u64_to_i8(light.as_u64_slice()).to_vec()),
                }
            })
            .collect(),
        entities: entities.into(),
        post_processing: vec![vec![]; 16],
        worldgen_status: "postprocessed".into(),
    }
}

fn slice_u64_to_i8(input: &[u64]) -> &[i8] {
    // TODO: someone should check this isn't undefined behavior.
    // Pretty sure the alignment check makes this sound,
    // but I'm not certain.
    let (head, body, tail) = unsafe { input.align_to::<i8>() };

    // Ensure that alignment is correct
    assert!(head.is_empty());
    assert!(tail.is_empty());

    body
}

/// An allocator for sectors.
struct SectorAllocator {
    /// Vector of bits, with a bit set for each sector which is in use.
    ///
    /// TODO: use a more efficient allocation model, such as a `LinkedList`
    /// of free blocks.
    used_sectors: BitVec,
}

impl SectorAllocator {
    /// Creates a `SectorAllocator` from the given file header
    /// and total file size __in sectors.__
    pub fn new(header: &RegionHeader, file_size: u32) -> Self {
        let mut used_sectors: BitVec = (0..file_size as usize).map(|_| 0).collect();

        // Detect used sectors
        for chunk_location in &header.locations {
            if !chunk_location.exists() {
                continue;
            }

            let offset = chunk_location.0.offset;
            let count = chunk_location.0.count;
            (offset..offset + count).for_each(|sector| used_sectors.set(sector as usize, true));
        }

        // Allocate two sectors at start for header
        used_sectors.set(0, true);
        used_sectors.set(1, true);

        Self { used_sectors }
    }

    /// Frees the given block from this allocator.
    pub fn free(&mut self, block: SectorBlock) {
        (block.offset..block.offset + block.count)
            .for_each(|sector| self.used_sectors.set(sector as usize, false));
    }

    /// Allocates a block of sectors with the given
    /// minimum size __in sectors__.
    ///
    /// The returned block may
    /// stretch past the end of the file.
    pub fn allocate(&mut self, min_size: u32) -> SectorBlock {
        // TODO: fairly inefficient way to do this.
        let mut start = 0;
        let mut length = 0;

        let mut found_block = None;
        for (index, is_used) in self.used_sectors.iter().enumerate() {
            if *is_used {
                start = 0;
                length = 0;
            } else {
                if start == 0 {
                    start = index;
                }
                length += 1;

                if length >= min_size {
                    let block = SectorBlock {
                        offset: start as u32,
                        count: length as u32,
                    };

                    found_block = Some(block);
                    break;
                }
            }
        }

        if let Some(block) = found_block {
            for sector in block.offset..block.offset + block.count {
                self.used_sectors.set(sector as usize, true);
            }
            return block;
        }

        // No sector found: must allocate into end

        let block = SectorBlock {
            offset: self.used_sectors.len() as u32,
            count: min_size,
        };

        self.used_sectors
            .extend(iter::repeat(true).take(min_size as usize));

        block
    }
}

/// An error which occurred during region file processing.
#[derive(Debug)]
pub enum Error {
    /// The region file header was invalid
    Header(&'static str),
    /// The region file contained invalid NBT data
    Nbt(nbt::Error),
    /// The chunk was too large
    ChunkTooLarge(usize),
    /// The chunk contained an invalid compression type
    InvalidCompression(u8),
    /// An IO error occurred
    Io(io::Error),
    /// There was an invalid block in the chunk
    UnknownBlock(String),
    /// The chunk does not exist
    ChunkNotExist,
    /// The chunk uses an unsupported data version
    UnsupportedDataVersion(i32),
    /// The palette for the chunk contained in invalid block type
    InvalidBlockType(String),
    /// The "Chunk [x, z]" tag was missing
    MissingRootTag,
    /// Chunk section index was out of bounds
    IndexOutOfBounds,
    /// Invalid biome ID
    InvalidBiomeId(i32),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::Io(ierr) => ierr.fmt(f)?,
            Error::Header(msg) => f.write_str(msg)?,
            Error::Nbt(e) => f.write_str(&format!("Region file contains invalid NBT: {}", e))?,
            Error::ChunkTooLarge(size) => {
                f.write_str(&format!("Chunk is too large: {} bytes", size))?
            }
            Error::InvalidCompression(id) => {
                f.write_str(&format!("Chunk uses invalid compression type {}", id))?
            }
            Error::UnknownBlock(name) => f.write_str(&format!("Chunk contains invalid block {}", name))?,
            Error::ChunkNotExist => f.write_str("The chunk does not exist")?,
            Error::UnsupportedDataVersion(_) => f.write_str("The chunk uses an unsupported data version. Feather currently only supports 1.18.1-1.18.2 region files.")?,
            Error::InvalidBlockType(s) => f.write_str(&format!("Chunk contains invalid block type {}", s))?,
            Error::MissingRootTag => f.write_str("Chunk is missing a root NBT tag")?,
            Error::IndexOutOfBounds => f.write_str("Section index out of bounds")?,
            Error::InvalidBiomeId(id) => write!(f, "Invalid biome ID {}", id)?,
        }

        Ok(())
    }
}

impl std::error::Error for Error {}

/// Loads the region at the specified position
/// from the specified world directory.
///
/// The world directory should be the root directory
/// of the world, e.g. `${SERVER_DIR}/world` for
/// normal servers.
///
/// This function does not actually load all the chunks
/// in the region into memory; it only reads the file's
/// header so that chunks can be retrieved later.
pub fn load_region(
    dir: &Path,
    pos: RegionPosition,
    world_height: WorldHeight,
) -> Result<RegionHandle, Error> {
    let mut file = {
        let buf = region_file_path(dir, pos);

        open_opts()
            .create(false)
            .open(buf.as_path())
            .map_err(Error::Io)?
    };

    let header = read_header(&mut file)?;

    let num_sectors =
        (file.metadata().map_err(Error::Io)?.len() + SECTOR_BYTES as u64 - 1) / SECTOR_BYTES as u64;

    let allocator = SectorAllocator::new(&header, num_sectors as u32);

    Ok(RegionHandle {
        file,
        header,
        allocator,
        world_height,
    })
}

/// Creates the region file at the given region position and initializes
/// a handle.
///
/// The world directory should be the root directory
/// of the world, e.g. `${SERVER_DIR}/world` for
/// normal servers.
///
/// # Warning
/// If the region file already exist, it will be __overwritten__.
/// Care must be taken to ensure that this function is only called
/// for nonexistent regions.
pub fn create_region(
    dir: &Path,
    pos: RegionPosition,
    world_height: WorldHeight,
) -> Result<RegionHandle, Error> {
    create_region_dir(dir).map_err(Error::Io)?;
    let mut file = {
        let buf = region_file_path(dir, pos);

        open_opts().create(true).open(buf.as_path())
    }
    .map_err(Error::Io)?;

    let header = RegionHeader::default();
    header.write_to(&mut file).map_err(Error::Io)?;

    let allocator = SectorAllocator::new(&header, 2);
    Ok(RegionHandle {
        file,
        header,
        allocator,
        world_height,
    })
}

fn open_opts() -> OpenOptions {
    OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .clone()
}

fn region_file_path(dir: &Path, pos: RegionPosition) -> PathBuf {
    let mut buf = dir.to_path_buf();
    buf.push(format!("region/r.{}.{}.mca", pos.x, pos.z));
    buf
}

fn create_region_dir(dir: &Path) -> Result<(), io::Error> {
    let mut dir = dir.to_path_buf();
    dir.push("region");
    fs::create_dir_all(dir.as_path())
}

/// Reads the region header from the given file.
fn read_header(file: &mut File) -> Result<RegionHeader, Error> {
    let len = {
        let metadata = file.metadata().map_err(Error::Io)?;
        metadata.len()
    };

    // The header consists of 8 KiB of data, so
    // we can return an error early if it's too small.
    if len < 8192 {
        return Err(Error::Header("The region header is too small."));
    }

    let mut header = RegionHeader {
        locations: vec![],
        timestamps: vec![],
    };

    // The first 4 KiB contains the location
    // and sector length data. The first three
    // bytes of a 4-byte value contain the offset,
    // while the next byte contains the sector length.
    for _ in 0..1024 {
        let val = file.read_u32::<BigEndian>().map_err(Error::Io)?;
        let offset = val >> 8;
        let count = val & 0b1111_1111;

        header
            .locations
            .push(ChunkLocation(SectorBlock { offset, count }));
    }

    // The next 4 KiB contains timestamp data - one
    // for each chunk.
    for _ in 0..1024 {
        let timestamp = file.read_u32::<BigEndian>().map_err(Error::Io)?;
        header.timestamps.push(timestamp);
    }

    Ok(header)
}

/// A region file's header contains information
/// about the positions and timestamps of chunks in the region
/// file.
struct RegionHeader {
    /// Locations of chunks in the file, relative to the start.
    locations: Vec<ChunkLocation>,
    /// UNIX timestamps (supposedly) indicating the last time a chunk
    /// was modified.
    timestamps: Vec<u32>,
}

impl Default for RegionHeader {
    fn default() -> Self {
        Self {
            locations: vec![
                ChunkLocation(SectorBlock {
                    offset: 0,
                    count: 0
                });
                REGION_SIZE * REGION_SIZE
            ],
            timestamps: vec![0; REGION_SIZE * REGION_SIZE],
        }
    }
}

impl RegionHeader {
    /// Returns the `ChunkLocation` for the given
    /// chunk position.
    ///
    /// If the given position is
    /// not inside the region this header is for,
    /// a panic will occur.
    fn location_for_chunk(&self, pos: ChunkPosition) -> ChunkLocation {
        let index = Self::index(pos);
        self.locations[index]
    }

    /// Sets the location for the given chunk position.
    fn set_location_for_chunk(&mut self, pos: ChunkPosition, location: ChunkLocation) {
        let index = Self::index(pos);
        self.locations[index] = location;
    }

    /// Writes this header to the given writer.
    fn write_to<W>(&self, w: &mut W) -> Result<(), io::Error>
    where
        W: Write,
    {
        for location in &self.locations {
            let value = (location.0.offset << 8) | (location.0.count & 0b1111_1111);
            w.write_u32::<BigEndian>(value)?;
        }

        for timestamp in &self.timestamps {
            w.write_u32::<BigEndian>(*timestamp)?;
        }

        Ok(())
    }

    fn index(pos: ChunkPosition) -> usize {
        ((pos.x & 31) + (pos.z & 31) * (REGION_SIZE as i32)) as usize
    }
}

/// Contains information about a chunk inside
/// a region file.
#[derive(Clone, Copy, Debug)]
struct ChunkLocation(SectorBlock);

impl ChunkLocation {
    /// Chunks in a region which have not been generated
    /// have a 0 offset and sector_count value.
    /// This function checks whether a chunk exists
    /// in a region file or not.
    pub fn exists(self) -> bool {
        self.0.offset != 0 && self.0.count != 0
    }
}

fn serialize_block_state(block: &BlockState) -> SerializedBlockState {
    SerializedBlockState {
        name: block.kind().namespaced_id().into(),
        properties: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sector_allocator() {
        let header = RegionHeader {
            locations: vec![
                ChunkLocation(SectorBlock {
                    offset: 0,
                    count: 0,
                }),
                ChunkLocation(SectorBlock {
                    offset: 6,
                    count: 5,
                }),
            ],
            timestamps: vec![0; 2],
        };

        let mut alloc = SectorAllocator::new(&header, 1024);

        assert_eq!(
            alloc.allocate(2),
            SectorBlock {
                offset: 2,
                count: 2
            }
        );
        assert_eq!(
            alloc.allocate(2),
            SectorBlock {
                offset: 4,
                count: 2
            }
        );
        assert_eq!(
            alloc.allocate(2),
            SectorBlock {
                offset: 11,
                count: 2,
            }
        );
        alloc.free(SectorBlock {
            offset: 2,
            count: 2,
        });
        assert_eq!(
            alloc.allocate(2),
            SectorBlock {
                offset: 2,
                count: 2
            }
        );
    }
}
