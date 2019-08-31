#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate feather_codegen;
#[macro_use]
extern crate num_derive;
#[macro_use]
extern crate smallvec;

extern crate nalgebra_glm as glm;

pub mod bytebuf;
pub mod entitymeta;
pub mod inventory;
pub mod network;
pub mod prelude;
mod save;
#[macro_use]
pub mod world;

pub use entitymeta::EntityMetadata;
pub use feather_items as item;
pub use inventory::{ItemStack, Slot};
pub use item::{Item, ItemExt};
pub use network::packet::{implementation as packet, Packet, PacketType};
pub use save::{level, player_data, region};
pub use world::{
    block::{self, Block, BlockExt},
    chunk::{Chunk, ChunkSection},
    BlockPosition, ChunkPosition, Position,
};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl Gamemode {
    pub fn get_id(self) -> u8 {
        match self {
            Gamemode::Survival => 0,
            Gamemode::Creative => 1,
            Gamemode::Adventure => 2,
            Gamemode::Spectator => 3,
        }
    }

    pub fn from_id(id: u8) -> Self {
        match id {
            0 => Gamemode::Survival,
            1 => Gamemode::Creative,
            2 => Gamemode::Adventure,
            3 => Gamemode::Spectator,
            _ => Gamemode::Survival,
        }
    }

    pub fn from_string(name: &str) -> Self {
        match name.to_lowercase().as_str() {
            "survival" => Gamemode::Survival,
            "creative" => Gamemode::Creative,
            "adventure" => Gamemode::Adventure,
            "spectator" => Gamemode::Spectator,
            _ => Gamemode::Survival,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Difficulty {
    Peaceful,
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn get_id(self) -> u8 {
        match self {
            Difficulty::Peaceful => 0,
            Difficulty::Easy => 1,
            Difficulty::Medium => 2,
            Difficulty::Hard => 3,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dimension {
    Nether,
    Overwold,
    End,
}

impl Dimension {
    pub fn get_id(self) -> i32 {
        match self {
            Dimension::Nether => -1,
            Dimension::Overwold => 0,
            Dimension::End => 1,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PvpStyle {
    Classic,
    New,
}

#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive,
)]
pub enum ClientboundAnimation {
    SwingMainArm,
    TakeDamage,
    LeaveBed,
    SwingOffhand,
    CriticalEffect,
    MagicCriticalEffect,
}

impl Default for ClientboundAnimation {
    fn default() -> Self {
        ClientboundAnimation::SwingMainArm
    }
}
#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive,
)]
pub enum Hand {
    Main,
    Off,
}

impl Default for Hand {
    fn default() -> Self {
        Hand::Main
    }
}
