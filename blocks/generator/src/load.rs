//! Loads the vanilla blocks.json report into a `BlocksReport`, then
//! converts this report into a `Blocks`.

use crate::{Block, Blocks, Property, PropertyKind};
use heck::CamelCase;
use indexmap::map::IndexMap;
use proc_macro2::{Ident, Span};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::str::FromStr;

#[derive(Debug, Deserialize)]
struct BlocksReport {
    #[serde(flatten)]
    blocks: IndexMap<String, BlockDefinition>,
}

#[derive(Debug, Deserialize)]
struct BlockDefinition {
    states: Vec<StateDefinition>,
    #[serde(default)]
    properties: HashMap<String, Vec<String>>, // map from property name => possible values
}

#[derive(Debug, Deserialize)]
struct StateDefinition {
    id: u16,
    #[serde(default)]
    default: bool,
    #[serde(default)]
    properties: HashMap<String, String>,
}

#[derive(Debug, Default)]
struct PropertyStore {
    /// Mapping from property name to the set of different sets
    /// of values known for this property.
    properties: HashMap<String, BTreeSet<Vec<String>>>,
}

impl PropertyStore {
    fn register(&mut self, property: String, possible_values: impl IntoIterator<Item = String>) {
        self.properties
            .entry(property)
            .or_default()
            .insert(possible_values.into_iter().collect());
    }

    fn finish(self) -> BTreeMap<String, Property> {
        let mut map = BTreeMap::new();

        for (name, possible_value_sets) in self.properties {
            if possible_value_sets.len() == 1
                || possible_value_sets.iter().next().unwrap()[0]
                    .parse::<i32>()
                    .is_ok()
            {
                let possible_values = possible_value_sets.into_iter().next().unwrap();
                map.insert(
                    name.clone(),
                    Self::prop_from_possible_values_and_name(&name, possible_values),
                );
            } else {
                // There are multiple variants of this property, each with their own set of values.
                // Create properties suffixed with an index to differentiate between these variants.
                for (i, possible_values) in possible_value_sets.into_iter().enumerate() {
                    // Name is the name of the property followed by the first letter of each possible value.
                    let name = {
                        let mut name = format!("{}_", name);
                        for value in &possible_values {
                            name.push(value.chars().next().unwrap().to_ascii_lowercase());
                        }
                        name
                    };

                    map.insert(
                        name.clone(),
                        Self::prop_from_possible_values_and_name(&name, possible_values),
                    );
                }
            }
        }

        map
    }

    fn prop_from_possible_values_and_name(name: &str, possible_values: Vec<String>) -> Property {
        Property {
            name: ident(name),
            name_camel_case: ident(name.to_camel_case()),
            kind: guess_property_kind(&possible_values, &name.to_camel_case()),
        }
    }
}

/// Parses the vanilla blocks report, returning a `Blocks`.
pub(super) fn load() -> anyhow::Result<Blocks> {
    let report = parse_report()?;

    let mut blocks = vec![];
    let mut properties = PropertyStore::default();

    for (identifier, block) in &report.blocks {
        if let Some(block) = load_block(identifier, block, &mut properties)? {
            blocks.push(block);
        }
    }

    Ok(Blocks {
        blocks,
        property_types: properties.finish(),
    })
}

fn load_block(
    identifier: &str,
    block: &BlockDefinition,
    properties: &mut PropertyStore,
) -> anyhow::Result<Option<Block>> {
    let identifier = strip_prefix(identifier)?;

    let name_camel_case = identifier.to_camel_case();

    let properties = load_block_properties(block, &name_camel_case, properties);

    let block = Block {
        name: ident(identifier),
        name_camel_case: ident(name_camel_case),
        properties,
        ids: Default::default(),
        index_parameters: Default::default(),
    };

    Ok(Some(block))
}

fn load_block_properties(
    block: &BlockDefinition,
    name_camel_case: &str,
    properties: &mut PropertyStore,
) -> Vec<String> {
    let mut props = vec![];

    for (identifier, possible_values) in &block.properties {
        let identifier = fix_keywords(identifier);

        properties.register(identifier.to_owned(), possible_values.clone());
        props.push(identifier.to_owned());
    }

    props
}

fn guess_property_kind(possible_values: &[String], property_struct_name: &str) -> PropertyKind {
    let first = &possible_values[0];

    if i32::from_str(first).is_ok() {
        // integer
        let as_integer: Vec<_> = possible_values
            .iter()
            .map(|x| i32::from_str(x).unwrap())
            .collect();

        let min = *as_integer.iter().min().unwrap();
        let max = *as_integer.iter().max().unwrap();

        PropertyKind::Integer { range: min..=max }
    } else if bool::from_str(first).is_ok() {
        // boolean
        PropertyKind::Boolean
    } else {
        // enum
        let name = ident(property_struct_name);
        let variants: Vec<_> = possible_values
            .iter()
            .map(|variant| variant.to_camel_case())
            .map(ident)
            .collect();
        PropertyKind::Enum { name, variants }
    }
}

/// Renames Rust keywords to alternative identifiers.
fn fix_keywords(x: &str) -> &str {
    match x {
        "type" => "kind",
        x => x,
    }
}

/// Strips the minecraft: prefix from a block identifier.
fn strip_prefix(x: &str) -> anyhow::Result<&str> {
    const PREFIX: &str = "minecraft:";

    if x.len() <= PREFIX.len() {
        anyhow::bail!("missing minecraft: prefix for block {}", x);
    }

    Ok(&x[PREFIX.len()..])
}

pub fn ident(x: impl AsRef<str>) -> Ident {
    Ident::new(x.as_ref(), Span::call_site()) // span doesn't matter as this is not a proc macro
}

fn parse_report() -> anyhow::Result<BlocksReport> {
    static REPORT: &str = include_str!("../../../data/minecraft/generated/reports/blocks.json");

    let report = serde_json::from_str(REPORT)?;

    Ok(report)
}
