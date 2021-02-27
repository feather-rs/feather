//! Writes out generated data files, such as block and item enums.

use crate::model::{Model, ModelFile, Type, VecOrOne};
use anyhow::Context;
use std::fs::File;
use std::io::Write;

use indexmap::map::IndexMap;
use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use ron::value::Number;
use serde::de::IgnoredAny;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub fn write(dir: &str) -> anyhow::Result<()> {
    let block = format!("{}/block.ron", dir);
    let item = format!("{}/item.ron", dir);

    std::fs::create_dir_all(dir)
        .with_context(|| format!("failed to create directory `{}`", dir))?;

    let block_model =
        load_block_model().context("failed to load blocks.json from minecraft-data repo")?;
    let collision_shape_model = load_collision_shape_model()
        .context("failed to load blockCollisionShapes.json from minecraft-data repo")?;
    let gblock = generate_block(&block_model, &collision_shape_model)
        .context("failed to generate block data file")?;

    let model: ItemModel = serde_json::from_slice(feather_data::minecraft_data::ITEMS)?;
    let gitem = generate_item(&model).context("failed to generate item data file")?;

    for (path, content) in &[(block, gblock), (item, gitem)] {
        let mut file =
            File::create(path).with_context(|| format!("failed to create `{}`", path))?;
        let s = ron::ser::to_string_pretty(content, Default::default())?;

        file.write_all(b"// This files is @generated\n")
            .and_then(|_| file.write_all(s.as_bytes()))
            .with_context(|| format!("failed to write to `{}`", path))?;
        file.flush()?;
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct BlockModel<'a>(#[serde(borrow)] Vec<Block<'a>>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Block<'a> {
    id: i32,
    display_name: &'a str,
    name: &'a str,
    hardness: Option<f64>,
    min_state_id: i32,
    max_state_id: u32,
    drops: Vec<usize>,
    diggable: bool,
    transparent: bool,
    filter_light: u8,
    emit_light: u8,
    bounding_box: &'a str,
    stack_size: u32,
}

fn load_block_model() -> anyhow::Result<BlockModel<'static>> {
    serde_json::from_slice(feather_data::minecraft_data::BLOCKS).map_err(anyhow::Error::from)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CollisionShapeModel<'a> {
    #[serde(borrow)]
    blocks: IndexMap<&'a str, VecOrOne<usize>>,
    shapes: IgnoredAny,
}

fn load_collision_shape_model() -> anyhow::Result<CollisionShapeModel<'static>> {
    serde_json::from_slice(feather_data::minecraft_data::BLOCKCOLLISIONSHAPES)
        .map_err(anyhow::Error::from)
}

static SIMPLIFIED_REGEX: Lazy<Vec<(Regex, &'static str)>> = Lazy::new(|| {
    let mut vec = Vec::new();

    vec.push((Regex::new(r"^.*air$").unwrap(), "air"));
    vec.push((Regex::new(r"^.+_planks$").unwrap(), "planks"));
    vec.push((Regex::new(r"^(\w+|dark_oak)_sapling$").unwrap(), "sapling"));
    vec.push((Regex::new(r"^.+_(log|wood)$").unwrap(), "log"));
    vec.push((Regex::new(r"^.+_leaves$").unwrap(), "leaves"));
    vec.push((Regex::new(r"^.+_bed$").unwrap(), "bed"));
    vec.push((Regex::new(r"^.+_wool$").unwrap(), "wool"));
    vec.push((
        Regex::new(r"^(allium|poppy|dandelion|\w+_(orchid|bluet|tulip|daisy))$").unwrap(),
        "flower",
    ));
    vec.push((
        Regex::new(r"^(oak|spruce|birch|jungle|acacia|dark_oak)_pressure_plate$").unwrap(),
        "wooden_pressure_plate",
    ));
    vec.push((Regex::new(r"^.+_stained_glass$").unwrap(), "stained_glass"));
    vec.push((
        Regex::new(r"^(oak|spruce|birch|jungle|acacia|dark_oak)_trapdoor$").unwrap(),
        "wooden_trapdoor",
    ));
    vec.push((Regex::new(r"^potted_.+$").unwrap(), "potted_plant"));
    vec.push((
        Regex::new(r"^(oak|spruce|birch|jungle|acacia|dark_oak)_button$").unwrap(),
        "wooden_button",
    ));
    vec.push((Regex::new(r"^(\w+_)?anvil$").unwrap(), "anvil"));
    vec.push((
        Regex::new(r"^.+_glazed_terracotta$").unwrap(),
        "glazed_terracotta",
    )); // this will be matched first
    vec.push((Regex::new(r"^.*terracotta$").unwrap(), "terracotta"));
    vec.push((
        Regex::new(r"^.+_stained_glass_pane$").unwrap(),
        "stained_glass_pane",
    ));
    vec.push((Regex::new(r"^.+_carpet$").unwrap(), "carpet"));
    vec.push((Regex::new(r"^.+_wall_banner$").unwrap(), "wall_banner")); // this will be matched first
    vec.push((Regex::new(r"^.+_banner$").unwrap(), "banner"));
    vec.push((Regex::new(r"^.+_slab$").unwrap(), "slab"));
    vec.push((Regex::new(r"^.+_stairs$").unwrap(), "stairs"));
    vec.push((Regex::new(r"^.+_fence_gate$").unwrap(), "fence_gate"));
    vec.push((Regex::new(r"^.+_fence$").unwrap(), "fence"));
    vec.push((
        Regex::new(r"^(oak|spruce|birch|jungle|acacia|dark_oak)_door$").unwrap(),
        "wooden_door",
    ));
    vec.push((Regex::new(r"^.*shulker_box$").unwrap(), "shulker_box"));
    vec.push((Regex::new(r"^.+_concrete$").unwrap(), "concrete"));
    vec.push((
        Regex::new(r"^.+_concrete_powder$").unwrap(),
        "concrete_powder",
    ));
    vec.push((Regex::new(r"^.+_coral$").unwrap(), "coral"));
    vec.push((Regex::new(r"^.+_coral_block$").unwrap(), "coral_block"));
    vec.push((Regex::new(r"^.+_coral_fan$").unwrap(), "coral_fan"));
    vec.push((
        Regex::new(r"^.+_coral_wall_fan$").unwrap(),
        "coral_wall_fan",
    ));
    vec.push((Regex::new(r"^\w+_mushroom$").unwrap(), "mushroom"));

    vec
});

fn to_simplified_name(block_name: &str) -> Option<&'static str> {
    for (regex, replacement) in SIMPLIFIED_REGEX.iter() {
        if regex.is_match(block_name) {
            return Some(replacement);
        }
    }

    None
}

fn generate_block<'a>(
    block_model: &'a BlockModel,
    collision_shape_model: &'a CollisionShapeModel,
) -> anyhow::Result<ModelFile<'a>> {
    let known_bounding_boxes: BTreeSet<_> = block_model
        .0
        .iter()
        .map(|block| block.bounding_box)
        .collect();

    let bbox = Model::Enum {
        name: "block_bounding_box",
        variants: known_bounding_boxes.into_iter().collect(),
    };

    let display_name = block_property(
        "display_name",
        true,
        block_model,
        |block| ron::Value::String(block.display_name.to_owned()),
        Type::String,
    );
    let diggable = block_property(
        "diggable",
        false,
        block_model,
        |block| ron::Value::Bool(block.diggable),
        Type::Bool,
    );
    let hardness = block_property(
        "hardness",
        false,
        block_model,
        |block| ron::Value::Number(Number::new(block.hardness.unwrap_or_default())),
        Type::F64,
    );
    let opaque = block_property(
        "opaque",
        false,
        block_model,
        |block| ron::Value::Bool(!block.transparent),
        Type::Bool,
    );
    let solid = block_property(
        "solid",
        false,
        block_model,
        |block| ron::Value::Bool(block.bounding_box == "block"),
        Type::Bool,
    );
    let full_block = Model::Property {
        on: "block_kind",
        name: "full_block",
        reverse: false,
        typ: Type::Bool,
        mapping: collision_shape_model
            .blocks
            .iter()
            .map(|(&name, cb_index)| {
                (
                    VecOrOne::One(name),
                    ron::Value::Bool(matches!(cb_index, VecOrOne::One(cb_index) if *cb_index == 1)),
                )
            })
            .collect(),
    };
    let to_simplified_kind = Model::Property {
        on: "block_kind",
        name: "to_simplified_kind",
        reverse: false,
        typ: Type::Custom("simplified_block_kind"),
        mapping: block_model
            .0
            .iter()
            .map(|block| block.name)
            .map(|name| {
                (
                    VecOrOne::One(name),
                    ron::Value::String(to_simplified_name(name).unwrap_or(name).to_string()),
                )
            })
            .collect(),
    };

    let kind = Model::Enum {
        name: "block_kind",
        variants: block_model.0.iter().map(|block| block.name).collect(),
    };

    let simplified_kind = Model::Enum {
        name: "simplified_block_kind",
        variants: block_model
            .0
            .iter()
            .map(|block| block.name)
            .map(|name| to_simplified_name(name).unwrap_or(name))
            .unique()
            .collect(),
    };

    Ok(ModelFile::Multiple(vec![
        kind,
        bbox,
        display_name,
        diggable,
        hardness,
        opaque,
        solid,
        full_block,
        simplified_kind,
        to_simplified_kind,
    ]))
}

fn block_property<'a>(
    name: &'a str,
    reverse: bool,
    model: &BlockModel<'a>,
    mut accessor: impl FnMut(&Block) -> ron::Value,
    typ: Type<'a>,
) -> Model<'a> {
    Model::Property {
        on: "block_kind",
        name,
        typ,
        reverse,
        mapping: model
            .0
            .iter()
            .map(|block| (VecOrOne::One(block.name), accessor(block)))
            .collect(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ItemModel<'a>(#[serde(borrow)] Vec<Item<'a>>);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Item<'a> {
    id: i32,
    display_name: &'a str,
    name: &'a str,
    stack_size: u32,
}

fn generate_item<'a>(model: &'a ItemModel) -> anyhow::Result<ModelFile<'a>> {
    let item = Model::Enum {
        name: "item",
        variants: model.0.iter().map(|item| item.name).collect(),
    };

    let display_name = item_property(
        "display_name",
        false,
        &model,
        |item| ron::Value::String(item.display_name.to_string()),
        Type::String,
    );
    let stack_size = item_property(
        "stack_size",
        false,
        &model,
        |item| ron::Value::Number(ron::value::Number::new(item.stack_size as f64)),
        Type::U32,
    );

    let vanilla_id = item_property(
        "vanilla_id",
        true,
        &model,
        |item| ron::Value::Number(ron::value::Number::new(item.id as f64)),
        Type::U32,
    );

    let identifier = item_property(
        "identifier",
        true,
        &model,
        |item| ron::Value::String(format!("minecraft:{}", item.name)),
        Type::String,
    );

    Ok(ModelFile::Multiple(vec![
        item,
        display_name,
        stack_size,
        vanilla_id,
        identifier,
    ]))
}

fn item_property<'a>(
    name: &'a str,
    reverse: bool,
    model: &'a ItemModel,
    mut accessor: impl FnMut(&Item) -> ron::Value,
    typ: Type<'a>,
) -> Model<'a> {
    Model::Property {
        on: "item",
        name,
        typ,
        reverse,
        mapping: model
            .0
            .iter()
            .map(|item| (VecOrOne::One(item.name), accessor(item)))
            .collect(),
    }
}
