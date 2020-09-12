use std::sync::Arc;

use base::{Chunk, CHUNK_WIDTH};
use parking_lot::RwLock;
use serde::Serialize;

use crate::{io::VarInt, Writeable};

#[derive(Serialize)]
struct Heightmaps {
    #[serde(rename = "MOTION_BLOCKING")]
    #[serde(serialize_with = "nbt::i64_array")]
    motion_blocking: [i64; 36],
}

/// Packet to load a chunk on the client.
pub struct ChunkData {
    /// The chunk to send.
    pub chunk: Arc<RwLock<Chunk>>,
}

impl Writeable for ChunkData {
    fn write(&self, buffer: &mut Vec<u8>, version: crate::ProtocolVersion) {
        let chunk = self.chunk.read();

        chunk.position().x.write(buffer, version);
        chunk.position().z.write(buffer, version);

        // full chunk (always true for us)
        true.write(buffer, version);

        // Compute primary bit mask
        let mut bitmask = 0;
        for (y, section) in chunk.sections().iter().enumerate() {
            if section.is_some() {
                bitmask |= 1 << y as i32;
            }
        }
        VarInt(bitmask).write(buffer, version);

        let heightmaps = build_heightmaps(&chunk);
    }
}

fn build_heightmaps(chunk: &Chunk) -> Heightmaps {
    // TODO: this could incur substantial overhead. Might be a spot
    // to check for in profiling.
    let mut motion_blocking = [0i64; 36];

    for x in 0..CHUNK_WIDTH {
        for z in 0..CHUNK_WIDTH {
            let heightmap = chunk.heightmap(x, z);
            
            let index = 
        }
    }

    Heightmaps { motion_blocking }
}
