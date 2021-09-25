use crate::{vec3, Vec3d, Vec3i};
use hash32_derive::Hash32;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

#[macro_export]
macro_rules! position {
    ($x:expr, $y:expr, $z:expr, $pitch:expr, $yaw:expr, $on_ground:expr $(,)?) => {
        $crate::Position {
            x: $x,
            y: $y,
            z: $z,
            pitch: $pitch,
            yaw: $yaw,
            on_ground: $on_ground,
        }
    };
    ($x:expr, $y:expr, $z:expr, $pitch: expr, $yaw: expr $(,)?) => {
        position!($x, $y, $z, $pitch, $yaw, true)
    };
    ($x:expr, $y:expr, $z:expr $(,)?) => {
        position!($x, $y, $z, 0.0, 0.0)
    };
    ($x:expr, $y:expr, $z:expr, $on_ground: expr $(,)?) => {
        position!($x, $y, $z, 0.0, 0.0, $on_ground)
    };
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub pitch: f32,
    pub yaw: f32,
    pub on_ground: bool,
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

impl From<Position> for Vec3d {
    fn from(pos: Position) -> Self {
        vec3(pos.x, pos.y, pos.z)
    }
}

impl From<Position> for glm::DVec3 {
    fn from(pos: Position) -> Self {
        glm::vec3(pos.x, pos.y, pos.z)
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Hash32)]
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
            x: pos.x >> 4,
            z: pos.z >> 4,
        }
    }
}
