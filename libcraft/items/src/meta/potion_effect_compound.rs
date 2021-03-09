use serde::{Deserialize, Serialize};

// TODO Generate potion effects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PotionEffectCompound {}
