//! Position and math-related types.

use bytemuck::{Pod, Zeroable};
use fmt::Formatter;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    ops::{Add, Sub},
};
use vek::{Mat4, Vec2, Vec3, Vec4};

use crate::CHUNK_WIDTH;
pub type Vec2i = Vec2<i32>;
pub type Vec3i = Vec3<i32>;
pub type Vec4i = Vec4<i32>;

pub type Vec2f = Vec2<f32>;
pub type Vec3f = Vec3<f32>;
pub type Vec4f = Vec4<f32>;

/// Two-component double-precision floating point vector.
pub type Vec2d = Vec2<f64>;
/// Three-component double-precision floating point vector.
pub type Vec3d = Vec3<f64>;
/// Four-compounent double-precision floating point vector.
pub type Vec4d = Vec4<f64>;

pub type Aabb = vek::Aabb<f64>;

pub type Mat4f = Mat4<f32>;

/// Creates a `Vec3<T>`.
pub fn vec3<T>(x: T, y: T, z: T) -> Vec3<T> {
    Vec3::new(x, y, z)
}

/// Creates a `Position`.
#[macro_export]
macro_rules! position {
    ($x:expr, $y:expr, $z:expr, $pitch:expr, $yaw:expr $(,)?) => {
        $crate::Position {
            x: $x,
            y: $y,
            z: $z,
            pitch: $pitch,
            yaw: $yaw,
        }
    };
    ($x:expr, $y:expr, $z:expr $(,)?) => {
        position!($x, $y, $z, 0.0, 0.0)
    };
    ($x:expr, $y:expr, $z:expr, $on_ground: expr $(,)?) => {
        position!($x, $y, $z, 0.0, 0.0, $on_ground)
    };
}

/// The position of an entity.
///
/// This includes a world-space transform,
/// a 2D Euler angle rotation, and an on_ground field used for physics.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize, Pod, Zeroable)]
#[repr(C)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for Position {
    fn default() -> Self {
        position!(0.0, 64.0, 0.0)
    }
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

    pub fn vec(&self) -> Vec3d {
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

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(mut self, rhs: Position) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}

impl From<Position> for Vec3d {
    fn from(pos: Position) -> Self {
        vec3(pos.x, pos.y, pos.z)
    }
}

impl From<Vec3d> for Position {
    fn from(vec: Vec3d) -> Self {
        position!(vec.x, vec.y, vec.z)
    }
}

impl From<Position> for ChunkPosition {
    fn from(pos: Position) -> Self {
        Self {
            x: (pos.x / 16.0).floor() as i32,
            z: (pos.z / 16.0).floor() as i32,
        }
    }
}

impl From<Position> for BlockPosition {
    fn from(pos: Position) -> Self {
        Self {
            x: pos.x.floor() as i32,
            y: pos.y.floor() as i32,
            z: pos.z.floor() as i32,
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

/// Position of a chunk.
///
/// Units are in chunks. 1 chunk equals 16 blocks.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
    Serialize,
    Deserialize,
    Zeroable,
    Pod,
)]
#[repr(C)]
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
        (self.x - other.x).abs() + (self.z - other.z).abs()
    }

    /// Computes the squared Euclidean distance (in chunks) between `self` and `other`.
    pub fn distance_squared_to(self, other: ChunkPosition) -> i32 {
        (self.x - other.x).pow(2) + (self.z - other.z).pow(2)
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

/// Position of a block.
///
/// Y coordinate should be within
/// the interval [0, 256).
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
    Serialize,
    Deserialize,
    Zeroable,
    Pod,
)]
#[repr(C)]
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

    /// Converts into a `ChunkPosition`.
    pub fn chunk(self) -> ChunkPosition {
        self.into()
    }

    pub fn up(self) -> BlockPosition {
        Self {
            x: self.x,
            y: self.y + 1,
            z: self.z,
        }
    }

    pub fn down(self) -> BlockPosition {
        Self {
            x: self.x,
            y: self.y - 1,
            z: self.z,
        }
    }

    pub fn north(self) -> BlockPosition {
        Self {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        }
    }

    pub fn south(self) -> BlockPosition {
        Self {
            x: self.x,
            y: self.y,
            z: self.z + 1,
        }
    }

    pub fn east(self) -> BlockPosition {
        Self {
            x: self.x + 1,
            y: self.y,
            z: self.z,
        }
    }

    pub fn west(self) -> BlockPosition {
        Self {
            x: self.x - 1,
            y: self.y,
            z: self.z,
        }
    }

    /// Returns `true` if the [`BlockPosition`] is valid.
    ///
    /// Minecraft defines a valid block position with the following limits:
    /// - X (-33554432 to 33554431)
    /// - Y (-2048 to 2047)
    /// - Z (-33554432 to 33554431)
    pub fn valid(self) -> bool {
        (-33554432 <= self.x && self.x <= 33554431)
            && (-2048 <= self.y && self.y <= 2047)
            && (-33554432 <= self.z && self.z <= 33554431)
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

impl From<BlockPosition> for Vec3i {
    fn from(pos: BlockPosition) -> Self {
        vec3(pos.x, pos.y, pos.z)
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

impl From<BlockPosition> for Position {
    fn from(pos: BlockPosition) -> Self {
        position!(pos.x as f64 + 0.5, pos.y as f64 + 0.5, pos.z as f64 + 0.5)
    }
}

impl From<BlockPosition> for ChunkPosition {
    fn from(pos: BlockPosition) -> Self {
        Self {
            x: pos.x.div_euclid(CHUNK_WIDTH as i32),
            z: pos.z.div_euclid(CHUNK_WIDTH as i32),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum BlockFace {
    Bottom,
    Top,
    North,
    South,
    West,
    East,
}
