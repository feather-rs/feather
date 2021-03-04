use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};

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
