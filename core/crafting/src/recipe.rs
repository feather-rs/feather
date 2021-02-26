//! Intermediate representation of a recipe used directly
//! in solving.

use crate::{model, solver, Grid, TABLE_SIZE};
use anyhow::anyhow;
use arrayvec::ArrayVec;
use feather_items::{Item, ItemStack};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct ShapedRecipe {
    /// Input grid of required items.
    /// Normalized to the upper left corner:
    /// all empty rows on top and all empty
    /// columns to the left are removed.
    pub input: Grid,
    /// Output item stack.
    pub output: ItemStack,
}

#[derive(Clone, Debug)]
pub struct ShapelessRecipe {
    /// The set of input items required.
    /// Must be a sorted vector to allow for efficient
    /// comparison.
    pub input: ArrayVec<[Item; TABLE_SIZE]>,
    /// Output item stack.
    pub output: ItemStack,
}

#[derive(Clone, Debug)]
pub enum Recipe {
    Shaped(ShapedRecipe),
    Shapeless(ShapelessRecipe),
}

/// Converts a `model::Recipe` to a `Recipe`.
pub fn convert(model: model::Recipe) -> anyhow::Result<Recipe> {
    match model {
        model::Recipe::Shaped {
            pattern,
            key,
            output,
        } => convert_shaped(pattern, key, output),
        model::Recipe::Shapeless {
            ingredients,
            output,
        } => convert_shapeless(&ingredients, output),
    }
}

fn convert_shaped(
    pattern: model::Table,
    key: BTreeMap<char, model::Key>,
    output: model::Output,
) -> anyhow::Result<Recipe> {
    let mut input = Grid::default();

    for (y, row) in pattern.iter().enumerate() {
        for (x, slot) in row.as_str().chars().enumerate() {
            let key = key
                .get(&slot)
                .ok_or_else(|| anyhow!("no entry in key for character '{}'", slot))?;

            let item = convert_key(key)?;

            if let Some(item) = item {
                input[x][y] = Some(item);
            }
        }
    }

    solver::normalize(&mut input);

    let output = convert_output(&output)?;

    Ok(Recipe::Shaped(ShapedRecipe { input, output }))
}

fn convert_shapeless(ingredients: &[model::Key], output: model::Output) -> anyhow::Result<Recipe> {
    let mut input = ArrayVec::new();

    for ingredient in ingredients {
        let item = convert_key(ingredient)?;

        if let Some(item) = item {
            input.push(item);
        }
    }

    input.sort_unstable();

    let output = convert_output(&output)?;

    Ok(Recipe::Shapeless(ShapelessRecipe { input, output }))
}

fn convert_key(key: &model::Key) -> anyhow::Result<Option<Item>> {
    match key {
        model::Key::Item(identifier) => Ok(Some(
            Item::from_identifier(*identifier)
                .ok_or_else(|| anyhow!("invalid item '{}'", identifier))?,
        )),
        model::Key::Tag(_) => Ok(None), // not implemented
    }
}

fn convert_output(output: &model::Output) -> anyhow::Result<ItemStack> {
    let ty = Item::from_identifier(output.item)
        .ok_or_else(|| anyhow!("invalid item '{}'", output.item))?;
    Ok(ItemStack::new(ty, output.count))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_output() {
        assert_eq!(
            convert_output(&model::Output {
                item: "minecraft:stone",
                count: 5
            })
            .unwrap(),
            ItemStack::new(Item::Stone, 5)
        );
    }

    #[test]
    fn test_convert_output_invalid_item() {
        assert!(convert_output(&model::Output {
            item: "minecraft:doesnotexist",
            count: 1
        })
        .is_err());
    }

    #[test]
    fn test_convert_key() {
        assert_eq!(
            convert_key(&model::Key::Item("minecraft:diamond_sword")).unwrap(),
            Some(Item::DiamondSword)
        );
        assert_eq!(
            convert_key(&model::Key::Tag("unimplemented")).unwrap(),
            None
        );
    }
}
