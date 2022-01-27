use std::{fmt::Debug, sync::Arc};

use base::{chunk::PackedArray, Chunk, ChunkHandle, ChunkLock, ChunkPosition, ChunkSection};

use crate::{io::VarInt, ProtocolVersion, Readable, Writeable};

#[derive(Clone)]
pub struct UpdateLight {
    pub chunk: ChunkHandle,
}

impl Debug for UpdateLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut debug_struct = f.debug_struct("UpdateLight");
        debug_struct.field("position", &self.chunk.read().position());
        debug_struct.finish()
    }
}

impl Writeable for UpdateLight {
    fn write(&self, buffer: &mut Vec<u8>, version: crate::ProtocolVersion) -> anyhow::Result<()> {
        let chunk = self.chunk.read();
        VarInt(chunk.position().x).write(buffer, version)?;
        VarInt(chunk.position().z).write(buffer, version)?;

        true.write(buffer, version)?; // trust edges?

        let mut mask = 0;
        for (y, section) in chunk.sections().iter().enumerate() {
            if section.is_some() {
                mask |= 1 << y;
            }
        }

        VarInt(mask).write(buffer, version)?; // sky light mask
        VarInt(mask).write(buffer, version)?; // block light mask

        VarInt(!mask).write(buffer, version)?; // empty sky light mask
        VarInt(!mask).write(buffer, version)?; // empty block light mask

        for section in chunk.sections().iter().flatten() {
            encode_light(section.light().sky_light(), buffer, version);
        }

        for section in chunk.sections().iter().flatten() {
            encode_light(section.light().block_light(), buffer, version);
        }

        Ok(())
    }
}

fn encode_light(light: &PackedArray, buffer: &mut Vec<u8>, version: ProtocolVersion) {
    VarInt(2048).write(buffer, version).unwrap();
    let light_data: &[u8] = bytemuck::cast_slice(light.as_u64_slice());
    assert_eq!(light_data.len(), 2048);
    buffer.extend_from_slice(light_data);
}

impl Readable for UpdateLight {
    fn read(
        buffer: &mut std::io::Cursor<&[u8]>,
        version: crate::ProtocolVersion,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let mut chunk = Chunk::new(ChunkPosition {
            x: VarInt::read(buffer, version)?.0,
            z: VarInt::read(buffer, version)?.0,
        });

        let _trust_edges = bool::read(buffer, version)?;

        let sky_light_mask = VarInt::read(buffer, version)?.0;
        let block_light_mask = VarInt::read(buffer, version)?.0;
        let _empty_sky_light_mask = VarInt::read(buffer, version)?;
        let _empty_block_light_mask = VarInt::read(buffer, version)?;

        for i in 0..18 {
            if (sky_light_mask & (1 << i)) != 0 {
                let probably_2048 = VarInt::read(buffer, version)?.0 as usize;
                assert_eq!(probably_2048, 2048);
                let mut bytes: Vec<u8> = Vec::new();
                for _ in 0..probably_2048 {
                    bytes.push(u8::read(buffer, version)?);
                }
                let mut bytes = bytes.iter();
                if chunk.section(i).is_none() {
                    chunk.set_section_at(i as isize, Some(ChunkSection::default()));
                }
                if let Some(section) = chunk.section_mut(i + 1) {
                    for x in 0..16 {
                        for y in 0..16 {
                            for z in 0..16 {
                                section.set_sky_light_at(x, y, z, *bytes.next().unwrap_or(&15));
                            }
                        }
                    }
                }
            }
        }

        for i in 0..18 {
            if (block_light_mask & (1 << i)) != 0 {
                let probably_2048 = VarInt::read(buffer, version)?.0 as usize;
                assert_eq!(probably_2048, 2048);
                let mut bytes: Vec<u8> = Vec::new();
                for _ in 0..probably_2048 {
                    bytes.push(u8::read(buffer, version)?);
                }
                let mut bytes = bytes.iter();
                if let Some(section) = chunk.section_mut(i) {
                    for x in 0..16 {
                        for y in 0..16 {
                            for z in 0..16 {
                                section.set_block_light_at(x, y, z, *bytes.next().unwrap_or(&15));
                            }
                        }
                    }
                }
            }
        }

        Ok(Self {
            chunk: Arc::new(ChunkLock::new(chunk, true)),
        })
    }
}
