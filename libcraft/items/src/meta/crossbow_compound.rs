use crate::ItemStack;

use serde::{Deserialize, Serialize};

use crate::utils::{bool_from_u8, bool_to_u8};

/// Contains data related to crossbows:
/// * If the crossbow is charged or not.
/// * A list of charged items on the crossbow, usually one entry,
///   but it may have multiple items charged if enchanted with multishot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CrossbowCompound {
    /// True if the crossbow is charged.
    #[serde(serialize_with = "bool_to_u8", deserialize_with = "bool_from_u8")]
    charged: bool,

    /// A list of the charged projectiles if charged.
    charged_projectiles: Option<Vec<ItemStack>>,
}
