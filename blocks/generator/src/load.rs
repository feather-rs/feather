//! Loads the vanilla blocks.json report into a `BlocksReport`, then
//! converts this report into a `Blocks`.

use crate::{Block, Blocks, Property, PropertyIdentifier, PropertyKind, PropertyValue, State};
use heck::CamelCase;
use indexmap::map::IndexMap;
use proc_macro2::{Ident, Span};
use serde::Deserialize;
use std::collections::HashMap;
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
    properties: HashMap<PropertyIdentifier, String>,
}

/// Parses the vanilla blocks report, returning a `Blocks`.
pub(super) fn load() -> anyhow::Result<Blocks> {
    let report = parse_report()?;

    let mut blocks = vec![];

    for (identifier, block) in &report.blocks {
        if let Some(block) = load_block(identifier, block)? {
            blocks.push(block);
        }
    }

    Ok(Blocks(blocks))
}

fn load_block(identifier: &str, block: &BlockDefinition) -> anyhow::Result<Option<Block>> {
    let identifier = strip_prefix(identifier)?;

    let property_struct_name = identifier.to_camel_case();

    let properties = load_block_properties(block, &property_struct_name);
    let states = load_block_states(&properties, block);

    let base_id = states.iter().map(|state| state.vanilla_id).min().unwrap();

    let block = Block {
        name: ident(identifier),
        property_struct_name: ident(property_struct_name),
        properties,
        states,
        base_id,
    };

    Ok(Some(block))
}

fn load_block_properties(
    block: &BlockDefinition,
    property_struct_name: &str,
) -> HashMap<PropertyIdentifier, Property> {
    let mut map = HashMap::with_capacity(block.properties.len());

    for (identifier, possible_values) in &block.properties {
        let identifier = fix_keywords(identifier);
        let (kind, possible_values) =
            guess_property_kind(possible_values, property_struct_name, identifier);

        let property = Property {
            name: ident(identifier),
            struct_name: ident(format!(
                "{}{}",
                property_struct_name,
                identifier.to_camel_case()
            )),
            possible_values,
            kind,
        };
        map.insert(identifier.to_owned(), property);
    }

    map
}

fn load_block_states(
    properties: &HashMap<PropertyIdentifier, Property>,
    block: &BlockDefinition,
) -> Vec<State> {
    let mut states = vec![];

    for state in &block.states {
        let mut property_values = HashMap::new();

        for (prop_identifier, value) in &state.properties {
            let prop_identifier = fix_keywords(prop_identifier);
            let property = &properties[prop_identifier];
            let value = match &property.kind {
                PropertyKind::Integer { range } => PropertyValue::Integer {
                    range: range.clone(),
                    value: value.parse().unwrap(),
                },
                PropertyKind::Boolean => PropertyValue::Boolean(value.parse().unwrap()),
                PropertyKind::Enum { name, .. } => PropertyValue::Enum {
                    name: name.clone(),
                    variant: ident(value.to_camel_case()),
                },
            };
            property_values.insert(prop_identifier.to_owned(), value);
        }

        let state = State {
            property_values,
            vanilla_id: state.id,
        };
        states.push(state);
    }

    states
}

fn guess_property_kind(
    possible_values: &[String],
    property_struct_name: &str,
    property_name: &str,
) -> (PropertyKind, Vec<PropertyValue>) {
    let first = &possible_values[0];

    if i32::from_str(first).is_ok() {
        // integer
        let as_integer: Vec<_> = possible_values
            .iter()
            .map(|x| i32::from_str(x).unwrap())
            .collect();

        let min = *as_integer.iter().min().unwrap();
        let max = *as_integer.iter().max().unwrap();

        let kind = PropertyKind::Integer { range: min..=max };

        let values = as_integer
            .into_iter()
            .map(|value| PropertyValue::Integer {
                value,
                range: min..=max,
            })
            .collect();
        (kind, values)
    } else if bool::from_str(first).is_ok() {
        // boolean
        (
            PropertyKind::Boolean,
            vec![PropertyValue::Boolean(false), PropertyValue::Boolean(true)],
        )
    } else {
        // enum
        let name = ident(format!(
            "{}{}",
            property_struct_name,
            property_name.to_camel_case()
        ));
        let variants: Vec<_> = possible_values
            .iter()
            .map(|variant| variant.to_camel_case())
            .map(ident)
            .collect();
        let possible_values = variants
            .iter()
            .cloned()
            .map(|variant| PropertyValue::Enum {
                name: name.clone(),
                variant,
            })
            .collect();
        let kind = PropertyKind::Enum { name, variants };

        (kind, possible_values)
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
