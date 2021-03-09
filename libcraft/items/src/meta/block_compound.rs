use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BlockCompound {
    can_place_on: Option<Vec<String>>,
    // TODO BlockEntityTag.
    // TODO BlockStateTag. Connect with block_data?
}
