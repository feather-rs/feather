//! Loads the vanilla blocks.json report into a `BlocksReport`, then
//! converts this report into a `Blocks`.

use crate::{Block, Blocks, Property, PropertyKind};
use anyhow::Context;
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
       "half_ul" => "half_upper_lower",
       "half_tb" => "half_top_bottom",
       "kind_slr" => "chest_kind",
       "kind_tbd" => "slab_kind",
       "kind_ns" => "piston_kind",
       "mode_cs" => "comparator_mode",
       "mode_slcd" => "structure_block_mode",
       "shape_neaaaa" => "powered_rail_shape",
       "shape_siioo" => "stairs_shape",
       "shape_neaaaassnn" => "rail_shape",
       "level_0_3" => "cauldron_level",
       "level_0_15" => "water_level",
       "type_slr" => "chest_kind",
       "type_tbd" => "slab_kind",
       "type_ns" => "piston_kind",
    }
});

#[derive(Debug, Deserialize)]
struct BlocksReport {
    #[serde(flatten)]
    blocks: IndexMap<String, BlockDefinition>,
}

#[derive(Debug, Deserialize)]
struct BlockDefinition {
    states: Vec<StateDefinition>,
    #[serde(default)]
    properties: BTreeMap<String, Vec<String>>, // map from property name => possible values
}

#[derive(Debug, Deserialize)]
struct StateDefinition {
    id: u16,
    #[serde(default)]
    default: bool,
    #[serde(default)]
    properties: BTreeMap<String, String>,
}

#[derive(Debug, Default, Clone)]
struct PropertyStore {
    /// Mapping from property name to the set of different sets
    /// of values known for this property.
    properties: BTreeMap<String, BTreeSet<Vec<String>>>,
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

            if possible_value_sets.len() == 1 {
                let possible_values = possible_value_sets.into_iter().next().unwrap();
                map.insert(
                    name.to_owned(),
                    Self::prop_from_possible_values_and_name(name, name, possible_values),
                );
            } else {
                // There are multiple variants of this property, each with their own set of values.
                // Create properties suffixed with an index to differentiate between these variants.
                for possible_values in possible_value_sets {
                    // Name is the name of the property followed by the first letter of each possible value.
                    // If it's an integer, it is the range of possible values.
                    let new_name = if possible_values[0].parse::<i32>().is_ok() {
                        let as_integer = possible_values
                            .iter()
                            .map(String::as_str)
                            .map(i32::from_str)
                            .map(Result::unwrap)
                            .collect::<Vec<_>>();

                        let min = *as_integer.iter().min().unwrap();
                        let max = *as_integer.iter().max().unwrap();

                        format!("{}_{}_{}", name, min, max)
                    } else {
                        let mut name = format!("{}_", name);
                        for value in &possible_values {
                            name.push(value.chars().next().unwrap().to_ascii_lowercase());
                        }
                        name
                    };

                    let new_name = Self::update_name(&new_name);

                    map.insert(
                        new_name.to_owned(),
                        Self::prop_from_possible_values_and_name(new_name, name, possible_values),
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

    fn prop_from_possible_values_and_name(
        name: &str,
        real_name: &str,
        possible_values: Vec<String>,
    ) -> Property {
        Property {
            name: ident(name),
            real_name: real_name.to_owned(),
            _name_camel_case: ident(name.to_camel_case()),
            kind: guess_property_kind(&possible_values, &name.to_camel_case()),
            possible_values,
        }
    }
}

/// Parses the vanilla blocks report, returning a `Blocks`.
pub(super) fn load() -> anyhow::Result<Blocks> {
    let mut report = parse_report()?;

    let mut blocks = vec![];
    let properties = fix_property_names(&mut report);

    for (identifier, block) in &report.blocks {
        if let Some(block) = load_block(identifier, block)? {
            blocks.push(block);
        }
    }

    Ok(Blocks {
        blocks,
        property_types: properties.finish(),
    })
}

fn fix_property_names(report: &mut BlocksReport) -> PropertyStore {
    let mut store = PropertyStore::default();

    for block in report.blocks.values() {
        for (property_name, possible_values) in &block.properties {
            store.register(property_name.to_owned(), possible_values.clone());
        }
    }

    // Correct block property names
    let result = store.clone().finish();

    for block in report.blocks.values_mut() {
        let block: &mut BlockDefinition = block;
        let mut overrides = vec![];
        for (property_name, possible_values) in &mut block.properties {
            if result.get(property_name).is_none() {
                let name = if possible_values[0].parse::<i32>().is_ok() {
                    let as_integer = possible_values
                        .iter()
                        .map(String::as_str)
                        .map(i32::from_str)
                        .map(Result::unwrap)
                        .collect::<Vec<_>>();

                    let min = *as_integer.iter().min().unwrap();
                    let max = *as_integer.iter().max().unwrap();

                    format!("{}_{}_{}", property_name, min, max)
                } else {
                    let mut name = format!("{}_", property_name);
                    for value in possible_values {
                        name.push(value.chars().next().unwrap().to_ascii_lowercase());
                    }
                    name
                };
                let name = if let Some(name) = NAME_OVERRIDES.get(&name.as_str()) {
                    (*name).to_owned()
                } else {
                    name
                };

                overrides.push((property_name.to_owned(), name));
            }
        }

        for (old_name, new_name) in overrides {
            let old_values = block.properties.remove(&old_name).unwrap();
            block.properties.insert(new_name.clone(), old_values);

            for state in &mut block.states {
                let old_value = state.properties.remove(&old_name).unwrap();
                state.properties.insert(new_name.clone(), old_value);
            }
        }
    }

    store
}

fn load_block(identifier: &str, block: &BlockDefinition) -> anyhow::Result<Option<Block>> {
    let identifier = strip_prefix(identifier)?;

    let name_camel_case = identifier.to_camel_case();

    let properties = load_block_properties(block);

    let index_parameters = load_block_index_parameters(block, &properties);

    let ids = load_block_ids(block);

    let default_state = block
        .states
        .iter()
        .find(|state| state.default)
        .map(|state| state.properties.clone())
        .unwrap_or_default()
        .into_iter()
        .collect();

    let block = Block {
        name: ident(identifier),
        name_camel_case: ident(name_camel_case),
        properties,
        ids,
        default_state,
        index_parameters,
    };

    Ok(Some(block))
}

fn load_block_properties(block: &BlockDefinition) -> Vec<String> {
    let mut props = vec![];

    for identifier in block.properties.keys() {
        props.push(identifier.to_owned());
    }

    props
}

fn load_block_index_parameters(
    block: &BlockDefinition,
    block_props: &[String],
) -> BTreeMap<String, (u16, u16)> {
    let mut map = BTreeMap::new();

    let possible_values = block_props
        .iter()
        .map(|block_prop| block.properties.get(block_prop).map(Vec::len).unwrap_or(0))
        .map(|x| x as u16)
        .collect::<Vec<_>>();

    for (i, block_prop) in block_props.iter().enumerate() {
        let stride = possible_values.iter().skip(i + 1).product::<u16>();
        let offset_coefficient = stride * possible_values[i];

        map.insert(block_prop.clone(), (offset_coefficient, stride));
    }

    map
}

fn load_block_ids(block: &BlockDefinition) -> Vec<(Vec<(String, String)>, u16)> {
    let mut res: Vec<(Vec<(String, String)>, u16)> = vec![];

    for state in &block.states {
        let properties = state.properties.clone().into_iter().collect();

        res.push((properties, state.id));
    }

    res
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
    let report = std::fs::read_to_string("blocks.json")
    .context("failed to load blocks report. Please run the vanilla data generator and copy blocks.json to the current directory")?;

    Ok(serde_json::from_str(&report)?)
}
