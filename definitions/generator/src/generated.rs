//! Writes out generated data files, such as block and item enums.

use crate::model::{Model, ModelFile, Type, VecOrOne};
use anyhow::Context;
use std::fs::File;
use std::io::Write;

use ron::value::Number;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub fn write(dir: &str) -> anyhow::Result<()> {
    let block = format!("{}/block.ron", dir);
    let item = format!("{}/item.ron", dir);

    std::fs::create_dir_all(dir)
        .with_context(|| format!("failed to create directory `{}`", dir))?;

    let model =
        load_block_model().context("failed to load blocks.json from minecraft-data repo")?;
    let gblock = generate_block(&model).context("failed to generate block data file")?;

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

fn generate_block<'a>(model: &'a BlockModel) -> anyhow::Result<ModelFile<'a>> {
    let known_bounding_boxes: BTreeSet<_> =
        model.0.iter().map(|block| block.bounding_box).collect();

    let bbox = Model::Enum {
        name: "block_bounding_box",
        variants: known_bounding_boxes.into_iter().collect(),
    };

    let display_name = block_property(
        "display_name",
        true,
        model,
        |block| ron::Value::String(block.display_name.to_owned()),
        Type::String,
    );
    let diggable = block_property(
        "diggable",
        false,
        model,
        |block| ron::Value::Bool(block.diggable),
        Type::Bool,
    );
    let hardness = block_property(
        "hardness",
        false,
        model,
        |block| ron::Value::Number(Number::new(block.hardness.unwrap_or_default())),
        Type::F64,
    );
    let opaque = block_property(
        "opaque",
        false,
        model,
        |block| ron::Value::Bool(!block.transparent),
        Type::Bool,
    );

    let kind = Model::Enum {
        name: "block_kind",
        variants: model.0.iter().map(|block| block.name).collect(),
    };

    Ok(ModelFile::Multiple(vec![
        kind,
        bbox,
        display_name,
        diggable,
        hardness,
        opaque,
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
