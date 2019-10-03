//! This module implements the loading and saving (soon)
//! of Anvil region files.

use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::prelude::*;
use std::io::{Cursor, SeekFrom};
use std::path::PathBuf;
use std::{io, iter};

use byteorder::{BigEndian, ReadBytesExt};
use serde::Deserialize;

use crate::save::entity::EntityData;
use crate::world::block::*;
use crate::world::chunk::{BitArray, Chunk, ChunkSection};
use crate::world::ChunkPosition;
use crate::Biome;
use bitvec::bitvec;
use bitvec::vec::BitVec;

mod blob;

/// The length and width of a region, in chunks.
const REGION_SIZE: usize = 32;

/// The data version supported by this code, currently corresponding
/// to 1.13.2.
const DATA_VERSION: i32 = 1631;

/// Represents the data for a chunk after the "Chunk [x, y]" tag.
#[derive(Serialize, Deserialize, Debug)]
struct ChunkRoot {
    #[serde(rename = "Level")]
    level: ChunkLevel,
    #[serde(rename = "DataVersion")]
    data_version: i32,
}

/// Represents the level data for a chunk.
#[derive(Serialize, Deserialize, Debug)]
struct ChunkLevel {
    // TODO heightmaps, etc.
    #[serde(rename = "xPos")]
    x_pos: i32,
    #[serde(rename = "zPos")]
    z_pos: i32,
    #[serde(rename = "Sections")]
    sections: Vec<LevelSection>,
    #[serde(rename = "Biomes")]
    biomes: Vec<i32>,
    #[serde(rename = "Entities")]
    entities: Vec<EntityData>,
}

/// Represents a chunk section in a region file.
#[derive(Serialize, Deserialize, Debug)]
struct LevelSection {
    #[serde(rename = "Y")]
    y: i8,
    #[serde(rename = "BlockStates")]
    states: Vec<i64>,
    #[serde(rename = "Palette")]
    palette: Vec<LevelPaletteEntry>,
    #[serde(rename = "BlockLight")]
    block_light: Vec<i8>,
    #[serde(rename = "SkyLight")]
    sky_light: Vec<i8>,
}

/// Represents a palette entry in a region file.
#[derive(Serialize, Deserialize, Debug)]
struct LevelPaletteEntry {
    /// The identifier of the type of this block
    #[serde(rename = "Name")]
    name: String,
    /// Optional properties for this block
    #[serde(rename = "Properties")]
    props: Option<LevelProperties>,
}

/// Represents the proprties for a palette entry.
#[derive(Serialize, Deserialize, Debug)]
struct LevelProperties {
    /// Map containing a list of property names to values.
    #[serde(flatten)]
    props: HashMap<String, String>,
}

/// A block of sectors in a region file.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct SectorBlock {
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
    ) -> Result<(Chunk, Vec<EntityData>), Error> {
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
        // is in "sectors" of 4KiB each, the value needs to be multiplied by 4096
        // to get the offset in bytes.
        self.file
            .seek(SeekFrom::Start(u64::from(offset) * 4096))
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

        Ok((chunk, level.entities.to_vec()))
    }

    /// Saves the given chunk to this region file. The header will be updated
    /// accordingly and saved as well.
    pub fn save_chunk(&mut self, chunk: &Chunk) {
        unimplemented!()
    }
}

fn read_section_into_chunk(section: &LevelSection, chunk: &mut Chunk) -> Result<(), Error> {
    let data = &section.states;

    // Create palette
    let mut palette = vec![];
    for entry in &section.palette {
        // Construct properties map
        let mut props = HashMap::new();
        if let Some(entry_props) = entry.props.as_ref() {
            props.extend(
                entry_props
                    .props
                    .iter()
                    .map(|(k, v)| (k.clone(), v.clone())),
            );
        }

        // Attempt to get block from the given values
        let block = Block::from_name_and_props(&entry.name, &props).ok_or(Error::InvalidBlock)?;
        palette.push(block.native_state_id());
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
            if is_used {
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
        self.used_sectors
            .extend(iter::repeat(true).take(min_size as usize));

        SectorBlock {
            offset: self.used_sectors.len() as u32,
            count: min_size,
        }
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
    InvalidBlock,
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
            Error::InvalidBlock => f.write_str("Chunk contains invalid block")?,
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
        let mut buf = dir.clone();
        buf.push(format!("region/r.{}.{}.mca", pos.x, pos.z));

        File::open(buf.as_path()).map_err(Error::Io)?
    };

    let header = read_header(&mut file)?;

    let num_sectors = file.metadata().map_err(Error::Io)?.len() / 4096;

    let allocator = SectorAllocator::new(&header, num_sectors as u32);

    Ok(RegionHandle {
        file,
        header,
        allocator,
    })
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

impl RegionHeader {
    /// Returns the `ChunkLocation` for the given
    /// chunk position. If the given position is
    /// not inside the region this header is for,
    /// a panic will occur.
    fn location_for_chunk(&self, pos: ChunkPosition) -> ChunkLocation {
        let index = (pos.x & 31) + (pos.z & 31) * (REGION_SIZE as i32);
        self.locations[index as usize]
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
}
