use libcraft_core::BlockPosition;

use serde::{Deserialize, Serialize};

use crate::utils::{bool_from_u8, bool_to_u8};

/// Contains metadata related to compasses:
/// * If the compass is tracking a lodestone.
/// * The dimension of the lodestone if applicable.
/// * The position of the lodestone if applicable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CompassCompound {
    /// True if the compass is tracking a lodestone.
    #[serde(serialize_with = "bool_to_u8", deserialize_with = "bool_from_u8")]
    lodestone_tracked: bool,

    // TODO Change from Option<String> to Option<Dimension> where Dimension is a custom enum.
    /// The dimension where the lodestone is found.
    /// Only Some if the compass is tracking a lodestone.
    lodestone_dimension: Option<String>,

    /// The position of the lodestone.
    lodestone_position: Option<BlockPosition>,
}
