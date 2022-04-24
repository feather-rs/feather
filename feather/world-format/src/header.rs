use std::{
    fs::File,
    io::{Cursor, Read, Seek, SeekFrom, Write},
};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use libcraft::REGION_SIZE;

use crate::SECTOR_OFFSET;

pub const HEADER_SIZE: usize = 12 * 1024;
const NUM_CHUNKS: usize = REGION_SIZE * REGION_SIZE;

#[derive(Debug, Clone)]
pub struct Header {
    chunks: Vec<Entry>,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            chunks: vec![Entry::default(); NUM_CHUNKS],
        }
    }
}

impl Header {
    pub fn read_from(file: &mut File) -> anyhow::Result<Self> {
        file.seek(SeekFrom::Start(0))?;
        let mut buf = [0u8; HEADER_SIZE];
        file.read_exact(&mut buf)?;

        let mut cursor = Cursor::new(&buf[..]);
        let mut chunks = Vec::with_capacity(NUM_CHUNKS);
        for _ in 0..NUM_CHUNKS {
            chunks.push(Entry {
                offset_in_sectors: cursor.read_u32::<BigEndian>()?,
                size_in_bytes: cursor.read_u32::<BigEndian>()?,
                uncompressed_size_in_bytes: cursor.read_u32::<BigEndian>()?,
            });
        }

        Ok(Self { chunks })
    }

    pub fn write_to(&self, file: &mut File) -> anyhow::Result<()> {
        let mut buf = [0u8; HEADER_SIZE];
        let mut cursor = Cursor::new(&mut buf[..]);

        for entry in &self.chunks {
            cursor.write_u32::<BigEndian>(entry.offset_in_sectors)?;
            cursor.write_u32::<BigEndian>(entry.size_in_bytes)?;
            cursor.write_u32::<BigEndian>(entry.uncompressed_size_in_bytes)?;
        }

        file.seek(SeekFrom::Start(0))?;
        file.write_all(&buf)?;

        Ok(())
    }

    /// Note that the chunk coordinates are relative to the region
    /// (i.e., modulo 32)
    pub fn get(&self, chunk_x: u32, chunk_z: u32) -> Entry {
        self.chunks[self.index(chunk_x, chunk_z)]
    }

    pub fn set(&mut self, chunk_x: u32, chunk_z: u32, entry: Entry) {
        let index = self.index(chunk_x, chunk_z);
        self.chunks[index] = entry;
    }

    pub fn iter(&self) -> impl Iterator<Item = Entry> + '_ {
        self.chunks.iter().copied()
    }

    fn index(&self, chunk_x: u32, chunk_z: u32) -> usize {
        assert!(chunk_x < REGION_SIZE as u32);
        assert!(chunk_z < REGION_SIZE as u32);
        (chunk_x as usize * REGION_SIZE) + chunk_z as usize
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Entry {
    pub offset_in_sectors: u32,
    pub size_in_bytes: u32,
    pub uncompressed_size_in_bytes: u32,
}

impl Entry {
    pub fn exists(&self) -> bool {
        self.offset_in_sectors >= SECTOR_OFFSET
    }
}
