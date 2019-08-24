use feather_blocks::*;
use feather_items::Item;
pub fn item_to_block(item: Item) -> Option<Block> {
    match item {
        Item::Air => Some(Block::Air),
        Item::Stone => Some(Block::Stone),
        Item::Granite => Some(Block::Granite),
        Item::PolishedGranite => Some(Block::PolishedGranite),
        Item::Diorite => Some(Block::Diorite),
        Item::PolishedDiorite => Some(Block::PolishedDiorite),
        Item::Andesite => Some(Block::Andesite),
        Item::PolishedAndesite => Some(Block::PolishedAndesite),
        Item::GrassBlock => Some(Block::GrassBlock(GrassBlockData { snowy: false })),
        Item::Dirt => Some(Block::Dirt),
        Item::CoarseDirt => Some(Block::CoarseDirt),
        Item::Podzol => Some(Block::Podzol(PodzolData { snowy: false })),
        Item::Cobblestone => Some(Block::Cobblestone),
        Item::OakPlanks => Some(Block::OakPlanks),
        Item::SprucePlanks => Some(Block::SprucePlanks),
        Item::BirchPlanks => Some(Block::BirchPlanks),
        Item::JunglePlanks => Some(Block::JunglePlanks),
        Item::AcaciaPlanks => Some(Block::AcaciaPlanks),
        Item::DarkOakPlanks => Some(Block::DarkOakPlanks),
        Item::OakSapling => Some(Block::OakSapling(OakSaplingData { stage: 0i32 })),
        Item::SpruceSapling => Some(Block::SpruceSapling(SpruceSaplingData { stage: 0i32 })),
        Item::BirchSapling => Some(Block::BirchSapling(BirchSaplingData { stage: 0i32 })),
        Item::JungleSapling => Some(Block::JungleSapling(JungleSaplingData { stage: 0i32 })),
        Item::AcaciaSapling => Some(Block::AcaciaSapling(AcaciaSaplingData { stage: 0i32 })),
        Item::DarkOakSapling => Some(Block::DarkOakSapling(DarkOakSaplingData { stage: 0i32 })),
        Item::Bedrock => Some(Block::Bedrock),
        Item::Sand => Some(Block::Sand),
        Item::RedSand => Some(Block::RedSand),
        Item::Gravel => Some(Block::Gravel),
        Item::GoldOre => Some(Block::GoldOre),
        Item::IronOre => Some(Block::IronOre),
        Item::CoalOre => Some(Block::CoalOre),
        Item::OakLog => Some(Block::OakLog(OakLogData {
            axis: OakLogAxis::Y,
        })),
        Item::SpruceLog => Some(Block::SpruceLog(SpruceLogData {
            axis: SpruceLogAxis::Y,
        })),
        Item::BirchLog => Some(Block::BirchLog(BirchLogData {
            axis: BirchLogAxis::Y,
        })),
        Item::JungleLog => Some(Block::JungleLog(JungleLogData {
            axis: JungleLogAxis::Y,
        })),
        Item::AcaciaLog => Some(Block::AcaciaLog(AcaciaLogData {
            axis: AcaciaLogAxis::Y,
        })),
        Item::DarkOakLog => Some(Block::DarkOakLog(DarkOakLogData {
            axis: DarkOakLogAxis::Y,
        })),
        Item::StrippedOakLog => Some(Block::StrippedOakLog(StrippedOakLogData {
            axis: StrippedOakLogAxis::Y,
        })),
        Item::StrippedSpruceLog => Some(Block::StrippedSpruceLog(StrippedSpruceLogData {
            axis: StrippedSpruceLogAxis::Y,
        })),
        Item::StrippedBirchLog => Some(Block::StrippedBirchLog(StrippedBirchLogData {
            axis: StrippedBirchLogAxis::Y,
        })),
        Item::StrippedJungleLog => Some(Block::StrippedJungleLog(StrippedJungleLogData {
            axis: StrippedJungleLogAxis::Y,
        })),
        Item::StrippedAcaciaLog => Some(Block::StrippedAcaciaLog(StrippedAcaciaLogData {
            axis: StrippedAcaciaLogAxis::Y,
        })),
        Item::StrippedDarkOakLog => Some(Block::StrippedDarkOakLog(StrippedDarkOakLogData {
            axis: StrippedDarkOakLogAxis::Y,
        })),
        Item::StrippedOakWood => Some(Block::StrippedOakWood(StrippedOakWoodData {
            axis: StrippedOakWoodAxis::Y,
        })),
        Item::StrippedSpruceWood => Some(Block::StrippedSpruceWood(StrippedSpruceWoodData {
            axis: StrippedSpruceWoodAxis::Y,
        })),
        Item::StrippedBirchWood => Some(Block::StrippedBirchWood(StrippedBirchWoodData {
            axis: StrippedBirchWoodAxis::Y,
        })),
        Item::StrippedJungleWood => Some(Block::StrippedJungleWood(StrippedJungleWoodData {
            axis: StrippedJungleWoodAxis::Y,
        })),
        Item::StrippedAcaciaWood => Some(Block::StrippedAcaciaWood(StrippedAcaciaWoodData {
            axis: StrippedAcaciaWoodAxis::Y,
        })),
        Item::StrippedDarkOakWood => Some(Block::StrippedDarkOakWood(StrippedDarkOakWoodData {
            axis: StrippedDarkOakWoodAxis::Y,
        })),
        Item::OakWood => Some(Block::OakWood(OakWoodData {
            axis: OakWoodAxis::Y,
        })),
        Item::SpruceWood => Some(Block::SpruceWood(SpruceWoodData {
            axis: SpruceWoodAxis::Y,
        })),
        Item::BirchWood => Some(Block::BirchWood(BirchWoodData {
            axis: BirchWoodAxis::Y,
        })),
        Item::JungleWood => Some(Block::JungleWood(JungleWoodData {
            axis: JungleWoodAxis::Y,
        })),
        Item::AcaciaWood => Some(Block::AcaciaWood(AcaciaWoodData {
            axis: AcaciaWoodAxis::Y,
        })),
        Item::DarkOakWood => Some(Block::DarkOakWood(DarkOakWoodData {
            axis: DarkOakWoodAxis::Y,
        })),
        Item::OakLeaves => Some(Block::OakLeaves(OakLeavesData {
            persistent: false,
            distance: 7i32,
        })),
        Item::SpruceLeaves => Some(Block::SpruceLeaves(SpruceLeavesData {
            persistent: false,
            distance: 7i32,
        })),
        Item::BirchLeaves => Some(Block::BirchLeaves(BirchLeavesData {
            persistent: false,
            distance: 7i32,
        })),
        Item::JungleLeaves => Some(Block::JungleLeaves(JungleLeavesData {
            persistent: false,
            distance: 7i32,
        })),
        Item::AcaciaLeaves => Some(Block::AcaciaLeaves(AcaciaLeavesData {
            distance: 7i32,
            persistent: false,
        })),
        Item::DarkOakLeaves => Some(Block::DarkOakLeaves(DarkOakLeavesData {
            distance: 7i32,
            persistent: false,
        })),
        Item::Sponge => Some(Block::Sponge),
        Item::WetSponge => Some(Block::WetSponge),
        Item::Glass => Some(Block::Glass),
        Item::LapisOre => Some(Block::LapisOre),
        Item::LapisBlock => Some(Block::LapisBlock),
        Item::Dispenser => Some(Block::Dispenser(DispenserData {
            triggered: false,
            facing: DispenserFacing::North,
        })),
        Item::Sandstone => Some(Block::Sandstone),
        Item::ChiseledSandstone => Some(Block::ChiseledSandstone),
        Item::CutSandstone => Some(Block::CutSandstone),
        Item::NoteBlock => Some(Block::NoteBlock(NoteBlockData {
            instrument: NoteBlockInstrument::Harp,
            powered: false,
            note: 0i32,
        })),
        Item::PoweredRail => Some(Block::PoweredRail(PoweredRailData {
            powered: false,
            shape: PoweredRailShape::NorthSouth,
        })),
        Item::DetectorRail => Some(Block::DetectorRail(DetectorRailData {
            shape: DetectorRailShape::NorthSouth,
            powered: false,
        })),
        Item::StickyPiston => Some(Block::StickyPiston(StickyPistonData {
            extended: false,
            facing: StickyPistonFacing::North,
        })),
        Item::Cobweb => Some(Block::Cobweb),
        Item::Grass => Some(Block::Grass),
        Item::Fern => Some(Block::Fern),
        Item::DeadBush => Some(Block::DeadBush),
        Item::Seagrass => Some(Block::Seagrass),
        Item::SeaPickle => Some(Block::SeaPickle(SeaPickleData {
            waterlogged: true,
            pickles: 1i32,
        })),
        Item::Piston => Some(Block::Piston(PistonData {
            extended: false,
            facing: PistonFacing::North,
        })),
        Item::WhiteWool => Some(Block::WhiteWool),
        Item::OrangeWool => Some(Block::OrangeWool),
        Item::MagentaWool => Some(Block::MagentaWool),
        Item::LightBlueWool => Some(Block::LightBlueWool),
        Item::YellowWool => Some(Block::YellowWool),
        Item::LimeWool => Some(Block::LimeWool),
        Item::PinkWool => Some(Block::PinkWool),
        Item::GrayWool => Some(Block::GrayWool),
        Item::LightGrayWool => Some(Block::LightGrayWool),
        Item::CyanWool => Some(Block::CyanWool),
        Item::PurpleWool => Some(Block::PurpleWool),
        Item::BlueWool => Some(Block::BlueWool),
        Item::BrownWool => Some(Block::BrownWool),
        Item::GreenWool => Some(Block::GreenWool),
        Item::RedWool => Some(Block::RedWool),
        Item::BlackWool => Some(Block::BlackWool),
        Item::Dandelion => Some(Block::Dandelion),
        Item::Poppy => Some(Block::Poppy),
        Item::BlueOrchid => Some(Block::BlueOrchid),
        Item::Allium => Some(Block::Allium),
        Item::AzureBluet => Some(Block::AzureBluet),
        Item::RedTulip => Some(Block::RedTulip),
        Item::OrangeTulip => Some(Block::OrangeTulip),
        Item::WhiteTulip => Some(Block::WhiteTulip),
        Item::PinkTulip => Some(Block::PinkTulip),
        Item::OxeyeDaisy => Some(Block::OxeyeDaisy),
        Item::BrownMushroom => Some(Block::BrownMushroom),
        Item::RedMushroom => Some(Block::RedMushroom),
        Item::GoldBlock => Some(Block::GoldBlock),
        Item::IronBlock => Some(Block::IronBlock),
        Item::OakSlab => Some(Block::OakSlab(OakSlabData {
            ty: OakSlabType::Bottom,
            waterlogged: false,
        })),
        Item::SpruceSlab => Some(Block::SpruceSlab(SpruceSlabData {
            ty: SpruceSlabType::Bottom,
            waterlogged: false,
        })),
        Item::BirchSlab => Some(Block::BirchSlab(BirchSlabData {
            waterlogged: false,
            ty: BirchSlabType::Bottom,
        })),
        Item::JungleSlab => Some(Block::JungleSlab(JungleSlabData {
            waterlogged: false,
            ty: JungleSlabType::Bottom,
        })),
        Item::AcaciaSlab => Some(Block::AcaciaSlab(AcaciaSlabData {
            ty: AcaciaSlabType::Bottom,
            waterlogged: false,
        })),
        Item::DarkOakSlab => Some(Block::DarkOakSlab(DarkOakSlabData {
            waterlogged: false,
            ty: DarkOakSlabType::Bottom,
        })),
        Item::StoneSlab => Some(Block::StoneSlab(StoneSlabData {
            ty: StoneSlabType::Bottom,
            waterlogged: false,
        })),
        Item::SandstoneSlab => Some(Block::SandstoneSlab(SandstoneSlabData {
            ty: SandstoneSlabType::Bottom,
            waterlogged: false,
        })),
        Item::PetrifiedOakSlab => Some(Block::PetrifiedOakSlab(PetrifiedOakSlabData {
            waterlogged: false,
            ty: PetrifiedOakSlabType::Bottom,
        })),
        Item::CobblestoneSlab => Some(Block::CobblestoneSlab(CobblestoneSlabData {
            waterlogged: false,
            ty: CobblestoneSlabType::Bottom,
        })),
        Item::BrickSlab => Some(Block::BrickSlab(BrickSlabData {
            ty: BrickSlabType::Bottom,
            waterlogged: false,
        })),
        Item::StoneBrickSlab => Some(Block::StoneBrickSlab(StoneBrickSlabData {
            ty: StoneBrickSlabType::Bottom,
            waterlogged: false,
        })),
        Item::NetherBrickSlab => Some(Block::NetherBrickSlab(NetherBrickSlabData {
            ty: NetherBrickSlabType::Bottom,
            waterlogged: false,
        })),
        Item::QuartzSlab => Some(Block::QuartzSlab(QuartzSlabData {
            waterlogged: false,
            ty: QuartzSlabType::Bottom,
        })),
        Item::RedSandstoneSlab => Some(Block::RedSandstoneSlab(RedSandstoneSlabData {
            ty: RedSandstoneSlabType::Bottom,
            waterlogged: false,
        })),
        Item::PurpurSlab => Some(Block::PurpurSlab(PurpurSlabData {
            waterlogged: false,
            ty: PurpurSlabType::Bottom,
        })),
        Item::PrismarineSlab => Some(Block::PrismarineSlab(PrismarineSlabData {
            waterlogged: false,
            ty: PrismarineSlabType::Bottom,
        })),
        Item::PrismarineBrickSlab => Some(Block::PrismarineBrickSlab(PrismarineBrickSlabData {
            waterlogged: false,
            ty: PrismarineBrickSlabType::Bottom,
        })),
        Item::DarkPrismarineSlab => Some(Block::DarkPrismarineSlab(DarkPrismarineSlabData {
            waterlogged: false,
            ty: DarkPrismarineSlabType::Bottom,
        })),
        Item::SmoothQuartz => Some(Block::SmoothQuartz),
        Item::SmoothRedSandstone => Some(Block::SmoothRedSandstone),
        Item::SmoothSandstone => Some(Block::SmoothSandstone),
        Item::SmoothStone => Some(Block::SmoothStone),
        Item::Bricks => Some(Block::Bricks),
        Item::Tnt => Some(Block::Tnt(TntData { unstable: false })),
        Item::Bookshelf => Some(Block::Bookshelf),
        Item::MossyCobblestone => Some(Block::MossyCobblestone),
        Item::Obsidian => Some(Block::Obsidian),
        Item::Torch => Some(Block::Torch),
        Item::EndRod => Some(Block::EndRod(EndRodData {
            facing: EndRodFacing::Up,
        })),
        Item::ChorusPlant => Some(Block::ChorusPlant(ChorusPlantData {
            north: false,
            east: false,
            south: false,
            up: false,
            down: false,
            west: false,
        })),
        Item::ChorusFlower => Some(Block::ChorusFlower(ChorusFlowerData { age: 0i32 })),
        Item::PurpurBlock => Some(Block::PurpurBlock),
        Item::PurpurPillar => Some(Block::PurpurPillar(PurpurPillarData {
            axis: PurpurPillarAxis::Y,
        })),
        Item::PurpurStairs => Some(Block::PurpurStairs(PurpurStairsData {
            half: PurpurStairsHalf::Bottom,
            waterlogged: false,
            facing: PurpurStairsFacing::North,
            shape: PurpurStairsShape::Straight,
        })),
        Item::Spawner => Some(Block::Spawner),
        Item::OakStairs => Some(Block::OakStairs(OakStairsData {
            half: OakStairsHalf::Bottom,
            waterlogged: false,
            shape: OakStairsShape::Straight,
            facing: OakStairsFacing::North,
        })),
        Item::Chest => Some(Block::Chest(ChestData {
            ty: ChestType::Single,
            facing: ChestFacing::North,
            waterlogged: false,
        })),
        Item::DiamondOre => Some(Block::DiamondOre),
        Item::DiamondBlock => Some(Block::DiamondBlock),
        Item::CraftingTable => Some(Block::CraftingTable),
        Item::Farmland => Some(Block::Farmland(FarmlandData { moisture: 0i32 })),
        Item::Furnace => Some(Block::Furnace(FurnaceData {
            lit: false,
            facing: FurnaceFacing::North,
        })),
        Item::Ladder => Some(Block::Ladder(LadderData {
            facing: LadderFacing::North,
            waterlogged: false,
        })),
        Item::Rail => Some(Block::Rail(RailData {
            shape: RailShape::NorthSouth,
        })),
        Item::CobblestoneStairs => Some(Block::CobblestoneStairs(CobblestoneStairsData {
            half: CobblestoneStairsHalf::Bottom,
            waterlogged: false,
            facing: CobblestoneStairsFacing::North,
            shape: CobblestoneStairsShape::Straight,
        })),
        Item::Lever => Some(Block::Lever(LeverData {
            facing: LeverFacing::North,
            powered: false,
            face: LeverFace::Wall,
        })),
        Item::StonePressurePlate => Some(Block::StonePressurePlate(StonePressurePlateData {
            powered: false,
        })),
        Item::OakPressurePlate => Some(Block::OakPressurePlate(OakPressurePlateData {
            powered: false,
        })),
        Item::SprucePressurePlate => Some(Block::SprucePressurePlate(SprucePressurePlateData {
            powered: false,
        })),
        Item::BirchPressurePlate => Some(Block::BirchPressurePlate(BirchPressurePlateData {
            powered: false,
        })),
        Item::JunglePressurePlate => Some(Block::JunglePressurePlate(JunglePressurePlateData {
            powered: false,
        })),
        Item::AcaciaPressurePlate => Some(Block::AcaciaPressurePlate(AcaciaPressurePlateData {
            powered: false,
        })),
        Item::DarkOakPressurePlate => Some(Block::DarkOakPressurePlate(DarkOakPressurePlateData {
            powered: false,
        })),
        Item::RedstoneOre => Some(Block::RedstoneOre(RedstoneOreData { lit: false })),
        Item::RedstoneTorch => Some(Block::RedstoneTorch(RedstoneTorchData { lit: true })),
        Item::StoneButton => Some(Block::StoneButton(StoneButtonData {
            face: StoneButtonFace::Wall,
            powered: false,
            facing: StoneButtonFacing::North,
        })),
        Item::Snow => Some(Block::Snow(SnowData { layers: 1i32 })),
        Item::Ice => Some(Block::Ice),
        Item::SnowBlock => Some(Block::SnowBlock),
        Item::Cactus => Some(Block::Cactus(CactusData { age: 0i32 })),
        Item::Clay => Some(Block::Clay),
        Item::Jukebox => Some(Block::Jukebox(JukeboxData { has_record: false })),
        Item::OakFence => Some(Block::OakFence(OakFenceData {
            waterlogged: false,
            west: false,
            east: false,
            north: false,
            south: false,
        })),
        Item::SpruceFence => Some(Block::SpruceFence(SpruceFenceData {
            south: false,
            waterlogged: false,
            west: false,
            east: false,
            north: false,
        })),
        Item::BirchFence => Some(Block::BirchFence(BirchFenceData {
            west: false,
            south: false,
            east: false,
            north: false,
            waterlogged: false,
        })),
        Item::JungleFence => Some(Block::JungleFence(JungleFenceData {
            south: false,
            east: false,
            waterlogged: false,
            north: false,
            west: false,
        })),
        Item::AcaciaFence => Some(Block::AcaciaFence(AcaciaFenceData {
            waterlogged: false,
            south: false,
            east: false,
            north: false,
            west: false,
        })),
        Item::DarkOakFence => Some(Block::DarkOakFence(DarkOakFenceData {
            south: false,
            west: false,
            east: false,
            waterlogged: false,
            north: false,
        })),
        Item::Pumpkin => Some(Block::Pumpkin),
        Item::CarvedPumpkin => Some(Block::CarvedPumpkin(CarvedPumpkinData {
            facing: CarvedPumpkinFacing::North,
        })),
        Item::Netherrack => Some(Block::Netherrack),
        Item::SoulSand => Some(Block::SoulSand),
        Item::Glowstone => Some(Block::Glowstone),
        Item::JackOLantern => Some(Block::JackOLantern(JackOLanternData {
            facing: JackOLanternFacing::North,
        })),
        Item::OakTrapdoor => Some(Block::OakTrapdoor(OakTrapdoorData {
            half: OakTrapdoorHalf::Bottom,
            powered: false,
            open: false,
            waterlogged: false,
            facing: OakTrapdoorFacing::North,
        })),
        Item::SpruceTrapdoor => Some(Block::SpruceTrapdoor(SpruceTrapdoorData {
            open: false,
            facing: SpruceTrapdoorFacing::North,
            half: SpruceTrapdoorHalf::Bottom,
            waterlogged: false,
            powered: false,
        })),
        Item::BirchTrapdoor => Some(Block::BirchTrapdoor(BirchTrapdoorData {
            open: false,
            facing: BirchTrapdoorFacing::North,
            waterlogged: false,
            powered: false,
            half: BirchTrapdoorHalf::Bottom,
        })),
        Item::JungleTrapdoor => Some(Block::JungleTrapdoor(JungleTrapdoorData {
            half: JungleTrapdoorHalf::Bottom,
            open: false,
            facing: JungleTrapdoorFacing::North,
            powered: false,
            waterlogged: false,
        })),
        Item::AcaciaTrapdoor => Some(Block::AcaciaTrapdoor(AcaciaTrapdoorData {
            waterlogged: false,
            facing: AcaciaTrapdoorFacing::North,
            half: AcaciaTrapdoorHalf::Bottom,
            open: false,
            powered: false,
        })),
        Item::DarkOakTrapdoor => Some(Block::DarkOakTrapdoor(DarkOakTrapdoorData {
            powered: false,
            facing: DarkOakTrapdoorFacing::North,
            open: false,
            waterlogged: false,
            half: DarkOakTrapdoorHalf::Bottom,
        })),
        Item::InfestedStone => Some(Block::InfestedStone),
        Item::InfestedCobblestone => Some(Block::InfestedCobblestone),
        Item::InfestedStoneBricks => Some(Block::InfestedStoneBricks),
        Item::InfestedMossyStoneBricks => Some(Block::InfestedMossyStoneBricks),
        Item::InfestedCrackedStoneBricks => Some(Block::InfestedCrackedStoneBricks),
        Item::InfestedChiseledStoneBricks => Some(Block::InfestedChiseledStoneBricks),
        Item::StoneBricks => Some(Block::StoneBricks),
        Item::MossyStoneBricks => Some(Block::MossyStoneBricks),
        Item::CrackedStoneBricks => Some(Block::CrackedStoneBricks),
        Item::ChiseledStoneBricks => Some(Block::ChiseledStoneBricks),
        Item::BrownMushroomBlock => Some(Block::BrownMushroomBlock(BrownMushroomBlockData {
            west: true,
            east: true,
            up: true,
            north: true,
            south: true,
            down: true,
        })),
        Item::RedMushroomBlock => Some(Block::RedMushroomBlock(RedMushroomBlockData {
            east: true,
            down: true,
            south: true,
            west: true,
            up: true,
            north: true,
        })),
        Item::MushroomStem => Some(Block::MushroomStem(MushroomStemData {
            south: true,
            north: true,
            up: true,
            west: true,
            east: true,
            down: true,
        })),
        Item::IronBars => Some(Block::IronBars(IronBarsData {
            west: false,
            waterlogged: false,
            east: false,
            north: false,
            south: false,
        })),
        Item::GlassPane => Some(Block::GlassPane(GlassPaneData {
            south: false,
            waterlogged: false,
            west: false,
            north: false,
            east: false,
        })),
        Item::Melon => Some(Block::Melon),
        Item::Vine => Some(Block::Vine(VineData {
            south: false,
            north: false,
            up: false,
            west: false,
            east: false,
        })),
        Item::OakFenceGate => Some(Block::OakFenceGate(OakFenceGateData {
            powered: false,
            in_wall: false,
            open: false,
            facing: OakFenceGateFacing::North,
        })),
        Item::SpruceFenceGate => Some(Block::SpruceFenceGate(SpruceFenceGateData {
            facing: SpruceFenceGateFacing::North,
            open: false,
            powered: false,
            in_wall: false,
        })),
        Item::BirchFenceGate => Some(Block::BirchFenceGate(BirchFenceGateData {
            open: false,
            powered: false,
            facing: BirchFenceGateFacing::North,
            in_wall: false,
        })),
        Item::JungleFenceGate => Some(Block::JungleFenceGate(JungleFenceGateData {
            open: false,
            in_wall: false,
            powered: false,
            facing: JungleFenceGateFacing::North,
        })),
        Item::AcaciaFenceGate => Some(Block::AcaciaFenceGate(AcaciaFenceGateData {
            open: false,
            facing: AcaciaFenceGateFacing::North,
            in_wall: false,
            powered: false,
        })),
        Item::DarkOakFenceGate => Some(Block::DarkOakFenceGate(DarkOakFenceGateData {
            in_wall: false,
            facing: DarkOakFenceGateFacing::North,
            open: false,
            powered: false,
        })),
        Item::BrickStairs => Some(Block::BrickStairs(BrickStairsData {
            half: BrickStairsHalf::Bottom,
            facing: BrickStairsFacing::North,
            waterlogged: false,
            shape: BrickStairsShape::Straight,
        })),
        Item::StoneBrickStairs => Some(Block::StoneBrickStairs(StoneBrickStairsData {
            half: StoneBrickStairsHalf::Bottom,
            shape: StoneBrickStairsShape::Straight,
            facing: StoneBrickStairsFacing::North,
            waterlogged: false,
        })),
        Item::Mycelium => Some(Block::Mycelium(MyceliumData { snowy: false })),
        Item::LilyPad => Some(Block::LilyPad),
        Item::NetherBricks => Some(Block::NetherBricks),
        Item::NetherBrickFence => Some(Block::NetherBrickFence(NetherBrickFenceData {
            north: false,
            south: false,
            east: false,
            waterlogged: false,
            west: false,
        })),
        Item::NetherBrickStairs => Some(Block::NetherBrickStairs(NetherBrickStairsData {
            shape: NetherBrickStairsShape::Straight,
            waterlogged: false,
            half: NetherBrickStairsHalf::Bottom,
            facing: NetherBrickStairsFacing::North,
        })),
        Item::EnchantingTable => Some(Block::EnchantingTable),
        Item::EndPortalFrame => Some(Block::EndPortalFrame(EndPortalFrameData {
            facing: EndPortalFrameFacing::North,
            eye: false,
        })),
        Item::EndStone => Some(Block::EndStone),
        Item::EndStoneBricks => Some(Block::EndStoneBricks),
        Item::DragonEgg => Some(Block::DragonEgg),
        Item::RedstoneLamp => Some(Block::RedstoneLamp(RedstoneLampData { lit: false })),
        Item::SandstoneStairs => Some(Block::SandstoneStairs(SandstoneStairsData {
            shape: SandstoneStairsShape::Straight,
            half: SandstoneStairsHalf::Bottom,
            facing: SandstoneStairsFacing::North,
            waterlogged: false,
        })),
        Item::EmeraldOre => Some(Block::EmeraldOre),
        Item::EnderChest => Some(Block::EnderChest(EnderChestData {
            facing: EnderChestFacing::North,
            waterlogged: false,
        })),
        Item::TripwireHook => Some(Block::TripwireHook(TripwireHookData {
            facing: TripwireHookFacing::North,
            attached: false,
            powered: false,
        })),
        Item::EmeraldBlock => Some(Block::EmeraldBlock),
        Item::SpruceStairs => Some(Block::SpruceStairs(SpruceStairsData {
            shape: SpruceStairsShape::Straight,
            half: SpruceStairsHalf::Bottom,
            waterlogged: false,
            facing: SpruceStairsFacing::North,
        })),
        Item::BirchStairs => Some(Block::BirchStairs(BirchStairsData {
            half: BirchStairsHalf::Bottom,
            waterlogged: false,
            facing: BirchStairsFacing::North,
            shape: BirchStairsShape::Straight,
        })),
        Item::JungleStairs => Some(Block::JungleStairs(JungleStairsData {
            half: JungleStairsHalf::Bottom,
            waterlogged: false,
            shape: JungleStairsShape::Straight,
            facing: JungleStairsFacing::North,
        })),
        Item::CommandBlock => Some(Block::CommandBlock(CommandBlockData {
            facing: CommandBlockFacing::North,
            conditional: false,
        })),
        Item::Beacon => Some(Block::Beacon),
        Item::CobblestoneWall => Some(Block::CobblestoneWall(CobblestoneWallData {
            up: true,
            east: false,
            waterlogged: false,
            west: false,
            north: false,
            south: false,
        })),
        Item::MossyCobblestoneWall => Some(Block::MossyCobblestoneWall(MossyCobblestoneWallData {
            waterlogged: false,
            west: false,
            north: false,
            south: false,
            east: false,
            up: true,
        })),
        Item::OakButton => Some(Block::OakButton(OakButtonData {
            facing: OakButtonFacing::North,
            powered: false,
            face: OakButtonFace::Wall,
        })),
        Item::SpruceButton => Some(Block::SpruceButton(SpruceButtonData {
            powered: false,
            face: SpruceButtonFace::Wall,
            facing: SpruceButtonFacing::North,
        })),
        Item::BirchButton => Some(Block::BirchButton(BirchButtonData {
            facing: BirchButtonFacing::North,
            powered: false,
            face: BirchButtonFace::Wall,
        })),
        Item::JungleButton => Some(Block::JungleButton(JungleButtonData {
            facing: JungleButtonFacing::North,
            powered: false,
            face: JungleButtonFace::Wall,
        })),
        Item::AcaciaButton => Some(Block::AcaciaButton(AcaciaButtonData {
            powered: false,
            face: AcaciaButtonFace::Wall,
            facing: AcaciaButtonFacing::North,
        })),
        Item::DarkOakButton => Some(Block::DarkOakButton(DarkOakButtonData {
            face: DarkOakButtonFace::Wall,
            powered: false,
            facing: DarkOakButtonFacing::North,
        })),
        Item::Anvil => Some(Block::Anvil(AnvilData {
            facing: AnvilFacing::North,
        })),
        Item::ChippedAnvil => Some(Block::ChippedAnvil(ChippedAnvilData {
            facing: ChippedAnvilFacing::North,
        })),
        Item::DamagedAnvil => Some(Block::DamagedAnvil(DamagedAnvilData {
            facing: DamagedAnvilFacing::North,
        })),
        Item::TrappedChest => Some(Block::TrappedChest(TrappedChestData {
            ty: TrappedChestType::Single,
            waterlogged: false,
            facing: TrappedChestFacing::North,
        })),
        Item::LightWeightedPressurePlate => Some(Block::LightWeightedPressurePlate(
            LightWeightedPressurePlateData { power: 0i32 },
        )),
        Item::HeavyWeightedPressurePlate => Some(Block::HeavyWeightedPressurePlate(
            HeavyWeightedPressurePlateData { power: 0i32 },
        )),
        Item::DaylightDetector => Some(Block::DaylightDetector(DaylightDetectorData {
            power: 0i32,
            inverted: false,
        })),
        Item::RedstoneBlock => Some(Block::RedstoneBlock),
        Item::NetherQuartzOre => Some(Block::NetherQuartzOre),
        Item::Hopper => Some(Block::Hopper(HopperData {
            enabled: true,
            facing: HopperFacing::Down,
        })),
        Item::ChiseledQuartzBlock => Some(Block::ChiseledQuartzBlock),
        Item::QuartzBlock => Some(Block::QuartzBlock),
        Item::QuartzPillar => Some(Block::QuartzPillar(QuartzPillarData {
            axis: QuartzPillarAxis::Y,
        })),
        Item::QuartzStairs => Some(Block::QuartzStairs(QuartzStairsData {
            shape: QuartzStairsShape::Straight,
            waterlogged: false,
            half: QuartzStairsHalf::Bottom,
            facing: QuartzStairsFacing::North,
        })),
        Item::ActivatorRail => Some(Block::ActivatorRail(ActivatorRailData {
            shape: ActivatorRailShape::NorthSouth,
            powered: false,
        })),
        Item::Dropper => Some(Block::Dropper(DropperData {
            facing: DropperFacing::North,
            triggered: false,
        })),
        Item::WhiteTerracotta => Some(Block::WhiteTerracotta),
        Item::OrangeTerracotta => Some(Block::OrangeTerracotta),
        Item::MagentaTerracotta => Some(Block::MagentaTerracotta),
        Item::LightBlueTerracotta => Some(Block::LightBlueTerracotta),
        Item::YellowTerracotta => Some(Block::YellowTerracotta),
        Item::LimeTerracotta => Some(Block::LimeTerracotta),
        Item::PinkTerracotta => Some(Block::PinkTerracotta),
        Item::GrayTerracotta => Some(Block::GrayTerracotta),
        Item::LightGrayTerracotta => Some(Block::LightGrayTerracotta),
        Item::CyanTerracotta => Some(Block::CyanTerracotta),
        Item::PurpleTerracotta => Some(Block::PurpleTerracotta),
        Item::BlueTerracotta => Some(Block::BlueTerracotta),
        Item::BrownTerracotta => Some(Block::BrownTerracotta),
        Item::GreenTerracotta => Some(Block::GreenTerracotta),
        Item::RedTerracotta => Some(Block::RedTerracotta),
        Item::BlackTerracotta => Some(Block::BlackTerracotta),
        Item::Barrier => Some(Block::Barrier),
        Item::IronTrapdoor => Some(Block::IronTrapdoor(IronTrapdoorData {
            powered: false,
            facing: IronTrapdoorFacing::North,
            half: IronTrapdoorHalf::Bottom,
            open: false,
            waterlogged: false,
        })),
        Item::HayBlock => Some(Block::HayBlock(HayBlockData {
            axis: HayBlockAxis::Y,
        })),
        Item::WhiteCarpet => Some(Block::WhiteCarpet),
        Item::OrangeCarpet => Some(Block::OrangeCarpet),
        Item::MagentaCarpet => Some(Block::MagentaCarpet),
        Item::LightBlueCarpet => Some(Block::LightBlueCarpet),
        Item::YellowCarpet => Some(Block::YellowCarpet),
        Item::LimeCarpet => Some(Block::LimeCarpet),
        Item::PinkCarpet => Some(Block::PinkCarpet),
        Item::GrayCarpet => Some(Block::GrayCarpet),
        Item::LightGrayCarpet => Some(Block::LightGrayCarpet),
        Item::CyanCarpet => Some(Block::CyanCarpet),
        Item::PurpleCarpet => Some(Block::PurpleCarpet),
        Item::BlueCarpet => Some(Block::BlueCarpet),
        Item::BrownCarpet => Some(Block::BrownCarpet),
        Item::GreenCarpet => Some(Block::GreenCarpet),
        Item::RedCarpet => Some(Block::RedCarpet),
        Item::BlackCarpet => Some(Block::BlackCarpet),
        Item::Terracotta => Some(Block::Terracotta),
        Item::CoalBlock => Some(Block::CoalBlock),
        Item::PackedIce => Some(Block::PackedIce),
        Item::AcaciaStairs => Some(Block::AcaciaStairs(AcaciaStairsData {
            waterlogged: false,
            facing: AcaciaStairsFacing::North,
            half: AcaciaStairsHalf::Bottom,
            shape: AcaciaStairsShape::Straight,
        })),
        Item::DarkOakStairs => Some(Block::DarkOakStairs(DarkOakStairsData {
            waterlogged: false,
            shape: DarkOakStairsShape::Straight,
            facing: DarkOakStairsFacing::North,
            half: DarkOakStairsHalf::Bottom,
        })),
        Item::SlimeBlock => Some(Block::SlimeBlock),
        Item::GrassPath => Some(Block::GrassPath),
        Item::Sunflower => Some(Block::Sunflower(SunflowerData {
            half: SunflowerHalf::Lower,
        })),
        Item::Lilac => Some(Block::Lilac(LilacData {
            half: LilacHalf::Lower,
        })),
        Item::RoseBush => Some(Block::RoseBush(RoseBushData {
            half: RoseBushHalf::Lower,
        })),
        Item::Peony => Some(Block::Peony(PeonyData {
            half: PeonyHalf::Lower,
        })),
        Item::TallGrass => Some(Block::TallGrass(TallGrassData {
            half: TallGrassHalf::Lower,
        })),
        Item::LargeFern => Some(Block::LargeFern(LargeFernData {
            half: LargeFernHalf::Lower,
        })),
        Item::WhiteStainedGlass => Some(Block::WhiteStainedGlass),
        Item::OrangeStainedGlass => Some(Block::OrangeStainedGlass),
        Item::MagentaStainedGlass => Some(Block::MagentaStainedGlass),
        Item::LightBlueStainedGlass => Some(Block::LightBlueStainedGlass),
        Item::YellowStainedGlass => Some(Block::YellowStainedGlass),
        Item::LimeStainedGlass => Some(Block::LimeStainedGlass),
        Item::PinkStainedGlass => Some(Block::PinkStainedGlass),
        Item::GrayStainedGlass => Some(Block::GrayStainedGlass),
        Item::LightGrayStainedGlass => Some(Block::LightGrayStainedGlass),
        Item::CyanStainedGlass => Some(Block::CyanStainedGlass),
        Item::PurpleStainedGlass => Some(Block::PurpleStainedGlass),
        Item::BlueStainedGlass => Some(Block::BlueStainedGlass),
        Item::BrownStainedGlass => Some(Block::BrownStainedGlass),
        Item::GreenStainedGlass => Some(Block::GreenStainedGlass),
        Item::RedStainedGlass => Some(Block::RedStainedGlass),
        Item::BlackStainedGlass => Some(Block::BlackStainedGlass),
        Item::WhiteStainedGlassPane => {
            Some(Block::WhiteStainedGlassPane(WhiteStainedGlassPaneData {
                south: false,
                waterlogged: false,
                west: false,
                east: false,
                north: false,
            }))
        }
        Item::OrangeStainedGlassPane => {
            Some(Block::OrangeStainedGlassPane(OrangeStainedGlassPaneData {
                north: false,
                west: false,
                east: false,
                south: false,
                waterlogged: false,
            }))
        }
        Item::MagentaStainedGlassPane => Some(Block::MagentaStainedGlassPane(
            MagentaStainedGlassPaneData {
                south: false,
                west: false,
                north: false,
                east: false,
                waterlogged: false,
            },
        )),
        Item::LightBlueStainedGlassPane => Some(Block::LightBlueStainedGlassPane(
            LightBlueStainedGlassPaneData {
                east: false,
                waterlogged: false,
                west: false,
                north: false,
                south: false,
            },
        )),
        Item::YellowStainedGlassPane => {
            Some(Block::YellowStainedGlassPane(YellowStainedGlassPaneData {
                west: false,
                south: false,
                east: false,
                north: false,
                waterlogged: false,
            }))
        }
        Item::LimeStainedGlassPane => Some(Block::LimeStainedGlassPane(LimeStainedGlassPaneData {
            waterlogged: false,
            east: false,
            north: false,
            south: false,
            west: false,
        })),
        Item::PinkStainedGlassPane => Some(Block::PinkStainedGlassPane(PinkStainedGlassPaneData {
            waterlogged: false,
            south: false,
            north: false,
            west: false,
            east: false,
        })),
        Item::GrayStainedGlassPane => Some(Block::GrayStainedGlassPane(GrayStainedGlassPaneData {
            east: false,
            waterlogged: false,
            north: false,
            south: false,
            west: false,
        })),
        Item::LightGrayStainedGlassPane => Some(Block::LightGrayStainedGlassPane(
            LightGrayStainedGlassPaneData {
                east: false,
                south: false,
                north: false,
                west: false,
                waterlogged: false,
            },
        )),
        Item::CyanStainedGlassPane => Some(Block::CyanStainedGlassPane(CyanStainedGlassPaneData {
            east: false,
            north: false,
            south: false,
            west: false,
            waterlogged: false,
        })),
        Item::PurpleStainedGlassPane => {
            Some(Block::PurpleStainedGlassPane(PurpleStainedGlassPaneData {
                south: false,
                north: false,
                west: false,
                east: false,
                waterlogged: false,
            }))
        }
        Item::BlueStainedGlassPane => Some(Block::BlueStainedGlassPane(BlueStainedGlassPaneData {
            north: false,
            east: false,
            south: false,
            waterlogged: false,
            west: false,
        })),
        Item::BrownStainedGlassPane => {
            Some(Block::BrownStainedGlassPane(BrownStainedGlassPaneData {
                west: false,
                south: false,
                east: false,
                north: false,
                waterlogged: false,
            }))
        }
        Item::GreenStainedGlassPane => {
            Some(Block::GreenStainedGlassPane(GreenStainedGlassPaneData {
                south: false,
                north: false,
                waterlogged: false,
                west: false,
                east: false,
            }))
        }
        Item::RedStainedGlassPane => Some(Block::RedStainedGlassPane(RedStainedGlassPaneData {
            north: false,
            east: false,
            south: false,
            waterlogged: false,
            west: false,
        })),
        Item::BlackStainedGlassPane => {
            Some(Block::BlackStainedGlassPane(BlackStainedGlassPaneData {
                south: false,
                west: false,
                north: false,
                east: false,
                waterlogged: false,
            }))
        }
        Item::Prismarine => Some(Block::Prismarine),
        Item::PrismarineBricks => Some(Block::PrismarineBricks),
        Item::DarkPrismarine => Some(Block::DarkPrismarine),
        Item::PrismarineStairs => Some(Block::PrismarineStairs(PrismarineStairsData {
            shape: PrismarineStairsShape::Straight,
            facing: PrismarineStairsFacing::North,
            waterlogged: false,
            half: PrismarineStairsHalf::Bottom,
        })),
        Item::PrismarineBrickStairs => {
            Some(Block::PrismarineBrickStairs(PrismarineBrickStairsData {
                half: PrismarineBrickStairsHalf::Bottom,
                waterlogged: false,
                facing: PrismarineBrickStairsFacing::North,
                shape: PrismarineBrickStairsShape::Straight,
            }))
        }
        Item::DarkPrismarineStairs => Some(Block::DarkPrismarineStairs(DarkPrismarineStairsData {
            waterlogged: false,
            facing: DarkPrismarineStairsFacing::North,
            shape: DarkPrismarineStairsShape::Straight,
            half: DarkPrismarineStairsHalf::Bottom,
        })),
        Item::SeaLantern => Some(Block::SeaLantern),
        Item::RedSandstone => Some(Block::RedSandstone),
        Item::ChiseledRedSandstone => Some(Block::ChiseledRedSandstone),
        Item::CutRedSandstone => Some(Block::CutRedSandstone),
        Item::RedSandstoneStairs => Some(Block::RedSandstoneStairs(RedSandstoneStairsData {
            facing: RedSandstoneStairsFacing::North,
            shape: RedSandstoneStairsShape::Straight,
            waterlogged: false,
            half: RedSandstoneStairsHalf::Bottom,
        })),
        Item::RepeatingCommandBlock => {
            Some(Block::RepeatingCommandBlock(RepeatingCommandBlockData {
                facing: RepeatingCommandBlockFacing::North,
                conditional: false,
            }))
        }
        Item::ChainCommandBlock => Some(Block::ChainCommandBlock(ChainCommandBlockData {
            conditional: false,
            facing: ChainCommandBlockFacing::North,
        })),
        Item::MagmaBlock => Some(Block::MagmaBlock),
        Item::NetherWartBlock => Some(Block::NetherWartBlock),
        Item::RedNetherBricks => Some(Block::RedNetherBricks),
        Item::BoneBlock => Some(Block::BoneBlock(BoneBlockData {
            axis: BoneBlockAxis::Y,
        })),
        Item::StructureVoid => Some(Block::StructureVoid),
        Item::Observer => Some(Block::Observer(ObserverData {
            facing: ObserverFacing::South,
            powered: false,
        })),
        Item::ShulkerBox => Some(Block::ShulkerBox(ShulkerBoxData {
            facing: ShulkerBoxFacing::Up,
        })),
        Item::WhiteShulkerBox => Some(Block::WhiteShulkerBox(WhiteShulkerBoxData {
            facing: WhiteShulkerBoxFacing::Up,
        })),
        Item::OrangeShulkerBox => Some(Block::OrangeShulkerBox(OrangeShulkerBoxData {
            facing: OrangeShulkerBoxFacing::Up,
        })),
        Item::MagentaShulkerBox => Some(Block::MagentaShulkerBox(MagentaShulkerBoxData {
            facing: MagentaShulkerBoxFacing::Up,
        })),
        Item::LightBlueShulkerBox => Some(Block::LightBlueShulkerBox(LightBlueShulkerBoxData {
            facing: LightBlueShulkerBoxFacing::Up,
        })),
        Item::YellowShulkerBox => Some(Block::YellowShulkerBox(YellowShulkerBoxData {
            facing: YellowShulkerBoxFacing::Up,
        })),
        Item::LimeShulkerBox => Some(Block::LimeShulkerBox(LimeShulkerBoxData {
            facing: LimeShulkerBoxFacing::Up,
        })),
        Item::PinkShulkerBox => Some(Block::PinkShulkerBox(PinkShulkerBoxData {
            facing: PinkShulkerBoxFacing::Up,
        })),
        Item::GrayShulkerBox => Some(Block::GrayShulkerBox(GrayShulkerBoxData {
            facing: GrayShulkerBoxFacing::Up,
        })),
        Item::LightGrayShulkerBox => Some(Block::LightGrayShulkerBox(LightGrayShulkerBoxData {
            facing: LightGrayShulkerBoxFacing::Up,
        })),
        Item::CyanShulkerBox => Some(Block::CyanShulkerBox(CyanShulkerBoxData {
            facing: CyanShulkerBoxFacing::Up,
        })),
        Item::PurpleShulkerBox => Some(Block::PurpleShulkerBox(PurpleShulkerBoxData {
            facing: PurpleShulkerBoxFacing::Up,
        })),
        Item::BlueShulkerBox => Some(Block::BlueShulkerBox(BlueShulkerBoxData {
            facing: BlueShulkerBoxFacing::Up,
        })),
        Item::BrownShulkerBox => Some(Block::BrownShulkerBox(BrownShulkerBoxData {
            facing: BrownShulkerBoxFacing::Up,
        })),
        Item::GreenShulkerBox => Some(Block::GreenShulkerBox(GreenShulkerBoxData {
            facing: GreenShulkerBoxFacing::Up,
        })),
        Item::RedShulkerBox => Some(Block::RedShulkerBox(RedShulkerBoxData {
            facing: RedShulkerBoxFacing::Up,
        })),
        Item::BlackShulkerBox => Some(Block::BlackShulkerBox(BlackShulkerBoxData {
            facing: BlackShulkerBoxFacing::Up,
        })),
        Item::WhiteGlazedTerracotta => {
            Some(Block::WhiteGlazedTerracotta(WhiteGlazedTerracottaData {
                facing: WhiteGlazedTerracottaFacing::North,
            }))
        }
        Item::OrangeGlazedTerracotta => {
            Some(Block::OrangeGlazedTerracotta(OrangeGlazedTerracottaData {
                facing: OrangeGlazedTerracottaFacing::North,
            }))
        }
        Item::MagentaGlazedTerracotta => Some(Block::MagentaGlazedTerracotta(
            MagentaGlazedTerracottaData {
                facing: MagentaGlazedTerracottaFacing::North,
            },
        )),
        Item::LightBlueGlazedTerracotta => Some(Block::LightBlueGlazedTerracotta(
            LightBlueGlazedTerracottaData {
                facing: LightBlueGlazedTerracottaFacing::North,
            },
        )),
        Item::YellowGlazedTerracotta => {
            Some(Block::YellowGlazedTerracotta(YellowGlazedTerracottaData {
                facing: YellowGlazedTerracottaFacing::North,
            }))
        }
        Item::LimeGlazedTerracotta => Some(Block::LimeGlazedTerracotta(LimeGlazedTerracottaData {
            facing: LimeGlazedTerracottaFacing::North,
        })),
        Item::PinkGlazedTerracotta => Some(Block::PinkGlazedTerracotta(PinkGlazedTerracottaData {
            facing: PinkGlazedTerracottaFacing::North,
        })),
        Item::GrayGlazedTerracotta => Some(Block::GrayGlazedTerracotta(GrayGlazedTerracottaData {
            facing: GrayGlazedTerracottaFacing::North,
        })),
        Item::LightGrayGlazedTerracotta => Some(Block::LightGrayGlazedTerracotta(
            LightGrayGlazedTerracottaData {
                facing: LightGrayGlazedTerracottaFacing::North,
            },
        )),
        Item::CyanGlazedTerracotta => Some(Block::CyanGlazedTerracotta(CyanGlazedTerracottaData {
            facing: CyanGlazedTerracottaFacing::North,
        })),
        Item::PurpleGlazedTerracotta => {
            Some(Block::PurpleGlazedTerracotta(PurpleGlazedTerracottaData {
                facing: PurpleGlazedTerracottaFacing::North,
            }))
        }
        Item::BlueGlazedTerracotta => Some(Block::BlueGlazedTerracotta(BlueGlazedTerracottaData {
            facing: BlueGlazedTerracottaFacing::North,
        })),
        Item::BrownGlazedTerracotta => {
            Some(Block::BrownGlazedTerracotta(BrownGlazedTerracottaData {
                facing: BrownGlazedTerracottaFacing::North,
            }))
        }
        Item::GreenGlazedTerracotta => {
            Some(Block::GreenGlazedTerracotta(GreenGlazedTerracottaData {
                facing: GreenGlazedTerracottaFacing::North,
            }))
        }
        Item::RedGlazedTerracotta => Some(Block::RedGlazedTerracotta(RedGlazedTerracottaData {
            facing: RedGlazedTerracottaFacing::North,
        })),
        Item::BlackGlazedTerracotta => {
            Some(Block::BlackGlazedTerracotta(BlackGlazedTerracottaData {
                facing: BlackGlazedTerracottaFacing::North,
            }))
        }
        Item::WhiteConcrete => Some(Block::WhiteConcrete),
        Item::OrangeConcrete => Some(Block::OrangeConcrete),
        Item::MagentaConcrete => Some(Block::MagentaConcrete),
        Item::LightBlueConcrete => Some(Block::LightBlueConcrete),
        Item::YellowConcrete => Some(Block::YellowConcrete),
        Item::LimeConcrete => Some(Block::LimeConcrete),
        Item::PinkConcrete => Some(Block::PinkConcrete),
        Item::GrayConcrete => Some(Block::GrayConcrete),
        Item::LightGrayConcrete => Some(Block::LightGrayConcrete),
        Item::CyanConcrete => Some(Block::CyanConcrete),
        Item::PurpleConcrete => Some(Block::PurpleConcrete),
        Item::BlueConcrete => Some(Block::BlueConcrete),
        Item::BrownConcrete => Some(Block::BrownConcrete),
        Item::GreenConcrete => Some(Block::GreenConcrete),
        Item::RedConcrete => Some(Block::RedConcrete),
        Item::BlackConcrete => Some(Block::BlackConcrete),
        Item::WhiteConcretePowder => Some(Block::WhiteConcretePowder),
        Item::OrangeConcretePowder => Some(Block::OrangeConcretePowder),
        Item::MagentaConcretePowder => Some(Block::MagentaConcretePowder),
        Item::LightBlueConcretePowder => Some(Block::LightBlueConcretePowder),
        Item::YellowConcretePowder => Some(Block::YellowConcretePowder),
        Item::LimeConcretePowder => Some(Block::LimeConcretePowder),
        Item::PinkConcretePowder => Some(Block::PinkConcretePowder),
        Item::GrayConcretePowder => Some(Block::GrayConcretePowder),
        Item::LightGrayConcretePowder => Some(Block::LightGrayConcretePowder),
        Item::CyanConcretePowder => Some(Block::CyanConcretePowder),
        Item::PurpleConcretePowder => Some(Block::PurpleConcretePowder),
        Item::BlueConcretePowder => Some(Block::BlueConcretePowder),
        Item::BrownConcretePowder => Some(Block::BrownConcretePowder),
        Item::GreenConcretePowder => Some(Block::GreenConcretePowder),
        Item::RedConcretePowder => Some(Block::RedConcretePowder),
        Item::BlackConcretePowder => Some(Block::BlackConcretePowder),
        Item::TurtleEgg => Some(Block::TurtleEgg(TurtleEggData {
            hatch: 0i32,
            eggs: 1i32,
        })),
        Item::DeadTubeCoralBlock => Some(Block::DeadTubeCoralBlock),
        Item::DeadBrainCoralBlock => Some(Block::DeadBrainCoralBlock),
        Item::DeadBubbleCoralBlock => Some(Block::DeadBubbleCoralBlock),
        Item::DeadFireCoralBlock => Some(Block::DeadFireCoralBlock),
        Item::DeadHornCoralBlock => Some(Block::DeadHornCoralBlock),
        Item::TubeCoralBlock => Some(Block::TubeCoralBlock),
        Item::BrainCoralBlock => Some(Block::BrainCoralBlock),
        Item::BubbleCoralBlock => Some(Block::BubbleCoralBlock),
        Item::FireCoralBlock => Some(Block::FireCoralBlock),
        Item::HornCoralBlock => Some(Block::HornCoralBlock),
        Item::TubeCoral => Some(Block::TubeCoral(TubeCoralData { waterlogged: true })),
        Item::BrainCoral => Some(Block::BrainCoral(BrainCoralData { waterlogged: true })),
        Item::BubbleCoral => Some(Block::BubbleCoral(BubbleCoralData { waterlogged: true })),
        Item::FireCoral => Some(Block::FireCoral(FireCoralData { waterlogged: true })),
        Item::HornCoral => Some(Block::HornCoral(HornCoralData { waterlogged: true })),
        Item::DeadBrainCoral => Some(Block::DeadBrainCoral(DeadBrainCoralData {
            waterlogged: true,
        })),
        Item::DeadBubbleCoral => Some(Block::DeadBubbleCoral(DeadBubbleCoralData {
            waterlogged: true,
        })),
        Item::DeadFireCoral => Some(Block::DeadFireCoral(DeadFireCoralData {
            waterlogged: true,
        })),
        Item::DeadHornCoral => Some(Block::DeadHornCoral(DeadHornCoralData {
            waterlogged: true,
        })),
        Item::DeadTubeCoral => Some(Block::DeadTubeCoral(DeadTubeCoralData {
            waterlogged: true,
        })),
        Item::TubeCoralFan => Some(Block::TubeCoralFan(TubeCoralFanData { waterlogged: true })),
        Item::BrainCoralFan => Some(Block::BrainCoralFan(BrainCoralFanData {
            waterlogged: true,
        })),
        Item::BubbleCoralFan => Some(Block::BubbleCoralFan(BubbleCoralFanData {
            waterlogged: true,
        })),
        Item::FireCoralFan => Some(Block::FireCoralFan(FireCoralFanData { waterlogged: true })),
        Item::HornCoralFan => Some(Block::HornCoralFan(HornCoralFanData { waterlogged: true })),
        Item::DeadTubeCoralFan => Some(Block::DeadTubeCoralFan(DeadTubeCoralFanData {
            waterlogged: true,
        })),
        Item::DeadBrainCoralFan => Some(Block::DeadBrainCoralFan(DeadBrainCoralFanData {
            waterlogged: true,
        })),
        Item::DeadBubbleCoralFan => Some(Block::DeadBubbleCoralFan(DeadBubbleCoralFanData {
            waterlogged: true,
        })),
        Item::DeadFireCoralFan => Some(Block::DeadFireCoralFan(DeadFireCoralFanData {
            waterlogged: true,
        })),
        Item::DeadHornCoralFan => Some(Block::DeadHornCoralFan(DeadHornCoralFanData {
            waterlogged: true,
        })),
        Item::BlueIce => Some(Block::BlueIce),
        Item::Conduit => Some(Block::Conduit(ConduitData { waterlogged: true })),
        Item::IronDoor => Some(Block::IronDoor(IronDoorData {
            half: IronDoorHalf::Lower,
            hinge: IronDoorHinge::Left,
            powered: false,
            open: false,
            facing: IronDoorFacing::North,
        })),
        Item::OakDoor => Some(Block::OakDoor(OakDoorData {
            open: false,
            powered: false,
            facing: OakDoorFacing::North,
            half: OakDoorHalf::Lower,
            hinge: OakDoorHinge::Left,
        })),
        Item::SpruceDoor => Some(Block::SpruceDoor(SpruceDoorData {
            open: false,
            half: SpruceDoorHalf::Lower,
            facing: SpruceDoorFacing::North,
            powered: false,
            hinge: SpruceDoorHinge::Left,
        })),
        Item::BirchDoor => Some(Block::BirchDoor(BirchDoorData {
            hinge: BirchDoorHinge::Left,
            open: false,
            powered: false,
            facing: BirchDoorFacing::North,
            half: BirchDoorHalf::Lower,
        })),
        Item::JungleDoor => Some(Block::JungleDoor(JungleDoorData {
            facing: JungleDoorFacing::North,
            half: JungleDoorHalf::Lower,
            open: false,
            powered: false,
            hinge: JungleDoorHinge::Left,
        })),
        Item::AcaciaDoor => Some(Block::AcaciaDoor(AcaciaDoorData {
            half: AcaciaDoorHalf::Lower,
            powered: false,
            facing: AcaciaDoorFacing::North,
            hinge: AcaciaDoorHinge::Left,
            open: false,
        })),
        Item::DarkOakDoor => Some(Block::DarkOakDoor(DarkOakDoorData {
            facing: DarkOakDoorFacing::North,
            powered: false,
            hinge: DarkOakDoorHinge::Left,
            open: false,
            half: DarkOakDoorHalf::Lower,
        })),
        Item::Repeater => Some(Block::Repeater(RepeaterData {
            locked: false,
            delay: 1i32,
            facing: RepeaterFacing::North,
            powered: false,
        })),
        Item::Comparator => Some(Block::Comparator(ComparatorData {
            mode: ComparatorMode::Compare,
            powered: false,
            facing: ComparatorFacing::North,
        })),
        Item::StructureBlock => Some(Block::StructureBlock(StructureBlockData {
            mode: StructureBlockMode::Save,
        })),
        Item::Wheat => Some(Block::Wheat(WheatData { age: 0i32 })),
        Item::Sign => Some(Block::Sign(SignData {
            waterlogged: false,
            rotation: 0i32,
        })),
        Item::SugarCane => Some(Block::SugarCane(SugarCaneData { age: 0i32 })),
        Item::Kelp => Some(Block::Kelp(KelpData { age: 0i32 })),
        Item::DriedKelpBlock => Some(Block::DriedKelpBlock),
        Item::Cake => Some(Block::Cake(CakeData { bites: 0i32 })),
        Item::WhiteBed => Some(Block::WhiteBed(WhiteBedData {
            facing: WhiteBedFacing::North,
            occupied: false,
            part: WhiteBedPart::Foot,
        })),
        Item::OrangeBed => Some(Block::OrangeBed(OrangeBedData {
            facing: OrangeBedFacing::North,
            occupied: false,
            part: OrangeBedPart::Foot,
        })),
        Item::MagentaBed => Some(Block::MagentaBed(MagentaBedData {
            facing: MagentaBedFacing::North,
            part: MagentaBedPart::Foot,
            occupied: false,
        })),
        Item::LightBlueBed => Some(Block::LightBlueBed(LightBlueBedData {
            occupied: false,
            part: LightBlueBedPart::Foot,
            facing: LightBlueBedFacing::North,
        })),
        Item::YellowBed => Some(Block::YellowBed(YellowBedData {
            occupied: false,
            facing: YellowBedFacing::North,
            part: YellowBedPart::Foot,
        })),
        Item::LimeBed => Some(Block::LimeBed(LimeBedData {
            facing: LimeBedFacing::North,
            part: LimeBedPart::Foot,
            occupied: false,
        })),
        Item::PinkBed => Some(Block::PinkBed(PinkBedData {
            occupied: false,
            part: PinkBedPart::Foot,
            facing: PinkBedFacing::North,
        })),
        Item::GrayBed => Some(Block::GrayBed(GrayBedData {
            occupied: false,
            part: GrayBedPart::Foot,
            facing: GrayBedFacing::North,
        })),
        Item::LightGrayBed => Some(Block::LightGrayBed(LightGrayBedData {
            occupied: false,
            part: LightGrayBedPart::Foot,
            facing: LightGrayBedFacing::North,
        })),
        Item::CyanBed => Some(Block::CyanBed(CyanBedData {
            occupied: false,
            part: CyanBedPart::Foot,
            facing: CyanBedFacing::North,
        })),
        Item::PurpleBed => Some(Block::PurpleBed(PurpleBedData {
            occupied: false,
            facing: PurpleBedFacing::North,
            part: PurpleBedPart::Foot,
        })),
        Item::BlueBed => Some(Block::BlueBed(BlueBedData {
            part: BlueBedPart::Foot,
            facing: BlueBedFacing::North,
            occupied: false,
        })),
        Item::BrownBed => Some(Block::BrownBed(BrownBedData {
            occupied: false,
            facing: BrownBedFacing::North,
            part: BrownBedPart::Foot,
        })),
        Item::GreenBed => Some(Block::GreenBed(GreenBedData {
            facing: GreenBedFacing::North,
            occupied: false,
            part: GreenBedPart::Foot,
        })),
        Item::RedBed => Some(Block::RedBed(RedBedData {
            occupied: false,
            facing: RedBedFacing::North,
            part: RedBedPart::Foot,
        })),
        Item::BlackBed => Some(Block::BlackBed(BlackBedData {
            occupied: false,
            part: BlackBedPart::Foot,
            facing: BlackBedFacing::North,
        })),
        Item::NetherWart => Some(Block::NetherWart(NetherWartData { age: 0i32 })),
        Item::BrewingStand => Some(Block::BrewingStand(BrewingStandData {
            has_bottle_0: false,
            has_bottle_1: false,
            has_bottle_2: false,
        })),
        Item::Cauldron => Some(Block::Cauldron(CauldronData { level: 0i32 })),
        Item::FlowerPot => Some(Block::FlowerPot),
        Item::SkeletonSkull => Some(Block::SkeletonSkull(SkeletonSkullData { rotation: 0i32 })),
        Item::WitherSkeletonSkull => Some(Block::WitherSkeletonSkull(WitherSkeletonSkullData {
            rotation: 0i32,
        })),
        Item::PlayerHead => Some(Block::PlayerHead(PlayerHeadData { rotation: 0i32 })),
        Item::ZombieHead => Some(Block::ZombieHead(ZombieHeadData { rotation: 0i32 })),
        Item::CreeperHead => Some(Block::CreeperHead(CreeperHeadData { rotation: 0i32 })),
        Item::DragonHead => Some(Block::DragonHead(DragonHeadData { rotation: 0i32 })),
        Item::WhiteBanner => Some(Block::WhiteBanner(WhiteBannerData { rotation: 0i32 })),
        Item::OrangeBanner => Some(Block::OrangeBanner(OrangeBannerData { rotation: 0i32 })),
        Item::MagentaBanner => Some(Block::MagentaBanner(MagentaBannerData { rotation: 0i32 })),
        Item::LightBlueBanner => Some(Block::LightBlueBanner(LightBlueBannerData {
            rotation: 0i32,
        })),
        Item::YellowBanner => Some(Block::YellowBanner(YellowBannerData { rotation: 0i32 })),
        Item::LimeBanner => Some(Block::LimeBanner(LimeBannerData { rotation: 0i32 })),
        Item::PinkBanner => Some(Block::PinkBanner(PinkBannerData { rotation: 0i32 })),
        Item::GrayBanner => Some(Block::GrayBanner(GrayBannerData { rotation: 0i32 })),
        Item::LightGrayBanner => Some(Block::LightGrayBanner(LightGrayBannerData {
            rotation: 0i32,
        })),
        Item::CyanBanner => Some(Block::CyanBanner(CyanBannerData { rotation: 0i32 })),
        Item::PurpleBanner => Some(Block::PurpleBanner(PurpleBannerData { rotation: 0i32 })),
        Item::BlueBanner => Some(Block::BlueBanner(BlueBannerData { rotation: 0i32 })),
        Item::BrownBanner => Some(Block::BrownBanner(BrownBannerData { rotation: 0i32 })),
        Item::GreenBanner => Some(Block::GreenBanner(GreenBannerData { rotation: 0i32 })),
        Item::RedBanner => Some(Block::RedBanner(RedBannerData { rotation: 0i32 })),
        Item::BlackBanner => Some(Block::BlackBanner(BlackBannerData { rotation: 0i32 })),
        _ => None,
    }
}
