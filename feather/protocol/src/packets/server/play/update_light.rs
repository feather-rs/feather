use std::{fmt::Debug, sync::Arc};

use base::{chunk::PackedArray, Chunk};
use parking_lot::RwLock;

use crate::{io::VarInt, ProtocolVersion, Readable, Writeable};

#[derive(Clone)]
pub struct UpdateLight {
    pub chunk: Arc<RwLock<Chunk>>,
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
        _buffer: &mut std::io::Cursor<&[u8]>,
        _version: crate::ProtocolVersion,
    ) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        todo!()
    }
}
