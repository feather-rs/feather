//! This module implements the loading and saving
//! of Anvil region files.

use crate::world::ChunkPosition;
use byteorder::{BigEndian, ReadBytesExt};
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

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

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Header(&'static str),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::Io(ierr) => ierr.fmt(f)?,
            Error::Header(msg) => f.write_str(msg)?,
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
