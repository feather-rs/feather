//! This module implements the loading and saving
//! of Anvil region files.

use crate::{block_entity::BlockEntityData, entity::EntityData};
use bitvec::{bitvec, vec::BitVec};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use feather_biomes::Biome;
use feather_blocks::BlockId;
use feather_chunk::{BitArray, Chunk, ChunkSection};
use feather_util::ChunkPosition;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt::{self, Display, Formatter};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{Cursor, SeekFrom};
use std::ops::Deref;
use std::path::PathBuf;
use std::{fs, io, iter};

mod blob;

/// The length and width of a region, in chunks.
const REGION_SIZE: usize = 32;

/// The data version supported by this code, currently corresponding
/// to 1.13.2.
const DATA_VERSION: i32 = 1631;

/// Length, in bytes, of a sector.
const SECTOR_BYTES: usize = 4096;

/// Represents the data for a chunk after the "Chunk [x, y]" tag.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ChunkRoot {
    level: ChunkLevel,
    data_version: i32,
}

/// Represents the level data for a chunk.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ChunkLevel {
    // TODO heightmaps, etc.
    #[serde(rename = "xPos")]
    x_pos: i32,
    #[serde(rename = "zPos")]
    z_pos: i32,
    sections: Vec<LevelSection>,
    biomes: Vec<i32>,
    entities: Vec<EntityData>,
    #[serde(rename = "TileEntities")]
    block_entities: Vec<BlockEntityData>,
    heightmaps: Heightmaps,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Heightmaps {
    // sometimes a few of those are missing, but we regenerate them anyways
    #[serde(with = "packed_u9", default)]
    light_blocking: Vec<u16>,
    #[serde(with = "packed_u9", default)]
    motion_blocking: Vec<u16>,
    #[serde(with = "packed_u9", default)]
    motion_blocking_no_leaves: Vec<u16>,
    #[serde(with = "packed_u9", default)]
    ocean_floor: Vec<u16>,
    #[serde(with = "packed_u9", default)]
    world_surface: Vec<u16>,
}

impl Heightmaps {
    // TODO remove this and use hermatite-nbt serde for serialization
    pub(crate) fn packed_i64_vec(unpacked_array: Vec<u16>) -> Vec<i64> {
        let mut vec = Vec::with_capacity(256);

        let mut unpacked_index = 0;
        let mut bits_left = 0;

        for _packed_index in 0..36 {
            let mut packed = 0i64;
            bits_left += 64;

            while bits_left > 0 {
                let unpacked = unpacked_array[unpacked_index] as i64;

                if unpacked > 0x1FF {
                    // Invalid heightmap value
                    panic!("Invalid heightmap value {}", unpacked);
                }

                let shift_amount = bits_left - 9;
                bits_left -= 9;

                let packed_u9 = if shift_amount >= 0 {
                    unpacked << shift_amount
                } else {
                    unpacked >> -shift_amount
                };

                packed |= packed_u9 as i64;

                if bits_left >= 0 {
                    unpacked_index += 1;
                }
            }

            vec.push(packed);
        }

        vec
    }
}

mod packed_u9 {
    // TODO IMPORTANT remove any panics on de/serialization, find out how to return Error instead

    // TODO write test
    use serde::de::{SeqAccess, Visitor};
    use serde::export::Formatter;
    use serde::ser::SerializeSeq;
    use serde::{Deserializer, Serializer};
    use std::marker::PhantomData;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u16>, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PackedVisitor(PhantomData<fn() -> u16>);

        impl<'de> Visitor<'de> for PackedVisitor {
            type Value = Vec<u16>;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a sequence of type long with length 36")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let len = seq.size_hint().unwrap(); // nbt always knows sequence size
                if len != 36 {
                    // Invalid sequence length
                    //return Err(Error::custom("sequence length not equal to 36"));
                    panic!("sequence length not equal to 36");
                }

                let mut unpacked_array = Vec::with_capacity(256);

                let mut unpacked_element = 0;
                let mut bits_left = 0;

                for _packed_index in 0..36 {
                    let packed: i64 = seq.next_element()?.unwrap(); // we already checked the length
                    bits_left += 64;

                    while bits_left > 0 {
                        let shift_amount = bits_left - 9;
                        bits_left -= 9;

                        let unpacked = if shift_amount >= 0 {
                            let mask = 0x1FFi64 << shift_amount;
                            (packed & mask) >> shift_amount
                        } else {
                            let mask = 0x1FFi64 >> -shift_amount;
                            (packed & mask) << -shift_amount
                        };

                        unpacked_element |= unpacked as u16;

                        if bits_left >= 0 {
                            unpacked_array.push(unpacked_element);
                            unpacked_element = 0;
                        }
                    }
                }

                Ok(unpacked_array)
            }
        }

        deserializer.deserialize_seq(PackedVisitor(PhantomData))
    }

    pub fn serialize<S>(unpacked_array: &Vec<u16>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // TODO use NBT LongArray type once implemented in hematite_nbt ( https://github.com/PistonDevelopers/hematite_nbt/pull/51)

        if unpacked_array.len() != 256 {
            // Invalid array length
            //return Err(Error::custom("array length not equal to 256"));
            panic!("array length not equal to 256");
        }

        let mut seq = serializer.serialize_seq(Some(36))?;

        let mut unpacked_index = 0;
        let mut bits_left = 0;

        for _packed_index in 0..36 {
            let mut packed = 0i64;
            bits_left += 64;

            while bits_left > 0 {
                let unpacked = unpacked_array[unpacked_index] as i64;

                if unpacked > 0x1FF {
                    // Invalid heightmap value
                    //return Err(Error::custom(format!("Invalid heightmap value {}", unpacked)));
                    panic!("Invalid heightmap value {}", unpacked)
                }

                let shift_amount = bits_left - 9;
                bits_left -= 9;

                let packed_u9 = if shift_amount >= 0 {
                    unpacked << shift_amount
                } else {
                    unpacked >> -shift_amount
                };

                packed |= packed_u9 as i64;

                if bits_left >= 0 {
                    unpacked_index += 1;
                }
            }

            seq.serialize_element(&packed)?;
        }

        seq.end()
    }
}

/// Represents a chunk section in a region file.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LevelSection {
    y: i8,
    #[serde(rename = "BlockStates")]
    states: Vec<i64>,
    palette: Vec<LevelPaletteEntry>,
    block_light: Vec<i8>,
    sky_light: Vec<i8>,
}

/// Represents a palette entry in a region file.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LevelPaletteEntry {
    /// The identifier of the type of this block
    name: Cow<'static, str>,
    /// Optional properties for this block
    properties: Option<LevelProperties>,
}

/// Represents the proprties for a palette entry.
#[derive(Serialize, Deserialize, Debug)]
pub struct LevelProperties {
    /// Map containing a list of property names to values.
    #[serde(flatten)]
    props: BTreeMap<Cow<'static, str>, Cow<'static, str>>,
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
        let cursor = Cursor::new(&buf[1..]);
        let root: ChunkRoot = match compression_type {
            1 => nbt::from_gzip_reader(cursor).map_err(Error::Nbt)?,
            2 => nbt::from_zlib_reader(cursor).map_err(Error::Nbt)?,
            _ => return Err(Error::InvalidCompression(compression_type)),
        };

        // Check data version
        if root.data_version != DATA_VERSION {
            return Err(Error::UnsupportedDataVersion(root.data_version));
        }

        let level = &root.level;

        let mut chunk = Chunk::new(original_pos);

        // Read sections
        for section in &level.sections {
            read_section_into_chunk(section, &mut chunk)?;
        }

        // Read biomes
        if level.biomes.len() != 256 {
            return Err(Error::IndexOutOfBounds);
        }
        for index in 0..256 {
            let id = level.biomes[index];
            chunk.biomes_mut()[index] =
                Biome::from_protocol_id(id).ok_or_else(|| Error::InvalidBiomeId(id))?;
        }

        // Chunk was not modified, but it thinks it was: disable this
        chunk.check_modified();

        chunk.recalculate_heightmap();

        Ok((chunk, level.entities.clone(), level.block_entities.clone()))
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

        // Write chunk to `ChunkRoot` tag.
        let root = chunk_to_chunk_root(chunk, entities, block_entities);

        let blob = blob::chunk_root_to_blob(root);

        // Write to intermediate buffer, because we need to know the length.
        let mut buf = Vec::with_capacity(4096);
        buf.write_u8(2).map_err(Error::Io)?; // Compression type: zlib

        blob.to_zlib_writer(&mut buf)
            .expect("Could not write chunk blob");

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

fn read_section_into_chunk(section: &LevelSection, chunk: &mut Chunk) -> Result<(), Error> {
    let data = &section.states;

    // Create palette
    let mut palette = vec![];
    for entry in &section.palette {
        // Construct properties map
        let mut props = BTreeMap::new();
        if let Some(entry_props) = entry.properties.as_ref() {
            props.extend(
                entry_props
                    .props
                    .iter()
                    .map(|(k, v)| (k.clone().into_owned(), v.clone().into_owned())),
            );
        }

        // Attempt to get block from the given values
        let block = BlockId::from_identifier_and_properties(&entry.name, &props)
            .ok_or_else(|| Error::InvalidBlock(entry.name.deref().to_owned()))?;
        palette.push(block);
    }

    // Create section
    // TODO don't clone data - need way around this
    let data = BitArray::from_raw(
        data.iter().map(|x| *x as u64).collect(),
        ((data.len() as f32 * 64.0) / 4096.0).ceil() as u8,
        4096,
    );

    // Light
    // convert raw lighting data (4bits / block) into a BitArray
    let convert_light_data = |light_data: &Vec<i8>| {
        let data = light_data
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
        BitArray::from_raw(data, 4, 4096)
    };

    if section.block_light.len() != 2048 || section.sky_light.len() != 2048 {
        return Err(Error::IndexOutOfBounds);
    }

    let block_light = convert_light_data(&section.block_light);
    let sky_light = convert_light_data(&section.sky_light);

    let chunk_section = ChunkSection::new(data, Some(palette), block_light, sky_light);

    if section.y >= 16 {
        // Haha... nope.
        return Err(Error::IndexOutOfBounds);
    }

    chunk.set_section_at(usize::from(section.y as u8), Some(chunk_section));

    Ok(())
}

fn chunk_to_chunk_root(
    chunk: &Chunk,
    entities: &[EntityData],
    block_entities: &[BlockEntityData],
) -> ChunkRoot {
    ChunkRoot {
        level: ChunkLevel {
            x_pos: chunk.position().x,
            z_pos: chunk.position().z,
            block_entities: block_entities.into(),
            sections: chunk
                .sections()
                .iter()
                .enumerate()
                .filter_map(|(y, sec)| sec.map(|sec| (y, sec.clone())))
                .map(|(y, mut section)| {
                    let palette = convert_palette(&mut section);
                    LevelSection {
                        y: y as i8,
                        states: section.data().inner().iter().map(|x| *x as i64).collect(),
                        palette,
                        block_light: slice_u64_to_i8(section.block_light().inner()).to_vec(),
                        sky_light: slice_u64_to_i8(section.sky_light().inner()).to_vec(),
                    }
                })
                .collect(),
            biomes: chunk
                .biomes()
                .iter()
                .map(|biome| biome.protocol_id())
                .collect(),
            entities: entities.into(),
            heightmaps: Heightmaps {
                light_blocking: chunk
                    .heightmaps()
                    .iter()
                    .map(|h| h.light_blocking())
                    .collect(),
                motion_blocking: chunk
                    .heightmaps()
                    .iter()
                    .map(|h| h.motion_blocking())
                    .collect(),
                motion_blocking_no_leaves: chunk
                    .heightmaps()
                    .iter()
                    .map(|h| h.motion_blocking_no_leaves())
                    .collect(),
                ocean_floor: chunk.heightmaps().iter().map(|h| h.ocean_floor()).collect(),
                world_surface: chunk
                    .heightmaps()
                    .iter()
                    .map(|h| h.world_surface())
                    .collect(),
            },
        },
        data_version: DATA_VERSION,
    }
}

fn convert_palette(section: &mut ChunkSection) -> Vec<LevelPaletteEntry> {
    section.convert_palette_to_section();
    raw_palette_to_palette_entries(section.palette().unwrap())
}

fn raw_palette_to_palette_entries(palette: &[BlockId]) -> Vec<LevelPaletteEntry> {
    palette
        .iter()
        .map(|block| {
            let props = block.to_properties_map();
            let identifier = block.identifier();

            LevelPaletteEntry {
                name: identifier.into(),
                properties: Some(LevelProperties {
                    props: props
                        .into_iter()
                        .map(|(k, v)| (Cow::from(k), Cow::from(v)))
                        .collect(),
                }),
            }
        })
        .collect()
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
        let mut used_sectors = bitvec![0; file_size as usize];

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

                    (block.offset..block.offset + block.count)
                        .for_each(|sector| self.used_sectors.set(sector as usize, true));

                    return block;
                }
            }
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
    InvalidBlock(String),
    /// The chunk does not exist
    ChunkNotExist,
    /// The chunk uses an unsupported data version
    UnsupportedDataVersion(i32),
    /// The palette for the chunk contained in invalid block type
    InvalidBlockType,
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
            Error::InvalidBlock(name) => f.write_str(&format!("Chunk contains invalid block {}", name))?,
            Error::ChunkNotExist => f.write_str("The chunk does not exist")?,
            Error::UnsupportedDataVersion(_) => f.write_str("The chunk uses an unsupported data version. Feather currently only supports 1.13.2 region files.")?,
            Error::InvalidBlockType => f.write_str("Chunk contains invalid block type")?,
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
pub fn load_region(dir: &PathBuf, pos: RegionPosition) -> Result<RegionHandle, Error> {
    let mut file = {
        let buf = region_file_path(dir, pos);

        open_opts()
            .create(false)
            .open(buf.as_path())
            .map_err(Error::Io)?
    };

    let header = read_header(&mut file)?;

    let num_sectors = file.metadata().map_err(Error::Io)?.len() / SECTOR_BYTES as u64;

    let allocator = SectorAllocator::new(&header, num_sectors as u32);

    Ok(RegionHandle {
        file,
        header,
        allocator,
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
pub fn create_region(dir: &PathBuf, pos: RegionPosition) -> Result<RegionHandle, Error> {
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
    })
}

fn open_opts() -> OpenOptions {
    OpenOptions::new()
        .read(true)
        .write(true)
        .append(false)
        .clone()
}

fn region_file_path(dir: &PathBuf, pos: RegionPosition) -> PathBuf {
    let mut buf = dir.clone();
    buf.push(format!("region/r.{}.{}.mca", pos.x, pos.z));
    buf
}

fn create_region_dir(dir: &PathBuf) -> Result<(), io::Error> {
    let mut dir = dir.clone();
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

/// A region contains a 32x32 grid of chunk columns.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct RegionPosition {
    x: i32,
    z: i32,
}

impl RegionPosition {
    /// Returns the coordinates of the region corresponding
    /// to the specified chunk position.
    pub fn from_chunk(chunk_coords: ChunkPosition) -> Self {
        Self {
            x: chunk_coords.x >> 5,
            z: chunk_coords.z >> 5,
        }
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

    #[test]
    fn test_packed_u9_de_serializer() {
        // TODO fill with data
        let unpacked = vec![0u16; 256];
        let packed = vec![0i64; 36];

        // Test serializer
        // TODO use packed_u9::deserialize
        assert_eq!(Heightmaps::packed_i64_vec(unpacked), packed);
        // Test deserializer
        // TODO
    }
}
