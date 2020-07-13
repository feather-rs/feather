// This file is @generated
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ToPrimitive, FromPrimitive)]
pub enum Tool {
    Axe,
    Pickaxe,
    Shovel,
    Hoe,
    Sword,
    Shears,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ToPrimitive, FromPrimitive)]
pub enum ToolMaterial {
    Wooden,
    Stone,
    Iron,
    Diamond,
    Golden,
}
impl crate::Item {
    pub fn tool(self) -> Option<crate::Tool> {
        match self {
            crate::Item::DiamondAxe => Some(crate::Tool::Axe),
            crate::Item::DiamondHoe => Some(crate::Tool::Hoe),
            crate::Item::DiamondPickaxe => Some(crate::Tool::Pickaxe),
            crate::Item::DiamondShovel => Some(crate::Tool::Shovel),
            crate::Item::DiamondSword => Some(crate::Tool::Sword),
            crate::Item::GoldenAxe => Some(crate::Tool::Axe),
            crate::Item::GoldenHoe => Some(crate::Tool::Hoe),
            crate::Item::GoldenPickaxe => Some(crate::Tool::Pickaxe),
            crate::Item::GoldenShovel => Some(crate::Tool::Shovel),
            crate::Item::GoldenSword => Some(crate::Tool::Sword),
            crate::Item::IronAxe => Some(crate::Tool::Axe),
            crate::Item::IronHoe => Some(crate::Tool::Hoe),
            crate::Item::IronPickaxe => Some(crate::Tool::Pickaxe),
            crate::Item::IronShovel => Some(crate::Tool::Shovel),
            crate::Item::IronSword => Some(crate::Tool::Sword),
            crate::Item::Shears => Some(crate::Tool::Shears),
            crate::Item::StoneAxe => Some(crate::Tool::Axe),
            crate::Item::StoneHoe => Some(crate::Tool::Hoe),
            crate::Item::StonePickaxe => Some(crate::Tool::Pickaxe),
            crate::Item::StoneShovel => Some(crate::Tool::Shovel),
            crate::Item::StoneSword => Some(crate::Tool::Sword),
            crate::Item::WoodenAxe => Some(crate::Tool::Axe),
            crate::Item::WoodenHoe => Some(crate::Tool::Hoe),
            crate::Item::WoodenPickaxe => Some(crate::Tool::Pickaxe),
            crate::Item::WoodenShovel => Some(crate::Tool::Shovel),
            crate::Item::WoodenSword => Some(crate::Tool::Sword),
            _ => None,
        }
    }
}
impl crate::Item {
    pub fn tool_material(self) -> Option<crate::ToolMaterial> {
        match self {
            crate::Item::DiamondAxe => Some(crate::ToolMaterial::Diamond),
            crate::Item::DiamondHoe => Some(crate::ToolMaterial::Diamond),
            crate::Item::DiamondPickaxe => Some(crate::ToolMaterial::Diamond),
            crate::Item::DiamondShovel => Some(crate::ToolMaterial::Diamond),
            crate::Item::DiamondSword => Some(crate::ToolMaterial::Diamond),
            crate::Item::GoldenAxe => Some(crate::ToolMaterial::Golden),
            crate::Item::GoldenHoe => Some(crate::ToolMaterial::Golden),
            crate::Item::GoldenPickaxe => Some(crate::ToolMaterial::Golden),
            crate::Item::GoldenShovel => Some(crate::ToolMaterial::Golden),
            crate::Item::GoldenSword => Some(crate::ToolMaterial::Golden),
            crate::Item::IronAxe => Some(crate::ToolMaterial::Iron),
            crate::Item::IronHoe => Some(crate::ToolMaterial::Iron),
            crate::Item::IronPickaxe => Some(crate::ToolMaterial::Iron),
            crate::Item::IronShovel => Some(crate::ToolMaterial::Iron),
            crate::Item::IronSword => Some(crate::ToolMaterial::Iron),
            crate::Item::StoneAxe => Some(crate::ToolMaterial::Stone),
            crate::Item::StoneHoe => Some(crate::ToolMaterial::Stone),
            crate::Item::StonePickaxe => Some(crate::ToolMaterial::Stone),
            crate::Item::StoneShovel => Some(crate::ToolMaterial::Stone),
            crate::Item::StoneSword => Some(crate::ToolMaterial::Stone),
            crate::Item::WoodenAxe => Some(crate::ToolMaterial::Wooden),
            crate::Item::WoodenHoe => Some(crate::ToolMaterial::Wooden),
            crate::Item::WoodenPickaxe => Some(crate::ToolMaterial::Wooden),
            crate::Item::WoodenShovel => Some(crate::ToolMaterial::Wooden),
            crate::Item::WoodenSword => Some(crate::ToolMaterial::Wooden),
            _ => None,
        }
    }
}
impl crate::ToolMaterial {
    pub fn dig_multiplier(self) -> f64 {
        match self {
            crate::ToolMaterial::Diamond => 8f64,
            crate::ToolMaterial::Golden => 12f64,
            crate::ToolMaterial::Iron => 6f64,
            crate::ToolMaterial::Stone => 4f64,
            crate::ToolMaterial::Wooden => 2f64,
        }
    }
}
impl crate::BlockKind {
    pub fn best_tool(self) -> Option<crate::Tool> {
        match self {
            crate::BlockKind::Cobblestone => Some(crate::Tool::Pickaxe),
            crate::BlockKind::Dirt => Some(crate::Tool::Shovel),
            crate::BlockKind::GrassBlock => Some(crate::Tool::Shovel),
            crate::BlockKind::RedSand => Some(crate::Tool::Shovel),
            crate::BlockKind::Sand => Some(crate::Tool::Shovel),
            crate::BlockKind::Sandstone => Some(crate::Tool::Pickaxe),
            crate::BlockKind::Stone => Some(crate::Tool::Pickaxe),
            _ => None,
        }
    }
}
impl crate::BlockKind {
    pub fn best_tool_required(self) -> bool {
        match self {
            crate::BlockKind::Cobblestone => true,
            crate::BlockKind::Sandstone => true,
            crate::BlockKind::Stone => true,
            _ => false,
        }
    }
}
