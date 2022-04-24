//! Implements the modernized, efficient world format used
//! by default in Feather.
//!
//! # Comparison to vanilla (Anvil) format
//! * The format is currently much simpler, because Feather doesn't yet support
//! much of the data stored in vanilla saves. There are fewer fields to worry
//! about and fewer edge cases with missing data (which happens with upgraded world
//! saves in vanilla).
//! * The format uses `zstd` for compression instead of `zlib`, which improves
//! performance and compression rates.
//! * The format uses CBOR for encoding rather than NBT. CBOR can be slightly smaller
//! than NBT, and it is more widely supported.
//!
//! # Format specification
//! Similar to Anvil, we pack each 32x32 region of chunks into one file.
//! Also similar to Anvil, each file is divided into sectors of 1 KiB each.
//!
//! The file begins with a 12 KiB header containing metadata for each of the 1024
//! chunks in the region. Each metadata entry consists of 8 bytes:
//! * bytes 0..4: the offset from the start of the file of the stored chunk, _in sectors_.
//!   If set to a value less than 2 (which would point into the header), then the chunk does not exist.
//! * bytes 4..8: the size of the chunk in the file, _in bytes_.
//! * bytes 8..12: the uncompressed size of the chunk, _in bytes_.
//! Both values are encoded big-endian.

use std::{
    fs::{self, File, OpenOptions},
    io::{Cursor, Read, Seek, SeekFrom, Write},
    iter,
    ops::Range,
    path::Path,
    sync::Arc,
};

use anyhow::Context;
use header::{Entry, Header};
use libcraft::{biome::BiomeList, Chunk, RegionPosition, Sections, REGION_SIZE};
use model::ChunkModel;
use range_alloc::RangeAllocator;
use zstd::bulk::{Compressor, Decompressor};

mod header;
mod model;
mod range_alloc;

const INITIAL_SECTORS: u32 = 256;
const SECTOR_GROWTH_FACTOR: u32 = 2;

const SECTOR_OFFSET: u32 = 12;

const SECTOR_SIZE: u64 = 1024;

const COMPRESSION_LEVEL: i32 = 9;

/// Handle to an open region file.
pub struct RegionHandle {
    pos: RegionPosition,
    file: File,
    header: Header,
    sector_allocator: RangeAllocator,

    biomes: Arc<BiomeList>,
    sections: Sections,
    min_y: i32,

    buffer: Vec<u8>,
    compression_buffer: Vec<u8>,

    compressor: Compressor<'static>,
    decompressor: Decompressor<'static>,
}

impl RegionHandle {
    /// Opens a handle to the given region, creating it if allowed and
    /// necessary.
    ///
    /// `dir` points to the directory where region files are stored.
    pub fn open(
        dir: impl AsRef<Path>,
        region: RegionPosition,
        allow_creation: bool,
        biomes: Arc<BiomeList>,
        sections: Sections,
        min_y: i32,
    ) -> anyhow::Result<Self> {
        if !dir.as_ref().exists() {
            fs::create_dir_all(dir.as_ref()).ok();
        }

        let path = dir.as_ref().join(file_name(region));
        let was_created = !path.exists();
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .append(false)
            .create(allow_creation)
            .open(path)?;

        let header = if was_created {
            let header = Header::default();
            header.write_to(&mut file)?;
            header
        } else {
            Header::read_from(&mut file)?
        };

        let size = if was_created {
            INITIAL_SECTORS
        } else {
            u32::try_from(file.metadata()?.len() / SECTOR_SIZE)? - SECTOR_OFFSET
        };

        let mut sector_allocator = RangeAllocator::new(size);
        for entry in header.iter() {
            if entry.exists() {
                sector_allocator.mark_used(entry_range(entry));
            }
        }

        let mut this = Self {
            pos: region,
            file,
            header,
            sector_allocator,
            biomes,
            sections,
            min_y,
            buffer: Vec::with_capacity(2048),
            compression_buffer: Vec::with_capacity(2048),

            compressor: Compressor::new(COMPRESSION_LEVEL)?,
            decompressor: Decompressor::new()?,
        };
        this.pad_file()?;
        Ok(this)
    }

    /// Writes a chunk into the region.
    pub fn save(&mut self, chunk: &Chunk) -> anyhow::Result<()> {
        let (chunk_x, chunk_z) = self
            .pos
            .chunk_offset(chunk.position())
            .context("chunk does not lie within this region")?;
        let old_entry = self.header.get(chunk_x, chunk_z);

        let model = ChunkModel::from_chunk(chunk, &self.biomes);
        ciborium::ser::into_writer(&model, &mut self.compression_buffer)
            .context("failed to serialize chunk")?;
        self.compressor
            .compress_to_buffer(&self.compression_buffer, &mut self.buffer)?;

        let data_size = self.buffer.len() as u64;
        let num_sectors: u32 = ((data_size + SECTOR_SIZE - 1) / SECTOR_SIZE).try_into()?;
        let range = match self.sector_allocator.allocate(num_sectors) {
            Some(r) => r,
            None => {
                let size = self.sector_allocator.size();
                self.sector_allocator.grow_to(size * SECTOR_GROWTH_FACTOR);
                self.pad_file()?;
                self.sector_allocator
                    .allocate(num_sectors)
                    .expect("did not grow enough")
            }
        };

        // Write the data
        self.file.seek(SeekFrom::Start(
            (range.start + SECTOR_OFFSET) as u64 * SECTOR_SIZE,
        ))?;
        self.file.write_all(&self.buffer)?;

        // Update the header (in memory, not yet committed)
        let new_entry = Entry {
            offset_in_sectors: SECTOR_OFFSET + range.start,
            size_in_bytes: self.buffer.len().try_into()?,
            uncompressed_size_in_bytes: self.compression_buffer.len().try_into()?,
        };
        self.header.set(chunk_x, chunk_z, new_entry);

        self.buffer.clear();
        self.compression_buffer.clear();

        // NB: we've waited to free the old entry from the allocator until _after_
        // the new data has been written. This way, in the event of a crash,
        // the old data remains intact and referenced in the header. (Otherwise,
        // it would be overwritten because the space from the old data was reused.)
        if old_entry.exists() {
            self.sector_allocator.deallocate(entry_range(old_entry));
        }

        // Commit the header
        self.header.write_to(&mut self.file)?;

        Ok(())
    }

    /// Reads a chunk from the region.
    pub fn load(&mut self, chunk_x: u32, chunk_z: u32) -> anyhow::Result<Option<Chunk>> {
        assert!(chunk_x < REGION_SIZE as u32);
        assert!(chunk_z < REGION_SIZE as u32);
        let entry = self.header.get(chunk_x, chunk_z);
        if !entry.exists() {
            return Ok(None);
        }

        self.buffer.clear();
        self.compression_buffer.clear();

        let offset = entry.offset_in_sectors as u64 * SECTOR_SIZE;
        self.file.seek(SeekFrom::Start(offset))?;

        // Prep the buffer
        self.compression_buffer
            .extend(iter::repeat(0).take(entry.size_in_bytes as usize));
        self.file.read_exact(&mut self.compression_buffer)?;
        self.buffer
            .reserve(entry.uncompressed_size_in_bytes as usize);
        self.decompressor
            .decompress_to_buffer(&self.compression_buffer, &mut self.buffer)?;

        let model: ChunkModel = ciborium::de::from_reader(Cursor::new(&self.buffer))?;

        self.buffer.clear();
        self.compression_buffer.clear();

        let chunk = model
            .to_chunk(
                &self.biomes,
                self.pos.chunk(chunk_x, chunk_z),
                self.sections,
                self.min_y,
            )
            .context("malformed chunk")?;

        Ok(Some(chunk))
    }

    fn pad_file(&mut self) -> anyhow::Result<()> {
        let min_size =
            self.sector_allocator.size() as u64 * SECTOR_SIZE + SECTOR_SIZE * SECTOR_OFFSET as u64;
        let current_size = self.file.metadata()?.len();

        if current_size < min_size {
            let num_zeros = min_size - current_size;
            log::debug!("Growing region {:?} by {} KiB", self.pos, num_zeros / 1024);
            let buf = vec![0u8; num_zeros.try_into()?];
            self.file.seek(SeekFrom::End(0))?;
            self.file.write_all(&buf)?;
        }

        Ok(())
    }
}

fn entry_range(entry: Entry) -> Range<u32> {
    assert!(entry.exists());
    Range {
        start: entry.offset_in_sectors - SECTOR_OFFSET,
        end: entry.offset_in_sectors - SECTOR_OFFSET
            + (entry.size_in_bytes + SECTOR_SIZE as u32 - 1) / SECTOR_SIZE as u32,
    }
}

fn file_name(region: RegionPosition) -> String {
    format!("r.{}.{}.freg", region.x, region.z)
}

#[cfg(test)]
mod tests {
    use libcraft::{BlockKind, BlockState, ChunkPosition};
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn read_write_single_chunk() {
        let dir = tempdir().unwrap();

        let mut region = RegionHandle::open(
            dir.path(),
            RegionPosition { x: 0, z: 0 },
            true,
            Arc::new(BiomeList::vanilla()),
            Sections(16),
            0,
        )
        .unwrap();

        let mut chunk = Chunk::new(ChunkPosition { x: 0, z: 0 }, Sections(16), 0);
        for y in 0..64 {
            for x in 0..16 {
                for z in 0..16 {
                    chunk
                        .set_block_at(x, y, z, BlockState::new(BlockKind::OakWood), false)
                        .unwrap();
                }
            }
        }

        region.save(&chunk).unwrap();

        let loaded_chunk = region.load(0, 0).unwrap().unwrap();
        for y in 0..64 {
            for x in 0..16 {
                for z in 0..16 {
                    assert_eq!(
                        loaded_chunk.block_at(x, y, z),
                        Some(BlockState::new(BlockKind::OakWood))
                    );
                }
            }
        }

        assert!(region.load(0, 1).unwrap().is_none());

        chunk.set_position(ChunkPosition { x: 0, z: 1 });
        region.save(&chunk).unwrap();
        region.load(0, 0).unwrap().unwrap();
        region.load(0, 1).unwrap().unwrap();
        chunk.set_position(ChunkPosition { x: 0, z: 0 });
        region.save(&chunk).unwrap();
        region.load(0, 0).unwrap();
        region.load(0, 1).unwrap();
    }
}
