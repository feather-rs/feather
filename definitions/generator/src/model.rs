use indexmap::map::IndexMap;
use itertools::Either;
use serde::{Deserialize, Serialize};

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
        #[serde(default)]
        reverse: bool,
        #[serde(rename = "type")]
        typ: Type<'a>,
        mapping: IndexMap<VecOrOne<&'a str>, ron::Value>,
    },
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(untagged)]
pub enum VecOrOne<T> {
    Vec(Vec<T>),
    One(T),
}

impl<T> VecOrOne<T> {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &T> + 'a {
        match self {
            VecOrOne::Vec(v) => Either::Left(v.iter()),
            VecOrOne::One(v) => Either::Right(std::iter::once(v)),
        }
    }
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
