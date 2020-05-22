//! Defines a Serde model for loot tables. Used as an intermediate
//! representation of the table.
//!
//! The build script for `feather-loot` requires this functionality,
//! which is why it has been split into another crate. This
//! may be alleviated in the future.

use inlinable_string::InlinableString;
use rand::Rng;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::collections::HashMap;

/// The set of all loaded loot tables.
/// TODO: consider a typed API as opposed to a string-based one.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootTableSet(pub HashMap<InlinableString, LootTable, ahash::RandomState>);

impl Default for LootTableSet {
    fn default() -> Self {
        LootTableSet(HashMap::with_hasher(ahash::RandomState::new()))
    }
}

/// See https://minecraft.gamepedia.com/Loot_table
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootTable {
    #[serde(rename = "type")]
    pub kind: Option<LootTableKind>,
    #[serde(default)]
    pub pools: SmallVec<[Pool; 2]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pool {
    /// Conditions which must be satisfied for this pool
    /// to be applied.
    #[serde(default)]
    pub conditions: SmallVec<[Condition; 1]>,
    /// Functions to apply to the resulting item stack.
    #[serde(default)]
    pub functions: SmallVec<[Function; 2]>,
    /// Number of times to take an item from the pool.
    pub rolls: FixedOrRandom,
    /// The entries in the pool. Each roll, one entry
    /// is selected at random from the set of entries
    /// whose conditions are satisfied. The item
    /// from the entry is then yielded.
    pub entries: SmallVec<[Entry; 2]>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    /// Conditions which must be satisfied for this entry
    /// to apply.
    #[serde(default)]
    pub conditions: SmallVec<[Condition; 1]>,
    /// The kind of this entry. Determines in what way
    /// we should produce the resulting item.
    #[serde(rename = "type")]
    pub kind: EntryKind,
    /// Value depends on `kind`. See [`EntryKind`](enum.EntryKind.html)
    #[serde(default)]
    pub name: InlinableString,
    /// A list of child `Entry`s interpreted depending on the value of `kind`.
    #[serde(default)]
    pub children: Vec<Entry>,
    /// If `kind == EntryKind::Tag`, determines how the item
    /// is selected from the tag.
    #[serde(default)]
    pub expand: bool,
    /// Functions to apply to the resulting items.
    #[serde(default)]
    pub functions: SmallVec<[Function; 2]>,
    /// Weight of this entry as compared to others,
    /// when there are multiple entries in a pool.
    #[serde(default = "one")]
    pub weight: u32,
}

const fn one() -> u32 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryKind {
    /// An item type. The name field specifies
    /// the identifier of the item.
    #[serde(alias = "minecraft:item")]
    Item,
    /// Depending on the value of the `expand`
    /// field:
    /// * Gives a random item from the tag if set to `true`.
    /// * Gives all items in the tag if set to `false`.
    ///
    /// The tag is chosen based on the `name` field.
    #[serde(alias = "minecraft:tag")]
    Tag,
    /// Determine the item by evaluating another loot table.
    /// Name field specifies path of the loot table.
    #[serde(alias = "minecraft:loot_table")]
    LootTable,
    /// A set of multiple child entries, each of them applied.
    /// Children field contains the entries.
    #[serde(alias = "minecraft:group")]
    Group,
    /// Like `Group`, but only selects one entry from the sub-list:
    /// the first one whose conditions are satisfied.
    #[serde(alias = "minecraft:alternatives")]
    Alternatives,
    /// Like `Group`, but only applies entries until a condition
    /// is not satisfied.
    #[serde(alias = "minecraft:sequence")]
    Sequence,
    /// Dynamically determine result based on block entity.
    /// For chests, the `name` field can be set to "contents".
    /// For others, such as banners, skulls, etc., the name field
    /// is set to "self."
    #[serde(alias = "minecraft:dynamic")]
    Dynamic,
    /// No action (seems to be used to add dead weights)
    #[serde(alias = "minecraft:empty")]
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LootTableKind {
    /// Loot table for block drops when block is broken,
    #[serde(rename = "minecraft:block")]
    Block,
    #[serde(other)]
    /// Unknown loot table (one we don't use yet)
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "condition")]
#[serde(rename_all = "snake_case")]
pub enum Condition {
    /// Tool used to break block must match this item.
    #[serde(alias = "minecraft:match_tool")]
    MatchTool { predicate: ItemPredicate },

    #[serde(alias = "minecraft:random_chance")]
    RandomChance { chance: f64 },

    // TODO
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemPredicate {
    /// Enchantments present on the item
    #[serde(default)]
    pub enchantments: SmallVec<[Enchantment; 2]>,
    /// Item identifier of the held item
    pub item: Option<InlinableString>,
    // TODO: tag, count, durability, nbt, potion
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "enchantment")]
pub enum Enchantment {
    // TODO (blocked on Feather enchantment support)
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    /// Function will only be applied if conditions are satisfied.
    #[serde(default)]
    pub conditions: SmallVec<[Condition; 1]>,
    /// Kind of this function. Determines what the function does.
    #[serde(flatten)]
    pub kind: FunctionKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "function")]
pub enum FunctionKind {
    // TODO
    // apply_bonus, copy_name, copy_nbt, copy_state, enchant_randomly, enchant_with_levels, exploration_map,
    // explosion_decay, furnace_smelt, fill_player_head, set_attribute, set_contents, set_damage, set_lore,
    // set_name, set_nbt, set_stew_effect
    /// Sets the stack amount.
    #[serde(alias = "minecraft:set_count")]
    SetCount { count: SetCountValue },

    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetCountValue {
    Fixed(u32),
    Random(SetCountRandom),
}

impl SetCountValue {
    pub fn sample(&self, rng: &mut impl Rng) -> u32 {
        match self {
            SetCountValue::Fixed(n) => *n,
            SetCountValue::Random(random) => match random {
                SetCountRandom::Uniform { min, max } => {
                    rng.gen_range(min.round() as u32, max.round() as u32 + 1)
                }
                SetCountRandom::Binomial { n, p } => {
                    let p = p.min(1.0).max(0.0);
                    debug_assert!(p <= 1.0);
                    debug_assert!(p >= 0.0);
                    (0..n.round() as u32)
                        .take(1000)
                        .map(|_| if rng.gen_bool(p) { 1 } else { 0 })
                        .sum()
                }
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetCountRandom {
    Uniform {
        min: f64,
        max: f64,
    },
    Binomial {
        /// Number of rolls
        n: f64,
        /// Chance of each roll
        p: f64,
    },
}

/// A struct which may have either a fixed value
/// or an inclusive range of values, selected randomly.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FixedOrRandom {
    Fixed(f64),
    Random { min: f64, max: f64 },
}

impl FixedOrRandom {
    /// Given an RNG, returns a value for this integer.
    pub fn sample(&self, rng: &mut impl Rng) -> u32 {
        match self {
            FixedOrRandom::Fixed(n) => n.round() as u32,
            FixedOrRandom::Random { min, max } => {
                rng.gen_range(min.round() as u32, max.round() as u32 + 1)
            }
        }
    }
}
