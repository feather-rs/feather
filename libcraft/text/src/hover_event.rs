use super::Text;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "action", content = "value")]
pub enum HoverEvent<'a> {
    ShowText(Box<Text<'a>>),
    ShowItem(ItemStack),
    ShowEntity(LivingEntity),
}

pub type Uuid = ();
pub type ItemStack = ();
pub type LivingEntity = ();
