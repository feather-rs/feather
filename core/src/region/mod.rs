//! This module implements the loading and saving
//! of Anvil region files.

use crate::world::ChunkPosition;
use byteorder::{BigEndian, ReadBytesExt};
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use crate::world::chunk::Chunk;
use std::io::SeekFrom;
use flate2::bufread::{ZlibDecoder, GzDecoder};

/// The length and width of a region, in chunks.
const REGION_SIZE: usize = 32;

/// A region file handle.
pub struct RegionHandle {
    /// The region file.
    file: File,
    /// The position of this region.
    pos: RegionPosition,
    /// The region file's header, pre-loaded into memory.
    header: RegionHeader,
}

impl RegionHandle {
    /// Loads the chunk at the given position (global, not region-relative).
    ///
    /// The specified chunk is expected to be contained within this region.
    ///
    /// # Panics
    /// If the specified chunk position is not within this
    /// region file.
    pub fn load_chunk(&mut self, pos: ChunkPosition) -> Result<Chunk, Error> {
        // Get the offset of the chunk within the file
        // so that it can be read.
        let offset = self.header.location_for_chunk(pos).offset;

        // Seek to the offset position. Note that since the offset in the header
        // is in "sectors" of 4KiB each, the value needs to be multiplied by 4096
        // to get the offset in bytes.
        self.file.seek(SeekFrom::Start(offset as u64 * 4096)).map_err(|e| Error::Io(e))?;

        // A chunk begins with a four-byte, big-endian value
        // indicating the exact length of the chunk's data
        // in bytes.
        let len = self.file.read_u32::<BigEndian>().map_err(|e| Error::Io(e))?;

        // Avoid DoS attacks
        if len > 1048576 {
            return Err(Error::ChunkTooLarge(len as usize));
        }

        // Read `len` bytes into memory.
        let mut buf = Vec::with_capacity(len as usize);
        let amnt_read = self.file.read(&mut buf).map_err(|e| Error::Io(e))?;

        if amnt_read != len as usize {
            return Err(Error::ChunkTooLarge(0));
        }

        // The compression type is indicated by a byte,
        // 1 corresponds to gzip compression, while 2
        // corresponds to zlib.
        let compression_type = self.file.read_u8().map_err(|e| Error::Io(e))?;

        let mut uncompressed = vec![];

        // Uncompress the data
        match compression_type {
            1 => {
                let mut decoder = GzDecoder::new(buf.as_slice());
                decoder.read(&mut uncompressed).map_err(|e| Error::BadCompression(e))?;
            },
            2 => {
                let mut decoder = ZlibDecoder::new(buf.as_slice());
                decoder.read(&mut uncompressed).map_err(|e| Error::BadCompression(e))?;
            },
            _ => return Err(Error::InvalidCompression(compression_type)),
        }

        // Read NBT-encoded chunk
        let nbt = rnbt::parse_bytes(&uncompressed).map_err(|_| Error::Nbt)?;
        let root = nbt.compound().ok_or_else(|| Error::Nbt)?;

        let level = root.get("Level").ok_or_else(|| Error::Nbt)?.compound().ok_or_else(|| Error::Nbt)?;

        let mut chunk = Chunk::new(pos);

        let sections = level.get("Sections").ok_or_else(|| Error::Nbt)?.list().ok_or_else(|| Error::Nbt)?;
        for section in sections.values {
            let section = section.compound().ok_or_else(|| Error::Nbt)?;

            let index = section.get("Y").ok_or_else(|| Error::Nbt)?.int().ok_or_else(|| Error::Nbt)?.value as usize;

            let mem_section = chunk.section_mut(index);

            // Set blocks + palette in section.
            let block_states = section.get("BlockStates").ok_or_else(|| Error::Nbt)?.long_array().ok_or_else(|| Error::Nbt)?;
            let palette = section.get("Palette").ok_or_else(|| Error::Nbt)?.list().ok_or_else(|| Error::Nbt)?;

            let mut block_state_buf = Vec::with_capacity(block_states.values.len());
            for x in block_states.values {
                block_state_buf.push(x as u64);
            }

            // Read palette. Unfortunately, Mojang
            // insists on using string IDs instead of numerical
            // IDs in the world format palette. This seems like
            // a horrible waste of space, but too bad.
            // TODO
        }

        Ok(chunk)
    }
}

/// An error which occurred during region file processing.
#[derive(Debug)]
pub enum Error {
    /// An IO error occurred.
    Io(io::Error),
    /// The region file header was invalid.
    Header(&'static str),
    /// The region file contained invalid NBT data.
    Nbt,
    /// The chunk was too large
    ChunkTooLarge(usize),
    /// The chunk contained an invalid compression type
    InvalidCompression(u8),
    /// We were unable to decompress the chunk
    BadCompression(io::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::Io(ierr) => ierr.fmt(f)?,
            Error::Header(msg) => f.write_str(msg)?,
            Error::Nbt => f.write_str("Region file contains invalid NBT")?,
            Error::ChunkTooLarge(size) => f.write_str(&format!("Chunk is too large: {} bytes", size))?,
            Error::InvalidCompression(id) => f.write_str(&format!("Chunk uses invalid compression type {}", id))?,
            Error::BadCompression(err) => {
                f.write_str("Unable to decompress chunk data: ")?;
                err.fmt(f)?;
            }
        }

        Ok(())
    }
}

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
pub fn load_region(dir: &str, pos: RegionPosition) -> Result<RegionHandle, Error> {
    let mut file = {
        let mut buf = PathBuf::from(dir);
        buf.push(format!("region/r.{}.{}.mca", pos.x, pos.z));

        File::open(buf.as_path()).map_err(|e| Error::Io(e))?
    };

    let header = read_header(&mut file)?;

    Ok(RegionHandle { pos, file, header })
}

/// Reads the region header from the given file.
fn read_header(file: &mut File) -> Result<RegionHeader, Error> {
    let len = {
        let metadata = file.metadata().map_err(|e| Error::Io(e))?;
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
    for _ in 0..4096 {
        let val = file.read_u32::<BigEndian>().map_err(|e| Error::Io(e))?;
        let offset = val >> 8;
        let sector_count = (val & 0b11111111) as u8;

        header.locations.push(ChunkLocation {
            offset,
            sector_count,
        });
    }

    // The next 4 KiB contains timestamp data - one
    // for each chunk.
    for _ in 0..4096 {
        let timestamp = file.read_u32::<BigEndian>().map_err(|e| Error::Io(e))?;
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
struct ChunkLocation {
    /// The offset of the chunk from the start of the file
    /// in 4 KiB sectors such that a value of 2 corresponds
    /// to byte 8192 in the file.
    offset: u32,
    /// The length of the data for the chunk, also
    /// in 4 KiB sectors. This value is always rounded up.
    sector_count: u8,
}

impl ChunkLocation {
    /// Chunks in a region which have not been generated
    /// have a 0 offset and sector_count value.
    /// This function checks whether a chunk exists
    /// in a region file or not.
    pub fn exists(&self) -> bool {
        self.offset != 0 && self.sector_count != 0
    }
}

/// A region contains a 32x32 grid of chunk columns.
#[derive(Debug, Clone, Copy)]
pub struct RegionPosition {
    x: i32,
    z: i32,
}

impl RegionPosition {
    /// Returns the coordinates of the region corresponding
    /// to the specified chunk position.
    fn from_chunk(chunk_coords: ChunkPosition) -> Self {
        Self {
            x: chunk_coords.x / 32,
            z: chunk_coords.z / 32,
        }
    }
}
