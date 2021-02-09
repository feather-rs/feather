//! Foundational types for Minecraft.

mod positions;
pub use positions::{
    vec3, Aabb, BlockPosition, ChunkPosition, Mat4f, Position, Vec2d, Vec2f, Vec2i, Vec3d, Vec3f,
    Vec3i, Vec4d, Vec4f, Vec4i,
};

mod consts;
pub use consts::*;
