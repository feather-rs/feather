#[derive(Clone, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: f32,
    pub yaw: f32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChunkPosition {
    pub x: u32,
    pub z: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BlockPosition {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

impl BlockPosition {
    pub fn new(x: u32, y: u32, z: u32) -> Self {
        Self { x, y, z }
    }
}
