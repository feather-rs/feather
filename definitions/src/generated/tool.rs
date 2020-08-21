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
impl crate::Item {
    pub fn durability(self) -> Option<u32> {
        match self {
            crate::Item::Bow => Some(384u32),
            crate::Item::CarrotOnAStick => Some(25u32),
            crate::Item::ChainmailBoots => Some(195u32),
            crate::Item::ChainmailChestplate => Some(240u32),
            crate::Item::ChainmailHelmet => Some(165u32),
            crate::Item::ChainmailLeggings => Some(225u32),
            crate::Item::DiamondAxe => Some(1561u32),
            crate::Item::DiamondBoots => Some(429u32),
            crate::Item::DiamondChestplate => Some(528u32),
            crate::Item::DiamondHelmet => Some(363u32),
            crate::Item::DiamondHoe => Some(1561u32),
            crate::Item::DiamondLeggings => Some(495u32),
            crate::Item::DiamondPickaxe => Some(1561u32),
            crate::Item::DiamondShovel => Some(1561u32),
            crate::Item::DiamondSword => Some(1561u32),
            crate::Item::Elytra => Some(432u32),
            crate::Item::FishingRod => Some(64u32),
            crate::Item::FlintAndSteel => Some(64u32),
            crate::Item::GoldenAxe => Some(32u32),
            crate::Item::GoldenBoots => Some(91u32),
            crate::Item::GoldenChestplate => Some(112u32),
            crate::Item::GoldenHelmet => Some(77u32),
            crate::Item::GoldenHoe => Some(32u32),
            crate::Item::GoldenLeggings => Some(105u32),
            crate::Item::GoldenPickaxe => Some(32u32),
            crate::Item::GoldenShovel => Some(32u32),
            crate::Item::GoldenSword => Some(32u32),
            crate::Item::IronAxe => Some(250u32),
            crate::Item::IronBoots => Some(195u32),
            crate::Item::IronChestplate => Some(240u32),
            crate::Item::IronHelmet => Some(165u32),
            crate::Item::IronHoe => Some(250u32),
            crate::Item::IronLeggings => Some(225u32),
            crate::Item::IronPickaxe => Some(250u32),
            crate::Item::IronShovel => Some(250u32),
            crate::Item::IronSword => Some(250u32),
            crate::Item::LeatherBoots => Some(65u32),
            crate::Item::LeatherChestplate => Some(80u32),
            crate::Item::LeatherHelmet => Some(55u32),
            crate::Item::LeatherLeggings => Some(75u32),
            crate::Item::Shears => Some(238u32),
            crate::Item::Shield => Some(336u32),
            crate::Item::StoneAxe => Some(131u32),
            crate::Item::StoneHoe => Some(131u32),
            crate::Item::StonePickaxe => Some(131u32),
            crate::Item::StoneShovel => Some(131u32),
            crate::Item::StoneSword => Some(131u32),
            crate::Item::Trident => Some(250u32),
            crate::Item::WoodenAxe => Some(59u32),
            crate::Item::WoodenHoe => Some(59u32),
            crate::Item::WoodenPickaxe => Some(59u32),
            crate::Item::WoodenShovel => Some(59u32),
            crate::Item::WoodenSword => Some(59u32),
            _ => None,
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
