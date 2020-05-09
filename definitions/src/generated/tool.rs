// This file is @generated
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ToPrimitive, FromPrimitive)]
pub enum HarvestTool {
    WoodenAxe,
    StoneAxe,
    IronAxe,
    DiamondAxe,
    GoldenAxe,
    WoodenPickaxe,
    StonePickaxe,
    IronPickaxe,
    DiamondPickaxe,
    GoldenPickaxe,
    WoodenShovel,
    StoneShovel,
    IronShovel,
    DiamondShovel,
    GoldenShovel,
    WoodenHoe,
    StoneHoe,
    IronHoe,
    DiamondHoe,
    GoldenHoe,
    WoodenSword,
    StoneSword,
    IronSword,
    DiamondSword,
    GoldenSword,
    WoodenShears,
    StoneShears,
    IronShears,
    DiamondShears,
    GoldenShears,
}
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
impl crate::HarvestTool {
    pub fn item(self) -> Option<crate::Item> {
        match self {
            crate::HarvestTool::DiamondAxe => Some(crate::Item::DiamondAxe),
            crate::HarvestTool::DiamondHoe => Some(crate::Item::DiamondHoe),
            crate::HarvestTool::DiamondPickaxe => Some(crate::Item::DiamondPickaxe),
            crate::HarvestTool::DiamondShovel => Some(crate::Item::DiamondShovel),
            crate::HarvestTool::DiamondSword => Some(crate::Item::DiamondSword),
            crate::HarvestTool::GoldenAxe => Some(crate::Item::GoldenAxe),
            crate::HarvestTool::GoldenHoe => Some(crate::Item::GoldenHoe),
            crate::HarvestTool::GoldenPickaxe => Some(crate::Item::GoldenPickaxe),
            crate::HarvestTool::GoldenShovel => Some(crate::Item::GoldenShovel),
            crate::HarvestTool::GoldenSword => Some(crate::Item::GoldenSword),
            crate::HarvestTool::IronAxe => Some(crate::Item::IronAxe),
            crate::HarvestTool::IronHoe => Some(crate::Item::IronHoe),
            crate::HarvestTool::IronPickaxe => Some(crate::Item::IronPickaxe),
            crate::HarvestTool::IronShovel => Some(crate::Item::IronShovel),
            crate::HarvestTool::IronSword => Some(crate::Item::IronSword),
            crate::HarvestTool::StoneAxe => Some(crate::Item::StoneAxe),
            crate::HarvestTool::StoneHoe => Some(crate::Item::StoneHoe),
            crate::HarvestTool::StonePickaxe => Some(crate::Item::StonePickaxe),
            crate::HarvestTool::StoneShovel => Some(crate::Item::StoneShovel),
            crate::HarvestTool::StoneSword => Some(crate::Item::StoneSword),
            crate::HarvestTool::WoodenAxe => Some(crate::Item::WoodenAxe),
            crate::HarvestTool::WoodenHoe => Some(crate::Item::WoodenHoe),
            crate::HarvestTool::WoodenPickaxe => Some(crate::Item::WoodenPickaxe),
            crate::HarvestTool::WoodenShovel => Some(crate::Item::WoodenShovel),
            crate::HarvestTool::WoodenSword => Some(crate::Item::WoodenSword),
            _ => None,
        }
    }
    pub fn from_item(prop: crate::Item) -> Option<HarvestTool> {
        match prop {
            crate::Item::DiamondAxe => Some(crate::HarvestTool::DiamondAxe),
            crate::Item::DiamondHoe => Some(crate::HarvestTool::DiamondHoe),
            crate::Item::DiamondPickaxe => Some(crate::HarvestTool::DiamondPickaxe),
            crate::Item::DiamondShovel => Some(crate::HarvestTool::DiamondShovel),
            crate::Item::DiamondSword => Some(crate::HarvestTool::DiamondSword),
            crate::Item::GoldenAxe => Some(crate::HarvestTool::GoldenAxe),
            crate::Item::GoldenHoe => Some(crate::HarvestTool::GoldenHoe),
            crate::Item::GoldenPickaxe => Some(crate::HarvestTool::GoldenPickaxe),
            crate::Item::GoldenShovel => Some(crate::HarvestTool::GoldenShovel),
            crate::Item::GoldenSword => Some(crate::HarvestTool::GoldenSword),
            crate::Item::IronAxe => Some(crate::HarvestTool::IronAxe),
            crate::Item::IronHoe => Some(crate::HarvestTool::IronHoe),
            crate::Item::IronPickaxe => Some(crate::HarvestTool::IronPickaxe),
            crate::Item::IronShovel => Some(crate::HarvestTool::IronShovel),
            crate::Item::IronSword => Some(crate::HarvestTool::IronSword),
            crate::Item::StoneAxe => Some(crate::HarvestTool::StoneAxe),
            crate::Item::StoneHoe => Some(crate::HarvestTool::StoneHoe),
            crate::Item::StonePickaxe => Some(crate::HarvestTool::StonePickaxe),
            crate::Item::StoneShovel => Some(crate::HarvestTool::StoneShovel),
            crate::Item::StoneSword => Some(crate::HarvestTool::StoneSword),
            crate::Item::WoodenAxe => Some(crate::HarvestTool::WoodenAxe),
            crate::Item::WoodenHoe => Some(crate::HarvestTool::WoodenHoe),
            crate::Item::WoodenPickaxe => Some(crate::HarvestTool::WoodenPickaxe),
            crate::Item::WoodenShovel => Some(crate::HarvestTool::WoodenShovel),
            crate::Item::WoodenSword => Some(crate::HarvestTool::WoodenSword),
            _ => None,
        }
    }
}
impl crate::HarvestTool {
    pub fn tool(self) -> crate::Tool {
        match self {
            crate::HarvestTool::DiamondAxe => crate::Tool::Axe,
            crate::HarvestTool::DiamondHoe => crate::Tool::Hoe,
            crate::HarvestTool::DiamondPickaxe => crate::Tool::Pickaxe,
            crate::HarvestTool::DiamondShears => crate::Tool::Shears,
            crate::HarvestTool::DiamondShovel => crate::Tool::Shovel,
            crate::HarvestTool::DiamondSword => crate::Tool::Sword,
            crate::HarvestTool::GoldenAxe => crate::Tool::Axe,
            crate::HarvestTool::GoldenHoe => crate::Tool::Hoe,
            crate::HarvestTool::GoldenPickaxe => crate::Tool::Pickaxe,
            crate::HarvestTool::GoldenShears => crate::Tool::Shears,
            crate::HarvestTool::GoldenShovel => crate::Tool::Shovel,
            crate::HarvestTool::GoldenSword => crate::Tool::Sword,
            crate::HarvestTool::IronAxe => crate::Tool::Axe,
            crate::HarvestTool::IronHoe => crate::Tool::Hoe,
            crate::HarvestTool::IronPickaxe => crate::Tool::Pickaxe,
            crate::HarvestTool::IronShears => crate::Tool::Shears,
            crate::HarvestTool::IronShovel => crate::Tool::Shovel,
            crate::HarvestTool::IronSword => crate::Tool::Sword,
            crate::HarvestTool::StoneAxe => crate::Tool::Axe,
            crate::HarvestTool::StoneHoe => crate::Tool::Hoe,
            crate::HarvestTool::StonePickaxe => crate::Tool::Pickaxe,
            crate::HarvestTool::StoneShears => crate::Tool::Shears,
            crate::HarvestTool::StoneShovel => crate::Tool::Shovel,
            crate::HarvestTool::StoneSword => crate::Tool::Sword,
            crate::HarvestTool::WoodenAxe => crate::Tool::Axe,
            crate::HarvestTool::WoodenHoe => crate::Tool::Hoe,
            crate::HarvestTool::WoodenPickaxe => crate::Tool::Pickaxe,
            crate::HarvestTool::WoodenShears => crate::Tool::Shears,
            crate::HarvestTool::WoodenShovel => crate::Tool::Shovel,
            crate::HarvestTool::WoodenSword => crate::Tool::Sword,
        }
    }
}
impl crate::HarvestTool {
    pub fn material(self) -> crate::ToolMaterial {
        match self {
            crate::HarvestTool::DiamondAxe => crate::ToolMaterial::Diamond,
            crate::HarvestTool::DiamondHoe => crate::ToolMaterial::Diamond,
            crate::HarvestTool::DiamondPickaxe => crate::ToolMaterial::Diamond,
            crate::HarvestTool::DiamondShears => crate::ToolMaterial::Diamond,
            crate::HarvestTool::DiamondShovel => crate::ToolMaterial::Diamond,
            crate::HarvestTool::DiamondSword => crate::ToolMaterial::Diamond,
            crate::HarvestTool::GoldenAxe => crate::ToolMaterial::Golden,
            crate::HarvestTool::GoldenHoe => crate::ToolMaterial::Golden,
            crate::HarvestTool::GoldenPickaxe => crate::ToolMaterial::Golden,
            crate::HarvestTool::GoldenShears => crate::ToolMaterial::Golden,
            crate::HarvestTool::GoldenShovel => crate::ToolMaterial::Golden,
            crate::HarvestTool::GoldenSword => crate::ToolMaterial::Golden,
            crate::HarvestTool::IronAxe => crate::ToolMaterial::Iron,
            crate::HarvestTool::IronHoe => crate::ToolMaterial::Iron,
            crate::HarvestTool::IronPickaxe => crate::ToolMaterial::Iron,
            crate::HarvestTool::IronShears => crate::ToolMaterial::Iron,
            crate::HarvestTool::IronShovel => crate::ToolMaterial::Iron,
            crate::HarvestTool::IronSword => crate::ToolMaterial::Iron,
            crate::HarvestTool::StoneAxe => crate::ToolMaterial::Stone,
            crate::HarvestTool::StoneHoe => crate::ToolMaterial::Stone,
            crate::HarvestTool::StonePickaxe => crate::ToolMaterial::Stone,
            crate::HarvestTool::StoneShears => crate::ToolMaterial::Stone,
            crate::HarvestTool::StoneShovel => crate::ToolMaterial::Stone,
            crate::HarvestTool::StoneSword => crate::ToolMaterial::Stone,
            crate::HarvestTool::WoodenAxe => crate::ToolMaterial::Wooden,
            crate::HarvestTool::WoodenHoe => crate::ToolMaterial::Wooden,
            crate::HarvestTool::WoodenPickaxe => crate::ToolMaterial::Wooden,
            crate::HarvestTool::WoodenShears => crate::ToolMaterial::Wooden,
            crate::HarvestTool::WoodenShovel => crate::ToolMaterial::Wooden,
            crate::HarvestTool::WoodenSword => crate::ToolMaterial::Wooden,
        }
    }
}
impl crate::BlockKind {
    pub fn harvest_tool(self) -> Option<crate::HarvestTool> {
        match self {
            crate::BlockKind::Cobblestone => Some(crate::HarvestTool::WoodenPickaxe),
            crate::BlockKind::Dirt => Some(crate::HarvestTool::WoodenShovel),
            crate::BlockKind::GrassBlock => Some(crate::HarvestTool::WoodenShovel),
            crate::BlockKind::Sand => Some(crate::HarvestTool::WoodenShovel),
            crate::BlockKind::Stone => Some(crate::HarvestTool::WoodenPickaxe),
            _ => None,
        }
    }
}
