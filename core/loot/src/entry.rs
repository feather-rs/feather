use serde::{Serialize, Deserialize};
use super::{Condition, Function, LootTable};

type Item = String;

#[derive(Serialize, Deserialize)]
pub enum DynamicName {
    #[serde(rename = "contents")]
    Contents,
    #[serde(rename = "self")]
    This
}

#[derive(Serialize, Deserialize)]
pub struct ConditionalFunction {
    #[serde(flatten)]
    function: Function,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    conditions: Vec<Condition>
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Entry {
    Item {
        name: Item,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        conditions: Vec<Condition>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        functions: Vec<ConditionalFunction>,
        #[serde(default, skip_serializing_if = "is_default")]
        weight: usize,
        quality: Option<isize>,
    },
    LootTable {
        name: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        conditions: Vec<Condition>,
        weight: usize,
        quality: Option<isize>,
    },
    Empty,
}

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}