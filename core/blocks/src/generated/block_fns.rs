use crate::*;
use std::collections::BTreeMap;
use std::str::FromStr;
impl BlockId {
    #[doc = "Returns an instance of `air` with default state values."]
    pub fn air() -> Self {
        let mut block = Self {
            kind: BlockKind::Air,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `stone` with default state values."]
    pub fn stone() -> Self {
        let mut block = Self {
            kind: BlockKind::Stone,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `granite` with default state values."]
    pub fn granite() -> Self {
        let mut block = Self {
            kind: BlockKind::Granite,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `polished_granite` with default state values."]
    pub fn polished_granite() -> Self {
        let mut block = Self {
            kind: BlockKind::PolishedGranite,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `diorite` with default state values."]
    pub fn diorite() -> Self {
        let mut block = Self {
            kind: BlockKind::Diorite,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `polished_diorite` with default state values."]
    pub fn polished_diorite() -> Self {
        let mut block = Self {
            kind: BlockKind::PolishedDiorite,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `andesite` with default state values."]
    pub fn andesite() -> Self {
        let mut block = Self {
            kind: BlockKind::Andesite,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `polished_andesite` with default state values."]
    pub fn polished_andesite() -> Self {
        let mut block = Self {
            kind: BlockKind::PolishedAndesite,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `grass_block` with default state values.\nThe default state values are as follows:\n* `snowy`: false\n"]
    pub fn grass_block() -> Self {
        let mut block = Self {
            kind: BlockKind::GrassBlock,
            state: 0,
        };
        block.set_snowy(false);
        block
    }
    #[doc = "Returns an instance of `dirt` with default state values."]
    pub fn dirt() -> Self {
        let mut block = Self {
            kind: BlockKind::Dirt,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `coarse_dirt` with default state values."]
    pub fn coarse_dirt() -> Self {
        let mut block = Self {
            kind: BlockKind::CoarseDirt,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `podzol` with default state values.\nThe default state values are as follows:\n* `snowy`: false\n"]
    pub fn podzol() -> Self {
        let mut block = Self {
            kind: BlockKind::Podzol,
            state: 0,
        };
        block.set_snowy(false);
        block
    }
    #[doc = "Returns an instance of `cobblestone` with default state values."]
    pub fn cobblestone() -> Self {
        let mut block = Self {
            kind: BlockKind::Cobblestone,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `oak_planks` with default state values."]
    pub fn oak_planks() -> Self {
        let mut block = Self {
            kind: BlockKind::OakPlanks,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `spruce_planks` with default state values."]
    pub fn spruce_planks() -> Self {
        let mut block = Self {
            kind: BlockKind::SprucePlanks,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `birch_planks` with default state values."]
    pub fn birch_planks() -> Self {
        let mut block = Self {
            kind: BlockKind::BirchPlanks,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `jungle_planks` with default state values."]
    pub fn jungle_planks() -> Self {
        let mut block = Self {
            kind: BlockKind::JunglePlanks,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `acacia_planks` with default state values."]
    pub fn acacia_planks() -> Self {
        let mut block = Self {
            kind: BlockKind::AcaciaPlanks,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `dark_oak_planks` with default state values."]
    pub fn dark_oak_planks() -> Self {
        let mut block = Self {
            kind: BlockKind::DarkOakPlanks,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `oak_sapling` with default state values.\nThe default state values are as follows:\n* `stage`: 0\n"]
    pub fn oak_sapling() -> Self {
        let mut block = Self {
            kind: BlockKind::OakSapling,
            state: 0,
        };
        block.set_stage(0i32);
        block
    }
    #[doc = "Returns an instance of `spruce_sapling` with default state values.\nThe default state values are as follows:\n* `stage`: 0\n"]
    pub fn spruce_sapling() -> Self {
        let mut block = Self {
            kind: BlockKind::SpruceSapling,
            state: 0,
        };
        block.set_stage(0i32);
        block
    }
    #[doc = "Returns an instance of `birch_sapling` with default state values.\nThe default state values are as follows:\n* `stage`: 0\n"]
    pub fn birch_sapling() -> Self {
        let mut block = Self {
            kind: BlockKind::BirchSapling,
            state: 0,
        };
        block.set_stage(0i32);
        block
    }
    #[doc = "Returns an instance of `jungle_sapling` with default state values.\nThe default state values are as follows:\n* `stage`: 0\n"]
    pub fn jungle_sapling() -> Self {
        let mut block = Self {
            kind: BlockKind::JungleSapling,
            state: 0,
        };
        block.set_stage(0i32);
        block
    }
    #[doc = "Returns an instance of `acacia_sapling` with default state values.\nThe default state values are as follows:\n* `stage`: 0\n"]
    pub fn acacia_sapling() -> Self {
        let mut block = Self {
            kind: BlockKind::AcaciaSapling,
            state: 0,
        };
        block.set_stage(0i32);
        block
    }
    #[doc = "Returns an instance of `dark_oak_sapling` with default state values.\nThe default state values are as follows:\n* `stage`: 0\n"]
    pub fn dark_oak_sapling() -> Self {
        let mut block = Self {
            kind: BlockKind::DarkOakSapling,
            state: 0,
        };
        block.set_stage(0i32);
        block
    }
    #[doc = "Returns an instance of `bedrock` with default state values."]
    pub fn bedrock() -> Self {
        let mut block = Self {
            kind: BlockKind::Bedrock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `water` with default state values.\nThe default state values are as follows:\n* `water_level`: 0\n"]
    pub fn water() -> Self {
        let mut block = Self {
            kind: BlockKind::Water,
            state: 0,
        };
        block.set_water_level(0i32);
        block
    }
    #[doc = "Returns an instance of `lava` with default state values.\nThe default state values are as follows:\n* `water_level`: 0\n"]
    pub fn lava() -> Self {
        let mut block = Self {
            kind: BlockKind::Lava,
            state: 0,
        };
        block.set_water_level(0i32);
        block
    }
    #[doc = "Returns an instance of `sand` with default state values."]
    pub fn sand() -> Self {
        let mut block = Self {
            kind: BlockKind::Sand,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `red_sand` with default state values."]
    pub fn red_sand() -> Self {
        let mut block = Self {
            kind: BlockKind::RedSand,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `gravel` with default state values."]
    pub fn gravel() -> Self {
        let mut block = Self {
            kind: BlockKind::Gravel,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `gold_ore` with default state values."]
    pub fn gold_ore() -> Self {
        let mut block = Self {
            kind: BlockKind::GoldOre,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `iron_ore` with default state values."]
    pub fn iron_ore() -> Self {
        let mut block = Self {
            kind: BlockKind::IronOre,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `coal_ore` with default state values."]
    pub fn coal_ore() -> Self {
        let mut block = Self {
            kind: BlockKind::CoalOre,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `oak_log` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn oak_log() -> Self {
        let mut block = Self {
            kind: BlockKind::OakLog,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `spruce_log` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn spruce_log() -> Self {
        let mut block = Self {
            kind: BlockKind::SpruceLog,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `birch_log` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn birch_log() -> Self {
        let mut block = Self {
            kind: BlockKind::BirchLog,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `jungle_log` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn jungle_log() -> Self {
        let mut block = Self {
            kind: BlockKind::JungleLog,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `acacia_log` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn acacia_log() -> Self {
        let mut block = Self {
            kind: BlockKind::AcaciaLog,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `dark_oak_log` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn dark_oak_log() -> Self {
        let mut block = Self {
            kind: BlockKind::DarkOakLog,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `stripped_spruce_log` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn stripped_spruce_log() -> Self {
        let mut block = Self {
            kind: BlockKind::StrippedSpruceLog,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `stripped_birch_log` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn stripped_birch_log() -> Self {
        let mut block = Self {
            kind: BlockKind::StrippedBirchLog,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `stripped_jungle_log` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn stripped_jungle_log() -> Self {
        let mut block = Self {
            kind: BlockKind::StrippedJungleLog,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `stripped_acacia_log` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn stripped_acacia_log() -> Self {
        let mut block = Self {
            kind: BlockKind::StrippedAcaciaLog,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `stripped_dark_oak_log` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn stripped_dark_oak_log() -> Self {
        let mut block = Self {
            kind: BlockKind::StrippedDarkOakLog,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `stripped_oak_log` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn stripped_oak_log() -> Self {
        let mut block = Self {
            kind: BlockKind::StrippedOakLog,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `oak_wood` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn oak_wood() -> Self {
        let mut block = Self {
            kind: BlockKind::OakWood,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `spruce_wood` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn spruce_wood() -> Self {
        let mut block = Self {
            kind: BlockKind::SpruceWood,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `birch_wood` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn birch_wood() -> Self {
        let mut block = Self {
            kind: BlockKind::BirchWood,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `jungle_wood` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn jungle_wood() -> Self {
        let mut block = Self {
            kind: BlockKind::JungleWood,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `acacia_wood` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn acacia_wood() -> Self {
        let mut block = Self {
            kind: BlockKind::AcaciaWood,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `dark_oak_wood` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn dark_oak_wood() -> Self {
        let mut block = Self {
            kind: BlockKind::DarkOakWood,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `stripped_oak_wood` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn stripped_oak_wood() -> Self {
        let mut block = Self {
            kind: BlockKind::StrippedOakWood,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `stripped_spruce_wood` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn stripped_spruce_wood() -> Self {
        let mut block = Self {
            kind: BlockKind::StrippedSpruceWood,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `stripped_birch_wood` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn stripped_birch_wood() -> Self {
        let mut block = Self {
            kind: BlockKind::StrippedBirchWood,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `stripped_jungle_wood` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn stripped_jungle_wood() -> Self {
        let mut block = Self {
            kind: BlockKind::StrippedJungleWood,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `stripped_acacia_wood` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn stripped_acacia_wood() -> Self {
        let mut block = Self {
            kind: BlockKind::StrippedAcaciaWood,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `stripped_dark_oak_wood` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn stripped_dark_oak_wood() -> Self {
        let mut block = Self {
            kind: BlockKind::StrippedDarkOakWood,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `oak_leaves` with default state values.\nThe default state values are as follows:\n* `distance`: 7\n* `persistent`: false\n"]
    pub fn oak_leaves() -> Self {
        let mut block = Self {
            kind: BlockKind::OakLeaves,
            state: 0,
        };
        block.set_distance(7i32);
        block.set_persistent(false);
        block
    }
    #[doc = "Returns an instance of `spruce_leaves` with default state values.\nThe default state values are as follows:\n* `distance`: 7\n* `persistent`: false\n"]
    pub fn spruce_leaves() -> Self {
        let mut block = Self {
            kind: BlockKind::SpruceLeaves,
            state: 0,
        };
        block.set_distance(7i32);
        block.set_persistent(false);
        block
    }
    #[doc = "Returns an instance of `birch_leaves` with default state values.\nThe default state values are as follows:\n* `distance`: 7\n* `persistent`: false\n"]
    pub fn birch_leaves() -> Self {
        let mut block = Self {
            kind: BlockKind::BirchLeaves,
            state: 0,
        };
        block.set_distance(7i32);
        block.set_persistent(false);
        block
    }
    #[doc = "Returns an instance of `jungle_leaves` with default state values.\nThe default state values are as follows:\n* `distance`: 7\n* `persistent`: false\n"]
    pub fn jungle_leaves() -> Self {
        let mut block = Self {
            kind: BlockKind::JungleLeaves,
            state: 0,
        };
        block.set_distance(7i32);
        block.set_persistent(false);
        block
    }
    #[doc = "Returns an instance of `acacia_leaves` with default state values.\nThe default state values are as follows:\n* `distance`: 7\n* `persistent`: false\n"]
    pub fn acacia_leaves() -> Self {
        let mut block = Self {
            kind: BlockKind::AcaciaLeaves,
            state: 0,
        };
        block.set_distance(7i32);
        block.set_persistent(false);
        block
    }
    #[doc = "Returns an instance of `dark_oak_leaves` with default state values.\nThe default state values are as follows:\n* `distance`: 7\n* `persistent`: false\n"]
    pub fn dark_oak_leaves() -> Self {
        let mut block = Self {
            kind: BlockKind::DarkOakLeaves,
            state: 0,
        };
        block.set_distance(7i32);
        block.set_persistent(false);
        block
    }
    #[doc = "Returns an instance of `sponge` with default state values."]
    pub fn sponge() -> Self {
        let mut block = Self {
            kind: BlockKind::Sponge,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `wet_sponge` with default state values."]
    pub fn wet_sponge() -> Self {
        let mut block = Self {
            kind: BlockKind::WetSponge,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `glass` with default state values."]
    pub fn glass() -> Self {
        let mut block = Self {
            kind: BlockKind::Glass,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `lapis_ore` with default state values."]
    pub fn lapis_ore() -> Self {
        let mut block = Self {
            kind: BlockKind::LapisOre,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `lapis_block` with default state values."]
    pub fn lapis_block() -> Self {
        let mut block = Self {
            kind: BlockKind::LapisBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `dispenser` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: north\n* `triggered`: false\n"]
    pub fn dispenser() -> Self {
        let mut block = Self {
            kind: BlockKind::Dispenser,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::North);
        block.set_triggered(false);
        block
    }
    #[doc = "Returns an instance of `sandstone` with default state values."]
    pub fn sandstone() -> Self {
        let mut block = Self {
            kind: BlockKind::Sandstone,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `chiseled_sandstone` with default state values."]
    pub fn chiseled_sandstone() -> Self {
        let mut block = Self {
            kind: BlockKind::ChiseledSandstone,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `cut_sandstone` with default state values."]
    pub fn cut_sandstone() -> Self {
        let mut block = Self {
            kind: BlockKind::CutSandstone,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `note_block` with default state values.\nThe default state values are as follows:\n* `instrument`: harp\n* `note`: 0\n* `powered`: false\n"]
    pub fn note_block() -> Self {
        let mut block = Self {
            kind: BlockKind::NoteBlock,
            state: 0,
        };
        block.set_instrument(Instrument::Harp);
        block.set_note(0i32);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `white_bed` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `occupied`: false\n* `part`: foot\n"]
    pub fn white_bed() -> Self {
        let mut block = Self {
            kind: BlockKind::WhiteBed,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_occupied(false);
        block.set_part(Part::Foot);
        block
    }
    #[doc = "Returns an instance of `orange_bed` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `occupied`: false\n* `part`: foot\n"]
    pub fn orange_bed() -> Self {
        let mut block = Self {
            kind: BlockKind::OrangeBed,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_occupied(false);
        block.set_part(Part::Foot);
        block
    }
    #[doc = "Returns an instance of `magenta_bed` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `occupied`: false\n* `part`: foot\n"]
    pub fn magenta_bed() -> Self {
        let mut block = Self {
            kind: BlockKind::MagentaBed,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_occupied(false);
        block.set_part(Part::Foot);
        block
    }
    #[doc = "Returns an instance of `light_blue_bed` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `occupied`: false\n* `part`: foot\n"]
    pub fn light_blue_bed() -> Self {
        let mut block = Self {
            kind: BlockKind::LightBlueBed,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_occupied(false);
        block.set_part(Part::Foot);
        block
    }
    #[doc = "Returns an instance of `yellow_bed` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `occupied`: false\n* `part`: foot\n"]
    pub fn yellow_bed() -> Self {
        let mut block = Self {
            kind: BlockKind::YellowBed,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_occupied(false);
        block.set_part(Part::Foot);
        block
    }
    #[doc = "Returns an instance of `lime_bed` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `occupied`: false\n* `part`: foot\n"]
    pub fn lime_bed() -> Self {
        let mut block = Self {
            kind: BlockKind::LimeBed,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_occupied(false);
        block.set_part(Part::Foot);
        block
    }
    #[doc = "Returns an instance of `pink_bed` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `occupied`: false\n* `part`: foot\n"]
    pub fn pink_bed() -> Self {
        let mut block = Self {
            kind: BlockKind::PinkBed,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_occupied(false);
        block.set_part(Part::Foot);
        block
    }
    #[doc = "Returns an instance of `gray_bed` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `occupied`: false\n* `part`: foot\n"]
    pub fn gray_bed() -> Self {
        let mut block = Self {
            kind: BlockKind::GrayBed,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_occupied(false);
        block.set_part(Part::Foot);
        block
    }
    #[doc = "Returns an instance of `light_gray_bed` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `occupied`: false\n* `part`: foot\n"]
    pub fn light_gray_bed() -> Self {
        let mut block = Self {
            kind: BlockKind::LightGrayBed,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_occupied(false);
        block.set_part(Part::Foot);
        block
    }
    #[doc = "Returns an instance of `cyan_bed` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `occupied`: false\n* `part`: foot\n"]
    pub fn cyan_bed() -> Self {
        let mut block = Self {
            kind: BlockKind::CyanBed,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_occupied(false);
        block.set_part(Part::Foot);
        block
    }
    #[doc = "Returns an instance of `purple_bed` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `occupied`: false\n* `part`: foot\n"]
    pub fn purple_bed() -> Self {
        let mut block = Self {
            kind: BlockKind::PurpleBed,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_occupied(false);
        block.set_part(Part::Foot);
        block
    }
    #[doc = "Returns an instance of `blue_bed` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `occupied`: false\n* `part`: foot\n"]
    pub fn blue_bed() -> Self {
        let mut block = Self {
            kind: BlockKind::BlueBed,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_occupied(false);
        block.set_part(Part::Foot);
        block
    }
    #[doc = "Returns an instance of `brown_bed` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `occupied`: false\n* `part`: foot\n"]
    pub fn brown_bed() -> Self {
        let mut block = Self {
            kind: BlockKind::BrownBed,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_occupied(false);
        block.set_part(Part::Foot);
        block
    }
    #[doc = "Returns an instance of `green_bed` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `occupied`: false\n* `part`: foot\n"]
    pub fn green_bed() -> Self {
        let mut block = Self {
            kind: BlockKind::GreenBed,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_occupied(false);
        block.set_part(Part::Foot);
        block
    }
    #[doc = "Returns an instance of `red_bed` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `occupied`: false\n* `part`: foot\n"]
    pub fn red_bed() -> Self {
        let mut block = Self {
            kind: BlockKind::RedBed,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_occupied(false);
        block.set_part(Part::Foot);
        block
    }
    #[doc = "Returns an instance of `black_bed` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `occupied`: false\n* `part`: foot\n"]
    pub fn black_bed() -> Self {
        let mut block = Self {
            kind: BlockKind::BlackBed,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_occupied(false);
        block.set_part(Part::Foot);
        block
    }
    #[doc = "Returns an instance of `powered_rail` with default state values.\nThe default state values are as follows:\n* `powered`: false\n* `powered_rail_shape`: north_south\n"]
    pub fn powered_rail() -> Self {
        let mut block = Self {
            kind: BlockKind::PoweredRail,
            state: 0,
        };
        block.set_powered(false);
        block.set_powered_rail_shape(PoweredRailShape::NorthSouth);
        block
    }
    #[doc = "Returns an instance of `detector_rail` with default state values.\nThe default state values are as follows:\n* `powered`: false\n* `powered_rail_shape`: north_south\n"]
    pub fn detector_rail() -> Self {
        let mut block = Self {
            kind: BlockKind::DetectorRail,
            state: 0,
        };
        block.set_powered(false);
        block.set_powered_rail_shape(PoweredRailShape::NorthSouth);
        block
    }
    #[doc = "Returns an instance of `sticky_piston` with default state values.\nThe default state values are as follows:\n* `extended`: false\n* `facing_cubic`: north\n"]
    pub fn sticky_piston() -> Self {
        let mut block = Self {
            kind: BlockKind::StickyPiston,
            state: 0,
        };
        block.set_extended(false);
        block.set_facing_cubic(FacingCubic::North);
        block
    }
    #[doc = "Returns an instance of `cobweb` with default state values."]
    pub fn cobweb() -> Self {
        let mut block = Self {
            kind: BlockKind::Cobweb,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `grass` with default state values."]
    pub fn grass() -> Self {
        let mut block = Self {
            kind: BlockKind::Grass,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `fern` with default state values."]
    pub fn fern() -> Self {
        let mut block = Self {
            kind: BlockKind::Fern,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `dead_bush` with default state values."]
    pub fn dead_bush() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadBush,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `seagrass` with default state values."]
    pub fn seagrass() -> Self {
        let mut block = Self {
            kind: BlockKind::Seagrass,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `tall_seagrass` with default state values.\nThe default state values are as follows:\n* `half_upper_lower`: lower\n"]
    pub fn tall_seagrass() -> Self {
        let mut block = Self {
            kind: BlockKind::TallSeagrass,
            state: 0,
        };
        block.set_half_upper_lower(HalfUpperLower::Lower);
        block
    }
    #[doc = "Returns an instance of `piston` with default state values.\nThe default state values are as follows:\n* `extended`: false\n* `facing_cubic`: north\n"]
    pub fn piston() -> Self {
        let mut block = Self {
            kind: BlockKind::Piston,
            state: 0,
        };
        block.set_extended(false);
        block.set_facing_cubic(FacingCubic::North);
        block
    }
    #[doc = "Returns an instance of `piston_head` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: north\n* `piston_kind`: normal\n* `short`: false\n"]
    pub fn piston_head() -> Self {
        let mut block = Self {
            kind: BlockKind::PistonHead,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::North);
        block.set_piston_kind(PistonKind::Normal);
        block.set_short(false);
        block
    }
    #[doc = "Returns an instance of `white_wool` with default state values."]
    pub fn white_wool() -> Self {
        let mut block = Self {
            kind: BlockKind::WhiteWool,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `orange_wool` with default state values."]
    pub fn orange_wool() -> Self {
        let mut block = Self {
            kind: BlockKind::OrangeWool,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `magenta_wool` with default state values."]
    pub fn magenta_wool() -> Self {
        let mut block = Self {
            kind: BlockKind::MagentaWool,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `light_blue_wool` with default state values."]
    pub fn light_blue_wool() -> Self {
        let mut block = Self {
            kind: BlockKind::LightBlueWool,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `yellow_wool` with default state values."]
    pub fn yellow_wool() -> Self {
        let mut block = Self {
            kind: BlockKind::YellowWool,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `lime_wool` with default state values."]
    pub fn lime_wool() -> Self {
        let mut block = Self {
            kind: BlockKind::LimeWool,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `pink_wool` with default state values."]
    pub fn pink_wool() -> Self {
        let mut block = Self {
            kind: BlockKind::PinkWool,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `gray_wool` with default state values."]
    pub fn gray_wool() -> Self {
        let mut block = Self {
            kind: BlockKind::GrayWool,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `light_gray_wool` with default state values."]
    pub fn light_gray_wool() -> Self {
        let mut block = Self {
            kind: BlockKind::LightGrayWool,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `cyan_wool` with default state values."]
    pub fn cyan_wool() -> Self {
        let mut block = Self {
            kind: BlockKind::CyanWool,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `purple_wool` with default state values."]
    pub fn purple_wool() -> Self {
        let mut block = Self {
            kind: BlockKind::PurpleWool,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `blue_wool` with default state values."]
    pub fn blue_wool() -> Self {
        let mut block = Self {
            kind: BlockKind::BlueWool,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `brown_wool` with default state values."]
    pub fn brown_wool() -> Self {
        let mut block = Self {
            kind: BlockKind::BrownWool,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `green_wool` with default state values."]
    pub fn green_wool() -> Self {
        let mut block = Self {
            kind: BlockKind::GreenWool,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `red_wool` with default state values."]
    pub fn red_wool() -> Self {
        let mut block = Self {
            kind: BlockKind::RedWool,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `black_wool` with default state values."]
    pub fn black_wool() -> Self {
        let mut block = Self {
            kind: BlockKind::BlackWool,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `moving_piston` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: north\n* `piston_kind`: normal\n"]
    pub fn moving_piston() -> Self {
        let mut block = Self {
            kind: BlockKind::MovingPiston,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::North);
        block.set_piston_kind(PistonKind::Normal);
        block
    }
    #[doc = "Returns an instance of `dandelion` with default state values."]
    pub fn dandelion() -> Self {
        let mut block = Self {
            kind: BlockKind::Dandelion,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `poppy` with default state values."]
    pub fn poppy() -> Self {
        let mut block = Self {
            kind: BlockKind::Poppy,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `blue_orchid` with default state values."]
    pub fn blue_orchid() -> Self {
        let mut block = Self {
            kind: BlockKind::BlueOrchid,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `allium` with default state values."]
    pub fn allium() -> Self {
        let mut block = Self {
            kind: BlockKind::Allium,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `azure_bluet` with default state values."]
    pub fn azure_bluet() -> Self {
        let mut block = Self {
            kind: BlockKind::AzureBluet,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `red_tulip` with default state values."]
    pub fn red_tulip() -> Self {
        let mut block = Self {
            kind: BlockKind::RedTulip,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `orange_tulip` with default state values."]
    pub fn orange_tulip() -> Self {
        let mut block = Self {
            kind: BlockKind::OrangeTulip,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `white_tulip` with default state values."]
    pub fn white_tulip() -> Self {
        let mut block = Self {
            kind: BlockKind::WhiteTulip,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `pink_tulip` with default state values."]
    pub fn pink_tulip() -> Self {
        let mut block = Self {
            kind: BlockKind::PinkTulip,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `oxeye_daisy` with default state values."]
    pub fn oxeye_daisy() -> Self {
        let mut block = Self {
            kind: BlockKind::OxeyeDaisy,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `brown_mushroom` with default state values."]
    pub fn brown_mushroom() -> Self {
        let mut block = Self {
            kind: BlockKind::BrownMushroom,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `red_mushroom` with default state values."]
    pub fn red_mushroom() -> Self {
        let mut block = Self {
            kind: BlockKind::RedMushroom,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `gold_block` with default state values."]
    pub fn gold_block() -> Self {
        let mut block = Self {
            kind: BlockKind::GoldBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `iron_block` with default state values."]
    pub fn iron_block() -> Self {
        let mut block = Self {
            kind: BlockKind::IronBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `bricks` with default state values."]
    pub fn bricks() -> Self {
        let mut block = Self {
            kind: BlockKind::Bricks,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `tnt` with default state values.\nThe default state values are as follows:\n* `unstable`: false\n"]
    pub fn tnt() -> Self {
        let mut block = Self {
            kind: BlockKind::Tnt,
            state: 0,
        };
        block.set_unstable(false);
        block
    }
    #[doc = "Returns an instance of `bookshelf` with default state values."]
    pub fn bookshelf() -> Self {
        let mut block = Self {
            kind: BlockKind::Bookshelf,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `mossy_cobblestone` with default state values."]
    pub fn mossy_cobblestone() -> Self {
        let mut block = Self {
            kind: BlockKind::MossyCobblestone,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `obsidian` with default state values."]
    pub fn obsidian() -> Self {
        let mut block = Self {
            kind: BlockKind::Obsidian,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `torch` with default state values."]
    pub fn torch() -> Self {
        let mut block = Self {
            kind: BlockKind::Torch,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `wall_torch` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn wall_torch() -> Self {
        let mut block = Self {
            kind: BlockKind::WallTorch,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `fire` with default state values.\nThe default state values are as follows:\n* `age_0_15`: 0\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `up`: false\n* `west_connected`: false\n"]
    pub fn fire() -> Self {
        let mut block = Self {
            kind: BlockKind::Fire,
            state: 0,
        };
        block.set_age_0_15(0i32);
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_up(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `spawner` with default state values."]
    pub fn spawner() -> Self {
        let mut block = Self {
            kind: BlockKind::Spawner,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `oak_stairs` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `stairs_shape`: straight\n* `waterlogged`: false\n"]
    pub fn oak_stairs() -> Self {
        let mut block = Self {
            kind: BlockKind::OakStairs,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_stairs_shape(StairsShape::Straight);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `chest` with default state values.\nThe default state values are as follows:\n* `chest_kind`: single\n* `facing_cardinal`: north\n* `waterlogged`: false\n"]
    pub fn chest() -> Self {
        let mut block = Self {
            kind: BlockKind::Chest,
            state: 0,
        };
        block.set_chest_kind(ChestKind::Single);
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `redstone_wire` with default state values.\nThe default state values are as follows:\n* `east_wire`: none\n* `north_wire`: none\n* `power`: 0\n* `south_wire`: none\n* `west_wire`: none\n"]
    pub fn redstone_wire() -> Self {
        let mut block = Self {
            kind: BlockKind::RedstoneWire,
            state: 0,
        };
        block.set_east_wire(EastWire::None);
        block.set_north_wire(NorthWire::None);
        block.set_power(0i32);
        block.set_south_wire(SouthWire::None);
        block.set_west_wire(WestWire::None);
        block
    }
    #[doc = "Returns an instance of `diamond_ore` with default state values."]
    pub fn diamond_ore() -> Self {
        let mut block = Self {
            kind: BlockKind::DiamondOre,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `diamond_block` with default state values."]
    pub fn diamond_block() -> Self {
        let mut block = Self {
            kind: BlockKind::DiamondBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `crafting_table` with default state values."]
    pub fn crafting_table() -> Self {
        let mut block = Self {
            kind: BlockKind::CraftingTable,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `wheat` with default state values.\nThe default state values are as follows:\n* `age_0_7`: 0\n"]
    pub fn wheat() -> Self {
        let mut block = Self {
            kind: BlockKind::Wheat,
            state: 0,
        };
        block.set_age_0_7(0i32);
        block
    }
    #[doc = "Returns an instance of `farmland` with default state values.\nThe default state values are as follows:\n* `moisture`: 0\n"]
    pub fn farmland() -> Self {
        let mut block = Self {
            kind: BlockKind::Farmland,
            state: 0,
        };
        block.set_moisture(0i32);
        block
    }
    #[doc = "Returns an instance of `furnace` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `lit`: false\n"]
    pub fn furnace() -> Self {
        let mut block = Self {
            kind: BlockKind::Furnace,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_lit(false);
        block
    }
    #[doc = "Returns an instance of `sign` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n* `waterlogged`: false\n"]
    pub fn sign() -> Self {
        let mut block = Self {
            kind: BlockKind::Sign,
            state: 0,
        };
        block.set_rotation(0i32);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `oak_door` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_upper_lower`: lower\n* `hinge`: left\n* `open`: false\n* `powered`: false\n"]
    pub fn oak_door() -> Self {
        let mut block = Self {
            kind: BlockKind::OakDoor,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_upper_lower(HalfUpperLower::Lower);
        block.set_hinge(Hinge::Left);
        block.set_open(false);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `ladder` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `waterlogged`: false\n"]
    pub fn ladder() -> Self {
        let mut block = Self {
            kind: BlockKind::Ladder,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `rail` with default state values.\nThe default state values are as follows:\n* `rail_shape`: north_south\n"]
    pub fn rail() -> Self {
        let mut block = Self {
            kind: BlockKind::Rail,
            state: 0,
        };
        block.set_rail_shape(RailShape::NorthSouth);
        block
    }
    #[doc = "Returns an instance of `cobblestone_stairs` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `stairs_shape`: straight\n* `waterlogged`: false\n"]
    pub fn cobblestone_stairs() -> Self {
        let mut block = Self {
            kind: BlockKind::CobblestoneStairs,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_stairs_shape(StairsShape::Straight);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `wall_sign` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `waterlogged`: false\n"]
    pub fn wall_sign() -> Self {
        let mut block = Self {
            kind: BlockKind::WallSign,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `lever` with default state values.\nThe default state values are as follows:\n* `face`: wall\n* `facing_cardinal`: north\n* `powered`: false\n"]
    pub fn lever() -> Self {
        let mut block = Self {
            kind: BlockKind::Lever,
            state: 0,
        };
        block.set_face(Face::Wall);
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `stone_pressure_plate` with default state values.\nThe default state values are as follows:\n* `powered`: false\n"]
    pub fn stone_pressure_plate() -> Self {
        let mut block = Self {
            kind: BlockKind::StonePressurePlate,
            state: 0,
        };
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `iron_door` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_upper_lower`: lower\n* `hinge`: left\n* `open`: false\n* `powered`: false\n"]
    pub fn iron_door() -> Self {
        let mut block = Self {
            kind: BlockKind::IronDoor,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_upper_lower(HalfUpperLower::Lower);
        block.set_hinge(Hinge::Left);
        block.set_open(false);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `oak_pressure_plate` with default state values.\nThe default state values are as follows:\n* `powered`: false\n"]
    pub fn oak_pressure_plate() -> Self {
        let mut block = Self {
            kind: BlockKind::OakPressurePlate,
            state: 0,
        };
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `spruce_pressure_plate` with default state values.\nThe default state values are as follows:\n* `powered`: false\n"]
    pub fn spruce_pressure_plate() -> Self {
        let mut block = Self {
            kind: BlockKind::SprucePressurePlate,
            state: 0,
        };
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `birch_pressure_plate` with default state values.\nThe default state values are as follows:\n* `powered`: false\n"]
    pub fn birch_pressure_plate() -> Self {
        let mut block = Self {
            kind: BlockKind::BirchPressurePlate,
            state: 0,
        };
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `jungle_pressure_plate` with default state values.\nThe default state values are as follows:\n* `powered`: false\n"]
    pub fn jungle_pressure_plate() -> Self {
        let mut block = Self {
            kind: BlockKind::JunglePressurePlate,
            state: 0,
        };
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `acacia_pressure_plate` with default state values.\nThe default state values are as follows:\n* `powered`: false\n"]
    pub fn acacia_pressure_plate() -> Self {
        let mut block = Self {
            kind: BlockKind::AcaciaPressurePlate,
            state: 0,
        };
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `dark_oak_pressure_plate` with default state values.\nThe default state values are as follows:\n* `powered`: false\n"]
    pub fn dark_oak_pressure_plate() -> Self {
        let mut block = Self {
            kind: BlockKind::DarkOakPressurePlate,
            state: 0,
        };
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `redstone_ore` with default state values.\nThe default state values are as follows:\n* `lit`: false\n"]
    pub fn redstone_ore() -> Self {
        let mut block = Self {
            kind: BlockKind::RedstoneOre,
            state: 0,
        };
        block.set_lit(false);
        block
    }
    #[doc = "Returns an instance of `redstone_torch` with default state values.\nThe default state values are as follows:\n* `lit`: true\n"]
    pub fn redstone_torch() -> Self {
        let mut block = Self {
            kind: BlockKind::RedstoneTorch,
            state: 0,
        };
        block.set_lit(true);
        block
    }
    #[doc = "Returns an instance of `redstone_wall_torch` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `lit`: true\n"]
    pub fn redstone_wall_torch() -> Self {
        let mut block = Self {
            kind: BlockKind::RedstoneWallTorch,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_lit(true);
        block
    }
    #[doc = "Returns an instance of `stone_button` with default state values.\nThe default state values are as follows:\n* `face`: wall\n* `facing_cardinal`: north\n* `powered`: false\n"]
    pub fn stone_button() -> Self {
        let mut block = Self {
            kind: BlockKind::StoneButton,
            state: 0,
        };
        block.set_face(Face::Wall);
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `snow` with default state values.\nThe default state values are as follows:\n* `layers`: 1\n"]
    pub fn snow() -> Self {
        let mut block = Self {
            kind: BlockKind::Snow,
            state: 0,
        };
        block.set_layers(1i32);
        block
    }
    #[doc = "Returns an instance of `ice` with default state values."]
    pub fn ice() -> Self {
        let mut block = Self {
            kind: BlockKind::Ice,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `snow_block` with default state values."]
    pub fn snow_block() -> Self {
        let mut block = Self {
            kind: BlockKind::SnowBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `cactus` with default state values.\nThe default state values are as follows:\n* `age_0_15`: 0\n"]
    pub fn cactus() -> Self {
        let mut block = Self {
            kind: BlockKind::Cactus,
            state: 0,
        };
        block.set_age_0_15(0i32);
        block
    }
    #[doc = "Returns an instance of `clay` with default state values."]
    pub fn clay() -> Self {
        let mut block = Self {
            kind: BlockKind::Clay,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `sugar_cane` with default state values.\nThe default state values are as follows:\n* `age_0_15`: 0\n"]
    pub fn sugar_cane() -> Self {
        let mut block = Self {
            kind: BlockKind::SugarCane,
            state: 0,
        };
        block.set_age_0_15(0i32);
        block
    }
    #[doc = "Returns an instance of `jukebox` with default state values.\nThe default state values are as follows:\n* `has_record`: false\n"]
    pub fn jukebox() -> Self {
        let mut block = Self {
            kind: BlockKind::Jukebox,
            state: 0,
        };
        block.set_has_record(false);
        block
    }
    #[doc = "Returns an instance of `oak_fence` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn oak_fence() -> Self {
        let mut block = Self {
            kind: BlockKind::OakFence,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `pumpkin` with default state values."]
    pub fn pumpkin() -> Self {
        let mut block = Self {
            kind: BlockKind::Pumpkin,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `netherrack` with default state values."]
    pub fn netherrack() -> Self {
        let mut block = Self {
            kind: BlockKind::Netherrack,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `soul_sand` with default state values."]
    pub fn soul_sand() -> Self {
        let mut block = Self {
            kind: BlockKind::SoulSand,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `glowstone` with default state values."]
    pub fn glowstone() -> Self {
        let mut block = Self {
            kind: BlockKind::Glowstone,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `nether_portal` with default state values.\nThe default state values are as follows:\n* `axis_xz`: x\n"]
    pub fn nether_portal() -> Self {
        let mut block = Self {
            kind: BlockKind::NetherPortal,
            state: 0,
        };
        block.set_axis_xz(AxisXz::X);
        block
    }
    #[doc = "Returns an instance of `carved_pumpkin` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn carved_pumpkin() -> Self {
        let mut block = Self {
            kind: BlockKind::CarvedPumpkin,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `jack_o_lantern` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn jack_o_lantern() -> Self {
        let mut block = Self {
            kind: BlockKind::JackOLantern,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `cake` with default state values.\nThe default state values are as follows:\n* `bites`: 0\n"]
    pub fn cake() -> Self {
        let mut block = Self {
            kind: BlockKind::Cake,
            state: 0,
        };
        block.set_bites(0i32);
        block
    }
    #[doc = "Returns an instance of `repeater` with default state values.\nThe default state values are as follows:\n* `delay`: 1\n* `facing_cardinal`: north\n* `locked`: false\n* `powered`: false\n"]
    pub fn repeater() -> Self {
        let mut block = Self {
            kind: BlockKind::Repeater,
            state: 0,
        };
        block.set_delay(1i32);
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_locked(false);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `white_stained_glass` with default state values."]
    pub fn white_stained_glass() -> Self {
        let mut block = Self {
            kind: BlockKind::WhiteStainedGlass,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `orange_stained_glass` with default state values."]
    pub fn orange_stained_glass() -> Self {
        let mut block = Self {
            kind: BlockKind::OrangeStainedGlass,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `magenta_stained_glass` with default state values."]
    pub fn magenta_stained_glass() -> Self {
        let mut block = Self {
            kind: BlockKind::MagentaStainedGlass,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `light_blue_stained_glass` with default state values."]
    pub fn light_blue_stained_glass() -> Self {
        let mut block = Self {
            kind: BlockKind::LightBlueStainedGlass,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `yellow_stained_glass` with default state values."]
    pub fn yellow_stained_glass() -> Self {
        let mut block = Self {
            kind: BlockKind::YellowStainedGlass,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `lime_stained_glass` with default state values."]
    pub fn lime_stained_glass() -> Self {
        let mut block = Self {
            kind: BlockKind::LimeStainedGlass,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `pink_stained_glass` with default state values."]
    pub fn pink_stained_glass() -> Self {
        let mut block = Self {
            kind: BlockKind::PinkStainedGlass,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `gray_stained_glass` with default state values."]
    pub fn gray_stained_glass() -> Self {
        let mut block = Self {
            kind: BlockKind::GrayStainedGlass,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `light_gray_stained_glass` with default state values."]
    pub fn light_gray_stained_glass() -> Self {
        let mut block = Self {
            kind: BlockKind::LightGrayStainedGlass,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `cyan_stained_glass` with default state values."]
    pub fn cyan_stained_glass() -> Self {
        let mut block = Self {
            kind: BlockKind::CyanStainedGlass,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `purple_stained_glass` with default state values."]
    pub fn purple_stained_glass() -> Self {
        let mut block = Self {
            kind: BlockKind::PurpleStainedGlass,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `blue_stained_glass` with default state values."]
    pub fn blue_stained_glass() -> Self {
        let mut block = Self {
            kind: BlockKind::BlueStainedGlass,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `brown_stained_glass` with default state values."]
    pub fn brown_stained_glass() -> Self {
        let mut block = Self {
            kind: BlockKind::BrownStainedGlass,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `green_stained_glass` with default state values."]
    pub fn green_stained_glass() -> Self {
        let mut block = Self {
            kind: BlockKind::GreenStainedGlass,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `red_stained_glass` with default state values."]
    pub fn red_stained_glass() -> Self {
        let mut block = Self {
            kind: BlockKind::RedStainedGlass,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `black_stained_glass` with default state values."]
    pub fn black_stained_glass() -> Self {
        let mut block = Self {
            kind: BlockKind::BlackStainedGlass,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `oak_trapdoor` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `open`: false\n* `powered`: false\n* `waterlogged`: false\n"]
    pub fn oak_trapdoor() -> Self {
        let mut block = Self {
            kind: BlockKind::OakTrapdoor,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_open(false);
        block.set_powered(false);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `spruce_trapdoor` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `open`: false\n* `powered`: false\n* `waterlogged`: false\n"]
    pub fn spruce_trapdoor() -> Self {
        let mut block = Self {
            kind: BlockKind::SpruceTrapdoor,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_open(false);
        block.set_powered(false);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `birch_trapdoor` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `open`: false\n* `powered`: false\n* `waterlogged`: false\n"]
    pub fn birch_trapdoor() -> Self {
        let mut block = Self {
            kind: BlockKind::BirchTrapdoor,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_open(false);
        block.set_powered(false);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `jungle_trapdoor` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `open`: false\n* `powered`: false\n* `waterlogged`: false\n"]
    pub fn jungle_trapdoor() -> Self {
        let mut block = Self {
            kind: BlockKind::JungleTrapdoor,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_open(false);
        block.set_powered(false);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `acacia_trapdoor` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `open`: false\n* `powered`: false\n* `waterlogged`: false\n"]
    pub fn acacia_trapdoor() -> Self {
        let mut block = Self {
            kind: BlockKind::AcaciaTrapdoor,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_open(false);
        block.set_powered(false);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `dark_oak_trapdoor` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `open`: false\n* `powered`: false\n* `waterlogged`: false\n"]
    pub fn dark_oak_trapdoor() -> Self {
        let mut block = Self {
            kind: BlockKind::DarkOakTrapdoor,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_open(false);
        block.set_powered(false);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `infested_stone` with default state values."]
    pub fn infested_stone() -> Self {
        let mut block = Self {
            kind: BlockKind::InfestedStone,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `infested_cobblestone` with default state values."]
    pub fn infested_cobblestone() -> Self {
        let mut block = Self {
            kind: BlockKind::InfestedCobblestone,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `infested_stone_bricks` with default state values."]
    pub fn infested_stone_bricks() -> Self {
        let mut block = Self {
            kind: BlockKind::InfestedStoneBricks,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `infested_mossy_stone_bricks` with default state values."]
    pub fn infested_mossy_stone_bricks() -> Self {
        let mut block = Self {
            kind: BlockKind::InfestedMossyStoneBricks,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `infested_cracked_stone_bricks` with default state values."]
    pub fn infested_cracked_stone_bricks() -> Self {
        let mut block = Self {
            kind: BlockKind::InfestedCrackedStoneBricks,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `infested_chiseled_stone_bricks` with default state values."]
    pub fn infested_chiseled_stone_bricks() -> Self {
        let mut block = Self {
            kind: BlockKind::InfestedChiseledStoneBricks,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `stone_bricks` with default state values."]
    pub fn stone_bricks() -> Self {
        let mut block = Self {
            kind: BlockKind::StoneBricks,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `mossy_stone_bricks` with default state values."]
    pub fn mossy_stone_bricks() -> Self {
        let mut block = Self {
            kind: BlockKind::MossyStoneBricks,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `cracked_stone_bricks` with default state values."]
    pub fn cracked_stone_bricks() -> Self {
        let mut block = Self {
            kind: BlockKind::CrackedStoneBricks,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `chiseled_stone_bricks` with default state values."]
    pub fn chiseled_stone_bricks() -> Self {
        let mut block = Self {
            kind: BlockKind::ChiseledStoneBricks,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `brown_mushroom_block` with default state values.\nThe default state values are as follows:\n* `down`: true\n* `east_connected`: true\n* `north_connected`: true\n* `south_connected`: true\n* `up`: true\n* `west_connected`: true\n"]
    pub fn brown_mushroom_block() -> Self {
        let mut block = Self {
            kind: BlockKind::BrownMushroomBlock,
            state: 0,
        };
        block.set_down(true);
        block.set_east_connected(true);
        block.set_north_connected(true);
        block.set_south_connected(true);
        block.set_up(true);
        block.set_west_connected(true);
        block
    }
    #[doc = "Returns an instance of `red_mushroom_block` with default state values.\nThe default state values are as follows:\n* `down`: true\n* `east_connected`: true\n* `north_connected`: true\n* `south_connected`: true\n* `up`: true\n* `west_connected`: true\n"]
    pub fn red_mushroom_block() -> Self {
        let mut block = Self {
            kind: BlockKind::RedMushroomBlock,
            state: 0,
        };
        block.set_down(true);
        block.set_east_connected(true);
        block.set_north_connected(true);
        block.set_south_connected(true);
        block.set_up(true);
        block.set_west_connected(true);
        block
    }
    #[doc = "Returns an instance of `mushroom_stem` with default state values.\nThe default state values are as follows:\n* `down`: true\n* `east_connected`: true\n* `north_connected`: true\n* `south_connected`: true\n* `up`: true\n* `west_connected`: true\n"]
    pub fn mushroom_stem() -> Self {
        let mut block = Self {
            kind: BlockKind::MushroomStem,
            state: 0,
        };
        block.set_down(true);
        block.set_east_connected(true);
        block.set_north_connected(true);
        block.set_south_connected(true);
        block.set_up(true);
        block.set_west_connected(true);
        block
    }
    #[doc = "Returns an instance of `iron_bars` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn iron_bars() -> Self {
        let mut block = Self {
            kind: BlockKind::IronBars,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `glass_pane` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn glass_pane() -> Self {
        let mut block = Self {
            kind: BlockKind::GlassPane,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `melon` with default state values."]
    pub fn melon() -> Self {
        let mut block = Self {
            kind: BlockKind::Melon,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `attached_pumpkin_stem` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn attached_pumpkin_stem() -> Self {
        let mut block = Self {
            kind: BlockKind::AttachedPumpkinStem,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `attached_melon_stem` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn attached_melon_stem() -> Self {
        let mut block = Self {
            kind: BlockKind::AttachedMelonStem,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `pumpkin_stem` with default state values.\nThe default state values are as follows:\n* `age_0_7`: 0\n"]
    pub fn pumpkin_stem() -> Self {
        let mut block = Self {
            kind: BlockKind::PumpkinStem,
            state: 0,
        };
        block.set_age_0_7(0i32);
        block
    }
    #[doc = "Returns an instance of `melon_stem` with default state values.\nThe default state values are as follows:\n* `age_0_7`: 0\n"]
    pub fn melon_stem() -> Self {
        let mut block = Self {
            kind: BlockKind::MelonStem,
            state: 0,
        };
        block.set_age_0_7(0i32);
        block
    }
    #[doc = "Returns an instance of `vine` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `up`: false\n* `west_connected`: false\n"]
    pub fn vine() -> Self {
        let mut block = Self {
            kind: BlockKind::Vine,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_up(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `oak_fence_gate` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `in_wall`: false\n* `open`: false\n* `powered`: false\n"]
    pub fn oak_fence_gate() -> Self {
        let mut block = Self {
            kind: BlockKind::OakFenceGate,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_in_wall(false);
        block.set_open(false);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `brick_stairs` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `stairs_shape`: straight\n* `waterlogged`: false\n"]
    pub fn brick_stairs() -> Self {
        let mut block = Self {
            kind: BlockKind::BrickStairs,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_stairs_shape(StairsShape::Straight);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `stone_brick_stairs` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `stairs_shape`: straight\n* `waterlogged`: false\n"]
    pub fn stone_brick_stairs() -> Self {
        let mut block = Self {
            kind: BlockKind::StoneBrickStairs,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_stairs_shape(StairsShape::Straight);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `mycelium` with default state values.\nThe default state values are as follows:\n* `snowy`: false\n"]
    pub fn mycelium() -> Self {
        let mut block = Self {
            kind: BlockKind::Mycelium,
            state: 0,
        };
        block.set_snowy(false);
        block
    }
    #[doc = "Returns an instance of `lily_pad` with default state values."]
    pub fn lily_pad() -> Self {
        let mut block = Self {
            kind: BlockKind::LilyPad,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `nether_bricks` with default state values."]
    pub fn nether_bricks() -> Self {
        let mut block = Self {
            kind: BlockKind::NetherBricks,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `nether_brick_fence` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn nether_brick_fence() -> Self {
        let mut block = Self {
            kind: BlockKind::NetherBrickFence,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `nether_brick_stairs` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `stairs_shape`: straight\n* `waterlogged`: false\n"]
    pub fn nether_brick_stairs() -> Self {
        let mut block = Self {
            kind: BlockKind::NetherBrickStairs,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_stairs_shape(StairsShape::Straight);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `nether_wart` with default state values.\nThe default state values are as follows:\n* `age_0_3`: 0\n"]
    pub fn nether_wart() -> Self {
        let mut block = Self {
            kind: BlockKind::NetherWart,
            state: 0,
        };
        block.set_age_0_3(0i32);
        block
    }
    #[doc = "Returns an instance of `enchanting_table` with default state values."]
    pub fn enchanting_table() -> Self {
        let mut block = Self {
            kind: BlockKind::EnchantingTable,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `brewing_stand` with default state values.\nThe default state values are as follows:\n* `has_bottle_0`: false\n* `has_bottle_1`: false\n* `has_bottle_2`: false\n"]
    pub fn brewing_stand() -> Self {
        let mut block = Self {
            kind: BlockKind::BrewingStand,
            state: 0,
        };
        block.set_has_bottle_0(false);
        block.set_has_bottle_1(false);
        block.set_has_bottle_2(false);
        block
    }
    #[doc = "Returns an instance of `cauldron` with default state values.\nThe default state values are as follows:\n* `cauldron_level`: 0\n"]
    pub fn cauldron() -> Self {
        let mut block = Self {
            kind: BlockKind::Cauldron,
            state: 0,
        };
        block.set_cauldron_level(0i32);
        block
    }
    #[doc = "Returns an instance of `end_portal` with default state values."]
    pub fn end_portal() -> Self {
        let mut block = Self {
            kind: BlockKind::EndPortal,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `end_portal_frame` with default state values.\nThe default state values are as follows:\n* `eye`: false\n* `facing_cardinal`: north\n"]
    pub fn end_portal_frame() -> Self {
        let mut block = Self {
            kind: BlockKind::EndPortalFrame,
            state: 0,
        };
        block.set_eye(false);
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `end_stone` with default state values."]
    pub fn end_stone() -> Self {
        let mut block = Self {
            kind: BlockKind::EndStone,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `dragon_egg` with default state values."]
    pub fn dragon_egg() -> Self {
        let mut block = Self {
            kind: BlockKind::DragonEgg,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `redstone_lamp` with default state values.\nThe default state values are as follows:\n* `lit`: false\n"]
    pub fn redstone_lamp() -> Self {
        let mut block = Self {
            kind: BlockKind::RedstoneLamp,
            state: 0,
        };
        block.set_lit(false);
        block
    }
    #[doc = "Returns an instance of `cocoa` with default state values.\nThe default state values are as follows:\n* `age_0_2`: 0\n* `facing_cardinal`: north\n"]
    pub fn cocoa() -> Self {
        let mut block = Self {
            kind: BlockKind::Cocoa,
            state: 0,
        };
        block.set_age_0_2(0i32);
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `sandstone_stairs` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `stairs_shape`: straight\n* `waterlogged`: false\n"]
    pub fn sandstone_stairs() -> Self {
        let mut block = Self {
            kind: BlockKind::SandstoneStairs,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_stairs_shape(StairsShape::Straight);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `emerald_ore` with default state values."]
    pub fn emerald_ore() -> Self {
        let mut block = Self {
            kind: BlockKind::EmeraldOre,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `ender_chest` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `waterlogged`: false\n"]
    pub fn ender_chest() -> Self {
        let mut block = Self {
            kind: BlockKind::EnderChest,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `tripwire_hook` with default state values.\nThe default state values are as follows:\n* `attached`: false\n* `facing_cardinal`: north\n* `powered`: false\n"]
    pub fn tripwire_hook() -> Self {
        let mut block = Self {
            kind: BlockKind::TripwireHook,
            state: 0,
        };
        block.set_attached(false);
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `tripwire` with default state values.\nThe default state values are as follows:\n* `attached`: false\n* `disarmed`: false\n* `east_connected`: false\n* `north_connected`: false\n* `powered`: false\n* `south_connected`: false\n* `west_connected`: false\n"]
    pub fn tripwire() -> Self {
        let mut block = Self {
            kind: BlockKind::Tripwire,
            state: 0,
        };
        block.set_attached(false);
        block.set_disarmed(false);
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_powered(false);
        block.set_south_connected(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `emerald_block` with default state values."]
    pub fn emerald_block() -> Self {
        let mut block = Self {
            kind: BlockKind::EmeraldBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `spruce_stairs` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `stairs_shape`: straight\n* `waterlogged`: false\n"]
    pub fn spruce_stairs() -> Self {
        let mut block = Self {
            kind: BlockKind::SpruceStairs,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_stairs_shape(StairsShape::Straight);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `birch_stairs` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `stairs_shape`: straight\n* `waterlogged`: false\n"]
    pub fn birch_stairs() -> Self {
        let mut block = Self {
            kind: BlockKind::BirchStairs,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_stairs_shape(StairsShape::Straight);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `jungle_stairs` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `stairs_shape`: straight\n* `waterlogged`: false\n"]
    pub fn jungle_stairs() -> Self {
        let mut block = Self {
            kind: BlockKind::JungleStairs,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_stairs_shape(StairsShape::Straight);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `command_block` with default state values.\nThe default state values are as follows:\n* `conditional`: false\n* `facing_cubic`: north\n"]
    pub fn command_block() -> Self {
        let mut block = Self {
            kind: BlockKind::CommandBlock,
            state: 0,
        };
        block.set_conditional(false);
        block.set_facing_cubic(FacingCubic::North);
        block
    }
    #[doc = "Returns an instance of `beacon` with default state values."]
    pub fn beacon() -> Self {
        let mut block = Self {
            kind: BlockKind::Beacon,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `cobblestone_wall` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `up`: true\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn cobblestone_wall() -> Self {
        let mut block = Self {
            kind: BlockKind::CobblestoneWall,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_up(true);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `mossy_cobblestone_wall` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `up`: true\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn mossy_cobblestone_wall() -> Self {
        let mut block = Self {
            kind: BlockKind::MossyCobblestoneWall,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_up(true);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `flower_pot` with default state values."]
    pub fn flower_pot() -> Self {
        let mut block = Self {
            kind: BlockKind::FlowerPot,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_oak_sapling` with default state values."]
    pub fn potted_oak_sapling() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedOakSapling,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_spruce_sapling` with default state values."]
    pub fn potted_spruce_sapling() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedSpruceSapling,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_birch_sapling` with default state values."]
    pub fn potted_birch_sapling() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedBirchSapling,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_jungle_sapling` with default state values."]
    pub fn potted_jungle_sapling() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedJungleSapling,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_acacia_sapling` with default state values."]
    pub fn potted_acacia_sapling() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedAcaciaSapling,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_dark_oak_sapling` with default state values."]
    pub fn potted_dark_oak_sapling() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedDarkOakSapling,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_fern` with default state values."]
    pub fn potted_fern() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedFern,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_dandelion` with default state values."]
    pub fn potted_dandelion() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedDandelion,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_poppy` with default state values."]
    pub fn potted_poppy() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedPoppy,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_blue_orchid` with default state values."]
    pub fn potted_blue_orchid() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedBlueOrchid,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_allium` with default state values."]
    pub fn potted_allium() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedAllium,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_azure_bluet` with default state values."]
    pub fn potted_azure_bluet() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedAzureBluet,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_red_tulip` with default state values."]
    pub fn potted_red_tulip() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedRedTulip,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_orange_tulip` with default state values."]
    pub fn potted_orange_tulip() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedOrangeTulip,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_white_tulip` with default state values."]
    pub fn potted_white_tulip() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedWhiteTulip,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_pink_tulip` with default state values."]
    pub fn potted_pink_tulip() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedPinkTulip,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_oxeye_daisy` with default state values."]
    pub fn potted_oxeye_daisy() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedOxeyeDaisy,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_red_mushroom` with default state values."]
    pub fn potted_red_mushroom() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedRedMushroom,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_brown_mushroom` with default state values."]
    pub fn potted_brown_mushroom() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedBrownMushroom,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_dead_bush` with default state values."]
    pub fn potted_dead_bush() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedDeadBush,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `potted_cactus` with default state values."]
    pub fn potted_cactus() -> Self {
        let mut block = Self {
            kind: BlockKind::PottedCactus,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `carrots` with default state values.\nThe default state values are as follows:\n* `age_0_7`: 0\n"]
    pub fn carrots() -> Self {
        let mut block = Self {
            kind: BlockKind::Carrots,
            state: 0,
        };
        block.set_age_0_7(0i32);
        block
    }
    #[doc = "Returns an instance of `potatoes` with default state values.\nThe default state values are as follows:\n* `age_0_7`: 0\n"]
    pub fn potatoes() -> Self {
        let mut block = Self {
            kind: BlockKind::Potatoes,
            state: 0,
        };
        block.set_age_0_7(0i32);
        block
    }
    #[doc = "Returns an instance of `oak_button` with default state values.\nThe default state values are as follows:\n* `face`: wall\n* `facing_cardinal`: north\n* `powered`: false\n"]
    pub fn oak_button() -> Self {
        let mut block = Self {
            kind: BlockKind::OakButton,
            state: 0,
        };
        block.set_face(Face::Wall);
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `spruce_button` with default state values.\nThe default state values are as follows:\n* `face`: wall\n* `facing_cardinal`: north\n* `powered`: false\n"]
    pub fn spruce_button() -> Self {
        let mut block = Self {
            kind: BlockKind::SpruceButton,
            state: 0,
        };
        block.set_face(Face::Wall);
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `birch_button` with default state values.\nThe default state values are as follows:\n* `face`: wall\n* `facing_cardinal`: north\n* `powered`: false\n"]
    pub fn birch_button() -> Self {
        let mut block = Self {
            kind: BlockKind::BirchButton,
            state: 0,
        };
        block.set_face(Face::Wall);
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `jungle_button` with default state values.\nThe default state values are as follows:\n* `face`: wall\n* `facing_cardinal`: north\n* `powered`: false\n"]
    pub fn jungle_button() -> Self {
        let mut block = Self {
            kind: BlockKind::JungleButton,
            state: 0,
        };
        block.set_face(Face::Wall);
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `acacia_button` with default state values.\nThe default state values are as follows:\n* `face`: wall\n* `facing_cardinal`: north\n* `powered`: false\n"]
    pub fn acacia_button() -> Self {
        let mut block = Self {
            kind: BlockKind::AcaciaButton,
            state: 0,
        };
        block.set_face(Face::Wall);
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `dark_oak_button` with default state values.\nThe default state values are as follows:\n* `face`: wall\n* `facing_cardinal`: north\n* `powered`: false\n"]
    pub fn dark_oak_button() -> Self {
        let mut block = Self {
            kind: BlockKind::DarkOakButton,
            state: 0,
        };
        block.set_face(Face::Wall);
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `skeleton_wall_skull` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn skeleton_wall_skull() -> Self {
        let mut block = Self {
            kind: BlockKind::SkeletonWallSkull,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `skeleton_skull` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn skeleton_skull() -> Self {
        let mut block = Self {
            kind: BlockKind::SkeletonSkull,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `wither_skeleton_wall_skull` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn wither_skeleton_wall_skull() -> Self {
        let mut block = Self {
            kind: BlockKind::WitherSkeletonWallSkull,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `wither_skeleton_skull` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn wither_skeleton_skull() -> Self {
        let mut block = Self {
            kind: BlockKind::WitherSkeletonSkull,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `zombie_wall_head` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn zombie_wall_head() -> Self {
        let mut block = Self {
            kind: BlockKind::ZombieWallHead,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `zombie_head` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn zombie_head() -> Self {
        let mut block = Self {
            kind: BlockKind::ZombieHead,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `player_wall_head` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn player_wall_head() -> Self {
        let mut block = Self {
            kind: BlockKind::PlayerWallHead,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `player_head` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn player_head() -> Self {
        let mut block = Self {
            kind: BlockKind::PlayerHead,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `creeper_wall_head` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn creeper_wall_head() -> Self {
        let mut block = Self {
            kind: BlockKind::CreeperWallHead,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `creeper_head` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn creeper_head() -> Self {
        let mut block = Self {
            kind: BlockKind::CreeperHead,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `dragon_wall_head` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn dragon_wall_head() -> Self {
        let mut block = Self {
            kind: BlockKind::DragonWallHead,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `dragon_head` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn dragon_head() -> Self {
        let mut block = Self {
            kind: BlockKind::DragonHead,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `anvil` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn anvil() -> Self {
        let mut block = Self {
            kind: BlockKind::Anvil,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `chipped_anvil` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn chipped_anvil() -> Self {
        let mut block = Self {
            kind: BlockKind::ChippedAnvil,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `damaged_anvil` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn damaged_anvil() -> Self {
        let mut block = Self {
            kind: BlockKind::DamagedAnvil,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `trapped_chest` with default state values.\nThe default state values are as follows:\n* `chest_kind`: single\n* `facing_cardinal`: north\n* `waterlogged`: false\n"]
    pub fn trapped_chest() -> Self {
        let mut block = Self {
            kind: BlockKind::TrappedChest,
            state: 0,
        };
        block.set_chest_kind(ChestKind::Single);
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `light_weighted_pressure_plate` with default state values.\nThe default state values are as follows:\n* `power`: 0\n"]
    pub fn light_weighted_pressure_plate() -> Self {
        let mut block = Self {
            kind: BlockKind::LightWeightedPressurePlate,
            state: 0,
        };
        block.set_power(0i32);
        block
    }
    #[doc = "Returns an instance of `heavy_weighted_pressure_plate` with default state values.\nThe default state values are as follows:\n* `power`: 0\n"]
    pub fn heavy_weighted_pressure_plate() -> Self {
        let mut block = Self {
            kind: BlockKind::HeavyWeightedPressurePlate,
            state: 0,
        };
        block.set_power(0i32);
        block
    }
    #[doc = "Returns an instance of `comparator` with default state values.\nThe default state values are as follows:\n* `comparator_mode`: compare\n* `facing_cardinal`: north\n* `powered`: false\n"]
    pub fn comparator() -> Self {
        let mut block = Self {
            kind: BlockKind::Comparator,
            state: 0,
        };
        block.set_comparator_mode(ComparatorMode::Compare);
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `daylight_detector` with default state values.\nThe default state values are as follows:\n* `inverted`: false\n* `power`: 0\n"]
    pub fn daylight_detector() -> Self {
        let mut block = Self {
            kind: BlockKind::DaylightDetector,
            state: 0,
        };
        block.set_inverted(false);
        block.set_power(0i32);
        block
    }
    #[doc = "Returns an instance of `redstone_block` with default state values."]
    pub fn redstone_block() -> Self {
        let mut block = Self {
            kind: BlockKind::RedstoneBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `nether_quartz_ore` with default state values."]
    pub fn nether_quartz_ore() -> Self {
        let mut block = Self {
            kind: BlockKind::NetherQuartzOre,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `hopper` with default state values.\nThe default state values are as follows:\n* `enabled`: true\n* `facing_cardinal_and_down`: down\n"]
    pub fn hopper() -> Self {
        let mut block = Self {
            kind: BlockKind::Hopper,
            state: 0,
        };
        block.set_enabled(true);
        block.set_facing_cardinal_and_down(FacingCardinalAndDown::Down);
        block
    }
    #[doc = "Returns an instance of `quartz_block` with default state values."]
    pub fn quartz_block() -> Self {
        let mut block = Self {
            kind: BlockKind::QuartzBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `chiseled_quartz_block` with default state values."]
    pub fn chiseled_quartz_block() -> Self {
        let mut block = Self {
            kind: BlockKind::ChiseledQuartzBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `quartz_pillar` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn quartz_pillar() -> Self {
        let mut block = Self {
            kind: BlockKind::QuartzPillar,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `quartz_stairs` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `stairs_shape`: straight\n* `waterlogged`: false\n"]
    pub fn quartz_stairs() -> Self {
        let mut block = Self {
            kind: BlockKind::QuartzStairs,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_stairs_shape(StairsShape::Straight);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `activator_rail` with default state values.\nThe default state values are as follows:\n* `powered`: false\n* `powered_rail_shape`: north_south\n"]
    pub fn activator_rail() -> Self {
        let mut block = Self {
            kind: BlockKind::ActivatorRail,
            state: 0,
        };
        block.set_powered(false);
        block.set_powered_rail_shape(PoweredRailShape::NorthSouth);
        block
    }
    #[doc = "Returns an instance of `dropper` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: north\n* `triggered`: false\n"]
    pub fn dropper() -> Self {
        let mut block = Self {
            kind: BlockKind::Dropper,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::North);
        block.set_triggered(false);
        block
    }
    #[doc = "Returns an instance of `white_terracotta` with default state values."]
    pub fn white_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::WhiteTerracotta,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `orange_terracotta` with default state values."]
    pub fn orange_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::OrangeTerracotta,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `magenta_terracotta` with default state values."]
    pub fn magenta_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::MagentaTerracotta,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `light_blue_terracotta` with default state values."]
    pub fn light_blue_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::LightBlueTerracotta,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `yellow_terracotta` with default state values."]
    pub fn yellow_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::YellowTerracotta,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `lime_terracotta` with default state values."]
    pub fn lime_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::LimeTerracotta,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `pink_terracotta` with default state values."]
    pub fn pink_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::PinkTerracotta,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `gray_terracotta` with default state values."]
    pub fn gray_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::GrayTerracotta,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `light_gray_terracotta` with default state values."]
    pub fn light_gray_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::LightGrayTerracotta,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `cyan_terracotta` with default state values."]
    pub fn cyan_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::CyanTerracotta,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `purple_terracotta` with default state values."]
    pub fn purple_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::PurpleTerracotta,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `blue_terracotta` with default state values."]
    pub fn blue_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::BlueTerracotta,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `brown_terracotta` with default state values."]
    pub fn brown_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::BrownTerracotta,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `green_terracotta` with default state values."]
    pub fn green_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::GreenTerracotta,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `red_terracotta` with default state values."]
    pub fn red_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::RedTerracotta,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `black_terracotta` with default state values."]
    pub fn black_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::BlackTerracotta,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `white_stained_glass_pane` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn white_stained_glass_pane() -> Self {
        let mut block = Self {
            kind: BlockKind::WhiteStainedGlassPane,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `orange_stained_glass_pane` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn orange_stained_glass_pane() -> Self {
        let mut block = Self {
            kind: BlockKind::OrangeStainedGlassPane,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `magenta_stained_glass_pane` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn magenta_stained_glass_pane() -> Self {
        let mut block = Self {
            kind: BlockKind::MagentaStainedGlassPane,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `light_blue_stained_glass_pane` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn light_blue_stained_glass_pane() -> Self {
        let mut block = Self {
            kind: BlockKind::LightBlueStainedGlassPane,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `yellow_stained_glass_pane` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn yellow_stained_glass_pane() -> Self {
        let mut block = Self {
            kind: BlockKind::YellowStainedGlassPane,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `lime_stained_glass_pane` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn lime_stained_glass_pane() -> Self {
        let mut block = Self {
            kind: BlockKind::LimeStainedGlassPane,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `pink_stained_glass_pane` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn pink_stained_glass_pane() -> Self {
        let mut block = Self {
            kind: BlockKind::PinkStainedGlassPane,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `gray_stained_glass_pane` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn gray_stained_glass_pane() -> Self {
        let mut block = Self {
            kind: BlockKind::GrayStainedGlassPane,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `light_gray_stained_glass_pane` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn light_gray_stained_glass_pane() -> Self {
        let mut block = Self {
            kind: BlockKind::LightGrayStainedGlassPane,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `cyan_stained_glass_pane` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn cyan_stained_glass_pane() -> Self {
        let mut block = Self {
            kind: BlockKind::CyanStainedGlassPane,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `purple_stained_glass_pane` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn purple_stained_glass_pane() -> Self {
        let mut block = Self {
            kind: BlockKind::PurpleStainedGlassPane,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `blue_stained_glass_pane` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn blue_stained_glass_pane() -> Self {
        let mut block = Self {
            kind: BlockKind::BlueStainedGlassPane,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `brown_stained_glass_pane` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn brown_stained_glass_pane() -> Self {
        let mut block = Self {
            kind: BlockKind::BrownStainedGlassPane,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `green_stained_glass_pane` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn green_stained_glass_pane() -> Self {
        let mut block = Self {
            kind: BlockKind::GreenStainedGlassPane,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `red_stained_glass_pane` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn red_stained_glass_pane() -> Self {
        let mut block = Self {
            kind: BlockKind::RedStainedGlassPane,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `black_stained_glass_pane` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn black_stained_glass_pane() -> Self {
        let mut block = Self {
            kind: BlockKind::BlackStainedGlassPane,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `acacia_stairs` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `stairs_shape`: straight\n* `waterlogged`: false\n"]
    pub fn acacia_stairs() -> Self {
        let mut block = Self {
            kind: BlockKind::AcaciaStairs,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_stairs_shape(StairsShape::Straight);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `dark_oak_stairs` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `stairs_shape`: straight\n* `waterlogged`: false\n"]
    pub fn dark_oak_stairs() -> Self {
        let mut block = Self {
            kind: BlockKind::DarkOakStairs,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_stairs_shape(StairsShape::Straight);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `slime_block` with default state values."]
    pub fn slime_block() -> Self {
        let mut block = Self {
            kind: BlockKind::SlimeBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `barrier` with default state values."]
    pub fn barrier() -> Self {
        let mut block = Self {
            kind: BlockKind::Barrier,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `iron_trapdoor` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `open`: false\n* `powered`: false\n* `waterlogged`: false\n"]
    pub fn iron_trapdoor() -> Self {
        let mut block = Self {
            kind: BlockKind::IronTrapdoor,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_open(false);
        block.set_powered(false);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `prismarine` with default state values."]
    pub fn prismarine() -> Self {
        let mut block = Self {
            kind: BlockKind::Prismarine,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `prismarine_bricks` with default state values."]
    pub fn prismarine_bricks() -> Self {
        let mut block = Self {
            kind: BlockKind::PrismarineBricks,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `dark_prismarine` with default state values."]
    pub fn dark_prismarine() -> Self {
        let mut block = Self {
            kind: BlockKind::DarkPrismarine,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `prismarine_stairs` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `stairs_shape`: straight\n* `waterlogged`: false\n"]
    pub fn prismarine_stairs() -> Self {
        let mut block = Self {
            kind: BlockKind::PrismarineStairs,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_stairs_shape(StairsShape::Straight);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `prismarine_brick_stairs` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `stairs_shape`: straight\n* `waterlogged`: false\n"]
    pub fn prismarine_brick_stairs() -> Self {
        let mut block = Self {
            kind: BlockKind::PrismarineBrickStairs,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_stairs_shape(StairsShape::Straight);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `dark_prismarine_stairs` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `stairs_shape`: straight\n* `waterlogged`: false\n"]
    pub fn dark_prismarine_stairs() -> Self {
        let mut block = Self {
            kind: BlockKind::DarkPrismarineStairs,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_stairs_shape(StairsShape::Straight);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `prismarine_slab` with default state values.\nThe default state values are as follows:\n* `slab_kind`: bottom\n* `waterlogged`: false\n"]
    pub fn prismarine_slab() -> Self {
        let mut block = Self {
            kind: BlockKind::PrismarineSlab,
            state: 0,
        };
        block.set_slab_kind(SlabKind::Bottom);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `prismarine_brick_slab` with default state values.\nThe default state values are as follows:\n* `slab_kind`: bottom\n* `waterlogged`: false\n"]
    pub fn prismarine_brick_slab() -> Self {
        let mut block = Self {
            kind: BlockKind::PrismarineBrickSlab,
            state: 0,
        };
        block.set_slab_kind(SlabKind::Bottom);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `dark_prismarine_slab` with default state values.\nThe default state values are as follows:\n* `slab_kind`: bottom\n* `waterlogged`: false\n"]
    pub fn dark_prismarine_slab() -> Self {
        let mut block = Self {
            kind: BlockKind::DarkPrismarineSlab,
            state: 0,
        };
        block.set_slab_kind(SlabKind::Bottom);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `sea_lantern` with default state values."]
    pub fn sea_lantern() -> Self {
        let mut block = Self {
            kind: BlockKind::SeaLantern,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `hay_block` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn hay_block() -> Self {
        let mut block = Self {
            kind: BlockKind::HayBlock,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `white_carpet` with default state values."]
    pub fn white_carpet() -> Self {
        let mut block = Self {
            kind: BlockKind::WhiteCarpet,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `orange_carpet` with default state values."]
    pub fn orange_carpet() -> Self {
        let mut block = Self {
            kind: BlockKind::OrangeCarpet,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `magenta_carpet` with default state values."]
    pub fn magenta_carpet() -> Self {
        let mut block = Self {
            kind: BlockKind::MagentaCarpet,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `light_blue_carpet` with default state values."]
    pub fn light_blue_carpet() -> Self {
        let mut block = Self {
            kind: BlockKind::LightBlueCarpet,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `yellow_carpet` with default state values."]
    pub fn yellow_carpet() -> Self {
        let mut block = Self {
            kind: BlockKind::YellowCarpet,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `lime_carpet` with default state values."]
    pub fn lime_carpet() -> Self {
        let mut block = Self {
            kind: BlockKind::LimeCarpet,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `pink_carpet` with default state values."]
    pub fn pink_carpet() -> Self {
        let mut block = Self {
            kind: BlockKind::PinkCarpet,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `gray_carpet` with default state values."]
    pub fn gray_carpet() -> Self {
        let mut block = Self {
            kind: BlockKind::GrayCarpet,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `light_gray_carpet` with default state values."]
    pub fn light_gray_carpet() -> Self {
        let mut block = Self {
            kind: BlockKind::LightGrayCarpet,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `cyan_carpet` with default state values."]
    pub fn cyan_carpet() -> Self {
        let mut block = Self {
            kind: BlockKind::CyanCarpet,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `purple_carpet` with default state values."]
    pub fn purple_carpet() -> Self {
        let mut block = Self {
            kind: BlockKind::PurpleCarpet,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `blue_carpet` with default state values."]
    pub fn blue_carpet() -> Self {
        let mut block = Self {
            kind: BlockKind::BlueCarpet,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `brown_carpet` with default state values."]
    pub fn brown_carpet() -> Self {
        let mut block = Self {
            kind: BlockKind::BrownCarpet,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `green_carpet` with default state values."]
    pub fn green_carpet() -> Self {
        let mut block = Self {
            kind: BlockKind::GreenCarpet,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `red_carpet` with default state values."]
    pub fn red_carpet() -> Self {
        let mut block = Self {
            kind: BlockKind::RedCarpet,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `black_carpet` with default state values."]
    pub fn black_carpet() -> Self {
        let mut block = Self {
            kind: BlockKind::BlackCarpet,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `terracotta` with default state values."]
    pub fn terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::Terracotta,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `coal_block` with default state values."]
    pub fn coal_block() -> Self {
        let mut block = Self {
            kind: BlockKind::CoalBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `packed_ice` with default state values."]
    pub fn packed_ice() -> Self {
        let mut block = Self {
            kind: BlockKind::PackedIce,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `sunflower` with default state values.\nThe default state values are as follows:\n* `half_upper_lower`: lower\n"]
    pub fn sunflower() -> Self {
        let mut block = Self {
            kind: BlockKind::Sunflower,
            state: 0,
        };
        block.set_half_upper_lower(HalfUpperLower::Lower);
        block
    }
    #[doc = "Returns an instance of `lilac` with default state values.\nThe default state values are as follows:\n* `half_upper_lower`: lower\n"]
    pub fn lilac() -> Self {
        let mut block = Self {
            kind: BlockKind::Lilac,
            state: 0,
        };
        block.set_half_upper_lower(HalfUpperLower::Lower);
        block
    }
    #[doc = "Returns an instance of `rose_bush` with default state values.\nThe default state values are as follows:\n* `half_upper_lower`: lower\n"]
    pub fn rose_bush() -> Self {
        let mut block = Self {
            kind: BlockKind::RoseBush,
            state: 0,
        };
        block.set_half_upper_lower(HalfUpperLower::Lower);
        block
    }
    #[doc = "Returns an instance of `peony` with default state values.\nThe default state values are as follows:\n* `half_upper_lower`: lower\n"]
    pub fn peony() -> Self {
        let mut block = Self {
            kind: BlockKind::Peony,
            state: 0,
        };
        block.set_half_upper_lower(HalfUpperLower::Lower);
        block
    }
    #[doc = "Returns an instance of `tall_grass` with default state values.\nThe default state values are as follows:\n* `half_upper_lower`: lower\n"]
    pub fn tall_grass() -> Self {
        let mut block = Self {
            kind: BlockKind::TallGrass,
            state: 0,
        };
        block.set_half_upper_lower(HalfUpperLower::Lower);
        block
    }
    #[doc = "Returns an instance of `large_fern` with default state values.\nThe default state values are as follows:\n* `half_upper_lower`: lower\n"]
    pub fn large_fern() -> Self {
        let mut block = Self {
            kind: BlockKind::LargeFern,
            state: 0,
        };
        block.set_half_upper_lower(HalfUpperLower::Lower);
        block
    }
    #[doc = "Returns an instance of `white_banner` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn white_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::WhiteBanner,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `orange_banner` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn orange_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::OrangeBanner,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `magenta_banner` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn magenta_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::MagentaBanner,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `light_blue_banner` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn light_blue_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::LightBlueBanner,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `yellow_banner` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn yellow_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::YellowBanner,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `lime_banner` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn lime_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::LimeBanner,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `pink_banner` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn pink_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::PinkBanner,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `gray_banner` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn gray_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::GrayBanner,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `light_gray_banner` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn light_gray_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::LightGrayBanner,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `cyan_banner` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn cyan_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::CyanBanner,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `purple_banner` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn purple_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::PurpleBanner,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `blue_banner` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn blue_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::BlueBanner,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `brown_banner` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn brown_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::BrownBanner,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `green_banner` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn green_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::GreenBanner,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `red_banner` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn red_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::RedBanner,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `black_banner` with default state values.\nThe default state values are as follows:\n* `rotation`: 0\n"]
    pub fn black_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::BlackBanner,
            state: 0,
        };
        block.set_rotation(0i32);
        block
    }
    #[doc = "Returns an instance of `white_wall_banner` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn white_wall_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::WhiteWallBanner,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `orange_wall_banner` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn orange_wall_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::OrangeWallBanner,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `magenta_wall_banner` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn magenta_wall_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::MagentaWallBanner,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `light_blue_wall_banner` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn light_blue_wall_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::LightBlueWallBanner,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `yellow_wall_banner` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn yellow_wall_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::YellowWallBanner,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `lime_wall_banner` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn lime_wall_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::LimeWallBanner,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `pink_wall_banner` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn pink_wall_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::PinkWallBanner,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `gray_wall_banner` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn gray_wall_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::GrayWallBanner,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `light_gray_wall_banner` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn light_gray_wall_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::LightGrayWallBanner,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `cyan_wall_banner` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn cyan_wall_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::CyanWallBanner,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `purple_wall_banner` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn purple_wall_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::PurpleWallBanner,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `blue_wall_banner` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn blue_wall_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::BlueWallBanner,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `brown_wall_banner` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn brown_wall_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::BrownWallBanner,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `green_wall_banner` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn green_wall_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::GreenWallBanner,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `red_wall_banner` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn red_wall_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::RedWallBanner,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `black_wall_banner` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn black_wall_banner() -> Self {
        let mut block = Self {
            kind: BlockKind::BlackWallBanner,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `red_sandstone` with default state values."]
    pub fn red_sandstone() -> Self {
        let mut block = Self {
            kind: BlockKind::RedSandstone,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `chiseled_red_sandstone` with default state values."]
    pub fn chiseled_red_sandstone() -> Self {
        let mut block = Self {
            kind: BlockKind::ChiseledRedSandstone,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `cut_red_sandstone` with default state values."]
    pub fn cut_red_sandstone() -> Self {
        let mut block = Self {
            kind: BlockKind::CutRedSandstone,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `red_sandstone_stairs` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `stairs_shape`: straight\n* `waterlogged`: false\n"]
    pub fn red_sandstone_stairs() -> Self {
        let mut block = Self {
            kind: BlockKind::RedSandstoneStairs,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_stairs_shape(StairsShape::Straight);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `oak_slab` with default state values.\nThe default state values are as follows:\n* `slab_kind`: bottom\n* `waterlogged`: false\n"]
    pub fn oak_slab() -> Self {
        let mut block = Self {
            kind: BlockKind::OakSlab,
            state: 0,
        };
        block.set_slab_kind(SlabKind::Bottom);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `spruce_slab` with default state values.\nThe default state values are as follows:\n* `slab_kind`: bottom\n* `waterlogged`: false\n"]
    pub fn spruce_slab() -> Self {
        let mut block = Self {
            kind: BlockKind::SpruceSlab,
            state: 0,
        };
        block.set_slab_kind(SlabKind::Bottom);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `birch_slab` with default state values.\nThe default state values are as follows:\n* `slab_kind`: bottom\n* `waterlogged`: false\n"]
    pub fn birch_slab() -> Self {
        let mut block = Self {
            kind: BlockKind::BirchSlab,
            state: 0,
        };
        block.set_slab_kind(SlabKind::Bottom);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `jungle_slab` with default state values.\nThe default state values are as follows:\n* `slab_kind`: bottom\n* `waterlogged`: false\n"]
    pub fn jungle_slab() -> Self {
        let mut block = Self {
            kind: BlockKind::JungleSlab,
            state: 0,
        };
        block.set_slab_kind(SlabKind::Bottom);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `acacia_slab` with default state values.\nThe default state values are as follows:\n* `slab_kind`: bottom\n* `waterlogged`: false\n"]
    pub fn acacia_slab() -> Self {
        let mut block = Self {
            kind: BlockKind::AcaciaSlab,
            state: 0,
        };
        block.set_slab_kind(SlabKind::Bottom);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `dark_oak_slab` with default state values.\nThe default state values are as follows:\n* `slab_kind`: bottom\n* `waterlogged`: false\n"]
    pub fn dark_oak_slab() -> Self {
        let mut block = Self {
            kind: BlockKind::DarkOakSlab,
            state: 0,
        };
        block.set_slab_kind(SlabKind::Bottom);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `stone_slab` with default state values.\nThe default state values are as follows:\n* `slab_kind`: bottom\n* `waterlogged`: false\n"]
    pub fn stone_slab() -> Self {
        let mut block = Self {
            kind: BlockKind::StoneSlab,
            state: 0,
        };
        block.set_slab_kind(SlabKind::Bottom);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `sandstone_slab` with default state values.\nThe default state values are as follows:\n* `slab_kind`: bottom\n* `waterlogged`: false\n"]
    pub fn sandstone_slab() -> Self {
        let mut block = Self {
            kind: BlockKind::SandstoneSlab,
            state: 0,
        };
        block.set_slab_kind(SlabKind::Bottom);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `petrified_oak_slab` with default state values.\nThe default state values are as follows:\n* `slab_kind`: bottom\n* `waterlogged`: false\n"]
    pub fn petrified_oak_slab() -> Self {
        let mut block = Self {
            kind: BlockKind::PetrifiedOakSlab,
            state: 0,
        };
        block.set_slab_kind(SlabKind::Bottom);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `cobblestone_slab` with default state values.\nThe default state values are as follows:\n* `slab_kind`: bottom\n* `waterlogged`: false\n"]
    pub fn cobblestone_slab() -> Self {
        let mut block = Self {
            kind: BlockKind::CobblestoneSlab,
            state: 0,
        };
        block.set_slab_kind(SlabKind::Bottom);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `brick_slab` with default state values.\nThe default state values are as follows:\n* `slab_kind`: bottom\n* `waterlogged`: false\n"]
    pub fn brick_slab() -> Self {
        let mut block = Self {
            kind: BlockKind::BrickSlab,
            state: 0,
        };
        block.set_slab_kind(SlabKind::Bottom);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `stone_brick_slab` with default state values.\nThe default state values are as follows:\n* `slab_kind`: bottom\n* `waterlogged`: false\n"]
    pub fn stone_brick_slab() -> Self {
        let mut block = Self {
            kind: BlockKind::StoneBrickSlab,
            state: 0,
        };
        block.set_slab_kind(SlabKind::Bottom);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `nether_brick_slab` with default state values.\nThe default state values are as follows:\n* `slab_kind`: bottom\n* `waterlogged`: false\n"]
    pub fn nether_brick_slab() -> Self {
        let mut block = Self {
            kind: BlockKind::NetherBrickSlab,
            state: 0,
        };
        block.set_slab_kind(SlabKind::Bottom);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `quartz_slab` with default state values.\nThe default state values are as follows:\n* `slab_kind`: bottom\n* `waterlogged`: false\n"]
    pub fn quartz_slab() -> Self {
        let mut block = Self {
            kind: BlockKind::QuartzSlab,
            state: 0,
        };
        block.set_slab_kind(SlabKind::Bottom);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `red_sandstone_slab` with default state values.\nThe default state values are as follows:\n* `slab_kind`: bottom\n* `waterlogged`: false\n"]
    pub fn red_sandstone_slab() -> Self {
        let mut block = Self {
            kind: BlockKind::RedSandstoneSlab,
            state: 0,
        };
        block.set_slab_kind(SlabKind::Bottom);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `purpur_slab` with default state values.\nThe default state values are as follows:\n* `slab_kind`: bottom\n* `waterlogged`: false\n"]
    pub fn purpur_slab() -> Self {
        let mut block = Self {
            kind: BlockKind::PurpurSlab,
            state: 0,
        };
        block.set_slab_kind(SlabKind::Bottom);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `smooth_stone` with default state values."]
    pub fn smooth_stone() -> Self {
        let mut block = Self {
            kind: BlockKind::SmoothStone,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `smooth_sandstone` with default state values."]
    pub fn smooth_sandstone() -> Self {
        let mut block = Self {
            kind: BlockKind::SmoothSandstone,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `smooth_quartz` with default state values."]
    pub fn smooth_quartz() -> Self {
        let mut block = Self {
            kind: BlockKind::SmoothQuartz,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `smooth_red_sandstone` with default state values."]
    pub fn smooth_red_sandstone() -> Self {
        let mut block = Self {
            kind: BlockKind::SmoothRedSandstone,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `spruce_fence_gate` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `in_wall`: false\n* `open`: false\n* `powered`: false\n"]
    pub fn spruce_fence_gate() -> Self {
        let mut block = Self {
            kind: BlockKind::SpruceFenceGate,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_in_wall(false);
        block.set_open(false);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `birch_fence_gate` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `in_wall`: false\n* `open`: false\n* `powered`: false\n"]
    pub fn birch_fence_gate() -> Self {
        let mut block = Self {
            kind: BlockKind::BirchFenceGate,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_in_wall(false);
        block.set_open(false);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `jungle_fence_gate` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `in_wall`: false\n* `open`: false\n* `powered`: false\n"]
    pub fn jungle_fence_gate() -> Self {
        let mut block = Self {
            kind: BlockKind::JungleFenceGate,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_in_wall(false);
        block.set_open(false);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `acacia_fence_gate` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `in_wall`: false\n* `open`: false\n* `powered`: false\n"]
    pub fn acacia_fence_gate() -> Self {
        let mut block = Self {
            kind: BlockKind::AcaciaFenceGate,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_in_wall(false);
        block.set_open(false);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `dark_oak_fence_gate` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `in_wall`: false\n* `open`: false\n* `powered`: false\n"]
    pub fn dark_oak_fence_gate() -> Self {
        let mut block = Self {
            kind: BlockKind::DarkOakFenceGate,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_in_wall(false);
        block.set_open(false);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `spruce_fence` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn spruce_fence() -> Self {
        let mut block = Self {
            kind: BlockKind::SpruceFence,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `birch_fence` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn birch_fence() -> Self {
        let mut block = Self {
            kind: BlockKind::BirchFence,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `jungle_fence` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn jungle_fence() -> Self {
        let mut block = Self {
            kind: BlockKind::JungleFence,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `acacia_fence` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn acacia_fence() -> Self {
        let mut block = Self {
            kind: BlockKind::AcaciaFence,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `dark_oak_fence` with default state values.\nThe default state values are as follows:\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `waterlogged`: false\n* `west_connected`: false\n"]
    pub fn dark_oak_fence() -> Self {
        let mut block = Self {
            kind: BlockKind::DarkOakFence,
            state: 0,
        };
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_waterlogged(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `spruce_door` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_upper_lower`: lower\n* `hinge`: left\n* `open`: false\n* `powered`: false\n"]
    pub fn spruce_door() -> Self {
        let mut block = Self {
            kind: BlockKind::SpruceDoor,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_upper_lower(HalfUpperLower::Lower);
        block.set_hinge(Hinge::Left);
        block.set_open(false);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `birch_door` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_upper_lower`: lower\n* `hinge`: left\n* `open`: false\n* `powered`: false\n"]
    pub fn birch_door() -> Self {
        let mut block = Self {
            kind: BlockKind::BirchDoor,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_upper_lower(HalfUpperLower::Lower);
        block.set_hinge(Hinge::Left);
        block.set_open(false);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `jungle_door` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_upper_lower`: lower\n* `hinge`: left\n* `open`: false\n* `powered`: false\n"]
    pub fn jungle_door() -> Self {
        let mut block = Self {
            kind: BlockKind::JungleDoor,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_upper_lower(HalfUpperLower::Lower);
        block.set_hinge(Hinge::Left);
        block.set_open(false);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `acacia_door` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_upper_lower`: lower\n* `hinge`: left\n* `open`: false\n* `powered`: false\n"]
    pub fn acacia_door() -> Self {
        let mut block = Self {
            kind: BlockKind::AcaciaDoor,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_upper_lower(HalfUpperLower::Lower);
        block.set_hinge(Hinge::Left);
        block.set_open(false);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `dark_oak_door` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_upper_lower`: lower\n* `hinge`: left\n* `open`: false\n* `powered`: false\n"]
    pub fn dark_oak_door() -> Self {
        let mut block = Self {
            kind: BlockKind::DarkOakDoor,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_upper_lower(HalfUpperLower::Lower);
        block.set_hinge(Hinge::Left);
        block.set_open(false);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `end_rod` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: up\n"]
    pub fn end_rod() -> Self {
        let mut block = Self {
            kind: BlockKind::EndRod,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::Up);
        block
    }
    #[doc = "Returns an instance of `chorus_plant` with default state values.\nThe default state values are as follows:\n* `down`: false\n* `east_connected`: false\n* `north_connected`: false\n* `south_connected`: false\n* `up`: false\n* `west_connected`: false\n"]
    pub fn chorus_plant() -> Self {
        let mut block = Self {
            kind: BlockKind::ChorusPlant,
            state: 0,
        };
        block.set_down(false);
        block.set_east_connected(false);
        block.set_north_connected(false);
        block.set_south_connected(false);
        block.set_up(false);
        block.set_west_connected(false);
        block
    }
    #[doc = "Returns an instance of `chorus_flower` with default state values.\nThe default state values are as follows:\n* `age_0_5`: 0\n"]
    pub fn chorus_flower() -> Self {
        let mut block = Self {
            kind: BlockKind::ChorusFlower,
            state: 0,
        };
        block.set_age_0_5(0i32);
        block
    }
    #[doc = "Returns an instance of `purpur_block` with default state values."]
    pub fn purpur_block() -> Self {
        let mut block = Self {
            kind: BlockKind::PurpurBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `purpur_pillar` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn purpur_pillar() -> Self {
        let mut block = Self {
            kind: BlockKind::PurpurPillar,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `purpur_stairs` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `half_top_bottom`: bottom\n* `stairs_shape`: straight\n* `waterlogged`: false\n"]
    pub fn purpur_stairs() -> Self {
        let mut block = Self {
            kind: BlockKind::PurpurStairs,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_half_top_bottom(HalfTopBottom::Bottom);
        block.set_stairs_shape(StairsShape::Straight);
        block.set_waterlogged(false);
        block
    }
    #[doc = "Returns an instance of `end_stone_bricks` with default state values."]
    pub fn end_stone_bricks() -> Self {
        let mut block = Self {
            kind: BlockKind::EndStoneBricks,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `beetroots` with default state values.\nThe default state values are as follows:\n* `age_0_3`: 0\n"]
    pub fn beetroots() -> Self {
        let mut block = Self {
            kind: BlockKind::Beetroots,
            state: 0,
        };
        block.set_age_0_3(0i32);
        block
    }
    #[doc = "Returns an instance of `grass_path` with default state values."]
    pub fn grass_path() -> Self {
        let mut block = Self {
            kind: BlockKind::GrassPath,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `end_gateway` with default state values."]
    pub fn end_gateway() -> Self {
        let mut block = Self {
            kind: BlockKind::EndGateway,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `repeating_command_block` with default state values.\nThe default state values are as follows:\n* `conditional`: false\n* `facing_cubic`: north\n"]
    pub fn repeating_command_block() -> Self {
        let mut block = Self {
            kind: BlockKind::RepeatingCommandBlock,
            state: 0,
        };
        block.set_conditional(false);
        block.set_facing_cubic(FacingCubic::North);
        block
    }
    #[doc = "Returns an instance of `chain_command_block` with default state values.\nThe default state values are as follows:\n* `conditional`: false\n* `facing_cubic`: north\n"]
    pub fn chain_command_block() -> Self {
        let mut block = Self {
            kind: BlockKind::ChainCommandBlock,
            state: 0,
        };
        block.set_conditional(false);
        block.set_facing_cubic(FacingCubic::North);
        block
    }
    #[doc = "Returns an instance of `frosted_ice` with default state values.\nThe default state values are as follows:\n* `age_0_3`: 0\n"]
    pub fn frosted_ice() -> Self {
        let mut block = Self {
            kind: BlockKind::FrostedIce,
            state: 0,
        };
        block.set_age_0_3(0i32);
        block
    }
    #[doc = "Returns an instance of `magma_block` with default state values."]
    pub fn magma_block() -> Self {
        let mut block = Self {
            kind: BlockKind::MagmaBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `nether_wart_block` with default state values."]
    pub fn nether_wart_block() -> Self {
        let mut block = Self {
            kind: BlockKind::NetherWartBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `red_nether_bricks` with default state values."]
    pub fn red_nether_bricks() -> Self {
        let mut block = Self {
            kind: BlockKind::RedNetherBricks,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `bone_block` with default state values.\nThe default state values are as follows:\n* `axis_xyz`: y\n"]
    pub fn bone_block() -> Self {
        let mut block = Self {
            kind: BlockKind::BoneBlock,
            state: 0,
        };
        block.set_axis_xyz(AxisXyz::Y);
        block
    }
    #[doc = "Returns an instance of `structure_void` with default state values."]
    pub fn structure_void() -> Self {
        let mut block = Self {
            kind: BlockKind::StructureVoid,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `observer` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: south\n* `powered`: false\n"]
    pub fn observer() -> Self {
        let mut block = Self {
            kind: BlockKind::Observer,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::South);
        block.set_powered(false);
        block
    }
    #[doc = "Returns an instance of `shulker_box` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: up\n"]
    pub fn shulker_box() -> Self {
        let mut block = Self {
            kind: BlockKind::ShulkerBox,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::Up);
        block
    }
    #[doc = "Returns an instance of `white_shulker_box` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: up\n"]
    pub fn white_shulker_box() -> Self {
        let mut block = Self {
            kind: BlockKind::WhiteShulkerBox,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::Up);
        block
    }
    #[doc = "Returns an instance of `orange_shulker_box` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: up\n"]
    pub fn orange_shulker_box() -> Self {
        let mut block = Self {
            kind: BlockKind::OrangeShulkerBox,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::Up);
        block
    }
    #[doc = "Returns an instance of `magenta_shulker_box` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: up\n"]
    pub fn magenta_shulker_box() -> Self {
        let mut block = Self {
            kind: BlockKind::MagentaShulkerBox,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::Up);
        block
    }
    #[doc = "Returns an instance of `light_blue_shulker_box` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: up\n"]
    pub fn light_blue_shulker_box() -> Self {
        let mut block = Self {
            kind: BlockKind::LightBlueShulkerBox,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::Up);
        block
    }
    #[doc = "Returns an instance of `yellow_shulker_box` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: up\n"]
    pub fn yellow_shulker_box() -> Self {
        let mut block = Self {
            kind: BlockKind::YellowShulkerBox,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::Up);
        block
    }
    #[doc = "Returns an instance of `lime_shulker_box` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: up\n"]
    pub fn lime_shulker_box() -> Self {
        let mut block = Self {
            kind: BlockKind::LimeShulkerBox,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::Up);
        block
    }
    #[doc = "Returns an instance of `pink_shulker_box` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: up\n"]
    pub fn pink_shulker_box() -> Self {
        let mut block = Self {
            kind: BlockKind::PinkShulkerBox,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::Up);
        block
    }
    #[doc = "Returns an instance of `gray_shulker_box` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: up\n"]
    pub fn gray_shulker_box() -> Self {
        let mut block = Self {
            kind: BlockKind::GrayShulkerBox,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::Up);
        block
    }
    #[doc = "Returns an instance of `light_gray_shulker_box` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: up\n"]
    pub fn light_gray_shulker_box() -> Self {
        let mut block = Self {
            kind: BlockKind::LightGrayShulkerBox,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::Up);
        block
    }
    #[doc = "Returns an instance of `cyan_shulker_box` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: up\n"]
    pub fn cyan_shulker_box() -> Self {
        let mut block = Self {
            kind: BlockKind::CyanShulkerBox,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::Up);
        block
    }
    #[doc = "Returns an instance of `purple_shulker_box` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: up\n"]
    pub fn purple_shulker_box() -> Self {
        let mut block = Self {
            kind: BlockKind::PurpleShulkerBox,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::Up);
        block
    }
    #[doc = "Returns an instance of `blue_shulker_box` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: up\n"]
    pub fn blue_shulker_box() -> Self {
        let mut block = Self {
            kind: BlockKind::BlueShulkerBox,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::Up);
        block
    }
    #[doc = "Returns an instance of `brown_shulker_box` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: up\n"]
    pub fn brown_shulker_box() -> Self {
        let mut block = Self {
            kind: BlockKind::BrownShulkerBox,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::Up);
        block
    }
    #[doc = "Returns an instance of `green_shulker_box` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: up\n"]
    pub fn green_shulker_box() -> Self {
        let mut block = Self {
            kind: BlockKind::GreenShulkerBox,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::Up);
        block
    }
    #[doc = "Returns an instance of `red_shulker_box` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: up\n"]
    pub fn red_shulker_box() -> Self {
        let mut block = Self {
            kind: BlockKind::RedShulkerBox,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::Up);
        block
    }
    #[doc = "Returns an instance of `black_shulker_box` with default state values.\nThe default state values are as follows:\n* `facing_cubic`: up\n"]
    pub fn black_shulker_box() -> Self {
        let mut block = Self {
            kind: BlockKind::BlackShulkerBox,
            state: 0,
        };
        block.set_facing_cubic(FacingCubic::Up);
        block
    }
    #[doc = "Returns an instance of `white_glazed_terracotta` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn white_glazed_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::WhiteGlazedTerracotta,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `orange_glazed_terracotta` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn orange_glazed_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::OrangeGlazedTerracotta,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `magenta_glazed_terracotta` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn magenta_glazed_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::MagentaGlazedTerracotta,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `light_blue_glazed_terracotta` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn light_blue_glazed_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::LightBlueGlazedTerracotta,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `yellow_glazed_terracotta` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn yellow_glazed_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::YellowGlazedTerracotta,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `lime_glazed_terracotta` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn lime_glazed_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::LimeGlazedTerracotta,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `pink_glazed_terracotta` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn pink_glazed_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::PinkGlazedTerracotta,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `gray_glazed_terracotta` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn gray_glazed_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::GrayGlazedTerracotta,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `light_gray_glazed_terracotta` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn light_gray_glazed_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::LightGrayGlazedTerracotta,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `cyan_glazed_terracotta` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn cyan_glazed_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::CyanGlazedTerracotta,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `purple_glazed_terracotta` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn purple_glazed_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::PurpleGlazedTerracotta,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `blue_glazed_terracotta` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn blue_glazed_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::BlueGlazedTerracotta,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `brown_glazed_terracotta` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn brown_glazed_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::BrownGlazedTerracotta,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `green_glazed_terracotta` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn green_glazed_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::GreenGlazedTerracotta,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `red_glazed_terracotta` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn red_glazed_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::RedGlazedTerracotta,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `black_glazed_terracotta` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n"]
    pub fn black_glazed_terracotta() -> Self {
        let mut block = Self {
            kind: BlockKind::BlackGlazedTerracotta,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block
    }
    #[doc = "Returns an instance of `white_concrete` with default state values."]
    pub fn white_concrete() -> Self {
        let mut block = Self {
            kind: BlockKind::WhiteConcrete,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `orange_concrete` with default state values."]
    pub fn orange_concrete() -> Self {
        let mut block = Self {
            kind: BlockKind::OrangeConcrete,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `magenta_concrete` with default state values."]
    pub fn magenta_concrete() -> Self {
        let mut block = Self {
            kind: BlockKind::MagentaConcrete,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `light_blue_concrete` with default state values."]
    pub fn light_blue_concrete() -> Self {
        let mut block = Self {
            kind: BlockKind::LightBlueConcrete,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `yellow_concrete` with default state values."]
    pub fn yellow_concrete() -> Self {
        let mut block = Self {
            kind: BlockKind::YellowConcrete,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `lime_concrete` with default state values."]
    pub fn lime_concrete() -> Self {
        let mut block = Self {
            kind: BlockKind::LimeConcrete,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `pink_concrete` with default state values."]
    pub fn pink_concrete() -> Self {
        let mut block = Self {
            kind: BlockKind::PinkConcrete,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `gray_concrete` with default state values."]
    pub fn gray_concrete() -> Self {
        let mut block = Self {
            kind: BlockKind::GrayConcrete,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `light_gray_concrete` with default state values."]
    pub fn light_gray_concrete() -> Self {
        let mut block = Self {
            kind: BlockKind::LightGrayConcrete,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `cyan_concrete` with default state values."]
    pub fn cyan_concrete() -> Self {
        let mut block = Self {
            kind: BlockKind::CyanConcrete,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `purple_concrete` with default state values."]
    pub fn purple_concrete() -> Self {
        let mut block = Self {
            kind: BlockKind::PurpleConcrete,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `blue_concrete` with default state values."]
    pub fn blue_concrete() -> Self {
        let mut block = Self {
            kind: BlockKind::BlueConcrete,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `brown_concrete` with default state values."]
    pub fn brown_concrete() -> Self {
        let mut block = Self {
            kind: BlockKind::BrownConcrete,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `green_concrete` with default state values."]
    pub fn green_concrete() -> Self {
        let mut block = Self {
            kind: BlockKind::GreenConcrete,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `red_concrete` with default state values."]
    pub fn red_concrete() -> Self {
        let mut block = Self {
            kind: BlockKind::RedConcrete,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `black_concrete` with default state values."]
    pub fn black_concrete() -> Self {
        let mut block = Self {
            kind: BlockKind::BlackConcrete,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `white_concrete_powder` with default state values."]
    pub fn white_concrete_powder() -> Self {
        let mut block = Self {
            kind: BlockKind::WhiteConcretePowder,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `orange_concrete_powder` with default state values."]
    pub fn orange_concrete_powder() -> Self {
        let mut block = Self {
            kind: BlockKind::OrangeConcretePowder,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `magenta_concrete_powder` with default state values."]
    pub fn magenta_concrete_powder() -> Self {
        let mut block = Self {
            kind: BlockKind::MagentaConcretePowder,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `light_blue_concrete_powder` with default state values."]
    pub fn light_blue_concrete_powder() -> Self {
        let mut block = Self {
            kind: BlockKind::LightBlueConcretePowder,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `yellow_concrete_powder` with default state values."]
    pub fn yellow_concrete_powder() -> Self {
        let mut block = Self {
            kind: BlockKind::YellowConcretePowder,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `lime_concrete_powder` with default state values."]
    pub fn lime_concrete_powder() -> Self {
        let mut block = Self {
            kind: BlockKind::LimeConcretePowder,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `pink_concrete_powder` with default state values."]
    pub fn pink_concrete_powder() -> Self {
        let mut block = Self {
            kind: BlockKind::PinkConcretePowder,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `gray_concrete_powder` with default state values."]
    pub fn gray_concrete_powder() -> Self {
        let mut block = Self {
            kind: BlockKind::GrayConcretePowder,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `light_gray_concrete_powder` with default state values."]
    pub fn light_gray_concrete_powder() -> Self {
        let mut block = Self {
            kind: BlockKind::LightGrayConcretePowder,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `cyan_concrete_powder` with default state values."]
    pub fn cyan_concrete_powder() -> Self {
        let mut block = Self {
            kind: BlockKind::CyanConcretePowder,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `purple_concrete_powder` with default state values."]
    pub fn purple_concrete_powder() -> Self {
        let mut block = Self {
            kind: BlockKind::PurpleConcretePowder,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `blue_concrete_powder` with default state values."]
    pub fn blue_concrete_powder() -> Self {
        let mut block = Self {
            kind: BlockKind::BlueConcretePowder,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `brown_concrete_powder` with default state values."]
    pub fn brown_concrete_powder() -> Self {
        let mut block = Self {
            kind: BlockKind::BrownConcretePowder,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `green_concrete_powder` with default state values."]
    pub fn green_concrete_powder() -> Self {
        let mut block = Self {
            kind: BlockKind::GreenConcretePowder,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `red_concrete_powder` with default state values."]
    pub fn red_concrete_powder() -> Self {
        let mut block = Self {
            kind: BlockKind::RedConcretePowder,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `black_concrete_powder` with default state values."]
    pub fn black_concrete_powder() -> Self {
        let mut block = Self {
            kind: BlockKind::BlackConcretePowder,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `kelp` with default state values.\nThe default state values are as follows:\n* `age_0_25`: 0\n"]
    pub fn kelp() -> Self {
        let mut block = Self {
            kind: BlockKind::Kelp,
            state: 0,
        };
        block.set_age_0_25(0i32);
        block
    }
    #[doc = "Returns an instance of `kelp_plant` with default state values."]
    pub fn kelp_plant() -> Self {
        let mut block = Self {
            kind: BlockKind::KelpPlant,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `dried_kelp_block` with default state values."]
    pub fn dried_kelp_block() -> Self {
        let mut block = Self {
            kind: BlockKind::DriedKelpBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `turtle_egg` with default state values.\nThe default state values are as follows:\n* `eggs`: 1\n* `hatch`: 0\n"]
    pub fn turtle_egg() -> Self {
        let mut block = Self {
            kind: BlockKind::TurtleEgg,
            state: 0,
        };
        block.set_eggs(1i32);
        block.set_hatch(0i32);
        block
    }
    #[doc = "Returns an instance of `dead_tube_coral_block` with default state values."]
    pub fn dead_tube_coral_block() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadTubeCoralBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `dead_brain_coral_block` with default state values."]
    pub fn dead_brain_coral_block() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadBrainCoralBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `dead_bubble_coral_block` with default state values."]
    pub fn dead_bubble_coral_block() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadBubbleCoralBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `dead_fire_coral_block` with default state values."]
    pub fn dead_fire_coral_block() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadFireCoralBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `dead_horn_coral_block` with default state values."]
    pub fn dead_horn_coral_block() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadHornCoralBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `tube_coral_block` with default state values."]
    pub fn tube_coral_block() -> Self {
        let mut block = Self {
            kind: BlockKind::TubeCoralBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `brain_coral_block` with default state values."]
    pub fn brain_coral_block() -> Self {
        let mut block = Self {
            kind: BlockKind::BrainCoralBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `bubble_coral_block` with default state values."]
    pub fn bubble_coral_block() -> Self {
        let mut block = Self {
            kind: BlockKind::BubbleCoralBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `fire_coral_block` with default state values."]
    pub fn fire_coral_block() -> Self {
        let mut block = Self {
            kind: BlockKind::FireCoralBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `horn_coral_block` with default state values."]
    pub fn horn_coral_block() -> Self {
        let mut block = Self {
            kind: BlockKind::HornCoralBlock,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `dead_tube_coral` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn dead_tube_coral() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadTubeCoral,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `dead_brain_coral` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn dead_brain_coral() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadBrainCoral,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `dead_bubble_coral` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn dead_bubble_coral() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadBubbleCoral,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `dead_fire_coral` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn dead_fire_coral() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadFireCoral,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `dead_horn_coral` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn dead_horn_coral() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadHornCoral,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `tube_coral` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn tube_coral() -> Self {
        let mut block = Self {
            kind: BlockKind::TubeCoral,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `brain_coral` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn brain_coral() -> Self {
        let mut block = Self {
            kind: BlockKind::BrainCoral,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `bubble_coral` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn bubble_coral() -> Self {
        let mut block = Self {
            kind: BlockKind::BubbleCoral,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `fire_coral` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn fire_coral() -> Self {
        let mut block = Self {
            kind: BlockKind::FireCoral,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `horn_coral` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn horn_coral() -> Self {
        let mut block = Self {
            kind: BlockKind::HornCoral,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `dead_tube_coral_wall_fan` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `waterlogged`: true\n"]
    pub fn dead_tube_coral_wall_fan() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadTubeCoralWallFan,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `dead_brain_coral_wall_fan` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `waterlogged`: true\n"]
    pub fn dead_brain_coral_wall_fan() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadBrainCoralWallFan,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `dead_bubble_coral_wall_fan` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `waterlogged`: true\n"]
    pub fn dead_bubble_coral_wall_fan() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadBubbleCoralWallFan,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `dead_fire_coral_wall_fan` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `waterlogged`: true\n"]
    pub fn dead_fire_coral_wall_fan() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadFireCoralWallFan,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `dead_horn_coral_wall_fan` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `waterlogged`: true\n"]
    pub fn dead_horn_coral_wall_fan() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadHornCoralWallFan,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `tube_coral_wall_fan` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `waterlogged`: true\n"]
    pub fn tube_coral_wall_fan() -> Self {
        let mut block = Self {
            kind: BlockKind::TubeCoralWallFan,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `brain_coral_wall_fan` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `waterlogged`: true\n"]
    pub fn brain_coral_wall_fan() -> Self {
        let mut block = Self {
            kind: BlockKind::BrainCoralWallFan,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `bubble_coral_wall_fan` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `waterlogged`: true\n"]
    pub fn bubble_coral_wall_fan() -> Self {
        let mut block = Self {
            kind: BlockKind::BubbleCoralWallFan,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `fire_coral_wall_fan` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `waterlogged`: true\n"]
    pub fn fire_coral_wall_fan() -> Self {
        let mut block = Self {
            kind: BlockKind::FireCoralWallFan,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `horn_coral_wall_fan` with default state values.\nThe default state values are as follows:\n* `facing_cardinal`: north\n* `waterlogged`: true\n"]
    pub fn horn_coral_wall_fan() -> Self {
        let mut block = Self {
            kind: BlockKind::HornCoralWallFan,
            state: 0,
        };
        block.set_facing_cardinal(FacingCardinal::North);
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `dead_tube_coral_fan` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn dead_tube_coral_fan() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadTubeCoralFan,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `dead_brain_coral_fan` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn dead_brain_coral_fan() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadBrainCoralFan,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `dead_bubble_coral_fan` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn dead_bubble_coral_fan() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadBubbleCoralFan,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `dead_fire_coral_fan` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn dead_fire_coral_fan() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadFireCoralFan,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `dead_horn_coral_fan` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn dead_horn_coral_fan() -> Self {
        let mut block = Self {
            kind: BlockKind::DeadHornCoralFan,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `tube_coral_fan` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn tube_coral_fan() -> Self {
        let mut block = Self {
            kind: BlockKind::TubeCoralFan,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `brain_coral_fan` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn brain_coral_fan() -> Self {
        let mut block = Self {
            kind: BlockKind::BrainCoralFan,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `bubble_coral_fan` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn bubble_coral_fan() -> Self {
        let mut block = Self {
            kind: BlockKind::BubbleCoralFan,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `fire_coral_fan` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn fire_coral_fan() -> Self {
        let mut block = Self {
            kind: BlockKind::FireCoralFan,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `horn_coral_fan` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn horn_coral_fan() -> Self {
        let mut block = Self {
            kind: BlockKind::HornCoralFan,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `sea_pickle` with default state values.\nThe default state values are as follows:\n* `pickles`: 1\n* `waterlogged`: true\n"]
    pub fn sea_pickle() -> Self {
        let mut block = Self {
            kind: BlockKind::SeaPickle,
            state: 0,
        };
        block.set_pickles(1i32);
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `blue_ice` with default state values."]
    pub fn blue_ice() -> Self {
        let mut block = Self {
            kind: BlockKind::BlueIce,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `conduit` with default state values.\nThe default state values are as follows:\n* `waterlogged`: true\n"]
    pub fn conduit() -> Self {
        let mut block = Self {
            kind: BlockKind::Conduit,
            state: 0,
        };
        block.set_waterlogged(true);
        block
    }
    #[doc = "Returns an instance of `void_air` with default state values."]
    pub fn void_air() -> Self {
        let mut block = Self {
            kind: BlockKind::VoidAir,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `cave_air` with default state values."]
    pub fn cave_air() -> Self {
        let mut block = Self {
            kind: BlockKind::CaveAir,
            state: 0,
        };
        block
    }
    #[doc = "Returns an instance of `bubble_column` with default state values.\nThe default state values are as follows:\n* `drag`: true\n"]
    pub fn bubble_column() -> Self {
        let mut block = Self {
            kind: BlockKind::BubbleColumn,
            state: 0,
        };
        block.set_drag(true);
        block
    }
    #[doc = "Returns an instance of `structure_block` with default state values.\nThe default state values are as follows:\n* `structure_block_mode`: save\n"]
    pub fn structure_block() -> Self {
        let mut block = Self {
            kind: BlockKind::StructureBlock,
            state: 0,
        };
        block.set_structure_block_mode(StructureBlockMode::Save);
        block
    }
    pub fn age_0_15(self) -> Option<i32> {
        BLOCK_TABLE.age_0_15(self.kind, self.state)
    }
    pub fn set_age_0_15(&mut self, age_0_15: i32) -> bool {
        match BLOCK_TABLE.set_age_0_15(self.kind, self.state, age_0_15) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_age_0_15(mut self, age_0_15: i32) -> Self {
        self.set_age_0_15(age_0_15);
        self
    }
    pub fn age_0_2(self) -> Option<i32> {
        BLOCK_TABLE.age_0_2(self.kind, self.state)
    }
    pub fn set_age_0_2(&mut self, age_0_2: i32) -> bool {
        match BLOCK_TABLE.set_age_0_2(self.kind, self.state, age_0_2) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_age_0_2(mut self, age_0_2: i32) -> Self {
        self.set_age_0_2(age_0_2);
        self
    }
    pub fn age_0_25(self) -> Option<i32> {
        BLOCK_TABLE.age_0_25(self.kind, self.state)
    }
    pub fn set_age_0_25(&mut self, age_0_25: i32) -> bool {
        match BLOCK_TABLE.set_age_0_25(self.kind, self.state, age_0_25) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_age_0_25(mut self, age_0_25: i32) -> Self {
        self.set_age_0_25(age_0_25);
        self
    }
    pub fn age_0_3(self) -> Option<i32> {
        BLOCK_TABLE.age_0_3(self.kind, self.state)
    }
    pub fn set_age_0_3(&mut self, age_0_3: i32) -> bool {
        match BLOCK_TABLE.set_age_0_3(self.kind, self.state, age_0_3) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_age_0_3(mut self, age_0_3: i32) -> Self {
        self.set_age_0_3(age_0_3);
        self
    }
    pub fn age_0_5(self) -> Option<i32> {
        BLOCK_TABLE.age_0_5(self.kind, self.state)
    }
    pub fn set_age_0_5(&mut self, age_0_5: i32) -> bool {
        match BLOCK_TABLE.set_age_0_5(self.kind, self.state, age_0_5) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_age_0_5(mut self, age_0_5: i32) -> Self {
        self.set_age_0_5(age_0_5);
        self
    }
    pub fn age_0_7(self) -> Option<i32> {
        BLOCK_TABLE.age_0_7(self.kind, self.state)
    }
    pub fn set_age_0_7(&mut self, age_0_7: i32) -> bool {
        match BLOCK_TABLE.set_age_0_7(self.kind, self.state, age_0_7) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_age_0_7(mut self, age_0_7: i32) -> Self {
        self.set_age_0_7(age_0_7);
        self
    }
    pub fn attached(self) -> Option<bool> {
        BLOCK_TABLE.attached(self.kind, self.state)
    }
    pub fn set_attached(&mut self, attached: bool) -> bool {
        match BLOCK_TABLE.set_attached(self.kind, self.state, attached) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_attached(mut self, attached: bool) -> Self {
        self.set_attached(attached);
        self
    }
    pub fn axis_xyz(self) -> Option<AxisXyz> {
        BLOCK_TABLE.axis_xyz(self.kind, self.state)
    }
    pub fn set_axis_xyz(&mut self, axis_xyz: AxisXyz) -> bool {
        match BLOCK_TABLE.set_axis_xyz(self.kind, self.state, axis_xyz) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_axis_xyz(mut self, axis_xyz: AxisXyz) -> Self {
        self.set_axis_xyz(axis_xyz);
        self
    }
    pub fn axis_xz(self) -> Option<AxisXz> {
        BLOCK_TABLE.axis_xz(self.kind, self.state)
    }
    pub fn set_axis_xz(&mut self, axis_xz: AxisXz) -> bool {
        match BLOCK_TABLE.set_axis_xz(self.kind, self.state, axis_xz) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_axis_xz(mut self, axis_xz: AxisXz) -> Self {
        self.set_axis_xz(axis_xz);
        self
    }
    pub fn bites(self) -> Option<i32> {
        BLOCK_TABLE.bites(self.kind, self.state)
    }
    pub fn set_bites(&mut self, bites: i32) -> bool {
        match BLOCK_TABLE.set_bites(self.kind, self.state, bites) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_bites(mut self, bites: i32) -> Self {
        self.set_bites(bites);
        self
    }
    pub fn cauldron_level(self) -> Option<i32> {
        BLOCK_TABLE.cauldron_level(self.kind, self.state)
    }
    pub fn set_cauldron_level(&mut self, cauldron_level: i32) -> bool {
        match BLOCK_TABLE.set_cauldron_level(self.kind, self.state, cauldron_level) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_cauldron_level(mut self, cauldron_level: i32) -> Self {
        self.set_cauldron_level(cauldron_level);
        self
    }
    pub fn chest_kind(self) -> Option<ChestKind> {
        BLOCK_TABLE.chest_kind(self.kind, self.state)
    }
    pub fn set_chest_kind(&mut self, chest_kind: ChestKind) -> bool {
        match BLOCK_TABLE.set_chest_kind(self.kind, self.state, chest_kind) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_chest_kind(mut self, chest_kind: ChestKind) -> Self {
        self.set_chest_kind(chest_kind);
        self
    }
    pub fn comparator_mode(self) -> Option<ComparatorMode> {
        BLOCK_TABLE.comparator_mode(self.kind, self.state)
    }
    pub fn set_comparator_mode(&mut self, comparator_mode: ComparatorMode) -> bool {
        match BLOCK_TABLE.set_comparator_mode(self.kind, self.state, comparator_mode) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_comparator_mode(mut self, comparator_mode: ComparatorMode) -> Self {
        self.set_comparator_mode(comparator_mode);
        self
    }
    pub fn conditional(self) -> Option<bool> {
        BLOCK_TABLE.conditional(self.kind, self.state)
    }
    pub fn set_conditional(&mut self, conditional: bool) -> bool {
        match BLOCK_TABLE.set_conditional(self.kind, self.state, conditional) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_conditional(mut self, conditional: bool) -> Self {
        self.set_conditional(conditional);
        self
    }
    pub fn delay(self) -> Option<i32> {
        BLOCK_TABLE.delay(self.kind, self.state)
    }
    pub fn set_delay(&mut self, delay: i32) -> bool {
        match BLOCK_TABLE.set_delay(self.kind, self.state, delay) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_delay(mut self, delay: i32) -> Self {
        self.set_delay(delay);
        self
    }
    pub fn disarmed(self) -> Option<bool> {
        BLOCK_TABLE.disarmed(self.kind, self.state)
    }
    pub fn set_disarmed(&mut self, disarmed: bool) -> bool {
        match BLOCK_TABLE.set_disarmed(self.kind, self.state, disarmed) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_disarmed(mut self, disarmed: bool) -> Self {
        self.set_disarmed(disarmed);
        self
    }
    pub fn distance(self) -> Option<i32> {
        BLOCK_TABLE.distance(self.kind, self.state)
    }
    pub fn set_distance(&mut self, distance: i32) -> bool {
        match BLOCK_TABLE.set_distance(self.kind, self.state, distance) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_distance(mut self, distance: i32) -> Self {
        self.set_distance(distance);
        self
    }
    pub fn down(self) -> Option<bool> {
        BLOCK_TABLE.down(self.kind, self.state)
    }
    pub fn set_down(&mut self, down: bool) -> bool {
        match BLOCK_TABLE.set_down(self.kind, self.state, down) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_down(mut self, down: bool) -> Self {
        self.set_down(down);
        self
    }
    pub fn drag(self) -> Option<bool> {
        BLOCK_TABLE.drag(self.kind, self.state)
    }
    pub fn set_drag(&mut self, drag: bool) -> bool {
        match BLOCK_TABLE.set_drag(self.kind, self.state, drag) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_drag(mut self, drag: bool) -> Self {
        self.set_drag(drag);
        self
    }
    pub fn east_connected(self) -> Option<bool> {
        BLOCK_TABLE.east_connected(self.kind, self.state)
    }
    pub fn set_east_connected(&mut self, east_connected: bool) -> bool {
        match BLOCK_TABLE.set_east_connected(self.kind, self.state, east_connected) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_east_connected(mut self, east_connected: bool) -> Self {
        self.set_east_connected(east_connected);
        self
    }
    pub fn east_wire(self) -> Option<EastWire> {
        BLOCK_TABLE.east_wire(self.kind, self.state)
    }
    pub fn set_east_wire(&mut self, east_wire: EastWire) -> bool {
        match BLOCK_TABLE.set_east_wire(self.kind, self.state, east_wire) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_east_wire(mut self, east_wire: EastWire) -> Self {
        self.set_east_wire(east_wire);
        self
    }
    pub fn eggs(self) -> Option<i32> {
        BLOCK_TABLE.eggs(self.kind, self.state)
    }
    pub fn set_eggs(&mut self, eggs: i32) -> bool {
        match BLOCK_TABLE.set_eggs(self.kind, self.state, eggs) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_eggs(mut self, eggs: i32) -> Self {
        self.set_eggs(eggs);
        self
    }
    pub fn enabled(self) -> Option<bool> {
        BLOCK_TABLE.enabled(self.kind, self.state)
    }
    pub fn set_enabled(&mut self, enabled: bool) -> bool {
        match BLOCK_TABLE.set_enabled(self.kind, self.state, enabled) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.set_enabled(enabled);
        self
    }
    pub fn extended(self) -> Option<bool> {
        BLOCK_TABLE.extended(self.kind, self.state)
    }
    pub fn set_extended(&mut self, extended: bool) -> bool {
        match BLOCK_TABLE.set_extended(self.kind, self.state, extended) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_extended(mut self, extended: bool) -> Self {
        self.set_extended(extended);
        self
    }
    pub fn eye(self) -> Option<bool> {
        BLOCK_TABLE.eye(self.kind, self.state)
    }
    pub fn set_eye(&mut self, eye: bool) -> bool {
        match BLOCK_TABLE.set_eye(self.kind, self.state, eye) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_eye(mut self, eye: bool) -> Self {
        self.set_eye(eye);
        self
    }
    pub fn face(self) -> Option<Face> {
        BLOCK_TABLE.face(self.kind, self.state)
    }
    pub fn set_face(&mut self, face: Face) -> bool {
        match BLOCK_TABLE.set_face(self.kind, self.state, face) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_face(mut self, face: Face) -> Self {
        self.set_face(face);
        self
    }
    pub fn facing_cardinal(self) -> Option<FacingCardinal> {
        BLOCK_TABLE.facing_cardinal(self.kind, self.state)
    }
    pub fn set_facing_cardinal(&mut self, facing_cardinal: FacingCardinal) -> bool {
        match BLOCK_TABLE.set_facing_cardinal(self.kind, self.state, facing_cardinal) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_facing_cardinal(mut self, facing_cardinal: FacingCardinal) -> Self {
        self.set_facing_cardinal(facing_cardinal);
        self
    }
    pub fn facing_cardinal_and_down(self) -> Option<FacingCardinalAndDown> {
        BLOCK_TABLE.facing_cardinal_and_down(self.kind, self.state)
    }
    pub fn set_facing_cardinal_and_down(
        &mut self,
        facing_cardinal_and_down: FacingCardinalAndDown,
    ) -> bool {
        match BLOCK_TABLE.set_facing_cardinal_and_down(
            self.kind,
            self.state,
            facing_cardinal_and_down,
        ) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_facing_cardinal_and_down(
        mut self,
        facing_cardinal_and_down: FacingCardinalAndDown,
    ) -> Self {
        self.set_facing_cardinal_and_down(facing_cardinal_and_down);
        self
    }
    pub fn facing_cubic(self) -> Option<FacingCubic> {
        BLOCK_TABLE.facing_cubic(self.kind, self.state)
    }
    pub fn set_facing_cubic(&mut self, facing_cubic: FacingCubic) -> bool {
        match BLOCK_TABLE.set_facing_cubic(self.kind, self.state, facing_cubic) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_facing_cubic(mut self, facing_cubic: FacingCubic) -> Self {
        self.set_facing_cubic(facing_cubic);
        self
    }
    pub fn half_top_bottom(self) -> Option<HalfTopBottom> {
        BLOCK_TABLE.half_top_bottom(self.kind, self.state)
    }
    pub fn set_half_top_bottom(&mut self, half_top_bottom: HalfTopBottom) -> bool {
        match BLOCK_TABLE.set_half_top_bottom(self.kind, self.state, half_top_bottom) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_half_top_bottom(mut self, half_top_bottom: HalfTopBottom) -> Self {
        self.set_half_top_bottom(half_top_bottom);
        self
    }
    pub fn half_upper_lower(self) -> Option<HalfUpperLower> {
        BLOCK_TABLE.half_upper_lower(self.kind, self.state)
    }
    pub fn set_half_upper_lower(&mut self, half_upper_lower: HalfUpperLower) -> bool {
        match BLOCK_TABLE.set_half_upper_lower(self.kind, self.state, half_upper_lower) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_half_upper_lower(mut self, half_upper_lower: HalfUpperLower) -> Self {
        self.set_half_upper_lower(half_upper_lower);
        self
    }
    pub fn has_bottle_0(self) -> Option<bool> {
        BLOCK_TABLE.has_bottle_0(self.kind, self.state)
    }
    pub fn set_has_bottle_0(&mut self, has_bottle_0: bool) -> bool {
        match BLOCK_TABLE.set_has_bottle_0(self.kind, self.state, has_bottle_0) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_has_bottle_0(mut self, has_bottle_0: bool) -> Self {
        self.set_has_bottle_0(has_bottle_0);
        self
    }
    pub fn has_bottle_1(self) -> Option<bool> {
        BLOCK_TABLE.has_bottle_1(self.kind, self.state)
    }
    pub fn set_has_bottle_1(&mut self, has_bottle_1: bool) -> bool {
        match BLOCK_TABLE.set_has_bottle_1(self.kind, self.state, has_bottle_1) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_has_bottle_1(mut self, has_bottle_1: bool) -> Self {
        self.set_has_bottle_1(has_bottle_1);
        self
    }
    pub fn has_bottle_2(self) -> Option<bool> {
        BLOCK_TABLE.has_bottle_2(self.kind, self.state)
    }
    pub fn set_has_bottle_2(&mut self, has_bottle_2: bool) -> bool {
        match BLOCK_TABLE.set_has_bottle_2(self.kind, self.state, has_bottle_2) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_has_bottle_2(mut self, has_bottle_2: bool) -> Self {
        self.set_has_bottle_2(has_bottle_2);
        self
    }
    pub fn has_record(self) -> Option<bool> {
        BLOCK_TABLE.has_record(self.kind, self.state)
    }
    pub fn set_has_record(&mut self, has_record: bool) -> bool {
        match BLOCK_TABLE.set_has_record(self.kind, self.state, has_record) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_has_record(mut self, has_record: bool) -> Self {
        self.set_has_record(has_record);
        self
    }
    pub fn hatch(self) -> Option<i32> {
        BLOCK_TABLE.hatch(self.kind, self.state)
    }
    pub fn set_hatch(&mut self, hatch: i32) -> bool {
        match BLOCK_TABLE.set_hatch(self.kind, self.state, hatch) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_hatch(mut self, hatch: i32) -> Self {
        self.set_hatch(hatch);
        self
    }
    pub fn hinge(self) -> Option<Hinge> {
        BLOCK_TABLE.hinge(self.kind, self.state)
    }
    pub fn set_hinge(&mut self, hinge: Hinge) -> bool {
        match BLOCK_TABLE.set_hinge(self.kind, self.state, hinge) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_hinge(mut self, hinge: Hinge) -> Self {
        self.set_hinge(hinge);
        self
    }
    pub fn in_wall(self) -> Option<bool> {
        BLOCK_TABLE.in_wall(self.kind, self.state)
    }
    pub fn set_in_wall(&mut self, in_wall: bool) -> bool {
        match BLOCK_TABLE.set_in_wall(self.kind, self.state, in_wall) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_in_wall(mut self, in_wall: bool) -> Self {
        self.set_in_wall(in_wall);
        self
    }
    pub fn instrument(self) -> Option<Instrument> {
        BLOCK_TABLE.instrument(self.kind, self.state)
    }
    pub fn set_instrument(&mut self, instrument: Instrument) -> bool {
        match BLOCK_TABLE.set_instrument(self.kind, self.state, instrument) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_instrument(mut self, instrument: Instrument) -> Self {
        self.set_instrument(instrument);
        self
    }
    pub fn inverted(self) -> Option<bool> {
        BLOCK_TABLE.inverted(self.kind, self.state)
    }
    pub fn set_inverted(&mut self, inverted: bool) -> bool {
        match BLOCK_TABLE.set_inverted(self.kind, self.state, inverted) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_inverted(mut self, inverted: bool) -> Self {
        self.set_inverted(inverted);
        self
    }
    pub fn layers(self) -> Option<i32> {
        BLOCK_TABLE.layers(self.kind, self.state)
    }
    pub fn set_layers(&mut self, layers: i32) -> bool {
        match BLOCK_TABLE.set_layers(self.kind, self.state, layers) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_layers(mut self, layers: i32) -> Self {
        self.set_layers(layers);
        self
    }
    pub fn lit(self) -> Option<bool> {
        BLOCK_TABLE.lit(self.kind, self.state)
    }
    pub fn set_lit(&mut self, lit: bool) -> bool {
        match BLOCK_TABLE.set_lit(self.kind, self.state, lit) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_lit(mut self, lit: bool) -> Self {
        self.set_lit(lit);
        self
    }
    pub fn locked(self) -> Option<bool> {
        BLOCK_TABLE.locked(self.kind, self.state)
    }
    pub fn set_locked(&mut self, locked: bool) -> bool {
        match BLOCK_TABLE.set_locked(self.kind, self.state, locked) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_locked(mut self, locked: bool) -> Self {
        self.set_locked(locked);
        self
    }
    pub fn moisture(self) -> Option<i32> {
        BLOCK_TABLE.moisture(self.kind, self.state)
    }
    pub fn set_moisture(&mut self, moisture: i32) -> bool {
        match BLOCK_TABLE.set_moisture(self.kind, self.state, moisture) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_moisture(mut self, moisture: i32) -> Self {
        self.set_moisture(moisture);
        self
    }
    pub fn north_connected(self) -> Option<bool> {
        BLOCK_TABLE.north_connected(self.kind, self.state)
    }
    pub fn set_north_connected(&mut self, north_connected: bool) -> bool {
        match BLOCK_TABLE.set_north_connected(self.kind, self.state, north_connected) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_north_connected(mut self, north_connected: bool) -> Self {
        self.set_north_connected(north_connected);
        self
    }
    pub fn north_wire(self) -> Option<NorthWire> {
        BLOCK_TABLE.north_wire(self.kind, self.state)
    }
    pub fn set_north_wire(&mut self, north_wire: NorthWire) -> bool {
        match BLOCK_TABLE.set_north_wire(self.kind, self.state, north_wire) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_north_wire(mut self, north_wire: NorthWire) -> Self {
        self.set_north_wire(north_wire);
        self
    }
    pub fn note(self) -> Option<i32> {
        BLOCK_TABLE.note(self.kind, self.state)
    }
    pub fn set_note(&mut self, note: i32) -> bool {
        match BLOCK_TABLE.set_note(self.kind, self.state, note) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_note(mut self, note: i32) -> Self {
        self.set_note(note);
        self
    }
    pub fn occupied(self) -> Option<bool> {
        BLOCK_TABLE.occupied(self.kind, self.state)
    }
    pub fn set_occupied(&mut self, occupied: bool) -> bool {
        match BLOCK_TABLE.set_occupied(self.kind, self.state, occupied) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_occupied(mut self, occupied: bool) -> Self {
        self.set_occupied(occupied);
        self
    }
    pub fn open(self) -> Option<bool> {
        BLOCK_TABLE.open(self.kind, self.state)
    }
    pub fn set_open(&mut self, open: bool) -> bool {
        match BLOCK_TABLE.set_open(self.kind, self.state, open) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_open(mut self, open: bool) -> Self {
        self.set_open(open);
        self
    }
    pub fn part(self) -> Option<Part> {
        BLOCK_TABLE.part(self.kind, self.state)
    }
    pub fn set_part(&mut self, part: Part) -> bool {
        match BLOCK_TABLE.set_part(self.kind, self.state, part) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_part(mut self, part: Part) -> Self {
        self.set_part(part);
        self
    }
    pub fn persistent(self) -> Option<bool> {
        BLOCK_TABLE.persistent(self.kind, self.state)
    }
    pub fn set_persistent(&mut self, persistent: bool) -> bool {
        match BLOCK_TABLE.set_persistent(self.kind, self.state, persistent) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_persistent(mut self, persistent: bool) -> Self {
        self.set_persistent(persistent);
        self
    }
    pub fn pickles(self) -> Option<i32> {
        BLOCK_TABLE.pickles(self.kind, self.state)
    }
    pub fn set_pickles(&mut self, pickles: i32) -> bool {
        match BLOCK_TABLE.set_pickles(self.kind, self.state, pickles) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_pickles(mut self, pickles: i32) -> Self {
        self.set_pickles(pickles);
        self
    }
    pub fn piston_kind(self) -> Option<PistonKind> {
        BLOCK_TABLE.piston_kind(self.kind, self.state)
    }
    pub fn set_piston_kind(&mut self, piston_kind: PistonKind) -> bool {
        match BLOCK_TABLE.set_piston_kind(self.kind, self.state, piston_kind) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_piston_kind(mut self, piston_kind: PistonKind) -> Self {
        self.set_piston_kind(piston_kind);
        self
    }
    pub fn power(self) -> Option<i32> {
        BLOCK_TABLE.power(self.kind, self.state)
    }
    pub fn set_power(&mut self, power: i32) -> bool {
        match BLOCK_TABLE.set_power(self.kind, self.state, power) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_power(mut self, power: i32) -> Self {
        self.set_power(power);
        self
    }
    pub fn powered(self) -> Option<bool> {
        BLOCK_TABLE.powered(self.kind, self.state)
    }
    pub fn set_powered(&mut self, powered: bool) -> bool {
        match BLOCK_TABLE.set_powered(self.kind, self.state, powered) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_powered(mut self, powered: bool) -> Self {
        self.set_powered(powered);
        self
    }
    pub fn powered_rail_shape(self) -> Option<PoweredRailShape> {
        BLOCK_TABLE.powered_rail_shape(self.kind, self.state)
    }
    pub fn set_powered_rail_shape(&mut self, powered_rail_shape: PoweredRailShape) -> bool {
        match BLOCK_TABLE.set_powered_rail_shape(self.kind, self.state, powered_rail_shape) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_powered_rail_shape(mut self, powered_rail_shape: PoweredRailShape) -> Self {
        self.set_powered_rail_shape(powered_rail_shape);
        self
    }
    pub fn rail_shape(self) -> Option<RailShape> {
        BLOCK_TABLE.rail_shape(self.kind, self.state)
    }
    pub fn set_rail_shape(&mut self, rail_shape: RailShape) -> bool {
        match BLOCK_TABLE.set_rail_shape(self.kind, self.state, rail_shape) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_rail_shape(mut self, rail_shape: RailShape) -> Self {
        self.set_rail_shape(rail_shape);
        self
    }
    pub fn rotation(self) -> Option<i32> {
        BLOCK_TABLE.rotation(self.kind, self.state)
    }
    pub fn set_rotation(&mut self, rotation: i32) -> bool {
        match BLOCK_TABLE.set_rotation(self.kind, self.state, rotation) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_rotation(mut self, rotation: i32) -> Self {
        self.set_rotation(rotation);
        self
    }
    pub fn short(self) -> Option<bool> {
        BLOCK_TABLE.short(self.kind, self.state)
    }
    pub fn set_short(&mut self, short: bool) -> bool {
        match BLOCK_TABLE.set_short(self.kind, self.state, short) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_short(mut self, short: bool) -> Self {
        self.set_short(short);
        self
    }
    pub fn slab_kind(self) -> Option<SlabKind> {
        BLOCK_TABLE.slab_kind(self.kind, self.state)
    }
    pub fn set_slab_kind(&mut self, slab_kind: SlabKind) -> bool {
        match BLOCK_TABLE.set_slab_kind(self.kind, self.state, slab_kind) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_slab_kind(mut self, slab_kind: SlabKind) -> Self {
        self.set_slab_kind(slab_kind);
        self
    }
    pub fn snowy(self) -> Option<bool> {
        BLOCK_TABLE.snowy(self.kind, self.state)
    }
    pub fn set_snowy(&mut self, snowy: bool) -> bool {
        match BLOCK_TABLE.set_snowy(self.kind, self.state, snowy) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_snowy(mut self, snowy: bool) -> Self {
        self.set_snowy(snowy);
        self
    }
    pub fn south_connected(self) -> Option<bool> {
        BLOCK_TABLE.south_connected(self.kind, self.state)
    }
    pub fn set_south_connected(&mut self, south_connected: bool) -> bool {
        match BLOCK_TABLE.set_south_connected(self.kind, self.state, south_connected) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_south_connected(mut self, south_connected: bool) -> Self {
        self.set_south_connected(south_connected);
        self
    }
    pub fn south_wire(self) -> Option<SouthWire> {
        BLOCK_TABLE.south_wire(self.kind, self.state)
    }
    pub fn set_south_wire(&mut self, south_wire: SouthWire) -> bool {
        match BLOCK_TABLE.set_south_wire(self.kind, self.state, south_wire) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_south_wire(mut self, south_wire: SouthWire) -> Self {
        self.set_south_wire(south_wire);
        self
    }
    pub fn stage(self) -> Option<i32> {
        BLOCK_TABLE.stage(self.kind, self.state)
    }
    pub fn set_stage(&mut self, stage: i32) -> bool {
        match BLOCK_TABLE.set_stage(self.kind, self.state, stage) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_stage(mut self, stage: i32) -> Self {
        self.set_stage(stage);
        self
    }
    pub fn stairs_shape(self) -> Option<StairsShape> {
        BLOCK_TABLE.stairs_shape(self.kind, self.state)
    }
    pub fn set_stairs_shape(&mut self, stairs_shape: StairsShape) -> bool {
        match BLOCK_TABLE.set_stairs_shape(self.kind, self.state, stairs_shape) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_stairs_shape(mut self, stairs_shape: StairsShape) -> Self {
        self.set_stairs_shape(stairs_shape);
        self
    }
    pub fn structure_block_mode(self) -> Option<StructureBlockMode> {
        BLOCK_TABLE.structure_block_mode(self.kind, self.state)
    }
    pub fn set_structure_block_mode(&mut self, structure_block_mode: StructureBlockMode) -> bool {
        match BLOCK_TABLE.set_structure_block_mode(self.kind, self.state, structure_block_mode) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_structure_block_mode(mut self, structure_block_mode: StructureBlockMode) -> Self {
        self.set_structure_block_mode(structure_block_mode);
        self
    }
    pub fn triggered(self) -> Option<bool> {
        BLOCK_TABLE.triggered(self.kind, self.state)
    }
    pub fn set_triggered(&mut self, triggered: bool) -> bool {
        match BLOCK_TABLE.set_triggered(self.kind, self.state, triggered) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_triggered(mut self, triggered: bool) -> Self {
        self.set_triggered(triggered);
        self
    }
    pub fn unstable(self) -> Option<bool> {
        BLOCK_TABLE.unstable(self.kind, self.state)
    }
    pub fn set_unstable(&mut self, unstable: bool) -> bool {
        match BLOCK_TABLE.set_unstable(self.kind, self.state, unstable) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_unstable(mut self, unstable: bool) -> Self {
        self.set_unstable(unstable);
        self
    }
    pub fn up(self) -> Option<bool> {
        BLOCK_TABLE.up(self.kind, self.state)
    }
    pub fn set_up(&mut self, up: bool) -> bool {
        match BLOCK_TABLE.set_up(self.kind, self.state, up) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_up(mut self, up: bool) -> Self {
        self.set_up(up);
        self
    }
    pub fn water_level(self) -> Option<i32> {
        BLOCK_TABLE.water_level(self.kind, self.state)
    }
    pub fn set_water_level(&mut self, water_level: i32) -> bool {
        match BLOCK_TABLE.set_water_level(self.kind, self.state, water_level) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_water_level(mut self, water_level: i32) -> Self {
        self.set_water_level(water_level);
        self
    }
    pub fn waterlogged(self) -> Option<bool> {
        BLOCK_TABLE.waterlogged(self.kind, self.state)
    }
    pub fn set_waterlogged(&mut self, waterlogged: bool) -> bool {
        match BLOCK_TABLE.set_waterlogged(self.kind, self.state, waterlogged) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_waterlogged(mut self, waterlogged: bool) -> Self {
        self.set_waterlogged(waterlogged);
        self
    }
    pub fn west_connected(self) -> Option<bool> {
        BLOCK_TABLE.west_connected(self.kind, self.state)
    }
    pub fn set_west_connected(&mut self, west_connected: bool) -> bool {
        match BLOCK_TABLE.set_west_connected(self.kind, self.state, west_connected) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_west_connected(mut self, west_connected: bool) -> Self {
        self.set_west_connected(west_connected);
        self
    }
    pub fn west_wire(self) -> Option<WestWire> {
        BLOCK_TABLE.west_wire(self.kind, self.state)
    }
    pub fn set_west_wire(&mut self, west_wire: WestWire) -> bool {
        match BLOCK_TABLE.set_west_wire(self.kind, self.state, west_wire) {
            Some(new_state) => {
                self.state = new_state;
                true
            }
            None => false,
        }
    }
    pub fn with_west_wire(mut self, west_wire: WestWire) -> Self {
        self.set_west_wire(west_wire);
        self
    }
    #[doc = "Returns the identifier of this block. For example, returns `minecraft::air` for an air block."]
    pub fn identifier(self) -> &'static str {
        match self.kind {
            BlockKind::Air => "minecraft:air",
            BlockKind::Stone => "minecraft:stone",
            BlockKind::Granite => "minecraft:granite",
            BlockKind::PolishedGranite => "minecraft:polished_granite",
            BlockKind::Diorite => "minecraft:diorite",
            BlockKind::PolishedDiorite => "minecraft:polished_diorite",
            BlockKind::Andesite => "minecraft:andesite",
            BlockKind::PolishedAndesite => "minecraft:polished_andesite",
            BlockKind::GrassBlock => "minecraft:grass_block",
            BlockKind::Dirt => "minecraft:dirt",
            BlockKind::CoarseDirt => "minecraft:coarse_dirt",
            BlockKind::Podzol => "minecraft:podzol",
            BlockKind::Cobblestone => "minecraft:cobblestone",
            BlockKind::OakPlanks => "minecraft:oak_planks",
            BlockKind::SprucePlanks => "minecraft:spruce_planks",
            BlockKind::BirchPlanks => "minecraft:birch_planks",
            BlockKind::JunglePlanks => "minecraft:jungle_planks",
            BlockKind::AcaciaPlanks => "minecraft:acacia_planks",
            BlockKind::DarkOakPlanks => "minecraft:dark_oak_planks",
            BlockKind::OakSapling => "minecraft:oak_sapling",
            BlockKind::SpruceSapling => "minecraft:spruce_sapling",
            BlockKind::BirchSapling => "minecraft:birch_sapling",
            BlockKind::JungleSapling => "minecraft:jungle_sapling",
            BlockKind::AcaciaSapling => "minecraft:acacia_sapling",
            BlockKind::DarkOakSapling => "minecraft:dark_oak_sapling",
            BlockKind::Bedrock => "minecraft:bedrock",
            BlockKind::Water => "minecraft:water",
            BlockKind::Lava => "minecraft:lava",
            BlockKind::Sand => "minecraft:sand",
            BlockKind::RedSand => "minecraft:red_sand",
            BlockKind::Gravel => "minecraft:gravel",
            BlockKind::GoldOre => "minecraft:gold_ore",
            BlockKind::IronOre => "minecraft:iron_ore",
            BlockKind::CoalOre => "minecraft:coal_ore",
            BlockKind::OakLog => "minecraft:oak_log",
            BlockKind::SpruceLog => "minecraft:spruce_log",
            BlockKind::BirchLog => "minecraft:birch_log",
            BlockKind::JungleLog => "minecraft:jungle_log",
            BlockKind::AcaciaLog => "minecraft:acacia_log",
            BlockKind::DarkOakLog => "minecraft:dark_oak_log",
            BlockKind::StrippedSpruceLog => "minecraft:stripped_spruce_log",
            BlockKind::StrippedBirchLog => "minecraft:stripped_birch_log",
            BlockKind::StrippedJungleLog => "minecraft:stripped_jungle_log",
            BlockKind::StrippedAcaciaLog => "minecraft:stripped_acacia_log",
            BlockKind::StrippedDarkOakLog => "minecraft:stripped_dark_oak_log",
            BlockKind::StrippedOakLog => "minecraft:stripped_oak_log",
            BlockKind::OakWood => "minecraft:oak_wood",
            BlockKind::SpruceWood => "minecraft:spruce_wood",
            BlockKind::BirchWood => "minecraft:birch_wood",
            BlockKind::JungleWood => "minecraft:jungle_wood",
            BlockKind::AcaciaWood => "minecraft:acacia_wood",
            BlockKind::DarkOakWood => "minecraft:dark_oak_wood",
            BlockKind::StrippedOakWood => "minecraft:stripped_oak_wood",
            BlockKind::StrippedSpruceWood => "minecraft:stripped_spruce_wood",
            BlockKind::StrippedBirchWood => "minecraft:stripped_birch_wood",
            BlockKind::StrippedJungleWood => "minecraft:stripped_jungle_wood",
            BlockKind::StrippedAcaciaWood => "minecraft:stripped_acacia_wood",
            BlockKind::StrippedDarkOakWood => "minecraft:stripped_dark_oak_wood",
            BlockKind::OakLeaves => "minecraft:oak_leaves",
            BlockKind::SpruceLeaves => "minecraft:spruce_leaves",
            BlockKind::BirchLeaves => "minecraft:birch_leaves",
            BlockKind::JungleLeaves => "minecraft:jungle_leaves",
            BlockKind::AcaciaLeaves => "minecraft:acacia_leaves",
            BlockKind::DarkOakLeaves => "minecraft:dark_oak_leaves",
            BlockKind::Sponge => "minecraft:sponge",
            BlockKind::WetSponge => "minecraft:wet_sponge",
            BlockKind::Glass => "minecraft:glass",
            BlockKind::LapisOre => "minecraft:lapis_ore",
            BlockKind::LapisBlock => "minecraft:lapis_block",
            BlockKind::Dispenser => "minecraft:dispenser",
            BlockKind::Sandstone => "minecraft:sandstone",
            BlockKind::ChiseledSandstone => "minecraft:chiseled_sandstone",
            BlockKind::CutSandstone => "minecraft:cut_sandstone",
            BlockKind::NoteBlock => "minecraft:note_block",
            BlockKind::WhiteBed => "minecraft:white_bed",
            BlockKind::OrangeBed => "minecraft:orange_bed",
            BlockKind::MagentaBed => "minecraft:magenta_bed",
            BlockKind::LightBlueBed => "minecraft:light_blue_bed",
            BlockKind::YellowBed => "minecraft:yellow_bed",
            BlockKind::LimeBed => "minecraft:lime_bed",
            BlockKind::PinkBed => "minecraft:pink_bed",
            BlockKind::GrayBed => "minecraft:gray_bed",
            BlockKind::LightGrayBed => "minecraft:light_gray_bed",
            BlockKind::CyanBed => "minecraft:cyan_bed",
            BlockKind::PurpleBed => "minecraft:purple_bed",
            BlockKind::BlueBed => "minecraft:blue_bed",
            BlockKind::BrownBed => "minecraft:brown_bed",
            BlockKind::GreenBed => "minecraft:green_bed",
            BlockKind::RedBed => "minecraft:red_bed",
            BlockKind::BlackBed => "minecraft:black_bed",
            BlockKind::PoweredRail => "minecraft:powered_rail",
            BlockKind::DetectorRail => "minecraft:detector_rail",
            BlockKind::StickyPiston => "minecraft:sticky_piston",
            BlockKind::Cobweb => "minecraft:cobweb",
            BlockKind::Grass => "minecraft:grass",
            BlockKind::Fern => "minecraft:fern",
            BlockKind::DeadBush => "minecraft:dead_bush",
            BlockKind::Seagrass => "minecraft:seagrass",
            BlockKind::TallSeagrass => "minecraft:tall_seagrass",
            BlockKind::Piston => "minecraft:piston",
            BlockKind::PistonHead => "minecraft:piston_head",
            BlockKind::WhiteWool => "minecraft:white_wool",
            BlockKind::OrangeWool => "minecraft:orange_wool",
            BlockKind::MagentaWool => "minecraft:magenta_wool",
            BlockKind::LightBlueWool => "minecraft:light_blue_wool",
            BlockKind::YellowWool => "minecraft:yellow_wool",
            BlockKind::LimeWool => "minecraft:lime_wool",
            BlockKind::PinkWool => "minecraft:pink_wool",
            BlockKind::GrayWool => "minecraft:gray_wool",
            BlockKind::LightGrayWool => "minecraft:light_gray_wool",
            BlockKind::CyanWool => "minecraft:cyan_wool",
            BlockKind::PurpleWool => "minecraft:purple_wool",
            BlockKind::BlueWool => "minecraft:blue_wool",
            BlockKind::BrownWool => "minecraft:brown_wool",
            BlockKind::GreenWool => "minecraft:green_wool",
            BlockKind::RedWool => "minecraft:red_wool",
            BlockKind::BlackWool => "minecraft:black_wool",
            BlockKind::MovingPiston => "minecraft:moving_piston",
            BlockKind::Dandelion => "minecraft:dandelion",
            BlockKind::Poppy => "minecraft:poppy",
            BlockKind::BlueOrchid => "minecraft:blue_orchid",
            BlockKind::Allium => "minecraft:allium",
            BlockKind::AzureBluet => "minecraft:azure_bluet",
            BlockKind::RedTulip => "minecraft:red_tulip",
            BlockKind::OrangeTulip => "minecraft:orange_tulip",
            BlockKind::WhiteTulip => "minecraft:white_tulip",
            BlockKind::PinkTulip => "minecraft:pink_tulip",
            BlockKind::OxeyeDaisy => "minecraft:oxeye_daisy",
            BlockKind::BrownMushroom => "minecraft:brown_mushroom",
            BlockKind::RedMushroom => "minecraft:red_mushroom",
            BlockKind::GoldBlock => "minecraft:gold_block",
            BlockKind::IronBlock => "minecraft:iron_block",
            BlockKind::Bricks => "minecraft:bricks",
            BlockKind::Tnt => "minecraft:tnt",
            BlockKind::Bookshelf => "minecraft:bookshelf",
            BlockKind::MossyCobblestone => "minecraft:mossy_cobblestone",
            BlockKind::Obsidian => "minecraft:obsidian",
            BlockKind::Torch => "minecraft:torch",
            BlockKind::WallTorch => "minecraft:wall_torch",
            BlockKind::Fire => "minecraft:fire",
            BlockKind::Spawner => "minecraft:spawner",
            BlockKind::OakStairs => "minecraft:oak_stairs",
            BlockKind::Chest => "minecraft:chest",
            BlockKind::RedstoneWire => "minecraft:redstone_wire",
            BlockKind::DiamondOre => "minecraft:diamond_ore",
            BlockKind::DiamondBlock => "minecraft:diamond_block",
            BlockKind::CraftingTable => "minecraft:crafting_table",
            BlockKind::Wheat => "minecraft:wheat",
            BlockKind::Farmland => "minecraft:farmland",
            BlockKind::Furnace => "minecraft:furnace",
            BlockKind::Sign => "minecraft:sign",
            BlockKind::OakDoor => "minecraft:oak_door",
            BlockKind::Ladder => "minecraft:ladder",
            BlockKind::Rail => "minecraft:rail",
            BlockKind::CobblestoneStairs => "minecraft:cobblestone_stairs",
            BlockKind::WallSign => "minecraft:wall_sign",
            BlockKind::Lever => "minecraft:lever",
            BlockKind::StonePressurePlate => "minecraft:stone_pressure_plate",
            BlockKind::IronDoor => "minecraft:iron_door",
            BlockKind::OakPressurePlate => "minecraft:oak_pressure_plate",
            BlockKind::SprucePressurePlate => "minecraft:spruce_pressure_plate",
            BlockKind::BirchPressurePlate => "minecraft:birch_pressure_plate",
            BlockKind::JunglePressurePlate => "minecraft:jungle_pressure_plate",
            BlockKind::AcaciaPressurePlate => "minecraft:acacia_pressure_plate",
            BlockKind::DarkOakPressurePlate => "minecraft:dark_oak_pressure_plate",
            BlockKind::RedstoneOre => "minecraft:redstone_ore",
            BlockKind::RedstoneTorch => "minecraft:redstone_torch",
            BlockKind::RedstoneWallTorch => "minecraft:redstone_wall_torch",
            BlockKind::StoneButton => "minecraft:stone_button",
            BlockKind::Snow => "minecraft:snow",
            BlockKind::Ice => "minecraft:ice",
            BlockKind::SnowBlock => "minecraft:snow_block",
            BlockKind::Cactus => "minecraft:cactus",
            BlockKind::Clay => "minecraft:clay",
            BlockKind::SugarCane => "minecraft:sugar_cane",
            BlockKind::Jukebox => "minecraft:jukebox",
            BlockKind::OakFence => "minecraft:oak_fence",
            BlockKind::Pumpkin => "minecraft:pumpkin",
            BlockKind::Netherrack => "minecraft:netherrack",
            BlockKind::SoulSand => "minecraft:soul_sand",
            BlockKind::Glowstone => "minecraft:glowstone",
            BlockKind::NetherPortal => "minecraft:nether_portal",
            BlockKind::CarvedPumpkin => "minecraft:carved_pumpkin",
            BlockKind::JackOLantern => "minecraft:jack_o_lantern",
            BlockKind::Cake => "minecraft:cake",
            BlockKind::Repeater => "minecraft:repeater",
            BlockKind::WhiteStainedGlass => "minecraft:white_stained_glass",
            BlockKind::OrangeStainedGlass => "minecraft:orange_stained_glass",
            BlockKind::MagentaStainedGlass => "minecraft:magenta_stained_glass",
            BlockKind::LightBlueStainedGlass => "minecraft:light_blue_stained_glass",
            BlockKind::YellowStainedGlass => "minecraft:yellow_stained_glass",
            BlockKind::LimeStainedGlass => "minecraft:lime_stained_glass",
            BlockKind::PinkStainedGlass => "minecraft:pink_stained_glass",
            BlockKind::GrayStainedGlass => "minecraft:gray_stained_glass",
            BlockKind::LightGrayStainedGlass => "minecraft:light_gray_stained_glass",
            BlockKind::CyanStainedGlass => "minecraft:cyan_stained_glass",
            BlockKind::PurpleStainedGlass => "minecraft:purple_stained_glass",
            BlockKind::BlueStainedGlass => "minecraft:blue_stained_glass",
            BlockKind::BrownStainedGlass => "minecraft:brown_stained_glass",
            BlockKind::GreenStainedGlass => "minecraft:green_stained_glass",
            BlockKind::RedStainedGlass => "minecraft:red_stained_glass",
            BlockKind::BlackStainedGlass => "minecraft:black_stained_glass",
            BlockKind::OakTrapdoor => "minecraft:oak_trapdoor",
            BlockKind::SpruceTrapdoor => "minecraft:spruce_trapdoor",
            BlockKind::BirchTrapdoor => "minecraft:birch_trapdoor",
            BlockKind::JungleTrapdoor => "minecraft:jungle_trapdoor",
            BlockKind::AcaciaTrapdoor => "minecraft:acacia_trapdoor",
            BlockKind::DarkOakTrapdoor => "minecraft:dark_oak_trapdoor",
            BlockKind::InfestedStone => "minecraft:infested_stone",
            BlockKind::InfestedCobblestone => "minecraft:infested_cobblestone",
            BlockKind::InfestedStoneBricks => "minecraft:infested_stone_bricks",
            BlockKind::InfestedMossyStoneBricks => "minecraft:infested_mossy_stone_bricks",
            BlockKind::InfestedCrackedStoneBricks => "minecraft:infested_cracked_stone_bricks",
            BlockKind::InfestedChiseledStoneBricks => "minecraft:infested_chiseled_stone_bricks",
            BlockKind::StoneBricks => "minecraft:stone_bricks",
            BlockKind::MossyStoneBricks => "minecraft:mossy_stone_bricks",
            BlockKind::CrackedStoneBricks => "minecraft:cracked_stone_bricks",
            BlockKind::ChiseledStoneBricks => "minecraft:chiseled_stone_bricks",
            BlockKind::BrownMushroomBlock => "minecraft:brown_mushroom_block",
            BlockKind::RedMushroomBlock => "minecraft:red_mushroom_block",
            BlockKind::MushroomStem => "minecraft:mushroom_stem",
            BlockKind::IronBars => "minecraft:iron_bars",
            BlockKind::GlassPane => "minecraft:glass_pane",
            BlockKind::Melon => "minecraft:melon",
            BlockKind::AttachedPumpkinStem => "minecraft:attached_pumpkin_stem",
            BlockKind::AttachedMelonStem => "minecraft:attached_melon_stem",
            BlockKind::PumpkinStem => "minecraft:pumpkin_stem",
            BlockKind::MelonStem => "minecraft:melon_stem",
            BlockKind::Vine => "minecraft:vine",
            BlockKind::OakFenceGate => "minecraft:oak_fence_gate",
            BlockKind::BrickStairs => "minecraft:brick_stairs",
            BlockKind::StoneBrickStairs => "minecraft:stone_brick_stairs",
            BlockKind::Mycelium => "minecraft:mycelium",
            BlockKind::LilyPad => "minecraft:lily_pad",
            BlockKind::NetherBricks => "minecraft:nether_bricks",
            BlockKind::NetherBrickFence => "minecraft:nether_brick_fence",
            BlockKind::NetherBrickStairs => "minecraft:nether_brick_stairs",
            BlockKind::NetherWart => "minecraft:nether_wart",
            BlockKind::EnchantingTable => "minecraft:enchanting_table",
            BlockKind::BrewingStand => "minecraft:brewing_stand",
            BlockKind::Cauldron => "minecraft:cauldron",
            BlockKind::EndPortal => "minecraft:end_portal",
            BlockKind::EndPortalFrame => "minecraft:end_portal_frame",
            BlockKind::EndStone => "minecraft:end_stone",
            BlockKind::DragonEgg => "minecraft:dragon_egg",
            BlockKind::RedstoneLamp => "minecraft:redstone_lamp",
            BlockKind::Cocoa => "minecraft:cocoa",
            BlockKind::SandstoneStairs => "minecraft:sandstone_stairs",
            BlockKind::EmeraldOre => "minecraft:emerald_ore",
            BlockKind::EnderChest => "minecraft:ender_chest",
            BlockKind::TripwireHook => "minecraft:tripwire_hook",
            BlockKind::Tripwire => "minecraft:tripwire",
            BlockKind::EmeraldBlock => "minecraft:emerald_block",
            BlockKind::SpruceStairs => "minecraft:spruce_stairs",
            BlockKind::BirchStairs => "minecraft:birch_stairs",
            BlockKind::JungleStairs => "minecraft:jungle_stairs",
            BlockKind::CommandBlock => "minecraft:command_block",
            BlockKind::Beacon => "minecraft:beacon",
            BlockKind::CobblestoneWall => "minecraft:cobblestone_wall",
            BlockKind::MossyCobblestoneWall => "minecraft:mossy_cobblestone_wall",
            BlockKind::FlowerPot => "minecraft:flower_pot",
            BlockKind::PottedOakSapling => "minecraft:potted_oak_sapling",
            BlockKind::PottedSpruceSapling => "minecraft:potted_spruce_sapling",
            BlockKind::PottedBirchSapling => "minecraft:potted_birch_sapling",
            BlockKind::PottedJungleSapling => "minecraft:potted_jungle_sapling",
            BlockKind::PottedAcaciaSapling => "minecraft:potted_acacia_sapling",
            BlockKind::PottedDarkOakSapling => "minecraft:potted_dark_oak_sapling",
            BlockKind::PottedFern => "minecraft:potted_fern",
            BlockKind::PottedDandelion => "minecraft:potted_dandelion",
            BlockKind::PottedPoppy => "minecraft:potted_poppy",
            BlockKind::PottedBlueOrchid => "minecraft:potted_blue_orchid",
            BlockKind::PottedAllium => "minecraft:potted_allium",
            BlockKind::PottedAzureBluet => "minecraft:potted_azure_bluet",
            BlockKind::PottedRedTulip => "minecraft:potted_red_tulip",
            BlockKind::PottedOrangeTulip => "minecraft:potted_orange_tulip",
            BlockKind::PottedWhiteTulip => "minecraft:potted_white_tulip",
            BlockKind::PottedPinkTulip => "minecraft:potted_pink_tulip",
            BlockKind::PottedOxeyeDaisy => "minecraft:potted_oxeye_daisy",
            BlockKind::PottedRedMushroom => "minecraft:potted_red_mushroom",
            BlockKind::PottedBrownMushroom => "minecraft:potted_brown_mushroom",
            BlockKind::PottedDeadBush => "minecraft:potted_dead_bush",
            BlockKind::PottedCactus => "minecraft:potted_cactus",
            BlockKind::Carrots => "minecraft:carrots",
            BlockKind::Potatoes => "minecraft:potatoes",
            BlockKind::OakButton => "minecraft:oak_button",
            BlockKind::SpruceButton => "minecraft:spruce_button",
            BlockKind::BirchButton => "minecraft:birch_button",
            BlockKind::JungleButton => "minecraft:jungle_button",
            BlockKind::AcaciaButton => "minecraft:acacia_button",
            BlockKind::DarkOakButton => "minecraft:dark_oak_button",
            BlockKind::SkeletonWallSkull => "minecraft:skeleton_wall_skull",
            BlockKind::SkeletonSkull => "minecraft:skeleton_skull",
            BlockKind::WitherSkeletonWallSkull => "minecraft:wither_skeleton_wall_skull",
            BlockKind::WitherSkeletonSkull => "minecraft:wither_skeleton_skull",
            BlockKind::ZombieWallHead => "minecraft:zombie_wall_head",
            BlockKind::ZombieHead => "minecraft:zombie_head",
            BlockKind::PlayerWallHead => "minecraft:player_wall_head",
            BlockKind::PlayerHead => "minecraft:player_head",
            BlockKind::CreeperWallHead => "minecraft:creeper_wall_head",
            BlockKind::CreeperHead => "minecraft:creeper_head",
            BlockKind::DragonWallHead => "minecraft:dragon_wall_head",
            BlockKind::DragonHead => "minecraft:dragon_head",
            BlockKind::Anvil => "minecraft:anvil",
            BlockKind::ChippedAnvil => "minecraft:chipped_anvil",
            BlockKind::DamagedAnvil => "minecraft:damaged_anvil",
            BlockKind::TrappedChest => "minecraft:trapped_chest",
            BlockKind::LightWeightedPressurePlate => "minecraft:light_weighted_pressure_plate",
            BlockKind::HeavyWeightedPressurePlate => "minecraft:heavy_weighted_pressure_plate",
            BlockKind::Comparator => "minecraft:comparator",
            BlockKind::DaylightDetector => "minecraft:daylight_detector",
            BlockKind::RedstoneBlock => "minecraft:redstone_block",
            BlockKind::NetherQuartzOre => "minecraft:nether_quartz_ore",
            BlockKind::Hopper => "minecraft:hopper",
            BlockKind::QuartzBlock => "minecraft:quartz_block",
            BlockKind::ChiseledQuartzBlock => "minecraft:chiseled_quartz_block",
            BlockKind::QuartzPillar => "minecraft:quartz_pillar",
            BlockKind::QuartzStairs => "minecraft:quartz_stairs",
            BlockKind::ActivatorRail => "minecraft:activator_rail",
            BlockKind::Dropper => "minecraft:dropper",
            BlockKind::WhiteTerracotta => "minecraft:white_terracotta",
            BlockKind::OrangeTerracotta => "minecraft:orange_terracotta",
            BlockKind::MagentaTerracotta => "minecraft:magenta_terracotta",
            BlockKind::LightBlueTerracotta => "minecraft:light_blue_terracotta",
            BlockKind::YellowTerracotta => "minecraft:yellow_terracotta",
            BlockKind::LimeTerracotta => "minecraft:lime_terracotta",
            BlockKind::PinkTerracotta => "minecraft:pink_terracotta",
            BlockKind::GrayTerracotta => "minecraft:gray_terracotta",
            BlockKind::LightGrayTerracotta => "minecraft:light_gray_terracotta",
            BlockKind::CyanTerracotta => "minecraft:cyan_terracotta",
            BlockKind::PurpleTerracotta => "minecraft:purple_terracotta",
            BlockKind::BlueTerracotta => "minecraft:blue_terracotta",
            BlockKind::BrownTerracotta => "minecraft:brown_terracotta",
            BlockKind::GreenTerracotta => "minecraft:green_terracotta",
            BlockKind::RedTerracotta => "minecraft:red_terracotta",
            BlockKind::BlackTerracotta => "minecraft:black_terracotta",
            BlockKind::WhiteStainedGlassPane => "minecraft:white_stained_glass_pane",
            BlockKind::OrangeStainedGlassPane => "minecraft:orange_stained_glass_pane",
            BlockKind::MagentaStainedGlassPane => "minecraft:magenta_stained_glass_pane",
            BlockKind::LightBlueStainedGlassPane => "minecraft:light_blue_stained_glass_pane",
            BlockKind::YellowStainedGlassPane => "minecraft:yellow_stained_glass_pane",
            BlockKind::LimeStainedGlassPane => "minecraft:lime_stained_glass_pane",
            BlockKind::PinkStainedGlassPane => "minecraft:pink_stained_glass_pane",
            BlockKind::GrayStainedGlassPane => "minecraft:gray_stained_glass_pane",
            BlockKind::LightGrayStainedGlassPane => "minecraft:light_gray_stained_glass_pane",
            BlockKind::CyanStainedGlassPane => "minecraft:cyan_stained_glass_pane",
            BlockKind::PurpleStainedGlassPane => "minecraft:purple_stained_glass_pane",
            BlockKind::BlueStainedGlassPane => "minecraft:blue_stained_glass_pane",
            BlockKind::BrownStainedGlassPane => "minecraft:brown_stained_glass_pane",
            BlockKind::GreenStainedGlassPane => "minecraft:green_stained_glass_pane",
            BlockKind::RedStainedGlassPane => "minecraft:red_stained_glass_pane",
            BlockKind::BlackStainedGlassPane => "minecraft:black_stained_glass_pane",
            BlockKind::AcaciaStairs => "minecraft:acacia_stairs",
            BlockKind::DarkOakStairs => "minecraft:dark_oak_stairs",
            BlockKind::SlimeBlock => "minecraft:slime_block",
            BlockKind::Barrier => "minecraft:barrier",
            BlockKind::IronTrapdoor => "minecraft:iron_trapdoor",
            BlockKind::Prismarine => "minecraft:prismarine",
            BlockKind::PrismarineBricks => "minecraft:prismarine_bricks",
            BlockKind::DarkPrismarine => "minecraft:dark_prismarine",
            BlockKind::PrismarineStairs => "minecraft:prismarine_stairs",
            BlockKind::PrismarineBrickStairs => "minecraft:prismarine_brick_stairs",
            BlockKind::DarkPrismarineStairs => "minecraft:dark_prismarine_stairs",
            BlockKind::PrismarineSlab => "minecraft:prismarine_slab",
            BlockKind::PrismarineBrickSlab => "minecraft:prismarine_brick_slab",
            BlockKind::DarkPrismarineSlab => "minecraft:dark_prismarine_slab",
            BlockKind::SeaLantern => "minecraft:sea_lantern",
            BlockKind::HayBlock => "minecraft:hay_block",
            BlockKind::WhiteCarpet => "minecraft:white_carpet",
            BlockKind::OrangeCarpet => "minecraft:orange_carpet",
            BlockKind::MagentaCarpet => "minecraft:magenta_carpet",
            BlockKind::LightBlueCarpet => "minecraft:light_blue_carpet",
            BlockKind::YellowCarpet => "minecraft:yellow_carpet",
            BlockKind::LimeCarpet => "minecraft:lime_carpet",
            BlockKind::PinkCarpet => "minecraft:pink_carpet",
            BlockKind::GrayCarpet => "minecraft:gray_carpet",
            BlockKind::LightGrayCarpet => "minecraft:light_gray_carpet",
            BlockKind::CyanCarpet => "minecraft:cyan_carpet",
            BlockKind::PurpleCarpet => "minecraft:purple_carpet",
            BlockKind::BlueCarpet => "minecraft:blue_carpet",
            BlockKind::BrownCarpet => "minecraft:brown_carpet",
            BlockKind::GreenCarpet => "minecraft:green_carpet",
            BlockKind::RedCarpet => "minecraft:red_carpet",
            BlockKind::BlackCarpet => "minecraft:black_carpet",
            BlockKind::Terracotta => "minecraft:terracotta",
            BlockKind::CoalBlock => "minecraft:coal_block",
            BlockKind::PackedIce => "minecraft:packed_ice",
            BlockKind::Sunflower => "minecraft:sunflower",
            BlockKind::Lilac => "minecraft:lilac",
            BlockKind::RoseBush => "minecraft:rose_bush",
            BlockKind::Peony => "minecraft:peony",
            BlockKind::TallGrass => "minecraft:tall_grass",
            BlockKind::LargeFern => "minecraft:large_fern",
            BlockKind::WhiteBanner => "minecraft:white_banner",
            BlockKind::OrangeBanner => "minecraft:orange_banner",
            BlockKind::MagentaBanner => "minecraft:magenta_banner",
            BlockKind::LightBlueBanner => "minecraft:light_blue_banner",
            BlockKind::YellowBanner => "minecraft:yellow_banner",
            BlockKind::LimeBanner => "minecraft:lime_banner",
            BlockKind::PinkBanner => "minecraft:pink_banner",
            BlockKind::GrayBanner => "minecraft:gray_banner",
            BlockKind::LightGrayBanner => "minecraft:light_gray_banner",
            BlockKind::CyanBanner => "minecraft:cyan_banner",
            BlockKind::PurpleBanner => "minecraft:purple_banner",
            BlockKind::BlueBanner => "minecraft:blue_banner",
            BlockKind::BrownBanner => "minecraft:brown_banner",
            BlockKind::GreenBanner => "minecraft:green_banner",
            BlockKind::RedBanner => "minecraft:red_banner",
            BlockKind::BlackBanner => "minecraft:black_banner",
            BlockKind::WhiteWallBanner => "minecraft:white_wall_banner",
            BlockKind::OrangeWallBanner => "minecraft:orange_wall_banner",
            BlockKind::MagentaWallBanner => "minecraft:magenta_wall_banner",
            BlockKind::LightBlueWallBanner => "minecraft:light_blue_wall_banner",
            BlockKind::YellowWallBanner => "minecraft:yellow_wall_banner",
            BlockKind::LimeWallBanner => "minecraft:lime_wall_banner",
            BlockKind::PinkWallBanner => "minecraft:pink_wall_banner",
            BlockKind::GrayWallBanner => "minecraft:gray_wall_banner",
            BlockKind::LightGrayWallBanner => "minecraft:light_gray_wall_banner",
            BlockKind::CyanWallBanner => "minecraft:cyan_wall_banner",
            BlockKind::PurpleWallBanner => "minecraft:purple_wall_banner",
            BlockKind::BlueWallBanner => "minecraft:blue_wall_banner",
            BlockKind::BrownWallBanner => "minecraft:brown_wall_banner",
            BlockKind::GreenWallBanner => "minecraft:green_wall_banner",
            BlockKind::RedWallBanner => "minecraft:red_wall_banner",
            BlockKind::BlackWallBanner => "minecraft:black_wall_banner",
            BlockKind::RedSandstone => "minecraft:red_sandstone",
            BlockKind::ChiseledRedSandstone => "minecraft:chiseled_red_sandstone",
            BlockKind::CutRedSandstone => "minecraft:cut_red_sandstone",
            BlockKind::RedSandstoneStairs => "minecraft:red_sandstone_stairs",
            BlockKind::OakSlab => "minecraft:oak_slab",
            BlockKind::SpruceSlab => "minecraft:spruce_slab",
            BlockKind::BirchSlab => "minecraft:birch_slab",
            BlockKind::JungleSlab => "minecraft:jungle_slab",
            BlockKind::AcaciaSlab => "minecraft:acacia_slab",
            BlockKind::DarkOakSlab => "minecraft:dark_oak_slab",
            BlockKind::StoneSlab => "minecraft:stone_slab",
            BlockKind::SandstoneSlab => "minecraft:sandstone_slab",
            BlockKind::PetrifiedOakSlab => "minecraft:petrified_oak_slab",
            BlockKind::CobblestoneSlab => "minecraft:cobblestone_slab",
            BlockKind::BrickSlab => "minecraft:brick_slab",
            BlockKind::StoneBrickSlab => "minecraft:stone_brick_slab",
            BlockKind::NetherBrickSlab => "minecraft:nether_brick_slab",
            BlockKind::QuartzSlab => "minecraft:quartz_slab",
            BlockKind::RedSandstoneSlab => "minecraft:red_sandstone_slab",
            BlockKind::PurpurSlab => "minecraft:purpur_slab",
            BlockKind::SmoothStone => "minecraft:smooth_stone",
            BlockKind::SmoothSandstone => "minecraft:smooth_sandstone",
            BlockKind::SmoothQuartz => "minecraft:smooth_quartz",
            BlockKind::SmoothRedSandstone => "minecraft:smooth_red_sandstone",
            BlockKind::SpruceFenceGate => "minecraft:spruce_fence_gate",
            BlockKind::BirchFenceGate => "minecraft:birch_fence_gate",
            BlockKind::JungleFenceGate => "minecraft:jungle_fence_gate",
            BlockKind::AcaciaFenceGate => "minecraft:acacia_fence_gate",
            BlockKind::DarkOakFenceGate => "minecraft:dark_oak_fence_gate",
            BlockKind::SpruceFence => "minecraft:spruce_fence",
            BlockKind::BirchFence => "minecraft:birch_fence",
            BlockKind::JungleFence => "minecraft:jungle_fence",
            BlockKind::AcaciaFence => "minecraft:acacia_fence",
            BlockKind::DarkOakFence => "minecraft:dark_oak_fence",
            BlockKind::SpruceDoor => "minecraft:spruce_door",
            BlockKind::BirchDoor => "minecraft:birch_door",
            BlockKind::JungleDoor => "minecraft:jungle_door",
            BlockKind::AcaciaDoor => "minecraft:acacia_door",
            BlockKind::DarkOakDoor => "minecraft:dark_oak_door",
            BlockKind::EndRod => "minecraft:end_rod",
            BlockKind::ChorusPlant => "minecraft:chorus_plant",
            BlockKind::ChorusFlower => "minecraft:chorus_flower",
            BlockKind::PurpurBlock => "minecraft:purpur_block",
            BlockKind::PurpurPillar => "minecraft:purpur_pillar",
            BlockKind::PurpurStairs => "minecraft:purpur_stairs",
            BlockKind::EndStoneBricks => "minecraft:end_stone_bricks",
            BlockKind::Beetroots => "minecraft:beetroots",
            BlockKind::GrassPath => "minecraft:grass_path",
            BlockKind::EndGateway => "minecraft:end_gateway",
            BlockKind::RepeatingCommandBlock => "minecraft:repeating_command_block",
            BlockKind::ChainCommandBlock => "minecraft:chain_command_block",
            BlockKind::FrostedIce => "minecraft:frosted_ice",
            BlockKind::MagmaBlock => "minecraft:magma_block",
            BlockKind::NetherWartBlock => "minecraft:nether_wart_block",
            BlockKind::RedNetherBricks => "minecraft:red_nether_bricks",
            BlockKind::BoneBlock => "minecraft:bone_block",
            BlockKind::StructureVoid => "minecraft:structure_void",
            BlockKind::Observer => "minecraft:observer",
            BlockKind::ShulkerBox => "minecraft:shulker_box",
            BlockKind::WhiteShulkerBox => "minecraft:white_shulker_box",
            BlockKind::OrangeShulkerBox => "minecraft:orange_shulker_box",
            BlockKind::MagentaShulkerBox => "minecraft:magenta_shulker_box",
            BlockKind::LightBlueShulkerBox => "minecraft:light_blue_shulker_box",
            BlockKind::YellowShulkerBox => "minecraft:yellow_shulker_box",
            BlockKind::LimeShulkerBox => "minecraft:lime_shulker_box",
            BlockKind::PinkShulkerBox => "minecraft:pink_shulker_box",
            BlockKind::GrayShulkerBox => "minecraft:gray_shulker_box",
            BlockKind::LightGrayShulkerBox => "minecraft:light_gray_shulker_box",
            BlockKind::CyanShulkerBox => "minecraft:cyan_shulker_box",
            BlockKind::PurpleShulkerBox => "minecraft:purple_shulker_box",
            BlockKind::BlueShulkerBox => "minecraft:blue_shulker_box",
            BlockKind::BrownShulkerBox => "minecraft:brown_shulker_box",
            BlockKind::GreenShulkerBox => "minecraft:green_shulker_box",
            BlockKind::RedShulkerBox => "minecraft:red_shulker_box",
            BlockKind::BlackShulkerBox => "minecraft:black_shulker_box",
            BlockKind::WhiteGlazedTerracotta => "minecraft:white_glazed_terracotta",
            BlockKind::OrangeGlazedTerracotta => "minecraft:orange_glazed_terracotta",
            BlockKind::MagentaGlazedTerracotta => "minecraft:magenta_glazed_terracotta",
            BlockKind::LightBlueGlazedTerracotta => "minecraft:light_blue_glazed_terracotta",
            BlockKind::YellowGlazedTerracotta => "minecraft:yellow_glazed_terracotta",
            BlockKind::LimeGlazedTerracotta => "minecraft:lime_glazed_terracotta",
            BlockKind::PinkGlazedTerracotta => "minecraft:pink_glazed_terracotta",
            BlockKind::GrayGlazedTerracotta => "minecraft:gray_glazed_terracotta",
            BlockKind::LightGrayGlazedTerracotta => "minecraft:light_gray_glazed_terracotta",
            BlockKind::CyanGlazedTerracotta => "minecraft:cyan_glazed_terracotta",
            BlockKind::PurpleGlazedTerracotta => "minecraft:purple_glazed_terracotta",
            BlockKind::BlueGlazedTerracotta => "minecraft:blue_glazed_terracotta",
            BlockKind::BrownGlazedTerracotta => "minecraft:brown_glazed_terracotta",
            BlockKind::GreenGlazedTerracotta => "minecraft:green_glazed_terracotta",
            BlockKind::RedGlazedTerracotta => "minecraft:red_glazed_terracotta",
            BlockKind::BlackGlazedTerracotta => "minecraft:black_glazed_terracotta",
            BlockKind::WhiteConcrete => "minecraft:white_concrete",
            BlockKind::OrangeConcrete => "minecraft:orange_concrete",
            BlockKind::MagentaConcrete => "minecraft:magenta_concrete",
            BlockKind::LightBlueConcrete => "minecraft:light_blue_concrete",
            BlockKind::YellowConcrete => "minecraft:yellow_concrete",
            BlockKind::LimeConcrete => "minecraft:lime_concrete",
            BlockKind::PinkConcrete => "minecraft:pink_concrete",
            BlockKind::GrayConcrete => "minecraft:gray_concrete",
            BlockKind::LightGrayConcrete => "minecraft:light_gray_concrete",
            BlockKind::CyanConcrete => "minecraft:cyan_concrete",
            BlockKind::PurpleConcrete => "minecraft:purple_concrete",
            BlockKind::BlueConcrete => "minecraft:blue_concrete",
            BlockKind::BrownConcrete => "minecraft:brown_concrete",
            BlockKind::GreenConcrete => "minecraft:green_concrete",
            BlockKind::RedConcrete => "minecraft:red_concrete",
            BlockKind::BlackConcrete => "minecraft:black_concrete",
            BlockKind::WhiteConcretePowder => "minecraft:white_concrete_powder",
            BlockKind::OrangeConcretePowder => "minecraft:orange_concrete_powder",
            BlockKind::MagentaConcretePowder => "minecraft:magenta_concrete_powder",
            BlockKind::LightBlueConcretePowder => "minecraft:light_blue_concrete_powder",
            BlockKind::YellowConcretePowder => "minecraft:yellow_concrete_powder",
            BlockKind::LimeConcretePowder => "minecraft:lime_concrete_powder",
            BlockKind::PinkConcretePowder => "minecraft:pink_concrete_powder",
            BlockKind::GrayConcretePowder => "minecraft:gray_concrete_powder",
            BlockKind::LightGrayConcretePowder => "minecraft:light_gray_concrete_powder",
            BlockKind::CyanConcretePowder => "minecraft:cyan_concrete_powder",
            BlockKind::PurpleConcretePowder => "minecraft:purple_concrete_powder",
            BlockKind::BlueConcretePowder => "minecraft:blue_concrete_powder",
            BlockKind::BrownConcretePowder => "minecraft:brown_concrete_powder",
            BlockKind::GreenConcretePowder => "minecraft:green_concrete_powder",
            BlockKind::RedConcretePowder => "minecraft:red_concrete_powder",
            BlockKind::BlackConcretePowder => "minecraft:black_concrete_powder",
            BlockKind::Kelp => "minecraft:kelp",
            BlockKind::KelpPlant => "minecraft:kelp_plant",
            BlockKind::DriedKelpBlock => "minecraft:dried_kelp_block",
            BlockKind::TurtleEgg => "minecraft:turtle_egg",
            BlockKind::DeadTubeCoralBlock => "minecraft:dead_tube_coral_block",
            BlockKind::DeadBrainCoralBlock => "minecraft:dead_brain_coral_block",
            BlockKind::DeadBubbleCoralBlock => "minecraft:dead_bubble_coral_block",
            BlockKind::DeadFireCoralBlock => "minecraft:dead_fire_coral_block",
            BlockKind::DeadHornCoralBlock => "minecraft:dead_horn_coral_block",
            BlockKind::TubeCoralBlock => "minecraft:tube_coral_block",
            BlockKind::BrainCoralBlock => "minecraft:brain_coral_block",
            BlockKind::BubbleCoralBlock => "minecraft:bubble_coral_block",
            BlockKind::FireCoralBlock => "minecraft:fire_coral_block",
            BlockKind::HornCoralBlock => "minecraft:horn_coral_block",
            BlockKind::DeadTubeCoral => "minecraft:dead_tube_coral",
            BlockKind::DeadBrainCoral => "minecraft:dead_brain_coral",
            BlockKind::DeadBubbleCoral => "minecraft:dead_bubble_coral",
            BlockKind::DeadFireCoral => "minecraft:dead_fire_coral",
            BlockKind::DeadHornCoral => "minecraft:dead_horn_coral",
            BlockKind::TubeCoral => "minecraft:tube_coral",
            BlockKind::BrainCoral => "minecraft:brain_coral",
            BlockKind::BubbleCoral => "minecraft:bubble_coral",
            BlockKind::FireCoral => "minecraft:fire_coral",
            BlockKind::HornCoral => "minecraft:horn_coral",
            BlockKind::DeadTubeCoralWallFan => "minecraft:dead_tube_coral_wall_fan",
            BlockKind::DeadBrainCoralWallFan => "minecraft:dead_brain_coral_wall_fan",
            BlockKind::DeadBubbleCoralWallFan => "minecraft:dead_bubble_coral_wall_fan",
            BlockKind::DeadFireCoralWallFan => "minecraft:dead_fire_coral_wall_fan",
            BlockKind::DeadHornCoralWallFan => "minecraft:dead_horn_coral_wall_fan",
            BlockKind::TubeCoralWallFan => "minecraft:tube_coral_wall_fan",
            BlockKind::BrainCoralWallFan => "minecraft:brain_coral_wall_fan",
            BlockKind::BubbleCoralWallFan => "minecraft:bubble_coral_wall_fan",
            BlockKind::FireCoralWallFan => "minecraft:fire_coral_wall_fan",
            BlockKind::HornCoralWallFan => "minecraft:horn_coral_wall_fan",
            BlockKind::DeadTubeCoralFan => "minecraft:dead_tube_coral_fan",
            BlockKind::DeadBrainCoralFan => "minecraft:dead_brain_coral_fan",
            BlockKind::DeadBubbleCoralFan => "minecraft:dead_bubble_coral_fan",
            BlockKind::DeadFireCoralFan => "minecraft:dead_fire_coral_fan",
            BlockKind::DeadHornCoralFan => "minecraft:dead_horn_coral_fan",
            BlockKind::TubeCoralFan => "minecraft:tube_coral_fan",
            BlockKind::BrainCoralFan => "minecraft:brain_coral_fan",
            BlockKind::BubbleCoralFan => "minecraft:bubble_coral_fan",
            BlockKind::FireCoralFan => "minecraft:fire_coral_fan",
            BlockKind::HornCoralFan => "minecraft:horn_coral_fan",
            BlockKind::SeaPickle => "minecraft:sea_pickle",
            BlockKind::BlueIce => "minecraft:blue_ice",
            BlockKind::Conduit => "minecraft:conduit",
            BlockKind::VoidAir => "minecraft:void_air",
            BlockKind::CaveAir => "minecraft:cave_air",
            BlockKind::BubbleColumn => "minecraft:bubble_column",
            BlockKind::StructureBlock => "minecraft:structure_block",
        }
    }
    #[doc = "Returns a mapping from property name to property value for this block. Used to serialize blocks in vanilla world saves."]
    pub fn to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        match self.kind {
            BlockKind::Air => self.air_to_properties_map(),
            BlockKind::Stone => self.stone_to_properties_map(),
            BlockKind::Granite => self.granite_to_properties_map(),
            BlockKind::PolishedGranite => self.polished_granite_to_properties_map(),
            BlockKind::Diorite => self.diorite_to_properties_map(),
            BlockKind::PolishedDiorite => self.polished_diorite_to_properties_map(),
            BlockKind::Andesite => self.andesite_to_properties_map(),
            BlockKind::PolishedAndesite => self.polished_andesite_to_properties_map(),
            BlockKind::GrassBlock => self.grass_block_to_properties_map(),
            BlockKind::Dirt => self.dirt_to_properties_map(),
            BlockKind::CoarseDirt => self.coarse_dirt_to_properties_map(),
            BlockKind::Podzol => self.podzol_to_properties_map(),
            BlockKind::Cobblestone => self.cobblestone_to_properties_map(),
            BlockKind::OakPlanks => self.oak_planks_to_properties_map(),
            BlockKind::SprucePlanks => self.spruce_planks_to_properties_map(),
            BlockKind::BirchPlanks => self.birch_planks_to_properties_map(),
            BlockKind::JunglePlanks => self.jungle_planks_to_properties_map(),
            BlockKind::AcaciaPlanks => self.acacia_planks_to_properties_map(),
            BlockKind::DarkOakPlanks => self.dark_oak_planks_to_properties_map(),
            BlockKind::OakSapling => self.oak_sapling_to_properties_map(),
            BlockKind::SpruceSapling => self.spruce_sapling_to_properties_map(),
            BlockKind::BirchSapling => self.birch_sapling_to_properties_map(),
            BlockKind::JungleSapling => self.jungle_sapling_to_properties_map(),
            BlockKind::AcaciaSapling => self.acacia_sapling_to_properties_map(),
            BlockKind::DarkOakSapling => self.dark_oak_sapling_to_properties_map(),
            BlockKind::Bedrock => self.bedrock_to_properties_map(),
            BlockKind::Water => self.water_to_properties_map(),
            BlockKind::Lava => self.lava_to_properties_map(),
            BlockKind::Sand => self.sand_to_properties_map(),
            BlockKind::RedSand => self.red_sand_to_properties_map(),
            BlockKind::Gravel => self.gravel_to_properties_map(),
            BlockKind::GoldOre => self.gold_ore_to_properties_map(),
            BlockKind::IronOre => self.iron_ore_to_properties_map(),
            BlockKind::CoalOre => self.coal_ore_to_properties_map(),
            BlockKind::OakLog => self.oak_log_to_properties_map(),
            BlockKind::SpruceLog => self.spruce_log_to_properties_map(),
            BlockKind::BirchLog => self.birch_log_to_properties_map(),
            BlockKind::JungleLog => self.jungle_log_to_properties_map(),
            BlockKind::AcaciaLog => self.acacia_log_to_properties_map(),
            BlockKind::DarkOakLog => self.dark_oak_log_to_properties_map(),
            BlockKind::StrippedSpruceLog => self.stripped_spruce_log_to_properties_map(),
            BlockKind::StrippedBirchLog => self.stripped_birch_log_to_properties_map(),
            BlockKind::StrippedJungleLog => self.stripped_jungle_log_to_properties_map(),
            BlockKind::StrippedAcaciaLog => self.stripped_acacia_log_to_properties_map(),
            BlockKind::StrippedDarkOakLog => self.stripped_dark_oak_log_to_properties_map(),
            BlockKind::StrippedOakLog => self.stripped_oak_log_to_properties_map(),
            BlockKind::OakWood => self.oak_wood_to_properties_map(),
            BlockKind::SpruceWood => self.spruce_wood_to_properties_map(),
            BlockKind::BirchWood => self.birch_wood_to_properties_map(),
            BlockKind::JungleWood => self.jungle_wood_to_properties_map(),
            BlockKind::AcaciaWood => self.acacia_wood_to_properties_map(),
            BlockKind::DarkOakWood => self.dark_oak_wood_to_properties_map(),
            BlockKind::StrippedOakWood => self.stripped_oak_wood_to_properties_map(),
            BlockKind::StrippedSpruceWood => self.stripped_spruce_wood_to_properties_map(),
            BlockKind::StrippedBirchWood => self.stripped_birch_wood_to_properties_map(),
            BlockKind::StrippedJungleWood => self.stripped_jungle_wood_to_properties_map(),
            BlockKind::StrippedAcaciaWood => self.stripped_acacia_wood_to_properties_map(),
            BlockKind::StrippedDarkOakWood => self.stripped_dark_oak_wood_to_properties_map(),
            BlockKind::OakLeaves => self.oak_leaves_to_properties_map(),
            BlockKind::SpruceLeaves => self.spruce_leaves_to_properties_map(),
            BlockKind::BirchLeaves => self.birch_leaves_to_properties_map(),
            BlockKind::JungleLeaves => self.jungle_leaves_to_properties_map(),
            BlockKind::AcaciaLeaves => self.acacia_leaves_to_properties_map(),
            BlockKind::DarkOakLeaves => self.dark_oak_leaves_to_properties_map(),
            BlockKind::Sponge => self.sponge_to_properties_map(),
            BlockKind::WetSponge => self.wet_sponge_to_properties_map(),
            BlockKind::Glass => self.glass_to_properties_map(),
            BlockKind::LapisOre => self.lapis_ore_to_properties_map(),
            BlockKind::LapisBlock => self.lapis_block_to_properties_map(),
            BlockKind::Dispenser => self.dispenser_to_properties_map(),
            BlockKind::Sandstone => self.sandstone_to_properties_map(),
            BlockKind::ChiseledSandstone => self.chiseled_sandstone_to_properties_map(),
            BlockKind::CutSandstone => self.cut_sandstone_to_properties_map(),
            BlockKind::NoteBlock => self.note_block_to_properties_map(),
            BlockKind::WhiteBed => self.white_bed_to_properties_map(),
            BlockKind::OrangeBed => self.orange_bed_to_properties_map(),
            BlockKind::MagentaBed => self.magenta_bed_to_properties_map(),
            BlockKind::LightBlueBed => self.light_blue_bed_to_properties_map(),
            BlockKind::YellowBed => self.yellow_bed_to_properties_map(),
            BlockKind::LimeBed => self.lime_bed_to_properties_map(),
            BlockKind::PinkBed => self.pink_bed_to_properties_map(),
            BlockKind::GrayBed => self.gray_bed_to_properties_map(),
            BlockKind::LightGrayBed => self.light_gray_bed_to_properties_map(),
            BlockKind::CyanBed => self.cyan_bed_to_properties_map(),
            BlockKind::PurpleBed => self.purple_bed_to_properties_map(),
            BlockKind::BlueBed => self.blue_bed_to_properties_map(),
            BlockKind::BrownBed => self.brown_bed_to_properties_map(),
            BlockKind::GreenBed => self.green_bed_to_properties_map(),
            BlockKind::RedBed => self.red_bed_to_properties_map(),
            BlockKind::BlackBed => self.black_bed_to_properties_map(),
            BlockKind::PoweredRail => self.powered_rail_to_properties_map(),
            BlockKind::DetectorRail => self.detector_rail_to_properties_map(),
            BlockKind::StickyPiston => self.sticky_piston_to_properties_map(),
            BlockKind::Cobweb => self.cobweb_to_properties_map(),
            BlockKind::Grass => self.grass_to_properties_map(),
            BlockKind::Fern => self.fern_to_properties_map(),
            BlockKind::DeadBush => self.dead_bush_to_properties_map(),
            BlockKind::Seagrass => self.seagrass_to_properties_map(),
            BlockKind::TallSeagrass => self.tall_seagrass_to_properties_map(),
            BlockKind::Piston => self.piston_to_properties_map(),
            BlockKind::PistonHead => self.piston_head_to_properties_map(),
            BlockKind::WhiteWool => self.white_wool_to_properties_map(),
            BlockKind::OrangeWool => self.orange_wool_to_properties_map(),
            BlockKind::MagentaWool => self.magenta_wool_to_properties_map(),
            BlockKind::LightBlueWool => self.light_blue_wool_to_properties_map(),
            BlockKind::YellowWool => self.yellow_wool_to_properties_map(),
            BlockKind::LimeWool => self.lime_wool_to_properties_map(),
            BlockKind::PinkWool => self.pink_wool_to_properties_map(),
            BlockKind::GrayWool => self.gray_wool_to_properties_map(),
            BlockKind::LightGrayWool => self.light_gray_wool_to_properties_map(),
            BlockKind::CyanWool => self.cyan_wool_to_properties_map(),
            BlockKind::PurpleWool => self.purple_wool_to_properties_map(),
            BlockKind::BlueWool => self.blue_wool_to_properties_map(),
            BlockKind::BrownWool => self.brown_wool_to_properties_map(),
            BlockKind::GreenWool => self.green_wool_to_properties_map(),
            BlockKind::RedWool => self.red_wool_to_properties_map(),
            BlockKind::BlackWool => self.black_wool_to_properties_map(),
            BlockKind::MovingPiston => self.moving_piston_to_properties_map(),
            BlockKind::Dandelion => self.dandelion_to_properties_map(),
            BlockKind::Poppy => self.poppy_to_properties_map(),
            BlockKind::BlueOrchid => self.blue_orchid_to_properties_map(),
            BlockKind::Allium => self.allium_to_properties_map(),
            BlockKind::AzureBluet => self.azure_bluet_to_properties_map(),
            BlockKind::RedTulip => self.red_tulip_to_properties_map(),
            BlockKind::OrangeTulip => self.orange_tulip_to_properties_map(),
            BlockKind::WhiteTulip => self.white_tulip_to_properties_map(),
            BlockKind::PinkTulip => self.pink_tulip_to_properties_map(),
            BlockKind::OxeyeDaisy => self.oxeye_daisy_to_properties_map(),
            BlockKind::BrownMushroom => self.brown_mushroom_to_properties_map(),
            BlockKind::RedMushroom => self.red_mushroom_to_properties_map(),
            BlockKind::GoldBlock => self.gold_block_to_properties_map(),
            BlockKind::IronBlock => self.iron_block_to_properties_map(),
            BlockKind::Bricks => self.bricks_to_properties_map(),
            BlockKind::Tnt => self.tnt_to_properties_map(),
            BlockKind::Bookshelf => self.bookshelf_to_properties_map(),
            BlockKind::MossyCobblestone => self.mossy_cobblestone_to_properties_map(),
            BlockKind::Obsidian => self.obsidian_to_properties_map(),
            BlockKind::Torch => self.torch_to_properties_map(),
            BlockKind::WallTorch => self.wall_torch_to_properties_map(),
            BlockKind::Fire => self.fire_to_properties_map(),
            BlockKind::Spawner => self.spawner_to_properties_map(),
            BlockKind::OakStairs => self.oak_stairs_to_properties_map(),
            BlockKind::Chest => self.chest_to_properties_map(),
            BlockKind::RedstoneWire => self.redstone_wire_to_properties_map(),
            BlockKind::DiamondOre => self.diamond_ore_to_properties_map(),
            BlockKind::DiamondBlock => self.diamond_block_to_properties_map(),
            BlockKind::CraftingTable => self.crafting_table_to_properties_map(),
            BlockKind::Wheat => self.wheat_to_properties_map(),
            BlockKind::Farmland => self.farmland_to_properties_map(),
            BlockKind::Furnace => self.furnace_to_properties_map(),
            BlockKind::Sign => self.sign_to_properties_map(),
            BlockKind::OakDoor => self.oak_door_to_properties_map(),
            BlockKind::Ladder => self.ladder_to_properties_map(),
            BlockKind::Rail => self.rail_to_properties_map(),
            BlockKind::CobblestoneStairs => self.cobblestone_stairs_to_properties_map(),
            BlockKind::WallSign => self.wall_sign_to_properties_map(),
            BlockKind::Lever => self.lever_to_properties_map(),
            BlockKind::StonePressurePlate => self.stone_pressure_plate_to_properties_map(),
            BlockKind::IronDoor => self.iron_door_to_properties_map(),
            BlockKind::OakPressurePlate => self.oak_pressure_plate_to_properties_map(),
            BlockKind::SprucePressurePlate => self.spruce_pressure_plate_to_properties_map(),
            BlockKind::BirchPressurePlate => self.birch_pressure_plate_to_properties_map(),
            BlockKind::JunglePressurePlate => self.jungle_pressure_plate_to_properties_map(),
            BlockKind::AcaciaPressurePlate => self.acacia_pressure_plate_to_properties_map(),
            BlockKind::DarkOakPressurePlate => self.dark_oak_pressure_plate_to_properties_map(),
            BlockKind::RedstoneOre => self.redstone_ore_to_properties_map(),
            BlockKind::RedstoneTorch => self.redstone_torch_to_properties_map(),
            BlockKind::RedstoneWallTorch => self.redstone_wall_torch_to_properties_map(),
            BlockKind::StoneButton => self.stone_button_to_properties_map(),
            BlockKind::Snow => self.snow_to_properties_map(),
            BlockKind::Ice => self.ice_to_properties_map(),
            BlockKind::SnowBlock => self.snow_block_to_properties_map(),
            BlockKind::Cactus => self.cactus_to_properties_map(),
            BlockKind::Clay => self.clay_to_properties_map(),
            BlockKind::SugarCane => self.sugar_cane_to_properties_map(),
            BlockKind::Jukebox => self.jukebox_to_properties_map(),
            BlockKind::OakFence => self.oak_fence_to_properties_map(),
            BlockKind::Pumpkin => self.pumpkin_to_properties_map(),
            BlockKind::Netherrack => self.netherrack_to_properties_map(),
            BlockKind::SoulSand => self.soul_sand_to_properties_map(),
            BlockKind::Glowstone => self.glowstone_to_properties_map(),
            BlockKind::NetherPortal => self.nether_portal_to_properties_map(),
            BlockKind::CarvedPumpkin => self.carved_pumpkin_to_properties_map(),
            BlockKind::JackOLantern => self.jack_o_lantern_to_properties_map(),
            BlockKind::Cake => self.cake_to_properties_map(),
            BlockKind::Repeater => self.repeater_to_properties_map(),
            BlockKind::WhiteStainedGlass => self.white_stained_glass_to_properties_map(),
            BlockKind::OrangeStainedGlass => self.orange_stained_glass_to_properties_map(),
            BlockKind::MagentaStainedGlass => self.magenta_stained_glass_to_properties_map(),
            BlockKind::LightBlueStainedGlass => self.light_blue_stained_glass_to_properties_map(),
            BlockKind::YellowStainedGlass => self.yellow_stained_glass_to_properties_map(),
            BlockKind::LimeStainedGlass => self.lime_stained_glass_to_properties_map(),
            BlockKind::PinkStainedGlass => self.pink_stained_glass_to_properties_map(),
            BlockKind::GrayStainedGlass => self.gray_stained_glass_to_properties_map(),
            BlockKind::LightGrayStainedGlass => self.light_gray_stained_glass_to_properties_map(),
            BlockKind::CyanStainedGlass => self.cyan_stained_glass_to_properties_map(),
            BlockKind::PurpleStainedGlass => self.purple_stained_glass_to_properties_map(),
            BlockKind::BlueStainedGlass => self.blue_stained_glass_to_properties_map(),
            BlockKind::BrownStainedGlass => self.brown_stained_glass_to_properties_map(),
            BlockKind::GreenStainedGlass => self.green_stained_glass_to_properties_map(),
            BlockKind::RedStainedGlass => self.red_stained_glass_to_properties_map(),
            BlockKind::BlackStainedGlass => self.black_stained_glass_to_properties_map(),
            BlockKind::OakTrapdoor => self.oak_trapdoor_to_properties_map(),
            BlockKind::SpruceTrapdoor => self.spruce_trapdoor_to_properties_map(),
            BlockKind::BirchTrapdoor => self.birch_trapdoor_to_properties_map(),
            BlockKind::JungleTrapdoor => self.jungle_trapdoor_to_properties_map(),
            BlockKind::AcaciaTrapdoor => self.acacia_trapdoor_to_properties_map(),
            BlockKind::DarkOakTrapdoor => self.dark_oak_trapdoor_to_properties_map(),
            BlockKind::InfestedStone => self.infested_stone_to_properties_map(),
            BlockKind::InfestedCobblestone => self.infested_cobblestone_to_properties_map(),
            BlockKind::InfestedStoneBricks => self.infested_stone_bricks_to_properties_map(),
            BlockKind::InfestedMossyStoneBricks => {
                self.infested_mossy_stone_bricks_to_properties_map()
            }
            BlockKind::InfestedCrackedStoneBricks => {
                self.infested_cracked_stone_bricks_to_properties_map()
            }
            BlockKind::InfestedChiseledStoneBricks => {
                self.infested_chiseled_stone_bricks_to_properties_map()
            }
            BlockKind::StoneBricks => self.stone_bricks_to_properties_map(),
            BlockKind::MossyStoneBricks => self.mossy_stone_bricks_to_properties_map(),
            BlockKind::CrackedStoneBricks => self.cracked_stone_bricks_to_properties_map(),
            BlockKind::ChiseledStoneBricks => self.chiseled_stone_bricks_to_properties_map(),
            BlockKind::BrownMushroomBlock => self.brown_mushroom_block_to_properties_map(),
            BlockKind::RedMushroomBlock => self.red_mushroom_block_to_properties_map(),
            BlockKind::MushroomStem => self.mushroom_stem_to_properties_map(),
            BlockKind::IronBars => self.iron_bars_to_properties_map(),
            BlockKind::GlassPane => self.glass_pane_to_properties_map(),
            BlockKind::Melon => self.melon_to_properties_map(),
            BlockKind::AttachedPumpkinStem => self.attached_pumpkin_stem_to_properties_map(),
            BlockKind::AttachedMelonStem => self.attached_melon_stem_to_properties_map(),
            BlockKind::PumpkinStem => self.pumpkin_stem_to_properties_map(),
            BlockKind::MelonStem => self.melon_stem_to_properties_map(),
            BlockKind::Vine => self.vine_to_properties_map(),
            BlockKind::OakFenceGate => self.oak_fence_gate_to_properties_map(),
            BlockKind::BrickStairs => self.brick_stairs_to_properties_map(),
            BlockKind::StoneBrickStairs => self.stone_brick_stairs_to_properties_map(),
            BlockKind::Mycelium => self.mycelium_to_properties_map(),
            BlockKind::LilyPad => self.lily_pad_to_properties_map(),
            BlockKind::NetherBricks => self.nether_bricks_to_properties_map(),
            BlockKind::NetherBrickFence => self.nether_brick_fence_to_properties_map(),
            BlockKind::NetherBrickStairs => self.nether_brick_stairs_to_properties_map(),
            BlockKind::NetherWart => self.nether_wart_to_properties_map(),
            BlockKind::EnchantingTable => self.enchanting_table_to_properties_map(),
            BlockKind::BrewingStand => self.brewing_stand_to_properties_map(),
            BlockKind::Cauldron => self.cauldron_to_properties_map(),
            BlockKind::EndPortal => self.end_portal_to_properties_map(),
            BlockKind::EndPortalFrame => self.end_portal_frame_to_properties_map(),
            BlockKind::EndStone => self.end_stone_to_properties_map(),
            BlockKind::DragonEgg => self.dragon_egg_to_properties_map(),
            BlockKind::RedstoneLamp => self.redstone_lamp_to_properties_map(),
            BlockKind::Cocoa => self.cocoa_to_properties_map(),
            BlockKind::SandstoneStairs => self.sandstone_stairs_to_properties_map(),
            BlockKind::EmeraldOre => self.emerald_ore_to_properties_map(),
            BlockKind::EnderChest => self.ender_chest_to_properties_map(),
            BlockKind::TripwireHook => self.tripwire_hook_to_properties_map(),
            BlockKind::Tripwire => self.tripwire_to_properties_map(),
            BlockKind::EmeraldBlock => self.emerald_block_to_properties_map(),
            BlockKind::SpruceStairs => self.spruce_stairs_to_properties_map(),
            BlockKind::BirchStairs => self.birch_stairs_to_properties_map(),
            BlockKind::JungleStairs => self.jungle_stairs_to_properties_map(),
            BlockKind::CommandBlock => self.command_block_to_properties_map(),
            BlockKind::Beacon => self.beacon_to_properties_map(),
            BlockKind::CobblestoneWall => self.cobblestone_wall_to_properties_map(),
            BlockKind::MossyCobblestoneWall => self.mossy_cobblestone_wall_to_properties_map(),
            BlockKind::FlowerPot => self.flower_pot_to_properties_map(),
            BlockKind::PottedOakSapling => self.potted_oak_sapling_to_properties_map(),
            BlockKind::PottedSpruceSapling => self.potted_spruce_sapling_to_properties_map(),
            BlockKind::PottedBirchSapling => self.potted_birch_sapling_to_properties_map(),
            BlockKind::PottedJungleSapling => self.potted_jungle_sapling_to_properties_map(),
            BlockKind::PottedAcaciaSapling => self.potted_acacia_sapling_to_properties_map(),
            BlockKind::PottedDarkOakSapling => self.potted_dark_oak_sapling_to_properties_map(),
            BlockKind::PottedFern => self.potted_fern_to_properties_map(),
            BlockKind::PottedDandelion => self.potted_dandelion_to_properties_map(),
            BlockKind::PottedPoppy => self.potted_poppy_to_properties_map(),
            BlockKind::PottedBlueOrchid => self.potted_blue_orchid_to_properties_map(),
            BlockKind::PottedAllium => self.potted_allium_to_properties_map(),
            BlockKind::PottedAzureBluet => self.potted_azure_bluet_to_properties_map(),
            BlockKind::PottedRedTulip => self.potted_red_tulip_to_properties_map(),
            BlockKind::PottedOrangeTulip => self.potted_orange_tulip_to_properties_map(),
            BlockKind::PottedWhiteTulip => self.potted_white_tulip_to_properties_map(),
            BlockKind::PottedPinkTulip => self.potted_pink_tulip_to_properties_map(),
            BlockKind::PottedOxeyeDaisy => self.potted_oxeye_daisy_to_properties_map(),
            BlockKind::PottedRedMushroom => self.potted_red_mushroom_to_properties_map(),
            BlockKind::PottedBrownMushroom => self.potted_brown_mushroom_to_properties_map(),
            BlockKind::PottedDeadBush => self.potted_dead_bush_to_properties_map(),
            BlockKind::PottedCactus => self.potted_cactus_to_properties_map(),
            BlockKind::Carrots => self.carrots_to_properties_map(),
            BlockKind::Potatoes => self.potatoes_to_properties_map(),
            BlockKind::OakButton => self.oak_button_to_properties_map(),
            BlockKind::SpruceButton => self.spruce_button_to_properties_map(),
            BlockKind::BirchButton => self.birch_button_to_properties_map(),
            BlockKind::JungleButton => self.jungle_button_to_properties_map(),
            BlockKind::AcaciaButton => self.acacia_button_to_properties_map(),
            BlockKind::DarkOakButton => self.dark_oak_button_to_properties_map(),
            BlockKind::SkeletonWallSkull => self.skeleton_wall_skull_to_properties_map(),
            BlockKind::SkeletonSkull => self.skeleton_skull_to_properties_map(),
            BlockKind::WitherSkeletonWallSkull => {
                self.wither_skeleton_wall_skull_to_properties_map()
            }
            BlockKind::WitherSkeletonSkull => self.wither_skeleton_skull_to_properties_map(),
            BlockKind::ZombieWallHead => self.zombie_wall_head_to_properties_map(),
            BlockKind::ZombieHead => self.zombie_head_to_properties_map(),
            BlockKind::PlayerWallHead => self.player_wall_head_to_properties_map(),
            BlockKind::PlayerHead => self.player_head_to_properties_map(),
            BlockKind::CreeperWallHead => self.creeper_wall_head_to_properties_map(),
            BlockKind::CreeperHead => self.creeper_head_to_properties_map(),
            BlockKind::DragonWallHead => self.dragon_wall_head_to_properties_map(),
            BlockKind::DragonHead => self.dragon_head_to_properties_map(),
            BlockKind::Anvil => self.anvil_to_properties_map(),
            BlockKind::ChippedAnvil => self.chipped_anvil_to_properties_map(),
            BlockKind::DamagedAnvil => self.damaged_anvil_to_properties_map(),
            BlockKind::TrappedChest => self.trapped_chest_to_properties_map(),
            BlockKind::LightWeightedPressurePlate => {
                self.light_weighted_pressure_plate_to_properties_map()
            }
            BlockKind::HeavyWeightedPressurePlate => {
                self.heavy_weighted_pressure_plate_to_properties_map()
            }
            BlockKind::Comparator => self.comparator_to_properties_map(),
            BlockKind::DaylightDetector => self.daylight_detector_to_properties_map(),
            BlockKind::RedstoneBlock => self.redstone_block_to_properties_map(),
            BlockKind::NetherQuartzOre => self.nether_quartz_ore_to_properties_map(),
            BlockKind::Hopper => self.hopper_to_properties_map(),
            BlockKind::QuartzBlock => self.quartz_block_to_properties_map(),
            BlockKind::ChiseledQuartzBlock => self.chiseled_quartz_block_to_properties_map(),
            BlockKind::QuartzPillar => self.quartz_pillar_to_properties_map(),
            BlockKind::QuartzStairs => self.quartz_stairs_to_properties_map(),
            BlockKind::ActivatorRail => self.activator_rail_to_properties_map(),
            BlockKind::Dropper => self.dropper_to_properties_map(),
            BlockKind::WhiteTerracotta => self.white_terracotta_to_properties_map(),
            BlockKind::OrangeTerracotta => self.orange_terracotta_to_properties_map(),
            BlockKind::MagentaTerracotta => self.magenta_terracotta_to_properties_map(),
            BlockKind::LightBlueTerracotta => self.light_blue_terracotta_to_properties_map(),
            BlockKind::YellowTerracotta => self.yellow_terracotta_to_properties_map(),
            BlockKind::LimeTerracotta => self.lime_terracotta_to_properties_map(),
            BlockKind::PinkTerracotta => self.pink_terracotta_to_properties_map(),
            BlockKind::GrayTerracotta => self.gray_terracotta_to_properties_map(),
            BlockKind::LightGrayTerracotta => self.light_gray_terracotta_to_properties_map(),
            BlockKind::CyanTerracotta => self.cyan_terracotta_to_properties_map(),
            BlockKind::PurpleTerracotta => self.purple_terracotta_to_properties_map(),
            BlockKind::BlueTerracotta => self.blue_terracotta_to_properties_map(),
            BlockKind::BrownTerracotta => self.brown_terracotta_to_properties_map(),
            BlockKind::GreenTerracotta => self.green_terracotta_to_properties_map(),
            BlockKind::RedTerracotta => self.red_terracotta_to_properties_map(),
            BlockKind::BlackTerracotta => self.black_terracotta_to_properties_map(),
            BlockKind::WhiteStainedGlassPane => self.white_stained_glass_pane_to_properties_map(),
            BlockKind::OrangeStainedGlassPane => self.orange_stained_glass_pane_to_properties_map(),
            BlockKind::MagentaStainedGlassPane => {
                self.magenta_stained_glass_pane_to_properties_map()
            }
            BlockKind::LightBlueStainedGlassPane => {
                self.light_blue_stained_glass_pane_to_properties_map()
            }
            BlockKind::YellowStainedGlassPane => self.yellow_stained_glass_pane_to_properties_map(),
            BlockKind::LimeStainedGlassPane => self.lime_stained_glass_pane_to_properties_map(),
            BlockKind::PinkStainedGlassPane => self.pink_stained_glass_pane_to_properties_map(),
            BlockKind::GrayStainedGlassPane => self.gray_stained_glass_pane_to_properties_map(),
            BlockKind::LightGrayStainedGlassPane => {
                self.light_gray_stained_glass_pane_to_properties_map()
            }
            BlockKind::CyanStainedGlassPane => self.cyan_stained_glass_pane_to_properties_map(),
            BlockKind::PurpleStainedGlassPane => self.purple_stained_glass_pane_to_properties_map(),
            BlockKind::BlueStainedGlassPane => self.blue_stained_glass_pane_to_properties_map(),
            BlockKind::BrownStainedGlassPane => self.brown_stained_glass_pane_to_properties_map(),
            BlockKind::GreenStainedGlassPane => self.green_stained_glass_pane_to_properties_map(),
            BlockKind::RedStainedGlassPane => self.red_stained_glass_pane_to_properties_map(),
            BlockKind::BlackStainedGlassPane => self.black_stained_glass_pane_to_properties_map(),
            BlockKind::AcaciaStairs => self.acacia_stairs_to_properties_map(),
            BlockKind::DarkOakStairs => self.dark_oak_stairs_to_properties_map(),
            BlockKind::SlimeBlock => self.slime_block_to_properties_map(),
            BlockKind::Barrier => self.barrier_to_properties_map(),
            BlockKind::IronTrapdoor => self.iron_trapdoor_to_properties_map(),
            BlockKind::Prismarine => self.prismarine_to_properties_map(),
            BlockKind::PrismarineBricks => self.prismarine_bricks_to_properties_map(),
            BlockKind::DarkPrismarine => self.dark_prismarine_to_properties_map(),
            BlockKind::PrismarineStairs => self.prismarine_stairs_to_properties_map(),
            BlockKind::PrismarineBrickStairs => self.prismarine_brick_stairs_to_properties_map(),
            BlockKind::DarkPrismarineStairs => self.dark_prismarine_stairs_to_properties_map(),
            BlockKind::PrismarineSlab => self.prismarine_slab_to_properties_map(),
            BlockKind::PrismarineBrickSlab => self.prismarine_brick_slab_to_properties_map(),
            BlockKind::DarkPrismarineSlab => self.dark_prismarine_slab_to_properties_map(),
            BlockKind::SeaLantern => self.sea_lantern_to_properties_map(),
            BlockKind::HayBlock => self.hay_block_to_properties_map(),
            BlockKind::WhiteCarpet => self.white_carpet_to_properties_map(),
            BlockKind::OrangeCarpet => self.orange_carpet_to_properties_map(),
            BlockKind::MagentaCarpet => self.magenta_carpet_to_properties_map(),
            BlockKind::LightBlueCarpet => self.light_blue_carpet_to_properties_map(),
            BlockKind::YellowCarpet => self.yellow_carpet_to_properties_map(),
            BlockKind::LimeCarpet => self.lime_carpet_to_properties_map(),
            BlockKind::PinkCarpet => self.pink_carpet_to_properties_map(),
            BlockKind::GrayCarpet => self.gray_carpet_to_properties_map(),
            BlockKind::LightGrayCarpet => self.light_gray_carpet_to_properties_map(),
            BlockKind::CyanCarpet => self.cyan_carpet_to_properties_map(),
            BlockKind::PurpleCarpet => self.purple_carpet_to_properties_map(),
            BlockKind::BlueCarpet => self.blue_carpet_to_properties_map(),
            BlockKind::BrownCarpet => self.brown_carpet_to_properties_map(),
            BlockKind::GreenCarpet => self.green_carpet_to_properties_map(),
            BlockKind::RedCarpet => self.red_carpet_to_properties_map(),
            BlockKind::BlackCarpet => self.black_carpet_to_properties_map(),
            BlockKind::Terracotta => self.terracotta_to_properties_map(),
            BlockKind::CoalBlock => self.coal_block_to_properties_map(),
            BlockKind::PackedIce => self.packed_ice_to_properties_map(),
            BlockKind::Sunflower => self.sunflower_to_properties_map(),
            BlockKind::Lilac => self.lilac_to_properties_map(),
            BlockKind::RoseBush => self.rose_bush_to_properties_map(),
            BlockKind::Peony => self.peony_to_properties_map(),
            BlockKind::TallGrass => self.tall_grass_to_properties_map(),
            BlockKind::LargeFern => self.large_fern_to_properties_map(),
            BlockKind::WhiteBanner => self.white_banner_to_properties_map(),
            BlockKind::OrangeBanner => self.orange_banner_to_properties_map(),
            BlockKind::MagentaBanner => self.magenta_banner_to_properties_map(),
            BlockKind::LightBlueBanner => self.light_blue_banner_to_properties_map(),
            BlockKind::YellowBanner => self.yellow_banner_to_properties_map(),
            BlockKind::LimeBanner => self.lime_banner_to_properties_map(),
            BlockKind::PinkBanner => self.pink_banner_to_properties_map(),
            BlockKind::GrayBanner => self.gray_banner_to_properties_map(),
            BlockKind::LightGrayBanner => self.light_gray_banner_to_properties_map(),
            BlockKind::CyanBanner => self.cyan_banner_to_properties_map(),
            BlockKind::PurpleBanner => self.purple_banner_to_properties_map(),
            BlockKind::BlueBanner => self.blue_banner_to_properties_map(),
            BlockKind::BrownBanner => self.brown_banner_to_properties_map(),
            BlockKind::GreenBanner => self.green_banner_to_properties_map(),
            BlockKind::RedBanner => self.red_banner_to_properties_map(),
            BlockKind::BlackBanner => self.black_banner_to_properties_map(),
            BlockKind::WhiteWallBanner => self.white_wall_banner_to_properties_map(),
            BlockKind::OrangeWallBanner => self.orange_wall_banner_to_properties_map(),
            BlockKind::MagentaWallBanner => self.magenta_wall_banner_to_properties_map(),
            BlockKind::LightBlueWallBanner => self.light_blue_wall_banner_to_properties_map(),
            BlockKind::YellowWallBanner => self.yellow_wall_banner_to_properties_map(),
            BlockKind::LimeWallBanner => self.lime_wall_banner_to_properties_map(),
            BlockKind::PinkWallBanner => self.pink_wall_banner_to_properties_map(),
            BlockKind::GrayWallBanner => self.gray_wall_banner_to_properties_map(),
            BlockKind::LightGrayWallBanner => self.light_gray_wall_banner_to_properties_map(),
            BlockKind::CyanWallBanner => self.cyan_wall_banner_to_properties_map(),
            BlockKind::PurpleWallBanner => self.purple_wall_banner_to_properties_map(),
            BlockKind::BlueWallBanner => self.blue_wall_banner_to_properties_map(),
            BlockKind::BrownWallBanner => self.brown_wall_banner_to_properties_map(),
            BlockKind::GreenWallBanner => self.green_wall_banner_to_properties_map(),
            BlockKind::RedWallBanner => self.red_wall_banner_to_properties_map(),
            BlockKind::BlackWallBanner => self.black_wall_banner_to_properties_map(),
            BlockKind::RedSandstone => self.red_sandstone_to_properties_map(),
            BlockKind::ChiseledRedSandstone => self.chiseled_red_sandstone_to_properties_map(),
            BlockKind::CutRedSandstone => self.cut_red_sandstone_to_properties_map(),
            BlockKind::RedSandstoneStairs => self.red_sandstone_stairs_to_properties_map(),
            BlockKind::OakSlab => self.oak_slab_to_properties_map(),
            BlockKind::SpruceSlab => self.spruce_slab_to_properties_map(),
            BlockKind::BirchSlab => self.birch_slab_to_properties_map(),
            BlockKind::JungleSlab => self.jungle_slab_to_properties_map(),
            BlockKind::AcaciaSlab => self.acacia_slab_to_properties_map(),
            BlockKind::DarkOakSlab => self.dark_oak_slab_to_properties_map(),
            BlockKind::StoneSlab => self.stone_slab_to_properties_map(),
            BlockKind::SandstoneSlab => self.sandstone_slab_to_properties_map(),
            BlockKind::PetrifiedOakSlab => self.petrified_oak_slab_to_properties_map(),
            BlockKind::CobblestoneSlab => self.cobblestone_slab_to_properties_map(),
            BlockKind::BrickSlab => self.brick_slab_to_properties_map(),
            BlockKind::StoneBrickSlab => self.stone_brick_slab_to_properties_map(),
            BlockKind::NetherBrickSlab => self.nether_brick_slab_to_properties_map(),
            BlockKind::QuartzSlab => self.quartz_slab_to_properties_map(),
            BlockKind::RedSandstoneSlab => self.red_sandstone_slab_to_properties_map(),
            BlockKind::PurpurSlab => self.purpur_slab_to_properties_map(),
            BlockKind::SmoothStone => self.smooth_stone_to_properties_map(),
            BlockKind::SmoothSandstone => self.smooth_sandstone_to_properties_map(),
            BlockKind::SmoothQuartz => self.smooth_quartz_to_properties_map(),
            BlockKind::SmoothRedSandstone => self.smooth_red_sandstone_to_properties_map(),
            BlockKind::SpruceFenceGate => self.spruce_fence_gate_to_properties_map(),
            BlockKind::BirchFenceGate => self.birch_fence_gate_to_properties_map(),
            BlockKind::JungleFenceGate => self.jungle_fence_gate_to_properties_map(),
            BlockKind::AcaciaFenceGate => self.acacia_fence_gate_to_properties_map(),
            BlockKind::DarkOakFenceGate => self.dark_oak_fence_gate_to_properties_map(),
            BlockKind::SpruceFence => self.spruce_fence_to_properties_map(),
            BlockKind::BirchFence => self.birch_fence_to_properties_map(),
            BlockKind::JungleFence => self.jungle_fence_to_properties_map(),
            BlockKind::AcaciaFence => self.acacia_fence_to_properties_map(),
            BlockKind::DarkOakFence => self.dark_oak_fence_to_properties_map(),
            BlockKind::SpruceDoor => self.spruce_door_to_properties_map(),
            BlockKind::BirchDoor => self.birch_door_to_properties_map(),
            BlockKind::JungleDoor => self.jungle_door_to_properties_map(),
            BlockKind::AcaciaDoor => self.acacia_door_to_properties_map(),
            BlockKind::DarkOakDoor => self.dark_oak_door_to_properties_map(),
            BlockKind::EndRod => self.end_rod_to_properties_map(),
            BlockKind::ChorusPlant => self.chorus_plant_to_properties_map(),
            BlockKind::ChorusFlower => self.chorus_flower_to_properties_map(),
            BlockKind::PurpurBlock => self.purpur_block_to_properties_map(),
            BlockKind::PurpurPillar => self.purpur_pillar_to_properties_map(),
            BlockKind::PurpurStairs => self.purpur_stairs_to_properties_map(),
            BlockKind::EndStoneBricks => self.end_stone_bricks_to_properties_map(),
            BlockKind::Beetroots => self.beetroots_to_properties_map(),
            BlockKind::GrassPath => self.grass_path_to_properties_map(),
            BlockKind::EndGateway => self.end_gateway_to_properties_map(),
            BlockKind::RepeatingCommandBlock => self.repeating_command_block_to_properties_map(),
            BlockKind::ChainCommandBlock => self.chain_command_block_to_properties_map(),
            BlockKind::FrostedIce => self.frosted_ice_to_properties_map(),
            BlockKind::MagmaBlock => self.magma_block_to_properties_map(),
            BlockKind::NetherWartBlock => self.nether_wart_block_to_properties_map(),
            BlockKind::RedNetherBricks => self.red_nether_bricks_to_properties_map(),
            BlockKind::BoneBlock => self.bone_block_to_properties_map(),
            BlockKind::StructureVoid => self.structure_void_to_properties_map(),
            BlockKind::Observer => self.observer_to_properties_map(),
            BlockKind::ShulkerBox => self.shulker_box_to_properties_map(),
            BlockKind::WhiteShulkerBox => self.white_shulker_box_to_properties_map(),
            BlockKind::OrangeShulkerBox => self.orange_shulker_box_to_properties_map(),
            BlockKind::MagentaShulkerBox => self.magenta_shulker_box_to_properties_map(),
            BlockKind::LightBlueShulkerBox => self.light_blue_shulker_box_to_properties_map(),
            BlockKind::YellowShulkerBox => self.yellow_shulker_box_to_properties_map(),
            BlockKind::LimeShulkerBox => self.lime_shulker_box_to_properties_map(),
            BlockKind::PinkShulkerBox => self.pink_shulker_box_to_properties_map(),
            BlockKind::GrayShulkerBox => self.gray_shulker_box_to_properties_map(),
            BlockKind::LightGrayShulkerBox => self.light_gray_shulker_box_to_properties_map(),
            BlockKind::CyanShulkerBox => self.cyan_shulker_box_to_properties_map(),
            BlockKind::PurpleShulkerBox => self.purple_shulker_box_to_properties_map(),
            BlockKind::BlueShulkerBox => self.blue_shulker_box_to_properties_map(),
            BlockKind::BrownShulkerBox => self.brown_shulker_box_to_properties_map(),
            BlockKind::GreenShulkerBox => self.green_shulker_box_to_properties_map(),
            BlockKind::RedShulkerBox => self.red_shulker_box_to_properties_map(),
            BlockKind::BlackShulkerBox => self.black_shulker_box_to_properties_map(),
            BlockKind::WhiteGlazedTerracotta => self.white_glazed_terracotta_to_properties_map(),
            BlockKind::OrangeGlazedTerracotta => self.orange_glazed_terracotta_to_properties_map(),
            BlockKind::MagentaGlazedTerracotta => {
                self.magenta_glazed_terracotta_to_properties_map()
            }
            BlockKind::LightBlueGlazedTerracotta => {
                self.light_blue_glazed_terracotta_to_properties_map()
            }
            BlockKind::YellowGlazedTerracotta => self.yellow_glazed_terracotta_to_properties_map(),
            BlockKind::LimeGlazedTerracotta => self.lime_glazed_terracotta_to_properties_map(),
            BlockKind::PinkGlazedTerracotta => self.pink_glazed_terracotta_to_properties_map(),
            BlockKind::GrayGlazedTerracotta => self.gray_glazed_terracotta_to_properties_map(),
            BlockKind::LightGrayGlazedTerracotta => {
                self.light_gray_glazed_terracotta_to_properties_map()
            }
            BlockKind::CyanGlazedTerracotta => self.cyan_glazed_terracotta_to_properties_map(),
            BlockKind::PurpleGlazedTerracotta => self.purple_glazed_terracotta_to_properties_map(),
            BlockKind::BlueGlazedTerracotta => self.blue_glazed_terracotta_to_properties_map(),
            BlockKind::BrownGlazedTerracotta => self.brown_glazed_terracotta_to_properties_map(),
            BlockKind::GreenGlazedTerracotta => self.green_glazed_terracotta_to_properties_map(),
            BlockKind::RedGlazedTerracotta => self.red_glazed_terracotta_to_properties_map(),
            BlockKind::BlackGlazedTerracotta => self.black_glazed_terracotta_to_properties_map(),
            BlockKind::WhiteConcrete => self.white_concrete_to_properties_map(),
            BlockKind::OrangeConcrete => self.orange_concrete_to_properties_map(),
            BlockKind::MagentaConcrete => self.magenta_concrete_to_properties_map(),
            BlockKind::LightBlueConcrete => self.light_blue_concrete_to_properties_map(),
            BlockKind::YellowConcrete => self.yellow_concrete_to_properties_map(),
            BlockKind::LimeConcrete => self.lime_concrete_to_properties_map(),
            BlockKind::PinkConcrete => self.pink_concrete_to_properties_map(),
            BlockKind::GrayConcrete => self.gray_concrete_to_properties_map(),
            BlockKind::LightGrayConcrete => self.light_gray_concrete_to_properties_map(),
            BlockKind::CyanConcrete => self.cyan_concrete_to_properties_map(),
            BlockKind::PurpleConcrete => self.purple_concrete_to_properties_map(),
            BlockKind::BlueConcrete => self.blue_concrete_to_properties_map(),
            BlockKind::BrownConcrete => self.brown_concrete_to_properties_map(),
            BlockKind::GreenConcrete => self.green_concrete_to_properties_map(),
            BlockKind::RedConcrete => self.red_concrete_to_properties_map(),
            BlockKind::BlackConcrete => self.black_concrete_to_properties_map(),
            BlockKind::WhiteConcretePowder => self.white_concrete_powder_to_properties_map(),
            BlockKind::OrangeConcretePowder => self.orange_concrete_powder_to_properties_map(),
            BlockKind::MagentaConcretePowder => self.magenta_concrete_powder_to_properties_map(),
            BlockKind::LightBlueConcretePowder => {
                self.light_blue_concrete_powder_to_properties_map()
            }
            BlockKind::YellowConcretePowder => self.yellow_concrete_powder_to_properties_map(),
            BlockKind::LimeConcretePowder => self.lime_concrete_powder_to_properties_map(),
            BlockKind::PinkConcretePowder => self.pink_concrete_powder_to_properties_map(),
            BlockKind::GrayConcretePowder => self.gray_concrete_powder_to_properties_map(),
            BlockKind::LightGrayConcretePowder => {
                self.light_gray_concrete_powder_to_properties_map()
            }
            BlockKind::CyanConcretePowder => self.cyan_concrete_powder_to_properties_map(),
            BlockKind::PurpleConcretePowder => self.purple_concrete_powder_to_properties_map(),
            BlockKind::BlueConcretePowder => self.blue_concrete_powder_to_properties_map(),
            BlockKind::BrownConcretePowder => self.brown_concrete_powder_to_properties_map(),
            BlockKind::GreenConcretePowder => self.green_concrete_powder_to_properties_map(),
            BlockKind::RedConcretePowder => self.red_concrete_powder_to_properties_map(),
            BlockKind::BlackConcretePowder => self.black_concrete_powder_to_properties_map(),
            BlockKind::Kelp => self.kelp_to_properties_map(),
            BlockKind::KelpPlant => self.kelp_plant_to_properties_map(),
            BlockKind::DriedKelpBlock => self.dried_kelp_block_to_properties_map(),
            BlockKind::TurtleEgg => self.turtle_egg_to_properties_map(),
            BlockKind::DeadTubeCoralBlock => self.dead_tube_coral_block_to_properties_map(),
            BlockKind::DeadBrainCoralBlock => self.dead_brain_coral_block_to_properties_map(),
            BlockKind::DeadBubbleCoralBlock => self.dead_bubble_coral_block_to_properties_map(),
            BlockKind::DeadFireCoralBlock => self.dead_fire_coral_block_to_properties_map(),
            BlockKind::DeadHornCoralBlock => self.dead_horn_coral_block_to_properties_map(),
            BlockKind::TubeCoralBlock => self.tube_coral_block_to_properties_map(),
            BlockKind::BrainCoralBlock => self.brain_coral_block_to_properties_map(),
            BlockKind::BubbleCoralBlock => self.bubble_coral_block_to_properties_map(),
            BlockKind::FireCoralBlock => self.fire_coral_block_to_properties_map(),
            BlockKind::HornCoralBlock => self.horn_coral_block_to_properties_map(),
            BlockKind::DeadTubeCoral => self.dead_tube_coral_to_properties_map(),
            BlockKind::DeadBrainCoral => self.dead_brain_coral_to_properties_map(),
            BlockKind::DeadBubbleCoral => self.dead_bubble_coral_to_properties_map(),
            BlockKind::DeadFireCoral => self.dead_fire_coral_to_properties_map(),
            BlockKind::DeadHornCoral => self.dead_horn_coral_to_properties_map(),
            BlockKind::TubeCoral => self.tube_coral_to_properties_map(),
            BlockKind::BrainCoral => self.brain_coral_to_properties_map(),
            BlockKind::BubbleCoral => self.bubble_coral_to_properties_map(),
            BlockKind::FireCoral => self.fire_coral_to_properties_map(),
            BlockKind::HornCoral => self.horn_coral_to_properties_map(),
            BlockKind::DeadTubeCoralWallFan => self.dead_tube_coral_wall_fan_to_properties_map(),
            BlockKind::DeadBrainCoralWallFan => self.dead_brain_coral_wall_fan_to_properties_map(),
            BlockKind::DeadBubbleCoralWallFan => {
                self.dead_bubble_coral_wall_fan_to_properties_map()
            }
            BlockKind::DeadFireCoralWallFan => self.dead_fire_coral_wall_fan_to_properties_map(),
            BlockKind::DeadHornCoralWallFan => self.dead_horn_coral_wall_fan_to_properties_map(),
            BlockKind::TubeCoralWallFan => self.tube_coral_wall_fan_to_properties_map(),
            BlockKind::BrainCoralWallFan => self.brain_coral_wall_fan_to_properties_map(),
            BlockKind::BubbleCoralWallFan => self.bubble_coral_wall_fan_to_properties_map(),
            BlockKind::FireCoralWallFan => self.fire_coral_wall_fan_to_properties_map(),
            BlockKind::HornCoralWallFan => self.horn_coral_wall_fan_to_properties_map(),
            BlockKind::DeadTubeCoralFan => self.dead_tube_coral_fan_to_properties_map(),
            BlockKind::DeadBrainCoralFan => self.dead_brain_coral_fan_to_properties_map(),
            BlockKind::DeadBubbleCoralFan => self.dead_bubble_coral_fan_to_properties_map(),
            BlockKind::DeadFireCoralFan => self.dead_fire_coral_fan_to_properties_map(),
            BlockKind::DeadHornCoralFan => self.dead_horn_coral_fan_to_properties_map(),
            BlockKind::TubeCoralFan => self.tube_coral_fan_to_properties_map(),
            BlockKind::BrainCoralFan => self.brain_coral_fan_to_properties_map(),
            BlockKind::BubbleCoralFan => self.bubble_coral_fan_to_properties_map(),
            BlockKind::FireCoralFan => self.fire_coral_fan_to_properties_map(),
            BlockKind::HornCoralFan => self.horn_coral_fan_to_properties_map(),
            BlockKind::SeaPickle => self.sea_pickle_to_properties_map(),
            BlockKind::BlueIce => self.blue_ice_to_properties_map(),
            BlockKind::Conduit => self.conduit_to_properties_map(),
            BlockKind::VoidAir => self.void_air_to_properties_map(),
            BlockKind::CaveAir => self.cave_air_to_properties_map(),
            BlockKind::BubbleColumn => self.bubble_column_to_properties_map(),
            BlockKind::StructureBlock => self.structure_block_to_properties_map(),
        }
    }
    fn air_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn stone_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn granite_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn polished_granite_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn diorite_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn polished_diorite_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn andesite_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn polished_andesite_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn grass_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let snowy = self.snowy().unwrap();
        map.insert("snowy", {
            match snowy {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dirt_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn coarse_dirt_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn podzol_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let snowy = self.snowy().unwrap();
        map.insert("snowy", {
            match snowy {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn cobblestone_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn oak_planks_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn spruce_planks_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn birch_planks_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn jungle_planks_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn acacia_planks_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn dark_oak_planks_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn oak_sapling_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let stage = self.stage().unwrap();
        map.insert("stage", {
            match stage {
                0i32 => "0",
                1i32 => "1",
                _ => "unknown",
            }
        });
        map
    }
    fn spruce_sapling_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let stage = self.stage().unwrap();
        map.insert("stage", {
            match stage {
                0i32 => "0",
                1i32 => "1",
                _ => "unknown",
            }
        });
        map
    }
    fn birch_sapling_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let stage = self.stage().unwrap();
        map.insert("stage", {
            match stage {
                0i32 => "0",
                1i32 => "1",
                _ => "unknown",
            }
        });
        map
    }
    fn jungle_sapling_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let stage = self.stage().unwrap();
        map.insert("stage", {
            match stage {
                0i32 => "0",
                1i32 => "1",
                _ => "unknown",
            }
        });
        map
    }
    fn acacia_sapling_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let stage = self.stage().unwrap();
        map.insert("stage", {
            match stage {
                0i32 => "0",
                1i32 => "1",
                _ => "unknown",
            }
        });
        map
    }
    fn dark_oak_sapling_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let stage = self.stage().unwrap();
        map.insert("stage", {
            match stage {
                0i32 => "0",
                1i32 => "1",
                _ => "unknown",
            }
        });
        map
    }
    fn bedrock_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn water_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let water_level = self.water_level().unwrap();
        map.insert("level", {
            match water_level {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn lava_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let water_level = self.water_level().unwrap();
        map.insert("level", {
            match water_level {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn sand_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn red_sand_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn gravel_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn gold_ore_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn iron_ore_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn coal_ore_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn oak_log_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn spruce_log_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn birch_log_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn jungle_log_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn acacia_log_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn dark_oak_log_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn stripped_spruce_log_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn stripped_birch_log_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn stripped_jungle_log_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn stripped_acacia_log_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn stripped_dark_oak_log_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn stripped_oak_log_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn oak_wood_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn spruce_wood_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn birch_wood_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn jungle_wood_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn acacia_wood_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn dark_oak_wood_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn stripped_oak_wood_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn stripped_spruce_wood_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn stripped_birch_wood_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn stripped_jungle_wood_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn stripped_acacia_wood_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn stripped_dark_oak_wood_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn oak_leaves_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let distance = self.distance().unwrap();
        map.insert("distance", {
            match distance {
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                _ => "unknown",
            }
        });
        let persistent = self.persistent().unwrap();
        map.insert("persistent", {
            match persistent {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn spruce_leaves_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let distance = self.distance().unwrap();
        map.insert("distance", {
            match distance {
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                _ => "unknown",
            }
        });
        let persistent = self.persistent().unwrap();
        map.insert("persistent", {
            match persistent {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn birch_leaves_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let distance = self.distance().unwrap();
        map.insert("distance", {
            match distance {
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                _ => "unknown",
            }
        });
        let persistent = self.persistent().unwrap();
        map.insert("persistent", {
            match persistent {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn jungle_leaves_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let distance = self.distance().unwrap();
        map.insert("distance", {
            match distance {
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                _ => "unknown",
            }
        });
        let persistent = self.persistent().unwrap();
        map.insert("persistent", {
            match persistent {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn acacia_leaves_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let distance = self.distance().unwrap();
        map.insert("distance", {
            match distance {
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                _ => "unknown",
            }
        });
        let persistent = self.persistent().unwrap();
        map.insert("persistent", {
            match persistent {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dark_oak_leaves_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let distance = self.distance().unwrap();
        map.insert("distance", {
            match distance {
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                _ => "unknown",
            }
        });
        let persistent = self.persistent().unwrap();
        map.insert("persistent", {
            match persistent {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn sponge_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn wet_sponge_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn glass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn lapis_ore_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn lapis_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn dispenser_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        let triggered = self.triggered().unwrap();
        map.insert("triggered", {
            match triggered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn sandstone_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn chiseled_sandstone_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn cut_sandstone_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn note_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let instrument = self.instrument().unwrap();
        map.insert("instrument", { instrument.as_str() });
        let note = self.note().unwrap();
        map.insert("note", {
            match note {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                16i32 => "16",
                17i32 => "17",
                18i32 => "18",
                19i32 => "19",
                20i32 => "20",
                21i32 => "21",
                22i32 => "22",
                23i32 => "23",
                24i32 => "24",
                _ => "unknown",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn white_bed_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let occupied = self.occupied().unwrap();
        map.insert("occupied", {
            match occupied {
                true => "true",
                false => "false",
            }
        });
        let part = self.part().unwrap();
        map.insert("part", { part.as_str() });
        map
    }
    fn orange_bed_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let occupied = self.occupied().unwrap();
        map.insert("occupied", {
            match occupied {
                true => "true",
                false => "false",
            }
        });
        let part = self.part().unwrap();
        map.insert("part", { part.as_str() });
        map
    }
    fn magenta_bed_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let occupied = self.occupied().unwrap();
        map.insert("occupied", {
            match occupied {
                true => "true",
                false => "false",
            }
        });
        let part = self.part().unwrap();
        map.insert("part", { part.as_str() });
        map
    }
    fn light_blue_bed_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let occupied = self.occupied().unwrap();
        map.insert("occupied", {
            match occupied {
                true => "true",
                false => "false",
            }
        });
        let part = self.part().unwrap();
        map.insert("part", { part.as_str() });
        map
    }
    fn yellow_bed_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let occupied = self.occupied().unwrap();
        map.insert("occupied", {
            match occupied {
                true => "true",
                false => "false",
            }
        });
        let part = self.part().unwrap();
        map.insert("part", { part.as_str() });
        map
    }
    fn lime_bed_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let occupied = self.occupied().unwrap();
        map.insert("occupied", {
            match occupied {
                true => "true",
                false => "false",
            }
        });
        let part = self.part().unwrap();
        map.insert("part", { part.as_str() });
        map
    }
    fn pink_bed_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let occupied = self.occupied().unwrap();
        map.insert("occupied", {
            match occupied {
                true => "true",
                false => "false",
            }
        });
        let part = self.part().unwrap();
        map.insert("part", { part.as_str() });
        map
    }
    fn gray_bed_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let occupied = self.occupied().unwrap();
        map.insert("occupied", {
            match occupied {
                true => "true",
                false => "false",
            }
        });
        let part = self.part().unwrap();
        map.insert("part", { part.as_str() });
        map
    }
    fn light_gray_bed_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let occupied = self.occupied().unwrap();
        map.insert("occupied", {
            match occupied {
                true => "true",
                false => "false",
            }
        });
        let part = self.part().unwrap();
        map.insert("part", { part.as_str() });
        map
    }
    fn cyan_bed_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let occupied = self.occupied().unwrap();
        map.insert("occupied", {
            match occupied {
                true => "true",
                false => "false",
            }
        });
        let part = self.part().unwrap();
        map.insert("part", { part.as_str() });
        map
    }
    fn purple_bed_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let occupied = self.occupied().unwrap();
        map.insert("occupied", {
            match occupied {
                true => "true",
                false => "false",
            }
        });
        let part = self.part().unwrap();
        map.insert("part", { part.as_str() });
        map
    }
    fn blue_bed_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let occupied = self.occupied().unwrap();
        map.insert("occupied", {
            match occupied {
                true => "true",
                false => "false",
            }
        });
        let part = self.part().unwrap();
        map.insert("part", { part.as_str() });
        map
    }
    fn brown_bed_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let occupied = self.occupied().unwrap();
        map.insert("occupied", {
            match occupied {
                true => "true",
                false => "false",
            }
        });
        let part = self.part().unwrap();
        map.insert("part", { part.as_str() });
        map
    }
    fn green_bed_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let occupied = self.occupied().unwrap();
        map.insert("occupied", {
            match occupied {
                true => "true",
                false => "false",
            }
        });
        let part = self.part().unwrap();
        map.insert("part", { part.as_str() });
        map
    }
    fn red_bed_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let occupied = self.occupied().unwrap();
        map.insert("occupied", {
            match occupied {
                true => "true",
                false => "false",
            }
        });
        let part = self.part().unwrap();
        map.insert("part", { part.as_str() });
        map
    }
    fn black_bed_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let occupied = self.occupied().unwrap();
        map.insert("occupied", {
            match occupied {
                true => "true",
                false => "false",
            }
        });
        let part = self.part().unwrap();
        map.insert("part", { part.as_str() });
        map
    }
    fn powered_rail_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        let powered_rail_shape = self.powered_rail_shape().unwrap();
        map.insert("shape", { powered_rail_shape.as_str() });
        map
    }
    fn detector_rail_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        let powered_rail_shape = self.powered_rail_shape().unwrap();
        map.insert("shape", { powered_rail_shape.as_str() });
        map
    }
    fn sticky_piston_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let extended = self.extended().unwrap();
        map.insert("extended", {
            match extended {
                true => "true",
                false => "false",
            }
        });
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn cobweb_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn grass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn fern_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn dead_bush_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn seagrass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn tall_seagrass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let half_upper_lower = self.half_upper_lower().unwrap();
        map.insert("half", { half_upper_lower.as_str() });
        map
    }
    fn piston_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let extended = self.extended().unwrap();
        map.insert("extended", {
            match extended {
                true => "true",
                false => "false",
            }
        });
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn piston_head_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        let piston_kind = self.piston_kind().unwrap();
        map.insert("type", { piston_kind.as_str() });
        let short = self.short().unwrap();
        map.insert("short", {
            match short {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn white_wool_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn orange_wool_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn magenta_wool_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn light_blue_wool_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn yellow_wool_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn lime_wool_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn pink_wool_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn gray_wool_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn light_gray_wool_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn cyan_wool_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn purple_wool_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn blue_wool_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn brown_wool_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn green_wool_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn red_wool_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn black_wool_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn moving_piston_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        let piston_kind = self.piston_kind().unwrap();
        map.insert("type", { piston_kind.as_str() });
        map
    }
    fn dandelion_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn poppy_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn blue_orchid_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn allium_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn azure_bluet_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn red_tulip_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn orange_tulip_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn white_tulip_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn pink_tulip_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn oxeye_daisy_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn brown_mushroom_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn red_mushroom_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn gold_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn iron_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn bricks_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn tnt_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let unstable = self.unstable().unwrap();
        map.insert("unstable", {
            match unstable {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn bookshelf_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn mossy_cobblestone_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn obsidian_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn torch_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn wall_torch_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn fire_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let age_0_15 = self.age_0_15().unwrap();
        map.insert("age", {
            match age_0_15 {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let up = self.up().unwrap();
        map.insert("up", {
            match up {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn spawner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn oak_stairs_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let stairs_shape = self.stairs_shape().unwrap();
        map.insert("shape", { stairs_shape.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn chest_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let chest_kind = self.chest_kind().unwrap();
        map.insert("type", { chest_kind.as_str() });
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn redstone_wire_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_wire = self.east_wire().unwrap();
        map.insert("east", { east_wire.as_str() });
        let north_wire = self.north_wire().unwrap();
        map.insert("north", { north_wire.as_str() });
        let power = self.power().unwrap();
        map.insert("power", {
            match power {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        let south_wire = self.south_wire().unwrap();
        map.insert("south", { south_wire.as_str() });
        let west_wire = self.west_wire().unwrap();
        map.insert("west", { west_wire.as_str() });
        map
    }
    fn diamond_ore_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn diamond_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn crafting_table_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn wheat_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let age_0_7 = self.age_0_7().unwrap();
        map.insert("age", {
            match age_0_7 {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                _ => "unknown",
            }
        });
        map
    }
    fn farmland_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let moisture = self.moisture().unwrap();
        map.insert("moisture", {
            match moisture {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                _ => "unknown",
            }
        });
        map
    }
    fn furnace_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let lit = self.lit().unwrap();
        map.insert("lit", {
            match lit {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn sign_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn oak_door_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_upper_lower = self.half_upper_lower().unwrap();
        map.insert("half", { half_upper_lower.as_str() });
        let hinge = self.hinge().unwrap();
        map.insert("hinge", { hinge.as_str() });
        let open = self.open().unwrap();
        map.insert("open", {
            match open {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn ladder_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn rail_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rail_shape = self.rail_shape().unwrap();
        map.insert("shape", { rail_shape.as_str() });
        map
    }
    fn cobblestone_stairs_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let stairs_shape = self.stairs_shape().unwrap();
        map.insert("shape", { stairs_shape.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn wall_sign_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn lever_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let face = self.face().unwrap();
        map.insert("face", { face.as_str() });
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn stone_pressure_plate_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn iron_door_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_upper_lower = self.half_upper_lower().unwrap();
        map.insert("half", { half_upper_lower.as_str() });
        let hinge = self.hinge().unwrap();
        map.insert("hinge", { hinge.as_str() });
        let open = self.open().unwrap();
        map.insert("open", {
            match open {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn oak_pressure_plate_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn spruce_pressure_plate_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn birch_pressure_plate_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn jungle_pressure_plate_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn acacia_pressure_plate_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dark_oak_pressure_plate_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn redstone_ore_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let lit = self.lit().unwrap();
        map.insert("lit", {
            match lit {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn redstone_torch_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let lit = self.lit().unwrap();
        map.insert("lit", {
            match lit {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn redstone_wall_torch_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let lit = self.lit().unwrap();
        map.insert("lit", {
            match lit {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn stone_button_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let face = self.face().unwrap();
        map.insert("face", { face.as_str() });
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn snow_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let layers = self.layers().unwrap();
        map.insert("layers", {
            match layers {
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                _ => "unknown",
            }
        });
        map
    }
    fn ice_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn snow_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn cactus_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let age_0_15 = self.age_0_15().unwrap();
        map.insert("age", {
            match age_0_15 {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn clay_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn sugar_cane_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let age_0_15 = self.age_0_15().unwrap();
        map.insert("age", {
            match age_0_15 {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn jukebox_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let has_record = self.has_record().unwrap();
        map.insert("has_record", {
            match has_record {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn oak_fence_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn pumpkin_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn netherrack_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn soul_sand_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn glowstone_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn nether_portal_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xz = self.axis_xz().unwrap();
        map.insert("axis", { axis_xz.as_str() });
        map
    }
    fn carved_pumpkin_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn jack_o_lantern_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn cake_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let bites = self.bites().unwrap();
        map.insert("bites", {
            match bites {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                _ => "unknown",
            }
        });
        map
    }
    fn repeater_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let delay = self.delay().unwrap();
        map.insert("delay", {
            match delay {
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                _ => "unknown",
            }
        });
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let locked = self.locked().unwrap();
        map.insert("locked", {
            match locked {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn white_stained_glass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn orange_stained_glass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn magenta_stained_glass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn light_blue_stained_glass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn yellow_stained_glass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn lime_stained_glass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn pink_stained_glass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn gray_stained_glass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn light_gray_stained_glass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn cyan_stained_glass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn purple_stained_glass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn blue_stained_glass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn brown_stained_glass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn green_stained_glass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn red_stained_glass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn black_stained_glass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn oak_trapdoor_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let open = self.open().unwrap();
        map.insert("open", {
            match open {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn spruce_trapdoor_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let open = self.open().unwrap();
        map.insert("open", {
            match open {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn birch_trapdoor_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let open = self.open().unwrap();
        map.insert("open", {
            match open {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn jungle_trapdoor_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let open = self.open().unwrap();
        map.insert("open", {
            match open {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn acacia_trapdoor_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let open = self.open().unwrap();
        map.insert("open", {
            match open {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dark_oak_trapdoor_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let open = self.open().unwrap();
        map.insert("open", {
            match open {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn infested_stone_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn infested_cobblestone_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn infested_stone_bricks_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn infested_mossy_stone_bricks_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn infested_cracked_stone_bricks_to_properties_map(
        self,
    ) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn infested_chiseled_stone_bricks_to_properties_map(
        self,
    ) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn stone_bricks_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn mossy_stone_bricks_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn cracked_stone_bricks_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn chiseled_stone_bricks_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn brown_mushroom_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let down = self.down().unwrap();
        map.insert("down", {
            match down {
                true => "true",
                false => "false",
            }
        });
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let up = self.up().unwrap();
        map.insert("up", {
            match up {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn red_mushroom_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let down = self.down().unwrap();
        map.insert("down", {
            match down {
                true => "true",
                false => "false",
            }
        });
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let up = self.up().unwrap();
        map.insert("up", {
            match up {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn mushroom_stem_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let down = self.down().unwrap();
        map.insert("down", {
            match down {
                true => "true",
                false => "false",
            }
        });
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let up = self.up().unwrap();
        map.insert("up", {
            match up {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn iron_bars_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn glass_pane_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn melon_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn attached_pumpkin_stem_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn attached_melon_stem_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn pumpkin_stem_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let age_0_7 = self.age_0_7().unwrap();
        map.insert("age", {
            match age_0_7 {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                _ => "unknown",
            }
        });
        map
    }
    fn melon_stem_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let age_0_7 = self.age_0_7().unwrap();
        map.insert("age", {
            match age_0_7 {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                _ => "unknown",
            }
        });
        map
    }
    fn vine_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let up = self.up().unwrap();
        map.insert("up", {
            match up {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn oak_fence_gate_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let in_wall = self.in_wall().unwrap();
        map.insert("in_wall", {
            match in_wall {
                true => "true",
                false => "false",
            }
        });
        let open = self.open().unwrap();
        map.insert("open", {
            match open {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn brick_stairs_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let stairs_shape = self.stairs_shape().unwrap();
        map.insert("shape", { stairs_shape.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn stone_brick_stairs_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let stairs_shape = self.stairs_shape().unwrap();
        map.insert("shape", { stairs_shape.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn mycelium_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let snowy = self.snowy().unwrap();
        map.insert("snowy", {
            match snowy {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn lily_pad_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn nether_bricks_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn nether_brick_fence_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn nether_brick_stairs_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let stairs_shape = self.stairs_shape().unwrap();
        map.insert("shape", { stairs_shape.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn nether_wart_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let age_0_3 = self.age_0_3().unwrap();
        map.insert("age", {
            match age_0_3 {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                _ => "unknown",
            }
        });
        map
    }
    fn enchanting_table_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn brewing_stand_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let has_bottle_0 = self.has_bottle_0().unwrap();
        map.insert("has_bottle_0", {
            match has_bottle_0 {
                true => "true",
                false => "false",
            }
        });
        let has_bottle_1 = self.has_bottle_1().unwrap();
        map.insert("has_bottle_1", {
            match has_bottle_1 {
                true => "true",
                false => "false",
            }
        });
        let has_bottle_2 = self.has_bottle_2().unwrap();
        map.insert("has_bottle_2", {
            match has_bottle_2 {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn cauldron_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let cauldron_level = self.cauldron_level().unwrap();
        map.insert("level", {
            match cauldron_level {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                _ => "unknown",
            }
        });
        map
    }
    fn end_portal_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn end_portal_frame_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let eye = self.eye().unwrap();
        map.insert("eye", {
            match eye {
                true => "true",
                false => "false",
            }
        });
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn end_stone_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn dragon_egg_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn redstone_lamp_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let lit = self.lit().unwrap();
        map.insert("lit", {
            match lit {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn cocoa_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let age_0_2 = self.age_0_2().unwrap();
        map.insert("age", {
            match age_0_2 {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                _ => "unknown",
            }
        });
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn sandstone_stairs_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let stairs_shape = self.stairs_shape().unwrap();
        map.insert("shape", { stairs_shape.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn emerald_ore_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn ender_chest_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn tripwire_hook_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let attached = self.attached().unwrap();
        map.insert("attached", {
            match attached {
                true => "true",
                false => "false",
            }
        });
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn tripwire_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let attached = self.attached().unwrap();
        map.insert("attached", {
            match attached {
                true => "true",
                false => "false",
            }
        });
        let disarmed = self.disarmed().unwrap();
        map.insert("disarmed", {
            match disarmed {
                true => "true",
                false => "false",
            }
        });
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn emerald_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn spruce_stairs_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let stairs_shape = self.stairs_shape().unwrap();
        map.insert("shape", { stairs_shape.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn birch_stairs_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let stairs_shape = self.stairs_shape().unwrap();
        map.insert("shape", { stairs_shape.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn jungle_stairs_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let stairs_shape = self.stairs_shape().unwrap();
        map.insert("shape", { stairs_shape.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn command_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let conditional = self.conditional().unwrap();
        map.insert("conditional", {
            match conditional {
                true => "true",
                false => "false",
            }
        });
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn beacon_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn cobblestone_wall_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let up = self.up().unwrap();
        map.insert("up", {
            match up {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn mossy_cobblestone_wall_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let up = self.up().unwrap();
        map.insert("up", {
            match up {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn flower_pot_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_oak_sapling_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_spruce_sapling_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_birch_sapling_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_jungle_sapling_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_acacia_sapling_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_dark_oak_sapling_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_fern_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_dandelion_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_poppy_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_blue_orchid_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_allium_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_azure_bluet_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_red_tulip_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_orange_tulip_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_white_tulip_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_pink_tulip_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_oxeye_daisy_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_red_mushroom_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_brown_mushroom_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_dead_bush_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn potted_cactus_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn carrots_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let age_0_7 = self.age_0_7().unwrap();
        map.insert("age", {
            match age_0_7 {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                _ => "unknown",
            }
        });
        map
    }
    fn potatoes_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let age_0_7 = self.age_0_7().unwrap();
        map.insert("age", {
            match age_0_7 {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                _ => "unknown",
            }
        });
        map
    }
    fn oak_button_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let face = self.face().unwrap();
        map.insert("face", { face.as_str() });
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn spruce_button_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let face = self.face().unwrap();
        map.insert("face", { face.as_str() });
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn birch_button_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let face = self.face().unwrap();
        map.insert("face", { face.as_str() });
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn jungle_button_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let face = self.face().unwrap();
        map.insert("face", { face.as_str() });
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn acacia_button_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let face = self.face().unwrap();
        map.insert("face", { face.as_str() });
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dark_oak_button_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let face = self.face().unwrap();
        map.insert("face", { face.as_str() });
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn skeleton_wall_skull_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn skeleton_skull_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn wither_skeleton_wall_skull_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn wither_skeleton_skull_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn zombie_wall_head_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn zombie_head_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn player_wall_head_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn player_head_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn creeper_wall_head_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn creeper_head_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn dragon_wall_head_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn dragon_head_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn anvil_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn chipped_anvil_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn damaged_anvil_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn trapped_chest_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let chest_kind = self.chest_kind().unwrap();
        map.insert("type", { chest_kind.as_str() });
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn light_weighted_pressure_plate_to_properties_map(
        self,
    ) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let power = self.power().unwrap();
        map.insert("power", {
            match power {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn heavy_weighted_pressure_plate_to_properties_map(
        self,
    ) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let power = self.power().unwrap();
        map.insert("power", {
            match power {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn comparator_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let comparator_mode = self.comparator_mode().unwrap();
        map.insert("mode", { comparator_mode.as_str() });
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn daylight_detector_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let inverted = self.inverted().unwrap();
        map.insert("inverted", {
            match inverted {
                true => "true",
                false => "false",
            }
        });
        let power = self.power().unwrap();
        map.insert("power", {
            match power {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn redstone_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn nether_quartz_ore_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn hopper_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let enabled = self.enabled().unwrap();
        map.insert("enabled", {
            match enabled {
                true => "true",
                false => "false",
            }
        });
        let facing_cardinal_and_down = self.facing_cardinal_and_down().unwrap();
        map.insert("facing", { facing_cardinal_and_down.as_str() });
        map
    }
    fn quartz_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn chiseled_quartz_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn quartz_pillar_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn quartz_stairs_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let stairs_shape = self.stairs_shape().unwrap();
        map.insert("shape", { stairs_shape.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn activator_rail_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        let powered_rail_shape = self.powered_rail_shape().unwrap();
        map.insert("shape", { powered_rail_shape.as_str() });
        map
    }
    fn dropper_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        let triggered = self.triggered().unwrap();
        map.insert("triggered", {
            match triggered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn white_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn orange_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn magenta_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn light_blue_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn yellow_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn lime_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn pink_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn gray_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn light_gray_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn cyan_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn purple_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn blue_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn brown_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn green_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn red_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn black_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn white_stained_glass_pane_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn orange_stained_glass_pane_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn magenta_stained_glass_pane_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn light_blue_stained_glass_pane_to_properties_map(
        self,
    ) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn yellow_stained_glass_pane_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn lime_stained_glass_pane_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn pink_stained_glass_pane_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn gray_stained_glass_pane_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn light_gray_stained_glass_pane_to_properties_map(
        self,
    ) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn cyan_stained_glass_pane_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn purple_stained_glass_pane_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn blue_stained_glass_pane_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn brown_stained_glass_pane_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn green_stained_glass_pane_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn red_stained_glass_pane_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn black_stained_glass_pane_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn acacia_stairs_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let stairs_shape = self.stairs_shape().unwrap();
        map.insert("shape", { stairs_shape.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dark_oak_stairs_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let stairs_shape = self.stairs_shape().unwrap();
        map.insert("shape", { stairs_shape.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn slime_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn barrier_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn iron_trapdoor_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let open = self.open().unwrap();
        map.insert("open", {
            match open {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn prismarine_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn prismarine_bricks_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn dark_prismarine_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn prismarine_stairs_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let stairs_shape = self.stairs_shape().unwrap();
        map.insert("shape", { stairs_shape.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn prismarine_brick_stairs_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let stairs_shape = self.stairs_shape().unwrap();
        map.insert("shape", { stairs_shape.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dark_prismarine_stairs_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let stairs_shape = self.stairs_shape().unwrap();
        map.insert("shape", { stairs_shape.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn prismarine_slab_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let slab_kind = self.slab_kind().unwrap();
        map.insert("type", { slab_kind.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn prismarine_brick_slab_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let slab_kind = self.slab_kind().unwrap();
        map.insert("type", { slab_kind.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dark_prismarine_slab_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let slab_kind = self.slab_kind().unwrap();
        map.insert("type", { slab_kind.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn sea_lantern_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn hay_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn white_carpet_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn orange_carpet_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn magenta_carpet_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn light_blue_carpet_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn yellow_carpet_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn lime_carpet_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn pink_carpet_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn gray_carpet_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn light_gray_carpet_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn cyan_carpet_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn purple_carpet_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn blue_carpet_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn brown_carpet_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn green_carpet_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn red_carpet_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn black_carpet_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn coal_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn packed_ice_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn sunflower_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let half_upper_lower = self.half_upper_lower().unwrap();
        map.insert("half", { half_upper_lower.as_str() });
        map
    }
    fn lilac_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let half_upper_lower = self.half_upper_lower().unwrap();
        map.insert("half", { half_upper_lower.as_str() });
        map
    }
    fn rose_bush_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let half_upper_lower = self.half_upper_lower().unwrap();
        map.insert("half", { half_upper_lower.as_str() });
        map
    }
    fn peony_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let half_upper_lower = self.half_upper_lower().unwrap();
        map.insert("half", { half_upper_lower.as_str() });
        map
    }
    fn tall_grass_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let half_upper_lower = self.half_upper_lower().unwrap();
        map.insert("half", { half_upper_lower.as_str() });
        map
    }
    fn large_fern_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let half_upper_lower = self.half_upper_lower().unwrap();
        map.insert("half", { half_upper_lower.as_str() });
        map
    }
    fn white_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn orange_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn magenta_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn light_blue_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn yellow_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn lime_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn pink_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn gray_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn light_gray_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn cyan_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn purple_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn blue_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn brown_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn green_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn red_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn black_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let rotation = self.rotation().unwrap();
        map.insert("rotation", {
            match rotation {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                _ => "unknown",
            }
        });
        map
    }
    fn white_wall_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn orange_wall_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn magenta_wall_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn light_blue_wall_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn yellow_wall_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn lime_wall_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn pink_wall_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn gray_wall_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn light_gray_wall_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn cyan_wall_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn purple_wall_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn blue_wall_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn brown_wall_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn green_wall_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn red_wall_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn black_wall_banner_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn red_sandstone_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn chiseled_red_sandstone_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn cut_red_sandstone_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn red_sandstone_stairs_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let stairs_shape = self.stairs_shape().unwrap();
        map.insert("shape", { stairs_shape.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn oak_slab_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let slab_kind = self.slab_kind().unwrap();
        map.insert("type", { slab_kind.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn spruce_slab_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let slab_kind = self.slab_kind().unwrap();
        map.insert("type", { slab_kind.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn birch_slab_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let slab_kind = self.slab_kind().unwrap();
        map.insert("type", { slab_kind.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn jungle_slab_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let slab_kind = self.slab_kind().unwrap();
        map.insert("type", { slab_kind.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn acacia_slab_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let slab_kind = self.slab_kind().unwrap();
        map.insert("type", { slab_kind.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dark_oak_slab_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let slab_kind = self.slab_kind().unwrap();
        map.insert("type", { slab_kind.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn stone_slab_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let slab_kind = self.slab_kind().unwrap();
        map.insert("type", { slab_kind.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn sandstone_slab_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let slab_kind = self.slab_kind().unwrap();
        map.insert("type", { slab_kind.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn petrified_oak_slab_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let slab_kind = self.slab_kind().unwrap();
        map.insert("type", { slab_kind.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn cobblestone_slab_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let slab_kind = self.slab_kind().unwrap();
        map.insert("type", { slab_kind.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn brick_slab_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let slab_kind = self.slab_kind().unwrap();
        map.insert("type", { slab_kind.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn stone_brick_slab_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let slab_kind = self.slab_kind().unwrap();
        map.insert("type", { slab_kind.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn nether_brick_slab_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let slab_kind = self.slab_kind().unwrap();
        map.insert("type", { slab_kind.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn quartz_slab_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let slab_kind = self.slab_kind().unwrap();
        map.insert("type", { slab_kind.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn red_sandstone_slab_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let slab_kind = self.slab_kind().unwrap();
        map.insert("type", { slab_kind.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn purpur_slab_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let slab_kind = self.slab_kind().unwrap();
        map.insert("type", { slab_kind.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn smooth_stone_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn smooth_sandstone_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn smooth_quartz_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn smooth_red_sandstone_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn spruce_fence_gate_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let in_wall = self.in_wall().unwrap();
        map.insert("in_wall", {
            match in_wall {
                true => "true",
                false => "false",
            }
        });
        let open = self.open().unwrap();
        map.insert("open", {
            match open {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn birch_fence_gate_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let in_wall = self.in_wall().unwrap();
        map.insert("in_wall", {
            match in_wall {
                true => "true",
                false => "false",
            }
        });
        let open = self.open().unwrap();
        map.insert("open", {
            match open {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn jungle_fence_gate_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let in_wall = self.in_wall().unwrap();
        map.insert("in_wall", {
            match in_wall {
                true => "true",
                false => "false",
            }
        });
        let open = self.open().unwrap();
        map.insert("open", {
            match open {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn acacia_fence_gate_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let in_wall = self.in_wall().unwrap();
        map.insert("in_wall", {
            match in_wall {
                true => "true",
                false => "false",
            }
        });
        let open = self.open().unwrap();
        map.insert("open", {
            match open {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dark_oak_fence_gate_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let in_wall = self.in_wall().unwrap();
        map.insert("in_wall", {
            match in_wall {
                true => "true",
                false => "false",
            }
        });
        let open = self.open().unwrap();
        map.insert("open", {
            match open {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn spruce_fence_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn birch_fence_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn jungle_fence_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn acacia_fence_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dark_oak_fence_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn spruce_door_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_upper_lower = self.half_upper_lower().unwrap();
        map.insert("half", { half_upper_lower.as_str() });
        let hinge = self.hinge().unwrap();
        map.insert("hinge", { hinge.as_str() });
        let open = self.open().unwrap();
        map.insert("open", {
            match open {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn birch_door_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_upper_lower = self.half_upper_lower().unwrap();
        map.insert("half", { half_upper_lower.as_str() });
        let hinge = self.hinge().unwrap();
        map.insert("hinge", { hinge.as_str() });
        let open = self.open().unwrap();
        map.insert("open", {
            match open {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn jungle_door_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_upper_lower = self.half_upper_lower().unwrap();
        map.insert("half", { half_upper_lower.as_str() });
        let hinge = self.hinge().unwrap();
        map.insert("hinge", { hinge.as_str() });
        let open = self.open().unwrap();
        map.insert("open", {
            match open {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn acacia_door_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_upper_lower = self.half_upper_lower().unwrap();
        map.insert("half", { half_upper_lower.as_str() });
        let hinge = self.hinge().unwrap();
        map.insert("hinge", { hinge.as_str() });
        let open = self.open().unwrap();
        map.insert("open", {
            match open {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dark_oak_door_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_upper_lower = self.half_upper_lower().unwrap();
        map.insert("half", { half_upper_lower.as_str() });
        let hinge = self.hinge().unwrap();
        map.insert("hinge", { hinge.as_str() });
        let open = self.open().unwrap();
        map.insert("open", {
            match open {
                true => "true",
                false => "false",
            }
        });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn end_rod_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn chorus_plant_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let down = self.down().unwrap();
        map.insert("down", {
            match down {
                true => "true",
                false => "false",
            }
        });
        let east_connected = self.east_connected().unwrap();
        map.insert("east", {
            match east_connected {
                true => "true",
                false => "false",
            }
        });
        let north_connected = self.north_connected().unwrap();
        map.insert("north", {
            match north_connected {
                true => "true",
                false => "false",
            }
        });
        let south_connected = self.south_connected().unwrap();
        map.insert("south", {
            match south_connected {
                true => "true",
                false => "false",
            }
        });
        let up = self.up().unwrap();
        map.insert("up", {
            match up {
                true => "true",
                false => "false",
            }
        });
        let west_connected = self.west_connected().unwrap();
        map.insert("west", {
            match west_connected {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn chorus_flower_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let age_0_5 = self.age_0_5().unwrap();
        map.insert("age", {
            match age_0_5 {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                _ => "unknown",
            }
        });
        map
    }
    fn purpur_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn purpur_pillar_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn purpur_stairs_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let half_top_bottom = self.half_top_bottom().unwrap();
        map.insert("half", { half_top_bottom.as_str() });
        let stairs_shape = self.stairs_shape().unwrap();
        map.insert("shape", { stairs_shape.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn end_stone_bricks_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn beetroots_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let age_0_3 = self.age_0_3().unwrap();
        map.insert("age", {
            match age_0_3 {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                _ => "unknown",
            }
        });
        map
    }
    fn grass_path_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn end_gateway_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn repeating_command_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let conditional = self.conditional().unwrap();
        map.insert("conditional", {
            match conditional {
                true => "true",
                false => "false",
            }
        });
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn chain_command_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let conditional = self.conditional().unwrap();
        map.insert("conditional", {
            match conditional {
                true => "true",
                false => "false",
            }
        });
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn frosted_ice_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let age_0_3 = self.age_0_3().unwrap();
        map.insert("age", {
            match age_0_3 {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                _ => "unknown",
            }
        });
        map
    }
    fn magma_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn nether_wart_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn red_nether_bricks_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn bone_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let axis_xyz = self.axis_xyz().unwrap();
        map.insert("axis", { axis_xyz.as_str() });
        map
    }
    fn structure_void_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn observer_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        let powered = self.powered().unwrap();
        map.insert("powered", {
            match powered {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn shulker_box_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn white_shulker_box_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn orange_shulker_box_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn magenta_shulker_box_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn light_blue_shulker_box_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn yellow_shulker_box_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn lime_shulker_box_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn pink_shulker_box_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn gray_shulker_box_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn light_gray_shulker_box_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn cyan_shulker_box_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn purple_shulker_box_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn blue_shulker_box_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn brown_shulker_box_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn green_shulker_box_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn red_shulker_box_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn black_shulker_box_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cubic = self.facing_cubic().unwrap();
        map.insert("facing", { facing_cubic.as_str() });
        map
    }
    fn white_glazed_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn orange_glazed_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn magenta_glazed_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn light_blue_glazed_terracotta_to_properties_map(
        self,
    ) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn yellow_glazed_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn lime_glazed_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn pink_glazed_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn gray_glazed_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn light_gray_glazed_terracotta_to_properties_map(
        self,
    ) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn cyan_glazed_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn purple_glazed_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn blue_glazed_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn brown_glazed_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn green_glazed_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn red_glazed_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn black_glazed_terracotta_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        map
    }
    fn white_concrete_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn orange_concrete_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn magenta_concrete_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn light_blue_concrete_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn yellow_concrete_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn lime_concrete_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn pink_concrete_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn gray_concrete_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn light_gray_concrete_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn cyan_concrete_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn purple_concrete_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn blue_concrete_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn brown_concrete_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn green_concrete_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn red_concrete_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn black_concrete_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn white_concrete_powder_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn orange_concrete_powder_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn magenta_concrete_powder_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn light_blue_concrete_powder_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn yellow_concrete_powder_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn lime_concrete_powder_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn pink_concrete_powder_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn gray_concrete_powder_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn light_gray_concrete_powder_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn cyan_concrete_powder_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn purple_concrete_powder_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn blue_concrete_powder_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn brown_concrete_powder_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn green_concrete_powder_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn red_concrete_powder_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn black_concrete_powder_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn kelp_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let age_0_25 = self.age_0_25().unwrap();
        map.insert("age", {
            match age_0_25 {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                5i32 => "5",
                6i32 => "6",
                7i32 => "7",
                8i32 => "8",
                9i32 => "9",
                10i32 => "10",
                11i32 => "11",
                12i32 => "12",
                13i32 => "13",
                14i32 => "14",
                15i32 => "15",
                16i32 => "16",
                17i32 => "17",
                18i32 => "18",
                19i32 => "19",
                20i32 => "20",
                21i32 => "21",
                22i32 => "22",
                23i32 => "23",
                24i32 => "24",
                25i32 => "25",
                _ => "unknown",
            }
        });
        map
    }
    fn kelp_plant_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn dried_kelp_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn turtle_egg_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let eggs = self.eggs().unwrap();
        map.insert("eggs", {
            match eggs {
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                _ => "unknown",
            }
        });
        let hatch = self.hatch().unwrap();
        map.insert("hatch", {
            match hatch {
                0i32 => "0",
                1i32 => "1",
                2i32 => "2",
                _ => "unknown",
            }
        });
        map
    }
    fn dead_tube_coral_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn dead_brain_coral_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn dead_bubble_coral_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn dead_fire_coral_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn dead_horn_coral_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn tube_coral_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn brain_coral_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn bubble_coral_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn fire_coral_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn horn_coral_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn dead_tube_coral_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dead_brain_coral_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dead_bubble_coral_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dead_fire_coral_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dead_horn_coral_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn tube_coral_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn brain_coral_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn bubble_coral_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn fire_coral_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn horn_coral_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dead_tube_coral_wall_fan_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dead_brain_coral_wall_fan_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dead_bubble_coral_wall_fan_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dead_fire_coral_wall_fan_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dead_horn_coral_wall_fan_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn tube_coral_wall_fan_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn brain_coral_wall_fan_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn bubble_coral_wall_fan_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn fire_coral_wall_fan_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn horn_coral_wall_fan_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let facing_cardinal = self.facing_cardinal().unwrap();
        map.insert("facing", { facing_cardinal.as_str() });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dead_tube_coral_fan_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dead_brain_coral_fan_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dead_bubble_coral_fan_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dead_fire_coral_fan_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn dead_horn_coral_fan_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn tube_coral_fan_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn brain_coral_fan_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn bubble_coral_fan_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn fire_coral_fan_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn horn_coral_fan_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn sea_pickle_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let pickles = self.pickles().unwrap();
        map.insert("pickles", {
            match pickles {
                1i32 => "1",
                2i32 => "2",
                3i32 => "3",
                4i32 => "4",
                _ => "unknown",
            }
        });
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn blue_ice_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn conduit_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let waterlogged = self.waterlogged().unwrap();
        map.insert("waterlogged", {
            match waterlogged {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn void_air_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn cave_air_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        map
    }
    fn bubble_column_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let drag = self.drag().unwrap();
        map.insert("drag", {
            match drag {
                true => "true",
                false => "false",
            }
        });
        map
    }
    fn structure_block_to_properties_map(self) -> BTreeMap<&'static str, &'static str> {
        let mut map = BTreeMap::new();
        let structure_block_mode = self.structure_block_mode().unwrap();
        map.insert("mode", { structure_block_mode.as_str() });
        map
    }
    #[doc = "Attempts to convert a block kind identifier (e.g. `minecraft::air`) and properties map to a `BlockId`."]
    pub fn from_identifier_and_properties(
        identifier: &str,
        properties: &BTreeMap<String, String>,
    ) -> Option<Self> {
        match identifier {
            "minecraft:air" => Self::air_from_identifier_and_properties(properties),
            "minecraft:stone" => Self::stone_from_identifier_and_properties(properties),
            "minecraft:granite" => Self::granite_from_identifier_and_properties(properties),
            "minecraft:polished_granite" => {
                Self::polished_granite_from_identifier_and_properties(properties)
            }
            "minecraft:diorite" => Self::diorite_from_identifier_and_properties(properties),
            "minecraft:polished_diorite" => {
                Self::polished_diorite_from_identifier_and_properties(properties)
            }
            "minecraft:andesite" => Self::andesite_from_identifier_and_properties(properties),
            "minecraft:polished_andesite" => {
                Self::polished_andesite_from_identifier_and_properties(properties)
            }
            "minecraft:grass_block" => Self::grass_block_from_identifier_and_properties(properties),
            "minecraft:dirt" => Self::dirt_from_identifier_and_properties(properties),
            "minecraft:coarse_dirt" => Self::coarse_dirt_from_identifier_and_properties(properties),
            "minecraft:podzol" => Self::podzol_from_identifier_and_properties(properties),
            "minecraft:cobblestone" => Self::cobblestone_from_identifier_and_properties(properties),
            "minecraft:oak_planks" => Self::oak_planks_from_identifier_and_properties(properties),
            "minecraft:spruce_planks" => {
                Self::spruce_planks_from_identifier_and_properties(properties)
            }
            "minecraft:birch_planks" => {
                Self::birch_planks_from_identifier_and_properties(properties)
            }
            "minecraft:jungle_planks" => {
                Self::jungle_planks_from_identifier_and_properties(properties)
            }
            "minecraft:acacia_planks" => {
                Self::acacia_planks_from_identifier_and_properties(properties)
            }
            "minecraft:dark_oak_planks" => {
                Self::dark_oak_planks_from_identifier_and_properties(properties)
            }
            "minecraft:oak_sapling" => Self::oak_sapling_from_identifier_and_properties(properties),
            "minecraft:spruce_sapling" => {
                Self::spruce_sapling_from_identifier_and_properties(properties)
            }
            "minecraft:birch_sapling" => {
                Self::birch_sapling_from_identifier_and_properties(properties)
            }
            "minecraft:jungle_sapling" => {
                Self::jungle_sapling_from_identifier_and_properties(properties)
            }
            "minecraft:acacia_sapling" => {
                Self::acacia_sapling_from_identifier_and_properties(properties)
            }
            "minecraft:dark_oak_sapling" => {
                Self::dark_oak_sapling_from_identifier_and_properties(properties)
            }
            "minecraft:bedrock" => Self::bedrock_from_identifier_and_properties(properties),
            "minecraft:water" => Self::water_from_identifier_and_properties(properties),
            "minecraft:lava" => Self::lava_from_identifier_and_properties(properties),
            "minecraft:sand" => Self::sand_from_identifier_and_properties(properties),
            "minecraft:red_sand" => Self::red_sand_from_identifier_and_properties(properties),
            "minecraft:gravel" => Self::gravel_from_identifier_and_properties(properties),
            "minecraft:gold_ore" => Self::gold_ore_from_identifier_and_properties(properties),
            "minecraft:iron_ore" => Self::iron_ore_from_identifier_and_properties(properties),
            "minecraft:coal_ore" => Self::coal_ore_from_identifier_and_properties(properties),
            "minecraft:oak_log" => Self::oak_log_from_identifier_and_properties(properties),
            "minecraft:spruce_log" => Self::spruce_log_from_identifier_and_properties(properties),
            "minecraft:birch_log" => Self::birch_log_from_identifier_and_properties(properties),
            "minecraft:jungle_log" => Self::jungle_log_from_identifier_and_properties(properties),
            "minecraft:acacia_log" => Self::acacia_log_from_identifier_and_properties(properties),
            "minecraft:dark_oak_log" => {
                Self::dark_oak_log_from_identifier_and_properties(properties)
            }
            "minecraft:stripped_spruce_log" => {
                Self::stripped_spruce_log_from_identifier_and_properties(properties)
            }
            "minecraft:stripped_birch_log" => {
                Self::stripped_birch_log_from_identifier_and_properties(properties)
            }
            "minecraft:stripped_jungle_log" => {
                Self::stripped_jungle_log_from_identifier_and_properties(properties)
            }
            "minecraft:stripped_acacia_log" => {
                Self::stripped_acacia_log_from_identifier_and_properties(properties)
            }
            "minecraft:stripped_dark_oak_log" => {
                Self::stripped_dark_oak_log_from_identifier_and_properties(properties)
            }
            "minecraft:stripped_oak_log" => {
                Self::stripped_oak_log_from_identifier_and_properties(properties)
            }
            "minecraft:oak_wood" => Self::oak_wood_from_identifier_and_properties(properties),
            "minecraft:spruce_wood" => Self::spruce_wood_from_identifier_and_properties(properties),
            "minecraft:birch_wood" => Self::birch_wood_from_identifier_and_properties(properties),
            "minecraft:jungle_wood" => Self::jungle_wood_from_identifier_and_properties(properties),
            "minecraft:acacia_wood" => Self::acacia_wood_from_identifier_and_properties(properties),
            "minecraft:dark_oak_wood" => {
                Self::dark_oak_wood_from_identifier_and_properties(properties)
            }
            "minecraft:stripped_oak_wood" => {
                Self::stripped_oak_wood_from_identifier_and_properties(properties)
            }
            "minecraft:stripped_spruce_wood" => {
                Self::stripped_spruce_wood_from_identifier_and_properties(properties)
            }
            "minecraft:stripped_birch_wood" => {
                Self::stripped_birch_wood_from_identifier_and_properties(properties)
            }
            "minecraft:stripped_jungle_wood" => {
                Self::stripped_jungle_wood_from_identifier_and_properties(properties)
            }
            "minecraft:stripped_acacia_wood" => {
                Self::stripped_acacia_wood_from_identifier_and_properties(properties)
            }
            "minecraft:stripped_dark_oak_wood" => {
                Self::stripped_dark_oak_wood_from_identifier_and_properties(properties)
            }
            "minecraft:oak_leaves" => Self::oak_leaves_from_identifier_and_properties(properties),
            "minecraft:spruce_leaves" => {
                Self::spruce_leaves_from_identifier_and_properties(properties)
            }
            "minecraft:birch_leaves" => {
                Self::birch_leaves_from_identifier_and_properties(properties)
            }
            "minecraft:jungle_leaves" => {
                Self::jungle_leaves_from_identifier_and_properties(properties)
            }
            "minecraft:acacia_leaves" => {
                Self::acacia_leaves_from_identifier_and_properties(properties)
            }
            "minecraft:dark_oak_leaves" => {
                Self::dark_oak_leaves_from_identifier_and_properties(properties)
            }
            "minecraft:sponge" => Self::sponge_from_identifier_and_properties(properties),
            "minecraft:wet_sponge" => Self::wet_sponge_from_identifier_and_properties(properties),
            "minecraft:glass" => Self::glass_from_identifier_and_properties(properties),
            "minecraft:lapis_ore" => Self::lapis_ore_from_identifier_and_properties(properties),
            "minecraft:lapis_block" => Self::lapis_block_from_identifier_and_properties(properties),
            "minecraft:dispenser" => Self::dispenser_from_identifier_and_properties(properties),
            "minecraft:sandstone" => Self::sandstone_from_identifier_and_properties(properties),
            "minecraft:chiseled_sandstone" => {
                Self::chiseled_sandstone_from_identifier_and_properties(properties)
            }
            "minecraft:cut_sandstone" => {
                Self::cut_sandstone_from_identifier_and_properties(properties)
            }
            "minecraft:note_block" => Self::note_block_from_identifier_and_properties(properties),
            "minecraft:white_bed" => Self::white_bed_from_identifier_and_properties(properties),
            "minecraft:orange_bed" => Self::orange_bed_from_identifier_and_properties(properties),
            "minecraft:magenta_bed" => Self::magenta_bed_from_identifier_and_properties(properties),
            "minecraft:light_blue_bed" => {
                Self::light_blue_bed_from_identifier_and_properties(properties)
            }
            "minecraft:yellow_bed" => Self::yellow_bed_from_identifier_and_properties(properties),
            "minecraft:lime_bed" => Self::lime_bed_from_identifier_and_properties(properties),
            "minecraft:pink_bed" => Self::pink_bed_from_identifier_and_properties(properties),
            "minecraft:gray_bed" => Self::gray_bed_from_identifier_and_properties(properties),
            "minecraft:light_gray_bed" => {
                Self::light_gray_bed_from_identifier_and_properties(properties)
            }
            "minecraft:cyan_bed" => Self::cyan_bed_from_identifier_and_properties(properties),
            "minecraft:purple_bed" => Self::purple_bed_from_identifier_and_properties(properties),
            "minecraft:blue_bed" => Self::blue_bed_from_identifier_and_properties(properties),
            "minecraft:brown_bed" => Self::brown_bed_from_identifier_and_properties(properties),
            "minecraft:green_bed" => Self::green_bed_from_identifier_and_properties(properties),
            "minecraft:red_bed" => Self::red_bed_from_identifier_and_properties(properties),
            "minecraft:black_bed" => Self::black_bed_from_identifier_and_properties(properties),
            "minecraft:powered_rail" => {
                Self::powered_rail_from_identifier_and_properties(properties)
            }
            "minecraft:detector_rail" => {
                Self::detector_rail_from_identifier_and_properties(properties)
            }
            "minecraft:sticky_piston" => {
                Self::sticky_piston_from_identifier_and_properties(properties)
            }
            "minecraft:cobweb" => Self::cobweb_from_identifier_and_properties(properties),
            "minecraft:grass" => Self::grass_from_identifier_and_properties(properties),
            "minecraft:fern" => Self::fern_from_identifier_and_properties(properties),
            "minecraft:dead_bush" => Self::dead_bush_from_identifier_and_properties(properties),
            "minecraft:seagrass" => Self::seagrass_from_identifier_and_properties(properties),
            "minecraft:tall_seagrass" => {
                Self::tall_seagrass_from_identifier_and_properties(properties)
            }
            "minecraft:piston" => Self::piston_from_identifier_and_properties(properties),
            "minecraft:piston_head" => Self::piston_head_from_identifier_and_properties(properties),
            "minecraft:white_wool" => Self::white_wool_from_identifier_and_properties(properties),
            "minecraft:orange_wool" => Self::orange_wool_from_identifier_and_properties(properties),
            "minecraft:magenta_wool" => {
                Self::magenta_wool_from_identifier_and_properties(properties)
            }
            "minecraft:light_blue_wool" => {
                Self::light_blue_wool_from_identifier_and_properties(properties)
            }
            "minecraft:yellow_wool" => Self::yellow_wool_from_identifier_and_properties(properties),
            "minecraft:lime_wool" => Self::lime_wool_from_identifier_and_properties(properties),
            "minecraft:pink_wool" => Self::pink_wool_from_identifier_and_properties(properties),
            "minecraft:gray_wool" => Self::gray_wool_from_identifier_and_properties(properties),
            "minecraft:light_gray_wool" => {
                Self::light_gray_wool_from_identifier_and_properties(properties)
            }
            "minecraft:cyan_wool" => Self::cyan_wool_from_identifier_and_properties(properties),
            "minecraft:purple_wool" => Self::purple_wool_from_identifier_and_properties(properties),
            "minecraft:blue_wool" => Self::blue_wool_from_identifier_and_properties(properties),
            "minecraft:brown_wool" => Self::brown_wool_from_identifier_and_properties(properties),
            "minecraft:green_wool" => Self::green_wool_from_identifier_and_properties(properties),
            "minecraft:red_wool" => Self::red_wool_from_identifier_and_properties(properties),
            "minecraft:black_wool" => Self::black_wool_from_identifier_and_properties(properties),
            "minecraft:moving_piston" => {
                Self::moving_piston_from_identifier_and_properties(properties)
            }
            "minecraft:dandelion" => Self::dandelion_from_identifier_and_properties(properties),
            "minecraft:poppy" => Self::poppy_from_identifier_and_properties(properties),
            "minecraft:blue_orchid" => Self::blue_orchid_from_identifier_and_properties(properties),
            "minecraft:allium" => Self::allium_from_identifier_and_properties(properties),
            "minecraft:azure_bluet" => Self::azure_bluet_from_identifier_and_properties(properties),
            "minecraft:red_tulip" => Self::red_tulip_from_identifier_and_properties(properties),
            "minecraft:orange_tulip" => {
                Self::orange_tulip_from_identifier_and_properties(properties)
            }
            "minecraft:white_tulip" => Self::white_tulip_from_identifier_and_properties(properties),
            "minecraft:pink_tulip" => Self::pink_tulip_from_identifier_and_properties(properties),
            "minecraft:oxeye_daisy" => Self::oxeye_daisy_from_identifier_and_properties(properties),
            "minecraft:brown_mushroom" => {
                Self::brown_mushroom_from_identifier_and_properties(properties)
            }
            "minecraft:red_mushroom" => {
                Self::red_mushroom_from_identifier_and_properties(properties)
            }
            "minecraft:gold_block" => Self::gold_block_from_identifier_and_properties(properties),
            "minecraft:iron_block" => Self::iron_block_from_identifier_and_properties(properties),
            "minecraft:bricks" => Self::bricks_from_identifier_and_properties(properties),
            "minecraft:tnt" => Self::tnt_from_identifier_and_properties(properties),
            "minecraft:bookshelf" => Self::bookshelf_from_identifier_and_properties(properties),
            "minecraft:mossy_cobblestone" => {
                Self::mossy_cobblestone_from_identifier_and_properties(properties)
            }
            "minecraft:obsidian" => Self::obsidian_from_identifier_and_properties(properties),
            "minecraft:torch" => Self::torch_from_identifier_and_properties(properties),
            "minecraft:wall_torch" => Self::wall_torch_from_identifier_and_properties(properties),
            "minecraft:fire" => Self::fire_from_identifier_and_properties(properties),
            "minecraft:spawner" => Self::spawner_from_identifier_and_properties(properties),
            "minecraft:oak_stairs" => Self::oak_stairs_from_identifier_and_properties(properties),
            "minecraft:chest" => Self::chest_from_identifier_and_properties(properties),
            "minecraft:redstone_wire" => {
                Self::redstone_wire_from_identifier_and_properties(properties)
            }
            "minecraft:diamond_ore" => Self::diamond_ore_from_identifier_and_properties(properties),
            "minecraft:diamond_block" => {
                Self::diamond_block_from_identifier_and_properties(properties)
            }
            "minecraft:crafting_table" => {
                Self::crafting_table_from_identifier_and_properties(properties)
            }
            "minecraft:wheat" => Self::wheat_from_identifier_and_properties(properties),
            "minecraft:farmland" => Self::farmland_from_identifier_and_properties(properties),
            "minecraft:furnace" => Self::furnace_from_identifier_and_properties(properties),
            "minecraft:sign" => Self::sign_from_identifier_and_properties(properties),
            "minecraft:oak_door" => Self::oak_door_from_identifier_and_properties(properties),
            "minecraft:ladder" => Self::ladder_from_identifier_and_properties(properties),
            "minecraft:rail" => Self::rail_from_identifier_and_properties(properties),
            "minecraft:cobblestone_stairs" => {
                Self::cobblestone_stairs_from_identifier_and_properties(properties)
            }
            "minecraft:wall_sign" => Self::wall_sign_from_identifier_and_properties(properties),
            "minecraft:lever" => Self::lever_from_identifier_and_properties(properties),
            "minecraft:stone_pressure_plate" => {
                Self::stone_pressure_plate_from_identifier_and_properties(properties)
            }
            "minecraft:iron_door" => Self::iron_door_from_identifier_and_properties(properties),
            "minecraft:oak_pressure_plate" => {
                Self::oak_pressure_plate_from_identifier_and_properties(properties)
            }
            "minecraft:spruce_pressure_plate" => {
                Self::spruce_pressure_plate_from_identifier_and_properties(properties)
            }
            "minecraft:birch_pressure_plate" => {
                Self::birch_pressure_plate_from_identifier_and_properties(properties)
            }
            "minecraft:jungle_pressure_plate" => {
                Self::jungle_pressure_plate_from_identifier_and_properties(properties)
            }
            "minecraft:acacia_pressure_plate" => {
                Self::acacia_pressure_plate_from_identifier_and_properties(properties)
            }
            "minecraft:dark_oak_pressure_plate" => {
                Self::dark_oak_pressure_plate_from_identifier_and_properties(properties)
            }
            "minecraft:redstone_ore" => {
                Self::redstone_ore_from_identifier_and_properties(properties)
            }
            "minecraft:redstone_torch" => {
                Self::redstone_torch_from_identifier_and_properties(properties)
            }
            "minecraft:redstone_wall_torch" => {
                Self::redstone_wall_torch_from_identifier_and_properties(properties)
            }
            "minecraft:stone_button" => {
                Self::stone_button_from_identifier_and_properties(properties)
            }
            "minecraft:snow" => Self::snow_from_identifier_and_properties(properties),
            "minecraft:ice" => Self::ice_from_identifier_and_properties(properties),
            "minecraft:snow_block" => Self::snow_block_from_identifier_and_properties(properties),
            "minecraft:cactus" => Self::cactus_from_identifier_and_properties(properties),
            "minecraft:clay" => Self::clay_from_identifier_and_properties(properties),
            "minecraft:sugar_cane" => Self::sugar_cane_from_identifier_and_properties(properties),
            "minecraft:jukebox" => Self::jukebox_from_identifier_and_properties(properties),
            "minecraft:oak_fence" => Self::oak_fence_from_identifier_and_properties(properties),
            "minecraft:pumpkin" => Self::pumpkin_from_identifier_and_properties(properties),
            "minecraft:netherrack" => Self::netherrack_from_identifier_and_properties(properties),
            "minecraft:soul_sand" => Self::soul_sand_from_identifier_and_properties(properties),
            "minecraft:glowstone" => Self::glowstone_from_identifier_and_properties(properties),
            "minecraft:nether_portal" => {
                Self::nether_portal_from_identifier_and_properties(properties)
            }
            "minecraft:carved_pumpkin" => {
                Self::carved_pumpkin_from_identifier_and_properties(properties)
            }
            "minecraft:jack_o_lantern" => {
                Self::jack_o_lantern_from_identifier_and_properties(properties)
            }
            "minecraft:cake" => Self::cake_from_identifier_and_properties(properties),
            "minecraft:repeater" => Self::repeater_from_identifier_and_properties(properties),
            "minecraft:white_stained_glass" => {
                Self::white_stained_glass_from_identifier_and_properties(properties)
            }
            "minecraft:orange_stained_glass" => {
                Self::orange_stained_glass_from_identifier_and_properties(properties)
            }
            "minecraft:magenta_stained_glass" => {
                Self::magenta_stained_glass_from_identifier_and_properties(properties)
            }
            "minecraft:light_blue_stained_glass" => {
                Self::light_blue_stained_glass_from_identifier_and_properties(properties)
            }
            "minecraft:yellow_stained_glass" => {
                Self::yellow_stained_glass_from_identifier_and_properties(properties)
            }
            "minecraft:lime_stained_glass" => {
                Self::lime_stained_glass_from_identifier_and_properties(properties)
            }
            "minecraft:pink_stained_glass" => {
                Self::pink_stained_glass_from_identifier_and_properties(properties)
            }
            "minecraft:gray_stained_glass" => {
                Self::gray_stained_glass_from_identifier_and_properties(properties)
            }
            "minecraft:light_gray_stained_glass" => {
                Self::light_gray_stained_glass_from_identifier_and_properties(properties)
            }
            "minecraft:cyan_stained_glass" => {
                Self::cyan_stained_glass_from_identifier_and_properties(properties)
            }
            "minecraft:purple_stained_glass" => {
                Self::purple_stained_glass_from_identifier_and_properties(properties)
            }
            "minecraft:blue_stained_glass" => {
                Self::blue_stained_glass_from_identifier_and_properties(properties)
            }
            "minecraft:brown_stained_glass" => {
                Self::brown_stained_glass_from_identifier_and_properties(properties)
            }
            "minecraft:green_stained_glass" => {
                Self::green_stained_glass_from_identifier_and_properties(properties)
            }
            "minecraft:red_stained_glass" => {
                Self::red_stained_glass_from_identifier_and_properties(properties)
            }
            "minecraft:black_stained_glass" => {
                Self::black_stained_glass_from_identifier_and_properties(properties)
            }
            "minecraft:oak_trapdoor" => {
                Self::oak_trapdoor_from_identifier_and_properties(properties)
            }
            "minecraft:spruce_trapdoor" => {
                Self::spruce_trapdoor_from_identifier_and_properties(properties)
            }
            "minecraft:birch_trapdoor" => {
                Self::birch_trapdoor_from_identifier_and_properties(properties)
            }
            "minecraft:jungle_trapdoor" => {
                Self::jungle_trapdoor_from_identifier_and_properties(properties)
            }
            "minecraft:acacia_trapdoor" => {
                Self::acacia_trapdoor_from_identifier_and_properties(properties)
            }
            "minecraft:dark_oak_trapdoor" => {
                Self::dark_oak_trapdoor_from_identifier_and_properties(properties)
            }
            "minecraft:infested_stone" => {
                Self::infested_stone_from_identifier_and_properties(properties)
            }
            "minecraft:infested_cobblestone" => {
                Self::infested_cobblestone_from_identifier_and_properties(properties)
            }
            "minecraft:infested_stone_bricks" => {
                Self::infested_stone_bricks_from_identifier_and_properties(properties)
            }
            "minecraft:infested_mossy_stone_bricks" => {
                Self::infested_mossy_stone_bricks_from_identifier_and_properties(properties)
            }
            "minecraft:infested_cracked_stone_bricks" => {
                Self::infested_cracked_stone_bricks_from_identifier_and_properties(properties)
            }
            "minecraft:infested_chiseled_stone_bricks" => {
                Self::infested_chiseled_stone_bricks_from_identifier_and_properties(properties)
            }
            "minecraft:stone_bricks" => {
                Self::stone_bricks_from_identifier_and_properties(properties)
            }
            "minecraft:mossy_stone_bricks" => {
                Self::mossy_stone_bricks_from_identifier_and_properties(properties)
            }
            "minecraft:cracked_stone_bricks" => {
                Self::cracked_stone_bricks_from_identifier_and_properties(properties)
            }
            "minecraft:chiseled_stone_bricks" => {
                Self::chiseled_stone_bricks_from_identifier_and_properties(properties)
            }
            "minecraft:brown_mushroom_block" => {
                Self::brown_mushroom_block_from_identifier_and_properties(properties)
            }
            "minecraft:red_mushroom_block" => {
                Self::red_mushroom_block_from_identifier_and_properties(properties)
            }
            "minecraft:mushroom_stem" => {
                Self::mushroom_stem_from_identifier_and_properties(properties)
            }
            "minecraft:iron_bars" => Self::iron_bars_from_identifier_and_properties(properties),
            "minecraft:glass_pane" => Self::glass_pane_from_identifier_and_properties(properties),
            "minecraft:melon" => Self::melon_from_identifier_and_properties(properties),
            "minecraft:attached_pumpkin_stem" => {
                Self::attached_pumpkin_stem_from_identifier_and_properties(properties)
            }
            "minecraft:attached_melon_stem" => {
                Self::attached_melon_stem_from_identifier_and_properties(properties)
            }
            "minecraft:pumpkin_stem" => {
                Self::pumpkin_stem_from_identifier_and_properties(properties)
            }
            "minecraft:melon_stem" => Self::melon_stem_from_identifier_and_properties(properties),
            "minecraft:vine" => Self::vine_from_identifier_and_properties(properties),
            "minecraft:oak_fence_gate" => {
                Self::oak_fence_gate_from_identifier_and_properties(properties)
            }
            "minecraft:brick_stairs" => {
                Self::brick_stairs_from_identifier_and_properties(properties)
            }
            "minecraft:stone_brick_stairs" => {
                Self::stone_brick_stairs_from_identifier_and_properties(properties)
            }
            "minecraft:mycelium" => Self::mycelium_from_identifier_and_properties(properties),
            "minecraft:lily_pad" => Self::lily_pad_from_identifier_and_properties(properties),
            "minecraft:nether_bricks" => {
                Self::nether_bricks_from_identifier_and_properties(properties)
            }
            "minecraft:nether_brick_fence" => {
                Self::nether_brick_fence_from_identifier_and_properties(properties)
            }
            "minecraft:nether_brick_stairs" => {
                Self::nether_brick_stairs_from_identifier_and_properties(properties)
            }
            "minecraft:nether_wart" => Self::nether_wart_from_identifier_and_properties(properties),
            "minecraft:enchanting_table" => {
                Self::enchanting_table_from_identifier_and_properties(properties)
            }
            "minecraft:brewing_stand" => {
                Self::brewing_stand_from_identifier_and_properties(properties)
            }
            "minecraft:cauldron" => Self::cauldron_from_identifier_and_properties(properties),
            "minecraft:end_portal" => Self::end_portal_from_identifier_and_properties(properties),
            "minecraft:end_portal_frame" => {
                Self::end_portal_frame_from_identifier_and_properties(properties)
            }
            "minecraft:end_stone" => Self::end_stone_from_identifier_and_properties(properties),
            "minecraft:dragon_egg" => Self::dragon_egg_from_identifier_and_properties(properties),
            "minecraft:redstone_lamp" => {
                Self::redstone_lamp_from_identifier_and_properties(properties)
            }
            "minecraft:cocoa" => Self::cocoa_from_identifier_and_properties(properties),
            "minecraft:sandstone_stairs" => {
                Self::sandstone_stairs_from_identifier_and_properties(properties)
            }
            "minecraft:emerald_ore" => Self::emerald_ore_from_identifier_and_properties(properties),
            "minecraft:ender_chest" => Self::ender_chest_from_identifier_and_properties(properties),
            "minecraft:tripwire_hook" => {
                Self::tripwire_hook_from_identifier_and_properties(properties)
            }
            "minecraft:tripwire" => Self::tripwire_from_identifier_and_properties(properties),
            "minecraft:emerald_block" => {
                Self::emerald_block_from_identifier_and_properties(properties)
            }
            "minecraft:spruce_stairs" => {
                Self::spruce_stairs_from_identifier_and_properties(properties)
            }
            "minecraft:birch_stairs" => {
                Self::birch_stairs_from_identifier_and_properties(properties)
            }
            "minecraft:jungle_stairs" => {
                Self::jungle_stairs_from_identifier_and_properties(properties)
            }
            "minecraft:command_block" => {
                Self::command_block_from_identifier_and_properties(properties)
            }
            "minecraft:beacon" => Self::beacon_from_identifier_and_properties(properties),
            "minecraft:cobblestone_wall" => {
                Self::cobblestone_wall_from_identifier_and_properties(properties)
            }
            "minecraft:mossy_cobblestone_wall" => {
                Self::mossy_cobblestone_wall_from_identifier_and_properties(properties)
            }
            "minecraft:flower_pot" => Self::flower_pot_from_identifier_and_properties(properties),
            "minecraft:potted_oak_sapling" => {
                Self::potted_oak_sapling_from_identifier_and_properties(properties)
            }
            "minecraft:potted_spruce_sapling" => {
                Self::potted_spruce_sapling_from_identifier_and_properties(properties)
            }
            "minecraft:potted_birch_sapling" => {
                Self::potted_birch_sapling_from_identifier_and_properties(properties)
            }
            "minecraft:potted_jungle_sapling" => {
                Self::potted_jungle_sapling_from_identifier_and_properties(properties)
            }
            "minecraft:potted_acacia_sapling" => {
                Self::potted_acacia_sapling_from_identifier_and_properties(properties)
            }
            "minecraft:potted_dark_oak_sapling" => {
                Self::potted_dark_oak_sapling_from_identifier_and_properties(properties)
            }
            "minecraft:potted_fern" => Self::potted_fern_from_identifier_and_properties(properties),
            "minecraft:potted_dandelion" => {
                Self::potted_dandelion_from_identifier_and_properties(properties)
            }
            "minecraft:potted_poppy" => {
                Self::potted_poppy_from_identifier_and_properties(properties)
            }
            "minecraft:potted_blue_orchid" => {
                Self::potted_blue_orchid_from_identifier_and_properties(properties)
            }
            "minecraft:potted_allium" => {
                Self::potted_allium_from_identifier_and_properties(properties)
            }
            "minecraft:potted_azure_bluet" => {
                Self::potted_azure_bluet_from_identifier_and_properties(properties)
            }
            "minecraft:potted_red_tulip" => {
                Self::potted_red_tulip_from_identifier_and_properties(properties)
            }
            "minecraft:potted_orange_tulip" => {
                Self::potted_orange_tulip_from_identifier_and_properties(properties)
            }
            "minecraft:potted_white_tulip" => {
                Self::potted_white_tulip_from_identifier_and_properties(properties)
            }
            "minecraft:potted_pink_tulip" => {
                Self::potted_pink_tulip_from_identifier_and_properties(properties)
            }
            "minecraft:potted_oxeye_daisy" => {
                Self::potted_oxeye_daisy_from_identifier_and_properties(properties)
            }
            "minecraft:potted_red_mushroom" => {
                Self::potted_red_mushroom_from_identifier_and_properties(properties)
            }
            "minecraft:potted_brown_mushroom" => {
                Self::potted_brown_mushroom_from_identifier_and_properties(properties)
            }
            "minecraft:potted_dead_bush" => {
                Self::potted_dead_bush_from_identifier_and_properties(properties)
            }
            "minecraft:potted_cactus" => {
                Self::potted_cactus_from_identifier_and_properties(properties)
            }
            "minecraft:carrots" => Self::carrots_from_identifier_and_properties(properties),
            "minecraft:potatoes" => Self::potatoes_from_identifier_and_properties(properties),
            "minecraft:oak_button" => Self::oak_button_from_identifier_and_properties(properties),
            "minecraft:spruce_button" => {
                Self::spruce_button_from_identifier_and_properties(properties)
            }
            "minecraft:birch_button" => {
                Self::birch_button_from_identifier_and_properties(properties)
            }
            "minecraft:jungle_button" => {
                Self::jungle_button_from_identifier_and_properties(properties)
            }
            "minecraft:acacia_button" => {
                Self::acacia_button_from_identifier_and_properties(properties)
            }
            "minecraft:dark_oak_button" => {
                Self::dark_oak_button_from_identifier_and_properties(properties)
            }
            "minecraft:skeleton_wall_skull" => {
                Self::skeleton_wall_skull_from_identifier_and_properties(properties)
            }
            "minecraft:skeleton_skull" => {
                Self::skeleton_skull_from_identifier_and_properties(properties)
            }
            "minecraft:wither_skeleton_wall_skull" => {
                Self::wither_skeleton_wall_skull_from_identifier_and_properties(properties)
            }
            "minecraft:wither_skeleton_skull" => {
                Self::wither_skeleton_skull_from_identifier_and_properties(properties)
            }
            "minecraft:zombie_wall_head" => {
                Self::zombie_wall_head_from_identifier_and_properties(properties)
            }
            "minecraft:zombie_head" => Self::zombie_head_from_identifier_and_properties(properties),
            "minecraft:player_wall_head" => {
                Self::player_wall_head_from_identifier_and_properties(properties)
            }
            "minecraft:player_head" => Self::player_head_from_identifier_and_properties(properties),
            "minecraft:creeper_wall_head" => {
                Self::creeper_wall_head_from_identifier_and_properties(properties)
            }
            "minecraft:creeper_head" => {
                Self::creeper_head_from_identifier_and_properties(properties)
            }
            "minecraft:dragon_wall_head" => {
                Self::dragon_wall_head_from_identifier_and_properties(properties)
            }
            "minecraft:dragon_head" => Self::dragon_head_from_identifier_and_properties(properties),
            "minecraft:anvil" => Self::anvil_from_identifier_and_properties(properties),
            "minecraft:chipped_anvil" => {
                Self::chipped_anvil_from_identifier_and_properties(properties)
            }
            "minecraft:damaged_anvil" => {
                Self::damaged_anvil_from_identifier_and_properties(properties)
            }
            "minecraft:trapped_chest" => {
                Self::trapped_chest_from_identifier_and_properties(properties)
            }
            "minecraft:light_weighted_pressure_plate" => {
                Self::light_weighted_pressure_plate_from_identifier_and_properties(properties)
            }
            "minecraft:heavy_weighted_pressure_plate" => {
                Self::heavy_weighted_pressure_plate_from_identifier_and_properties(properties)
            }
            "minecraft:comparator" => Self::comparator_from_identifier_and_properties(properties),
            "minecraft:daylight_detector" => {
                Self::daylight_detector_from_identifier_and_properties(properties)
            }
            "minecraft:redstone_block" => {
                Self::redstone_block_from_identifier_and_properties(properties)
            }
            "minecraft:nether_quartz_ore" => {
                Self::nether_quartz_ore_from_identifier_and_properties(properties)
            }
            "minecraft:hopper" => Self::hopper_from_identifier_and_properties(properties),
            "minecraft:quartz_block" => {
                Self::quartz_block_from_identifier_and_properties(properties)
            }
            "minecraft:chiseled_quartz_block" => {
                Self::chiseled_quartz_block_from_identifier_and_properties(properties)
            }
            "minecraft:quartz_pillar" => {
                Self::quartz_pillar_from_identifier_and_properties(properties)
            }
            "minecraft:quartz_stairs" => {
                Self::quartz_stairs_from_identifier_and_properties(properties)
            }
            "minecraft:activator_rail" => {
                Self::activator_rail_from_identifier_and_properties(properties)
            }
            "minecraft:dropper" => Self::dropper_from_identifier_and_properties(properties),
            "minecraft:white_terracotta" => {
                Self::white_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:orange_terracotta" => {
                Self::orange_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:magenta_terracotta" => {
                Self::magenta_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:light_blue_terracotta" => {
                Self::light_blue_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:yellow_terracotta" => {
                Self::yellow_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:lime_terracotta" => {
                Self::lime_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:pink_terracotta" => {
                Self::pink_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:gray_terracotta" => {
                Self::gray_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:light_gray_terracotta" => {
                Self::light_gray_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:cyan_terracotta" => {
                Self::cyan_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:purple_terracotta" => {
                Self::purple_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:blue_terracotta" => {
                Self::blue_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:brown_terracotta" => {
                Self::brown_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:green_terracotta" => {
                Self::green_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:red_terracotta" => {
                Self::red_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:black_terracotta" => {
                Self::black_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:white_stained_glass_pane" => {
                Self::white_stained_glass_pane_from_identifier_and_properties(properties)
            }
            "minecraft:orange_stained_glass_pane" => {
                Self::orange_stained_glass_pane_from_identifier_and_properties(properties)
            }
            "minecraft:magenta_stained_glass_pane" => {
                Self::magenta_stained_glass_pane_from_identifier_and_properties(properties)
            }
            "minecraft:light_blue_stained_glass_pane" => {
                Self::light_blue_stained_glass_pane_from_identifier_and_properties(properties)
            }
            "minecraft:yellow_stained_glass_pane" => {
                Self::yellow_stained_glass_pane_from_identifier_and_properties(properties)
            }
            "minecraft:lime_stained_glass_pane" => {
                Self::lime_stained_glass_pane_from_identifier_and_properties(properties)
            }
            "minecraft:pink_stained_glass_pane" => {
                Self::pink_stained_glass_pane_from_identifier_and_properties(properties)
            }
            "minecraft:gray_stained_glass_pane" => {
                Self::gray_stained_glass_pane_from_identifier_and_properties(properties)
            }
            "minecraft:light_gray_stained_glass_pane" => {
                Self::light_gray_stained_glass_pane_from_identifier_and_properties(properties)
            }
            "minecraft:cyan_stained_glass_pane" => {
                Self::cyan_stained_glass_pane_from_identifier_and_properties(properties)
            }
            "minecraft:purple_stained_glass_pane" => {
                Self::purple_stained_glass_pane_from_identifier_and_properties(properties)
            }
            "minecraft:blue_stained_glass_pane" => {
                Self::blue_stained_glass_pane_from_identifier_and_properties(properties)
            }
            "minecraft:brown_stained_glass_pane" => {
                Self::brown_stained_glass_pane_from_identifier_and_properties(properties)
            }
            "minecraft:green_stained_glass_pane" => {
                Self::green_stained_glass_pane_from_identifier_and_properties(properties)
            }
            "minecraft:red_stained_glass_pane" => {
                Self::red_stained_glass_pane_from_identifier_and_properties(properties)
            }
            "minecraft:black_stained_glass_pane" => {
                Self::black_stained_glass_pane_from_identifier_and_properties(properties)
            }
            "minecraft:acacia_stairs" => {
                Self::acacia_stairs_from_identifier_and_properties(properties)
            }
            "minecraft:dark_oak_stairs" => {
                Self::dark_oak_stairs_from_identifier_and_properties(properties)
            }
            "minecraft:slime_block" => Self::slime_block_from_identifier_and_properties(properties),
            "minecraft:barrier" => Self::barrier_from_identifier_and_properties(properties),
            "minecraft:iron_trapdoor" => {
                Self::iron_trapdoor_from_identifier_and_properties(properties)
            }
            "minecraft:prismarine" => Self::prismarine_from_identifier_and_properties(properties),
            "minecraft:prismarine_bricks" => {
                Self::prismarine_bricks_from_identifier_and_properties(properties)
            }
            "minecraft:dark_prismarine" => {
                Self::dark_prismarine_from_identifier_and_properties(properties)
            }
            "minecraft:prismarine_stairs" => {
                Self::prismarine_stairs_from_identifier_and_properties(properties)
            }
            "minecraft:prismarine_brick_stairs" => {
                Self::prismarine_brick_stairs_from_identifier_and_properties(properties)
            }
            "minecraft:dark_prismarine_stairs" => {
                Self::dark_prismarine_stairs_from_identifier_and_properties(properties)
            }
            "minecraft:prismarine_slab" => {
                Self::prismarine_slab_from_identifier_and_properties(properties)
            }
            "minecraft:prismarine_brick_slab" => {
                Self::prismarine_brick_slab_from_identifier_and_properties(properties)
            }
            "minecraft:dark_prismarine_slab" => {
                Self::dark_prismarine_slab_from_identifier_and_properties(properties)
            }
            "minecraft:sea_lantern" => Self::sea_lantern_from_identifier_and_properties(properties),
            "minecraft:hay_block" => Self::hay_block_from_identifier_and_properties(properties),
            "minecraft:white_carpet" => {
                Self::white_carpet_from_identifier_and_properties(properties)
            }
            "minecraft:orange_carpet" => {
                Self::orange_carpet_from_identifier_and_properties(properties)
            }
            "minecraft:magenta_carpet" => {
                Self::magenta_carpet_from_identifier_and_properties(properties)
            }
            "minecraft:light_blue_carpet" => {
                Self::light_blue_carpet_from_identifier_and_properties(properties)
            }
            "minecraft:yellow_carpet" => {
                Self::yellow_carpet_from_identifier_and_properties(properties)
            }
            "minecraft:lime_carpet" => Self::lime_carpet_from_identifier_and_properties(properties),
            "minecraft:pink_carpet" => Self::pink_carpet_from_identifier_and_properties(properties),
            "minecraft:gray_carpet" => Self::gray_carpet_from_identifier_and_properties(properties),
            "minecraft:light_gray_carpet" => {
                Self::light_gray_carpet_from_identifier_and_properties(properties)
            }
            "minecraft:cyan_carpet" => Self::cyan_carpet_from_identifier_and_properties(properties),
            "minecraft:purple_carpet" => {
                Self::purple_carpet_from_identifier_and_properties(properties)
            }
            "minecraft:blue_carpet" => Self::blue_carpet_from_identifier_and_properties(properties),
            "minecraft:brown_carpet" => {
                Self::brown_carpet_from_identifier_and_properties(properties)
            }
            "minecraft:green_carpet" => {
                Self::green_carpet_from_identifier_and_properties(properties)
            }
            "minecraft:red_carpet" => Self::red_carpet_from_identifier_and_properties(properties),
            "minecraft:black_carpet" => {
                Self::black_carpet_from_identifier_and_properties(properties)
            }
            "minecraft:terracotta" => Self::terracotta_from_identifier_and_properties(properties),
            "minecraft:coal_block" => Self::coal_block_from_identifier_and_properties(properties),
            "minecraft:packed_ice" => Self::packed_ice_from_identifier_and_properties(properties),
            "minecraft:sunflower" => Self::sunflower_from_identifier_and_properties(properties),
            "minecraft:lilac" => Self::lilac_from_identifier_and_properties(properties),
            "minecraft:rose_bush" => Self::rose_bush_from_identifier_and_properties(properties),
            "minecraft:peony" => Self::peony_from_identifier_and_properties(properties),
            "minecraft:tall_grass" => Self::tall_grass_from_identifier_and_properties(properties),
            "minecraft:large_fern" => Self::large_fern_from_identifier_and_properties(properties),
            "minecraft:white_banner" => {
                Self::white_banner_from_identifier_and_properties(properties)
            }
            "minecraft:orange_banner" => {
                Self::orange_banner_from_identifier_and_properties(properties)
            }
            "minecraft:magenta_banner" => {
                Self::magenta_banner_from_identifier_and_properties(properties)
            }
            "minecraft:light_blue_banner" => {
                Self::light_blue_banner_from_identifier_and_properties(properties)
            }
            "minecraft:yellow_banner" => {
                Self::yellow_banner_from_identifier_and_properties(properties)
            }
            "minecraft:lime_banner" => Self::lime_banner_from_identifier_and_properties(properties),
            "minecraft:pink_banner" => Self::pink_banner_from_identifier_and_properties(properties),
            "minecraft:gray_banner" => Self::gray_banner_from_identifier_and_properties(properties),
            "minecraft:light_gray_banner" => {
                Self::light_gray_banner_from_identifier_and_properties(properties)
            }
            "minecraft:cyan_banner" => Self::cyan_banner_from_identifier_and_properties(properties),
            "minecraft:purple_banner" => {
                Self::purple_banner_from_identifier_and_properties(properties)
            }
            "minecraft:blue_banner" => Self::blue_banner_from_identifier_and_properties(properties),
            "minecraft:brown_banner" => {
                Self::brown_banner_from_identifier_and_properties(properties)
            }
            "minecraft:green_banner" => {
                Self::green_banner_from_identifier_and_properties(properties)
            }
            "minecraft:red_banner" => Self::red_banner_from_identifier_and_properties(properties),
            "minecraft:black_banner" => {
                Self::black_banner_from_identifier_and_properties(properties)
            }
            "minecraft:white_wall_banner" => {
                Self::white_wall_banner_from_identifier_and_properties(properties)
            }
            "minecraft:orange_wall_banner" => {
                Self::orange_wall_banner_from_identifier_and_properties(properties)
            }
            "minecraft:magenta_wall_banner" => {
                Self::magenta_wall_banner_from_identifier_and_properties(properties)
            }
            "minecraft:light_blue_wall_banner" => {
                Self::light_blue_wall_banner_from_identifier_and_properties(properties)
            }
            "minecraft:yellow_wall_banner" => {
                Self::yellow_wall_banner_from_identifier_and_properties(properties)
            }
            "minecraft:lime_wall_banner" => {
                Self::lime_wall_banner_from_identifier_and_properties(properties)
            }
            "minecraft:pink_wall_banner" => {
                Self::pink_wall_banner_from_identifier_and_properties(properties)
            }
            "minecraft:gray_wall_banner" => {
                Self::gray_wall_banner_from_identifier_and_properties(properties)
            }
            "minecraft:light_gray_wall_banner" => {
                Self::light_gray_wall_banner_from_identifier_and_properties(properties)
            }
            "minecraft:cyan_wall_banner" => {
                Self::cyan_wall_banner_from_identifier_and_properties(properties)
            }
            "minecraft:purple_wall_banner" => {
                Self::purple_wall_banner_from_identifier_and_properties(properties)
            }
            "minecraft:blue_wall_banner" => {
                Self::blue_wall_banner_from_identifier_and_properties(properties)
            }
            "minecraft:brown_wall_banner" => {
                Self::brown_wall_banner_from_identifier_and_properties(properties)
            }
            "minecraft:green_wall_banner" => {
                Self::green_wall_banner_from_identifier_and_properties(properties)
            }
            "minecraft:red_wall_banner" => {
                Self::red_wall_banner_from_identifier_and_properties(properties)
            }
            "minecraft:black_wall_banner" => {
                Self::black_wall_banner_from_identifier_and_properties(properties)
            }
            "minecraft:red_sandstone" => {
                Self::red_sandstone_from_identifier_and_properties(properties)
            }
            "minecraft:chiseled_red_sandstone" => {
                Self::chiseled_red_sandstone_from_identifier_and_properties(properties)
            }
            "minecraft:cut_red_sandstone" => {
                Self::cut_red_sandstone_from_identifier_and_properties(properties)
            }
            "minecraft:red_sandstone_stairs" => {
                Self::red_sandstone_stairs_from_identifier_and_properties(properties)
            }
            "minecraft:oak_slab" => Self::oak_slab_from_identifier_and_properties(properties),
            "minecraft:spruce_slab" => Self::spruce_slab_from_identifier_and_properties(properties),
            "minecraft:birch_slab" => Self::birch_slab_from_identifier_and_properties(properties),
            "minecraft:jungle_slab" => Self::jungle_slab_from_identifier_and_properties(properties),
            "minecraft:acacia_slab" => Self::acacia_slab_from_identifier_and_properties(properties),
            "minecraft:dark_oak_slab" => {
                Self::dark_oak_slab_from_identifier_and_properties(properties)
            }
            "minecraft:stone_slab" => Self::stone_slab_from_identifier_and_properties(properties),
            "minecraft:sandstone_slab" => {
                Self::sandstone_slab_from_identifier_and_properties(properties)
            }
            "minecraft:petrified_oak_slab" => {
                Self::petrified_oak_slab_from_identifier_and_properties(properties)
            }
            "minecraft:cobblestone_slab" => {
                Self::cobblestone_slab_from_identifier_and_properties(properties)
            }
            "minecraft:brick_slab" => Self::brick_slab_from_identifier_and_properties(properties),
            "minecraft:stone_brick_slab" => {
                Self::stone_brick_slab_from_identifier_and_properties(properties)
            }
            "minecraft:nether_brick_slab" => {
                Self::nether_brick_slab_from_identifier_and_properties(properties)
            }
            "minecraft:quartz_slab" => Self::quartz_slab_from_identifier_and_properties(properties),
            "minecraft:red_sandstone_slab" => {
                Self::red_sandstone_slab_from_identifier_and_properties(properties)
            }
            "minecraft:purpur_slab" => Self::purpur_slab_from_identifier_and_properties(properties),
            "minecraft:smooth_stone" => {
                Self::smooth_stone_from_identifier_and_properties(properties)
            }
            "minecraft:smooth_sandstone" => {
                Self::smooth_sandstone_from_identifier_and_properties(properties)
            }
            "minecraft:smooth_quartz" => {
                Self::smooth_quartz_from_identifier_and_properties(properties)
            }
            "minecraft:smooth_red_sandstone" => {
                Self::smooth_red_sandstone_from_identifier_and_properties(properties)
            }
            "minecraft:spruce_fence_gate" => {
                Self::spruce_fence_gate_from_identifier_and_properties(properties)
            }
            "minecraft:birch_fence_gate" => {
                Self::birch_fence_gate_from_identifier_and_properties(properties)
            }
            "minecraft:jungle_fence_gate" => {
                Self::jungle_fence_gate_from_identifier_and_properties(properties)
            }
            "minecraft:acacia_fence_gate" => {
                Self::acacia_fence_gate_from_identifier_and_properties(properties)
            }
            "minecraft:dark_oak_fence_gate" => {
                Self::dark_oak_fence_gate_from_identifier_and_properties(properties)
            }
            "minecraft:spruce_fence" => {
                Self::spruce_fence_from_identifier_and_properties(properties)
            }
            "minecraft:birch_fence" => Self::birch_fence_from_identifier_and_properties(properties),
            "minecraft:jungle_fence" => {
                Self::jungle_fence_from_identifier_and_properties(properties)
            }
            "minecraft:acacia_fence" => {
                Self::acacia_fence_from_identifier_and_properties(properties)
            }
            "minecraft:dark_oak_fence" => {
                Self::dark_oak_fence_from_identifier_and_properties(properties)
            }
            "minecraft:spruce_door" => Self::spruce_door_from_identifier_and_properties(properties),
            "minecraft:birch_door" => Self::birch_door_from_identifier_and_properties(properties),
            "minecraft:jungle_door" => Self::jungle_door_from_identifier_and_properties(properties),
            "minecraft:acacia_door" => Self::acacia_door_from_identifier_and_properties(properties),
            "minecraft:dark_oak_door" => {
                Self::dark_oak_door_from_identifier_and_properties(properties)
            }
            "minecraft:end_rod" => Self::end_rod_from_identifier_and_properties(properties),
            "minecraft:chorus_plant" => {
                Self::chorus_plant_from_identifier_and_properties(properties)
            }
            "minecraft:chorus_flower" => {
                Self::chorus_flower_from_identifier_and_properties(properties)
            }
            "minecraft:purpur_block" => {
                Self::purpur_block_from_identifier_and_properties(properties)
            }
            "minecraft:purpur_pillar" => {
                Self::purpur_pillar_from_identifier_and_properties(properties)
            }
            "minecraft:purpur_stairs" => {
                Self::purpur_stairs_from_identifier_and_properties(properties)
            }
            "minecraft:end_stone_bricks" => {
                Self::end_stone_bricks_from_identifier_and_properties(properties)
            }
            "minecraft:beetroots" => Self::beetroots_from_identifier_and_properties(properties),
            "minecraft:grass_path" => Self::grass_path_from_identifier_and_properties(properties),
            "minecraft:end_gateway" => Self::end_gateway_from_identifier_and_properties(properties),
            "minecraft:repeating_command_block" => {
                Self::repeating_command_block_from_identifier_and_properties(properties)
            }
            "minecraft:chain_command_block" => {
                Self::chain_command_block_from_identifier_and_properties(properties)
            }
            "minecraft:frosted_ice" => Self::frosted_ice_from_identifier_and_properties(properties),
            "minecraft:magma_block" => Self::magma_block_from_identifier_and_properties(properties),
            "minecraft:nether_wart_block" => {
                Self::nether_wart_block_from_identifier_and_properties(properties)
            }
            "minecraft:red_nether_bricks" => {
                Self::red_nether_bricks_from_identifier_and_properties(properties)
            }
            "minecraft:bone_block" => Self::bone_block_from_identifier_and_properties(properties),
            "minecraft:structure_void" => {
                Self::structure_void_from_identifier_and_properties(properties)
            }
            "minecraft:observer" => Self::observer_from_identifier_and_properties(properties),
            "minecraft:shulker_box" => Self::shulker_box_from_identifier_and_properties(properties),
            "minecraft:white_shulker_box" => {
                Self::white_shulker_box_from_identifier_and_properties(properties)
            }
            "minecraft:orange_shulker_box" => {
                Self::orange_shulker_box_from_identifier_and_properties(properties)
            }
            "minecraft:magenta_shulker_box" => {
                Self::magenta_shulker_box_from_identifier_and_properties(properties)
            }
            "minecraft:light_blue_shulker_box" => {
                Self::light_blue_shulker_box_from_identifier_and_properties(properties)
            }
            "minecraft:yellow_shulker_box" => {
                Self::yellow_shulker_box_from_identifier_and_properties(properties)
            }
            "minecraft:lime_shulker_box" => {
                Self::lime_shulker_box_from_identifier_and_properties(properties)
            }
            "minecraft:pink_shulker_box" => {
                Self::pink_shulker_box_from_identifier_and_properties(properties)
            }
            "minecraft:gray_shulker_box" => {
                Self::gray_shulker_box_from_identifier_and_properties(properties)
            }
            "minecraft:light_gray_shulker_box" => {
                Self::light_gray_shulker_box_from_identifier_and_properties(properties)
            }
            "minecraft:cyan_shulker_box" => {
                Self::cyan_shulker_box_from_identifier_and_properties(properties)
            }
            "minecraft:purple_shulker_box" => {
                Self::purple_shulker_box_from_identifier_and_properties(properties)
            }
            "minecraft:blue_shulker_box" => {
                Self::blue_shulker_box_from_identifier_and_properties(properties)
            }
            "minecraft:brown_shulker_box" => {
                Self::brown_shulker_box_from_identifier_and_properties(properties)
            }
            "minecraft:green_shulker_box" => {
                Self::green_shulker_box_from_identifier_and_properties(properties)
            }
            "minecraft:red_shulker_box" => {
                Self::red_shulker_box_from_identifier_and_properties(properties)
            }
            "minecraft:black_shulker_box" => {
                Self::black_shulker_box_from_identifier_and_properties(properties)
            }
            "minecraft:white_glazed_terracotta" => {
                Self::white_glazed_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:orange_glazed_terracotta" => {
                Self::orange_glazed_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:magenta_glazed_terracotta" => {
                Self::magenta_glazed_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:light_blue_glazed_terracotta" => {
                Self::light_blue_glazed_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:yellow_glazed_terracotta" => {
                Self::yellow_glazed_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:lime_glazed_terracotta" => {
                Self::lime_glazed_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:pink_glazed_terracotta" => {
                Self::pink_glazed_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:gray_glazed_terracotta" => {
                Self::gray_glazed_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:light_gray_glazed_terracotta" => {
                Self::light_gray_glazed_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:cyan_glazed_terracotta" => {
                Self::cyan_glazed_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:purple_glazed_terracotta" => {
                Self::purple_glazed_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:blue_glazed_terracotta" => {
                Self::blue_glazed_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:brown_glazed_terracotta" => {
                Self::brown_glazed_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:green_glazed_terracotta" => {
                Self::green_glazed_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:red_glazed_terracotta" => {
                Self::red_glazed_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:black_glazed_terracotta" => {
                Self::black_glazed_terracotta_from_identifier_and_properties(properties)
            }
            "minecraft:white_concrete" => {
                Self::white_concrete_from_identifier_and_properties(properties)
            }
            "minecraft:orange_concrete" => {
                Self::orange_concrete_from_identifier_and_properties(properties)
            }
            "minecraft:magenta_concrete" => {
                Self::magenta_concrete_from_identifier_and_properties(properties)
            }
            "minecraft:light_blue_concrete" => {
                Self::light_blue_concrete_from_identifier_and_properties(properties)
            }
            "minecraft:yellow_concrete" => {
                Self::yellow_concrete_from_identifier_and_properties(properties)
            }
            "minecraft:lime_concrete" => {
                Self::lime_concrete_from_identifier_and_properties(properties)
            }
            "minecraft:pink_concrete" => {
                Self::pink_concrete_from_identifier_and_properties(properties)
            }
            "minecraft:gray_concrete" => {
                Self::gray_concrete_from_identifier_and_properties(properties)
            }
            "minecraft:light_gray_concrete" => {
                Self::light_gray_concrete_from_identifier_and_properties(properties)
            }
            "minecraft:cyan_concrete" => {
                Self::cyan_concrete_from_identifier_and_properties(properties)
            }
            "minecraft:purple_concrete" => {
                Self::purple_concrete_from_identifier_and_properties(properties)
            }
            "minecraft:blue_concrete" => {
                Self::blue_concrete_from_identifier_and_properties(properties)
            }
            "minecraft:brown_concrete" => {
                Self::brown_concrete_from_identifier_and_properties(properties)
            }
            "minecraft:green_concrete" => {
                Self::green_concrete_from_identifier_and_properties(properties)
            }
            "minecraft:red_concrete" => {
                Self::red_concrete_from_identifier_and_properties(properties)
            }
            "minecraft:black_concrete" => {
                Self::black_concrete_from_identifier_and_properties(properties)
            }
            "minecraft:white_concrete_powder" => {
                Self::white_concrete_powder_from_identifier_and_properties(properties)
            }
            "minecraft:orange_concrete_powder" => {
                Self::orange_concrete_powder_from_identifier_and_properties(properties)
            }
            "minecraft:magenta_concrete_powder" => {
                Self::magenta_concrete_powder_from_identifier_and_properties(properties)
            }
            "minecraft:light_blue_concrete_powder" => {
                Self::light_blue_concrete_powder_from_identifier_and_properties(properties)
            }
            "minecraft:yellow_concrete_powder" => {
                Self::yellow_concrete_powder_from_identifier_and_properties(properties)
            }
            "minecraft:lime_concrete_powder" => {
                Self::lime_concrete_powder_from_identifier_and_properties(properties)
            }
            "minecraft:pink_concrete_powder" => {
                Self::pink_concrete_powder_from_identifier_and_properties(properties)
            }
            "minecraft:gray_concrete_powder" => {
                Self::gray_concrete_powder_from_identifier_and_properties(properties)
            }
            "minecraft:light_gray_concrete_powder" => {
                Self::light_gray_concrete_powder_from_identifier_and_properties(properties)
            }
            "minecraft:cyan_concrete_powder" => {
                Self::cyan_concrete_powder_from_identifier_and_properties(properties)
            }
            "minecraft:purple_concrete_powder" => {
                Self::purple_concrete_powder_from_identifier_and_properties(properties)
            }
            "minecraft:blue_concrete_powder" => {
                Self::blue_concrete_powder_from_identifier_and_properties(properties)
            }
            "minecraft:brown_concrete_powder" => {
                Self::brown_concrete_powder_from_identifier_and_properties(properties)
            }
            "minecraft:green_concrete_powder" => {
                Self::green_concrete_powder_from_identifier_and_properties(properties)
            }
            "minecraft:red_concrete_powder" => {
                Self::red_concrete_powder_from_identifier_and_properties(properties)
            }
            "minecraft:black_concrete_powder" => {
                Self::black_concrete_powder_from_identifier_and_properties(properties)
            }
            "minecraft:kelp" => Self::kelp_from_identifier_and_properties(properties),
            "minecraft:kelp_plant" => Self::kelp_plant_from_identifier_and_properties(properties),
            "minecraft:dried_kelp_block" => {
                Self::dried_kelp_block_from_identifier_and_properties(properties)
            }
            "minecraft:turtle_egg" => Self::turtle_egg_from_identifier_and_properties(properties),
            "minecraft:dead_tube_coral_block" => {
                Self::dead_tube_coral_block_from_identifier_and_properties(properties)
            }
            "minecraft:dead_brain_coral_block" => {
                Self::dead_brain_coral_block_from_identifier_and_properties(properties)
            }
            "minecraft:dead_bubble_coral_block" => {
                Self::dead_bubble_coral_block_from_identifier_and_properties(properties)
            }
            "minecraft:dead_fire_coral_block" => {
                Self::dead_fire_coral_block_from_identifier_and_properties(properties)
            }
            "minecraft:dead_horn_coral_block" => {
                Self::dead_horn_coral_block_from_identifier_and_properties(properties)
            }
            "minecraft:tube_coral_block" => {
                Self::tube_coral_block_from_identifier_and_properties(properties)
            }
            "minecraft:brain_coral_block" => {
                Self::brain_coral_block_from_identifier_and_properties(properties)
            }
            "minecraft:bubble_coral_block" => {
                Self::bubble_coral_block_from_identifier_and_properties(properties)
            }
            "minecraft:fire_coral_block" => {
                Self::fire_coral_block_from_identifier_and_properties(properties)
            }
            "minecraft:horn_coral_block" => {
                Self::horn_coral_block_from_identifier_and_properties(properties)
            }
            "minecraft:dead_tube_coral" => {
                Self::dead_tube_coral_from_identifier_and_properties(properties)
            }
            "minecraft:dead_brain_coral" => {
                Self::dead_brain_coral_from_identifier_and_properties(properties)
            }
            "minecraft:dead_bubble_coral" => {
                Self::dead_bubble_coral_from_identifier_and_properties(properties)
            }
            "minecraft:dead_fire_coral" => {
                Self::dead_fire_coral_from_identifier_and_properties(properties)
            }
            "minecraft:dead_horn_coral" => {
                Self::dead_horn_coral_from_identifier_and_properties(properties)
            }
            "minecraft:tube_coral" => Self::tube_coral_from_identifier_and_properties(properties),
            "minecraft:brain_coral" => Self::brain_coral_from_identifier_and_properties(properties),
            "minecraft:bubble_coral" => {
                Self::bubble_coral_from_identifier_and_properties(properties)
            }
            "minecraft:fire_coral" => Self::fire_coral_from_identifier_and_properties(properties),
            "minecraft:horn_coral" => Self::horn_coral_from_identifier_and_properties(properties),
            "minecraft:dead_tube_coral_wall_fan" => {
                Self::dead_tube_coral_wall_fan_from_identifier_and_properties(properties)
            }
            "minecraft:dead_brain_coral_wall_fan" => {
                Self::dead_brain_coral_wall_fan_from_identifier_and_properties(properties)
            }
            "minecraft:dead_bubble_coral_wall_fan" => {
                Self::dead_bubble_coral_wall_fan_from_identifier_and_properties(properties)
            }
            "minecraft:dead_fire_coral_wall_fan" => {
                Self::dead_fire_coral_wall_fan_from_identifier_and_properties(properties)
            }
            "minecraft:dead_horn_coral_wall_fan" => {
                Self::dead_horn_coral_wall_fan_from_identifier_and_properties(properties)
            }
            "minecraft:tube_coral_wall_fan" => {
                Self::tube_coral_wall_fan_from_identifier_and_properties(properties)
            }
            "minecraft:brain_coral_wall_fan" => {
                Self::brain_coral_wall_fan_from_identifier_and_properties(properties)
            }
            "minecraft:bubble_coral_wall_fan" => {
                Self::bubble_coral_wall_fan_from_identifier_and_properties(properties)
            }
            "minecraft:fire_coral_wall_fan" => {
                Self::fire_coral_wall_fan_from_identifier_and_properties(properties)
            }
            "minecraft:horn_coral_wall_fan" => {
                Self::horn_coral_wall_fan_from_identifier_and_properties(properties)
            }
            "minecraft:dead_tube_coral_fan" => {
                Self::dead_tube_coral_fan_from_identifier_and_properties(properties)
            }
            "minecraft:dead_brain_coral_fan" => {
                Self::dead_brain_coral_fan_from_identifier_and_properties(properties)
            }
            "minecraft:dead_bubble_coral_fan" => {
                Self::dead_bubble_coral_fan_from_identifier_and_properties(properties)
            }
            "minecraft:dead_fire_coral_fan" => {
                Self::dead_fire_coral_fan_from_identifier_and_properties(properties)
            }
            "minecraft:dead_horn_coral_fan" => {
                Self::dead_horn_coral_fan_from_identifier_and_properties(properties)
            }
            "minecraft:tube_coral_fan" => {
                Self::tube_coral_fan_from_identifier_and_properties(properties)
            }
            "minecraft:brain_coral_fan" => {
                Self::brain_coral_fan_from_identifier_and_properties(properties)
            }
            "minecraft:bubble_coral_fan" => {
                Self::bubble_coral_fan_from_identifier_and_properties(properties)
            }
            "minecraft:fire_coral_fan" => {
                Self::fire_coral_fan_from_identifier_and_properties(properties)
            }
            "minecraft:horn_coral_fan" => {
                Self::horn_coral_fan_from_identifier_and_properties(properties)
            }
            "minecraft:sea_pickle" => Self::sea_pickle_from_identifier_and_properties(properties),
            "minecraft:blue_ice" => Self::blue_ice_from_identifier_and_properties(properties),
            "minecraft:conduit" => Self::conduit_from_identifier_and_properties(properties),
            "minecraft:void_air" => Self::void_air_from_identifier_and_properties(properties),
            "minecraft:cave_air" => Self::cave_air_from_identifier_and_properties(properties),
            "minecraft:bubble_column" => {
                Self::bubble_column_from_identifier_and_properties(properties)
            }
            "minecraft:structure_block" => {
                Self::structure_block_from_identifier_and_properties(properties)
            }
            _ => None,
        }
    }
    fn air_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::air();
        Some(block)
    }
    fn stone_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::stone();
        Some(block)
    }
    fn granite_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::granite();
        Some(block)
    }
    fn polished_granite_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::polished_granite();
        Some(block)
    }
    fn diorite_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::diorite();
        Some(block)
    }
    fn polished_diorite_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::polished_diorite();
        Some(block)
    }
    fn andesite_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::andesite();
        Some(block)
    }
    fn polished_andesite_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::polished_andesite();
        Some(block)
    }
    fn grass_block_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::grass_block();
        let snowy = map.get("snowy")?;
        let snowy = bool::from_str(snowy).ok()?;
        block.set_snowy(snowy);
        Some(block)
    }
    fn dirt_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::dirt();
        Some(block)
    }
    fn coarse_dirt_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::coarse_dirt();
        Some(block)
    }
    fn podzol_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::podzol();
        let snowy = map.get("snowy")?;
        let snowy = bool::from_str(snowy).ok()?;
        block.set_snowy(snowy);
        Some(block)
    }
    fn cobblestone_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::cobblestone();
        Some(block)
    }
    fn oak_planks_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::oak_planks();
        Some(block)
    }
    fn spruce_planks_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::spruce_planks();
        Some(block)
    }
    fn birch_planks_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::birch_planks();
        Some(block)
    }
    fn jungle_planks_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::jungle_planks();
        Some(block)
    }
    fn acacia_planks_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::acacia_planks();
        Some(block)
    }
    fn dark_oak_planks_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dark_oak_planks();
        Some(block)
    }
    fn oak_sapling_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::oak_sapling();
        let stage = map.get("stage")?;
        let stage = {
            let x = i32::from_str(stage).ok()?;
            if !(0i32..=1i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_stage(stage);
        Some(block)
    }
    fn spruce_sapling_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::spruce_sapling();
        let stage = map.get("stage")?;
        let stage = {
            let x = i32::from_str(stage).ok()?;
            if !(0i32..=1i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_stage(stage);
        Some(block)
    }
    fn birch_sapling_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::birch_sapling();
        let stage = map.get("stage")?;
        let stage = {
            let x = i32::from_str(stage).ok()?;
            if !(0i32..=1i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_stage(stage);
        Some(block)
    }
    fn jungle_sapling_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::jungle_sapling();
        let stage = map.get("stage")?;
        let stage = {
            let x = i32::from_str(stage).ok()?;
            if !(0i32..=1i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_stage(stage);
        Some(block)
    }
    fn acacia_sapling_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::acacia_sapling();
        let stage = map.get("stage")?;
        let stage = {
            let x = i32::from_str(stage).ok()?;
            if !(0i32..=1i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_stage(stage);
        Some(block)
    }
    fn dark_oak_sapling_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dark_oak_sapling();
        let stage = map.get("stage")?;
        let stage = {
            let x = i32::from_str(stage).ok()?;
            if !(0i32..=1i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_stage(stage);
        Some(block)
    }
    fn bedrock_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::bedrock();
        Some(block)
    }
    fn water_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::water();
        let water_level = map.get("level")?;
        let water_level = {
            let x = i32::from_str(water_level).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_water_level(water_level);
        Some(block)
    }
    fn lava_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::lava();
        let water_level = map.get("level")?;
        let water_level = {
            let x = i32::from_str(water_level).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_water_level(water_level);
        Some(block)
    }
    fn sand_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::sand();
        Some(block)
    }
    fn red_sand_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::red_sand();
        Some(block)
    }
    fn gravel_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::gravel();
        Some(block)
    }
    fn gold_ore_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::gold_ore();
        Some(block)
    }
    fn iron_ore_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::iron_ore();
        Some(block)
    }
    fn coal_ore_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::coal_ore();
        Some(block)
    }
    fn oak_log_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::oak_log();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn spruce_log_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::spruce_log();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn birch_log_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::birch_log();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn jungle_log_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::jungle_log();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn acacia_log_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::acacia_log();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn dark_oak_log_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::dark_oak_log();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn stripped_spruce_log_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::stripped_spruce_log();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn stripped_birch_log_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::stripped_birch_log();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn stripped_jungle_log_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::stripped_jungle_log();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn stripped_acacia_log_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::stripped_acacia_log();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn stripped_dark_oak_log_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::stripped_dark_oak_log();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn stripped_oak_log_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::stripped_oak_log();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn oak_wood_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::oak_wood();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn spruce_wood_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::spruce_wood();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn birch_wood_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::birch_wood();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn jungle_wood_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::jungle_wood();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn acacia_wood_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::acacia_wood();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn dark_oak_wood_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dark_oak_wood();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn stripped_oak_wood_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::stripped_oak_wood();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn stripped_spruce_wood_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::stripped_spruce_wood();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn stripped_birch_wood_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::stripped_birch_wood();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn stripped_jungle_wood_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::stripped_jungle_wood();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn stripped_acacia_wood_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::stripped_acacia_wood();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn stripped_dark_oak_wood_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::stripped_dark_oak_wood();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn oak_leaves_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::oak_leaves();
        let distance = map.get("distance")?;
        let distance = {
            let x = i32::from_str(distance).ok()?;
            if !(1i32..=7i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_distance(distance);
        let persistent = map.get("persistent")?;
        let persistent = bool::from_str(persistent).ok()?;
        block.set_persistent(persistent);
        Some(block)
    }
    fn spruce_leaves_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::spruce_leaves();
        let distance = map.get("distance")?;
        let distance = {
            let x = i32::from_str(distance).ok()?;
            if !(1i32..=7i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_distance(distance);
        let persistent = map.get("persistent")?;
        let persistent = bool::from_str(persistent).ok()?;
        block.set_persistent(persistent);
        Some(block)
    }
    fn birch_leaves_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::birch_leaves();
        let distance = map.get("distance")?;
        let distance = {
            let x = i32::from_str(distance).ok()?;
            if !(1i32..=7i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_distance(distance);
        let persistent = map.get("persistent")?;
        let persistent = bool::from_str(persistent).ok()?;
        block.set_persistent(persistent);
        Some(block)
    }
    fn jungle_leaves_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::jungle_leaves();
        let distance = map.get("distance")?;
        let distance = {
            let x = i32::from_str(distance).ok()?;
            if !(1i32..=7i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_distance(distance);
        let persistent = map.get("persistent")?;
        let persistent = bool::from_str(persistent).ok()?;
        block.set_persistent(persistent);
        Some(block)
    }
    fn acacia_leaves_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::acacia_leaves();
        let distance = map.get("distance")?;
        let distance = {
            let x = i32::from_str(distance).ok()?;
            if !(1i32..=7i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_distance(distance);
        let persistent = map.get("persistent")?;
        let persistent = bool::from_str(persistent).ok()?;
        block.set_persistent(persistent);
        Some(block)
    }
    fn dark_oak_leaves_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dark_oak_leaves();
        let distance = map.get("distance")?;
        let distance = {
            let x = i32::from_str(distance).ok()?;
            if !(1i32..=7i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_distance(distance);
        let persistent = map.get("persistent")?;
        let persistent = bool::from_str(persistent).ok()?;
        block.set_persistent(persistent);
        Some(block)
    }
    fn sponge_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::sponge();
        Some(block)
    }
    fn wet_sponge_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::wet_sponge();
        Some(block)
    }
    fn glass_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::glass();
        Some(block)
    }
    fn lapis_ore_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::lapis_ore();
        Some(block)
    }
    fn lapis_block_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::lapis_block();
        Some(block)
    }
    fn dispenser_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::dispenser();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        let triggered = map.get("triggered")?;
        let triggered = bool::from_str(triggered).ok()?;
        block.set_triggered(triggered);
        Some(block)
    }
    fn sandstone_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::sandstone();
        Some(block)
    }
    fn chiseled_sandstone_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::chiseled_sandstone();
        Some(block)
    }
    fn cut_sandstone_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::cut_sandstone();
        Some(block)
    }
    fn note_block_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::note_block();
        let instrument = map.get("instrument")?;
        let instrument = Instrument::from_str(instrument).ok()?;
        block.set_instrument(instrument);
        let note = map.get("note")?;
        let note = {
            let x = i32::from_str(note).ok()?;
            if !(0i32..=24i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_note(note);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn white_bed_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::white_bed();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let occupied = map.get("occupied")?;
        let occupied = bool::from_str(occupied).ok()?;
        block.set_occupied(occupied);
        let part = map.get("part")?;
        let part = Part::from_str(part).ok()?;
        block.set_part(part);
        Some(block)
    }
    fn orange_bed_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::orange_bed();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let occupied = map.get("occupied")?;
        let occupied = bool::from_str(occupied).ok()?;
        block.set_occupied(occupied);
        let part = map.get("part")?;
        let part = Part::from_str(part).ok()?;
        block.set_part(part);
        Some(block)
    }
    fn magenta_bed_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::magenta_bed();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let occupied = map.get("occupied")?;
        let occupied = bool::from_str(occupied).ok()?;
        block.set_occupied(occupied);
        let part = map.get("part")?;
        let part = Part::from_str(part).ok()?;
        block.set_part(part);
        Some(block)
    }
    fn light_blue_bed_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_blue_bed();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let occupied = map.get("occupied")?;
        let occupied = bool::from_str(occupied).ok()?;
        block.set_occupied(occupied);
        let part = map.get("part")?;
        let part = Part::from_str(part).ok()?;
        block.set_part(part);
        Some(block)
    }
    fn yellow_bed_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::yellow_bed();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let occupied = map.get("occupied")?;
        let occupied = bool::from_str(occupied).ok()?;
        block.set_occupied(occupied);
        let part = map.get("part")?;
        let part = Part::from_str(part).ok()?;
        block.set_part(part);
        Some(block)
    }
    fn lime_bed_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::lime_bed();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let occupied = map.get("occupied")?;
        let occupied = bool::from_str(occupied).ok()?;
        block.set_occupied(occupied);
        let part = map.get("part")?;
        let part = Part::from_str(part).ok()?;
        block.set_part(part);
        Some(block)
    }
    fn pink_bed_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::pink_bed();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let occupied = map.get("occupied")?;
        let occupied = bool::from_str(occupied).ok()?;
        block.set_occupied(occupied);
        let part = map.get("part")?;
        let part = Part::from_str(part).ok()?;
        block.set_part(part);
        Some(block)
    }
    fn gray_bed_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::gray_bed();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let occupied = map.get("occupied")?;
        let occupied = bool::from_str(occupied).ok()?;
        block.set_occupied(occupied);
        let part = map.get("part")?;
        let part = Part::from_str(part).ok()?;
        block.set_part(part);
        Some(block)
    }
    fn light_gray_bed_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_gray_bed();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let occupied = map.get("occupied")?;
        let occupied = bool::from_str(occupied).ok()?;
        block.set_occupied(occupied);
        let part = map.get("part")?;
        let part = Part::from_str(part).ok()?;
        block.set_part(part);
        Some(block)
    }
    fn cyan_bed_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::cyan_bed();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let occupied = map.get("occupied")?;
        let occupied = bool::from_str(occupied).ok()?;
        block.set_occupied(occupied);
        let part = map.get("part")?;
        let part = Part::from_str(part).ok()?;
        block.set_part(part);
        Some(block)
    }
    fn purple_bed_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::purple_bed();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let occupied = map.get("occupied")?;
        let occupied = bool::from_str(occupied).ok()?;
        block.set_occupied(occupied);
        let part = map.get("part")?;
        let part = Part::from_str(part).ok()?;
        block.set_part(part);
        Some(block)
    }
    fn blue_bed_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::blue_bed();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let occupied = map.get("occupied")?;
        let occupied = bool::from_str(occupied).ok()?;
        block.set_occupied(occupied);
        let part = map.get("part")?;
        let part = Part::from_str(part).ok()?;
        block.set_part(part);
        Some(block)
    }
    fn brown_bed_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::brown_bed();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let occupied = map.get("occupied")?;
        let occupied = bool::from_str(occupied).ok()?;
        block.set_occupied(occupied);
        let part = map.get("part")?;
        let part = Part::from_str(part).ok()?;
        block.set_part(part);
        Some(block)
    }
    fn green_bed_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::green_bed();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let occupied = map.get("occupied")?;
        let occupied = bool::from_str(occupied).ok()?;
        block.set_occupied(occupied);
        let part = map.get("part")?;
        let part = Part::from_str(part).ok()?;
        block.set_part(part);
        Some(block)
    }
    fn red_bed_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::red_bed();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let occupied = map.get("occupied")?;
        let occupied = bool::from_str(occupied).ok()?;
        block.set_occupied(occupied);
        let part = map.get("part")?;
        let part = Part::from_str(part).ok()?;
        block.set_part(part);
        Some(block)
    }
    fn black_bed_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::black_bed();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let occupied = map.get("occupied")?;
        let occupied = bool::from_str(occupied).ok()?;
        block.set_occupied(occupied);
        let part = map.get("part")?;
        let part = Part::from_str(part).ok()?;
        block.set_part(part);
        Some(block)
    }
    fn powered_rail_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::powered_rail();
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        let powered_rail_shape = map.get("shape")?;
        let powered_rail_shape = PoweredRailShape::from_str(powered_rail_shape).ok()?;
        block.set_powered_rail_shape(powered_rail_shape);
        Some(block)
    }
    fn detector_rail_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::detector_rail();
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        let powered_rail_shape = map.get("shape")?;
        let powered_rail_shape = PoweredRailShape::from_str(powered_rail_shape).ok()?;
        block.set_powered_rail_shape(powered_rail_shape);
        Some(block)
    }
    fn sticky_piston_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::sticky_piston();
        let extended = map.get("extended")?;
        let extended = bool::from_str(extended).ok()?;
        block.set_extended(extended);
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn cobweb_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::cobweb();
        Some(block)
    }
    fn grass_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::grass();
        Some(block)
    }
    fn fern_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::fern();
        Some(block)
    }
    fn dead_bush_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::dead_bush();
        Some(block)
    }
    fn seagrass_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::seagrass();
        Some(block)
    }
    fn tall_seagrass_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::tall_seagrass();
        let half_upper_lower = map.get("half")?;
        let half_upper_lower = HalfUpperLower::from_str(half_upper_lower).ok()?;
        block.set_half_upper_lower(half_upper_lower);
        Some(block)
    }
    fn piston_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::piston();
        let extended = map.get("extended")?;
        let extended = bool::from_str(extended).ok()?;
        block.set_extended(extended);
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn piston_head_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::piston_head();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        let piston_kind = map.get("type")?;
        let piston_kind = PistonKind::from_str(piston_kind).ok()?;
        block.set_piston_kind(piston_kind);
        let short = map.get("short")?;
        let short = bool::from_str(short).ok()?;
        block.set_short(short);
        Some(block)
    }
    fn white_wool_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::white_wool();
        Some(block)
    }
    fn orange_wool_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::orange_wool();
        Some(block)
    }
    fn magenta_wool_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::magenta_wool();
        Some(block)
    }
    fn light_blue_wool_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_blue_wool();
        Some(block)
    }
    fn yellow_wool_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::yellow_wool();
        Some(block)
    }
    fn lime_wool_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::lime_wool();
        Some(block)
    }
    fn pink_wool_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::pink_wool();
        Some(block)
    }
    fn gray_wool_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::gray_wool();
        Some(block)
    }
    fn light_gray_wool_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_gray_wool();
        Some(block)
    }
    fn cyan_wool_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::cyan_wool();
        Some(block)
    }
    fn purple_wool_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::purple_wool();
        Some(block)
    }
    fn blue_wool_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::blue_wool();
        Some(block)
    }
    fn brown_wool_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::brown_wool();
        Some(block)
    }
    fn green_wool_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::green_wool();
        Some(block)
    }
    fn red_wool_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::red_wool();
        Some(block)
    }
    fn black_wool_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::black_wool();
        Some(block)
    }
    fn moving_piston_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::moving_piston();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        let piston_kind = map.get("type")?;
        let piston_kind = PistonKind::from_str(piston_kind).ok()?;
        block.set_piston_kind(piston_kind);
        Some(block)
    }
    fn dandelion_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::dandelion();
        Some(block)
    }
    fn poppy_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::poppy();
        Some(block)
    }
    fn blue_orchid_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::blue_orchid();
        Some(block)
    }
    fn allium_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::allium();
        Some(block)
    }
    fn azure_bluet_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::azure_bluet();
        Some(block)
    }
    fn red_tulip_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::red_tulip();
        Some(block)
    }
    fn orange_tulip_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::orange_tulip();
        Some(block)
    }
    fn white_tulip_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::white_tulip();
        Some(block)
    }
    fn pink_tulip_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::pink_tulip();
        Some(block)
    }
    fn oxeye_daisy_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::oxeye_daisy();
        Some(block)
    }
    fn brown_mushroom_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::brown_mushroom();
        Some(block)
    }
    fn red_mushroom_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::red_mushroom();
        Some(block)
    }
    fn gold_block_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::gold_block();
        Some(block)
    }
    fn iron_block_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::iron_block();
        Some(block)
    }
    fn bricks_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::bricks();
        Some(block)
    }
    fn tnt_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::tnt();
        let unstable = map.get("unstable")?;
        let unstable = bool::from_str(unstable).ok()?;
        block.set_unstable(unstable);
        Some(block)
    }
    fn bookshelf_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::bookshelf();
        Some(block)
    }
    fn mossy_cobblestone_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::mossy_cobblestone();
        Some(block)
    }
    fn obsidian_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::obsidian();
        Some(block)
    }
    fn torch_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::torch();
        Some(block)
    }
    fn wall_torch_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::wall_torch();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn fire_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::fire();
        let age_0_15 = map.get("age")?;
        let age_0_15 = {
            let x = i32::from_str(age_0_15).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_age_0_15(age_0_15);
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let up = map.get("up")?;
        let up = bool::from_str(up).ok()?;
        block.set_up(up);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn spawner_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::spawner();
        Some(block)
    }
    fn oak_stairs_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::oak_stairs();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let stairs_shape = map.get("shape")?;
        let stairs_shape = StairsShape::from_str(stairs_shape).ok()?;
        block.set_stairs_shape(stairs_shape);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn chest_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::chest();
        let chest_kind = map.get("type")?;
        let chest_kind = ChestKind::from_str(chest_kind).ok()?;
        block.set_chest_kind(chest_kind);
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn redstone_wire_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::redstone_wire();
        let east_wire = map.get("east")?;
        let east_wire = EastWire::from_str(east_wire).ok()?;
        block.set_east_wire(east_wire);
        let north_wire = map.get("north")?;
        let north_wire = NorthWire::from_str(north_wire).ok()?;
        block.set_north_wire(north_wire);
        let power = map.get("power")?;
        let power = {
            let x = i32::from_str(power).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_power(power);
        let south_wire = map.get("south")?;
        let south_wire = SouthWire::from_str(south_wire).ok()?;
        block.set_south_wire(south_wire);
        let west_wire = map.get("west")?;
        let west_wire = WestWire::from_str(west_wire).ok()?;
        block.set_west_wire(west_wire);
        Some(block)
    }
    fn diamond_ore_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::diamond_ore();
        Some(block)
    }
    fn diamond_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::diamond_block();
        Some(block)
    }
    fn crafting_table_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::crafting_table();
        Some(block)
    }
    fn wheat_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::wheat();
        let age_0_7 = map.get("age")?;
        let age_0_7 = {
            let x = i32::from_str(age_0_7).ok()?;
            if !(0i32..=7i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_age_0_7(age_0_7);
        Some(block)
    }
    fn farmland_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::farmland();
        let moisture = map.get("moisture")?;
        let moisture = {
            let x = i32::from_str(moisture).ok()?;
            if !(0i32..=7i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_moisture(moisture);
        Some(block)
    }
    fn furnace_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::furnace();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let lit = map.get("lit")?;
        let lit = bool::from_str(lit).ok()?;
        block.set_lit(lit);
        Some(block)
    }
    fn sign_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::sign();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn oak_door_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::oak_door();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_upper_lower = map.get("half")?;
        let half_upper_lower = HalfUpperLower::from_str(half_upper_lower).ok()?;
        block.set_half_upper_lower(half_upper_lower);
        let hinge = map.get("hinge")?;
        let hinge = Hinge::from_str(hinge).ok()?;
        block.set_hinge(hinge);
        let open = map.get("open")?;
        let open = bool::from_str(open).ok()?;
        block.set_open(open);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn ladder_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::ladder();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn rail_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::rail();
        let rail_shape = map.get("shape")?;
        let rail_shape = RailShape::from_str(rail_shape).ok()?;
        block.set_rail_shape(rail_shape);
        Some(block)
    }
    fn cobblestone_stairs_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::cobblestone_stairs();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let stairs_shape = map.get("shape")?;
        let stairs_shape = StairsShape::from_str(stairs_shape).ok()?;
        block.set_stairs_shape(stairs_shape);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn wall_sign_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::wall_sign();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn lever_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::lever();
        let face = map.get("face")?;
        let face = Face::from_str(face).ok()?;
        block.set_face(face);
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn stone_pressure_plate_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::stone_pressure_plate();
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn iron_door_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::iron_door();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_upper_lower = map.get("half")?;
        let half_upper_lower = HalfUpperLower::from_str(half_upper_lower).ok()?;
        block.set_half_upper_lower(half_upper_lower);
        let hinge = map.get("hinge")?;
        let hinge = Hinge::from_str(hinge).ok()?;
        block.set_hinge(hinge);
        let open = map.get("open")?;
        let open = bool::from_str(open).ok()?;
        block.set_open(open);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn oak_pressure_plate_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::oak_pressure_plate();
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn spruce_pressure_plate_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::spruce_pressure_plate();
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn birch_pressure_plate_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::birch_pressure_plate();
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn jungle_pressure_plate_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::jungle_pressure_plate();
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn acacia_pressure_plate_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::acacia_pressure_plate();
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn dark_oak_pressure_plate_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dark_oak_pressure_plate();
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn redstone_ore_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::redstone_ore();
        let lit = map.get("lit")?;
        let lit = bool::from_str(lit).ok()?;
        block.set_lit(lit);
        Some(block)
    }
    fn redstone_torch_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::redstone_torch();
        let lit = map.get("lit")?;
        let lit = bool::from_str(lit).ok()?;
        block.set_lit(lit);
        Some(block)
    }
    fn redstone_wall_torch_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::redstone_wall_torch();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let lit = map.get("lit")?;
        let lit = bool::from_str(lit).ok()?;
        block.set_lit(lit);
        Some(block)
    }
    fn stone_button_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::stone_button();
        let face = map.get("face")?;
        let face = Face::from_str(face).ok()?;
        block.set_face(face);
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn snow_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::snow();
        let layers = map.get("layers")?;
        let layers = {
            let x = i32::from_str(layers).ok()?;
            if !(1i32..=8i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_layers(layers);
        Some(block)
    }
    fn ice_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::ice();
        Some(block)
    }
    fn snow_block_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::snow_block();
        Some(block)
    }
    fn cactus_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::cactus();
        let age_0_15 = map.get("age")?;
        let age_0_15 = {
            let x = i32::from_str(age_0_15).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_age_0_15(age_0_15);
        Some(block)
    }
    fn clay_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::clay();
        Some(block)
    }
    fn sugar_cane_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::sugar_cane();
        let age_0_15 = map.get("age")?;
        let age_0_15 = {
            let x = i32::from_str(age_0_15).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_age_0_15(age_0_15);
        Some(block)
    }
    fn jukebox_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::jukebox();
        let has_record = map.get("has_record")?;
        let has_record = bool::from_str(has_record).ok()?;
        block.set_has_record(has_record);
        Some(block)
    }
    fn oak_fence_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::oak_fence();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn pumpkin_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::pumpkin();
        Some(block)
    }
    fn netherrack_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::netherrack();
        Some(block)
    }
    fn soul_sand_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::soul_sand();
        Some(block)
    }
    fn glowstone_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::glowstone();
        Some(block)
    }
    fn nether_portal_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::nether_portal();
        let axis_xz = map.get("axis")?;
        let axis_xz = AxisXz::from_str(axis_xz).ok()?;
        block.set_axis_xz(axis_xz);
        Some(block)
    }
    fn carved_pumpkin_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::carved_pumpkin();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn jack_o_lantern_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::jack_o_lantern();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn cake_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::cake();
        let bites = map.get("bites")?;
        let bites = {
            let x = i32::from_str(bites).ok()?;
            if !(0i32..=6i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_bites(bites);
        Some(block)
    }
    fn repeater_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::repeater();
        let delay = map.get("delay")?;
        let delay = {
            let x = i32::from_str(delay).ok()?;
            if !(1i32..=4i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_delay(delay);
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let locked = map.get("locked")?;
        let locked = bool::from_str(locked).ok()?;
        block.set_locked(locked);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn white_stained_glass_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::white_stained_glass();
        Some(block)
    }
    fn orange_stained_glass_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::orange_stained_glass();
        Some(block)
    }
    fn magenta_stained_glass_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::magenta_stained_glass();
        Some(block)
    }
    fn light_blue_stained_glass_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_blue_stained_glass();
        Some(block)
    }
    fn yellow_stained_glass_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::yellow_stained_glass();
        Some(block)
    }
    fn lime_stained_glass_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::lime_stained_glass();
        Some(block)
    }
    fn pink_stained_glass_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::pink_stained_glass();
        Some(block)
    }
    fn gray_stained_glass_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::gray_stained_glass();
        Some(block)
    }
    fn light_gray_stained_glass_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_gray_stained_glass();
        Some(block)
    }
    fn cyan_stained_glass_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::cyan_stained_glass();
        Some(block)
    }
    fn purple_stained_glass_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::purple_stained_glass();
        Some(block)
    }
    fn blue_stained_glass_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::blue_stained_glass();
        Some(block)
    }
    fn brown_stained_glass_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::brown_stained_glass();
        Some(block)
    }
    fn green_stained_glass_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::green_stained_glass();
        Some(block)
    }
    fn red_stained_glass_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::red_stained_glass();
        Some(block)
    }
    fn black_stained_glass_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::black_stained_glass();
        Some(block)
    }
    fn oak_trapdoor_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::oak_trapdoor();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let open = map.get("open")?;
        let open = bool::from_str(open).ok()?;
        block.set_open(open);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn spruce_trapdoor_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::spruce_trapdoor();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let open = map.get("open")?;
        let open = bool::from_str(open).ok()?;
        block.set_open(open);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn birch_trapdoor_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::birch_trapdoor();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let open = map.get("open")?;
        let open = bool::from_str(open).ok()?;
        block.set_open(open);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn jungle_trapdoor_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::jungle_trapdoor();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let open = map.get("open")?;
        let open = bool::from_str(open).ok()?;
        block.set_open(open);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn acacia_trapdoor_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::acacia_trapdoor();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let open = map.get("open")?;
        let open = bool::from_str(open).ok()?;
        block.set_open(open);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn dark_oak_trapdoor_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dark_oak_trapdoor();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let open = map.get("open")?;
        let open = bool::from_str(open).ok()?;
        block.set_open(open);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn infested_stone_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::infested_stone();
        Some(block)
    }
    fn infested_cobblestone_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::infested_cobblestone();
        Some(block)
    }
    fn infested_stone_bricks_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::infested_stone_bricks();
        Some(block)
    }
    fn infested_mossy_stone_bricks_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::infested_mossy_stone_bricks();
        Some(block)
    }
    fn infested_cracked_stone_bricks_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::infested_cracked_stone_bricks();
        Some(block)
    }
    fn infested_chiseled_stone_bricks_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::infested_chiseled_stone_bricks();
        Some(block)
    }
    fn stone_bricks_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::stone_bricks();
        Some(block)
    }
    fn mossy_stone_bricks_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::mossy_stone_bricks();
        Some(block)
    }
    fn cracked_stone_bricks_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::cracked_stone_bricks();
        Some(block)
    }
    fn chiseled_stone_bricks_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::chiseled_stone_bricks();
        Some(block)
    }
    fn brown_mushroom_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::brown_mushroom_block();
        let down = map.get("down")?;
        let down = bool::from_str(down).ok()?;
        block.set_down(down);
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let up = map.get("up")?;
        let up = bool::from_str(up).ok()?;
        block.set_up(up);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn red_mushroom_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::red_mushroom_block();
        let down = map.get("down")?;
        let down = bool::from_str(down).ok()?;
        block.set_down(down);
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let up = map.get("up")?;
        let up = bool::from_str(up).ok()?;
        block.set_up(up);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn mushroom_stem_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::mushroom_stem();
        let down = map.get("down")?;
        let down = bool::from_str(down).ok()?;
        block.set_down(down);
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let up = map.get("up")?;
        let up = bool::from_str(up).ok()?;
        block.set_up(up);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn iron_bars_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::iron_bars();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn glass_pane_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::glass_pane();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn melon_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::melon();
        Some(block)
    }
    fn attached_pumpkin_stem_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::attached_pumpkin_stem();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn attached_melon_stem_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::attached_melon_stem();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn pumpkin_stem_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::pumpkin_stem();
        let age_0_7 = map.get("age")?;
        let age_0_7 = {
            let x = i32::from_str(age_0_7).ok()?;
            if !(0i32..=7i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_age_0_7(age_0_7);
        Some(block)
    }
    fn melon_stem_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::melon_stem();
        let age_0_7 = map.get("age")?;
        let age_0_7 = {
            let x = i32::from_str(age_0_7).ok()?;
            if !(0i32..=7i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_age_0_7(age_0_7);
        Some(block)
    }
    fn vine_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::vine();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let up = map.get("up")?;
        let up = bool::from_str(up).ok()?;
        block.set_up(up);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn oak_fence_gate_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::oak_fence_gate();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let in_wall = map.get("in_wall")?;
        let in_wall = bool::from_str(in_wall).ok()?;
        block.set_in_wall(in_wall);
        let open = map.get("open")?;
        let open = bool::from_str(open).ok()?;
        block.set_open(open);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn brick_stairs_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::brick_stairs();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let stairs_shape = map.get("shape")?;
        let stairs_shape = StairsShape::from_str(stairs_shape).ok()?;
        block.set_stairs_shape(stairs_shape);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn stone_brick_stairs_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::stone_brick_stairs();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let stairs_shape = map.get("shape")?;
        let stairs_shape = StairsShape::from_str(stairs_shape).ok()?;
        block.set_stairs_shape(stairs_shape);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn mycelium_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::mycelium();
        let snowy = map.get("snowy")?;
        let snowy = bool::from_str(snowy).ok()?;
        block.set_snowy(snowy);
        Some(block)
    }
    fn lily_pad_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::lily_pad();
        Some(block)
    }
    fn nether_bricks_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::nether_bricks();
        Some(block)
    }
    fn nether_brick_fence_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::nether_brick_fence();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn nether_brick_stairs_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::nether_brick_stairs();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let stairs_shape = map.get("shape")?;
        let stairs_shape = StairsShape::from_str(stairs_shape).ok()?;
        block.set_stairs_shape(stairs_shape);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn nether_wart_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::nether_wart();
        let age_0_3 = map.get("age")?;
        let age_0_3 = {
            let x = i32::from_str(age_0_3).ok()?;
            if !(0i32..=3i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_age_0_3(age_0_3);
        Some(block)
    }
    fn enchanting_table_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::enchanting_table();
        Some(block)
    }
    fn brewing_stand_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::brewing_stand();
        let has_bottle_0 = map.get("has_bottle_0")?;
        let has_bottle_0 = bool::from_str(has_bottle_0).ok()?;
        block.set_has_bottle_0(has_bottle_0);
        let has_bottle_1 = map.get("has_bottle_1")?;
        let has_bottle_1 = bool::from_str(has_bottle_1).ok()?;
        block.set_has_bottle_1(has_bottle_1);
        let has_bottle_2 = map.get("has_bottle_2")?;
        let has_bottle_2 = bool::from_str(has_bottle_2).ok()?;
        block.set_has_bottle_2(has_bottle_2);
        Some(block)
    }
    fn cauldron_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::cauldron();
        let cauldron_level = map.get("level")?;
        let cauldron_level = {
            let x = i32::from_str(cauldron_level).ok()?;
            if !(0i32..=3i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_cauldron_level(cauldron_level);
        Some(block)
    }
    fn end_portal_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::end_portal();
        Some(block)
    }
    fn end_portal_frame_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::end_portal_frame();
        let eye = map.get("eye")?;
        let eye = bool::from_str(eye).ok()?;
        block.set_eye(eye);
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn end_stone_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::end_stone();
        Some(block)
    }
    fn dragon_egg_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::dragon_egg();
        Some(block)
    }
    fn redstone_lamp_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::redstone_lamp();
        let lit = map.get("lit")?;
        let lit = bool::from_str(lit).ok()?;
        block.set_lit(lit);
        Some(block)
    }
    fn cocoa_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::cocoa();
        let age_0_2 = map.get("age")?;
        let age_0_2 = {
            let x = i32::from_str(age_0_2).ok()?;
            if !(0i32..=2i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_age_0_2(age_0_2);
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn sandstone_stairs_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::sandstone_stairs();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let stairs_shape = map.get("shape")?;
        let stairs_shape = StairsShape::from_str(stairs_shape).ok()?;
        block.set_stairs_shape(stairs_shape);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn emerald_ore_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::emerald_ore();
        Some(block)
    }
    fn ender_chest_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::ender_chest();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn tripwire_hook_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::tripwire_hook();
        let attached = map.get("attached")?;
        let attached = bool::from_str(attached).ok()?;
        block.set_attached(attached);
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn tripwire_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::tripwire();
        let attached = map.get("attached")?;
        let attached = bool::from_str(attached).ok()?;
        block.set_attached(attached);
        let disarmed = map.get("disarmed")?;
        let disarmed = bool::from_str(disarmed).ok()?;
        block.set_disarmed(disarmed);
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn emerald_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::emerald_block();
        Some(block)
    }
    fn spruce_stairs_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::spruce_stairs();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let stairs_shape = map.get("shape")?;
        let stairs_shape = StairsShape::from_str(stairs_shape).ok()?;
        block.set_stairs_shape(stairs_shape);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn birch_stairs_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::birch_stairs();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let stairs_shape = map.get("shape")?;
        let stairs_shape = StairsShape::from_str(stairs_shape).ok()?;
        block.set_stairs_shape(stairs_shape);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn jungle_stairs_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::jungle_stairs();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let stairs_shape = map.get("shape")?;
        let stairs_shape = StairsShape::from_str(stairs_shape).ok()?;
        block.set_stairs_shape(stairs_shape);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn command_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::command_block();
        let conditional = map.get("conditional")?;
        let conditional = bool::from_str(conditional).ok()?;
        block.set_conditional(conditional);
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn beacon_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::beacon();
        Some(block)
    }
    fn cobblestone_wall_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::cobblestone_wall();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let up = map.get("up")?;
        let up = bool::from_str(up).ok()?;
        block.set_up(up);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn mossy_cobblestone_wall_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::mossy_cobblestone_wall();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let up = map.get("up")?;
        let up = bool::from_str(up).ok()?;
        block.set_up(up);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn flower_pot_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::flower_pot();
        Some(block)
    }
    fn potted_oak_sapling_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::potted_oak_sapling();
        Some(block)
    }
    fn potted_spruce_sapling_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::potted_spruce_sapling();
        Some(block)
    }
    fn potted_birch_sapling_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::potted_birch_sapling();
        Some(block)
    }
    fn potted_jungle_sapling_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::potted_jungle_sapling();
        Some(block)
    }
    fn potted_acacia_sapling_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::potted_acacia_sapling();
        Some(block)
    }
    fn potted_dark_oak_sapling_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::potted_dark_oak_sapling();
        Some(block)
    }
    fn potted_fern_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::potted_fern();
        Some(block)
    }
    fn potted_dandelion_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::potted_dandelion();
        Some(block)
    }
    fn potted_poppy_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::potted_poppy();
        Some(block)
    }
    fn potted_blue_orchid_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::potted_blue_orchid();
        Some(block)
    }
    fn potted_allium_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::potted_allium();
        Some(block)
    }
    fn potted_azure_bluet_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::potted_azure_bluet();
        Some(block)
    }
    fn potted_red_tulip_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::potted_red_tulip();
        Some(block)
    }
    fn potted_orange_tulip_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::potted_orange_tulip();
        Some(block)
    }
    fn potted_white_tulip_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::potted_white_tulip();
        Some(block)
    }
    fn potted_pink_tulip_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::potted_pink_tulip();
        Some(block)
    }
    fn potted_oxeye_daisy_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::potted_oxeye_daisy();
        Some(block)
    }
    fn potted_red_mushroom_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::potted_red_mushroom();
        Some(block)
    }
    fn potted_brown_mushroom_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::potted_brown_mushroom();
        Some(block)
    }
    fn potted_dead_bush_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::potted_dead_bush();
        Some(block)
    }
    fn potted_cactus_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::potted_cactus();
        Some(block)
    }
    fn carrots_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::carrots();
        let age_0_7 = map.get("age")?;
        let age_0_7 = {
            let x = i32::from_str(age_0_7).ok()?;
            if !(0i32..=7i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_age_0_7(age_0_7);
        Some(block)
    }
    fn potatoes_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::potatoes();
        let age_0_7 = map.get("age")?;
        let age_0_7 = {
            let x = i32::from_str(age_0_7).ok()?;
            if !(0i32..=7i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_age_0_7(age_0_7);
        Some(block)
    }
    fn oak_button_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::oak_button();
        let face = map.get("face")?;
        let face = Face::from_str(face).ok()?;
        block.set_face(face);
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn spruce_button_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::spruce_button();
        let face = map.get("face")?;
        let face = Face::from_str(face).ok()?;
        block.set_face(face);
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn birch_button_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::birch_button();
        let face = map.get("face")?;
        let face = Face::from_str(face).ok()?;
        block.set_face(face);
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn jungle_button_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::jungle_button();
        let face = map.get("face")?;
        let face = Face::from_str(face).ok()?;
        block.set_face(face);
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn acacia_button_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::acacia_button();
        let face = map.get("face")?;
        let face = Face::from_str(face).ok()?;
        block.set_face(face);
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn dark_oak_button_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dark_oak_button();
        let face = map.get("face")?;
        let face = Face::from_str(face).ok()?;
        block.set_face(face);
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn skeleton_wall_skull_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::skeleton_wall_skull();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn skeleton_skull_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::skeleton_skull();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn wither_skeleton_wall_skull_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::wither_skeleton_wall_skull();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn wither_skeleton_skull_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::wither_skeleton_skull();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn zombie_wall_head_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::zombie_wall_head();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn zombie_head_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::zombie_head();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn player_wall_head_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::player_wall_head();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn player_head_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::player_head();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn creeper_wall_head_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::creeper_wall_head();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn creeper_head_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::creeper_head();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn dragon_wall_head_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dragon_wall_head();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn dragon_head_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::dragon_head();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn anvil_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::anvil();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn chipped_anvil_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::chipped_anvil();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn damaged_anvil_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::damaged_anvil();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn trapped_chest_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::trapped_chest();
        let chest_kind = map.get("type")?;
        let chest_kind = ChestKind::from_str(chest_kind).ok()?;
        block.set_chest_kind(chest_kind);
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn light_weighted_pressure_plate_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_weighted_pressure_plate();
        let power = map.get("power")?;
        let power = {
            let x = i32::from_str(power).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_power(power);
        Some(block)
    }
    fn heavy_weighted_pressure_plate_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::heavy_weighted_pressure_plate();
        let power = map.get("power")?;
        let power = {
            let x = i32::from_str(power).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_power(power);
        Some(block)
    }
    fn comparator_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::comparator();
        let comparator_mode = map.get("mode")?;
        let comparator_mode = ComparatorMode::from_str(comparator_mode).ok()?;
        block.set_comparator_mode(comparator_mode);
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn daylight_detector_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::daylight_detector();
        let inverted = map.get("inverted")?;
        let inverted = bool::from_str(inverted).ok()?;
        block.set_inverted(inverted);
        let power = map.get("power")?;
        let power = {
            let x = i32::from_str(power).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_power(power);
        Some(block)
    }
    fn redstone_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::redstone_block();
        Some(block)
    }
    fn nether_quartz_ore_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::nether_quartz_ore();
        Some(block)
    }
    fn hopper_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::hopper();
        let enabled = map.get("enabled")?;
        let enabled = bool::from_str(enabled).ok()?;
        block.set_enabled(enabled);
        let facing_cardinal_and_down = map.get("facing")?;
        let facing_cardinal_and_down =
            FacingCardinalAndDown::from_str(facing_cardinal_and_down).ok()?;
        block.set_facing_cardinal_and_down(facing_cardinal_and_down);
        Some(block)
    }
    fn quartz_block_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::quartz_block();
        Some(block)
    }
    fn chiseled_quartz_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::chiseled_quartz_block();
        Some(block)
    }
    fn quartz_pillar_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::quartz_pillar();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn quartz_stairs_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::quartz_stairs();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let stairs_shape = map.get("shape")?;
        let stairs_shape = StairsShape::from_str(stairs_shape).ok()?;
        block.set_stairs_shape(stairs_shape);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn activator_rail_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::activator_rail();
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        let powered_rail_shape = map.get("shape")?;
        let powered_rail_shape = PoweredRailShape::from_str(powered_rail_shape).ok()?;
        block.set_powered_rail_shape(powered_rail_shape);
        Some(block)
    }
    fn dropper_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::dropper();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        let triggered = map.get("triggered")?;
        let triggered = bool::from_str(triggered).ok()?;
        block.set_triggered(triggered);
        Some(block)
    }
    fn white_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::white_terracotta();
        Some(block)
    }
    fn orange_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::orange_terracotta();
        Some(block)
    }
    fn magenta_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::magenta_terracotta();
        Some(block)
    }
    fn light_blue_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_blue_terracotta();
        Some(block)
    }
    fn yellow_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::yellow_terracotta();
        Some(block)
    }
    fn lime_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::lime_terracotta();
        Some(block)
    }
    fn pink_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::pink_terracotta();
        Some(block)
    }
    fn gray_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::gray_terracotta();
        Some(block)
    }
    fn light_gray_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_gray_terracotta();
        Some(block)
    }
    fn cyan_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::cyan_terracotta();
        Some(block)
    }
    fn purple_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::purple_terracotta();
        Some(block)
    }
    fn blue_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::blue_terracotta();
        Some(block)
    }
    fn brown_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::brown_terracotta();
        Some(block)
    }
    fn green_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::green_terracotta();
        Some(block)
    }
    fn red_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::red_terracotta();
        Some(block)
    }
    fn black_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::black_terracotta();
        Some(block)
    }
    fn white_stained_glass_pane_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::white_stained_glass_pane();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn orange_stained_glass_pane_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::orange_stained_glass_pane();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn magenta_stained_glass_pane_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::magenta_stained_glass_pane();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn light_blue_stained_glass_pane_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_blue_stained_glass_pane();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn yellow_stained_glass_pane_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::yellow_stained_glass_pane();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn lime_stained_glass_pane_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::lime_stained_glass_pane();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn pink_stained_glass_pane_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::pink_stained_glass_pane();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn gray_stained_glass_pane_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::gray_stained_glass_pane();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn light_gray_stained_glass_pane_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_gray_stained_glass_pane();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn cyan_stained_glass_pane_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::cyan_stained_glass_pane();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn purple_stained_glass_pane_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::purple_stained_glass_pane();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn blue_stained_glass_pane_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::blue_stained_glass_pane();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn brown_stained_glass_pane_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::brown_stained_glass_pane();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn green_stained_glass_pane_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::green_stained_glass_pane();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn red_stained_glass_pane_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::red_stained_glass_pane();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn black_stained_glass_pane_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::black_stained_glass_pane();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn acacia_stairs_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::acacia_stairs();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let stairs_shape = map.get("shape")?;
        let stairs_shape = StairsShape::from_str(stairs_shape).ok()?;
        block.set_stairs_shape(stairs_shape);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn dark_oak_stairs_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dark_oak_stairs();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let stairs_shape = map.get("shape")?;
        let stairs_shape = StairsShape::from_str(stairs_shape).ok()?;
        block.set_stairs_shape(stairs_shape);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn slime_block_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::slime_block();
        Some(block)
    }
    fn barrier_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::barrier();
        Some(block)
    }
    fn iron_trapdoor_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::iron_trapdoor();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let open = map.get("open")?;
        let open = bool::from_str(open).ok()?;
        block.set_open(open);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn prismarine_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::prismarine();
        Some(block)
    }
    fn prismarine_bricks_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::prismarine_bricks();
        Some(block)
    }
    fn dark_prismarine_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dark_prismarine();
        Some(block)
    }
    fn prismarine_stairs_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::prismarine_stairs();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let stairs_shape = map.get("shape")?;
        let stairs_shape = StairsShape::from_str(stairs_shape).ok()?;
        block.set_stairs_shape(stairs_shape);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn prismarine_brick_stairs_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::prismarine_brick_stairs();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let stairs_shape = map.get("shape")?;
        let stairs_shape = StairsShape::from_str(stairs_shape).ok()?;
        block.set_stairs_shape(stairs_shape);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn dark_prismarine_stairs_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dark_prismarine_stairs();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let stairs_shape = map.get("shape")?;
        let stairs_shape = StairsShape::from_str(stairs_shape).ok()?;
        block.set_stairs_shape(stairs_shape);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn prismarine_slab_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::prismarine_slab();
        let slab_kind = map.get("type")?;
        let slab_kind = SlabKind::from_str(slab_kind).ok()?;
        block.set_slab_kind(slab_kind);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn prismarine_brick_slab_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::prismarine_brick_slab();
        let slab_kind = map.get("type")?;
        let slab_kind = SlabKind::from_str(slab_kind).ok()?;
        block.set_slab_kind(slab_kind);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn dark_prismarine_slab_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dark_prismarine_slab();
        let slab_kind = map.get("type")?;
        let slab_kind = SlabKind::from_str(slab_kind).ok()?;
        block.set_slab_kind(slab_kind);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn sea_lantern_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::sea_lantern();
        Some(block)
    }
    fn hay_block_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::hay_block();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn white_carpet_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::white_carpet();
        Some(block)
    }
    fn orange_carpet_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::orange_carpet();
        Some(block)
    }
    fn magenta_carpet_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::magenta_carpet();
        Some(block)
    }
    fn light_blue_carpet_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_blue_carpet();
        Some(block)
    }
    fn yellow_carpet_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::yellow_carpet();
        Some(block)
    }
    fn lime_carpet_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::lime_carpet();
        Some(block)
    }
    fn pink_carpet_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::pink_carpet();
        Some(block)
    }
    fn gray_carpet_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::gray_carpet();
        Some(block)
    }
    fn light_gray_carpet_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_gray_carpet();
        Some(block)
    }
    fn cyan_carpet_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::cyan_carpet();
        Some(block)
    }
    fn purple_carpet_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::purple_carpet();
        Some(block)
    }
    fn blue_carpet_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::blue_carpet();
        Some(block)
    }
    fn brown_carpet_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::brown_carpet();
        Some(block)
    }
    fn green_carpet_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::green_carpet();
        Some(block)
    }
    fn red_carpet_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::red_carpet();
        Some(block)
    }
    fn black_carpet_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::black_carpet();
        Some(block)
    }
    fn terracotta_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::terracotta();
        Some(block)
    }
    fn coal_block_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::coal_block();
        Some(block)
    }
    fn packed_ice_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::packed_ice();
        Some(block)
    }
    fn sunflower_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::sunflower();
        let half_upper_lower = map.get("half")?;
        let half_upper_lower = HalfUpperLower::from_str(half_upper_lower).ok()?;
        block.set_half_upper_lower(half_upper_lower);
        Some(block)
    }
    fn lilac_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::lilac();
        let half_upper_lower = map.get("half")?;
        let half_upper_lower = HalfUpperLower::from_str(half_upper_lower).ok()?;
        block.set_half_upper_lower(half_upper_lower);
        Some(block)
    }
    fn rose_bush_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::rose_bush();
        let half_upper_lower = map.get("half")?;
        let half_upper_lower = HalfUpperLower::from_str(half_upper_lower).ok()?;
        block.set_half_upper_lower(half_upper_lower);
        Some(block)
    }
    fn peony_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::peony();
        let half_upper_lower = map.get("half")?;
        let half_upper_lower = HalfUpperLower::from_str(half_upper_lower).ok()?;
        block.set_half_upper_lower(half_upper_lower);
        Some(block)
    }
    fn tall_grass_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::tall_grass();
        let half_upper_lower = map.get("half")?;
        let half_upper_lower = HalfUpperLower::from_str(half_upper_lower).ok()?;
        block.set_half_upper_lower(half_upper_lower);
        Some(block)
    }
    fn large_fern_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::large_fern();
        let half_upper_lower = map.get("half")?;
        let half_upper_lower = HalfUpperLower::from_str(half_upper_lower).ok()?;
        block.set_half_upper_lower(half_upper_lower);
        Some(block)
    }
    fn white_banner_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::white_banner();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn orange_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::orange_banner();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn magenta_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::magenta_banner();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn light_blue_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_blue_banner();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn yellow_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::yellow_banner();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn lime_banner_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::lime_banner();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn pink_banner_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::pink_banner();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn gray_banner_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::gray_banner();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn light_gray_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_gray_banner();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn cyan_banner_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::cyan_banner();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn purple_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::purple_banner();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn blue_banner_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::blue_banner();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn brown_banner_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::brown_banner();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn green_banner_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::green_banner();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn red_banner_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::red_banner();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn black_banner_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::black_banner();
        let rotation = map.get("rotation")?;
        let rotation = {
            let x = i32::from_str(rotation).ok()?;
            if !(0i32..=15i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_rotation(rotation);
        Some(block)
    }
    fn white_wall_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::white_wall_banner();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn orange_wall_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::orange_wall_banner();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn magenta_wall_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::magenta_wall_banner();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn light_blue_wall_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_blue_wall_banner();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn yellow_wall_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::yellow_wall_banner();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn lime_wall_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::lime_wall_banner();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn pink_wall_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::pink_wall_banner();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn gray_wall_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::gray_wall_banner();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn light_gray_wall_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_gray_wall_banner();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn cyan_wall_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::cyan_wall_banner();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn purple_wall_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::purple_wall_banner();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn blue_wall_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::blue_wall_banner();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn brown_wall_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::brown_wall_banner();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn green_wall_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::green_wall_banner();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn red_wall_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::red_wall_banner();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn black_wall_banner_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::black_wall_banner();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn red_sandstone_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::red_sandstone();
        Some(block)
    }
    fn chiseled_red_sandstone_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::chiseled_red_sandstone();
        Some(block)
    }
    fn cut_red_sandstone_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::cut_red_sandstone();
        Some(block)
    }
    fn red_sandstone_stairs_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::red_sandstone_stairs();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let stairs_shape = map.get("shape")?;
        let stairs_shape = StairsShape::from_str(stairs_shape).ok()?;
        block.set_stairs_shape(stairs_shape);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn oak_slab_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::oak_slab();
        let slab_kind = map.get("type")?;
        let slab_kind = SlabKind::from_str(slab_kind).ok()?;
        block.set_slab_kind(slab_kind);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn spruce_slab_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::spruce_slab();
        let slab_kind = map.get("type")?;
        let slab_kind = SlabKind::from_str(slab_kind).ok()?;
        block.set_slab_kind(slab_kind);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn birch_slab_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::birch_slab();
        let slab_kind = map.get("type")?;
        let slab_kind = SlabKind::from_str(slab_kind).ok()?;
        block.set_slab_kind(slab_kind);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn jungle_slab_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::jungle_slab();
        let slab_kind = map.get("type")?;
        let slab_kind = SlabKind::from_str(slab_kind).ok()?;
        block.set_slab_kind(slab_kind);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn acacia_slab_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::acacia_slab();
        let slab_kind = map.get("type")?;
        let slab_kind = SlabKind::from_str(slab_kind).ok()?;
        block.set_slab_kind(slab_kind);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn dark_oak_slab_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dark_oak_slab();
        let slab_kind = map.get("type")?;
        let slab_kind = SlabKind::from_str(slab_kind).ok()?;
        block.set_slab_kind(slab_kind);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn stone_slab_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::stone_slab();
        let slab_kind = map.get("type")?;
        let slab_kind = SlabKind::from_str(slab_kind).ok()?;
        block.set_slab_kind(slab_kind);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn sandstone_slab_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::sandstone_slab();
        let slab_kind = map.get("type")?;
        let slab_kind = SlabKind::from_str(slab_kind).ok()?;
        block.set_slab_kind(slab_kind);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn petrified_oak_slab_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::petrified_oak_slab();
        let slab_kind = map.get("type")?;
        let slab_kind = SlabKind::from_str(slab_kind).ok()?;
        block.set_slab_kind(slab_kind);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn cobblestone_slab_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::cobblestone_slab();
        let slab_kind = map.get("type")?;
        let slab_kind = SlabKind::from_str(slab_kind).ok()?;
        block.set_slab_kind(slab_kind);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn brick_slab_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::brick_slab();
        let slab_kind = map.get("type")?;
        let slab_kind = SlabKind::from_str(slab_kind).ok()?;
        block.set_slab_kind(slab_kind);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn stone_brick_slab_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::stone_brick_slab();
        let slab_kind = map.get("type")?;
        let slab_kind = SlabKind::from_str(slab_kind).ok()?;
        block.set_slab_kind(slab_kind);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn nether_brick_slab_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::nether_brick_slab();
        let slab_kind = map.get("type")?;
        let slab_kind = SlabKind::from_str(slab_kind).ok()?;
        block.set_slab_kind(slab_kind);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn quartz_slab_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::quartz_slab();
        let slab_kind = map.get("type")?;
        let slab_kind = SlabKind::from_str(slab_kind).ok()?;
        block.set_slab_kind(slab_kind);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn red_sandstone_slab_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::red_sandstone_slab();
        let slab_kind = map.get("type")?;
        let slab_kind = SlabKind::from_str(slab_kind).ok()?;
        block.set_slab_kind(slab_kind);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn purpur_slab_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::purpur_slab();
        let slab_kind = map.get("type")?;
        let slab_kind = SlabKind::from_str(slab_kind).ok()?;
        block.set_slab_kind(slab_kind);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn smooth_stone_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::smooth_stone();
        Some(block)
    }
    fn smooth_sandstone_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::smooth_sandstone();
        Some(block)
    }
    fn smooth_quartz_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::smooth_quartz();
        Some(block)
    }
    fn smooth_red_sandstone_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::smooth_red_sandstone();
        Some(block)
    }
    fn spruce_fence_gate_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::spruce_fence_gate();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let in_wall = map.get("in_wall")?;
        let in_wall = bool::from_str(in_wall).ok()?;
        block.set_in_wall(in_wall);
        let open = map.get("open")?;
        let open = bool::from_str(open).ok()?;
        block.set_open(open);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn birch_fence_gate_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::birch_fence_gate();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let in_wall = map.get("in_wall")?;
        let in_wall = bool::from_str(in_wall).ok()?;
        block.set_in_wall(in_wall);
        let open = map.get("open")?;
        let open = bool::from_str(open).ok()?;
        block.set_open(open);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn jungle_fence_gate_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::jungle_fence_gate();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let in_wall = map.get("in_wall")?;
        let in_wall = bool::from_str(in_wall).ok()?;
        block.set_in_wall(in_wall);
        let open = map.get("open")?;
        let open = bool::from_str(open).ok()?;
        block.set_open(open);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn acacia_fence_gate_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::acacia_fence_gate();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let in_wall = map.get("in_wall")?;
        let in_wall = bool::from_str(in_wall).ok()?;
        block.set_in_wall(in_wall);
        let open = map.get("open")?;
        let open = bool::from_str(open).ok()?;
        block.set_open(open);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn dark_oak_fence_gate_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dark_oak_fence_gate();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let in_wall = map.get("in_wall")?;
        let in_wall = bool::from_str(in_wall).ok()?;
        block.set_in_wall(in_wall);
        let open = map.get("open")?;
        let open = bool::from_str(open).ok()?;
        block.set_open(open);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn spruce_fence_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::spruce_fence();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn birch_fence_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::birch_fence();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn jungle_fence_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::jungle_fence();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn acacia_fence_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::acacia_fence();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn dark_oak_fence_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dark_oak_fence();
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn spruce_door_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::spruce_door();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_upper_lower = map.get("half")?;
        let half_upper_lower = HalfUpperLower::from_str(half_upper_lower).ok()?;
        block.set_half_upper_lower(half_upper_lower);
        let hinge = map.get("hinge")?;
        let hinge = Hinge::from_str(hinge).ok()?;
        block.set_hinge(hinge);
        let open = map.get("open")?;
        let open = bool::from_str(open).ok()?;
        block.set_open(open);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn birch_door_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::birch_door();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_upper_lower = map.get("half")?;
        let half_upper_lower = HalfUpperLower::from_str(half_upper_lower).ok()?;
        block.set_half_upper_lower(half_upper_lower);
        let hinge = map.get("hinge")?;
        let hinge = Hinge::from_str(hinge).ok()?;
        block.set_hinge(hinge);
        let open = map.get("open")?;
        let open = bool::from_str(open).ok()?;
        block.set_open(open);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn jungle_door_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::jungle_door();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_upper_lower = map.get("half")?;
        let half_upper_lower = HalfUpperLower::from_str(half_upper_lower).ok()?;
        block.set_half_upper_lower(half_upper_lower);
        let hinge = map.get("hinge")?;
        let hinge = Hinge::from_str(hinge).ok()?;
        block.set_hinge(hinge);
        let open = map.get("open")?;
        let open = bool::from_str(open).ok()?;
        block.set_open(open);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn acacia_door_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::acacia_door();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_upper_lower = map.get("half")?;
        let half_upper_lower = HalfUpperLower::from_str(half_upper_lower).ok()?;
        block.set_half_upper_lower(half_upper_lower);
        let hinge = map.get("hinge")?;
        let hinge = Hinge::from_str(hinge).ok()?;
        block.set_hinge(hinge);
        let open = map.get("open")?;
        let open = bool::from_str(open).ok()?;
        block.set_open(open);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn dark_oak_door_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dark_oak_door();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_upper_lower = map.get("half")?;
        let half_upper_lower = HalfUpperLower::from_str(half_upper_lower).ok()?;
        block.set_half_upper_lower(half_upper_lower);
        let hinge = map.get("hinge")?;
        let hinge = Hinge::from_str(hinge).ok()?;
        block.set_hinge(hinge);
        let open = map.get("open")?;
        let open = bool::from_str(open).ok()?;
        block.set_open(open);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn end_rod_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::end_rod();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn chorus_plant_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::chorus_plant();
        let down = map.get("down")?;
        let down = bool::from_str(down).ok()?;
        block.set_down(down);
        let east_connected = map.get("east")?;
        let east_connected = bool::from_str(east_connected).ok()?;
        block.set_east_connected(east_connected);
        let north_connected = map.get("north")?;
        let north_connected = bool::from_str(north_connected).ok()?;
        block.set_north_connected(north_connected);
        let south_connected = map.get("south")?;
        let south_connected = bool::from_str(south_connected).ok()?;
        block.set_south_connected(south_connected);
        let up = map.get("up")?;
        let up = bool::from_str(up).ok()?;
        block.set_up(up);
        let west_connected = map.get("west")?;
        let west_connected = bool::from_str(west_connected).ok()?;
        block.set_west_connected(west_connected);
        Some(block)
    }
    fn chorus_flower_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::chorus_flower();
        let age_0_5 = map.get("age")?;
        let age_0_5 = {
            let x = i32::from_str(age_0_5).ok()?;
            if !(0i32..=5i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_age_0_5(age_0_5);
        Some(block)
    }
    fn purpur_block_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::purpur_block();
        Some(block)
    }
    fn purpur_pillar_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::purpur_pillar();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn purpur_stairs_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::purpur_stairs();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let half_top_bottom = map.get("half")?;
        let half_top_bottom = HalfTopBottom::from_str(half_top_bottom).ok()?;
        block.set_half_top_bottom(half_top_bottom);
        let stairs_shape = map.get("shape")?;
        let stairs_shape = StairsShape::from_str(stairs_shape).ok()?;
        block.set_stairs_shape(stairs_shape);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn end_stone_bricks_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::end_stone_bricks();
        Some(block)
    }
    fn beetroots_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::beetroots();
        let age_0_3 = map.get("age")?;
        let age_0_3 = {
            let x = i32::from_str(age_0_3).ok()?;
            if !(0i32..=3i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_age_0_3(age_0_3);
        Some(block)
    }
    fn grass_path_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::grass_path();
        Some(block)
    }
    fn end_gateway_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::end_gateway();
        Some(block)
    }
    fn repeating_command_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::repeating_command_block();
        let conditional = map.get("conditional")?;
        let conditional = bool::from_str(conditional).ok()?;
        block.set_conditional(conditional);
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn chain_command_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::chain_command_block();
        let conditional = map.get("conditional")?;
        let conditional = bool::from_str(conditional).ok()?;
        block.set_conditional(conditional);
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn frosted_ice_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::frosted_ice();
        let age_0_3 = map.get("age")?;
        let age_0_3 = {
            let x = i32::from_str(age_0_3).ok()?;
            if !(0i32..=3i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_age_0_3(age_0_3);
        Some(block)
    }
    fn magma_block_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::magma_block();
        Some(block)
    }
    fn nether_wart_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::nether_wart_block();
        Some(block)
    }
    fn red_nether_bricks_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::red_nether_bricks();
        Some(block)
    }
    fn bone_block_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::bone_block();
        let axis_xyz = map.get("axis")?;
        let axis_xyz = AxisXyz::from_str(axis_xyz).ok()?;
        block.set_axis_xyz(axis_xyz);
        Some(block)
    }
    fn structure_void_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::structure_void();
        Some(block)
    }
    fn observer_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::observer();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        let powered = map.get("powered")?;
        let powered = bool::from_str(powered).ok()?;
        block.set_powered(powered);
        Some(block)
    }
    fn shulker_box_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::shulker_box();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn white_shulker_box_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::white_shulker_box();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn orange_shulker_box_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::orange_shulker_box();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn magenta_shulker_box_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::magenta_shulker_box();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn light_blue_shulker_box_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_blue_shulker_box();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn yellow_shulker_box_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::yellow_shulker_box();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn lime_shulker_box_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::lime_shulker_box();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn pink_shulker_box_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::pink_shulker_box();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn gray_shulker_box_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::gray_shulker_box();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn light_gray_shulker_box_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_gray_shulker_box();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn cyan_shulker_box_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::cyan_shulker_box();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn purple_shulker_box_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::purple_shulker_box();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn blue_shulker_box_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::blue_shulker_box();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn brown_shulker_box_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::brown_shulker_box();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn green_shulker_box_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::green_shulker_box();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn red_shulker_box_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::red_shulker_box();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn black_shulker_box_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::black_shulker_box();
        let facing_cubic = map.get("facing")?;
        let facing_cubic = FacingCubic::from_str(facing_cubic).ok()?;
        block.set_facing_cubic(facing_cubic);
        Some(block)
    }
    fn white_glazed_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::white_glazed_terracotta();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn orange_glazed_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::orange_glazed_terracotta();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn magenta_glazed_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::magenta_glazed_terracotta();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn light_blue_glazed_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_blue_glazed_terracotta();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn yellow_glazed_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::yellow_glazed_terracotta();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn lime_glazed_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::lime_glazed_terracotta();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn pink_glazed_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::pink_glazed_terracotta();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn gray_glazed_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::gray_glazed_terracotta();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn light_gray_glazed_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_gray_glazed_terracotta();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn cyan_glazed_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::cyan_glazed_terracotta();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn purple_glazed_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::purple_glazed_terracotta();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn blue_glazed_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::blue_glazed_terracotta();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn brown_glazed_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::brown_glazed_terracotta();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn green_glazed_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::green_glazed_terracotta();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn red_glazed_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::red_glazed_terracotta();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn black_glazed_terracotta_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::black_glazed_terracotta();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        Some(block)
    }
    fn white_concrete_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::white_concrete();
        Some(block)
    }
    fn orange_concrete_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::orange_concrete();
        Some(block)
    }
    fn magenta_concrete_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::magenta_concrete();
        Some(block)
    }
    fn light_blue_concrete_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_blue_concrete();
        Some(block)
    }
    fn yellow_concrete_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::yellow_concrete();
        Some(block)
    }
    fn lime_concrete_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::lime_concrete();
        Some(block)
    }
    fn pink_concrete_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::pink_concrete();
        Some(block)
    }
    fn gray_concrete_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::gray_concrete();
        Some(block)
    }
    fn light_gray_concrete_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_gray_concrete();
        Some(block)
    }
    fn cyan_concrete_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::cyan_concrete();
        Some(block)
    }
    fn purple_concrete_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::purple_concrete();
        Some(block)
    }
    fn blue_concrete_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::blue_concrete();
        Some(block)
    }
    fn brown_concrete_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::brown_concrete();
        Some(block)
    }
    fn green_concrete_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::green_concrete();
        Some(block)
    }
    fn red_concrete_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::red_concrete();
        Some(block)
    }
    fn black_concrete_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::black_concrete();
        Some(block)
    }
    fn white_concrete_powder_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::white_concrete_powder();
        Some(block)
    }
    fn orange_concrete_powder_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::orange_concrete_powder();
        Some(block)
    }
    fn magenta_concrete_powder_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::magenta_concrete_powder();
        Some(block)
    }
    fn light_blue_concrete_powder_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_blue_concrete_powder();
        Some(block)
    }
    fn yellow_concrete_powder_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::yellow_concrete_powder();
        Some(block)
    }
    fn lime_concrete_powder_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::lime_concrete_powder();
        Some(block)
    }
    fn pink_concrete_powder_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::pink_concrete_powder();
        Some(block)
    }
    fn gray_concrete_powder_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::gray_concrete_powder();
        Some(block)
    }
    fn light_gray_concrete_powder_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::light_gray_concrete_powder();
        Some(block)
    }
    fn cyan_concrete_powder_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::cyan_concrete_powder();
        Some(block)
    }
    fn purple_concrete_powder_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::purple_concrete_powder();
        Some(block)
    }
    fn blue_concrete_powder_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::blue_concrete_powder();
        Some(block)
    }
    fn brown_concrete_powder_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::brown_concrete_powder();
        Some(block)
    }
    fn green_concrete_powder_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::green_concrete_powder();
        Some(block)
    }
    fn red_concrete_powder_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::red_concrete_powder();
        Some(block)
    }
    fn black_concrete_powder_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::black_concrete_powder();
        Some(block)
    }
    fn kelp_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::kelp();
        let age_0_25 = map.get("age")?;
        let age_0_25 = {
            let x = i32::from_str(age_0_25).ok()?;
            if !(0i32..=25i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_age_0_25(age_0_25);
        Some(block)
    }
    fn kelp_plant_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::kelp_plant();
        Some(block)
    }
    fn dried_kelp_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dried_kelp_block();
        Some(block)
    }
    fn turtle_egg_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::turtle_egg();
        let eggs = map.get("eggs")?;
        let eggs = {
            let x = i32::from_str(eggs).ok()?;
            if !(1i32..=4i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_eggs(eggs);
        let hatch = map.get("hatch")?;
        let hatch = {
            let x = i32::from_str(hatch).ok()?;
            if !(0i32..=2i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_hatch(hatch);
        Some(block)
    }
    fn dead_tube_coral_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dead_tube_coral_block();
        Some(block)
    }
    fn dead_brain_coral_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dead_brain_coral_block();
        Some(block)
    }
    fn dead_bubble_coral_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dead_bubble_coral_block();
        Some(block)
    }
    fn dead_fire_coral_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dead_fire_coral_block();
        Some(block)
    }
    fn dead_horn_coral_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dead_horn_coral_block();
        Some(block)
    }
    fn tube_coral_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::tube_coral_block();
        Some(block)
    }
    fn brain_coral_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::brain_coral_block();
        Some(block)
    }
    fn bubble_coral_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::bubble_coral_block();
        Some(block)
    }
    fn fire_coral_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::fire_coral_block();
        Some(block)
    }
    fn horn_coral_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::horn_coral_block();
        Some(block)
    }
    fn dead_tube_coral_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dead_tube_coral();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn dead_brain_coral_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dead_brain_coral();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn dead_bubble_coral_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dead_bubble_coral();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn dead_fire_coral_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dead_fire_coral();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn dead_horn_coral_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dead_horn_coral();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn tube_coral_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::tube_coral();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn brain_coral_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::brain_coral();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn bubble_coral_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::bubble_coral();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn fire_coral_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::fire_coral();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn horn_coral_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::horn_coral();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn dead_tube_coral_wall_fan_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dead_tube_coral_wall_fan();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn dead_brain_coral_wall_fan_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dead_brain_coral_wall_fan();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn dead_bubble_coral_wall_fan_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dead_bubble_coral_wall_fan();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn dead_fire_coral_wall_fan_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dead_fire_coral_wall_fan();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn dead_horn_coral_wall_fan_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dead_horn_coral_wall_fan();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn tube_coral_wall_fan_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::tube_coral_wall_fan();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn brain_coral_wall_fan_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::brain_coral_wall_fan();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn bubble_coral_wall_fan_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::bubble_coral_wall_fan();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn fire_coral_wall_fan_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::fire_coral_wall_fan();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn horn_coral_wall_fan_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::horn_coral_wall_fan();
        let facing_cardinal = map.get("facing")?;
        let facing_cardinal = FacingCardinal::from_str(facing_cardinal).ok()?;
        block.set_facing_cardinal(facing_cardinal);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn dead_tube_coral_fan_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dead_tube_coral_fan();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn dead_brain_coral_fan_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dead_brain_coral_fan();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn dead_bubble_coral_fan_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dead_bubble_coral_fan();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn dead_fire_coral_fan_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dead_fire_coral_fan();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn dead_horn_coral_fan_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::dead_horn_coral_fan();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn tube_coral_fan_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::tube_coral_fan();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn brain_coral_fan_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::brain_coral_fan();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn bubble_coral_fan_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::bubble_coral_fan();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn fire_coral_fan_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::fire_coral_fan();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn horn_coral_fan_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::horn_coral_fan();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn sea_pickle_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::sea_pickle();
        let pickles = map.get("pickles")?;
        let pickles = {
            let x = i32::from_str(pickles).ok()?;
            if !(1i32..=4i32).contains(&x) {
                return None;
            }
            x
        };
        block.set_pickles(pickles);
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn blue_ice_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::blue_ice();
        Some(block)
    }
    fn conduit_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::conduit();
        let waterlogged = map.get("waterlogged")?;
        let waterlogged = bool::from_str(waterlogged).ok()?;
        block.set_waterlogged(waterlogged);
        Some(block)
    }
    fn void_air_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::void_air();
        Some(block)
    }
    fn cave_air_from_identifier_and_properties(map: &BTreeMap<String, String>) -> Option<Self> {
        let mut block = BlockId::cave_air();
        Some(block)
    }
    fn bubble_column_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::bubble_column();
        let drag = map.get("drag")?;
        let drag = bool::from_str(drag).ok()?;
        block.set_drag(drag);
        Some(block)
    }
    fn structure_block_from_identifier_and_properties(
        map: &BTreeMap<String, String>,
    ) -> Option<Self> {
        let mut block = BlockId::structure_block();
        let structure_block_mode = map.get("mode")?;
        let structure_block_mode = StructureBlockMode::from_str(structure_block_mode).ok()?;
        block.set_structure_block_mode(structure_block_mode);
        Some(block)
    }
    #[doc = "Attempts to convert a block identifier to a block with default property values."]
    pub fn from_identifier(identifier: &str) -> Option<Self> {
        match identifier {
            "minecraft:air" => Some(Self::air()),
            "minecraft:stone" => Some(Self::stone()),
            "minecraft:granite" => Some(Self::granite()),
            "minecraft:polished_granite" => Some(Self::polished_granite()),
            "minecraft:diorite" => Some(Self::diorite()),
            "minecraft:polished_diorite" => Some(Self::polished_diorite()),
            "minecraft:andesite" => Some(Self::andesite()),
            "minecraft:polished_andesite" => Some(Self::polished_andesite()),
            "minecraft:grass_block" => Some(Self::grass_block()),
            "minecraft:dirt" => Some(Self::dirt()),
            "minecraft:coarse_dirt" => Some(Self::coarse_dirt()),
            "minecraft:podzol" => Some(Self::podzol()),
            "minecraft:cobblestone" => Some(Self::cobblestone()),
            "minecraft:oak_planks" => Some(Self::oak_planks()),
            "minecraft:spruce_planks" => Some(Self::spruce_planks()),
            "minecraft:birch_planks" => Some(Self::birch_planks()),
            "minecraft:jungle_planks" => Some(Self::jungle_planks()),
            "minecraft:acacia_planks" => Some(Self::acacia_planks()),
            "minecraft:dark_oak_planks" => Some(Self::dark_oak_planks()),
            "minecraft:oak_sapling" => Some(Self::oak_sapling()),
            "minecraft:spruce_sapling" => Some(Self::spruce_sapling()),
            "minecraft:birch_sapling" => Some(Self::birch_sapling()),
            "minecraft:jungle_sapling" => Some(Self::jungle_sapling()),
            "minecraft:acacia_sapling" => Some(Self::acacia_sapling()),
            "minecraft:dark_oak_sapling" => Some(Self::dark_oak_sapling()),
            "minecraft:bedrock" => Some(Self::bedrock()),
            "minecraft:water" => Some(Self::water()),
            "minecraft:lava" => Some(Self::lava()),
            "minecraft:sand" => Some(Self::sand()),
            "minecraft:red_sand" => Some(Self::red_sand()),
            "minecraft:gravel" => Some(Self::gravel()),
            "minecraft:gold_ore" => Some(Self::gold_ore()),
            "minecraft:iron_ore" => Some(Self::iron_ore()),
            "minecraft:coal_ore" => Some(Self::coal_ore()),
            "minecraft:oak_log" => Some(Self::oak_log()),
            "minecraft:spruce_log" => Some(Self::spruce_log()),
            "minecraft:birch_log" => Some(Self::birch_log()),
            "minecraft:jungle_log" => Some(Self::jungle_log()),
            "minecraft:acacia_log" => Some(Self::acacia_log()),
            "minecraft:dark_oak_log" => Some(Self::dark_oak_log()),
            "minecraft:stripped_spruce_log" => Some(Self::stripped_spruce_log()),
            "minecraft:stripped_birch_log" => Some(Self::stripped_birch_log()),
            "minecraft:stripped_jungle_log" => Some(Self::stripped_jungle_log()),
            "minecraft:stripped_acacia_log" => Some(Self::stripped_acacia_log()),
            "minecraft:stripped_dark_oak_log" => Some(Self::stripped_dark_oak_log()),
            "minecraft:stripped_oak_log" => Some(Self::stripped_oak_log()),
            "minecraft:oak_wood" => Some(Self::oak_wood()),
            "minecraft:spruce_wood" => Some(Self::spruce_wood()),
            "minecraft:birch_wood" => Some(Self::birch_wood()),
            "minecraft:jungle_wood" => Some(Self::jungle_wood()),
            "minecraft:acacia_wood" => Some(Self::acacia_wood()),
            "minecraft:dark_oak_wood" => Some(Self::dark_oak_wood()),
            "minecraft:stripped_oak_wood" => Some(Self::stripped_oak_wood()),
            "minecraft:stripped_spruce_wood" => Some(Self::stripped_spruce_wood()),
            "minecraft:stripped_birch_wood" => Some(Self::stripped_birch_wood()),
            "minecraft:stripped_jungle_wood" => Some(Self::stripped_jungle_wood()),
            "minecraft:stripped_acacia_wood" => Some(Self::stripped_acacia_wood()),
            "minecraft:stripped_dark_oak_wood" => Some(Self::stripped_dark_oak_wood()),
            "minecraft:oak_leaves" => Some(Self::oak_leaves()),
            "minecraft:spruce_leaves" => Some(Self::spruce_leaves()),
            "minecraft:birch_leaves" => Some(Self::birch_leaves()),
            "minecraft:jungle_leaves" => Some(Self::jungle_leaves()),
            "minecraft:acacia_leaves" => Some(Self::acacia_leaves()),
            "minecraft:dark_oak_leaves" => Some(Self::dark_oak_leaves()),
            "minecraft:sponge" => Some(Self::sponge()),
            "minecraft:wet_sponge" => Some(Self::wet_sponge()),
            "minecraft:glass" => Some(Self::glass()),
            "minecraft:lapis_ore" => Some(Self::lapis_ore()),
            "minecraft:lapis_block" => Some(Self::lapis_block()),
            "minecraft:dispenser" => Some(Self::dispenser()),
            "minecraft:sandstone" => Some(Self::sandstone()),
            "minecraft:chiseled_sandstone" => Some(Self::chiseled_sandstone()),
            "minecraft:cut_sandstone" => Some(Self::cut_sandstone()),
            "minecraft:note_block" => Some(Self::note_block()),
            "minecraft:white_bed" => Some(Self::white_bed()),
            "minecraft:orange_bed" => Some(Self::orange_bed()),
            "minecraft:magenta_bed" => Some(Self::magenta_bed()),
            "minecraft:light_blue_bed" => Some(Self::light_blue_bed()),
            "minecraft:yellow_bed" => Some(Self::yellow_bed()),
            "minecraft:lime_bed" => Some(Self::lime_bed()),
            "minecraft:pink_bed" => Some(Self::pink_bed()),
            "minecraft:gray_bed" => Some(Self::gray_bed()),
            "minecraft:light_gray_bed" => Some(Self::light_gray_bed()),
            "minecraft:cyan_bed" => Some(Self::cyan_bed()),
            "minecraft:purple_bed" => Some(Self::purple_bed()),
            "minecraft:blue_bed" => Some(Self::blue_bed()),
            "minecraft:brown_bed" => Some(Self::brown_bed()),
            "minecraft:green_bed" => Some(Self::green_bed()),
            "minecraft:red_bed" => Some(Self::red_bed()),
            "minecraft:black_bed" => Some(Self::black_bed()),
            "minecraft:powered_rail" => Some(Self::powered_rail()),
            "minecraft:detector_rail" => Some(Self::detector_rail()),
            "minecraft:sticky_piston" => Some(Self::sticky_piston()),
            "minecraft:cobweb" => Some(Self::cobweb()),
            "minecraft:grass" => Some(Self::grass()),
            "minecraft:fern" => Some(Self::fern()),
            "minecraft:dead_bush" => Some(Self::dead_bush()),
            "minecraft:seagrass" => Some(Self::seagrass()),
            "minecraft:tall_seagrass" => Some(Self::tall_seagrass()),
            "minecraft:piston" => Some(Self::piston()),
            "minecraft:piston_head" => Some(Self::piston_head()),
            "minecraft:white_wool" => Some(Self::white_wool()),
            "minecraft:orange_wool" => Some(Self::orange_wool()),
            "minecraft:magenta_wool" => Some(Self::magenta_wool()),
            "minecraft:light_blue_wool" => Some(Self::light_blue_wool()),
            "minecraft:yellow_wool" => Some(Self::yellow_wool()),
            "minecraft:lime_wool" => Some(Self::lime_wool()),
            "minecraft:pink_wool" => Some(Self::pink_wool()),
            "minecraft:gray_wool" => Some(Self::gray_wool()),
            "minecraft:light_gray_wool" => Some(Self::light_gray_wool()),
            "minecraft:cyan_wool" => Some(Self::cyan_wool()),
            "minecraft:purple_wool" => Some(Self::purple_wool()),
            "minecraft:blue_wool" => Some(Self::blue_wool()),
            "minecraft:brown_wool" => Some(Self::brown_wool()),
            "minecraft:green_wool" => Some(Self::green_wool()),
            "minecraft:red_wool" => Some(Self::red_wool()),
            "minecraft:black_wool" => Some(Self::black_wool()),
            "minecraft:moving_piston" => Some(Self::moving_piston()),
            "minecraft:dandelion" => Some(Self::dandelion()),
            "minecraft:poppy" => Some(Self::poppy()),
            "minecraft:blue_orchid" => Some(Self::blue_orchid()),
            "minecraft:allium" => Some(Self::allium()),
            "minecraft:azure_bluet" => Some(Self::azure_bluet()),
            "minecraft:red_tulip" => Some(Self::red_tulip()),
            "minecraft:orange_tulip" => Some(Self::orange_tulip()),
            "minecraft:white_tulip" => Some(Self::white_tulip()),
            "minecraft:pink_tulip" => Some(Self::pink_tulip()),
            "minecraft:oxeye_daisy" => Some(Self::oxeye_daisy()),
            "minecraft:brown_mushroom" => Some(Self::brown_mushroom()),
            "minecraft:red_mushroom" => Some(Self::red_mushroom()),
            "minecraft:gold_block" => Some(Self::gold_block()),
            "minecraft:iron_block" => Some(Self::iron_block()),
            "minecraft:bricks" => Some(Self::bricks()),
            "minecraft:tnt" => Some(Self::tnt()),
            "minecraft:bookshelf" => Some(Self::bookshelf()),
            "minecraft:mossy_cobblestone" => Some(Self::mossy_cobblestone()),
            "minecraft:obsidian" => Some(Self::obsidian()),
            "minecraft:torch" => Some(Self::torch()),
            "minecraft:wall_torch" => Some(Self::wall_torch()),
            "minecraft:fire" => Some(Self::fire()),
            "minecraft:spawner" => Some(Self::spawner()),
            "minecraft:oak_stairs" => Some(Self::oak_stairs()),
            "minecraft:chest" => Some(Self::chest()),
            "minecraft:redstone_wire" => Some(Self::redstone_wire()),
            "minecraft:diamond_ore" => Some(Self::diamond_ore()),
            "minecraft:diamond_block" => Some(Self::diamond_block()),
            "minecraft:crafting_table" => Some(Self::crafting_table()),
            "minecraft:wheat" => Some(Self::wheat()),
            "minecraft:farmland" => Some(Self::farmland()),
            "minecraft:furnace" => Some(Self::furnace()),
            "minecraft:sign" => Some(Self::sign()),
            "minecraft:oak_door" => Some(Self::oak_door()),
            "minecraft:ladder" => Some(Self::ladder()),
            "minecraft:rail" => Some(Self::rail()),
            "minecraft:cobblestone_stairs" => Some(Self::cobblestone_stairs()),
            "minecraft:wall_sign" => Some(Self::wall_sign()),
            "minecraft:lever" => Some(Self::lever()),
            "minecraft:stone_pressure_plate" => Some(Self::stone_pressure_plate()),
            "minecraft:iron_door" => Some(Self::iron_door()),
            "minecraft:oak_pressure_plate" => Some(Self::oak_pressure_plate()),
            "minecraft:spruce_pressure_plate" => Some(Self::spruce_pressure_plate()),
            "minecraft:birch_pressure_plate" => Some(Self::birch_pressure_plate()),
            "minecraft:jungle_pressure_plate" => Some(Self::jungle_pressure_plate()),
            "minecraft:acacia_pressure_plate" => Some(Self::acacia_pressure_plate()),
            "minecraft:dark_oak_pressure_plate" => Some(Self::dark_oak_pressure_plate()),
            "minecraft:redstone_ore" => Some(Self::redstone_ore()),
            "minecraft:redstone_torch" => Some(Self::redstone_torch()),
            "minecraft:redstone_wall_torch" => Some(Self::redstone_wall_torch()),
            "minecraft:stone_button" => Some(Self::stone_button()),
            "minecraft:snow" => Some(Self::snow()),
            "minecraft:ice" => Some(Self::ice()),
            "minecraft:snow_block" => Some(Self::snow_block()),
            "minecraft:cactus" => Some(Self::cactus()),
            "minecraft:clay" => Some(Self::clay()),
            "minecraft:sugar_cane" => Some(Self::sugar_cane()),
            "minecraft:jukebox" => Some(Self::jukebox()),
            "minecraft:oak_fence" => Some(Self::oak_fence()),
            "minecraft:pumpkin" => Some(Self::pumpkin()),
            "minecraft:netherrack" => Some(Self::netherrack()),
            "minecraft:soul_sand" => Some(Self::soul_sand()),
            "minecraft:glowstone" => Some(Self::glowstone()),
            "minecraft:nether_portal" => Some(Self::nether_portal()),
            "minecraft:carved_pumpkin" => Some(Self::carved_pumpkin()),
            "minecraft:jack_o_lantern" => Some(Self::jack_o_lantern()),
            "minecraft:cake" => Some(Self::cake()),
            "minecraft:repeater" => Some(Self::repeater()),
            "minecraft:white_stained_glass" => Some(Self::white_stained_glass()),
            "minecraft:orange_stained_glass" => Some(Self::orange_stained_glass()),
            "minecraft:magenta_stained_glass" => Some(Self::magenta_stained_glass()),
            "minecraft:light_blue_stained_glass" => Some(Self::light_blue_stained_glass()),
            "minecraft:yellow_stained_glass" => Some(Self::yellow_stained_glass()),
            "minecraft:lime_stained_glass" => Some(Self::lime_stained_glass()),
            "minecraft:pink_stained_glass" => Some(Self::pink_stained_glass()),
            "minecraft:gray_stained_glass" => Some(Self::gray_stained_glass()),
            "minecraft:light_gray_stained_glass" => Some(Self::light_gray_stained_glass()),
            "minecraft:cyan_stained_glass" => Some(Self::cyan_stained_glass()),
            "minecraft:purple_stained_glass" => Some(Self::purple_stained_glass()),
            "minecraft:blue_stained_glass" => Some(Self::blue_stained_glass()),
            "minecraft:brown_stained_glass" => Some(Self::brown_stained_glass()),
            "minecraft:green_stained_glass" => Some(Self::green_stained_glass()),
            "minecraft:red_stained_glass" => Some(Self::red_stained_glass()),
            "minecraft:black_stained_glass" => Some(Self::black_stained_glass()),
            "minecraft:oak_trapdoor" => Some(Self::oak_trapdoor()),
            "minecraft:spruce_trapdoor" => Some(Self::spruce_trapdoor()),
            "minecraft:birch_trapdoor" => Some(Self::birch_trapdoor()),
            "minecraft:jungle_trapdoor" => Some(Self::jungle_trapdoor()),
            "minecraft:acacia_trapdoor" => Some(Self::acacia_trapdoor()),
            "minecraft:dark_oak_trapdoor" => Some(Self::dark_oak_trapdoor()),
            "minecraft:infested_stone" => Some(Self::infested_stone()),
            "minecraft:infested_cobblestone" => Some(Self::infested_cobblestone()),
            "minecraft:infested_stone_bricks" => Some(Self::infested_stone_bricks()),
            "minecraft:infested_mossy_stone_bricks" => Some(Self::infested_mossy_stone_bricks()),
            "minecraft:infested_cracked_stone_bricks" => {
                Some(Self::infested_cracked_stone_bricks())
            }
            "minecraft:infested_chiseled_stone_bricks" => {
                Some(Self::infested_chiseled_stone_bricks())
            }
            "minecraft:stone_bricks" => Some(Self::stone_bricks()),
            "minecraft:mossy_stone_bricks" => Some(Self::mossy_stone_bricks()),
            "minecraft:cracked_stone_bricks" => Some(Self::cracked_stone_bricks()),
            "minecraft:chiseled_stone_bricks" => Some(Self::chiseled_stone_bricks()),
            "minecraft:brown_mushroom_block" => Some(Self::brown_mushroom_block()),
            "minecraft:red_mushroom_block" => Some(Self::red_mushroom_block()),
            "minecraft:mushroom_stem" => Some(Self::mushroom_stem()),
            "minecraft:iron_bars" => Some(Self::iron_bars()),
            "minecraft:glass_pane" => Some(Self::glass_pane()),
            "minecraft:melon" => Some(Self::melon()),
            "minecraft:attached_pumpkin_stem" => Some(Self::attached_pumpkin_stem()),
            "minecraft:attached_melon_stem" => Some(Self::attached_melon_stem()),
            "minecraft:pumpkin_stem" => Some(Self::pumpkin_stem()),
            "minecraft:melon_stem" => Some(Self::melon_stem()),
            "minecraft:vine" => Some(Self::vine()),
            "minecraft:oak_fence_gate" => Some(Self::oak_fence_gate()),
            "minecraft:brick_stairs" => Some(Self::brick_stairs()),
            "minecraft:stone_brick_stairs" => Some(Self::stone_brick_stairs()),
            "minecraft:mycelium" => Some(Self::mycelium()),
            "minecraft:lily_pad" => Some(Self::lily_pad()),
            "minecraft:nether_bricks" => Some(Self::nether_bricks()),
            "minecraft:nether_brick_fence" => Some(Self::nether_brick_fence()),
            "minecraft:nether_brick_stairs" => Some(Self::nether_brick_stairs()),
            "minecraft:nether_wart" => Some(Self::nether_wart()),
            "minecraft:enchanting_table" => Some(Self::enchanting_table()),
            "minecraft:brewing_stand" => Some(Self::brewing_stand()),
            "minecraft:cauldron" => Some(Self::cauldron()),
            "minecraft:end_portal" => Some(Self::end_portal()),
            "minecraft:end_portal_frame" => Some(Self::end_portal_frame()),
            "minecraft:end_stone" => Some(Self::end_stone()),
            "minecraft:dragon_egg" => Some(Self::dragon_egg()),
            "minecraft:redstone_lamp" => Some(Self::redstone_lamp()),
            "minecraft:cocoa" => Some(Self::cocoa()),
            "minecraft:sandstone_stairs" => Some(Self::sandstone_stairs()),
            "minecraft:emerald_ore" => Some(Self::emerald_ore()),
            "minecraft:ender_chest" => Some(Self::ender_chest()),
            "minecraft:tripwire_hook" => Some(Self::tripwire_hook()),
            "minecraft:tripwire" => Some(Self::tripwire()),
            "minecraft:emerald_block" => Some(Self::emerald_block()),
            "minecraft:spruce_stairs" => Some(Self::spruce_stairs()),
            "minecraft:birch_stairs" => Some(Self::birch_stairs()),
            "minecraft:jungle_stairs" => Some(Self::jungle_stairs()),
            "minecraft:command_block" => Some(Self::command_block()),
            "minecraft:beacon" => Some(Self::beacon()),
            "minecraft:cobblestone_wall" => Some(Self::cobblestone_wall()),
            "minecraft:mossy_cobblestone_wall" => Some(Self::mossy_cobblestone_wall()),
            "minecraft:flower_pot" => Some(Self::flower_pot()),
            "minecraft:potted_oak_sapling" => Some(Self::potted_oak_sapling()),
            "minecraft:potted_spruce_sapling" => Some(Self::potted_spruce_sapling()),
            "minecraft:potted_birch_sapling" => Some(Self::potted_birch_sapling()),
            "minecraft:potted_jungle_sapling" => Some(Self::potted_jungle_sapling()),
            "minecraft:potted_acacia_sapling" => Some(Self::potted_acacia_sapling()),
            "minecraft:potted_dark_oak_sapling" => Some(Self::potted_dark_oak_sapling()),
            "minecraft:potted_fern" => Some(Self::potted_fern()),
            "minecraft:potted_dandelion" => Some(Self::potted_dandelion()),
            "minecraft:potted_poppy" => Some(Self::potted_poppy()),
            "minecraft:potted_blue_orchid" => Some(Self::potted_blue_orchid()),
            "minecraft:potted_allium" => Some(Self::potted_allium()),
            "minecraft:potted_azure_bluet" => Some(Self::potted_azure_bluet()),
            "minecraft:potted_red_tulip" => Some(Self::potted_red_tulip()),
            "minecraft:potted_orange_tulip" => Some(Self::potted_orange_tulip()),
            "minecraft:potted_white_tulip" => Some(Self::potted_white_tulip()),
            "minecraft:potted_pink_tulip" => Some(Self::potted_pink_tulip()),
            "minecraft:potted_oxeye_daisy" => Some(Self::potted_oxeye_daisy()),
            "minecraft:potted_red_mushroom" => Some(Self::potted_red_mushroom()),
            "minecraft:potted_brown_mushroom" => Some(Self::potted_brown_mushroom()),
            "minecraft:potted_dead_bush" => Some(Self::potted_dead_bush()),
            "minecraft:potted_cactus" => Some(Self::potted_cactus()),
            "minecraft:carrots" => Some(Self::carrots()),
            "minecraft:potatoes" => Some(Self::potatoes()),
            "minecraft:oak_button" => Some(Self::oak_button()),
            "minecraft:spruce_button" => Some(Self::spruce_button()),
            "minecraft:birch_button" => Some(Self::birch_button()),
            "minecraft:jungle_button" => Some(Self::jungle_button()),
            "minecraft:acacia_button" => Some(Self::acacia_button()),
            "minecraft:dark_oak_button" => Some(Self::dark_oak_button()),
            "minecraft:skeleton_wall_skull" => Some(Self::skeleton_wall_skull()),
            "minecraft:skeleton_skull" => Some(Self::skeleton_skull()),
            "minecraft:wither_skeleton_wall_skull" => Some(Self::wither_skeleton_wall_skull()),
            "minecraft:wither_skeleton_skull" => Some(Self::wither_skeleton_skull()),
            "minecraft:zombie_wall_head" => Some(Self::zombie_wall_head()),
            "minecraft:zombie_head" => Some(Self::zombie_head()),
            "minecraft:player_wall_head" => Some(Self::player_wall_head()),
            "minecraft:player_head" => Some(Self::player_head()),
            "minecraft:creeper_wall_head" => Some(Self::creeper_wall_head()),
            "minecraft:creeper_head" => Some(Self::creeper_head()),
            "minecraft:dragon_wall_head" => Some(Self::dragon_wall_head()),
            "minecraft:dragon_head" => Some(Self::dragon_head()),
            "minecraft:anvil" => Some(Self::anvil()),
            "minecraft:chipped_anvil" => Some(Self::chipped_anvil()),
            "minecraft:damaged_anvil" => Some(Self::damaged_anvil()),
            "minecraft:trapped_chest" => Some(Self::trapped_chest()),
            "minecraft:light_weighted_pressure_plate" => {
                Some(Self::light_weighted_pressure_plate())
            }
            "minecraft:heavy_weighted_pressure_plate" => {
                Some(Self::heavy_weighted_pressure_plate())
            }
            "minecraft:comparator" => Some(Self::comparator()),
            "minecraft:daylight_detector" => Some(Self::daylight_detector()),
            "minecraft:redstone_block" => Some(Self::redstone_block()),
            "minecraft:nether_quartz_ore" => Some(Self::nether_quartz_ore()),
            "minecraft:hopper" => Some(Self::hopper()),
            "minecraft:quartz_block" => Some(Self::quartz_block()),
            "minecraft:chiseled_quartz_block" => Some(Self::chiseled_quartz_block()),
            "minecraft:quartz_pillar" => Some(Self::quartz_pillar()),
            "minecraft:quartz_stairs" => Some(Self::quartz_stairs()),
            "minecraft:activator_rail" => Some(Self::activator_rail()),
            "minecraft:dropper" => Some(Self::dropper()),
            "minecraft:white_terracotta" => Some(Self::white_terracotta()),
            "minecraft:orange_terracotta" => Some(Self::orange_terracotta()),
            "minecraft:magenta_terracotta" => Some(Self::magenta_terracotta()),
            "minecraft:light_blue_terracotta" => Some(Self::light_blue_terracotta()),
            "minecraft:yellow_terracotta" => Some(Self::yellow_terracotta()),
            "minecraft:lime_terracotta" => Some(Self::lime_terracotta()),
            "minecraft:pink_terracotta" => Some(Self::pink_terracotta()),
            "minecraft:gray_terracotta" => Some(Self::gray_terracotta()),
            "minecraft:light_gray_terracotta" => Some(Self::light_gray_terracotta()),
            "minecraft:cyan_terracotta" => Some(Self::cyan_terracotta()),
            "minecraft:purple_terracotta" => Some(Self::purple_terracotta()),
            "minecraft:blue_terracotta" => Some(Self::blue_terracotta()),
            "minecraft:brown_terracotta" => Some(Self::brown_terracotta()),
            "minecraft:green_terracotta" => Some(Self::green_terracotta()),
            "minecraft:red_terracotta" => Some(Self::red_terracotta()),
            "minecraft:black_terracotta" => Some(Self::black_terracotta()),
            "minecraft:white_stained_glass_pane" => Some(Self::white_stained_glass_pane()),
            "minecraft:orange_stained_glass_pane" => Some(Self::orange_stained_glass_pane()),
            "minecraft:magenta_stained_glass_pane" => Some(Self::magenta_stained_glass_pane()),
            "minecraft:light_blue_stained_glass_pane" => {
                Some(Self::light_blue_stained_glass_pane())
            }
            "minecraft:yellow_stained_glass_pane" => Some(Self::yellow_stained_glass_pane()),
            "minecraft:lime_stained_glass_pane" => Some(Self::lime_stained_glass_pane()),
            "minecraft:pink_stained_glass_pane" => Some(Self::pink_stained_glass_pane()),
            "minecraft:gray_stained_glass_pane" => Some(Self::gray_stained_glass_pane()),
            "minecraft:light_gray_stained_glass_pane" => {
                Some(Self::light_gray_stained_glass_pane())
            }
            "minecraft:cyan_stained_glass_pane" => Some(Self::cyan_stained_glass_pane()),
            "minecraft:purple_stained_glass_pane" => Some(Self::purple_stained_glass_pane()),
            "minecraft:blue_stained_glass_pane" => Some(Self::blue_stained_glass_pane()),
            "minecraft:brown_stained_glass_pane" => Some(Self::brown_stained_glass_pane()),
            "minecraft:green_stained_glass_pane" => Some(Self::green_stained_glass_pane()),
            "minecraft:red_stained_glass_pane" => Some(Self::red_stained_glass_pane()),
            "minecraft:black_stained_glass_pane" => Some(Self::black_stained_glass_pane()),
            "minecraft:acacia_stairs" => Some(Self::acacia_stairs()),
            "minecraft:dark_oak_stairs" => Some(Self::dark_oak_stairs()),
            "minecraft:slime_block" => Some(Self::slime_block()),
            "minecraft:barrier" => Some(Self::barrier()),
            "minecraft:iron_trapdoor" => Some(Self::iron_trapdoor()),
            "minecraft:prismarine" => Some(Self::prismarine()),
            "minecraft:prismarine_bricks" => Some(Self::prismarine_bricks()),
            "minecraft:dark_prismarine" => Some(Self::dark_prismarine()),
            "minecraft:prismarine_stairs" => Some(Self::prismarine_stairs()),
            "minecraft:prismarine_brick_stairs" => Some(Self::prismarine_brick_stairs()),
            "minecraft:dark_prismarine_stairs" => Some(Self::dark_prismarine_stairs()),
            "minecraft:prismarine_slab" => Some(Self::prismarine_slab()),
            "minecraft:prismarine_brick_slab" => Some(Self::prismarine_brick_slab()),
            "minecraft:dark_prismarine_slab" => Some(Self::dark_prismarine_slab()),
            "minecraft:sea_lantern" => Some(Self::sea_lantern()),
            "minecraft:hay_block" => Some(Self::hay_block()),
            "minecraft:white_carpet" => Some(Self::white_carpet()),
            "minecraft:orange_carpet" => Some(Self::orange_carpet()),
            "minecraft:magenta_carpet" => Some(Self::magenta_carpet()),
            "minecraft:light_blue_carpet" => Some(Self::light_blue_carpet()),
            "minecraft:yellow_carpet" => Some(Self::yellow_carpet()),
            "minecraft:lime_carpet" => Some(Self::lime_carpet()),
            "minecraft:pink_carpet" => Some(Self::pink_carpet()),
            "minecraft:gray_carpet" => Some(Self::gray_carpet()),
            "minecraft:light_gray_carpet" => Some(Self::light_gray_carpet()),
            "minecraft:cyan_carpet" => Some(Self::cyan_carpet()),
            "minecraft:purple_carpet" => Some(Self::purple_carpet()),
            "minecraft:blue_carpet" => Some(Self::blue_carpet()),
            "minecraft:brown_carpet" => Some(Self::brown_carpet()),
            "minecraft:green_carpet" => Some(Self::green_carpet()),
            "minecraft:red_carpet" => Some(Self::red_carpet()),
            "minecraft:black_carpet" => Some(Self::black_carpet()),
            "minecraft:terracotta" => Some(Self::terracotta()),
            "minecraft:coal_block" => Some(Self::coal_block()),
            "minecraft:packed_ice" => Some(Self::packed_ice()),
            "minecraft:sunflower" => Some(Self::sunflower()),
            "minecraft:lilac" => Some(Self::lilac()),
            "minecraft:rose_bush" => Some(Self::rose_bush()),
            "minecraft:peony" => Some(Self::peony()),
            "minecraft:tall_grass" => Some(Self::tall_grass()),
            "minecraft:large_fern" => Some(Self::large_fern()),
            "minecraft:white_banner" => Some(Self::white_banner()),
            "minecraft:orange_banner" => Some(Self::orange_banner()),
            "minecraft:magenta_banner" => Some(Self::magenta_banner()),
            "minecraft:light_blue_banner" => Some(Self::light_blue_banner()),
            "minecraft:yellow_banner" => Some(Self::yellow_banner()),
            "minecraft:lime_banner" => Some(Self::lime_banner()),
            "minecraft:pink_banner" => Some(Self::pink_banner()),
            "minecraft:gray_banner" => Some(Self::gray_banner()),
            "minecraft:light_gray_banner" => Some(Self::light_gray_banner()),
            "minecraft:cyan_banner" => Some(Self::cyan_banner()),
            "minecraft:purple_banner" => Some(Self::purple_banner()),
            "minecraft:blue_banner" => Some(Self::blue_banner()),
            "minecraft:brown_banner" => Some(Self::brown_banner()),
            "minecraft:green_banner" => Some(Self::green_banner()),
            "minecraft:red_banner" => Some(Self::red_banner()),
            "minecraft:black_banner" => Some(Self::black_banner()),
            "minecraft:white_wall_banner" => Some(Self::white_wall_banner()),
            "minecraft:orange_wall_banner" => Some(Self::orange_wall_banner()),
            "minecraft:magenta_wall_banner" => Some(Self::magenta_wall_banner()),
            "minecraft:light_blue_wall_banner" => Some(Self::light_blue_wall_banner()),
            "minecraft:yellow_wall_banner" => Some(Self::yellow_wall_banner()),
            "minecraft:lime_wall_banner" => Some(Self::lime_wall_banner()),
            "minecraft:pink_wall_banner" => Some(Self::pink_wall_banner()),
            "minecraft:gray_wall_banner" => Some(Self::gray_wall_banner()),
            "minecraft:light_gray_wall_banner" => Some(Self::light_gray_wall_banner()),
            "minecraft:cyan_wall_banner" => Some(Self::cyan_wall_banner()),
            "minecraft:purple_wall_banner" => Some(Self::purple_wall_banner()),
            "minecraft:blue_wall_banner" => Some(Self::blue_wall_banner()),
            "minecraft:brown_wall_banner" => Some(Self::brown_wall_banner()),
            "minecraft:green_wall_banner" => Some(Self::green_wall_banner()),
            "minecraft:red_wall_banner" => Some(Self::red_wall_banner()),
            "minecraft:black_wall_banner" => Some(Self::black_wall_banner()),
            "minecraft:red_sandstone" => Some(Self::red_sandstone()),
            "minecraft:chiseled_red_sandstone" => Some(Self::chiseled_red_sandstone()),
            "minecraft:cut_red_sandstone" => Some(Self::cut_red_sandstone()),
            "minecraft:red_sandstone_stairs" => Some(Self::red_sandstone_stairs()),
            "minecraft:oak_slab" => Some(Self::oak_slab()),
            "minecraft:spruce_slab" => Some(Self::spruce_slab()),
            "minecraft:birch_slab" => Some(Self::birch_slab()),
            "minecraft:jungle_slab" => Some(Self::jungle_slab()),
            "minecraft:acacia_slab" => Some(Self::acacia_slab()),
            "minecraft:dark_oak_slab" => Some(Self::dark_oak_slab()),
            "minecraft:stone_slab" => Some(Self::stone_slab()),
            "minecraft:sandstone_slab" => Some(Self::sandstone_slab()),
            "minecraft:petrified_oak_slab" => Some(Self::petrified_oak_slab()),
            "minecraft:cobblestone_slab" => Some(Self::cobblestone_slab()),
            "minecraft:brick_slab" => Some(Self::brick_slab()),
            "minecraft:stone_brick_slab" => Some(Self::stone_brick_slab()),
            "minecraft:nether_brick_slab" => Some(Self::nether_brick_slab()),
            "minecraft:quartz_slab" => Some(Self::quartz_slab()),
            "minecraft:red_sandstone_slab" => Some(Self::red_sandstone_slab()),
            "minecraft:purpur_slab" => Some(Self::purpur_slab()),
            "minecraft:smooth_stone" => Some(Self::smooth_stone()),
            "minecraft:smooth_sandstone" => Some(Self::smooth_sandstone()),
            "minecraft:smooth_quartz" => Some(Self::smooth_quartz()),
            "minecraft:smooth_red_sandstone" => Some(Self::smooth_red_sandstone()),
            "minecraft:spruce_fence_gate" => Some(Self::spruce_fence_gate()),
            "minecraft:birch_fence_gate" => Some(Self::birch_fence_gate()),
            "minecraft:jungle_fence_gate" => Some(Self::jungle_fence_gate()),
            "minecraft:acacia_fence_gate" => Some(Self::acacia_fence_gate()),
            "minecraft:dark_oak_fence_gate" => Some(Self::dark_oak_fence_gate()),
            "minecraft:spruce_fence" => Some(Self::spruce_fence()),
            "minecraft:birch_fence" => Some(Self::birch_fence()),
            "minecraft:jungle_fence" => Some(Self::jungle_fence()),
            "minecraft:acacia_fence" => Some(Self::acacia_fence()),
            "minecraft:dark_oak_fence" => Some(Self::dark_oak_fence()),
            "minecraft:spruce_door" => Some(Self::spruce_door()),
            "minecraft:birch_door" => Some(Self::birch_door()),
            "minecraft:jungle_door" => Some(Self::jungle_door()),
            "minecraft:acacia_door" => Some(Self::acacia_door()),
            "minecraft:dark_oak_door" => Some(Self::dark_oak_door()),
            "minecraft:end_rod" => Some(Self::end_rod()),
            "minecraft:chorus_plant" => Some(Self::chorus_plant()),
            "minecraft:chorus_flower" => Some(Self::chorus_flower()),
            "minecraft:purpur_block" => Some(Self::purpur_block()),
            "minecraft:purpur_pillar" => Some(Self::purpur_pillar()),
            "minecraft:purpur_stairs" => Some(Self::purpur_stairs()),
            "minecraft:end_stone_bricks" => Some(Self::end_stone_bricks()),
            "minecraft:beetroots" => Some(Self::beetroots()),
            "minecraft:grass_path" => Some(Self::grass_path()),
            "minecraft:end_gateway" => Some(Self::end_gateway()),
            "minecraft:repeating_command_block" => Some(Self::repeating_command_block()),
            "minecraft:chain_command_block" => Some(Self::chain_command_block()),
            "minecraft:frosted_ice" => Some(Self::frosted_ice()),
            "minecraft:magma_block" => Some(Self::magma_block()),
            "minecraft:nether_wart_block" => Some(Self::nether_wart_block()),
            "minecraft:red_nether_bricks" => Some(Self::red_nether_bricks()),
            "minecraft:bone_block" => Some(Self::bone_block()),
            "minecraft:structure_void" => Some(Self::structure_void()),
            "minecraft:observer" => Some(Self::observer()),
            "minecraft:shulker_box" => Some(Self::shulker_box()),
            "minecraft:white_shulker_box" => Some(Self::white_shulker_box()),
            "minecraft:orange_shulker_box" => Some(Self::orange_shulker_box()),
            "minecraft:magenta_shulker_box" => Some(Self::magenta_shulker_box()),
            "minecraft:light_blue_shulker_box" => Some(Self::light_blue_shulker_box()),
            "minecraft:yellow_shulker_box" => Some(Self::yellow_shulker_box()),
            "minecraft:lime_shulker_box" => Some(Self::lime_shulker_box()),
            "minecraft:pink_shulker_box" => Some(Self::pink_shulker_box()),
            "minecraft:gray_shulker_box" => Some(Self::gray_shulker_box()),
            "minecraft:light_gray_shulker_box" => Some(Self::light_gray_shulker_box()),
            "minecraft:cyan_shulker_box" => Some(Self::cyan_shulker_box()),
            "minecraft:purple_shulker_box" => Some(Self::purple_shulker_box()),
            "minecraft:blue_shulker_box" => Some(Self::blue_shulker_box()),
            "minecraft:brown_shulker_box" => Some(Self::brown_shulker_box()),
            "minecraft:green_shulker_box" => Some(Self::green_shulker_box()),
            "minecraft:red_shulker_box" => Some(Self::red_shulker_box()),
            "minecraft:black_shulker_box" => Some(Self::black_shulker_box()),
            "minecraft:white_glazed_terracotta" => Some(Self::white_glazed_terracotta()),
            "minecraft:orange_glazed_terracotta" => Some(Self::orange_glazed_terracotta()),
            "minecraft:magenta_glazed_terracotta" => Some(Self::magenta_glazed_terracotta()),
            "minecraft:light_blue_glazed_terracotta" => Some(Self::light_blue_glazed_terracotta()),
            "minecraft:yellow_glazed_terracotta" => Some(Self::yellow_glazed_terracotta()),
            "minecraft:lime_glazed_terracotta" => Some(Self::lime_glazed_terracotta()),
            "minecraft:pink_glazed_terracotta" => Some(Self::pink_glazed_terracotta()),
            "minecraft:gray_glazed_terracotta" => Some(Self::gray_glazed_terracotta()),
            "minecraft:light_gray_glazed_terracotta" => Some(Self::light_gray_glazed_terracotta()),
            "minecraft:cyan_glazed_terracotta" => Some(Self::cyan_glazed_terracotta()),
            "minecraft:purple_glazed_terracotta" => Some(Self::purple_glazed_terracotta()),
            "minecraft:blue_glazed_terracotta" => Some(Self::blue_glazed_terracotta()),
            "minecraft:brown_glazed_terracotta" => Some(Self::brown_glazed_terracotta()),
            "minecraft:green_glazed_terracotta" => Some(Self::green_glazed_terracotta()),
            "minecraft:red_glazed_terracotta" => Some(Self::red_glazed_terracotta()),
            "minecraft:black_glazed_terracotta" => Some(Self::black_glazed_terracotta()),
            "minecraft:white_concrete" => Some(Self::white_concrete()),
            "minecraft:orange_concrete" => Some(Self::orange_concrete()),
            "minecraft:magenta_concrete" => Some(Self::magenta_concrete()),
            "minecraft:light_blue_concrete" => Some(Self::light_blue_concrete()),
            "minecraft:yellow_concrete" => Some(Self::yellow_concrete()),
            "minecraft:lime_concrete" => Some(Self::lime_concrete()),
            "minecraft:pink_concrete" => Some(Self::pink_concrete()),
            "minecraft:gray_concrete" => Some(Self::gray_concrete()),
            "minecraft:light_gray_concrete" => Some(Self::light_gray_concrete()),
            "minecraft:cyan_concrete" => Some(Self::cyan_concrete()),
            "minecraft:purple_concrete" => Some(Self::purple_concrete()),
            "minecraft:blue_concrete" => Some(Self::blue_concrete()),
            "minecraft:brown_concrete" => Some(Self::brown_concrete()),
            "minecraft:green_concrete" => Some(Self::green_concrete()),
            "minecraft:red_concrete" => Some(Self::red_concrete()),
            "minecraft:black_concrete" => Some(Self::black_concrete()),
            "minecraft:white_concrete_powder" => Some(Self::white_concrete_powder()),
            "minecraft:orange_concrete_powder" => Some(Self::orange_concrete_powder()),
            "minecraft:magenta_concrete_powder" => Some(Self::magenta_concrete_powder()),
            "minecraft:light_blue_concrete_powder" => Some(Self::light_blue_concrete_powder()),
            "minecraft:yellow_concrete_powder" => Some(Self::yellow_concrete_powder()),
            "minecraft:lime_concrete_powder" => Some(Self::lime_concrete_powder()),
            "minecraft:pink_concrete_powder" => Some(Self::pink_concrete_powder()),
            "minecraft:gray_concrete_powder" => Some(Self::gray_concrete_powder()),
            "minecraft:light_gray_concrete_powder" => Some(Self::light_gray_concrete_powder()),
            "minecraft:cyan_concrete_powder" => Some(Self::cyan_concrete_powder()),
            "minecraft:purple_concrete_powder" => Some(Self::purple_concrete_powder()),
            "minecraft:blue_concrete_powder" => Some(Self::blue_concrete_powder()),
            "minecraft:brown_concrete_powder" => Some(Self::brown_concrete_powder()),
            "minecraft:green_concrete_powder" => Some(Self::green_concrete_powder()),
            "minecraft:red_concrete_powder" => Some(Self::red_concrete_powder()),
            "minecraft:black_concrete_powder" => Some(Self::black_concrete_powder()),
            "minecraft:kelp" => Some(Self::kelp()),
            "minecraft:kelp_plant" => Some(Self::kelp_plant()),
            "minecraft:dried_kelp_block" => Some(Self::dried_kelp_block()),
            "minecraft:turtle_egg" => Some(Self::turtle_egg()),
            "minecraft:dead_tube_coral_block" => Some(Self::dead_tube_coral_block()),
            "minecraft:dead_brain_coral_block" => Some(Self::dead_brain_coral_block()),
            "minecraft:dead_bubble_coral_block" => Some(Self::dead_bubble_coral_block()),
            "minecraft:dead_fire_coral_block" => Some(Self::dead_fire_coral_block()),
            "minecraft:dead_horn_coral_block" => Some(Self::dead_horn_coral_block()),
            "minecraft:tube_coral_block" => Some(Self::tube_coral_block()),
            "minecraft:brain_coral_block" => Some(Self::brain_coral_block()),
            "minecraft:bubble_coral_block" => Some(Self::bubble_coral_block()),
            "minecraft:fire_coral_block" => Some(Self::fire_coral_block()),
            "minecraft:horn_coral_block" => Some(Self::horn_coral_block()),
            "minecraft:dead_tube_coral" => Some(Self::dead_tube_coral()),
            "minecraft:dead_brain_coral" => Some(Self::dead_brain_coral()),
            "minecraft:dead_bubble_coral" => Some(Self::dead_bubble_coral()),
            "minecraft:dead_fire_coral" => Some(Self::dead_fire_coral()),
            "minecraft:dead_horn_coral" => Some(Self::dead_horn_coral()),
            "minecraft:tube_coral" => Some(Self::tube_coral()),
            "minecraft:brain_coral" => Some(Self::brain_coral()),
            "minecraft:bubble_coral" => Some(Self::bubble_coral()),
            "minecraft:fire_coral" => Some(Self::fire_coral()),
            "minecraft:horn_coral" => Some(Self::horn_coral()),
            "minecraft:dead_tube_coral_wall_fan" => Some(Self::dead_tube_coral_wall_fan()),
            "minecraft:dead_brain_coral_wall_fan" => Some(Self::dead_brain_coral_wall_fan()),
            "minecraft:dead_bubble_coral_wall_fan" => Some(Self::dead_bubble_coral_wall_fan()),
            "minecraft:dead_fire_coral_wall_fan" => Some(Self::dead_fire_coral_wall_fan()),
            "minecraft:dead_horn_coral_wall_fan" => Some(Self::dead_horn_coral_wall_fan()),
            "minecraft:tube_coral_wall_fan" => Some(Self::tube_coral_wall_fan()),
            "minecraft:brain_coral_wall_fan" => Some(Self::brain_coral_wall_fan()),
            "minecraft:bubble_coral_wall_fan" => Some(Self::bubble_coral_wall_fan()),
            "minecraft:fire_coral_wall_fan" => Some(Self::fire_coral_wall_fan()),
            "minecraft:horn_coral_wall_fan" => Some(Self::horn_coral_wall_fan()),
            "minecraft:dead_tube_coral_fan" => Some(Self::dead_tube_coral_fan()),
            "minecraft:dead_brain_coral_fan" => Some(Self::dead_brain_coral_fan()),
            "minecraft:dead_bubble_coral_fan" => Some(Self::dead_bubble_coral_fan()),
            "minecraft:dead_fire_coral_fan" => Some(Self::dead_fire_coral_fan()),
            "minecraft:dead_horn_coral_fan" => Some(Self::dead_horn_coral_fan()),
            "minecraft:tube_coral_fan" => Some(Self::tube_coral_fan()),
            "minecraft:brain_coral_fan" => Some(Self::brain_coral_fan()),
            "minecraft:bubble_coral_fan" => Some(Self::bubble_coral_fan()),
            "minecraft:fire_coral_fan" => Some(Self::fire_coral_fan()),
            "minecraft:horn_coral_fan" => Some(Self::horn_coral_fan()),
            "minecraft:sea_pickle" => Some(Self::sea_pickle()),
            "minecraft:blue_ice" => Some(Self::blue_ice()),
            "minecraft:conduit" => Some(Self::conduit()),
            "minecraft:void_air" => Some(Self::void_air()),
            "minecraft:cave_air" => Some(Self::cave_air()),
            "minecraft:bubble_column" => Some(Self::bubble_column()),
            "minecraft:structure_block" => Some(Self::structure_block()),
            _ => None,
        }
    }
}
