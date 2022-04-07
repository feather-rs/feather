use std::borrow::Cow;
use std::fmt::{self, Debug};

use either::Either;
use serde::{Deserialize, Serialize};

use libcraft::ChunkPosition;
use libcraft::{Chunk, ChunkSection};
use quill::ChunkHandle;

use crate::packets::server::play::update_light::LightData;
use crate::{Nbt, ProtocolVersion, Readable, VarInt, Writeable};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct Heightmaps<'a> {
    motion_blocking: Cow<'a, [i64]>,
    world_surface: Cow<'a, [i64]>,
}

/// Packet to load a chunk on the client.
#[derive(Clone)]
pub struct ChunkData {
    /// The chunk to send. Serialization panics if Either::Right. Right is only
    /// used for deserialization as we don't know the world height and
    /// can't infer section count
    pub chunk: Either<ChunkHandle, ChunkPosition>,
}

impl Debug for ChunkData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug_struct = f.debug_struct("ChunkData");
        debug_struct.field(
            "position",
            &self
                .chunk
                .clone()
                .map_left(|chunk| chunk.read().position())
                .into_inner(),
        );
        debug_struct.finish()
    }
}

impl Writeable for ChunkData {
    fn write(&self, buffer: &mut Vec<u8>, version: ProtocolVersion) -> anyhow::Result<()> {
        let chunk = self.chunk.as_ref().unwrap_left().read();

        chunk.position().x.write(buffer, version)?;
        chunk.position().z.write(buffer, version)?;

        let heightmaps = build_heightmaps(&chunk);
        Nbt(heightmaps).write(buffer, version)?;

        // Sections
        let mut data = Vec::new();
        for section in chunk
            .sections()
            .iter()
            .skip(1)
            .take(chunk.sections().len() - 2)
        {
            encode_section(section, &mut data, version)?;
        }
        VarInt(data.len() as i32).write(buffer, version)?;
        buffer.extend_from_slice(&data);

        VarInt(0).write(buffer, version)?; // block entities are not implemented yet

        LightData::from_full_chunk(&chunk).write(buffer, version)?;

        Ok(())
    }
}

impl Readable for ChunkData {
    fn read(buffer: &mut std::io::Cursor<&[u8]>, version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        Ok(ChunkData {
            chunk: Either::Right(ChunkPosition::new(
                i32::read(buffer, version)?,
                i32::read(buffer, version)?,
            )),
        })
    }
}

fn build_heightmaps(chunk: &Chunk) -> Heightmaps {
    let chunk_motion_blocking = chunk.heightmaps().motion_blocking.as_u64_slice();
    let chunk_world_surface = chunk.heightmaps().motion_blocking.as_u64_slice();
    let motion_blocking = Cow::Borrowed(bytemuck::cast_slice(chunk_motion_blocking));
    let world_surface = Cow::Borrowed(bytemuck::cast_slice(chunk_world_surface));
    Heightmaps {
        motion_blocking,
        world_surface,
    }
}

fn encode_section(
    section: &ChunkSection,
    buffer: &mut Vec<u8>,
    version: ProtocolVersion,
) -> anyhow::Result<()> {
    (section.non_air_blocks() as u16).write(buffer, version)?;
    section.blocks().write(buffer, version)?;
    section.biomes().write(buffer, version)?;
    Ok(())
}
