use crate::io::BitMask;
use crate::{ProtocolVersion, Readable, VarInt, Writeable};
use libcraft::chunk::{LightStore, PackedArray, SECTION_VOLUME};
use libcraft::Chunk;
use libcraft::ChunkPosition;
use std::collections::VecDeque;
use std::fmt::Debug;
use std::io::{Cursor, Read};
use std::mem::ManuallyDrop;

#[derive(Clone)]
pub struct UpdateLight {
    pub position: ChunkPosition,
    pub light: LightData,
}

impl Debug for UpdateLight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UpdateLight")
            .field("position", &self.position)
            .finish()
    }
}

impl Writeable for UpdateLight {
    fn write(&self, buffer: &mut Vec<u8>, version: ProtocolVersion) -> anyhow::Result<()> {
        VarInt(self.position.x).write(buffer, version)?;
        VarInt(self.position.z).write(buffer, version)?;

        self.light.write(buffer, version)
    }
}

impl Readable for UpdateLight {
    fn read(buffer: &mut std::io::Cursor<&[u8]>, version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let x = VarInt::read(buffer, version)?.0;
        let z = VarInt::read(buffer, version)?.0;

        let light = LightData::read(buffer, version)?;
        Ok(UpdateLight {
            position: ChunkPosition::new(x, z),
            light,
        })
    }
}

#[derive(Clone)]
pub struct LightData {
    pub trust_edges: bool,
    pub light: Vec<Option<LightStore>>,
}

impl LightData {
    pub fn from_full_chunk(chunk: &Chunk) -> LightData {
        LightData {
            trust_edges: true,
            light: chunk
                .sections()
                .iter()
                .map(|s| Some(s.light().clone()))
                .collect(),
        }
    }
    pub fn from_chunk(chunk: &Chunk, sections: Vec<usize>) -> LightData {
        LightData {
            trust_edges: true,
            light: chunk
                .sections()
                .iter()
                .map(|s| s.light().clone())
                .enumerate()
                .map(|(i, l)| if sections.contains(&i) { Some(l) } else { None })
                .collect(),
        }
    }
}

impl Writeable for LightData {
    fn write(&self, buffer: &mut Vec<u8>, version: ProtocolVersion) -> anyhow::Result<()> {
        self.trust_edges.write(buffer, version)?; // trust edges?

        let mut sky_mask = BitMask::default();
        let mut empty_sky_mask = BitMask::default();
        let mut block_mask = BitMask::default();
        let mut empty_block_mask = BitMask::default();

        let mut sky_set = 0;
        let mut block_set = 0;

        for (y, light) in self.light.iter().enumerate() {
            if let Some(light) = light {
                if !light.sky_light().map(|l| l.is_empty()).unwrap_or(true) {
                    sky_mask.set(y, true);
                    sky_set += 1;
                } else {
                    empty_sky_mask.set(y, true);
                }

                if !light.block_light().map(|l| l.is_empty()).unwrap_or(true) {
                    block_mask.set(y, true);
                    block_set += 1;
                } else {
                    empty_block_mask.set(y, true);
                }
            }
        }

        sky_mask.write(buffer, version)?;
        block_mask.write(buffer, version)?;

        empty_sky_mask.write(buffer, version)?;
        empty_block_mask.write(buffer, version)?;

        VarInt(sky_set).write(buffer, version)?;
        for (i, light) in self.light.iter().enumerate() {
            if sky_mask.is_set(i) {
                encode_light(
                    light.as_ref().unwrap().sky_light().unwrap(),
                    buffer,
                    version,
                )?;
            }
        }

        VarInt(block_set).write(buffer, version)?;
        for (i, light) in self.light.iter().enumerate() {
            if block_mask.is_set(i) {
                encode_light(
                    light.as_ref().unwrap().block_light().unwrap(),
                    buffer,
                    version,
                )?;
            }
        }

        Ok(())
    }
}

impl Readable for LightData {
    fn read(buffer: &mut Cursor<&[u8]>, version: ProtocolVersion) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let trust_edges = bool::read(buffer, version)?;

        let sky_mask = BitMask::read(buffer, version)?;
        let block_mask = BitMask::read(buffer, version)?;

        let empty_sky_mask = BitMask::read(buffer, version)?;
        let empty_block_mask = BitMask::read(buffer, version)?;

        let sky_set = VarInt::read(buffer, version)?.0 as usize;
        let mut sky_light_sections = VecDeque::new();
        for _ in 0..sky_set {
            sky_light_sections.push_back(decode_light(buffer, version)?);
        }
        let mut sky_light = Vec::new();
        // We don't have the world height, try to infer the max section count
        for i in 0..sky_mask.max_len() {
            if sky_mask.is_set(i) {
                sky_light.push(sky_light_sections.pop_front())
            } else if empty_sky_mask.is_set(i) {
                sky_light.push(Some(PackedArray::from_u64_vec(
                    vec![0; SECTION_VOLUME * 4 / u64::BITS as usize],
                    4,
                )))
            } else {
                sky_light.push(None)
            }
        }

        let block_set = VarInt::read(buffer, version)?.0 as usize;
        let mut block_light_sections = VecDeque::new();
        for _ in 0..block_set {
            block_light_sections.push_back(decode_light(buffer, version)?);
        }
        let mut block_light = Vec::new();
        // We don't have the world height, try to infer the max section count
        for i in 0..block_mask.max_len() {
            if block_mask.is_set(i) {
                block_light.push(sky_light_sections.pop_front())
            } else if empty_block_mask.is_set(i) {
                block_light.push(Some(PackedArray::from_u64_vec(
                    vec![0; SECTION_VOLUME * 4 / u64::BITS as usize],
                    4,
                )))
            } else {
                block_light.push(None)
            }
        }

        Ok(LightData {
            trust_edges,
            light: sky_light
                .into_iter()
                .zip(block_light.into_iter())
                .map(|(sky, block)| LightStore::from_packed_arrays(block, sky))
                .collect(),
        })
    }
}

fn encode_light(
    light: &PackedArray,
    buffer: &mut Vec<u8>,
    version: ProtocolVersion,
) -> anyhow::Result<()> {
    VarInt(light.len() as i32 / 2).write(buffer, version)?;
    let light_data: &[u8] = bytemuck::cast_slice(light.as_u64_slice());
    debug_assert_eq!(light_data.len(), 2048);
    buffer.extend_from_slice(light_data);
    Ok(())
}

fn decode_light(
    buffer: &mut Cursor<&[u8]>,
    version: ProtocolVersion,
) -> anyhow::Result<PackedArray> {
    let should_be_2048 = VarInt::read(buffer, version)?.0 as usize;
    debug_assert_eq!(should_be_2048, 2048);
    let mut light_data = vec![0; should_be_2048];
    buffer.read_exact(&mut light_data)?;
    let mut light_data = ManuallyDrop::new(light_data);
    // SAFETY: light_data is ManuallyDrop so there's no double-free
    Ok(PackedArray::from_u64_vec(
        unsafe {
            Vec::from_raw_parts(
                light_data.as_mut_ptr() as *mut u64,
                light_data.len() * u8::BITS as usize / u64::BITS as usize,
                light_data.capacity() * u8::BITS as usize / u64::BITS as usize,
            )
        },
        4,
    ))
}
