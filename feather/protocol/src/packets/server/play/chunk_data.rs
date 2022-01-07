use std::{
    fmt::{self, Debug},
    marker::PhantomData,
    sync::Arc,
};

use base::{Chunk, ChunkHandle, ChunkLock, ChunkPosition, ChunkSection};
use blocks::BlockId;
use libcraft_core::Biome;
use serde::{
    de,
    de::{SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};

use crate::{io::VarInt, Nbt, ProtocolVersion, Readable, Writeable};

#[derive(Serialize, Deserialize)]
struct Heightmaps {
    #[serde(rename = "MOTION_BLOCKING")]
    #[serde(serialize_with = "nbt::i64_array")]
    #[serde(deserialize_with = "deserialize_i64_37")]
    motion_blocking: [i64; 37],
}

#[derive(Debug, Clone)]
pub enum ChunkDataKind {
    /// Load a chunk on the client. Sends all sections + biomes.
    LoadChunk,
    /// Overwrite an existing chunk on the client. Sends
    /// only the sections in `sections`.
    OverwriteChunk { sections: Vec<usize> },
}

/// Packet to load a chunk on the client.
#[derive(Clone)]
pub struct ChunkData {
    /// The chunk to send.
    pub chunk: ChunkHandle,

    /// Whether this packet will load a chunk on
    /// the client or overwrite an existing one.
    pub kind: ChunkDataKind,
}

impl Debug for ChunkData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("ChunkData");
        debug_struct.field("position", &self.chunk.read().position());
        debug_struct.field("kind", &self.kind);
        debug_struct.finish()
    }
}

impl ChunkData {
    fn should_skip_section(&self, y: usize) -> bool {
        match &self.kind {
            ChunkDataKind::LoadChunk => false,
            ChunkDataKind::OverwriteChunk { sections } => !sections.contains(&y),
        }
    }
}

impl Writeable for ChunkData {
    fn write(&self, buffer: &mut Vec<u8>, version: ProtocolVersion) -> anyhow::Result<()> {
        let chunk = self.chunk.read();

        chunk.position().x.write(buffer, version)?;
        chunk.position().z.write(buffer, version)?;

        let full_chunk = matches!(self.kind, ChunkDataKind::LoadChunk);
        full_chunk.write(buffer, version)?;

        // Compute primary bit mask
        let mut bitmask = 0;
        for (y, section) in chunk.sections().iter().enumerate().skip(1).take(16) {
            if section.is_some() {
                if self.should_skip_section(y) {
                    continue;
                }

                bitmask |= 1 << (y - 1) as i32;
            }
        }
        VarInt(bitmask).write(buffer, version)?;

        let heightmaps = build_heightmaps(&chunk);
        Nbt(heightmaps).write(buffer, version)?;

        if full_chunk {
            // Write biomes (only if we're sending a new chunk)
            VarInt(1024).write(buffer, version)?; // length of biomes
            for &biome in chunk.biomes().as_slice() {
                VarInt(biome.id() as i32).write(buffer, version)?;
            }
        }

        // Sections
        let mut data = Vec::new();
        for (y, section) in chunk.sections().iter().enumerate().skip(1).take(16) {
            if let Some(section) = section {
                if self.should_skip_section(y) {
                    continue;
                }
                encode_section(section, &mut data, version)?;
            }
        }
        VarInt(data.len() as i32).write(buffer, version)?;
        buffer.extend_from_slice(&data);

        VarInt(0).write(buffer, version)?; // number of block entities - always 0 for Feather

        Ok(())
    }
}

fn build_heightmaps(chunk: &Chunk) -> Heightmaps {
    let mut motion_blocking = [0; 37];
    let chunk_motion_blocking = chunk.heightmaps().motion_blocking.as_u64_slice();
    motion_blocking.copy_from_slice(bytemuck::cast_slice::<_, i64>(chunk_motion_blocking));
    Heightmaps { motion_blocking }
}

fn encode_section(
    section: &ChunkSection,
    buffer: &mut Vec<u8>,
    version: ProtocolVersion,
) -> anyhow::Result<()> {
    (section.non_air_blocks() as u16).write(buffer, version)?;
    (section.blocks().data().bits_per_value() as u8).write(buffer, version)?;

    if let Some(palette) = section.blocks().palette() {
        VarInt(palette.len() as i32).write(buffer, version)?;
        for &block in palette.as_slice() {
            VarInt(block.vanilla_id() as i32).write(buffer, version)?;
        }
    }

    let data = section.blocks().data().as_u64_slice();
    VarInt(data.len() as i32).write(buffer, version)?;
    for &x in data {
        x.write(buffer, version)?;
    }

    Ok(())
}

impl Readable for ChunkData {
    fn read(buffer: &mut std::io::Cursor<&[u8]>, version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let chunk_x = i32::read(buffer, version)?;
        let chunk_z = i32::read(buffer, version)?;

        let mut chunk = Chunk::new(ChunkPosition {
            x: chunk_x,
            z: chunk_z,
        });

        let full_chunk = bool::read(buffer, version)?;
        let chunk_data_kind: ChunkDataKind = match full_chunk {
            true => ChunkDataKind::LoadChunk,
            false => ChunkDataKind::OverwriteChunk { sections: vec![] },
        };

        let primary_bit_mask = VarInt::read(buffer, version)?.0;
        let heightmaps: Nbt<Heightmaps> = Nbt::read(buffer, version)?;
        let heightmaps = heightmaps.0;
        for (heightmaps_index, i) in heightmaps.motion_blocking.iter().enumerate() {
            chunk
                .heightmaps_mut()
                .motion_blocking
                .set_height_index(heightmaps_index, *i);
        }

        if full_chunk {
            let biomes_length = VarInt::read(buffer, version)?.0;
            assert_eq!(biomes_length, 1024);
            for y in 0..64 {
                for z in 0..4 {
                    for x in 0..4 {
                        chunk.biomes_mut().set(
                            x,
                            y,
                            z,
                            Biome::from_id(VarInt::read(buffer, version)?.0 as u32)
                                .unwrap_or(Biome::Plains),
                        );
                    }
                }
            }
        }

        VarInt::read(buffer, version)?; // Size of following array

        for i in 0..16 {
            if (primary_bit_mask & (1 << i)) != 0 {
                if chunk.section(i).is_none() {
                    chunk.set_section_at(i as isize, Some(ChunkSection::default()));
                }
                if let Some(section) = chunk.section_mut(i + 1) {
                    let non_air_blocks = u16::read(buffer, version)?;
                    section
                        .blocks_mut()
                        .set_air_blocks(4096 - non_air_blocks as u32);
                    let bits_per_block = u8::read(buffer, version)?;
                    section
                        .blocks_mut()
                        .data_mut()
                        .set_bits_per_value(bits_per_block as usize);
                    if bits_per_block <= 4 || (5..=8).contains(&bits_per_block) {
                        if let Some(pallete) = section.blocks_mut().palette_mut() {
                            let pallete_length = VarInt::read(buffer, version)?.0 as usize;
                            for _ in 0..pallete_length {
                                let block_id = VarInt::read(buffer, version)?.0;
                                pallete.index_or_insert(BlockId::from_vanilla_id(block_id as u16));
                            }
                        }
                    }
                    let data_length = VarInt::read(buffer, version)?.0 as usize;
                    for i in 0..data_length {
                        section.blocks_mut().data_mut().as_u64_mut_vec()[i] =
                            u64::read(buffer, version)?;
                    }
                }
            }
        }

        VarInt::read(buffer, version)?; // Block entities length, redundant for feather right now

        Ok(Self {
            chunk: Arc::new(ChunkLock::new(chunk, true)),
            kind: chunk_data_kind,
        })
    }
}

fn deserialize_i64_37<'de, D>(deserializer: D) -> Result<[i64; 37], D::Error>
where
    D: Deserializer<'de>,
{
    struct MaxVisitor(PhantomData<fn() -> [i64; 37]>);

    impl<'de> Visitor<'de> for MaxVisitor {
        type Value = [i64; 37];

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence of 37 numbers")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<[i64; 37], S::Error>
        where
            S: SeqAccess<'de>,
        {
            let mut res = [0; 37];
            let mut index: usize = 0;

            while let Some(value) = seq.next_element()? {
                res[index] = value;
                index += 1;
            }

            if index != 37 {
                return Err(de::Error::custom(format!(
                    "expected 37 numbers, found {}",
                    index
                )));
            }

            Ok(res)
        }
    }

    // Create the visitor and ask the deserializer to drive it. The
    // deserializer will call visitor.visit_seq() if a seq is present in
    // the input data.
    let visitor = MaxVisitor(PhantomData);
    deserializer.deserialize_seq(visitor)
}
