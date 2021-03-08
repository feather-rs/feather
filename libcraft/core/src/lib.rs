//! Foundational types and constants for Minecraft.

mod biome;
pub mod block;
mod consts;
mod dimension;
mod entity;
mod gamemode;
mod gamerules;
mod interaction;
mod player;
mod positions;

pub use biome::Biome;
pub use consts::*;
pub use dimension::Dimension;
pub use entity::EntityKind;
pub use gamemode::Gamemode;
pub use gamerules::GameRules;
pub use interaction::InteractionType;
pub use player::Hand;
pub use positions::{
    vec3, Aabb, BlockFace, BlockPosition, ChunkPosition, Mat4f, Position, Vec2d, Vec2f, Vec2i,
    Vec3d, Vec3f, Vec3i, Vec4d, Vec4f, Vec4i,
};
