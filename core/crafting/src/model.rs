//! Serde data model for recipe files.
use crate::{TABLE_SIZE, TABLE_WIDTH};
use arrayvec::{ArrayString, ArrayVec};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type TableEntry = ArrayString<[u8; TABLE_WIDTH]>;
pub type Table = ArrayVec<[TableEntry; TABLE_WIDTH]>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Recipe<'a> {
    #[serde(rename = "minecraft:crafting_shaped")]
    Shaped {
        pattern: Table,
        #[serde(borrow)]
        key: BTreeMap<char, Key<'a>>,
        #[serde(rename = "result")]
        output: Output<'a>,
    },
    #[serde(rename = "minecraft:crafting_shapeless")]
    Shapeless {
        ingredients: ArrayVec<[Key<'a>; TABLE_SIZE]>,
        #[serde(rename = "result")]
        output: Output<'a>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "lowercase")]
pub enum Key<'a> {
    Item(&'a str),
    Tag(&'a str),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Output<'a> {
    /// Identifier of output item.
    pub item: &'a str,
    #[serde(default = "one")]
    pub count: u8,
}

const fn one() -> u8 {
    1
}
