use serde::{Deserialize, Serialize};

// TODO Implement effects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SuspiciousStewCompound {}
