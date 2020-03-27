use serde::{Serialize, Deserialize};
use super::{Condition, Entry, Function, ExactOrRandom};

#[derive(Serialize, Deserialize)]
pub struct LootTable {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pools: Vec<Pool>
}

impl LootTable {
    pub fn from_json(json: &str) -> serde_json::Result<LootTable> {
        serde_json::from_str(json)
    }

    pub fn from_reader<R: std::io::Read>(reader: R) -> serde_json::Result<LootTable> {
        serde_json::from_reader(reader)
    }
}

impl From<&LootTable> for serde_json::Value {
    fn from(table: &LootTable) -> Self {
        serde_json::to_value(table).unwrap()
    }
}

impl From<&LootTable> for String {
    fn from(table: &LootTable) -> Self {
        serde_json::to_string(table).unwrap()
    }
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize)]
pub struct Pool {
    /// Determines conditions for this pool to be used. If multiple conditions are specified, all must pass.
    #[serde(default, skip_serializing_if = "Vec::is_empty")] 
    pub conditions: Vec<Condition>,
    /// Applies functions to the item stack being produced.
    #[serde(default, skip_serializing_if = "Vec::is_empty")] 
    pub functions: Vec<Function>,
    /// Specifies the number of rolls on the pool.
    pub rolls: ExactOrRandom<usize>,
    /// Specifies the number of bonus rolls.
    pub bonus_rolls: Option<ExactOrRandom<f32>>,
    /// A list of all things that can be produced by this pool. One entry is chosen per roll as a weighted random selection from all entries without failing conditions.
    pub entries: Vec<Entry>,
}
