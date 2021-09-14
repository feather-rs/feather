use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// A gamemode.
#[derive(
    Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, FromPrimitive, ToPrimitive,
)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum Gamemode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

impl Gamemode {
    /// Gets a gamemode from its ID.
    pub fn from_id(id: u8) -> Option<Self> {
        Some(match id {
            0 => Gamemode::Survival,
            1 => Gamemode::Creative,
            2 => Gamemode::Adventure,
            3 => Gamemode::Spectator,
            _ => return None,
        })
    }
}

impl FromStr for Gamemode {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        match input.to_ascii_lowercase().as_str() {
            "survival" => Ok(Gamemode::Survival),
            "creative" => Ok(Gamemode::Creative),
            "adventure" => Ok(Gamemode::Adventure),
            "spectator" => Ok(Gamemode::Spectator),
            _ => Err(()),
        }
    }
}

impl ToString for Gamemode {
    fn to_string(&self) -> String {
        match self {
            Gamemode::Survival => "survival",
            Gamemode::Creative => "creative",
            Gamemode::Adventure => "adventure",
            Gamemode::Spectator => "spectator",
        }
        .to_owned()
    }
}
