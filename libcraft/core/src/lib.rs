//! Foundational types and constants for Minecraft.

pub mod block;
pub mod consts;
mod entity;
mod gamemode;
mod gamerules;
mod interaction;
mod player;
mod positions;

use std::fmt::Formatter;

pub use block::BlockFace;
pub use consts::*;
pub use entity::EntityKind;
pub use gamemode::Gamemode;
pub use gamerules::GameRules;
pub use interaction::InteractionType;
pub use player::Hand;
pub use positions::{
    vec3, Aabb, BlockPosition, ChunkPosition, Mat4f, Position, RegionPosition, ValidBlockPosition,
    Vec2d, Vec2f, Vec2i, Vec3d, Vec3f, Vec3i, Vec4d, Vec4f, Vec4i, REGION_SIZE,
};

use num_derive::{FromPrimitive, ToPrimitive};
use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, FromPrimitive, ToPrimitive)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// A profile property, which stores metadata
/// for some player's account. This is usually
/// used to store skin data.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProfileProperty {
    pub name: String,
    pub value: String,
    pub signature: String,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, derive_more::Deref)]
pub struct WorldHeight(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, Debug, derive_more::Deref)]
pub struct Sections(pub usize);

impl From<Sections> for WorldHeight {
    fn from(sections: Sections) -> Self {
        WorldHeight(sections.0 * CHUNK_SECTION_HEIGHT)
    }
}

impl From<WorldHeight> for Sections {
    fn from(sections: WorldHeight) -> Self {
        Sections(sections.0 / CHUNK_SECTION_HEIGHT)
    }
}

pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct BoolI8Visitor;

    impl Visitor<'_> for BoolI8Visitor {
        type Value = bool;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("a bool")
        }

        fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v)
        }

        fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v != 0)
        }

        fn visit_u8<E>(self, v: u8) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(v != 0)
        }
    }

    deserializer.deserialize_any(BoolI8Visitor)
}
