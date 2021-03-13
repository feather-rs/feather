use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "String", into = "&'static str")]
pub enum Dimension {
    Overworld,
    TheNether,
    TheEnd,
}

impl Dimension {
    pub fn id(&self) -> i32 {
        match self {
            Self::Overworld => 0,
            Self::TheNether => -1,
            Self::TheEnd => 1,
        }
    }

    pub fn from_id(id: i32) -> Option<Self> {
        match id {
            0 => Some(Self::Overworld),
            -1 => Some(Self::TheNether),
            1 => Some(Self::TheEnd),
            _ => None,
        }
    }

    pub fn namespaced_id(&self) -> &'static str {
        match self {
            Self::Overworld => "minecraft:overworld",
            Self::TheNether => "minecraft:the_nether",
            Self::TheEnd => "minecraft:the_end",
        }
    }

    pub fn from_namespaced_id(id: &str) -> Option<Self> {
        match id {
            "minecraft:overworld" => Some(Self::Overworld),
            "minecraft:the_nether" => Some(Self::TheNether),
            "minecraft:the_end" => Some(Self::TheEnd),
            _ => None,
        }
    }
}

impl TryFrom<String> for Dimension {
    type Error = &'static str;

    fn try_from(namespaced_value: String) -> Result<Self, Self::Error> {
        if let Some(val) = Self::from_namespaced_id(namespaced_value.as_str()) {
            Ok(val)
        } else {
            Err("Unknown dimension namespaced_id.")
        }
    }
}

impl From<Dimension> for &'static str {
    fn from(value: Dimension) -> Self {
        value.namespaced_id()
    }
}

impl From<Dimension> for i32 {
    fn from(value: Dimension) -> Self {
        value.id()
    }
}
