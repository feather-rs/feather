//! Loads the vanilla blocks.json report into a `BlocksReport`, then
//! converts this report into a `Blocks`.

use crate::{Block, Blocks, Property, PropertyKind};
use heck::CamelCase;
use indexmap::map::IndexMap;
use once_cell::sync::Lazy;
use proc_macro2::{Ident, Span};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::str::FromStr;

/// Special property name overrides, to avoid names like "shape_neaaaassnn."
static NAME_OVERRIDES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    maplit::hashmap! {
        "east_tf" => "east_connected",
        "east_usn" => "east_wire",
        "north_tf" => "north_connected",
        "north_usn" => "north_wire",
        "west_tf" => "west_connected",
        "west_usn" => "west_wire",
        "south_tf" => "south_connected",
        "south_usn" => "south_wire",
        "facing_dnswe" => "facing_cardinal_and_down",
        "facing_neswud" => "facing_cubic",
        "facing_nswe" => "facing_cardinal",
        "half_ul" => "half", // not sure why there is a "top-bottom" variant and an "upper-lower" variant
        "half_tb" => "half",
        "kind_slr" => "chest_kind",
        "kind_tbd" => "slab_kind",
        "kind_ns" => "piston_kind",
        "mode_cs" => "comparator_mode",
        "mode_slcd" => "structure_block_mode",
        "shape_neaaaa" => "powered_rail_shape",
        "shape_siioo" => "stairs_shape",
        "shape_neaaaassnn" => "rail_shape",
    }
});

fn property_override<'a>(orig_name: &'a str, possible_values: &[String]) -> &'a str {
    let mut name = format!("{}_", orig_name);
    possible_values
        .iter()
        .map(|s| s.chars().next().unwrap())
        .for_each(|c| name.push(c.to_ascii_lowercase()));

    if let Some(name) = NAME_OVERRIDES.get(&name.as_str()) {
        *name
    } else {
        orig_name
    }
}

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
            let name = Self::update_name(&name);

            if possible_value_sets.len() == 1
                || possible_value_sets.iter().next().unwrap()[0]
                    .parse::<i32>()
                    .is_ok()
            {
                let possible_values = possible_value_sets.into_iter().next().unwrap();
                map.insert(
                    name.to_owned(),
                    Self::prop_from_possible_values_and_name(&name, possible_values),
                );
            } else {
                // There are multiple variants of this property, each with their own set of values.
                // Create properties suffixed with an index to differentiate between these variants.
                for possible_values in possible_value_sets {
                    // Name is the name of the property followed by the first letter of each possible value.
                    let name = {
                        let mut name = format!("{}_", name);
                        for value in &possible_values {
                            name.push(value.chars().next().unwrap().to_ascii_lowercase());
                        }
                        name
                    };

                    let name = Self::update_name(&name);

                    map.insert(
                        name.to_owned(),
                        Self::prop_from_possible_values_and_name(&name, possible_values),
                    );
                }
            }
        }

        map
    }

    fn update_name(name: &str) -> &str {
        match NAME_OVERRIDES.get(&name) {
            Some(x) => *x,
            None => name,
        }
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
    properties_store: &mut PropertyStore,
) -> anyhow::Result<Option<Block>> {
    let identifier = strip_prefix(identifier)?;

    let name_camel_case = identifier.to_camel_case();

    let properties = load_block_properties(block, properties_store);

    let index_parameters = load_block_index_parameters(block, &properties);

    let default_state = block
        .states
        .iter()
        .find(|state| state.default)
        .map(|state| state.properties.clone())
        .unwrap_or_default()
        .into_iter()
        .map(|(name, value)| {
            let name = fix_keywords(&name).to_owned();
            (
                if properties_store.properties[dbg!(&name)].len() > 1 {
                    property_override(&name, &block.properties[&name]).to_owned()
                } else {
                    name
                },
                value,
            )
        })
        .collect();

    let block = Block {
        name: ident(identifier),
        name_camel_case: ident(name_camel_case),
        properties,
        ids: Default::default(),
        default_state,
        index_parameters,
    };

    Ok(Some(block))
}

fn load_block_properties(block: &BlockDefinition, properties: &mut PropertyStore) -> Vec<String> {
    let mut props = vec![];

    for (identifier, possible_values) in &block.properties {
        let identifier = fix_keywords(identifier);

        properties.register(identifier.to_owned(), possible_values.clone());
        props.push(identifier.to_owned());
    }

    props
}

fn load_block_index_parameters(
    block: &BlockDefinition,
    block_props: &[String],
) -> HashMap<String, (u16, u16)> {
    let mut map = HashMap::with_capacity(block_props.len());

    let possible_values = block_props
        .iter()
        .map(|block_prop| block.properties.get(block_prop).map(Vec::len).unwrap_or(0))
        .map(|x| x as u16)
        .collect::<Vec<_>>();

    let mut previous_product = u16::max_value();
    for (i, block_prop) in block_props.iter().enumerate() {
        let stride = possible_values.iter().skip(i + 1).product::<u16>();
        let offset_coefficient = previous_product;
        if previous_product == u16::max_value() {
            previous_product = possible_values[i];
        } else {
            previous_product *= possible_values[i];
        }

        map.insert(block_prop.clone(), (offset_coefficient, stride));
    }

    map
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
