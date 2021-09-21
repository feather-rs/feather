use generated::{Item, ItemStack};
use serde::{Deserialize, Serialize};

pub fn default_count() -> u32 {
    1
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct SerdeItem(Item);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Serialize, Deserialize)]
pub struct SerdeItemStack {
    pub item: SerdeItem,
    #[serde(default = "default_count")]
    pub count: u32,
    pub damage: Option<u32>,
}
impl From<SerdeItemStack> for ItemStack {
    fn from(s: SerdeItemStack) -> Self {
        Self {
            item: s.item.into(),
            count: s.count,
            damage: s.damage,
        }
    }
}
impl From<ItemStack> for SerdeItemStack {
    fn from(s: ItemStack) -> Self {
        Self {
            item: s.item.into(),
            count: s.count,
            damage: s.damage,
        }
    }
}
impl From<SerdeItem> for Item {
    fn from(s: SerdeItem) -> Self {
        s.0
    }
}
impl From<Item> for SerdeItem {
    fn from(s: Item) -> Self {
        Self(s)
    }
}

impl Serialize for SerdeItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = self.0.name().to_owned();
        s.insert_str(0, "minecraft:");
        s.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for SerdeItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        Ok(Self(
            Item::from_name(match string.strip_prefix("minecraft:") {
                Some(s) => s,
                None => &string,
            })
            .unwrap(),
        ))
    }
}
