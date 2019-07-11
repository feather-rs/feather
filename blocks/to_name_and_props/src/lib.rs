use feather_blocks_enum::*;
use heck::SnakeCase;
use std::collections::HashMap;

pub trait BlockFromNameAndProps {
    fn name(&self) -> &'static str;
    fn property_names(&self) -> HashMap<&'static str, String>;
}

impl BlockFromNameAndProps for Block {
    fn name(&self) -> &'static str {
        match self {
            Block::Air => "minecraft:air",
            Block::Stone => "minecraft:stone",
            Block::Granite => "minecraft:granite",
            Block::PolishedGranite => "minecraft:polished_granite",
            Block::Diorite => "minecraft:diorite",
            Block::PolishedDiorite => "minecraft:polished_diorite",
            Block::Andesite => "minecraft:andesite",
            Block::PolishedAndesite => "minecraft:polished_andesite",
            Block::GrassBlock(_) => "minecraft:grass_block",
            Block::Dirt => "minecraft:dirt",
            Block::CoarseDirt => "minecraft:coarse_dirt",
            Block::Podzol(_) => "minecraft:podzol",
            Block::Cobblestone => "minecraft:cobblestone",
            Block::OakPlanks => "minecraft:oak_planks",
            Block::SprucePlanks => "minecraft:spruce_planks",
            Block::BirchPlanks => "minecraft:birch_planks",
            Block::JunglePlanks => "minecraft:jungle_planks",
            Block::AcaciaPlanks => "minecraft:acacia_planks",
            Block::DarkOakPlanks => "minecraft:dark_oak_planks",
            Block::OakSapling(_) => "minecraft:oak_sapling",
            Block::SpruceSapling(_) => "minecraft:spruce_sapling",
            Block::BirchSapling(_) => "minecraft:birch_sapling",
            Block::JungleSapling(_) => "minecraft:jungle_sapling",
            Block::AcaciaSapling(_) => "minecraft:acacia_sapling",
            Block::DarkOakSapling(_) => "minecraft:dark_oak_sapling",
            Block::Bedrock => "minecraft:bedrock",
            Block::Water(_) => "minecraft:water",
            Block::Lava(_) => "minecraft:lava",
            Block::Sand => "minecraft:sand",
            Block::RedSand => "minecraft:red_sand",
            Block::Gravel => "minecraft:gravel",
            Block::GoldOre => "minecraft:gold_ore",
            Block::IronOre => "minecraft:iron_ore",
            Block::CoalOre => "minecraft:coal_ore",
            Block::OakLog(_) => "minecraft:oak_log",
            Block::SpruceLog(_) => "minecraft:spruce_log",
            Block::BirchLog(_) => "minecraft:birch_log",
            Block::JungleLog(_) => "minecraft:jungle_log",
            Block::AcaciaLog(_) => "minecraft:acacia_log",
            Block::DarkOakLog(_) => "minecraft:dark_oak_log",
            Block::StrippedSpruceLog(_) => "minecraft:stripped_spruce_log",
            Block::StrippedBirchLog(_) => "minecraft:stripped_birch_log",
            Block::StrippedJungleLog(_) => "minecraft:stripped_jungle_log",
            Block::StrippedAcaciaLog(_) => "minecraft:stripped_acacia_log",
            Block::StrippedDarkOakLog(_) => "minecraft:stripped_dark_oak_log",
            Block::StrippedOakLog(_) => "minecraft:stripped_oak_log",
            Block::OakWood(_) => "minecraft:oak_wood",
            Block::SpruceWood(_) => "minecraft:spruce_wood",
            Block::BirchWood(_) => "minecraft:birch_wood",
            Block::JungleWood(_) => "minecraft:jungle_wood",
            Block::AcaciaWood(_) => "minecraft:acacia_wood",
            Block::DarkOakWood(_) => "minecraft:dark_oak_wood",
            Block::StrippedOakWood(_) => "minecraft:stripped_oak_wood",
            Block::StrippedSpruceWood(_) => "minecraft:stripped_spruce_wood",
            Block::StrippedBirchWood(_) => "minecraft:stripped_birch_wood",
            Block::StrippedJungleWood(_) => "minecraft:stripped_jungle_wood",
            Block::StrippedAcaciaWood(_) => "minecraft:stripped_acacia_wood",
            Block::StrippedDarkOakWood(_) => "minecraft:stripped_dark_oak_wood",
            Block::OakLeaves(_) => "minecraft:oak_leaves",
            Block::SpruceLeaves(_) => "minecraft:spruce_leaves",
            Block::BirchLeaves(_) => "minecraft:birch_leaves",
            Block::JungleLeaves(_) => "minecraft:jungle_leaves",
            Block::AcaciaLeaves(_) => "minecraft:acacia_leaves",
            Block::DarkOakLeaves(_) => "minecraft:dark_oak_leaves",
            Block::Sponge => "minecraft:sponge",
            Block::WetSponge => "minecraft:wet_sponge",
            Block::Glass => "minecraft:glass",
            Block::LapisOre => "minecraft:lapis_ore",
            Block::LapisBlock => "minecraft:lapis_block",
            Block::Dispenser(_) => "minecraft:dispenser",
            Block::Sandstone => "minecraft:sandstone",
            Block::ChiseledSandstone => "minecraft:chiseled_sandstone",
            Block::CutSandstone => "minecraft:cut_sandstone",
            Block::NoteBlock(_) => "minecraft:note_block",
            Block::WhiteBed(_) => "minecraft:white_bed",
            Block::OrangeBed(_) => "minecraft:orange_bed",
            Block::MagentaBed(_) => "minecraft:magenta_bed",
            Block::LightBlueBed(_) => "minecraft:light_blue_bed",
            Block::YellowBed(_) => "minecraft:yellow_bed",
            Block::LimeBed(_) => "minecraft:lime_bed",
            Block::PinkBed(_) => "minecraft:pink_bed",
            Block::GrayBed(_) => "minecraft:gray_bed",
            Block::LightGrayBed(_) => "minecraft:light_gray_bed",
            Block::CyanBed(_) => "minecraft:cyan_bed",
            Block::PurpleBed(_) => "minecraft:purple_bed",
            Block::BlueBed(_) => "minecraft:blue_bed",
            Block::BrownBed(_) => "minecraft:brown_bed",
            Block::GreenBed(_) => "minecraft:green_bed",
            Block::RedBed(_) => "minecraft:red_bed",
            Block::BlackBed(_) => "minecraft:black_bed",
            Block::PoweredRail(_) => "minecraft:powered_rail",
            Block::DetectorRail(_) => "minecraft:detector_rail",
            Block::StickyPiston(_) => "minecraft:sticky_piston",
            Block::Cobweb => "minecraft:cobweb",
            Block::Grass => "minecraft:grass",
            Block::Fern => "minecraft:fern",
            Block::DeadBush => "minecraft:dead_bush",
            Block::Seagrass => "minecraft:seagrass",
            Block::TallSeagrass(_) => "minecraft:tall_seagrass",
            Block::Piston(_) => "minecraft:piston",
            Block::PistonHead(_) => "minecraft:piston_head",
            Block::WhiteWool => "minecraft:white_wool",
            Block::OrangeWool => "minecraft:orange_wool",
            Block::MagentaWool => "minecraft:magenta_wool",
            Block::LightBlueWool => "minecraft:light_blue_wool",
            Block::YellowWool => "minecraft:yellow_wool",
            Block::LimeWool => "minecraft:lime_wool",
            Block::PinkWool => "minecraft:pink_wool",
            Block::GrayWool => "minecraft:gray_wool",
            Block::LightGrayWool => "minecraft:light_gray_wool",
            Block::CyanWool => "minecraft:cyan_wool",
            Block::PurpleWool => "minecraft:purple_wool",
            Block::BlueWool => "minecraft:blue_wool",
            Block::BrownWool => "minecraft:brown_wool",
            Block::GreenWool => "minecraft:green_wool",
            Block::RedWool => "minecraft:red_wool",
            Block::BlackWool => "minecraft:black_wool",
            Block::MovingPiston(_) => "minecraft:moving_piston",
            Block::Dandelion => "minecraft:dandelion",
            Block::Poppy => "minecraft:poppy",
            Block::BlueOrchid => "minecraft:blue_orchid",
            Block::Allium => "minecraft:allium",
            Block::AzureBluet => "minecraft:azure_bluet",
            Block::RedTulip => "minecraft:red_tulip",
            Block::OrangeTulip => "minecraft:orange_tulip",
            Block::WhiteTulip => "minecraft:white_tulip",
            Block::PinkTulip => "minecraft:pink_tulip",
            Block::OxeyeDaisy => "minecraft:oxeye_daisy",
            Block::BrownMushroom => "minecraft:brown_mushroom",
            Block::RedMushroom => "minecraft:red_mushroom",
            Block::GoldBlock => "minecraft:gold_block",
            Block::IronBlock => "minecraft:iron_block",
            Block::Bricks => "minecraft:bricks",
            Block::Tnt(_) => "minecraft:tnt",
            Block::Bookshelf => "minecraft:bookshelf",
            Block::MossyCobblestone => "minecraft:mossy_cobblestone",
            Block::Obsidian => "minecraft:obsidian",
            Block::Torch => "minecraft:torch",
            Block::WallTorch(_) => "minecraft:wall_torch",
            Block::Fire(_) => "minecraft:fire",
            Block::Spawner => "minecraft:spawner",
            Block::OakStairs(_) => "minecraft:oak_stairs",
            Block::Chest(_) => "minecraft:chest",
            Block::RedstoneWire(_) => "minecraft:redstone_wire",
            Block::DiamondOre => "minecraft:diamond_ore",
            Block::DiamondBlock => "minecraft:diamond_block",
            Block::CraftingTable => "minecraft:crafting_table",
            Block::Wheat(_) => "minecraft:wheat",
            Block::Farmland(_) => "minecraft:farmland",
            Block::Furnace(_) => "minecraft:furnace",
            Block::Sign(_) => "minecraft:sign",
            Block::OakDoor(_) => "minecraft:oak_door",
            Block::Ladder(_) => "minecraft:ladder",
            Block::Rail(_) => "minecraft:rail",
            Block::CobblestoneStairs(_) => "minecraft:cobblestone_stairs",
            Block::WallSign(_) => "minecraft:wall_sign",
            Block::Lever(_) => "minecraft:lever",
            Block::StonePressurePlate(_) => "minecraft:stone_pressure_plate",
            Block::IronDoor(_) => "minecraft:iron_door",
            Block::OakPressurePlate(_) => "minecraft:oak_pressure_plate",
            Block::SprucePressurePlate(_) => "minecraft:spruce_pressure_plate",
            Block::BirchPressurePlate(_) => "minecraft:birch_pressure_plate",
            Block::JunglePressurePlate(_) => "minecraft:jungle_pressure_plate",
            Block::AcaciaPressurePlate(_) => "minecraft:acacia_pressure_plate",
            Block::DarkOakPressurePlate(_) => "minecraft:dark_oak_pressure_plate",
            Block::RedstoneOre(_) => "minecraft:redstone_ore",
            Block::RedstoneTorch(_) => "minecraft:redstone_torch",
            Block::RedstoneWallTorch(_) => "minecraft:redstone_wall_torch",
            Block::StoneButton(_) => "minecraft:stone_button",
            Block::Snow(_) => "minecraft:snow",
            Block::Ice => "minecraft:ice",
            Block::SnowBlock => "minecraft:snow_block",
            Block::Cactus(_) => "minecraft:cactus",
            Block::Clay => "minecraft:clay",
            Block::SugarCane(_) => "minecraft:sugar_cane",
            Block::Jukebox(_) => "minecraft:jukebox",
            Block::OakFence(_) => "minecraft:oak_fence",
            Block::Pumpkin => "minecraft:pumpkin",
            Block::Netherrack => "minecraft:netherrack",
            Block::SoulSand => "minecraft:soul_sand",
            Block::Glowstone => "minecraft:glowstone",
            Block::NetherPortal(_) => "minecraft:nether_portal",
            Block::CarvedPumpkin(_) => "minecraft:carved_pumpkin",
            Block::JackOLantern(_) => "minecraft:jack_o_lantern",
            Block::Cake(_) => "minecraft:cake",
            Block::Repeater(_) => "minecraft:repeater",
            Block::WhiteStainedGlass => "minecraft:white_stained_glass",
            Block::OrangeStainedGlass => "minecraft:orange_stained_glass",
            Block::MagentaStainedGlass => "minecraft:magenta_stained_glass",
            Block::LightBlueStainedGlass => "minecraft:light_blue_stained_glass",
            Block::YellowStainedGlass => "minecraft:yellow_stained_glass",
            Block::LimeStainedGlass => "minecraft:lime_stained_glass",
            Block::PinkStainedGlass => "minecraft:pink_stained_glass",
            Block::GrayStainedGlass => "minecraft:gray_stained_glass",
            Block::LightGrayStainedGlass => "minecraft:light_gray_stained_glass",
            Block::CyanStainedGlass => "minecraft:cyan_stained_glass",
            Block::PurpleStainedGlass => "minecraft:purple_stained_glass",
            Block::BlueStainedGlass => "minecraft:blue_stained_glass",
            Block::BrownStainedGlass => "minecraft:brown_stained_glass",
            Block::GreenStainedGlass => "minecraft:green_stained_glass",
            Block::RedStainedGlass => "minecraft:red_stained_glass",
            Block::BlackStainedGlass => "minecraft:black_stained_glass",
            Block::OakTrapdoor(_) => "minecraft:oak_trapdoor",
            Block::SpruceTrapdoor(_) => "minecraft:spruce_trapdoor",
            Block::BirchTrapdoor(_) => "minecraft:birch_trapdoor",
            Block::JungleTrapdoor(_) => "minecraft:jungle_trapdoor",
            Block::AcaciaTrapdoor(_) => "minecraft:acacia_trapdoor",
            Block::DarkOakTrapdoor(_) => "minecraft:dark_oak_trapdoor",
            Block::InfestedStone => "minecraft:infested_stone",
            Block::InfestedCobblestone => "minecraft:infested_cobblestone",
            Block::InfestedStoneBricks => "minecraft:infested_stone_bricks",
            Block::InfestedMossyStoneBricks => "minecraft:infested_mossy_stone_bricks",
            Block::InfestedCrackedStoneBricks => "minecraft:infested_cracked_stone_bricks",
            Block::InfestedChiseledStoneBricks => "minecraft:infested_chiseled_stone_bricks",
            Block::StoneBricks => "minecraft:stone_bricks",
            Block::MossyStoneBricks => "minecraft:mossy_stone_bricks",
            Block::CrackedStoneBricks => "minecraft:cracked_stone_bricks",
            Block::ChiseledStoneBricks => "minecraft:chiseled_stone_bricks",
            Block::BrownMushroomBlock(_) => "minecraft:brown_mushroom_block",
            Block::RedMushroomBlock(_) => "minecraft:red_mushroom_block",
            Block::MushroomStem(_) => "minecraft:mushroom_stem",
            Block::IronBars(_) => "minecraft:iron_bars",
            Block::GlassPane(_) => "minecraft:glass_pane",
            Block::Melon => "minecraft:melon",
            Block::AttachedPumpkinStem(_) => "minecraft:attached_pumpkin_stem",
            Block::AttachedMelonStem(_) => "minecraft:attached_melon_stem",
            Block::PumpkinStem(_) => "minecraft:pumpkin_stem",
            Block::MelonStem(_) => "minecraft:melon_stem",
            Block::Vine(_) => "minecraft:vine",
            Block::OakFenceGate(_) => "minecraft:oak_fence_gate",
            Block::BrickStairs(_) => "minecraft:brick_stairs",
            Block::StoneBrickStairs(_) => "minecraft:stone_brick_stairs",
            Block::Mycelium(_) => "minecraft:mycelium",
            Block::LilyPad => "minecraft:lily_pad",
            Block::NetherBricks => "minecraft:nether_bricks",
            Block::NetherBrickFence(_) => "minecraft:nether_brick_fence",
            Block::NetherBrickStairs(_) => "minecraft:nether_brick_stairs",
            Block::NetherWart(_) => "minecraft:nether_wart",
            Block::EnchantingTable => "minecraft:enchanting_table",
            Block::BrewingStand(_) => "minecraft:brewing_stand",
            Block::Cauldron(_) => "minecraft:cauldron",
            Block::EndPortal => "minecraft:end_portal",
            Block::EndPortalFrame(_) => "minecraft:end_portal_frame",
            Block::EndStone => "minecraft:end_stone",
            Block::DragonEgg => "minecraft:dragon_egg",
            Block::RedstoneLamp(_) => "minecraft:redstone_lamp",
            Block::Cocoa(_) => "minecraft:cocoa",
            Block::SandstoneStairs(_) => "minecraft:sandstone_stairs",
            Block::EmeraldOre => "minecraft:emerald_ore",
            Block::EnderChest(_) => "minecraft:ender_chest",
            Block::TripwireHook(_) => "minecraft:tripwire_hook",
            Block::Tripwire(_) => "minecraft:tripwire",
            Block::EmeraldBlock => "minecraft:emerald_block",
            Block::SpruceStairs(_) => "minecraft:spruce_stairs",
            Block::BirchStairs(_) => "minecraft:birch_stairs",
            Block::JungleStairs(_) => "minecraft:jungle_stairs",
            Block::CommandBlock(_) => "minecraft:command_block",
            Block::Beacon => "minecraft:beacon",
            Block::CobblestoneWall(_) => "minecraft:cobblestone_wall",
            Block::MossyCobblestoneWall(_) => "minecraft:mossy_cobblestone_wall",
            Block::FlowerPot => "minecraft:flower_pot",
            Block::PottedOakSapling => "minecraft:potted_oak_sapling",
            Block::PottedSpruceSapling => "minecraft:potted_spruce_sapling",
            Block::PottedBirchSapling => "minecraft:potted_birch_sapling",
            Block::PottedJungleSapling => "minecraft:potted_jungle_sapling",
            Block::PottedAcaciaSapling => "minecraft:potted_acacia_sapling",
            Block::PottedDarkOakSapling => "minecraft:potted_dark_oak_sapling",
            Block::PottedFern => "minecraft:potted_fern",
            Block::PottedDandelion => "minecraft:potted_dandelion",
            Block::PottedPoppy => "minecraft:potted_poppy",
            Block::PottedBlueOrchid => "minecraft:potted_blue_orchid",
            Block::PottedAllium => "minecraft:potted_allium",
            Block::PottedAzureBluet => "minecraft:potted_azure_bluet",
            Block::PottedRedTulip => "minecraft:potted_red_tulip",
            Block::PottedOrangeTulip => "minecraft:potted_orange_tulip",
            Block::PottedWhiteTulip => "minecraft:potted_white_tulip",
            Block::PottedPinkTulip => "minecraft:potted_pink_tulip",
            Block::PottedOxeyeDaisy => "minecraft:potted_oxeye_daisy",
            Block::PottedRedMushroom => "minecraft:potted_red_mushroom",
            Block::PottedBrownMushroom => "minecraft:potted_brown_mushroom",
            Block::PottedDeadBush => "minecraft:potted_dead_bush",
            Block::PottedCactus => "minecraft:potted_cactus",
            Block::Carrots(_) => "minecraft:carrots",
            Block::Potatoes(_) => "minecraft:potatoes",
            Block::OakButton(_) => "minecraft:oak_button",
            Block::SpruceButton(_) => "minecraft:spruce_button",
            Block::BirchButton(_) => "minecraft:birch_button",
            Block::JungleButton(_) => "minecraft:jungle_button",
            Block::AcaciaButton(_) => "minecraft:acacia_button",
            Block::DarkOakButton(_) => "minecraft:dark_oak_button",
            Block::SkeletonWallSkull(_) => "minecraft:skeleton_wall_skull",
            Block::SkeletonSkull(_) => "minecraft:skeleton_skull",
            Block::WitherSkeletonWallSkull(_) => "minecraft:wither_skeleton_wall_skull",
            Block::WitherSkeletonSkull(_) => "minecraft:wither_skeleton_skull",
            Block::ZombieWallHead(_) => "minecraft:zombie_wall_head",
            Block::ZombieHead(_) => "minecraft:zombie_head",
            Block::PlayerWallHead(_) => "minecraft:player_wall_head",
            Block::PlayerHead(_) => "minecraft:player_head",
            Block::CreeperWallHead(_) => "minecraft:creeper_wall_head",
            Block::CreeperHead(_) => "minecraft:creeper_head",
            Block::DragonWallHead(_) => "minecraft:dragon_wall_head",
            Block::DragonHead(_) => "minecraft:dragon_head",
            Block::Anvil(_) => "minecraft:anvil",
            Block::ChippedAnvil(_) => "minecraft:chipped_anvil",
            Block::DamagedAnvil(_) => "minecraft:damaged_anvil",
            Block::TrappedChest(_) => "minecraft:trapped_chest",
            Block::LightWeightedPressurePlate(_) => "minecraft:light_weighted_pressure_plate",
            Block::HeavyWeightedPressurePlate(_) => "minecraft:heavy_weighted_pressure_plate",
            Block::Comparator(_) => "minecraft:comparator",
            Block::DaylightDetector(_) => "minecraft:daylight_detector",
            Block::RedstoneBlock => "minecraft:redstone_block",
            Block::NetherQuartzOre => "minecraft:nether_quartz_ore",
            Block::Hopper(_) => "minecraft:hopper",
            Block::QuartzBlock => "minecraft:quartz_block",
            Block::ChiseledQuartzBlock => "minecraft:chiseled_quartz_block",
            Block::QuartzPillar(_) => "minecraft:quartz_pillar",
            Block::QuartzStairs(_) => "minecraft:quartz_stairs",
            Block::ActivatorRail(_) => "minecraft:activator_rail",
            Block::Dropper(_) => "minecraft:dropper",
            Block::WhiteTerracotta => "minecraft:white_terracotta",
            Block::OrangeTerracotta => "minecraft:orange_terracotta",
            Block::MagentaTerracotta => "minecraft:magenta_terracotta",
            Block::LightBlueTerracotta => "minecraft:light_blue_terracotta",
            Block::YellowTerracotta => "minecraft:yellow_terracotta",
            Block::LimeTerracotta => "minecraft:lime_terracotta",
            Block::PinkTerracotta => "minecraft:pink_terracotta",
            Block::GrayTerracotta => "minecraft:gray_terracotta",
            Block::LightGrayTerracotta => "minecraft:light_gray_terracotta",
            Block::CyanTerracotta => "minecraft:cyan_terracotta",
            Block::PurpleTerracotta => "minecraft:purple_terracotta",
            Block::BlueTerracotta => "minecraft:blue_terracotta",
            Block::BrownTerracotta => "minecraft:brown_terracotta",
            Block::GreenTerracotta => "minecraft:green_terracotta",
            Block::RedTerracotta => "minecraft:red_terracotta",
            Block::BlackTerracotta => "minecraft:black_terracotta",
            Block::WhiteStainedGlassPane(_) => "minecraft:white_stained_glass_pane",
            Block::OrangeStainedGlassPane(_) => "minecraft:orange_stained_glass_pane",
            Block::MagentaStainedGlassPane(_) => "minecraft:magenta_stained_glass_pane",
            Block::LightBlueStainedGlassPane(_) => "minecraft:light_blue_stained_glass_pane",
            Block::YellowStainedGlassPane(_) => "minecraft:yellow_stained_glass_pane",
            Block::LimeStainedGlassPane(_) => "minecraft:lime_stained_glass_pane",
            Block::PinkStainedGlassPane(_) => "minecraft:pink_stained_glass_pane",
            Block::GrayStainedGlassPane(_) => "minecraft:gray_stained_glass_pane",
            Block::LightGrayStainedGlassPane(_) => "minecraft:light_gray_stained_glass_pane",
            Block::CyanStainedGlassPane(_) => "minecraft:cyan_stained_glass_pane",
            Block::PurpleStainedGlassPane(_) => "minecraft:purple_stained_glass_pane",
            Block::BlueStainedGlassPane(_) => "minecraft:blue_stained_glass_pane",
            Block::BrownStainedGlassPane(_) => "minecraft:brown_stained_glass_pane",
            Block::GreenStainedGlassPane(_) => "minecraft:green_stained_glass_pane",
            Block::RedStainedGlassPane(_) => "minecraft:red_stained_glass_pane",
            Block::BlackStainedGlassPane(_) => "minecraft:black_stained_glass_pane",
            Block::AcaciaStairs(_) => "minecraft:acacia_stairs",
            Block::DarkOakStairs(_) => "minecraft:dark_oak_stairs",
            Block::SlimeBlock => "minecraft:slime_block",
            Block::Barrier => "minecraft:barrier",
            Block::IronTrapdoor(_) => "minecraft:iron_trapdoor",
            Block::Prismarine => "minecraft:prismarine",
            Block::PrismarineBricks => "minecraft:prismarine_bricks",
            Block::DarkPrismarine => "minecraft:dark_prismarine",
            Block::PrismarineStairs(_) => "minecraft:prismarine_stairs",
            Block::PrismarineBrickStairs(_) => "minecraft:prismarine_brick_stairs",
            Block::DarkPrismarineStairs(_) => "minecraft:dark_prismarine_stairs",
            Block::PrismarineSlab(_) => "minecraft:prismarine_slab",
            Block::PrismarineBrickSlab(_) => "minecraft:prismarine_brick_slab",
            Block::DarkPrismarineSlab(_) => "minecraft:dark_prismarine_slab",
            Block::SeaLantern => "minecraft:sea_lantern",
            Block::HayBlock(_) => "minecraft:hay_block",
            Block::WhiteCarpet => "minecraft:white_carpet",
            Block::OrangeCarpet => "minecraft:orange_carpet",
            Block::MagentaCarpet => "minecraft:magenta_carpet",
            Block::LightBlueCarpet => "minecraft:light_blue_carpet",
            Block::YellowCarpet => "minecraft:yellow_carpet",
            Block::LimeCarpet => "minecraft:lime_carpet",
            Block::PinkCarpet => "minecraft:pink_carpet",
            Block::GrayCarpet => "minecraft:gray_carpet",
            Block::LightGrayCarpet => "minecraft:light_gray_carpet",
            Block::CyanCarpet => "minecraft:cyan_carpet",
            Block::PurpleCarpet => "minecraft:purple_carpet",
            Block::BlueCarpet => "minecraft:blue_carpet",
            Block::BrownCarpet => "minecraft:brown_carpet",
            Block::GreenCarpet => "minecraft:green_carpet",
            Block::RedCarpet => "minecraft:red_carpet",
            Block::BlackCarpet => "minecraft:black_carpet",
            Block::Terracotta => "minecraft:terracotta",
            Block::CoalBlock => "minecraft:coal_block",
            Block::PackedIce => "minecraft:packed_ice",
            Block::Sunflower(_) => "minecraft:sunflower",
            Block::Lilac(_) => "minecraft:lilac",
            Block::RoseBush(_) => "minecraft:rose_bush",
            Block::Peony(_) => "minecraft:peony",
            Block::TallGrass(_) => "minecraft:tall_grass",
            Block::LargeFern(_) => "minecraft:large_fern",
            Block::WhiteBanner(_) => "minecraft:white_banner",
            Block::OrangeBanner(_) => "minecraft:orange_banner",
            Block::MagentaBanner(_) => "minecraft:magenta_banner",
            Block::LightBlueBanner(_) => "minecraft:light_blue_banner",
            Block::YellowBanner(_) => "minecraft:yellow_banner",
            Block::LimeBanner(_) => "minecraft:lime_banner",
            Block::PinkBanner(_) => "minecraft:pink_banner",
            Block::GrayBanner(_) => "minecraft:gray_banner",
            Block::LightGrayBanner(_) => "minecraft:light_gray_banner",
            Block::CyanBanner(_) => "minecraft:cyan_banner",
            Block::PurpleBanner(_) => "minecraft:purple_banner",
            Block::BlueBanner(_) => "minecraft:blue_banner",
            Block::BrownBanner(_) => "minecraft:brown_banner",
            Block::GreenBanner(_) => "minecraft:green_banner",
            Block::RedBanner(_) => "minecraft:red_banner",
            Block::BlackBanner(_) => "minecraft:black_banner",
            Block::WhiteWallBanner(_) => "minecraft:white_wall_banner",
            Block::OrangeWallBanner(_) => "minecraft:orange_wall_banner",
            Block::MagentaWallBanner(_) => "minecraft:magenta_wall_banner",
            Block::LightBlueWallBanner(_) => "minecraft:light_blue_wall_banner",
            Block::YellowWallBanner(_) => "minecraft:yellow_wall_banner",
            Block::LimeWallBanner(_) => "minecraft:lime_wall_banner",
            Block::PinkWallBanner(_) => "minecraft:pink_wall_banner",
            Block::GrayWallBanner(_) => "minecraft:gray_wall_banner",
            Block::LightGrayWallBanner(_) => "minecraft:light_gray_wall_banner",
            Block::CyanWallBanner(_) => "minecraft:cyan_wall_banner",
            Block::PurpleWallBanner(_) => "minecraft:purple_wall_banner",
            Block::BlueWallBanner(_) => "minecraft:blue_wall_banner",
            Block::BrownWallBanner(_) => "minecraft:brown_wall_banner",
            Block::GreenWallBanner(_) => "minecraft:green_wall_banner",
            Block::RedWallBanner(_) => "minecraft:red_wall_banner",
            Block::BlackWallBanner(_) => "minecraft:black_wall_banner",
            Block::RedSandstone => "minecraft:red_sandstone",
            Block::ChiseledRedSandstone => "minecraft:chiseled_red_sandstone",
            Block::CutRedSandstone => "minecraft:cut_red_sandstone",
            Block::RedSandstoneStairs(_) => "minecraft:red_sandstone_stairs",
            Block::OakSlab(_) => "minecraft:oak_slab",
            Block::SpruceSlab(_) => "minecraft:spruce_slab",
            Block::BirchSlab(_) => "minecraft:birch_slab",
            Block::JungleSlab(_) => "minecraft:jungle_slab",
            Block::AcaciaSlab(_) => "minecraft:acacia_slab",
            Block::DarkOakSlab(_) => "minecraft:dark_oak_slab",
            Block::StoneSlab(_) => "minecraft:stone_slab",
            Block::SandstoneSlab(_) => "minecraft:sandstone_slab",
            Block::PetrifiedOakSlab(_) => "minecraft:petrified_oak_slab",
            Block::CobblestoneSlab(_) => "minecraft:cobblestone_slab",
            Block::BrickSlab(_) => "minecraft:brick_slab",
            Block::StoneBrickSlab(_) => "minecraft:stone_brick_slab",
            Block::NetherBrickSlab(_) => "minecraft:nether_brick_slab",
            Block::QuartzSlab(_) => "minecraft:quartz_slab",
            Block::RedSandstoneSlab(_) => "minecraft:red_sandstone_slab",
            Block::PurpurSlab(_) => "minecraft:purpur_slab",
            Block::SmoothStone => "minecraft:smooth_stone",
            Block::SmoothSandstone => "minecraft:smooth_sandstone",
            Block::SmoothQuartz => "minecraft:smooth_quartz",
            Block::SmoothRedSandstone => "minecraft:smooth_red_sandstone",
            Block::SpruceFenceGate(_) => "minecraft:spruce_fence_gate",
            Block::BirchFenceGate(_) => "minecraft:birch_fence_gate",
            Block::JungleFenceGate(_) => "minecraft:jungle_fence_gate",
            Block::AcaciaFenceGate(_) => "minecraft:acacia_fence_gate",
            Block::DarkOakFenceGate(_) => "minecraft:dark_oak_fence_gate",
            Block::SpruceFence(_) => "minecraft:spruce_fence",
            Block::BirchFence(_) => "minecraft:birch_fence",
            Block::JungleFence(_) => "minecraft:jungle_fence",
            Block::AcaciaFence(_) => "minecraft:acacia_fence",
            Block::DarkOakFence(_) => "minecraft:dark_oak_fence",
            Block::SpruceDoor(_) => "minecraft:spruce_door",
            Block::BirchDoor(_) => "minecraft:birch_door",
            Block::JungleDoor(_) => "minecraft:jungle_door",
            Block::AcaciaDoor(_) => "minecraft:acacia_door",
            Block::DarkOakDoor(_) => "minecraft:dark_oak_door",
            Block::EndRod(_) => "minecraft:end_rod",
            Block::ChorusPlant(_) => "minecraft:chorus_plant",
            Block::ChorusFlower(_) => "minecraft:chorus_flower",
            Block::PurpurBlock => "minecraft:purpur_block",
            Block::PurpurPillar(_) => "minecraft:purpur_pillar",
            Block::PurpurStairs(_) => "minecraft:purpur_stairs",
            Block::EndStoneBricks => "minecraft:end_stone_bricks",
            Block::Beetroots(_) => "minecraft:beetroots",
            Block::GrassPath => "minecraft:grass_path",
            Block::EndGateway => "minecraft:end_gateway",
            Block::RepeatingCommandBlock(_) => "minecraft:repeating_command_block",
            Block::ChainCommandBlock(_) => "minecraft:chain_command_block",
            Block::FrostedIce(_) => "minecraft:frosted_ice",
            Block::MagmaBlock => "minecraft:magma_block",
            Block::NetherWartBlock => "minecraft:nether_wart_block",
            Block::RedNetherBricks => "minecraft:red_nether_bricks",
            Block::BoneBlock(_) => "minecraft:bone_block",
            Block::StructureVoid => "minecraft:structure_void",
            Block::Observer(_) => "minecraft:observer",
            Block::ShulkerBox(_) => "minecraft:shulker_box",
            Block::WhiteShulkerBox(_) => "minecraft:white_shulker_box",
            Block::OrangeShulkerBox(_) => "minecraft:orange_shulker_box",
            Block::MagentaShulkerBox(_) => "minecraft:magenta_shulker_box",
            Block::LightBlueShulkerBox(_) => "minecraft:light_blue_shulker_box",
            Block::YellowShulkerBox(_) => "minecraft:yellow_shulker_box",
            Block::LimeShulkerBox(_) => "minecraft:lime_shulker_box",
            Block::PinkShulkerBox(_) => "minecraft:pink_shulker_box",
            Block::GrayShulkerBox(_) => "minecraft:gray_shulker_box",
            Block::LightGrayShulkerBox(_) => "minecraft:light_gray_shulker_box",
            Block::CyanShulkerBox(_) => "minecraft:cyan_shulker_box",
            Block::PurpleShulkerBox(_) => "minecraft:purple_shulker_box",
            Block::BlueShulkerBox(_) => "minecraft:blue_shulker_box",
            Block::BrownShulkerBox(_) => "minecraft:brown_shulker_box",
            Block::GreenShulkerBox(_) => "minecraft:green_shulker_box",
            Block::RedShulkerBox(_) => "minecraft:red_shulker_box",
            Block::BlackShulkerBox(_) => "minecraft:black_shulker_box",
            Block::WhiteGlazedTerracotta(_) => "minecraft:white_glazed_terracotta",
            Block::OrangeGlazedTerracotta(_) => "minecraft:orange_glazed_terracotta",
            Block::MagentaGlazedTerracotta(_) => "minecraft:magenta_glazed_terracotta",
            Block::LightBlueGlazedTerracotta(_) => "minecraft:light_blue_glazed_terracotta",
            Block::YellowGlazedTerracotta(_) => "minecraft:yellow_glazed_terracotta",
            Block::LimeGlazedTerracotta(_) => "minecraft:lime_glazed_terracotta",
            Block::PinkGlazedTerracotta(_) => "minecraft:pink_glazed_terracotta",
            Block::GrayGlazedTerracotta(_) => "minecraft:gray_glazed_terracotta",
            Block::LightGrayGlazedTerracotta(_) => "minecraft:light_gray_glazed_terracotta",
            Block::CyanGlazedTerracotta(_) => "minecraft:cyan_glazed_terracotta",
            Block::PurpleGlazedTerracotta(_) => "minecraft:purple_glazed_terracotta",
            Block::BlueGlazedTerracotta(_) => "minecraft:blue_glazed_terracotta",
            Block::BrownGlazedTerracotta(_) => "minecraft:brown_glazed_terracotta",
            Block::GreenGlazedTerracotta(_) => "minecraft:green_glazed_terracotta",
            Block::RedGlazedTerracotta(_) => "minecraft:red_glazed_terracotta",
            Block::BlackGlazedTerracotta(_) => "minecraft:black_glazed_terracotta",
            Block::WhiteConcrete => "minecraft:white_concrete",
            Block::OrangeConcrete => "minecraft:orange_concrete",
            Block::MagentaConcrete => "minecraft:magenta_concrete",
            Block::LightBlueConcrete => "minecraft:light_blue_concrete",
            Block::YellowConcrete => "minecraft:yellow_concrete",
            Block::LimeConcrete => "minecraft:lime_concrete",
            Block::PinkConcrete => "minecraft:pink_concrete",
            Block::GrayConcrete => "minecraft:gray_concrete",
            Block::LightGrayConcrete => "minecraft:light_gray_concrete",
            Block::CyanConcrete => "minecraft:cyan_concrete",
            Block::PurpleConcrete => "minecraft:purple_concrete",
            Block::BlueConcrete => "minecraft:blue_concrete",
            Block::BrownConcrete => "minecraft:brown_concrete",
            Block::GreenConcrete => "minecraft:green_concrete",
            Block::RedConcrete => "minecraft:red_concrete",
            Block::BlackConcrete => "minecraft:black_concrete",
            Block::WhiteConcretePowder => "minecraft:white_concrete_powder",
            Block::OrangeConcretePowder => "minecraft:orange_concrete_powder",
            Block::MagentaConcretePowder => "minecraft:magenta_concrete_powder",
            Block::LightBlueConcretePowder => "minecraft:light_blue_concrete_powder",
            Block::YellowConcretePowder => "minecraft:yellow_concrete_powder",
            Block::LimeConcretePowder => "minecraft:lime_concrete_powder",
            Block::PinkConcretePowder => "minecraft:pink_concrete_powder",
            Block::GrayConcretePowder => "minecraft:gray_concrete_powder",
            Block::LightGrayConcretePowder => "minecraft:light_gray_concrete_powder",
            Block::CyanConcretePowder => "minecraft:cyan_concrete_powder",
            Block::PurpleConcretePowder => "minecraft:purple_concrete_powder",
            Block::BlueConcretePowder => "minecraft:blue_concrete_powder",
            Block::BrownConcretePowder => "minecraft:brown_concrete_powder",
            Block::GreenConcretePowder => "minecraft:green_concrete_powder",
            Block::RedConcretePowder => "minecraft:red_concrete_powder",
            Block::BlackConcretePowder => "minecraft:black_concrete_powder",
            Block::Kelp(_) => "minecraft:kelp",
            Block::KelpPlant => "minecraft:kelp_plant",
            Block::DriedKelpBlock => "minecraft:dried_kelp_block",
            Block::TurtleEgg(_) => "minecraft:turtle_egg",
            Block::DeadTubeCoralBlock => "minecraft:dead_tube_coral_block",
            Block::DeadBrainCoralBlock => "minecraft:dead_brain_coral_block",
            Block::DeadBubbleCoralBlock => "minecraft:dead_bubble_coral_block",
            Block::DeadFireCoralBlock => "minecraft:dead_fire_coral_block",
            Block::DeadHornCoralBlock => "minecraft:dead_horn_coral_block",
            Block::TubeCoralBlock => "minecraft:tube_coral_block",
            Block::BrainCoralBlock => "minecraft:brain_coral_block",
            Block::BubbleCoralBlock => "minecraft:bubble_coral_block",
            Block::FireCoralBlock => "minecraft:fire_coral_block",
            Block::HornCoralBlock => "minecraft:horn_coral_block",
            Block::DeadTubeCoral(_) => "minecraft:dead_tube_coral",
            Block::DeadBrainCoral(_) => "minecraft:dead_brain_coral",
            Block::DeadBubbleCoral(_) => "minecraft:dead_bubble_coral",
            Block::DeadFireCoral(_) => "minecraft:dead_fire_coral",
            Block::DeadHornCoral(_) => "minecraft:dead_horn_coral",
            Block::TubeCoral(_) => "minecraft:tube_coral",
            Block::BrainCoral(_) => "minecraft:brain_coral",
            Block::BubbleCoral(_) => "minecraft:bubble_coral",
            Block::FireCoral(_) => "minecraft:fire_coral",
            Block::HornCoral(_) => "minecraft:horn_coral",
            Block::DeadTubeCoralWallFan(_) => "minecraft:dead_tube_coral_wall_fan",
            Block::DeadBrainCoralWallFan(_) => "minecraft:dead_brain_coral_wall_fan",
            Block::DeadBubbleCoralWallFan(_) => "minecraft:dead_bubble_coral_wall_fan",
            Block::DeadFireCoralWallFan(_) => "minecraft:dead_fire_coral_wall_fan",
            Block::DeadHornCoralWallFan(_) => "minecraft:dead_horn_coral_wall_fan",
            Block::TubeCoralWallFan(_) => "minecraft:tube_coral_wall_fan",
            Block::BrainCoralWallFan(_) => "minecraft:brain_coral_wall_fan",
            Block::BubbleCoralWallFan(_) => "minecraft:bubble_coral_wall_fan",
            Block::FireCoralWallFan(_) => "minecraft:fire_coral_wall_fan",
            Block::HornCoralWallFan(_) => "minecraft:horn_coral_wall_fan",
            Block::DeadTubeCoralFan(_) => "minecraft:dead_tube_coral_fan",
            Block::DeadBrainCoralFan(_) => "minecraft:dead_brain_coral_fan",
            Block::DeadBubbleCoralFan(_) => "minecraft:dead_bubble_coral_fan",
            Block::DeadFireCoralFan(_) => "minecraft:dead_fire_coral_fan",
            Block::DeadHornCoralFan(_) => "minecraft:dead_horn_coral_fan",
            Block::TubeCoralFan(_) => "minecraft:tube_coral_fan",
            Block::BrainCoralFan(_) => "minecraft:brain_coral_fan",
            Block::BubbleCoralFan(_) => "minecraft:bubble_coral_fan",
            Block::FireCoralFan(_) => "minecraft:fire_coral_fan",
            Block::HornCoralFan(_) => "minecraft:horn_coral_fan",
            Block::SeaPickle(_) => "minecraft:sea_pickle",
            Block::BlueIce => "minecraft:blue_ice",
            Block::Conduit(_) => "minecraft:conduit",
            Block::VoidAir => "minecraft:void_air",
            Block::CaveAir => "minecraft:cave_air",
            Block::BubbleColumn(_) => "minecraft:bubble_column",
            Block::StructureBlock(_) => "minecraft:structure_block",
        }
    }

    fn property_names(&self) -> HashMap<&'static str, String> {
        let mut m = HashMap::new();
        match self {
            Block::GrassBlock(data) => {
                m.insert("snowy", format!("{:?}", data.snowy).to_snake_case());
            }
            Block::Podzol(data) => {
                m.insert("snowy", format!("{:?}", data.snowy).to_snake_case());
            }
            Block::OakSapling(data) => {
                m.insert("stage", format!("{:?}", data.stage).to_snake_case());
            }
            Block::SpruceSapling(data) => {
                m.insert("stage", format!("{:?}", data.stage).to_snake_case());
            }
            Block::BirchSapling(data) => {
                m.insert("stage", format!("{:?}", data.stage).to_snake_case());
            }
            Block::JungleSapling(data) => {
                m.insert("stage", format!("{:?}", data.stage).to_snake_case());
            }
            Block::AcaciaSapling(data) => {
                m.insert("stage", format!("{:?}", data.stage).to_snake_case());
            }
            Block::DarkOakSapling(data) => {
                m.insert("stage", format!("{:?}", data.stage).to_snake_case());
            }
            Block::Water(data) => {
                m.insert("level", format!("{:?}", data.level).to_snake_case());
            }
            Block::Lava(data) => {
                m.insert("level", format!("{:?}", data.level).to_snake_case());
            }
            Block::OakLog(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::SpruceLog(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::BirchLog(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::JungleLog(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::AcaciaLog(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::DarkOakLog(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::StrippedSpruceLog(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::StrippedBirchLog(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::StrippedJungleLog(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::StrippedAcaciaLog(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::StrippedDarkOakLog(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::StrippedOakLog(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::OakWood(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::SpruceWood(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::BirchWood(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::JungleWood(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::AcaciaWood(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::DarkOakWood(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::StrippedOakWood(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::StrippedSpruceWood(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::StrippedBirchWood(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::StrippedJungleWood(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::StrippedAcaciaWood(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::StrippedDarkOakWood(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::OakLeaves(data) => {
                m.insert("distance", format!("{:?}", data.distance).to_snake_case());
                m.insert(
                    "persistent",
                    format!("{:?}", data.persistent).to_snake_case(),
                );
            }
            Block::SpruceLeaves(data) => {
                m.insert("distance", format!("{:?}", data.distance).to_snake_case());
                m.insert(
                    "persistent",
                    format!("{:?}", data.persistent).to_snake_case(),
                );
            }
            Block::BirchLeaves(data) => {
                m.insert("distance", format!("{:?}", data.distance).to_snake_case());
                m.insert(
                    "persistent",
                    format!("{:?}", data.persistent).to_snake_case(),
                );
            }
            Block::JungleLeaves(data) => {
                m.insert("distance", format!("{:?}", data.distance).to_snake_case());
                m.insert(
                    "persistent",
                    format!("{:?}", data.persistent).to_snake_case(),
                );
            }
            Block::AcaciaLeaves(data) => {
                m.insert("distance", format!("{:?}", data.distance).to_snake_case());
                m.insert(
                    "persistent",
                    format!("{:?}", data.persistent).to_snake_case(),
                );
            }
            Block::DarkOakLeaves(data) => {
                m.insert("distance", format!("{:?}", data.distance).to_snake_case());
                m.insert(
                    "persistent",
                    format!("{:?}", data.persistent).to_snake_case(),
                );
            }
            Block::Dispenser(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("triggered", format!("{:?}", data.triggered).to_snake_case());
            }
            Block::NoteBlock(data) => {
                m.insert(
                    "instrument",
                    format!("{:?}", data.instrument).to_snake_case(),
                );
                m.insert("note", format!("{:?}", data.note).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::WhiteBed(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("occupied", format!("{:?}", data.occupied).to_snake_case());
                m.insert("part", format!("{:?}", data.part).to_snake_case());
            }
            Block::OrangeBed(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("occupied", format!("{:?}", data.occupied).to_snake_case());
                m.insert("part", format!("{:?}", data.part).to_snake_case());
            }
            Block::MagentaBed(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("occupied", format!("{:?}", data.occupied).to_snake_case());
                m.insert("part", format!("{:?}", data.part).to_snake_case());
            }
            Block::LightBlueBed(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("occupied", format!("{:?}", data.occupied).to_snake_case());
                m.insert("part", format!("{:?}", data.part).to_snake_case());
            }
            Block::YellowBed(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("occupied", format!("{:?}", data.occupied).to_snake_case());
                m.insert("part", format!("{:?}", data.part).to_snake_case());
            }
            Block::LimeBed(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("occupied", format!("{:?}", data.occupied).to_snake_case());
                m.insert("part", format!("{:?}", data.part).to_snake_case());
            }
            Block::PinkBed(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("occupied", format!("{:?}", data.occupied).to_snake_case());
                m.insert("part", format!("{:?}", data.part).to_snake_case());
            }
            Block::GrayBed(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("occupied", format!("{:?}", data.occupied).to_snake_case());
                m.insert("part", format!("{:?}", data.part).to_snake_case());
            }
            Block::LightGrayBed(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("occupied", format!("{:?}", data.occupied).to_snake_case());
                m.insert("part", format!("{:?}", data.part).to_snake_case());
            }
            Block::CyanBed(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("occupied", format!("{:?}", data.occupied).to_snake_case());
                m.insert("part", format!("{:?}", data.part).to_snake_case());
            }
            Block::PurpleBed(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("occupied", format!("{:?}", data.occupied).to_snake_case());
                m.insert("part", format!("{:?}", data.part).to_snake_case());
            }
            Block::BlueBed(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("occupied", format!("{:?}", data.occupied).to_snake_case());
                m.insert("part", format!("{:?}", data.part).to_snake_case());
            }
            Block::BrownBed(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("occupied", format!("{:?}", data.occupied).to_snake_case());
                m.insert("part", format!("{:?}", data.part).to_snake_case());
            }
            Block::GreenBed(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("occupied", format!("{:?}", data.occupied).to_snake_case());
                m.insert("part", format!("{:?}", data.part).to_snake_case());
            }
            Block::RedBed(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("occupied", format!("{:?}", data.occupied).to_snake_case());
                m.insert("part", format!("{:?}", data.part).to_snake_case());
            }
            Block::BlackBed(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("occupied", format!("{:?}", data.occupied).to_snake_case());
                m.insert("part", format!("{:?}", data.part).to_snake_case());
            }
            Block::PoweredRail(data) => {
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
            }
            Block::DetectorRail(data) => {
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
            }
            Block::StickyPiston(data) => {
                m.insert("extended", format!("{:?}", data.extended).to_snake_case());
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::TallSeagrass(data) => {
                m.insert("half", format!("{:?}", data.half).to_snake_case());
            }
            Block::Piston(data) => {
                m.insert("extended", format!("{:?}", data.extended).to_snake_case());
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::PistonHead(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("short", format!("{:?}", data.short).to_snake_case());
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
            }
            Block::MovingPiston(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
            }
            Block::Tnt(data) => {
                m.insert("unstable", format!("{:?}", data.unstable).to_snake_case());
            }
            Block::WallTorch(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::Fire(data) => {
                m.insert("age", format!("{:?}", data.age).to_snake_case());
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert("up", format!("{:?}", data.up).to_snake_case());
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::OakStairs(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::Chest(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::RedstoneWire(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("power", format!("{:?}", data.power).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::Wheat(data) => {
                m.insert("age", format!("{:?}", data.age).to_snake_case());
            }
            Block::Farmland(data) => {
                m.insert("moisture", format!("{:?}", data.moisture).to_snake_case());
            }
            Block::Furnace(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("lit", format!("{:?}", data.lit).to_snake_case());
            }
            Block::Sign(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::OakDoor(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("hinge", format!("{:?}", data.hinge).to_snake_case());
                m.insert("open", format!("{:?}", data.open).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::Ladder(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::Rail(data) => {
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
            }
            Block::CobblestoneStairs(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::WallSign(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::Lever(data) => {
                m.insert("face", format!("{:?}", data.face).to_snake_case());
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::StonePressurePlate(data) => {
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::IronDoor(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("hinge", format!("{:?}", data.hinge).to_snake_case());
                m.insert("open", format!("{:?}", data.open).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::OakPressurePlate(data) => {
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::SprucePressurePlate(data) => {
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::BirchPressurePlate(data) => {
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::JunglePressurePlate(data) => {
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::AcaciaPressurePlate(data) => {
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::DarkOakPressurePlate(data) => {
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::RedstoneOre(data) => {
                m.insert("lit", format!("{:?}", data.lit).to_snake_case());
            }
            Block::RedstoneTorch(data) => {
                m.insert("lit", format!("{:?}", data.lit).to_snake_case());
            }
            Block::RedstoneWallTorch(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("lit", format!("{:?}", data.lit).to_snake_case());
            }
            Block::StoneButton(data) => {
                m.insert("face", format!("{:?}", data.face).to_snake_case());
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::Snow(data) => {
                m.insert("layers", format!("{:?}", data.layers).to_snake_case());
            }
            Block::Cactus(data) => {
                m.insert("age", format!("{:?}", data.age).to_snake_case());
            }
            Block::SugarCane(data) => {
                m.insert("age", format!("{:?}", data.age).to_snake_case());
            }
            Block::Jukebox(data) => {
                m.insert(
                    "has_record",
                    format!("{:?}", data.has_record).to_snake_case(),
                );
            }
            Block::OakFence(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::NetherPortal(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::CarvedPumpkin(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::JackOLantern(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::Cake(data) => {
                m.insert("bites", format!("{:?}", data.bites).to_snake_case());
            }
            Block::Repeater(data) => {
                m.insert("delay", format!("{:?}", data.delay).to_snake_case());
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("locked", format!("{:?}", data.locked).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::OakTrapdoor(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("open", format!("{:?}", data.open).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::SpruceTrapdoor(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("open", format!("{:?}", data.open).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::BirchTrapdoor(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("open", format!("{:?}", data.open).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::JungleTrapdoor(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("open", format!("{:?}", data.open).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::AcaciaTrapdoor(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("open", format!("{:?}", data.open).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::DarkOakTrapdoor(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("open", format!("{:?}", data.open).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::BrownMushroomBlock(data) => {
                m.insert("down", format!("{:?}", data.down).to_snake_case());
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert("up", format!("{:?}", data.up).to_snake_case());
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::RedMushroomBlock(data) => {
                m.insert("down", format!("{:?}", data.down).to_snake_case());
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert("up", format!("{:?}", data.up).to_snake_case());
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::MushroomStem(data) => {
                m.insert("down", format!("{:?}", data.down).to_snake_case());
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert("up", format!("{:?}", data.up).to_snake_case());
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::IronBars(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::GlassPane(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::AttachedPumpkinStem(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::AttachedMelonStem(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::PumpkinStem(data) => {
                m.insert("age", format!("{:?}", data.age).to_snake_case());
            }
            Block::MelonStem(data) => {
                m.insert("age", format!("{:?}", data.age).to_snake_case());
            }
            Block::Vine(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert("up", format!("{:?}", data.up).to_snake_case());
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::OakFenceGate(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("in_wall", format!("{:?}", data.in_wall).to_snake_case());
                m.insert("open", format!("{:?}", data.open).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::BrickStairs(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::StoneBrickStairs(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::Mycelium(data) => {
                m.insert("snowy", format!("{:?}", data.snowy).to_snake_case());
            }
            Block::NetherBrickFence(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::NetherBrickStairs(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::NetherWart(data) => {
                m.insert("age", format!("{:?}", data.age).to_snake_case());
            }
            Block::BrewingStand(data) => {
                m.insert(
                    "has_bottle_0",
                    format!("{:?}", data.has_bottle_0).to_snake_case(),
                );
                m.insert(
                    "has_bottle_1",
                    format!("{:?}", data.has_bottle_1).to_snake_case(),
                );
                m.insert(
                    "has_bottle_2",
                    format!("{:?}", data.has_bottle_2).to_snake_case(),
                );
            }
            Block::Cauldron(data) => {
                m.insert("level", format!("{:?}", data.level).to_snake_case());
            }
            Block::EndPortalFrame(data) => {
                m.insert("eye", format!("{:?}", data.eye).to_snake_case());
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::RedstoneLamp(data) => {
                m.insert("lit", format!("{:?}", data.lit).to_snake_case());
            }
            Block::Cocoa(data) => {
                m.insert("age", format!("{:?}", data.age).to_snake_case());
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::SandstoneStairs(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::EnderChest(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::TripwireHook(data) => {
                m.insert("attached", format!("{:?}", data.attached).to_snake_case());
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::Tripwire(data) => {
                m.insert("attached", format!("{:?}", data.attached).to_snake_case());
                m.insert("disarmed", format!("{:?}", data.disarmed).to_snake_case());
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::SpruceStairs(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::BirchStairs(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::JungleStairs(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::CommandBlock(data) => {
                m.insert(
                    "conditional",
                    format!("{:?}", data.conditional).to_snake_case(),
                );
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::CobblestoneWall(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert("up", format!("{:?}", data.up).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::MossyCobblestoneWall(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert("up", format!("{:?}", data.up).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::Carrots(data) => {
                m.insert("age", format!("{:?}", data.age).to_snake_case());
            }
            Block::Potatoes(data) => {
                m.insert("age", format!("{:?}", data.age).to_snake_case());
            }
            Block::OakButton(data) => {
                m.insert("face", format!("{:?}", data.face).to_snake_case());
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::SpruceButton(data) => {
                m.insert("face", format!("{:?}", data.face).to_snake_case());
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::BirchButton(data) => {
                m.insert("face", format!("{:?}", data.face).to_snake_case());
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::JungleButton(data) => {
                m.insert("face", format!("{:?}", data.face).to_snake_case());
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::AcaciaButton(data) => {
                m.insert("face", format!("{:?}", data.face).to_snake_case());
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::DarkOakButton(data) => {
                m.insert("face", format!("{:?}", data.face).to_snake_case());
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::SkeletonWallSkull(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::SkeletonSkull(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::WitherSkeletonWallSkull(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::WitherSkeletonSkull(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::ZombieWallHead(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::ZombieHead(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::PlayerWallHead(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::PlayerHead(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::CreeperWallHead(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::CreeperHead(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::DragonWallHead(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::DragonHead(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::Anvil(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::ChippedAnvil(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::DamagedAnvil(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::TrappedChest(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::LightWeightedPressurePlate(data) => {
                m.insert("power", format!("{:?}", data.power).to_snake_case());
            }
            Block::HeavyWeightedPressurePlate(data) => {
                m.insert("power", format!("{:?}", data.power).to_snake_case());
            }
            Block::Comparator(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("mode", format!("{:?}", data.mode).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::DaylightDetector(data) => {
                m.insert("inverted", format!("{:?}", data.inverted).to_snake_case());
                m.insert("power", format!("{:?}", data.power).to_snake_case());
            }
            Block::Hopper(data) => {
                m.insert("enabled", format!("{:?}", data.enabled).to_snake_case());
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::QuartzPillar(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::QuartzStairs(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::ActivatorRail(data) => {
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
            }
            Block::Dropper(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("triggered", format!("{:?}", data.triggered).to_snake_case());
            }
            Block::WhiteStainedGlassPane(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::OrangeStainedGlassPane(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::MagentaStainedGlassPane(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::LightBlueStainedGlassPane(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::YellowStainedGlassPane(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::LimeStainedGlassPane(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::PinkStainedGlassPane(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::GrayStainedGlassPane(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::LightGrayStainedGlassPane(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::CyanStainedGlassPane(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::PurpleStainedGlassPane(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::BlueStainedGlassPane(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::BrownStainedGlassPane(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::GreenStainedGlassPane(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::RedStainedGlassPane(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::BlackStainedGlassPane(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::AcaciaStairs(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::DarkOakStairs(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::IronTrapdoor(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("open", format!("{:?}", data.open).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::PrismarineStairs(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::PrismarineBrickStairs(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::DarkPrismarineStairs(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::PrismarineSlab(data) => {
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::PrismarineBrickSlab(data) => {
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::DarkPrismarineSlab(data) => {
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::HayBlock(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::Sunflower(data) => {
                m.insert("half", format!("{:?}", data.half).to_snake_case());
            }
            Block::Lilac(data) => {
                m.insert("half", format!("{:?}", data.half).to_snake_case());
            }
            Block::RoseBush(data) => {
                m.insert("half", format!("{:?}", data.half).to_snake_case());
            }
            Block::Peony(data) => {
                m.insert("half", format!("{:?}", data.half).to_snake_case());
            }
            Block::TallGrass(data) => {
                m.insert("half", format!("{:?}", data.half).to_snake_case());
            }
            Block::LargeFern(data) => {
                m.insert("half", format!("{:?}", data.half).to_snake_case());
            }
            Block::WhiteBanner(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::OrangeBanner(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::MagentaBanner(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::LightBlueBanner(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::YellowBanner(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::LimeBanner(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::PinkBanner(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::GrayBanner(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::LightGrayBanner(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::CyanBanner(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::PurpleBanner(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::BlueBanner(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::BrownBanner(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::GreenBanner(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::RedBanner(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::BlackBanner(data) => {
                m.insert("rotation", format!("{:?}", data.rotation).to_snake_case());
            }
            Block::WhiteWallBanner(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::OrangeWallBanner(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::MagentaWallBanner(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::LightBlueWallBanner(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::YellowWallBanner(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::LimeWallBanner(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::PinkWallBanner(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::GrayWallBanner(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::LightGrayWallBanner(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::CyanWallBanner(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::PurpleWallBanner(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::BlueWallBanner(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::BrownWallBanner(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::GreenWallBanner(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::RedWallBanner(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::BlackWallBanner(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::RedSandstoneStairs(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::OakSlab(data) => {
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::SpruceSlab(data) => {
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::BirchSlab(data) => {
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::JungleSlab(data) => {
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::AcaciaSlab(data) => {
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::DarkOakSlab(data) => {
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::StoneSlab(data) => {
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::SandstoneSlab(data) => {
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::PetrifiedOakSlab(data) => {
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::CobblestoneSlab(data) => {
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::BrickSlab(data) => {
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::StoneBrickSlab(data) => {
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::NetherBrickSlab(data) => {
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::QuartzSlab(data) => {
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::RedSandstoneSlab(data) => {
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::PurpurSlab(data) => {
                m.insert("type_", format!("{:?}", data.type_).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::SpruceFenceGate(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("in_wall", format!("{:?}", data.in_wall).to_snake_case());
                m.insert("open", format!("{:?}", data.open).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::BirchFenceGate(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("in_wall", format!("{:?}", data.in_wall).to_snake_case());
                m.insert("open", format!("{:?}", data.open).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::JungleFenceGate(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("in_wall", format!("{:?}", data.in_wall).to_snake_case());
                m.insert("open", format!("{:?}", data.open).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::AcaciaFenceGate(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("in_wall", format!("{:?}", data.in_wall).to_snake_case());
                m.insert("open", format!("{:?}", data.open).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::DarkOakFenceGate(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("in_wall", format!("{:?}", data.in_wall).to_snake_case());
                m.insert("open", format!("{:?}", data.open).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::SpruceFence(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::BirchFence(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::JungleFence(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::AcaciaFence(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::DarkOakFence(data) => {
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::SpruceDoor(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("hinge", format!("{:?}", data.hinge).to_snake_case());
                m.insert("open", format!("{:?}", data.open).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::BirchDoor(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("hinge", format!("{:?}", data.hinge).to_snake_case());
                m.insert("open", format!("{:?}", data.open).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::JungleDoor(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("hinge", format!("{:?}", data.hinge).to_snake_case());
                m.insert("open", format!("{:?}", data.open).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::AcaciaDoor(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("hinge", format!("{:?}", data.hinge).to_snake_case());
                m.insert("open", format!("{:?}", data.open).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::DarkOakDoor(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("hinge", format!("{:?}", data.hinge).to_snake_case());
                m.insert("open", format!("{:?}", data.open).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::EndRod(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::ChorusPlant(data) => {
                m.insert("down", format!("{:?}", data.down).to_snake_case());
                m.insert("east", format!("{:?}", data.east).to_snake_case());
                m.insert("north", format!("{:?}", data.north).to_snake_case());
                m.insert("south", format!("{:?}", data.south).to_snake_case());
                m.insert("up", format!("{:?}", data.up).to_snake_case());
                m.insert("west", format!("{:?}", data.west).to_snake_case());
            }
            Block::ChorusFlower(data) => {
                m.insert("age", format!("{:?}", data.age).to_snake_case());
            }
            Block::PurpurPillar(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::PurpurStairs(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("half", format!("{:?}", data.half).to_snake_case());
                m.insert("shape", format!("{:?}", data.shape).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::Beetroots(data) => {
                m.insert("age", format!("{:?}", data.age).to_snake_case());
            }
            Block::RepeatingCommandBlock(data) => {
                m.insert(
                    "conditional",
                    format!("{:?}", data.conditional).to_snake_case(),
                );
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::ChainCommandBlock(data) => {
                m.insert(
                    "conditional",
                    format!("{:?}", data.conditional).to_snake_case(),
                );
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::FrostedIce(data) => {
                m.insert("age", format!("{:?}", data.age).to_snake_case());
            }
            Block::BoneBlock(data) => {
                m.insert("axis", format!("{:?}", data.axis).to_snake_case());
            }
            Block::Observer(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert("powered", format!("{:?}", data.powered).to_snake_case());
            }
            Block::ShulkerBox(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::WhiteShulkerBox(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::OrangeShulkerBox(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::MagentaShulkerBox(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::LightBlueShulkerBox(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::YellowShulkerBox(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::LimeShulkerBox(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::PinkShulkerBox(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::GrayShulkerBox(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::LightGrayShulkerBox(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::CyanShulkerBox(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::PurpleShulkerBox(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::BlueShulkerBox(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::BrownShulkerBox(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::GreenShulkerBox(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::RedShulkerBox(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::BlackShulkerBox(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::WhiteGlazedTerracotta(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::OrangeGlazedTerracotta(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::MagentaGlazedTerracotta(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::LightBlueGlazedTerracotta(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::YellowGlazedTerracotta(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::LimeGlazedTerracotta(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::PinkGlazedTerracotta(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::GrayGlazedTerracotta(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::LightGrayGlazedTerracotta(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::CyanGlazedTerracotta(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::PurpleGlazedTerracotta(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::BlueGlazedTerracotta(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::BrownGlazedTerracotta(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::GreenGlazedTerracotta(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::RedGlazedTerracotta(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::BlackGlazedTerracotta(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
            }
            Block::Kelp(data) => {
                m.insert("age", format!("{:?}", data.age).to_snake_case());
            }
            Block::TurtleEgg(data) => {
                m.insert("eggs", format!("{:?}", data.eggs).to_snake_case());
                m.insert("hatch", format!("{:?}", data.hatch).to_snake_case());
            }
            Block::DeadTubeCoral(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::DeadBrainCoral(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::DeadBubbleCoral(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::DeadFireCoral(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::DeadHornCoral(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::TubeCoral(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::BrainCoral(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::BubbleCoral(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::FireCoral(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::HornCoral(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::DeadTubeCoralWallFan(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::DeadBrainCoralWallFan(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::DeadBubbleCoralWallFan(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::DeadFireCoralWallFan(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::DeadHornCoralWallFan(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::TubeCoralWallFan(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::BrainCoralWallFan(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::BubbleCoralWallFan(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::FireCoralWallFan(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::HornCoralWallFan(data) => {
                m.insert("facing", format!("{:?}", data.facing).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::DeadTubeCoralFan(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::DeadBrainCoralFan(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::DeadBubbleCoralFan(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::DeadFireCoralFan(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::DeadHornCoralFan(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::TubeCoralFan(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::BrainCoralFan(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::BubbleCoralFan(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::FireCoralFan(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::HornCoralFan(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::SeaPickle(data) => {
                m.insert("pickles", format!("{:?}", data.pickles).to_snake_case());
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::Conduit(data) => {
                m.insert(
                    "waterlogged",
                    format!("{:?}", data.waterlogged).to_snake_case(),
                );
            }
            Block::BubbleColumn(data) => {
                m.insert("drag", format!("{:?}", data.drag).to_snake_case());
            }
            Block::StructureBlock(data) => {
                m.insert("mode", format!("{:?}", data.mode).to_snake_case());
            }
            _ => (),
        }
        m
    }
}
