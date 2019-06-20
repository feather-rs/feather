pub mod block;
#[allow(clippy::cast_lossless)]
pub mod chunk;

#[derive(Clone, Debug, new)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, new)]
pub struct ChunkPosition {
    pub x: u32,
    pub z: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default, new)]
pub struct BlockPosition {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

pub struct World {}
