use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::fmt;

extern crate nalgebra_glm as glm;

mod math_types;
#[macro_use]
mod positions;

pub use math_types::*;
pub use positions::{BlockPosition, ChunkPosition, Position};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Gamemode {
    Survival,
    Creative,
    Adventure,
    Spectator,
}

impl Gamemode {
    pub fn id(self) -> u8 {
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

impl fmt::Display for Gamemode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Gamemode::Survival => "Survival",
                Gamemode::Creative => "Creative",
                Gamemode::Adventure => "Adventure",
                Gamemode::Spectator => "Spectator",
            }
        )
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
    pub fn id(self) -> u8 {
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
    pub fn id(self) -> i32 {
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

#[derive(
    Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive,
)]
pub enum Direction {
    Down,
    Up,
    North,
    South,
    West,
    East,
}

impl Direction {
    pub fn id(self) -> i32 {
        match self {
            Direction::Down => 0,
            Direction::Up => 1,
            Direction::North => 2,
            Direction::South => 3,
            Direction::West => 4,
            Direction::East => 5,
        }
    }
}
