use feather_blocks_enum::*;
use std::collections::HashMap;

pub trait BlockFromNameAndProps {
    fn from_name_and_props(name: &str, props: HashMap<String, String>) -> Option<Block>;
}

impl BlockFromNameAndProps for Block {
    fn from_name_and_props(name: &str, props: HashMap<String, String>) -> Option<Self> {
        match name {
            "minecraft:air" => Some(Block::Air),
            "minecraft:stone" => Some(Block::Stone),
            "minecraft:granite" => Some(Block::Granite),
            "minecraft:polished_granite" => Some(Block::PolishedGranite),
            "minecraft:diorite" => Some(Block::Diorite),
            "minecraft:polished_diorite" => Some(Block::PolishedDiorite),
            "minecraft:andesite" => Some(Block::Andesite),
            "minecraft:polished_andesite" => Some(Block::PolishedAndesite),
            "minecraft:grass_block" => {
                let data = GrassBlockData {
                    snowy: match props["snowy"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::GrassBlock(data))
            }
            "minecraft:dirt" => Some(Block::Dirt),
            "minecraft:coarse_dirt" => Some(Block::CoarseDirt),
            "minecraft:podzol" => {
                let data = PodzolData {
                    snowy: match props["snowy"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::Podzol(data))
            }
            "minecraft:cobblestone" => Some(Block::Cobblestone),
            "minecraft:oak_planks" => Some(Block::OakPlanks),
            "minecraft:spruce_planks" => Some(Block::SprucePlanks),
            "minecraft:birch_planks" => Some(Block::BirchPlanks),
            "minecraft:jungle_planks" => Some(Block::JunglePlanks),
            "minecraft:acacia_planks" => Some(Block::AcaciaPlanks),
            "minecraft:dark_oak_planks" => Some(Block::DarkOakPlanks),
            "minecraft:oak_sapling" => {
                let data = OakSaplingData {
                    stage: match props["stage"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        _ => return None,
                    },
                };
                Some(Block::OakSapling(data))
            }
            "minecraft:spruce_sapling" => {
                let data = SpruceSaplingData {
                    stage: match props["stage"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        _ => return None,
                    },
                };
                Some(Block::SpruceSapling(data))
            }
            "minecraft:birch_sapling" => {
                let data = BirchSaplingData {
                    stage: match props["stage"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        _ => return None,
                    },
                };
                Some(Block::BirchSapling(data))
            }
            "minecraft:jungle_sapling" => {
                let data = JungleSaplingData {
                    stage: match props["stage"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        _ => return None,
                    },
                };
                Some(Block::JungleSapling(data))
            }
            "minecraft:acacia_sapling" => {
                let data = AcaciaSaplingData {
                    stage: match props["stage"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        _ => return None,
                    },
                };
                Some(Block::AcaciaSapling(data))
            }
            "minecraft:dark_oak_sapling" => {
                let data = DarkOakSaplingData {
                    stage: match props["stage"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        _ => return None,
                    },
                };
                Some(Block::DarkOakSapling(data))
            }
            "minecraft:bedrock" => Some(Block::Bedrock),
            "minecraft:water" => {
                let data = WaterData {
                    level: match props["level"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::Water(data))
            }
            "minecraft:lava" => {
                let data = LavaData {
                    level: match props["level"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::Lava(data))
            }
            "minecraft:sand" => Some(Block::Sand),
            "minecraft:red_sand" => Some(Block::RedSand),
            "minecraft:gravel" => Some(Block::Gravel),
            "minecraft:gold_ore" => Some(Block::GoldOre),
            "minecraft:iron_ore" => Some(Block::IronOre),
            "minecraft:coal_ore" => Some(Block::CoalOre),
            "minecraft:oak_log" => {
                let data = OakLogData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::OakLog(data))
            }
            "minecraft:spruce_log" => {
                let data = SpruceLogData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::SpruceLog(data))
            }
            "minecraft:birch_log" => {
                let data = BirchLogData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::BirchLog(data))
            }
            "minecraft:jungle_log" => {
                let data = JungleLogData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::JungleLog(data))
            }
            "minecraft:acacia_log" => {
                let data = AcaciaLogData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::AcaciaLog(data))
            }
            "minecraft:dark_oak_log" => {
                let data = DarkOakLogData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::DarkOakLog(data))
            }
            "minecraft:stripped_spruce_log" => {
                let data = StrippedSpruceLogData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::StrippedSpruceLog(data))
            }
            "minecraft:stripped_birch_log" => {
                let data = StrippedBirchLogData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::StrippedBirchLog(data))
            }
            "minecraft:stripped_jungle_log" => {
                let data = StrippedJungleLogData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::StrippedJungleLog(data))
            }
            "minecraft:stripped_acacia_log" => {
                let data = StrippedAcaciaLogData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::StrippedAcaciaLog(data))
            }
            "minecraft:stripped_dark_oak_log" => {
                let data = StrippedDarkOakLogData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::StrippedDarkOakLog(data))
            }
            "minecraft:stripped_oak_log" => {
                let data = StrippedOakLogData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::StrippedOakLog(data))
            }
            "minecraft:oak_wood" => {
                let data = OakWoodData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::OakWood(data))
            }
            "minecraft:spruce_wood" => {
                let data = SpruceWoodData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::SpruceWood(data))
            }
            "minecraft:birch_wood" => {
                let data = BirchWoodData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::BirchWood(data))
            }
            "minecraft:jungle_wood" => {
                let data = JungleWoodData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::JungleWood(data))
            }
            "minecraft:acacia_wood" => {
                let data = AcaciaWoodData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::AcaciaWood(data))
            }
            "minecraft:dark_oak_wood" => {
                let data = DarkOakWoodData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::DarkOakWood(data))
            }
            "minecraft:stripped_oak_wood" => {
                let data = StrippedOakWoodData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::StrippedOakWood(data))
            }
            "minecraft:stripped_spruce_wood" => {
                let data = StrippedSpruceWoodData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::StrippedSpruceWood(data))
            }
            "minecraft:stripped_birch_wood" => {
                let data = StrippedBirchWoodData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::StrippedBirchWood(data))
            }
            "minecraft:stripped_jungle_wood" => {
                let data = StrippedJungleWoodData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::StrippedJungleWood(data))
            }
            "minecraft:stripped_acacia_wood" => {
                let data = StrippedAcaciaWoodData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::StrippedAcaciaWood(data))
            }
            "minecraft:stripped_dark_oak_wood" => {
                let data = StrippedDarkOakWoodData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::StrippedDarkOakWood(data))
            }
            "minecraft:oak_leaves" => {
                let data = OakLeavesData {
                    distance: match props["distance"].as_str() {
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        _ => return None,
                    },
                    persistent: match props["persistent"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::OakLeaves(data))
            }
            "minecraft:spruce_leaves" => {
                let data = SpruceLeavesData {
                    distance: match props["distance"].as_str() {
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        _ => return None,
                    },
                    persistent: match props["persistent"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::SpruceLeaves(data))
            }
            "minecraft:birch_leaves" => {
                let data = BirchLeavesData {
                    distance: match props["distance"].as_str() {
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        _ => return None,
                    },
                    persistent: match props["persistent"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BirchLeaves(data))
            }
            "minecraft:jungle_leaves" => {
                let data = JungleLeavesData {
                    distance: match props["distance"].as_str() {
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        _ => return None,
                    },
                    persistent: match props["persistent"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::JungleLeaves(data))
            }
            "minecraft:acacia_leaves" => {
                let data = AcaciaLeavesData {
                    distance: match props["distance"].as_str() {
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        _ => return None,
                    },
                    persistent: match props["persistent"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::AcaciaLeaves(data))
            }
            "minecraft:dark_oak_leaves" => {
                let data = DarkOakLeavesData {
                    distance: match props["distance"].as_str() {
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        _ => return None,
                    },
                    persistent: match props["persistent"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DarkOakLeaves(data))
            }
            "minecraft:sponge" => Some(Block::Sponge),
            "minecraft:wet_sponge" => Some(Block::WetSponge),
            "minecraft:glass" => Some(Block::Glass),
            "minecraft:lapis_ore" => Some(Block::LapisOre),
            "minecraft:lapis_block" => Some(Block::LapisBlock),
            "minecraft:dispenser" => {
                let data = DispenserData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                    triggered: match props["triggered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::Dispenser(data))
            }
            "minecraft:sandstone" => Some(Block::Sandstone),
            "minecraft:chiseled_sandstone" => Some(Block::ChiseledSandstone),
            "minecraft:cut_sandstone" => Some(Block::CutSandstone),
            "minecraft:note_block" => {
                let data = NoteBlockData {
                    instrument: match props["instrument"].as_str() {
                        "harp" => NoteBlockInstrument::Harp,
                        "basedrum" => NoteBlockInstrument::Basedrum,
                        "snare" => NoteBlockInstrument::Snare,
                        "hat" => NoteBlockInstrument::Hat,
                        "bass" => NoteBlockInstrument::Bass,
                        "flute" => NoteBlockInstrument::Flute,
                        "bell" => NoteBlockInstrument::Bell,
                        "guitar" => NoteBlockInstrument::Guitar,
                        "chime" => NoteBlockInstrument::Chime,
                        "xylophone" => NoteBlockInstrument::Xylophone,
                        _ => return None,
                    },
                    note: match props["note"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        "16" => 16,
                        "17" => 17,
                        "18" => 18,
                        "19" => 19,
                        "20" => 20,
                        "21" => 21,
                        "22" => 22,
                        "23" => 23,
                        "24" => 24,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::NoteBlock(data))
            }
            "minecraft:white_bed" => {
                let data = WhiteBedData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    occupied: match props["occupied"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    part: match props["part"].as_str() {
                        "head" => Part::Head,
                        "foot" => Part::Foot,
                        _ => return None,
                    },
                };
                Some(Block::WhiteBed(data))
            }
            "minecraft:orange_bed" => {
                let data = OrangeBedData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    occupied: match props["occupied"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    part: match props["part"].as_str() {
                        "head" => Part::Head,
                        "foot" => Part::Foot,
                        _ => return None,
                    },
                };
                Some(Block::OrangeBed(data))
            }
            "minecraft:magenta_bed" => {
                let data = MagentaBedData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    occupied: match props["occupied"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    part: match props["part"].as_str() {
                        "head" => Part::Head,
                        "foot" => Part::Foot,
                        _ => return None,
                    },
                };
                Some(Block::MagentaBed(data))
            }
            "minecraft:light_blue_bed" => {
                let data = LightBlueBedData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    occupied: match props["occupied"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    part: match props["part"].as_str() {
                        "head" => Part::Head,
                        "foot" => Part::Foot,
                        _ => return None,
                    },
                };
                Some(Block::LightBlueBed(data))
            }
            "minecraft:yellow_bed" => {
                let data = YellowBedData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    occupied: match props["occupied"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    part: match props["part"].as_str() {
                        "head" => Part::Head,
                        "foot" => Part::Foot,
                        _ => return None,
                    },
                };
                Some(Block::YellowBed(data))
            }
            "minecraft:lime_bed" => {
                let data = LimeBedData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    occupied: match props["occupied"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    part: match props["part"].as_str() {
                        "head" => Part::Head,
                        "foot" => Part::Foot,
                        _ => return None,
                    },
                };
                Some(Block::LimeBed(data))
            }
            "minecraft:pink_bed" => {
                let data = PinkBedData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    occupied: match props["occupied"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    part: match props["part"].as_str() {
                        "head" => Part::Head,
                        "foot" => Part::Foot,
                        _ => return None,
                    },
                };
                Some(Block::PinkBed(data))
            }
            "minecraft:gray_bed" => {
                let data = GrayBedData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    occupied: match props["occupied"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    part: match props["part"].as_str() {
                        "head" => Part::Head,
                        "foot" => Part::Foot,
                        _ => return None,
                    },
                };
                Some(Block::GrayBed(data))
            }
            "minecraft:light_gray_bed" => {
                let data = LightGrayBedData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    occupied: match props["occupied"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    part: match props["part"].as_str() {
                        "head" => Part::Head,
                        "foot" => Part::Foot,
                        _ => return None,
                    },
                };
                Some(Block::LightGrayBed(data))
            }
            "minecraft:cyan_bed" => {
                let data = CyanBedData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    occupied: match props["occupied"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    part: match props["part"].as_str() {
                        "head" => Part::Head,
                        "foot" => Part::Foot,
                        _ => return None,
                    },
                };
                Some(Block::CyanBed(data))
            }
            "minecraft:purple_bed" => {
                let data = PurpleBedData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    occupied: match props["occupied"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    part: match props["part"].as_str() {
                        "head" => Part::Head,
                        "foot" => Part::Foot,
                        _ => return None,
                    },
                };
                Some(Block::PurpleBed(data))
            }
            "minecraft:blue_bed" => {
                let data = BlueBedData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    occupied: match props["occupied"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    part: match props["part"].as_str() {
                        "head" => Part::Head,
                        "foot" => Part::Foot,
                        _ => return None,
                    },
                };
                Some(Block::BlueBed(data))
            }
            "minecraft:brown_bed" => {
                let data = BrownBedData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    occupied: match props["occupied"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    part: match props["part"].as_str() {
                        "head" => Part::Head,
                        "foot" => Part::Foot,
                        _ => return None,
                    },
                };
                Some(Block::BrownBed(data))
            }
            "minecraft:green_bed" => {
                let data = GreenBedData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    occupied: match props["occupied"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    part: match props["part"].as_str() {
                        "head" => Part::Head,
                        "foot" => Part::Foot,
                        _ => return None,
                    },
                };
                Some(Block::GreenBed(data))
            }
            "minecraft:red_bed" => {
                let data = RedBedData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    occupied: match props["occupied"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    part: match props["part"].as_str() {
                        "head" => Part::Head,
                        "foot" => Part::Foot,
                        _ => return None,
                    },
                };
                Some(Block::RedBed(data))
            }
            "minecraft:black_bed" => {
                let data = BlackBedData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    occupied: match props["occupied"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    part: match props["part"].as_str() {
                        "head" => Part::Head,
                        "foot" => Part::Foot,
                        _ => return None,
                    },
                };
                Some(Block::BlackBed(data))
            }
            "minecraft:powered_rail" => {
                let data = PoweredRailData {
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    shape: match props["shape"].as_str() {
                        "north_south" => Shape::NorthSouth,
                        "east_west" => Shape::EastWest,
                        "ascending_east" => Shape::AscendingEast,
                        "ascending_west" => Shape::AscendingWest,
                        "ascending_north" => Shape::AscendingNorth,
                        "ascending_south" => Shape::AscendingSouth,
                        _ => return None,
                    },
                };
                Some(Block::PoweredRail(data))
            }
            "minecraft:detector_rail" => {
                let data = DetectorRailData {
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    shape: match props["shape"].as_str() {
                        "north_south" => Shape::NorthSouth,
                        "east_west" => Shape::EastWest,
                        "ascending_east" => Shape::AscendingEast,
                        "ascending_west" => Shape::AscendingWest,
                        "ascending_north" => Shape::AscendingNorth,
                        "ascending_south" => Shape::AscendingSouth,
                        _ => return None,
                    },
                };
                Some(Block::DetectorRail(data))
            }
            "minecraft:sticky_piston" => {
                let data = StickyPistonData {
                    extended: match props["extended"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::StickyPiston(data))
            }
            "minecraft:cobweb" => Some(Block::Cobweb),
            "minecraft:grass" => Some(Block::Grass),
            "minecraft:fern" => Some(Block::Fern),
            "minecraft:dead_bush" => Some(Block::DeadBush),
            "minecraft:seagrass" => Some(Block::Seagrass),
            "minecraft:tall_seagrass" => {
                let data = TallSeagrassData {
                    half: match props["half"].as_str() {
                        "upper" => Half::Upper,
                        "lower" => Half::Lower,
                        _ => return None,
                    },
                };
                Some(Block::TallSeagrass(data))
            }
            "minecraft:piston" => {
                let data = PistonData {
                    extended: match props["extended"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::Piston(data))
            }
            "minecraft:piston_head" => {
                let data = PistonHeadData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                    short: match props["short"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    type_: match props["type"].as_str() {
                        "normal" => PistonHeadType::Normal,
                        "sticky" => PistonHeadType::Sticky,
                        _ => return None,
                    },
                };
                Some(Block::PistonHead(data))
            }
            "minecraft:white_wool" => Some(Block::WhiteWool),
            "minecraft:orange_wool" => Some(Block::OrangeWool),
            "minecraft:magenta_wool" => Some(Block::MagentaWool),
            "minecraft:light_blue_wool" => Some(Block::LightBlueWool),
            "minecraft:yellow_wool" => Some(Block::YellowWool),
            "minecraft:lime_wool" => Some(Block::LimeWool),
            "minecraft:pink_wool" => Some(Block::PinkWool),
            "minecraft:gray_wool" => Some(Block::GrayWool),
            "minecraft:light_gray_wool" => Some(Block::LightGrayWool),
            "minecraft:cyan_wool" => Some(Block::CyanWool),
            "minecraft:purple_wool" => Some(Block::PurpleWool),
            "minecraft:blue_wool" => Some(Block::BlueWool),
            "minecraft:brown_wool" => Some(Block::BrownWool),
            "minecraft:green_wool" => Some(Block::GreenWool),
            "minecraft:red_wool" => Some(Block::RedWool),
            "minecraft:black_wool" => Some(Block::BlackWool),
            "minecraft:moving_piston" => {
                let data = MovingPistonData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                    type_: match props["type"].as_str() {
                        "normal" => MovingPistonType::Normal,
                        "sticky" => MovingPistonType::Sticky,
                        _ => return None,
                    },
                };
                Some(Block::MovingPiston(data))
            }
            "minecraft:dandelion" => Some(Block::Dandelion),
            "minecraft:poppy" => Some(Block::Poppy),
            "minecraft:blue_orchid" => Some(Block::BlueOrchid),
            "minecraft:allium" => Some(Block::Allium),
            "minecraft:azure_bluet" => Some(Block::AzureBluet),
            "minecraft:red_tulip" => Some(Block::RedTulip),
            "minecraft:orange_tulip" => Some(Block::OrangeTulip),
            "minecraft:white_tulip" => Some(Block::WhiteTulip),
            "minecraft:pink_tulip" => Some(Block::PinkTulip),
            "minecraft:oxeye_daisy" => Some(Block::OxeyeDaisy),
            "minecraft:brown_mushroom" => Some(Block::BrownMushroom),
            "minecraft:red_mushroom" => Some(Block::RedMushroom),
            "minecraft:gold_block" => Some(Block::GoldBlock),
            "minecraft:iron_block" => Some(Block::IronBlock),
            "minecraft:bricks" => Some(Block::Bricks),
            "minecraft:tnt" => {
                let data = TntData {
                    unstable: match props["unstable"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::Tnt(data))
            }
            "minecraft:bookshelf" => Some(Block::Bookshelf),
            "minecraft:mossy_cobblestone" => Some(Block::MossyCobblestone),
            "minecraft:obsidian" => Some(Block::Obsidian),
            "minecraft:torch" => Some(Block::Torch),
            "minecraft:wall_torch" => {
                let data = WallTorchData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::WallTorch(data))
            }
            "minecraft:fire" => {
                let data = FireData {
                    age: match props["age"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    up: match props["up"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::Fire(data))
            }
            "minecraft:spawner" => Some(Block::Spawner),
            "minecraft:oak_stairs" => {
                let data = OakStairsData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    shape: match props["shape"].as_str() {
                        "straight" => Shape::Straight,
                        "inner_left" => Shape::InnerLeft,
                        "inner_right" => Shape::InnerRight,
                        "outer_left" => Shape::OuterLeft,
                        "outer_right" => Shape::OuterRight,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::OakStairs(data))
            }
            "minecraft:chest" => {
                let data = ChestData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    type_: match props["type"].as_str() {
                        "single" => ChestType::Single,
                        "left" => ChestType::Left,
                        "right" => ChestType::Right,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::Chest(data))
            }
            "minecraft:redstone_wire" => {
                let data = RedstoneWireData {
                    east: match props["east"].as_str() {
                        "up" => RedstoneWireEast::Up,
                        "side" => RedstoneWireEast::Side,
                        "none" => RedstoneWireEast::None,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "up" => RedstoneWireNorth::Up,
                        "side" => RedstoneWireNorth::Side,
                        "none" => RedstoneWireNorth::None,
                        _ => return None,
                    },
                    power: match props["power"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "up" => RedstoneWireSouth::Up,
                        "side" => RedstoneWireSouth::Side,
                        "none" => RedstoneWireSouth::None,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "up" => RedstoneWireWest::Up,
                        "side" => RedstoneWireWest::Side,
                        "none" => RedstoneWireWest::None,
                        _ => return None,
                    },
                };
                Some(Block::RedstoneWire(data))
            }
            "minecraft:diamond_ore" => Some(Block::DiamondOre),
            "minecraft:diamond_block" => Some(Block::DiamondBlock),
            "minecraft:crafting_table" => Some(Block::CraftingTable),
            "minecraft:wheat" => {
                let data = WheatData {
                    age: match props["age"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        _ => return None,
                    },
                };
                Some(Block::Wheat(data))
            }
            "minecraft:farmland" => {
                let data = FarmlandData {
                    moisture: match props["moisture"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        _ => return None,
                    },
                };
                Some(Block::Farmland(data))
            }
            "minecraft:furnace" => {
                let data = FurnaceData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    lit: match props["lit"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::Furnace(data))
            }
            "minecraft:sign" => {
                let data = SignData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::Sign(data))
            }
            "minecraft:oak_door" => {
                let data = OakDoorData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "upper" => Half::Upper,
                        "lower" => Half::Lower,
                        _ => return None,
                    },
                    hinge: match props["hinge"].as_str() {
                        "left" => Hinge::Left,
                        "right" => Hinge::Right,
                        _ => return None,
                    },
                    open: match props["open"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::OakDoor(data))
            }
            "minecraft:ladder" => {
                let data = LadderData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::Ladder(data))
            }
            "minecraft:rail" => {
                let data = RailData {
                    shape: match props["shape"].as_str() {
                        "north_south" => Shape::NorthSouth,
                        "east_west" => Shape::EastWest,
                        "ascending_east" => Shape::AscendingEast,
                        "ascending_west" => Shape::AscendingWest,
                        "ascending_north" => Shape::AscendingNorth,
                        "ascending_south" => Shape::AscendingSouth,
                        "south_east" => Shape::SouthEast,
                        "south_west" => Shape::SouthWest,
                        "north_west" => Shape::NorthWest,
                        "north_east" => Shape::NorthEast,
                        _ => return None,
                    },
                };
                Some(Block::Rail(data))
            }
            "minecraft:cobblestone_stairs" => {
                let data = CobblestoneStairsData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    shape: match props["shape"].as_str() {
                        "straight" => Shape::Straight,
                        "inner_left" => Shape::InnerLeft,
                        "inner_right" => Shape::InnerRight,
                        "outer_left" => Shape::OuterLeft,
                        "outer_right" => Shape::OuterRight,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::CobblestoneStairs(data))
            }
            "minecraft:wall_sign" => {
                let data = WallSignData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::WallSign(data))
            }
            "minecraft:lever" => {
                let data = LeverData {
                    face: match props["face"].as_str() {
                        "floor" => Face::Floor,
                        "wall" => Face::Wall,
                        "ceiling" => Face::Ceiling,
                        _ => return None,
                    },
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::Lever(data))
            }
            "minecraft:stone_pressure_plate" => {
                let data = StonePressurePlateData {
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::StonePressurePlate(data))
            }
            "minecraft:iron_door" => {
                let data = IronDoorData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "upper" => Half::Upper,
                        "lower" => Half::Lower,
                        _ => return None,
                    },
                    hinge: match props["hinge"].as_str() {
                        "left" => Hinge::Left,
                        "right" => Hinge::Right,
                        _ => return None,
                    },
                    open: match props["open"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::IronDoor(data))
            }
            "minecraft:oak_pressure_plate" => {
                let data = OakPressurePlateData {
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::OakPressurePlate(data))
            }
            "minecraft:spruce_pressure_plate" => {
                let data = SprucePressurePlateData {
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::SprucePressurePlate(data))
            }
            "minecraft:birch_pressure_plate" => {
                let data = BirchPressurePlateData {
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BirchPressurePlate(data))
            }
            "minecraft:jungle_pressure_plate" => {
                let data = JunglePressurePlateData {
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::JunglePressurePlate(data))
            }
            "minecraft:acacia_pressure_plate" => {
                let data = AcaciaPressurePlateData {
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::AcaciaPressurePlate(data))
            }
            "minecraft:dark_oak_pressure_plate" => {
                let data = DarkOakPressurePlateData {
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DarkOakPressurePlate(data))
            }
            "minecraft:redstone_ore" => {
                let data = RedstoneOreData {
                    lit: match props["lit"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::RedstoneOre(data))
            }
            "minecraft:redstone_torch" => {
                let data = RedstoneTorchData {
                    lit: match props["lit"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::RedstoneTorch(data))
            }
            "minecraft:redstone_wall_torch" => {
                let data = RedstoneWallTorchData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    lit: match props["lit"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::RedstoneWallTorch(data))
            }
            "minecraft:stone_button" => {
                let data = StoneButtonData {
                    face: match props["face"].as_str() {
                        "floor" => Face::Floor,
                        "wall" => Face::Wall,
                        "ceiling" => Face::Ceiling,
                        _ => return None,
                    },
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::StoneButton(data))
            }
            "minecraft:snow" => {
                let data = SnowData {
                    layers: match props["layers"].as_str() {
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        _ => return None,
                    },
                };
                Some(Block::Snow(data))
            }
            "minecraft:ice" => Some(Block::Ice),
            "minecraft:snow_block" => Some(Block::SnowBlock),
            "minecraft:cactus" => {
                let data = CactusData {
                    age: match props["age"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::Cactus(data))
            }
            "minecraft:clay" => Some(Block::Clay),
            "minecraft:sugar_cane" => {
                let data = SugarCaneData {
                    age: match props["age"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::SugarCane(data))
            }
            "minecraft:jukebox" => {
                let data = JukeboxData {
                    has_record: match props["has_record"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::Jukebox(data))
            }
            "minecraft:oak_fence" => {
                let data = OakFenceData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::OakFence(data))
            }
            "minecraft:pumpkin" => Some(Block::Pumpkin),
            "minecraft:netherrack" => Some(Block::Netherrack),
            "minecraft:soul_sand" => Some(Block::SoulSand),
            "minecraft:glowstone" => Some(Block::Glowstone),
            "minecraft:nether_portal" => {
                let data = NetherPortalData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::NetherPortal(data))
            }
            "minecraft:carved_pumpkin" => {
                let data = CarvedPumpkinData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::CarvedPumpkin(data))
            }
            "minecraft:jack_o_lantern" => {
                let data = JackOLanternData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::JackOLantern(data))
            }
            "minecraft:cake" => {
                let data = CakeData {
                    bites: match props["bites"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        _ => return None,
                    },
                };
                Some(Block::Cake(data))
            }
            "minecraft:repeater" => {
                let data = RepeaterData {
                    delay: match props["delay"].as_str() {
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        _ => return None,
                    },
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    locked: match props["locked"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::Repeater(data))
            }
            "minecraft:white_stained_glass" => Some(Block::WhiteStainedGlass),
            "minecraft:orange_stained_glass" => Some(Block::OrangeStainedGlass),
            "minecraft:magenta_stained_glass" => Some(Block::MagentaStainedGlass),
            "minecraft:light_blue_stained_glass" => Some(Block::LightBlueStainedGlass),
            "minecraft:yellow_stained_glass" => Some(Block::YellowStainedGlass),
            "minecraft:lime_stained_glass" => Some(Block::LimeStainedGlass),
            "minecraft:pink_stained_glass" => Some(Block::PinkStainedGlass),
            "minecraft:gray_stained_glass" => Some(Block::GrayStainedGlass),
            "minecraft:light_gray_stained_glass" => Some(Block::LightGrayStainedGlass),
            "minecraft:cyan_stained_glass" => Some(Block::CyanStainedGlass),
            "minecraft:purple_stained_glass" => Some(Block::PurpleStainedGlass),
            "minecraft:blue_stained_glass" => Some(Block::BlueStainedGlass),
            "minecraft:brown_stained_glass" => Some(Block::BrownStainedGlass),
            "minecraft:green_stained_glass" => Some(Block::GreenStainedGlass),
            "minecraft:red_stained_glass" => Some(Block::RedStainedGlass),
            "minecraft:black_stained_glass" => Some(Block::BlackStainedGlass),
            "minecraft:oak_trapdoor" => {
                let data = OakTrapdoorData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    open: match props["open"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::OakTrapdoor(data))
            }
            "minecraft:spruce_trapdoor" => {
                let data = SpruceTrapdoorData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    open: match props["open"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::SpruceTrapdoor(data))
            }
            "minecraft:birch_trapdoor" => {
                let data = BirchTrapdoorData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    open: match props["open"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BirchTrapdoor(data))
            }
            "minecraft:jungle_trapdoor" => {
                let data = JungleTrapdoorData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    open: match props["open"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::JungleTrapdoor(data))
            }
            "minecraft:acacia_trapdoor" => {
                let data = AcaciaTrapdoorData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    open: match props["open"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::AcaciaTrapdoor(data))
            }
            "minecraft:dark_oak_trapdoor" => {
                let data = DarkOakTrapdoorData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    open: match props["open"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DarkOakTrapdoor(data))
            }
            "minecraft:infested_stone" => Some(Block::InfestedStone),
            "minecraft:infested_cobblestone" => Some(Block::InfestedCobblestone),
            "minecraft:infested_stone_bricks" => Some(Block::InfestedStoneBricks),
            "minecraft:infested_mossy_stone_bricks" => Some(Block::InfestedMossyStoneBricks),
            "minecraft:infested_cracked_stone_bricks" => Some(Block::InfestedCrackedStoneBricks),
            "minecraft:infested_chiseled_stone_bricks" => Some(Block::InfestedChiseledStoneBricks),
            "minecraft:stone_bricks" => Some(Block::StoneBricks),
            "minecraft:mossy_stone_bricks" => Some(Block::MossyStoneBricks),
            "minecraft:cracked_stone_bricks" => Some(Block::CrackedStoneBricks),
            "minecraft:chiseled_stone_bricks" => Some(Block::ChiseledStoneBricks),
            "minecraft:brown_mushroom_block" => {
                let data = BrownMushroomBlockData {
                    down: match props["down"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    up: match props["up"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BrownMushroomBlock(data))
            }
            "minecraft:red_mushroom_block" => {
                let data = RedMushroomBlockData {
                    down: match props["down"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    up: match props["up"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::RedMushroomBlock(data))
            }
            "minecraft:mushroom_stem" => {
                let data = MushroomStemData {
                    down: match props["down"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    up: match props["up"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::MushroomStem(data))
            }
            "minecraft:iron_bars" => {
                let data = IronBarsData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::IronBars(data))
            }
            "minecraft:glass_pane" => {
                let data = GlassPaneData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::GlassPane(data))
            }
            "minecraft:melon" => Some(Block::Melon),
            "minecraft:attached_pumpkin_stem" => {
                let data = AttachedPumpkinStemData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::AttachedPumpkinStem(data))
            }
            "minecraft:attached_melon_stem" => {
                let data = AttachedMelonStemData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::AttachedMelonStem(data))
            }
            "minecraft:pumpkin_stem" => {
                let data = PumpkinStemData {
                    age: match props["age"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        _ => return None,
                    },
                };
                Some(Block::PumpkinStem(data))
            }
            "minecraft:melon_stem" => {
                let data = MelonStemData {
                    age: match props["age"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        _ => return None,
                    },
                };
                Some(Block::MelonStem(data))
            }
            "minecraft:vine" => {
                let data = VineData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    up: match props["up"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::Vine(data))
            }
            "minecraft:oak_fence_gate" => {
                let data = OakFenceGateData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    in_wall: match props["in_wall"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    open: match props["open"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::OakFenceGate(data))
            }
            "minecraft:brick_stairs" => {
                let data = BrickStairsData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    shape: match props["shape"].as_str() {
                        "straight" => Shape::Straight,
                        "inner_left" => Shape::InnerLeft,
                        "inner_right" => Shape::InnerRight,
                        "outer_left" => Shape::OuterLeft,
                        "outer_right" => Shape::OuterRight,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BrickStairs(data))
            }
            "minecraft:stone_brick_stairs" => {
                let data = StoneBrickStairsData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    shape: match props["shape"].as_str() {
                        "straight" => Shape::Straight,
                        "inner_left" => Shape::InnerLeft,
                        "inner_right" => Shape::InnerRight,
                        "outer_left" => Shape::OuterLeft,
                        "outer_right" => Shape::OuterRight,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::StoneBrickStairs(data))
            }
            "minecraft:mycelium" => {
                let data = MyceliumData {
                    snowy: match props["snowy"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::Mycelium(data))
            }
            "minecraft:lily_pad" => Some(Block::LilyPad),
            "minecraft:nether_bricks" => Some(Block::NetherBricks),
            "minecraft:nether_brick_fence" => {
                let data = NetherBrickFenceData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::NetherBrickFence(data))
            }
            "minecraft:nether_brick_stairs" => {
                let data = NetherBrickStairsData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    shape: match props["shape"].as_str() {
                        "straight" => Shape::Straight,
                        "inner_left" => Shape::InnerLeft,
                        "inner_right" => Shape::InnerRight,
                        "outer_left" => Shape::OuterLeft,
                        "outer_right" => Shape::OuterRight,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::NetherBrickStairs(data))
            }
            "minecraft:nether_wart" => {
                let data = NetherWartData {
                    age: match props["age"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        _ => return None,
                    },
                };
                Some(Block::NetherWart(data))
            }
            "minecraft:enchanting_table" => Some(Block::EnchantingTable),
            "minecraft:brewing_stand" => {
                let data = BrewingStandData {
                    has_bottle_0: match props["has_bottle_0"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    has_bottle_1: match props["has_bottle_1"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    has_bottle_2: match props["has_bottle_2"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BrewingStand(data))
            }
            "minecraft:cauldron" => {
                let data = CauldronData {
                    level: match props["level"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        _ => return None,
                    },
                };
                Some(Block::Cauldron(data))
            }
            "minecraft:end_portal" => Some(Block::EndPortal),
            "minecraft:end_portal_frame" => {
                let data = EndPortalFrameData {
                    eye: match props["eye"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::EndPortalFrame(data))
            }
            "minecraft:end_stone" => Some(Block::EndStone),
            "minecraft:dragon_egg" => Some(Block::DragonEgg),
            "minecraft:redstone_lamp" => {
                let data = RedstoneLampData {
                    lit: match props["lit"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::RedstoneLamp(data))
            }
            "minecraft:cocoa" => {
                let data = CocoaData {
                    age: match props["age"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        _ => return None,
                    },
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::Cocoa(data))
            }
            "minecraft:sandstone_stairs" => {
                let data = SandstoneStairsData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    shape: match props["shape"].as_str() {
                        "straight" => Shape::Straight,
                        "inner_left" => Shape::InnerLeft,
                        "inner_right" => Shape::InnerRight,
                        "outer_left" => Shape::OuterLeft,
                        "outer_right" => Shape::OuterRight,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::SandstoneStairs(data))
            }
            "minecraft:emerald_ore" => Some(Block::EmeraldOre),
            "minecraft:ender_chest" => {
                let data = EnderChestData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::EnderChest(data))
            }
            "minecraft:tripwire_hook" => {
                let data = TripwireHookData {
                    attached: match props["attached"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::TripwireHook(data))
            }
            "minecraft:tripwire" => {
                let data = TripwireData {
                    attached: match props["attached"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    disarmed: match props["disarmed"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::Tripwire(data))
            }
            "minecraft:emerald_block" => Some(Block::EmeraldBlock),
            "minecraft:spruce_stairs" => {
                let data = SpruceStairsData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    shape: match props["shape"].as_str() {
                        "straight" => Shape::Straight,
                        "inner_left" => Shape::InnerLeft,
                        "inner_right" => Shape::InnerRight,
                        "outer_left" => Shape::OuterLeft,
                        "outer_right" => Shape::OuterRight,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::SpruceStairs(data))
            }
            "minecraft:birch_stairs" => {
                let data = BirchStairsData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    shape: match props["shape"].as_str() {
                        "straight" => Shape::Straight,
                        "inner_left" => Shape::InnerLeft,
                        "inner_right" => Shape::InnerRight,
                        "outer_left" => Shape::OuterLeft,
                        "outer_right" => Shape::OuterRight,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BirchStairs(data))
            }
            "minecraft:jungle_stairs" => {
                let data = JungleStairsData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    shape: match props["shape"].as_str() {
                        "straight" => Shape::Straight,
                        "inner_left" => Shape::InnerLeft,
                        "inner_right" => Shape::InnerRight,
                        "outer_left" => Shape::OuterLeft,
                        "outer_right" => Shape::OuterRight,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::JungleStairs(data))
            }
            "minecraft:command_block" => {
                let data = CommandBlockData {
                    conditional: match props["conditional"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::CommandBlock(data))
            }
            "minecraft:beacon" => Some(Block::Beacon),
            "minecraft:cobblestone_wall" => {
                let data = CobblestoneWallData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    up: match props["up"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::CobblestoneWall(data))
            }
            "minecraft:mossy_cobblestone_wall" => {
                let data = MossyCobblestoneWallData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    up: match props["up"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::MossyCobblestoneWall(data))
            }
            "minecraft:flower_pot" => Some(Block::FlowerPot),
            "minecraft:potted_oak_sapling" => Some(Block::PottedOakSapling),
            "minecraft:potted_spruce_sapling" => Some(Block::PottedSpruceSapling),
            "minecraft:potted_birch_sapling" => Some(Block::PottedBirchSapling),
            "minecraft:potted_jungle_sapling" => Some(Block::PottedJungleSapling),
            "minecraft:potted_acacia_sapling" => Some(Block::PottedAcaciaSapling),
            "minecraft:potted_dark_oak_sapling" => Some(Block::PottedDarkOakSapling),
            "minecraft:potted_fern" => Some(Block::PottedFern),
            "minecraft:potted_dandelion" => Some(Block::PottedDandelion),
            "minecraft:potted_poppy" => Some(Block::PottedPoppy),
            "minecraft:potted_blue_orchid" => Some(Block::PottedBlueOrchid),
            "minecraft:potted_allium" => Some(Block::PottedAllium),
            "minecraft:potted_azure_bluet" => Some(Block::PottedAzureBluet),
            "minecraft:potted_red_tulip" => Some(Block::PottedRedTulip),
            "minecraft:potted_orange_tulip" => Some(Block::PottedOrangeTulip),
            "minecraft:potted_white_tulip" => Some(Block::PottedWhiteTulip),
            "minecraft:potted_pink_tulip" => Some(Block::PottedPinkTulip),
            "minecraft:potted_oxeye_daisy" => Some(Block::PottedOxeyeDaisy),
            "minecraft:potted_red_mushroom" => Some(Block::PottedRedMushroom),
            "minecraft:potted_brown_mushroom" => Some(Block::PottedBrownMushroom),
            "minecraft:potted_dead_bush" => Some(Block::PottedDeadBush),
            "minecraft:potted_cactus" => Some(Block::PottedCactus),
            "minecraft:carrots" => {
                let data = CarrotsData {
                    age: match props["age"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        _ => return None,
                    },
                };
                Some(Block::Carrots(data))
            }
            "minecraft:potatoes" => {
                let data = PotatoesData {
                    age: match props["age"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        _ => return None,
                    },
                };
                Some(Block::Potatoes(data))
            }
            "minecraft:oak_button" => {
                let data = OakButtonData {
                    face: match props["face"].as_str() {
                        "floor" => Face::Floor,
                        "wall" => Face::Wall,
                        "ceiling" => Face::Ceiling,
                        _ => return None,
                    },
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::OakButton(data))
            }
            "minecraft:spruce_button" => {
                let data = SpruceButtonData {
                    face: match props["face"].as_str() {
                        "floor" => Face::Floor,
                        "wall" => Face::Wall,
                        "ceiling" => Face::Ceiling,
                        _ => return None,
                    },
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::SpruceButton(data))
            }
            "minecraft:birch_button" => {
                let data = BirchButtonData {
                    face: match props["face"].as_str() {
                        "floor" => Face::Floor,
                        "wall" => Face::Wall,
                        "ceiling" => Face::Ceiling,
                        _ => return None,
                    },
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BirchButton(data))
            }
            "minecraft:jungle_button" => {
                let data = JungleButtonData {
                    face: match props["face"].as_str() {
                        "floor" => Face::Floor,
                        "wall" => Face::Wall,
                        "ceiling" => Face::Ceiling,
                        _ => return None,
                    },
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::JungleButton(data))
            }
            "minecraft:acacia_button" => {
                let data = AcaciaButtonData {
                    face: match props["face"].as_str() {
                        "floor" => Face::Floor,
                        "wall" => Face::Wall,
                        "ceiling" => Face::Ceiling,
                        _ => return None,
                    },
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::AcaciaButton(data))
            }
            "minecraft:dark_oak_button" => {
                let data = DarkOakButtonData {
                    face: match props["face"].as_str() {
                        "floor" => Face::Floor,
                        "wall" => Face::Wall,
                        "ceiling" => Face::Ceiling,
                        _ => return None,
                    },
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DarkOakButton(data))
            }
            "minecraft:skeleton_wall_skull" => {
                let data = SkeletonWallSkullData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::SkeletonWallSkull(data))
            }
            "minecraft:skeleton_skull" => {
                let data = SkeletonSkullData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::SkeletonSkull(data))
            }
            "minecraft:wither_skeleton_wall_skull" => {
                let data = WitherSkeletonWallSkullData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::WitherSkeletonWallSkull(data))
            }
            "minecraft:wither_skeleton_skull" => {
                let data = WitherSkeletonSkullData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::WitherSkeletonSkull(data))
            }
            "minecraft:zombie_wall_head" => {
                let data = ZombieWallHeadData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::ZombieWallHead(data))
            }
            "minecraft:zombie_head" => {
                let data = ZombieHeadData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::ZombieHead(data))
            }
            "minecraft:player_wall_head" => {
                let data = PlayerWallHeadData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::PlayerWallHead(data))
            }
            "minecraft:player_head" => {
                let data = PlayerHeadData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::PlayerHead(data))
            }
            "minecraft:creeper_wall_head" => {
                let data = CreeperWallHeadData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::CreeperWallHead(data))
            }
            "minecraft:creeper_head" => {
                let data = CreeperHeadData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::CreeperHead(data))
            }
            "minecraft:dragon_wall_head" => {
                let data = DragonWallHeadData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::DragonWallHead(data))
            }
            "minecraft:dragon_head" => {
                let data = DragonHeadData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::DragonHead(data))
            }
            "minecraft:anvil" => {
                let data = AnvilData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::Anvil(data))
            }
            "minecraft:chipped_anvil" => {
                let data = ChippedAnvilData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::ChippedAnvil(data))
            }
            "minecraft:damaged_anvil" => {
                let data = DamagedAnvilData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::DamagedAnvil(data))
            }
            "minecraft:trapped_chest" => {
                let data = TrappedChestData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    type_: match props["type"].as_str() {
                        "single" => TrappedChestType::Single,
                        "left" => TrappedChestType::Left,
                        "right" => TrappedChestType::Right,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::TrappedChest(data))
            }
            "minecraft:light_weighted_pressure_plate" => {
                let data = LightWeightedPressurePlateData {
                    power: match props["power"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::LightWeightedPressurePlate(data))
            }
            "minecraft:heavy_weighted_pressure_plate" => {
                let data = HeavyWeightedPressurePlateData {
                    power: match props["power"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::HeavyWeightedPressurePlate(data))
            }
            "minecraft:comparator" => {
                let data = ComparatorData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    mode: match props["mode"].as_str() {
                        "compare" => ComparatorMode::Compare,
                        "subtract" => ComparatorMode::Subtract,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::Comparator(data))
            }
            "minecraft:daylight_detector" => {
                let data = DaylightDetectorData {
                    inverted: match props["inverted"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    power: match props["power"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::DaylightDetector(data))
            }
            "minecraft:redstone_block" => Some(Block::RedstoneBlock),
            "minecraft:nether_quartz_ore" => Some(Block::NetherQuartzOre),
            "minecraft:hopper" => {
                let data = HopperData {
                    enabled: match props["enabled"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    facing: match props["facing"].as_str() {
                        "down" => Facing::Down,
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::Hopper(data))
            }
            "minecraft:quartz_block" => Some(Block::QuartzBlock),
            "minecraft:chiseled_quartz_block" => Some(Block::ChiseledQuartzBlock),
            "minecraft:quartz_pillar" => {
                let data = QuartzPillarData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::QuartzPillar(data))
            }
            "minecraft:quartz_stairs" => {
                let data = QuartzStairsData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    shape: match props["shape"].as_str() {
                        "straight" => Shape::Straight,
                        "inner_left" => Shape::InnerLeft,
                        "inner_right" => Shape::InnerRight,
                        "outer_left" => Shape::OuterLeft,
                        "outer_right" => Shape::OuterRight,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::QuartzStairs(data))
            }
            "minecraft:activator_rail" => {
                let data = ActivatorRailData {
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    shape: match props["shape"].as_str() {
                        "north_south" => Shape::NorthSouth,
                        "east_west" => Shape::EastWest,
                        "ascending_east" => Shape::AscendingEast,
                        "ascending_west" => Shape::AscendingWest,
                        "ascending_north" => Shape::AscendingNorth,
                        "ascending_south" => Shape::AscendingSouth,
                        _ => return None,
                    },
                };
                Some(Block::ActivatorRail(data))
            }
            "minecraft:dropper" => {
                let data = DropperData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                    triggered: match props["triggered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::Dropper(data))
            }
            "minecraft:white_terracotta" => Some(Block::WhiteTerracotta),
            "minecraft:orange_terracotta" => Some(Block::OrangeTerracotta),
            "minecraft:magenta_terracotta" => Some(Block::MagentaTerracotta),
            "minecraft:light_blue_terracotta" => Some(Block::LightBlueTerracotta),
            "minecraft:yellow_terracotta" => Some(Block::YellowTerracotta),
            "minecraft:lime_terracotta" => Some(Block::LimeTerracotta),
            "minecraft:pink_terracotta" => Some(Block::PinkTerracotta),
            "minecraft:gray_terracotta" => Some(Block::GrayTerracotta),
            "minecraft:light_gray_terracotta" => Some(Block::LightGrayTerracotta),
            "minecraft:cyan_terracotta" => Some(Block::CyanTerracotta),
            "minecraft:purple_terracotta" => Some(Block::PurpleTerracotta),
            "minecraft:blue_terracotta" => Some(Block::BlueTerracotta),
            "minecraft:brown_terracotta" => Some(Block::BrownTerracotta),
            "minecraft:green_terracotta" => Some(Block::GreenTerracotta),
            "minecraft:red_terracotta" => Some(Block::RedTerracotta),
            "minecraft:black_terracotta" => Some(Block::BlackTerracotta),
            "minecraft:white_stained_glass_pane" => {
                let data = WhiteStainedGlassPaneData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::WhiteStainedGlassPane(data))
            }
            "minecraft:orange_stained_glass_pane" => {
                let data = OrangeStainedGlassPaneData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::OrangeStainedGlassPane(data))
            }
            "minecraft:magenta_stained_glass_pane" => {
                let data = MagentaStainedGlassPaneData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::MagentaStainedGlassPane(data))
            }
            "minecraft:light_blue_stained_glass_pane" => {
                let data = LightBlueStainedGlassPaneData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::LightBlueStainedGlassPane(data))
            }
            "minecraft:yellow_stained_glass_pane" => {
                let data = YellowStainedGlassPaneData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::YellowStainedGlassPane(data))
            }
            "minecraft:lime_stained_glass_pane" => {
                let data = LimeStainedGlassPaneData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::LimeStainedGlassPane(data))
            }
            "minecraft:pink_stained_glass_pane" => {
                let data = PinkStainedGlassPaneData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::PinkStainedGlassPane(data))
            }
            "minecraft:gray_stained_glass_pane" => {
                let data = GrayStainedGlassPaneData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::GrayStainedGlassPane(data))
            }
            "minecraft:light_gray_stained_glass_pane" => {
                let data = LightGrayStainedGlassPaneData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::LightGrayStainedGlassPane(data))
            }
            "minecraft:cyan_stained_glass_pane" => {
                let data = CyanStainedGlassPaneData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::CyanStainedGlassPane(data))
            }
            "minecraft:purple_stained_glass_pane" => {
                let data = PurpleStainedGlassPaneData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::PurpleStainedGlassPane(data))
            }
            "minecraft:blue_stained_glass_pane" => {
                let data = BlueStainedGlassPaneData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BlueStainedGlassPane(data))
            }
            "minecraft:brown_stained_glass_pane" => {
                let data = BrownStainedGlassPaneData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BrownStainedGlassPane(data))
            }
            "minecraft:green_stained_glass_pane" => {
                let data = GreenStainedGlassPaneData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::GreenStainedGlassPane(data))
            }
            "minecraft:red_stained_glass_pane" => {
                let data = RedStainedGlassPaneData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::RedStainedGlassPane(data))
            }
            "minecraft:black_stained_glass_pane" => {
                let data = BlackStainedGlassPaneData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BlackStainedGlassPane(data))
            }
            "minecraft:acacia_stairs" => {
                let data = AcaciaStairsData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    shape: match props["shape"].as_str() {
                        "straight" => Shape::Straight,
                        "inner_left" => Shape::InnerLeft,
                        "inner_right" => Shape::InnerRight,
                        "outer_left" => Shape::OuterLeft,
                        "outer_right" => Shape::OuterRight,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::AcaciaStairs(data))
            }
            "minecraft:dark_oak_stairs" => {
                let data = DarkOakStairsData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    shape: match props["shape"].as_str() {
                        "straight" => Shape::Straight,
                        "inner_left" => Shape::InnerLeft,
                        "inner_right" => Shape::InnerRight,
                        "outer_left" => Shape::OuterLeft,
                        "outer_right" => Shape::OuterRight,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DarkOakStairs(data))
            }
            "minecraft:slime_block" => Some(Block::SlimeBlock),
            "minecraft:barrier" => Some(Block::Barrier),
            "minecraft:iron_trapdoor" => {
                let data = IronTrapdoorData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    open: match props["open"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::IronTrapdoor(data))
            }
            "minecraft:prismarine" => Some(Block::Prismarine),
            "minecraft:prismarine_bricks" => Some(Block::PrismarineBricks),
            "minecraft:dark_prismarine" => Some(Block::DarkPrismarine),
            "minecraft:prismarine_stairs" => {
                let data = PrismarineStairsData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    shape: match props["shape"].as_str() {
                        "straight" => Shape::Straight,
                        "inner_left" => Shape::InnerLeft,
                        "inner_right" => Shape::InnerRight,
                        "outer_left" => Shape::OuterLeft,
                        "outer_right" => Shape::OuterRight,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::PrismarineStairs(data))
            }
            "minecraft:prismarine_brick_stairs" => {
                let data = PrismarineBrickStairsData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    shape: match props["shape"].as_str() {
                        "straight" => Shape::Straight,
                        "inner_left" => Shape::InnerLeft,
                        "inner_right" => Shape::InnerRight,
                        "outer_left" => Shape::OuterLeft,
                        "outer_right" => Shape::OuterRight,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::PrismarineBrickStairs(data))
            }
            "minecraft:dark_prismarine_stairs" => {
                let data = DarkPrismarineStairsData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    shape: match props["shape"].as_str() {
                        "straight" => Shape::Straight,
                        "inner_left" => Shape::InnerLeft,
                        "inner_right" => Shape::InnerRight,
                        "outer_left" => Shape::OuterLeft,
                        "outer_right" => Shape::OuterRight,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DarkPrismarineStairs(data))
            }
            "minecraft:prismarine_slab" => {
                let data = PrismarineSlabData {
                    type_: match props["type"].as_str() {
                        "top" => PrismarineSlabType::Top,
                        "bottom" => PrismarineSlabType::Bottom,
                        "double" => PrismarineSlabType::Double,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::PrismarineSlab(data))
            }
            "minecraft:prismarine_brick_slab" => {
                let data = PrismarineBrickSlabData {
                    type_: match props["type"].as_str() {
                        "top" => PrismarineBrickSlabType::Top,
                        "bottom" => PrismarineBrickSlabType::Bottom,
                        "double" => PrismarineBrickSlabType::Double,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::PrismarineBrickSlab(data))
            }
            "minecraft:dark_prismarine_slab" => {
                let data = DarkPrismarineSlabData {
                    type_: match props["type"].as_str() {
                        "top" => DarkPrismarineSlabType::Top,
                        "bottom" => DarkPrismarineSlabType::Bottom,
                        "double" => DarkPrismarineSlabType::Double,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DarkPrismarineSlab(data))
            }
            "minecraft:sea_lantern" => Some(Block::SeaLantern),
            "minecraft:hay_block" => {
                let data = HayBlockData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::HayBlock(data))
            }
            "minecraft:white_carpet" => Some(Block::WhiteCarpet),
            "minecraft:orange_carpet" => Some(Block::OrangeCarpet),
            "minecraft:magenta_carpet" => Some(Block::MagentaCarpet),
            "minecraft:light_blue_carpet" => Some(Block::LightBlueCarpet),
            "minecraft:yellow_carpet" => Some(Block::YellowCarpet),
            "minecraft:lime_carpet" => Some(Block::LimeCarpet),
            "minecraft:pink_carpet" => Some(Block::PinkCarpet),
            "minecraft:gray_carpet" => Some(Block::GrayCarpet),
            "minecraft:light_gray_carpet" => Some(Block::LightGrayCarpet),
            "minecraft:cyan_carpet" => Some(Block::CyanCarpet),
            "minecraft:purple_carpet" => Some(Block::PurpleCarpet),
            "minecraft:blue_carpet" => Some(Block::BlueCarpet),
            "minecraft:brown_carpet" => Some(Block::BrownCarpet),
            "minecraft:green_carpet" => Some(Block::GreenCarpet),
            "minecraft:red_carpet" => Some(Block::RedCarpet),
            "minecraft:black_carpet" => Some(Block::BlackCarpet),
            "minecraft:terracotta" => Some(Block::Terracotta),
            "minecraft:coal_block" => Some(Block::CoalBlock),
            "minecraft:packed_ice" => Some(Block::PackedIce),
            "minecraft:sunflower" => {
                let data = SunflowerData {
                    half: match props["half"].as_str() {
                        "upper" => Half::Upper,
                        "lower" => Half::Lower,
                        _ => return None,
                    },
                };
                Some(Block::Sunflower(data))
            }
            "minecraft:lilac" => {
                let data = LilacData {
                    half: match props["half"].as_str() {
                        "upper" => Half::Upper,
                        "lower" => Half::Lower,
                        _ => return None,
                    },
                };
                Some(Block::Lilac(data))
            }
            "minecraft:rose_bush" => {
                let data = RoseBushData {
                    half: match props["half"].as_str() {
                        "upper" => Half::Upper,
                        "lower" => Half::Lower,
                        _ => return None,
                    },
                };
                Some(Block::RoseBush(data))
            }
            "minecraft:peony" => {
                let data = PeonyData {
                    half: match props["half"].as_str() {
                        "upper" => Half::Upper,
                        "lower" => Half::Lower,
                        _ => return None,
                    },
                };
                Some(Block::Peony(data))
            }
            "minecraft:tall_grass" => {
                let data = TallGrassData {
                    half: match props["half"].as_str() {
                        "upper" => Half::Upper,
                        "lower" => Half::Lower,
                        _ => return None,
                    },
                };
                Some(Block::TallGrass(data))
            }
            "minecraft:large_fern" => {
                let data = LargeFernData {
                    half: match props["half"].as_str() {
                        "upper" => Half::Upper,
                        "lower" => Half::Lower,
                        _ => return None,
                    },
                };
                Some(Block::LargeFern(data))
            }
            "minecraft:white_banner" => {
                let data = WhiteBannerData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::WhiteBanner(data))
            }
            "minecraft:orange_banner" => {
                let data = OrangeBannerData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::OrangeBanner(data))
            }
            "minecraft:magenta_banner" => {
                let data = MagentaBannerData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::MagentaBanner(data))
            }
            "minecraft:light_blue_banner" => {
                let data = LightBlueBannerData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::LightBlueBanner(data))
            }
            "minecraft:yellow_banner" => {
                let data = YellowBannerData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::YellowBanner(data))
            }
            "minecraft:lime_banner" => {
                let data = LimeBannerData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::LimeBanner(data))
            }
            "minecraft:pink_banner" => {
                let data = PinkBannerData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::PinkBanner(data))
            }
            "minecraft:gray_banner" => {
                let data = GrayBannerData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::GrayBanner(data))
            }
            "minecraft:light_gray_banner" => {
                let data = LightGrayBannerData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::LightGrayBanner(data))
            }
            "minecraft:cyan_banner" => {
                let data = CyanBannerData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::CyanBanner(data))
            }
            "minecraft:purple_banner" => {
                let data = PurpleBannerData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::PurpleBanner(data))
            }
            "minecraft:blue_banner" => {
                let data = BlueBannerData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::BlueBanner(data))
            }
            "minecraft:brown_banner" => {
                let data = BrownBannerData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::BrownBanner(data))
            }
            "minecraft:green_banner" => {
                let data = GreenBannerData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::GreenBanner(data))
            }
            "minecraft:red_banner" => {
                let data = RedBannerData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::RedBanner(data))
            }
            "minecraft:black_banner" => {
                let data = BlackBannerData {
                    rotation: match props["rotation"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        _ => return None,
                    },
                };
                Some(Block::BlackBanner(data))
            }
            "minecraft:white_wall_banner" => {
                let data = WhiteWallBannerData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::WhiteWallBanner(data))
            }
            "minecraft:orange_wall_banner" => {
                let data = OrangeWallBannerData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::OrangeWallBanner(data))
            }
            "minecraft:magenta_wall_banner" => {
                let data = MagentaWallBannerData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::MagentaWallBanner(data))
            }
            "minecraft:light_blue_wall_banner" => {
                let data = LightBlueWallBannerData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::LightBlueWallBanner(data))
            }
            "minecraft:yellow_wall_banner" => {
                let data = YellowWallBannerData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::YellowWallBanner(data))
            }
            "minecraft:lime_wall_banner" => {
                let data = LimeWallBannerData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::LimeWallBanner(data))
            }
            "minecraft:pink_wall_banner" => {
                let data = PinkWallBannerData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::PinkWallBanner(data))
            }
            "minecraft:gray_wall_banner" => {
                let data = GrayWallBannerData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::GrayWallBanner(data))
            }
            "minecraft:light_gray_wall_banner" => {
                let data = LightGrayWallBannerData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::LightGrayWallBanner(data))
            }
            "minecraft:cyan_wall_banner" => {
                let data = CyanWallBannerData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::CyanWallBanner(data))
            }
            "minecraft:purple_wall_banner" => {
                let data = PurpleWallBannerData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::PurpleWallBanner(data))
            }
            "minecraft:blue_wall_banner" => {
                let data = BlueWallBannerData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::BlueWallBanner(data))
            }
            "minecraft:brown_wall_banner" => {
                let data = BrownWallBannerData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::BrownWallBanner(data))
            }
            "minecraft:green_wall_banner" => {
                let data = GreenWallBannerData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::GreenWallBanner(data))
            }
            "minecraft:red_wall_banner" => {
                let data = RedWallBannerData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::RedWallBanner(data))
            }
            "minecraft:black_wall_banner" => {
                let data = BlackWallBannerData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::BlackWallBanner(data))
            }
            "minecraft:red_sandstone" => Some(Block::RedSandstone),
            "minecraft:chiseled_red_sandstone" => Some(Block::ChiseledRedSandstone),
            "minecraft:cut_red_sandstone" => Some(Block::CutRedSandstone),
            "minecraft:red_sandstone_stairs" => {
                let data = RedSandstoneStairsData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    shape: match props["shape"].as_str() {
                        "straight" => Shape::Straight,
                        "inner_left" => Shape::InnerLeft,
                        "inner_right" => Shape::InnerRight,
                        "outer_left" => Shape::OuterLeft,
                        "outer_right" => Shape::OuterRight,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::RedSandstoneStairs(data))
            }
            "minecraft:oak_slab" => {
                let data = OakSlabData {
                    type_: match props["type"].as_str() {
                        "top" => OakSlabType::Top,
                        "bottom" => OakSlabType::Bottom,
                        "double" => OakSlabType::Double,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::OakSlab(data))
            }
            "minecraft:spruce_slab" => {
                let data = SpruceSlabData {
                    type_: match props["type"].as_str() {
                        "top" => SpruceSlabType::Top,
                        "bottom" => SpruceSlabType::Bottom,
                        "double" => SpruceSlabType::Double,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::SpruceSlab(data))
            }
            "minecraft:birch_slab" => {
                let data = BirchSlabData {
                    type_: match props["type"].as_str() {
                        "top" => BirchSlabType::Top,
                        "bottom" => BirchSlabType::Bottom,
                        "double" => BirchSlabType::Double,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BirchSlab(data))
            }
            "minecraft:jungle_slab" => {
                let data = JungleSlabData {
                    type_: match props["type"].as_str() {
                        "top" => JungleSlabType::Top,
                        "bottom" => JungleSlabType::Bottom,
                        "double" => JungleSlabType::Double,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::JungleSlab(data))
            }
            "minecraft:acacia_slab" => {
                let data = AcaciaSlabData {
                    type_: match props["type"].as_str() {
                        "top" => AcaciaSlabType::Top,
                        "bottom" => AcaciaSlabType::Bottom,
                        "double" => AcaciaSlabType::Double,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::AcaciaSlab(data))
            }
            "minecraft:dark_oak_slab" => {
                let data = DarkOakSlabData {
                    type_: match props["type"].as_str() {
                        "top" => DarkOakSlabType::Top,
                        "bottom" => DarkOakSlabType::Bottom,
                        "double" => DarkOakSlabType::Double,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DarkOakSlab(data))
            }
            "minecraft:stone_slab" => {
                let data = StoneSlabData {
                    type_: match props["type"].as_str() {
                        "top" => StoneSlabType::Top,
                        "bottom" => StoneSlabType::Bottom,
                        "double" => StoneSlabType::Double,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::StoneSlab(data))
            }
            "minecraft:sandstone_slab" => {
                let data = SandstoneSlabData {
                    type_: match props["type"].as_str() {
                        "top" => SandstoneSlabType::Top,
                        "bottom" => SandstoneSlabType::Bottom,
                        "double" => SandstoneSlabType::Double,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::SandstoneSlab(data))
            }
            "minecraft:petrified_oak_slab" => {
                let data = PetrifiedOakSlabData {
                    type_: match props["type"].as_str() {
                        "top" => PetrifiedOakSlabType::Top,
                        "bottom" => PetrifiedOakSlabType::Bottom,
                        "double" => PetrifiedOakSlabType::Double,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::PetrifiedOakSlab(data))
            }
            "minecraft:cobblestone_slab" => {
                let data = CobblestoneSlabData {
                    type_: match props["type"].as_str() {
                        "top" => CobblestoneSlabType::Top,
                        "bottom" => CobblestoneSlabType::Bottom,
                        "double" => CobblestoneSlabType::Double,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::CobblestoneSlab(data))
            }
            "minecraft:brick_slab" => {
                let data = BrickSlabData {
                    type_: match props["type"].as_str() {
                        "top" => BrickSlabType::Top,
                        "bottom" => BrickSlabType::Bottom,
                        "double" => BrickSlabType::Double,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BrickSlab(data))
            }
            "minecraft:stone_brick_slab" => {
                let data = StoneBrickSlabData {
                    type_: match props["type"].as_str() {
                        "top" => StoneBrickSlabType::Top,
                        "bottom" => StoneBrickSlabType::Bottom,
                        "double" => StoneBrickSlabType::Double,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::StoneBrickSlab(data))
            }
            "minecraft:nether_brick_slab" => {
                let data = NetherBrickSlabData {
                    type_: match props["type"].as_str() {
                        "top" => NetherBrickSlabType::Top,
                        "bottom" => NetherBrickSlabType::Bottom,
                        "double" => NetherBrickSlabType::Double,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::NetherBrickSlab(data))
            }
            "minecraft:quartz_slab" => {
                let data = QuartzSlabData {
                    type_: match props["type"].as_str() {
                        "top" => QuartzSlabType::Top,
                        "bottom" => QuartzSlabType::Bottom,
                        "double" => QuartzSlabType::Double,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::QuartzSlab(data))
            }
            "minecraft:red_sandstone_slab" => {
                let data = RedSandstoneSlabData {
                    type_: match props["type"].as_str() {
                        "top" => RedSandstoneSlabType::Top,
                        "bottom" => RedSandstoneSlabType::Bottom,
                        "double" => RedSandstoneSlabType::Double,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::RedSandstoneSlab(data))
            }
            "minecraft:purpur_slab" => {
                let data = PurpurSlabData {
                    type_: match props["type"].as_str() {
                        "top" => PurpurSlabType::Top,
                        "bottom" => PurpurSlabType::Bottom,
                        "double" => PurpurSlabType::Double,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::PurpurSlab(data))
            }
            "minecraft:smooth_stone" => Some(Block::SmoothStone),
            "minecraft:smooth_sandstone" => Some(Block::SmoothSandstone),
            "minecraft:smooth_quartz" => Some(Block::SmoothQuartz),
            "minecraft:smooth_red_sandstone" => Some(Block::SmoothRedSandstone),
            "minecraft:spruce_fence_gate" => {
                let data = SpruceFenceGateData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    in_wall: match props["in_wall"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    open: match props["open"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::SpruceFenceGate(data))
            }
            "minecraft:birch_fence_gate" => {
                let data = BirchFenceGateData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    in_wall: match props["in_wall"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    open: match props["open"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BirchFenceGate(data))
            }
            "minecraft:jungle_fence_gate" => {
                let data = JungleFenceGateData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    in_wall: match props["in_wall"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    open: match props["open"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::JungleFenceGate(data))
            }
            "minecraft:acacia_fence_gate" => {
                let data = AcaciaFenceGateData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    in_wall: match props["in_wall"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    open: match props["open"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::AcaciaFenceGate(data))
            }
            "minecraft:dark_oak_fence_gate" => {
                let data = DarkOakFenceGateData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    in_wall: match props["in_wall"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    open: match props["open"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DarkOakFenceGate(data))
            }
            "minecraft:spruce_fence" => {
                let data = SpruceFenceData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::SpruceFence(data))
            }
            "minecraft:birch_fence" => {
                let data = BirchFenceData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BirchFence(data))
            }
            "minecraft:jungle_fence" => {
                let data = JungleFenceData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::JungleFence(data))
            }
            "minecraft:acacia_fence" => {
                let data = AcaciaFenceData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::AcaciaFence(data))
            }
            "minecraft:dark_oak_fence" => {
                let data = DarkOakFenceData {
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DarkOakFence(data))
            }
            "minecraft:spruce_door" => {
                let data = SpruceDoorData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "upper" => Half::Upper,
                        "lower" => Half::Lower,
                        _ => return None,
                    },
                    hinge: match props["hinge"].as_str() {
                        "left" => Hinge::Left,
                        "right" => Hinge::Right,
                        _ => return None,
                    },
                    open: match props["open"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::SpruceDoor(data))
            }
            "minecraft:birch_door" => {
                let data = BirchDoorData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "upper" => Half::Upper,
                        "lower" => Half::Lower,
                        _ => return None,
                    },
                    hinge: match props["hinge"].as_str() {
                        "left" => Hinge::Left,
                        "right" => Hinge::Right,
                        _ => return None,
                    },
                    open: match props["open"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BirchDoor(data))
            }
            "minecraft:jungle_door" => {
                let data = JungleDoorData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "upper" => Half::Upper,
                        "lower" => Half::Lower,
                        _ => return None,
                    },
                    hinge: match props["hinge"].as_str() {
                        "left" => Hinge::Left,
                        "right" => Hinge::Right,
                        _ => return None,
                    },
                    open: match props["open"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::JungleDoor(data))
            }
            "minecraft:acacia_door" => {
                let data = AcaciaDoorData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "upper" => Half::Upper,
                        "lower" => Half::Lower,
                        _ => return None,
                    },
                    hinge: match props["hinge"].as_str() {
                        "left" => Hinge::Left,
                        "right" => Hinge::Right,
                        _ => return None,
                    },
                    open: match props["open"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::AcaciaDoor(data))
            }
            "minecraft:dark_oak_door" => {
                let data = DarkOakDoorData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "upper" => Half::Upper,
                        "lower" => Half::Lower,
                        _ => return None,
                    },
                    hinge: match props["hinge"].as_str() {
                        "left" => Hinge::Left,
                        "right" => Hinge::Right,
                        _ => return None,
                    },
                    open: match props["open"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DarkOakDoor(data))
            }
            "minecraft:end_rod" => {
                let data = EndRodData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::EndRod(data))
            }
            "minecraft:chorus_plant" => {
                let data = ChorusPlantData {
                    down: match props["down"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    east: match props["east"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    north: match props["north"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    south: match props["south"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    up: match props["up"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    west: match props["west"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::ChorusPlant(data))
            }
            "minecraft:chorus_flower" => {
                let data = ChorusFlowerData {
                    age: match props["age"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        _ => return None,
                    },
                };
                Some(Block::ChorusFlower(data))
            }
            "minecraft:purpur_block" => Some(Block::PurpurBlock),
            "minecraft:purpur_pillar" => {
                let data = PurpurPillarData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::PurpurPillar(data))
            }
            "minecraft:purpur_stairs" => {
                let data = PurpurStairsData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    half: match props["half"].as_str() {
                        "top" => Half::Top,
                        "bottom" => Half::Bottom,
                        _ => return None,
                    },
                    shape: match props["shape"].as_str() {
                        "straight" => Shape::Straight,
                        "inner_left" => Shape::InnerLeft,
                        "inner_right" => Shape::InnerRight,
                        "outer_left" => Shape::OuterLeft,
                        "outer_right" => Shape::OuterRight,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::PurpurStairs(data))
            }
            "minecraft:end_stone_bricks" => Some(Block::EndStoneBricks),
            "minecraft:beetroots" => {
                let data = BeetrootsData {
                    age: match props["age"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        _ => return None,
                    },
                };
                Some(Block::Beetroots(data))
            }
            "minecraft:grass_path" => Some(Block::GrassPath),
            "minecraft:end_gateway" => Some(Block::EndGateway),
            "minecraft:repeating_command_block" => {
                let data = RepeatingCommandBlockData {
                    conditional: match props["conditional"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::RepeatingCommandBlock(data))
            }
            "minecraft:chain_command_block" => {
                let data = ChainCommandBlockData {
                    conditional: match props["conditional"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::ChainCommandBlock(data))
            }
            "minecraft:frosted_ice" => {
                let data = FrostedIceData {
                    age: match props["age"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        _ => return None,
                    },
                };
                Some(Block::FrostedIce(data))
            }
            "minecraft:magma_block" => Some(Block::MagmaBlock),
            "minecraft:nether_wart_block" => Some(Block::NetherWartBlock),
            "minecraft:red_nether_bricks" => Some(Block::RedNetherBricks),
            "minecraft:bone_block" => {
                let data = BoneBlockData {
                    axis: match props["axis"].as_str() {
                        "x" => Axis::X,
                        "y" => Axis::Y,
                        "z" => Axis::Z,
                        _ => return None,
                    },
                };
                Some(Block::BoneBlock(data))
            }
            "minecraft:structure_void" => Some(Block::StructureVoid),
            "minecraft:observer" => {
                let data = ObserverData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                    powered: match props["powered"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::Observer(data))
            }
            "minecraft:shulker_box" => {
                let data = ShulkerBoxData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::ShulkerBox(data))
            }
            "minecraft:white_shulker_box" => {
                let data = WhiteShulkerBoxData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::WhiteShulkerBox(data))
            }
            "minecraft:orange_shulker_box" => {
                let data = OrangeShulkerBoxData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::OrangeShulkerBox(data))
            }
            "minecraft:magenta_shulker_box" => {
                let data = MagentaShulkerBoxData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::MagentaShulkerBox(data))
            }
            "minecraft:light_blue_shulker_box" => {
                let data = LightBlueShulkerBoxData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::LightBlueShulkerBox(data))
            }
            "minecraft:yellow_shulker_box" => {
                let data = YellowShulkerBoxData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::YellowShulkerBox(data))
            }
            "minecraft:lime_shulker_box" => {
                let data = LimeShulkerBoxData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::LimeShulkerBox(data))
            }
            "minecraft:pink_shulker_box" => {
                let data = PinkShulkerBoxData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::PinkShulkerBox(data))
            }
            "minecraft:gray_shulker_box" => {
                let data = GrayShulkerBoxData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::GrayShulkerBox(data))
            }
            "minecraft:light_gray_shulker_box" => {
                let data = LightGrayShulkerBoxData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::LightGrayShulkerBox(data))
            }
            "minecraft:cyan_shulker_box" => {
                let data = CyanShulkerBoxData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::CyanShulkerBox(data))
            }
            "minecraft:purple_shulker_box" => {
                let data = PurpleShulkerBoxData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::PurpleShulkerBox(data))
            }
            "minecraft:blue_shulker_box" => {
                let data = BlueShulkerBoxData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::BlueShulkerBox(data))
            }
            "minecraft:brown_shulker_box" => {
                let data = BrownShulkerBoxData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::BrownShulkerBox(data))
            }
            "minecraft:green_shulker_box" => {
                let data = GreenShulkerBoxData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::GreenShulkerBox(data))
            }
            "minecraft:red_shulker_box" => {
                let data = RedShulkerBoxData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::RedShulkerBox(data))
            }
            "minecraft:black_shulker_box" => {
                let data = BlackShulkerBoxData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "east" => Facing::East,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "up" => Facing::Up,
                        "down" => Facing::Down,
                        _ => return None,
                    },
                };
                Some(Block::BlackShulkerBox(data))
            }
            "minecraft:white_glazed_terracotta" => {
                let data = WhiteGlazedTerracottaData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::WhiteGlazedTerracotta(data))
            }
            "minecraft:orange_glazed_terracotta" => {
                let data = OrangeGlazedTerracottaData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::OrangeGlazedTerracotta(data))
            }
            "minecraft:magenta_glazed_terracotta" => {
                let data = MagentaGlazedTerracottaData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::MagentaGlazedTerracotta(data))
            }
            "minecraft:light_blue_glazed_terracotta" => {
                let data = LightBlueGlazedTerracottaData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::LightBlueGlazedTerracotta(data))
            }
            "minecraft:yellow_glazed_terracotta" => {
                let data = YellowGlazedTerracottaData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::YellowGlazedTerracotta(data))
            }
            "minecraft:lime_glazed_terracotta" => {
                let data = LimeGlazedTerracottaData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::LimeGlazedTerracotta(data))
            }
            "minecraft:pink_glazed_terracotta" => {
                let data = PinkGlazedTerracottaData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::PinkGlazedTerracotta(data))
            }
            "minecraft:gray_glazed_terracotta" => {
                let data = GrayGlazedTerracottaData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::GrayGlazedTerracotta(data))
            }
            "minecraft:light_gray_glazed_terracotta" => {
                let data = LightGrayGlazedTerracottaData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::LightGrayGlazedTerracotta(data))
            }
            "minecraft:cyan_glazed_terracotta" => {
                let data = CyanGlazedTerracottaData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::CyanGlazedTerracotta(data))
            }
            "minecraft:purple_glazed_terracotta" => {
                let data = PurpleGlazedTerracottaData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::PurpleGlazedTerracotta(data))
            }
            "minecraft:blue_glazed_terracotta" => {
                let data = BlueGlazedTerracottaData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::BlueGlazedTerracotta(data))
            }
            "minecraft:brown_glazed_terracotta" => {
                let data = BrownGlazedTerracottaData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::BrownGlazedTerracotta(data))
            }
            "minecraft:green_glazed_terracotta" => {
                let data = GreenGlazedTerracottaData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::GreenGlazedTerracotta(data))
            }
            "minecraft:red_glazed_terracotta" => {
                let data = RedGlazedTerracottaData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::RedGlazedTerracotta(data))
            }
            "minecraft:black_glazed_terracotta" => {
                let data = BlackGlazedTerracottaData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                };
                Some(Block::BlackGlazedTerracotta(data))
            }
            "minecraft:white_concrete" => Some(Block::WhiteConcrete),
            "minecraft:orange_concrete" => Some(Block::OrangeConcrete),
            "minecraft:magenta_concrete" => Some(Block::MagentaConcrete),
            "minecraft:light_blue_concrete" => Some(Block::LightBlueConcrete),
            "minecraft:yellow_concrete" => Some(Block::YellowConcrete),
            "minecraft:lime_concrete" => Some(Block::LimeConcrete),
            "minecraft:pink_concrete" => Some(Block::PinkConcrete),
            "minecraft:gray_concrete" => Some(Block::GrayConcrete),
            "minecraft:light_gray_concrete" => Some(Block::LightGrayConcrete),
            "minecraft:cyan_concrete" => Some(Block::CyanConcrete),
            "minecraft:purple_concrete" => Some(Block::PurpleConcrete),
            "minecraft:blue_concrete" => Some(Block::BlueConcrete),
            "minecraft:brown_concrete" => Some(Block::BrownConcrete),
            "minecraft:green_concrete" => Some(Block::GreenConcrete),
            "minecraft:red_concrete" => Some(Block::RedConcrete),
            "minecraft:black_concrete" => Some(Block::BlackConcrete),
            "minecraft:white_concrete_powder" => Some(Block::WhiteConcretePowder),
            "minecraft:orange_concrete_powder" => Some(Block::OrangeConcretePowder),
            "minecraft:magenta_concrete_powder" => Some(Block::MagentaConcretePowder),
            "minecraft:light_blue_concrete_powder" => Some(Block::LightBlueConcretePowder),
            "minecraft:yellow_concrete_powder" => Some(Block::YellowConcretePowder),
            "minecraft:lime_concrete_powder" => Some(Block::LimeConcretePowder),
            "minecraft:pink_concrete_powder" => Some(Block::PinkConcretePowder),
            "minecraft:gray_concrete_powder" => Some(Block::GrayConcretePowder),
            "minecraft:light_gray_concrete_powder" => Some(Block::LightGrayConcretePowder),
            "minecraft:cyan_concrete_powder" => Some(Block::CyanConcretePowder),
            "minecraft:purple_concrete_powder" => Some(Block::PurpleConcretePowder),
            "minecraft:blue_concrete_powder" => Some(Block::BlueConcretePowder),
            "minecraft:brown_concrete_powder" => Some(Block::BrownConcretePowder),
            "minecraft:green_concrete_powder" => Some(Block::GreenConcretePowder),
            "minecraft:red_concrete_powder" => Some(Block::RedConcretePowder),
            "minecraft:black_concrete_powder" => Some(Block::BlackConcretePowder),
            "minecraft:kelp" => {
                let data = KelpData {
                    age: match props["age"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        "5" => 5,
                        "6" => 6,
                        "7" => 7,
                        "8" => 8,
                        "9" => 9,
                        "10" => 10,
                        "11" => 11,
                        "12" => 12,
                        "13" => 13,
                        "14" => 14,
                        "15" => 15,
                        "16" => 16,
                        "17" => 17,
                        "18" => 18,
                        "19" => 19,
                        "20" => 20,
                        "21" => 21,
                        "22" => 22,
                        "23" => 23,
                        "24" => 24,
                        "25" => 25,
                        _ => return None,
                    },
                };
                Some(Block::Kelp(data))
            }
            "minecraft:kelp_plant" => Some(Block::KelpPlant),
            "minecraft:dried_kelp_block" => Some(Block::DriedKelpBlock),
            "minecraft:turtle_egg" => {
                let data = TurtleEggData {
                    eggs: match props["eggs"].as_str() {
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        _ => return None,
                    },
                    hatch: match props["hatch"].as_str() {
                        "0" => 0,
                        "1" => 1,
                        "2" => 2,
                        _ => return None,
                    },
                };
                Some(Block::TurtleEgg(data))
            }
            "minecraft:dead_tube_coral_block" => Some(Block::DeadTubeCoralBlock),
            "minecraft:dead_brain_coral_block" => Some(Block::DeadBrainCoralBlock),
            "minecraft:dead_bubble_coral_block" => Some(Block::DeadBubbleCoralBlock),
            "minecraft:dead_fire_coral_block" => Some(Block::DeadFireCoralBlock),
            "minecraft:dead_horn_coral_block" => Some(Block::DeadHornCoralBlock),
            "minecraft:tube_coral_block" => Some(Block::TubeCoralBlock),
            "minecraft:brain_coral_block" => Some(Block::BrainCoralBlock),
            "minecraft:bubble_coral_block" => Some(Block::BubbleCoralBlock),
            "minecraft:fire_coral_block" => Some(Block::FireCoralBlock),
            "minecraft:horn_coral_block" => Some(Block::HornCoralBlock),
            "minecraft:dead_tube_coral" => {
                let data = DeadTubeCoralData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DeadTubeCoral(data))
            }
            "minecraft:dead_brain_coral" => {
                let data = DeadBrainCoralData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DeadBrainCoral(data))
            }
            "minecraft:dead_bubble_coral" => {
                let data = DeadBubbleCoralData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DeadBubbleCoral(data))
            }
            "minecraft:dead_fire_coral" => {
                let data = DeadFireCoralData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DeadFireCoral(data))
            }
            "minecraft:dead_horn_coral" => {
                let data = DeadHornCoralData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DeadHornCoral(data))
            }
            "minecraft:tube_coral" => {
                let data = TubeCoralData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::TubeCoral(data))
            }
            "minecraft:brain_coral" => {
                let data = BrainCoralData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BrainCoral(data))
            }
            "minecraft:bubble_coral" => {
                let data = BubbleCoralData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BubbleCoral(data))
            }
            "minecraft:fire_coral" => {
                let data = FireCoralData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::FireCoral(data))
            }
            "minecraft:horn_coral" => {
                let data = HornCoralData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::HornCoral(data))
            }
            "minecraft:dead_tube_coral_wall_fan" => {
                let data = DeadTubeCoralWallFanData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DeadTubeCoralWallFan(data))
            }
            "minecraft:dead_brain_coral_wall_fan" => {
                let data = DeadBrainCoralWallFanData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DeadBrainCoralWallFan(data))
            }
            "minecraft:dead_bubble_coral_wall_fan" => {
                let data = DeadBubbleCoralWallFanData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DeadBubbleCoralWallFan(data))
            }
            "minecraft:dead_fire_coral_wall_fan" => {
                let data = DeadFireCoralWallFanData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DeadFireCoralWallFan(data))
            }
            "minecraft:dead_horn_coral_wall_fan" => {
                let data = DeadHornCoralWallFanData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DeadHornCoralWallFan(data))
            }
            "minecraft:tube_coral_wall_fan" => {
                let data = TubeCoralWallFanData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::TubeCoralWallFan(data))
            }
            "minecraft:brain_coral_wall_fan" => {
                let data = BrainCoralWallFanData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BrainCoralWallFan(data))
            }
            "minecraft:bubble_coral_wall_fan" => {
                let data = BubbleCoralWallFanData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BubbleCoralWallFan(data))
            }
            "minecraft:fire_coral_wall_fan" => {
                let data = FireCoralWallFanData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::FireCoralWallFan(data))
            }
            "minecraft:horn_coral_wall_fan" => {
                let data = HornCoralWallFanData {
                    facing: match props["facing"].as_str() {
                        "north" => Facing::North,
                        "south" => Facing::South,
                        "west" => Facing::West,
                        "east" => Facing::East,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::HornCoralWallFan(data))
            }
            "minecraft:dead_tube_coral_fan" => {
                let data = DeadTubeCoralFanData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DeadTubeCoralFan(data))
            }
            "minecraft:dead_brain_coral_fan" => {
                let data = DeadBrainCoralFanData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DeadBrainCoralFan(data))
            }
            "minecraft:dead_bubble_coral_fan" => {
                let data = DeadBubbleCoralFanData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DeadBubbleCoralFan(data))
            }
            "minecraft:dead_fire_coral_fan" => {
                let data = DeadFireCoralFanData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DeadFireCoralFan(data))
            }
            "minecraft:dead_horn_coral_fan" => {
                let data = DeadHornCoralFanData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::DeadHornCoralFan(data))
            }
            "minecraft:tube_coral_fan" => {
                let data = TubeCoralFanData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::TubeCoralFan(data))
            }
            "minecraft:brain_coral_fan" => {
                let data = BrainCoralFanData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BrainCoralFan(data))
            }
            "minecraft:bubble_coral_fan" => {
                let data = BubbleCoralFanData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BubbleCoralFan(data))
            }
            "minecraft:fire_coral_fan" => {
                let data = FireCoralFanData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::FireCoralFan(data))
            }
            "minecraft:horn_coral_fan" => {
                let data = HornCoralFanData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::HornCoralFan(data))
            }
            "minecraft:sea_pickle" => {
                let data = SeaPickleData {
                    pickles: match props["pickles"].as_str() {
                        "1" => 1,
                        "2" => 2,
                        "3" => 3,
                        "4" => 4,
                        _ => return None,
                    },
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::SeaPickle(data))
            }
            "minecraft:blue_ice" => Some(Block::BlueIce),
            "minecraft:conduit" => {
                let data = ConduitData {
                    waterlogged: match props["waterlogged"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::Conduit(data))
            }
            "minecraft:void_air" => Some(Block::VoidAir),
            "minecraft:cave_air" => Some(Block::CaveAir),
            "minecraft:bubble_column" => {
                let data = BubbleColumnData {
                    drag: match props["drag"].as_str() {
                        "true" => true,
                        "false" => false,
                        _ => return None,
                    },
                };
                Some(Block::BubbleColumn(data))
            }
            "minecraft:structure_block" => {
                let data = StructureBlockData {
                    mode: match props["mode"].as_str() {
                        "save" => StructureBlockMode::Save,
                        "load" => StructureBlockMode::Load,
                        "corner" => StructureBlockMode::Corner,
                        "data" => StructureBlockMode::Data,
                        _ => return None,
                    },
                };
                Some(Block::StructureBlock(data))
            }
            _ => None,
        }
    }
}
