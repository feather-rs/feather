use crate::{
    ArmorStandSpawnEggCompound, BlockCompound, BookAndQuillCompound, BucketOfFishCompound,
    CompassCompound, CrossbowCompound, DebugStickCompound, DisplayPropertiesCompound,
    EnchantmentCompound, FireworkCompound, GeneralCompound, MapCompound, PotionEffectCompound,
    SuspiciousStewCompound, WrittenBookCompound,
};
use serde::{Deserialize, Serialize};

/// Represents the metadata of an `ItemStack`. Can contain
/// multiple `ItemMetaTagCompounds`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ItemStackMeta {
    /// All the tag compounds found in this enum.
    pub(crate) compounds: Vec<ItemMetaTagCompound>,
}

/// All the possible meta tag groups that an
/// `ItemStack` can have.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ItemMetaTagCompound {
    General(GeneralCompound),
    Block(BlockCompound),
    Enchantments(EnchantmentCompound),
    AttributeModifiers,
    PotionEffects(PotionEffectCompound),
    Crossbows(CrossbowCompound),
    DisplayProperties(DisplayPropertiesCompound),
    WrittenBooks(WrittenBookCompound),
    BooksAndQuills(BookAndQuillCompound),
    PlayerHeads,
    Fireworks(FireworkCompound),
    ArmorStandsSpawnEggs(ArmorStandSpawnEggCompound),
    FishBuckets(BucketOfFishCompound),
    Maps(MapCompound),
    SuspiciousStew(SuspiciousStewCompound),
    DebugSticks(DebugStickCompound),
    Compasses(CompassCompound),
}
