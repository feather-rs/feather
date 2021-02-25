//! Foundational types and constants for Minecraft.

pub mod block;
mod consts;
mod enchantment;
mod gamemode;
mod gamerules;
mod particle;
mod positions;

pub use consts::*;
pub use enchantment::{Enchantment, EnchantmentKind};
pub use gamemode::Gamemode;
pub use gamerules::GameRules;
pub use particle::Particle;
pub use positions::{
    vec3, Aabb, BlockPosition, ChunkPosition, Mat4f, Position, Vec2d, Vec2f, Vec2i, Vec3d, Vec3f,
    Vec3i, Vec4d, Vec4f, Vec4i,
};
