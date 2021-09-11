use std::result::Result;

use libcraft_items::ItemStack;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "minecraft:crafting_shaped")]
pub struct ShapedRecipe {
    input: [[ItemStack; 3]; 3],
    output: ItemStack,
}

impl ShapedRecipe {
    // Code to transform into hash and do whatever
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "minecraft:crafting_shapeless")]
pub struct ShapelessRecipe {
    input: Vec<ItemStack>,
}

impl ShapelessRecipe {

}

#[derive(Serialize, Deserialize)]
pub enum SpecialRecipe {}

#[derive(Serialize, Deserialize)]
pub enum CraftingRecipe {
    Shaped(ShapedRecipe),
    Shapeless(ShapelessRecipe),
    Special(SpecialRecipe),
}

impl CraftingRecipe {
    fn Deserialize(json: String) -> Result<CraftingRecipe, serde_json::Error> {
        let recipe: CraftingRecipe = serde_json::from_str(&json)?;

        Ok(recipe)
    }
}

#[cfg(test)]
mod tests {
    use super::CraftingRecipe;

    #[test]
    fn deserialize() {
        let json = r#"
        {
            "type": "minecraft:crafting_shaped",
            "pattern":
            [
                "xxa",
                "x x",
                "xxx"
            ],
            "key":
            {
                "x":
                {
                    "tag": "forge:gems/diamond"
                },
                "a":
                {
                    "item": "mymod:myfirstitem"
                }
            },
            "result":
            {
                "item": "mymod:myitem",
                "count": 9
            }
        }
        "#;
        let recipe = CraftingRecipe::Deserialize(json.to_string()).unwrap();
        
    }
}