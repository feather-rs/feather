use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Loads a model file.
pub fn from_str(s: &str) -> anyhow::Result<ModelFile> {
    ron::de::from_str(s).map_err(anyhow::Error::from)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ModelFile<'a> {
    Single(#[serde(borrow)] Model<'a>),
    Multiple(#[serde(borrow)] Vec<Model<'a>>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Model<'a> {
    Enum {
        name: &'a str,
        variants: Vec<&'a str>,
    },
    Property {
        on: &'a str,
        name: &'a str,
        #[serde(rename = "type")]
        typ: Type<'a>,
        mapping: BTreeMap<VecOrOne<&'a str>, ron::Value>,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, Ord, PartialEq, Eq)]
#[serde(untagged)]
pub enum VecOrOne<T> {
    Vec(Vec<T>),
    One(T),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Type<'a> {
    Slice(Box<Type<'a>>),
    #[serde(rename = "u32")]
    U32,
    #[serde(rename = "f64")]
    F64,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "bool")]
    Bool,
    Custom(&'a str),
}
