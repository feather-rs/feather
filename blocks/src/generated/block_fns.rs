use crate::*;
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
}
