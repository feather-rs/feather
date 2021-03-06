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
    pub fn dim_id(&self) -> i32 {
        match self {
            Self::Overworld => 0,
            Self::TheNether => -1,
            Self::TheEnd => 1,
        }
    }

    pub fn from_dim_id(id: i32) -> Option<Self> {
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

    pub fn from_namespaced_id(id: &String) -> Option<Self> {
        match id.as_str() {
            "minecraft:overworld" => Some(Self::Overworld),
            "minecraft:the_nether" => Some(Self::TheNether),
            "minecraft:the_end" => Some(Self::TheEnd),
            _ => None,
        }
    }
}

impl TryFrom<String> for Dimension {
    type Error = String;

    fn try_from(namespaced_value: String) -> Result<Self, Self::Error> {
        if let Some(val) = Self::from_namespaced_id(&namespaced_value) {
            Ok(val)
        } else {
            Err(format!(
                "There is no dimension with namespaced_id {}",
                namespaced_value
            )
            .to_string())
        }
    }
}

impl Into<&'static str> for Dimension {
    fn into(self) -> &'static str {
        self.namespaced_id()
    }
}
