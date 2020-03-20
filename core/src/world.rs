use crate::Chunk;
use crate::{vec3, Block, Vec3d, Vec3i};
use hashbrown::HashMap;
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use rayon::iter::ParallelIterator;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
use std::sync::Arc;

#[macro_export]
macro_rules! position {
    ($x:expr, $y:expr, $z:expr, $pitch:expr, $yaw:expr, $on_ground:expr) => {
        $crate::Position {
            x: $x,
            y: $y,
            z: $z,
            pitch: $pitch,
            yaw: $yaw,
            on_ground: $on_ground,
        }
    };
    ($x:expr, $y:expr, $z:expr, $pitch: expr, $yaw: expr) => {
        position!($x, $y, $z, $pitch, $yaw, true)
    };
    ($x:expr, $y:expr, $z:expr) => {
        position!($x, $y, $z, 0.0, 0.0)
    };
    ($x:expr, $y:expr, $z:expr, $on_ground: expr) => {
        position!($x, $y, $z, 0.0, 0.0, $on_ground)
    };
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: f32,
    pub yaw: f32,
    pub on_ground: bool,
}

impl Position {
    pub fn distance_to(&self, other: Position) -> f64 {
        self.distance_squared_to(other).sqrt()
    }

    pub fn distance_squared_to(&self, other: Position) -> f64 {
        square(self.x - other.x) + square(self.y - other.y) + square(self.z - other.z)
    }

    /// Returns a unit vector representing
    /// the direction of this position's pitch
    /// and yaw.
    pub fn direction(&self) -> Vec3d {
        let rotation_x = f64::from(self.yaw.to_radians());
        let rotation_y = f64::from(self.pitch.to_radians());

        let y = -rotation_y.sin();

        let xz = rotation_y.cos();

        let x = -xz * rotation_x.sin();
        let z = xz * rotation_x.cos();

        vec3(x, y, z)
    }

    pub fn chunk(self) -> ChunkPosition {
        self.into()
    }

    pub fn block(self) -> BlockPosition {
        self.into()
    }

    pub fn as_vec(&self) -> Vec3d {
        (*self).into()
    }
}

impl Add<Vec3d> for Position {
    type Output = Position;

    fn add(mut self, rhs: Vec3d) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

impl Add<glm::DVec3> for Position {
    type Output = Position;

    fn add(mut self, rhs: glm::DVec3) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(mut self, rhs: Position) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.pitch += rhs.pitch;
        self.yaw += rhs.yaw;
        self
    }
}

impl Sub<Vec3d> for Position {
    type Output = Position;

    fn sub(mut self, rhs: Vec3d) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}

impl Sub<glm::DVec3> for Position {
    type Output = Position;

    fn sub(mut self, rhs: glm::DVec3) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(mut self, rhs: Position) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}

impl Into<Vec3d> for Position {
    fn into(self) -> Vec3d {
        vec3(self.x, self.y, self.z)
    }
}

impl Into<glm::DVec3> for Position {
    fn into(self) -> glm::DVec3 {
        glm::vec3(self.x, self.y, self.z)
    }
}

impl From<Vec3d> for Position {
    fn from(vec: Vec3d) -> Self {
        position!(vec.x, vec.y, vec.z)
    }
}

impl From<glm::DVec3> for Position {
    fn from(vec: glm::DVec3) -> Self {
        position!(vec.x, vec.y, vec.z)
    }
}

impl Into<ChunkPosition> for Position {
    fn into(self) -> ChunkPosition {
        ChunkPosition {
            x: self.x.floor() as i32 / 16,
            z: self.z.floor() as i32 / 16,
        }
    }
}

impl Into<BlockPosition> for Position {
    fn into(self) -> BlockPosition {
        BlockPosition {
            x: self.x.floor() as i32,
            y: self.y.floor() as i32,
            z: self.z.floor() as i32,
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "({:.2}, {:.2}, {:.2})", self.x, self.y, self.z,)
    }
}

fn square(x: f64) -> f64 {
    x * x
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Hash32, Default)]
pub struct ChunkPosition {
    pub x: i32,
    pub z: i32,
}

impl ChunkPosition {
    pub const fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    /// Computes the Manhattan distance from this chunk to another.
    pub fn manhattan_distance_to(self, other: ChunkPosition) -> i32 {
        (self.x - other.z).abs() + (self.z - other.z).abs()
    }
}

impl Display for ChunkPosition {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "({}, {})", self.x, self.z)
    }
}

impl Add<ChunkPosition> for ChunkPosition {
    type Output = ChunkPosition;

    fn add(self, rhs: ChunkPosition) -> Self::Output {
        ChunkPosition {
            x: self.x + rhs.x,
            z: self.z + rhs.z,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Hash32, Default)]
pub struct BlockPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPosition {
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// Returns the Manhattan distance from this position to another.
    pub fn manhattan_distance(self, other: BlockPosition) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    /// Converts this `BlockPosition` to a `Position`.
    pub fn position(self) -> Position {
        self.into()
    }
}

impl Add<BlockPosition> for BlockPosition {
    type Output = BlockPosition;

    fn add(mut self, rhs: BlockPosition) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

impl Add<Vec3i> for BlockPosition {
    type Output = Self;

    fn add(self, rhs: Vec3i) -> Self::Output {
        self + BlockPosition::from(rhs)
    }
}

impl Sub<BlockPosition> for BlockPosition {
    type Output = Self;

    fn sub(mut self, rhs: BlockPosition) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}

impl Sub<Vec3i> for BlockPosition {
    type Output = Self;

    fn sub(self, rhs: Vec3i) -> Self::Output {
        self - BlockPosition::from(rhs)
    }
}

impl Into<Vec3i> for BlockPosition {
    fn into(self) -> Vec3i {
        vec3(self.x, self.y, self.z)
    }
}

impl From<Vec3i> for BlockPosition {
    fn from(vec: Vec3i) -> Self {
        BlockPosition {
            x: vec.x,
            y: vec.y,
            z: vec.z,
        }
    }
}

impl Into<Position> for BlockPosition {
    fn into(self) -> Position {
        position!(self.x as f64, self.y as f64, self.z as f64)
    }
}

impl Into<ChunkPosition> for BlockPosition {
    fn into(self) -> ChunkPosition {
        ChunkPosition {
            x: self.x >> 4,
            z: self.z >> 4,
        }
    }
}

pub type ChunkMapInner = HashMap<ChunkPosition, Arc<RwLock<Chunk>>>;

/// The chunk map.
///
/// This struct stores all the chunks on the server,
/// so it allows access to blocks and lighting data.
///
/// Chunks are internally wrapped in `Arc<RwLock>`,
/// allowing multiple systems to access different parts
/// of the world in parallel. Mutable access to this
/// type is only required for inserting and removing
/// chunks.
pub struct ChunkMap(ChunkMapInner);

impl ChunkMap {
    /// Creates a new chunk map with no chunks.
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Retrieves a handle to the chunk at the given
    /// position, or `None` if it is not loaded.
    pub fn chunk_at(&self, pos: ChunkPosition) -> Option<RwLockReadGuard<Chunk>> {
        self.0.get(&pos).map(|lock| lock.read())
    }

    /// Retrieves a handle to the chunk at the given
    /// position, or `None` if it is not loaded.
    pub fn chunk_at_mut(&self, pos: ChunkPosition) -> Option<RwLockWriteGuard<Chunk>> {
        self.0.get(&pos).map(|lock| lock.write())
    }

    /// Returns an `Arc<RwLock<Chunk>>` at the given position.
    pub fn chunk_handle_at(&self, pos: ChunkPosition) -> Option<Arc<RwLock<Chunk>>> {
        self.0.get(&pos).map(Arc::clone)
    }

    /// Retrieves the block at the specified
    /// location. If the chunk in which the block
    /// exists is not laoded, `None` is returned.
    pub fn block_at(&self, pos: BlockPosition) -> Option<Block> {
        let (x, y, z) = chunk_relative_pos(pos);
        self.chunk_at(pos.into())
            .map(|chunk| chunk.block_at(x, y, z))
    }

    /// Sets the block at the given position.
    ///
    /// Returns `true` if the block was set, or `false`
    /// if its chunk was not loaded and thus no operation
    /// was performed.
    pub fn set_block_at(&self, pos: BlockPosition, block: Block) -> bool {
        let (x, y, z) = chunk_relative_pos(pos);

        self.chunk_at_mut(pos.into())
            .map(|mut chunk| chunk.set_block_at(x, y, z, block))
            .is_some()
    }

    /// Returns an iterator over chunks.
    pub fn iter_chunks(&self) -> impl IntoIterator<Item = &Arc<RwLock<Chunk>>> {
        self.0.values()
    }

    /// Returns a parallel iterator over chunks.
    pub fn par_iter_chunks(&self) -> impl ParallelIterator<Item = &Arc<RwLock<Chunk>>> {
        self.0.par_values()
    }

    /// Inserts a new chunk into the chunk map.
    pub fn insert(&mut self, chunk: Chunk) {
        self.0
            .insert(chunk.position(), Arc::new(RwLock::new(chunk)));
    }

    /// Removes the chunk at the given position, returning `true` if it existed.
    pub fn remove(&mut self, pos: ChunkPosition) -> bool {
        self.0.remove(&pos).is_some()
    }
}

impl Default for ChunkMap {
    fn default() -> Self {
        Self::new()
    }
}

pub fn chunk_relative_pos(block_pos: BlockPosition) -> (usize, usize, usize) {
    (
        block_pos.x as usize & 0xf,
        block_pos.y as usize,
        block_pos.z as usize & 0xf,
    )
}
