#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Block {
    Air,
    Stone,
    Granite,
    PolishedGranite,
    Diorite,
    PolishedDiorite,
    Andesite,
    PolishedAndesite,
    GrassBlock(GrassBlockData),
    Dirt,
    CoarseDirt,
    Podzol(PodzolData),
    Cobblestone,
    OakPlanks,
    SprucePlanks,
    BirchPlanks,
    JunglePlanks,
    AcaciaPlanks,
    DarkOakPlanks,
    OakSapling(OakSaplingData),
    SpruceSapling(SpruceSaplingData),
    BirchSapling(BirchSaplingData),
    JungleSapling(JungleSaplingData),
    AcaciaSapling(AcaciaSaplingData),
    DarkOakSapling(DarkOakSaplingData),
    Bedrock,
    Water(WaterData),
    Lava(LavaData),
    Sand,
    RedSand,
    Gravel,
    GoldOre,
    IronOre,
    CoalOre,
    OakLog(OakLogData),
    SpruceLog(SpruceLogData),
    BirchLog(BirchLogData),
    JungleLog(JungleLogData),
    AcaciaLog(AcaciaLogData),
    DarkOakLog(DarkOakLogData),
    StrippedSpruceLog(StrippedSpruceLogData),
    StrippedBirchLog(StrippedBirchLogData),
    StrippedJungleLog(StrippedJungleLogData),
    StrippedAcaciaLog(StrippedAcaciaLogData),
    StrippedDarkOakLog(StrippedDarkOakLogData),
    StrippedOakLog(StrippedOakLogData),
    OakWood(OakWoodData),
    SpruceWood(SpruceWoodData),
    BirchWood(BirchWoodData),
    JungleWood(JungleWoodData),
    AcaciaWood(AcaciaWoodData),
    DarkOakWood(DarkOakWoodData),
    StrippedOakWood(StrippedOakWoodData),
    StrippedSpruceWood(StrippedSpruceWoodData),
    StrippedBirchWood(StrippedBirchWoodData),
    StrippedJungleWood(StrippedJungleWoodData),
    StrippedAcaciaWood(StrippedAcaciaWoodData),
    StrippedDarkOakWood(StrippedDarkOakWoodData),
    OakLeaves(OakLeavesData),
    SpruceLeaves(SpruceLeavesData),
    BirchLeaves(BirchLeavesData),
    JungleLeaves(JungleLeavesData),
    AcaciaLeaves(AcaciaLeavesData),
    DarkOakLeaves(DarkOakLeavesData),
    Sponge,
    WetSponge,
    Glass,
    LapisOre,
    LapisBlock,
    Dispenser(DispenserData),
    Sandstone,
    ChiseledSandstone,
    CutSandstone,
    NoteBlock(NoteBlockData),
    WhiteBed(WhiteBedData),
    OrangeBed(OrangeBedData),
    MagentaBed(MagentaBedData),
    LightBlueBed(LightBlueBedData),
    YellowBed(YellowBedData),
    LimeBed(LimeBedData),
    PinkBed(PinkBedData),
    GrayBed(GrayBedData),
    LightGrayBed(LightGrayBedData),
    CyanBed(CyanBedData),
    PurpleBed(PurpleBedData),
    BlueBed(BlueBedData),
    BrownBed(BrownBedData),
    GreenBed(GreenBedData),
    RedBed(RedBedData),
    BlackBed(BlackBedData),
    PoweredRail(PoweredRailData),
    DetectorRail(DetectorRailData),
    StickyPiston(StickyPistonData),
    Cobweb,
    Grass,
    Fern,
    DeadBush,
    Seagrass,
    TallSeagrass(TallSeagrassData),
    Piston(PistonData),
    PistonHead(PistonHeadData),
    WhiteWool,
    OrangeWool,
    MagentaWool,
    LightBlueWool,
    YellowWool,
    LimeWool,
    PinkWool,
    GrayWool,
    LightGrayWool,
    CyanWool,
    PurpleWool,
    BlueWool,
    BrownWool,
    GreenWool,
    RedWool,
    BlackWool,
    MovingPiston(MovingPistonData),
    Dandelion,
    Poppy,
    BlueOrchid,
    Allium,
    AzureBluet,
    RedTulip,
    OrangeTulip,
    WhiteTulip,
    PinkTulip,
    OxeyeDaisy,
    BrownMushroom,
    RedMushroom,
    GoldBlock,
    IronBlock,
    Bricks,
    Tnt(TntData),
    Bookshelf,
    MossyCobblestone,
    Obsidian,
    Torch,
    WallTorch(WallTorchData),
    Fire(FireData),
    Spawner,
    OakStairs(OakStairsData),
    Chest(ChestData),
    RedstoneWire(RedstoneWireData),
    DiamondOre,
    DiamondBlock,
    CraftingTable,
    Wheat(WheatData),
    Farmland(FarmlandData),
    Furnace(FurnaceData),
    Sign(SignData),
    OakDoor(OakDoorData),
    Ladder(LadderData),
    Rail(RailData),
    CobblestoneStairs(CobblestoneStairsData),
    WallSign(WallSignData),
    Lever(LeverData),
    StonePressurePlate(StonePressurePlateData),
    IronDoor(IronDoorData),
    OakPressurePlate(OakPressurePlateData),
    SprucePressurePlate(SprucePressurePlateData),
    BirchPressurePlate(BirchPressurePlateData),
    JunglePressurePlate(JunglePressurePlateData),
    AcaciaPressurePlate(AcaciaPressurePlateData),
    DarkOakPressurePlate(DarkOakPressurePlateData),
    RedstoneOre(RedstoneOreData),
    RedstoneTorch(RedstoneTorchData),
    RedstoneWallTorch(RedstoneWallTorchData),
    StoneButton(StoneButtonData),
    Snow(SnowData),
    Ice,
    SnowBlock,
    Cactus(CactusData),
    Clay,
    SugarCane(SugarCaneData),
    Jukebox(JukeboxData),
    OakFence(OakFenceData),
    Pumpkin,
    Netherrack,
    SoulSand,
    Glowstone,
    NetherPortal(NetherPortalData),
    CarvedPumpkin(CarvedPumpkinData),
    JackOLantern(JackOLanternData),
    Cake(CakeData),
    Repeater(RepeaterData),
    WhiteStainedGlass,
    OrangeStainedGlass,
    MagentaStainedGlass,
    LightBlueStainedGlass,
    YellowStainedGlass,
    LimeStainedGlass,
    PinkStainedGlass,
    GrayStainedGlass,
    LightGrayStainedGlass,
    CyanStainedGlass,
    PurpleStainedGlass,
    BlueStainedGlass,
    BrownStainedGlass,
    GreenStainedGlass,
    RedStainedGlass,
    BlackStainedGlass,
    OakTrapdoor(OakTrapdoorData),
    SpruceTrapdoor(SpruceTrapdoorData),
    BirchTrapdoor(BirchTrapdoorData),
    JungleTrapdoor(JungleTrapdoorData),
    AcaciaTrapdoor(AcaciaTrapdoorData),
    DarkOakTrapdoor(DarkOakTrapdoorData),
    InfestedStone,
    InfestedCobblestone,
    InfestedStoneBricks,
    InfestedMossyStoneBricks,
    InfestedCrackedStoneBricks,
    InfestedChiseledStoneBricks,
    StoneBricks,
    MossyStoneBricks,
    CrackedStoneBricks,
    ChiseledStoneBricks,
    BrownMushroomBlock(BrownMushroomBlockData),
    RedMushroomBlock(RedMushroomBlockData),
    MushroomStem(MushroomStemData),
    IronBars(IronBarsData),
    GlassPane(GlassPaneData),
    Melon,
    AttachedPumpkinStem(AttachedPumpkinStemData),
    AttachedMelonStem(AttachedMelonStemData),
    PumpkinStem(PumpkinStemData),
    MelonStem(MelonStemData),
    Vine(VineData),
    OakFenceGate(OakFenceGateData),
    BrickStairs(BrickStairsData),
    StoneBrickStairs(StoneBrickStairsData),
    Mycelium(MyceliumData),
    LilyPad,
    NetherBricks,
    NetherBrickFence(NetherBrickFenceData),
    NetherBrickStairs(NetherBrickStairsData),
    NetherWart(NetherWartData),
    EnchantingTable,
    BrewingStand(BrewingStandData),
    Cauldron(CauldronData),
    EndPortal,
    EndPortalFrame(EndPortalFrameData),
    EndStone,
    DragonEgg,
    RedstoneLamp(RedstoneLampData),
    Cocoa(CocoaData),
    SandstoneStairs(SandstoneStairsData),
    EmeraldOre,
    EnderChest(EnderChestData),
    TripwireHook(TripwireHookData),
    Tripwire(TripwireData),
    EmeraldBlock,
    SpruceStairs(SpruceStairsData),
    BirchStairs(BirchStairsData),
    JungleStairs(JungleStairsData),
    CommandBlock(CommandBlockData),
    Beacon,
    CobblestoneWall(CobblestoneWallData),
    MossyCobblestoneWall(MossyCobblestoneWallData),
    FlowerPot,
    PottedOakSapling,
    PottedSpruceSapling,
    PottedBirchSapling,
    PottedJungleSapling,
    PottedAcaciaSapling,
    PottedDarkOakSapling,
    PottedFern,
    PottedDandelion,
    PottedPoppy,
    PottedBlueOrchid,
    PottedAllium,
    PottedAzureBluet,
    PottedRedTulip,
    PottedOrangeTulip,
    PottedWhiteTulip,
    PottedPinkTulip,
    PottedOxeyeDaisy,
    PottedRedMushroom,
    PottedBrownMushroom,
    PottedDeadBush,
    PottedCactus,
    Carrots(CarrotsData),
    Potatoes(PotatoesData),
    OakButton(OakButtonData),
    SpruceButton(SpruceButtonData),
    BirchButton(BirchButtonData),
    JungleButton(JungleButtonData),
    AcaciaButton(AcaciaButtonData),
    DarkOakButton(DarkOakButtonData),
    SkeletonWallSkull(SkeletonWallSkullData),
    SkeletonSkull(SkeletonSkullData),
    WitherSkeletonWallSkull(WitherSkeletonWallSkullData),
    WitherSkeletonSkull(WitherSkeletonSkullData),
    ZombieWallHead(ZombieWallHeadData),
    ZombieHead(ZombieHeadData),
    PlayerWallHead(PlayerWallHeadData),
    PlayerHead(PlayerHeadData),
    CreeperWallHead(CreeperWallHeadData),
    CreeperHead(CreeperHeadData),
    DragonWallHead(DragonWallHeadData),
    DragonHead(DragonHeadData),
    Anvil(AnvilData),
    ChippedAnvil(ChippedAnvilData),
    DamagedAnvil(DamagedAnvilData),
    TrappedChest(TrappedChestData),
    LightWeightedPressurePlate(LightWeightedPressurePlateData),
    HeavyWeightedPressurePlate(HeavyWeightedPressurePlateData),
    Comparator(ComparatorData),
    DaylightDetector(DaylightDetectorData),
    RedstoneBlock,
    NetherQuartzOre,
    Hopper(HopperData),
    QuartzBlock,
    ChiseledQuartzBlock,
    QuartzPillar(QuartzPillarData),
    QuartzStairs(QuartzStairsData),
    ActivatorRail(ActivatorRailData),
    Dropper(DropperData),
    WhiteTerracotta,
    OrangeTerracotta,
    MagentaTerracotta,
    LightBlueTerracotta,
    YellowTerracotta,
    LimeTerracotta,
    PinkTerracotta,
    GrayTerracotta,
    LightGrayTerracotta,
    CyanTerracotta,
    PurpleTerracotta,
    BlueTerracotta,
    BrownTerracotta,
    GreenTerracotta,
    RedTerracotta,
    BlackTerracotta,
    WhiteStainedGlassPane(WhiteStainedGlassPaneData),
    OrangeStainedGlassPane(OrangeStainedGlassPaneData),
    MagentaStainedGlassPane(MagentaStainedGlassPaneData),
    LightBlueStainedGlassPane(LightBlueStainedGlassPaneData),
    YellowStainedGlassPane(YellowStainedGlassPaneData),
    LimeStainedGlassPane(LimeStainedGlassPaneData),
    PinkStainedGlassPane(PinkStainedGlassPaneData),
    GrayStainedGlassPane(GrayStainedGlassPaneData),
    LightGrayStainedGlassPane(LightGrayStainedGlassPaneData),
    CyanStainedGlassPane(CyanStainedGlassPaneData),
    PurpleStainedGlassPane(PurpleStainedGlassPaneData),
    BlueStainedGlassPane(BlueStainedGlassPaneData),
    BrownStainedGlassPane(BrownStainedGlassPaneData),
    GreenStainedGlassPane(GreenStainedGlassPaneData),
    RedStainedGlassPane(RedStainedGlassPaneData),
    BlackStainedGlassPane(BlackStainedGlassPaneData),
    AcaciaStairs(AcaciaStairsData),
    DarkOakStairs(DarkOakStairsData),
    SlimeBlock,
    Barrier,
    IronTrapdoor(IronTrapdoorData),
    Prismarine,
    PrismarineBricks,
    DarkPrismarine,
    PrismarineStairs(PrismarineStairsData),
    PrismarineBrickStairs(PrismarineBrickStairsData),
    DarkPrismarineStairs(DarkPrismarineStairsData),
    PrismarineSlab(PrismarineSlabData),
    PrismarineBrickSlab(PrismarineBrickSlabData),
    DarkPrismarineSlab(DarkPrismarineSlabData),
    SeaLantern,
    HayBlock(HayBlockData),
    WhiteCarpet,
    OrangeCarpet,
    MagentaCarpet,
    LightBlueCarpet,
    YellowCarpet,
    LimeCarpet,
    PinkCarpet,
    GrayCarpet,
    LightGrayCarpet,
    CyanCarpet,
    PurpleCarpet,
    BlueCarpet,
    BrownCarpet,
    GreenCarpet,
    RedCarpet,
    BlackCarpet,
    Terracotta,
    CoalBlock,
    PackedIce,
    Sunflower(SunflowerData),
    Lilac(LilacData),
    RoseBush(RoseBushData),
    Peony(PeonyData),
    TallGrass(TallGrassData),
    LargeFern(LargeFernData),
    WhiteBanner(WhiteBannerData),
    OrangeBanner(OrangeBannerData),
    MagentaBanner(MagentaBannerData),
    LightBlueBanner(LightBlueBannerData),
    YellowBanner(YellowBannerData),
    LimeBanner(LimeBannerData),
    PinkBanner(PinkBannerData),
    GrayBanner(GrayBannerData),
    LightGrayBanner(LightGrayBannerData),
    CyanBanner(CyanBannerData),
    PurpleBanner(PurpleBannerData),
    BlueBanner(BlueBannerData),
    BrownBanner(BrownBannerData),
    GreenBanner(GreenBannerData),
    RedBanner(RedBannerData),
    BlackBanner(BlackBannerData),
    WhiteWallBanner(WhiteWallBannerData),
    OrangeWallBanner(OrangeWallBannerData),
    MagentaWallBanner(MagentaWallBannerData),
    LightBlueWallBanner(LightBlueWallBannerData),
    YellowWallBanner(YellowWallBannerData),
    LimeWallBanner(LimeWallBannerData),
    PinkWallBanner(PinkWallBannerData),
    GrayWallBanner(GrayWallBannerData),
    LightGrayWallBanner(LightGrayWallBannerData),
    CyanWallBanner(CyanWallBannerData),
    PurpleWallBanner(PurpleWallBannerData),
    BlueWallBanner(BlueWallBannerData),
    BrownWallBanner(BrownWallBannerData),
    GreenWallBanner(GreenWallBannerData),
    RedWallBanner(RedWallBannerData),
    BlackWallBanner(BlackWallBannerData),
    RedSandstone,
    ChiseledRedSandstone,
    CutRedSandstone,
    RedSandstoneStairs(RedSandstoneStairsData),
    OakSlab(OakSlabData),
    SpruceSlab(SpruceSlabData),
    BirchSlab(BirchSlabData),
    JungleSlab(JungleSlabData),
    AcaciaSlab(AcaciaSlabData),
    DarkOakSlab(DarkOakSlabData),
    StoneSlab(StoneSlabData),
    SandstoneSlab(SandstoneSlabData),
    PetrifiedOakSlab(PetrifiedOakSlabData),
    CobblestoneSlab(CobblestoneSlabData),
    BrickSlab(BrickSlabData),
    StoneBrickSlab(StoneBrickSlabData),
    NetherBrickSlab(NetherBrickSlabData),
    QuartzSlab(QuartzSlabData),
    RedSandstoneSlab(RedSandstoneSlabData),
    PurpurSlab(PurpurSlabData),
    SmoothStone,
    SmoothSandstone,
    SmoothQuartz,
    SmoothRedSandstone,
    SpruceFenceGate(SpruceFenceGateData),
    BirchFenceGate(BirchFenceGateData),
    JungleFenceGate(JungleFenceGateData),
    AcaciaFenceGate(AcaciaFenceGateData),
    DarkOakFenceGate(DarkOakFenceGateData),
    SpruceFence(SpruceFenceData),
    BirchFence(BirchFenceData),
    JungleFence(JungleFenceData),
    AcaciaFence(AcaciaFenceData),
    DarkOakFence(DarkOakFenceData),
    SpruceDoor(SpruceDoorData),
    BirchDoor(BirchDoorData),
    JungleDoor(JungleDoorData),
    AcaciaDoor(AcaciaDoorData),
    DarkOakDoor(DarkOakDoorData),
    EndRod(EndRodData),
    ChorusPlant(ChorusPlantData),
    ChorusFlower(ChorusFlowerData),
    PurpurBlock,
    PurpurPillar(PurpurPillarData),
    PurpurStairs(PurpurStairsData),
    EndStoneBricks,
    Beetroots(BeetrootsData),
    GrassPath,
    EndGateway,
    RepeatingCommandBlock(RepeatingCommandBlockData),
    ChainCommandBlock(ChainCommandBlockData),
    FrostedIce(FrostedIceData),
    MagmaBlock,
    NetherWartBlock,
    RedNetherBricks,
    BoneBlock(BoneBlockData),
    StructureVoid,
    Observer(ObserverData),
    ShulkerBox(ShulkerBoxData),
    WhiteShulkerBox(WhiteShulkerBoxData),
    OrangeShulkerBox(OrangeShulkerBoxData),
    MagentaShulkerBox(MagentaShulkerBoxData),
    LightBlueShulkerBox(LightBlueShulkerBoxData),
    YellowShulkerBox(YellowShulkerBoxData),
    LimeShulkerBox(LimeShulkerBoxData),
    PinkShulkerBox(PinkShulkerBoxData),
    GrayShulkerBox(GrayShulkerBoxData),
    LightGrayShulkerBox(LightGrayShulkerBoxData),
    CyanShulkerBox(CyanShulkerBoxData),
    PurpleShulkerBox(PurpleShulkerBoxData),
    BlueShulkerBox(BlueShulkerBoxData),
    BrownShulkerBox(BrownShulkerBoxData),
    GreenShulkerBox(GreenShulkerBoxData),
    RedShulkerBox(RedShulkerBoxData),
    BlackShulkerBox(BlackShulkerBoxData),
    WhiteGlazedTerracotta(WhiteGlazedTerracottaData),
    OrangeGlazedTerracotta(OrangeGlazedTerracottaData),
    MagentaGlazedTerracotta(MagentaGlazedTerracottaData),
    LightBlueGlazedTerracotta(LightBlueGlazedTerracottaData),
    YellowGlazedTerracotta(YellowGlazedTerracottaData),
    LimeGlazedTerracotta(LimeGlazedTerracottaData),
    PinkGlazedTerracotta(PinkGlazedTerracottaData),
    GrayGlazedTerracotta(GrayGlazedTerracottaData),
    LightGrayGlazedTerracotta(LightGrayGlazedTerracottaData),
    CyanGlazedTerracotta(CyanGlazedTerracottaData),
    PurpleGlazedTerracotta(PurpleGlazedTerracottaData),
    BlueGlazedTerracotta(BlueGlazedTerracottaData),
    BrownGlazedTerracotta(BrownGlazedTerracottaData),
    GreenGlazedTerracotta(GreenGlazedTerracottaData),
    RedGlazedTerracotta(RedGlazedTerracottaData),
    BlackGlazedTerracotta(BlackGlazedTerracottaData),
    WhiteConcrete,
    OrangeConcrete,
    MagentaConcrete,
    LightBlueConcrete,
    YellowConcrete,
    LimeConcrete,
    PinkConcrete,
    GrayConcrete,
    LightGrayConcrete,
    CyanConcrete,
    PurpleConcrete,
    BlueConcrete,
    BrownConcrete,
    GreenConcrete,
    RedConcrete,
    BlackConcrete,
    WhiteConcretePowder,
    OrangeConcretePowder,
    MagentaConcretePowder,
    LightBlueConcretePowder,
    YellowConcretePowder,
    LimeConcretePowder,
    PinkConcretePowder,
    GrayConcretePowder,
    LightGrayConcretePowder,
    CyanConcretePowder,
    PurpleConcretePowder,
    BlueConcretePowder,
    BrownConcretePowder,
    GreenConcretePowder,
    RedConcretePowder,
    BlackConcretePowder,
    Kelp(KelpData),
    KelpPlant,
    DriedKelpBlock,
    TurtleEgg(TurtleEggData),
    DeadTubeCoralBlock,
    DeadBrainCoralBlock,
    DeadBubbleCoralBlock,
    DeadFireCoralBlock,
    DeadHornCoralBlock,
    TubeCoralBlock,
    BrainCoralBlock,
    BubbleCoralBlock,
    FireCoralBlock,
    HornCoralBlock,
    DeadTubeCoral(DeadTubeCoralData),
    DeadBrainCoral(DeadBrainCoralData),
    DeadBubbleCoral(DeadBubbleCoralData),
    DeadFireCoral(DeadFireCoralData),
    DeadHornCoral(DeadHornCoralData),
    TubeCoral(TubeCoralData),
    BrainCoral(BrainCoralData),
    BubbleCoral(BubbleCoralData),
    FireCoral(FireCoralData),
    HornCoral(HornCoralData),
    DeadTubeCoralWallFan(DeadTubeCoralWallFanData),
    DeadBrainCoralWallFan(DeadBrainCoralWallFanData),
    DeadBubbleCoralWallFan(DeadBubbleCoralWallFanData),
    DeadFireCoralWallFan(DeadFireCoralWallFanData),
    DeadHornCoralWallFan(DeadHornCoralWallFanData),
    TubeCoralWallFan(TubeCoralWallFanData),
    BrainCoralWallFan(BrainCoralWallFanData),
    BubbleCoralWallFan(BubbleCoralWallFanData),
    FireCoralWallFan(FireCoralWallFanData),
    HornCoralWallFan(HornCoralWallFanData),
    DeadTubeCoralFan(DeadTubeCoralFanData),
    DeadBrainCoralFan(DeadBrainCoralFanData),
    DeadBubbleCoralFan(DeadBubbleCoralFanData),
    DeadFireCoralFan(DeadFireCoralFanData),
    DeadHornCoralFan(DeadHornCoralFanData),
    TubeCoralFan(TubeCoralFanData),
    BrainCoralFan(BrainCoralFanData),
    BubbleCoralFan(BubbleCoralFanData),
    FireCoralFan(FireCoralFanData),
    HornCoralFan(HornCoralFanData),
    SeaPickle(SeaPickleData),
    BlueIce,
    Conduit(ConduitData),
    VoidAir,
    CaveAir,
    BubbleColumn(BubbleColumnData),
    StructureBlock(StructureBlockData),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GrassBlockData {
    pub snowy: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PodzolData {
    pub snowy: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OakSaplingData {
    pub stage: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpruceSaplingData {
    pub stage: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BirchSaplingData {
    pub stage: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JungleSaplingData {
    pub stage: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AcaciaSaplingData {
    pub stage: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DarkOakSaplingData {
    pub stage: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WaterData {
    pub level: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LavaData {
    pub level: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OakLogData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpruceLogData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BirchLogData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JungleLogData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AcaciaLogData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DarkOakLogData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StrippedSpruceLogData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StrippedBirchLogData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StrippedJungleLogData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StrippedAcaciaLogData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StrippedDarkOakLogData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StrippedOakLogData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OakWoodData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpruceWoodData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BirchWoodData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JungleWoodData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AcaciaWoodData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DarkOakWoodData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StrippedOakWoodData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StrippedSpruceWoodData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StrippedBirchWoodData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StrippedJungleWoodData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StrippedAcaciaWoodData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StrippedDarkOakWoodData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OakLeavesData {
    pub distance: i32,
    pub persistent: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpruceLeavesData {
    pub distance: i32,
    pub persistent: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BirchLeavesData {
    pub distance: i32,
    pub persistent: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JungleLeavesData {
    pub distance: i32,
    pub persistent: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AcaciaLeavesData {
    pub distance: i32,
    pub persistent: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DarkOakLeavesData {
    pub distance: i32,
    pub persistent: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DispenserData {
    pub facing: Facing,
    pub triggered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NoteBlockData {
    pub instrument: NoteBlockInstrument,
    pub note: i32,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WhiteBedData {
    pub facing: Facing,
    pub occupied: bool,
    pub part: Part,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrangeBedData {
    pub facing: Facing,
    pub occupied: bool,
    pub part: Part,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MagentaBedData {
    pub facing: Facing,
    pub occupied: bool,
    pub part: Part,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LightBlueBedData {
    pub facing: Facing,
    pub occupied: bool,
    pub part: Part,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct YellowBedData {
    pub facing: Facing,
    pub occupied: bool,
    pub part: Part,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LimeBedData {
    pub facing: Facing,
    pub occupied: bool,
    pub part: Part,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PinkBedData {
    pub facing: Facing,
    pub occupied: bool,
    pub part: Part,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GrayBedData {
    pub facing: Facing,
    pub occupied: bool,
    pub part: Part,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LightGrayBedData {
    pub facing: Facing,
    pub occupied: bool,
    pub part: Part,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CyanBedData {
    pub facing: Facing,
    pub occupied: bool,
    pub part: Part,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PurpleBedData {
    pub facing: Facing,
    pub occupied: bool,
    pub part: Part,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlueBedData {
    pub facing: Facing,
    pub occupied: bool,
    pub part: Part,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BrownBedData {
    pub facing: Facing,
    pub occupied: bool,
    pub part: Part,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GreenBedData {
    pub facing: Facing,
    pub occupied: bool,
    pub part: Part,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RedBedData {
    pub facing: Facing,
    pub occupied: bool,
    pub part: Part,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlackBedData {
    pub facing: Facing,
    pub occupied: bool,
    pub part: Part,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PoweredRailData {
    pub powered: bool,
    pub shape: Shape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DetectorRailData {
    pub powered: bool,
    pub shape: Shape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StickyPistonData {
    pub extended: bool,
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TallSeagrassData {
    pub half: Half,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PistonData {
    pub extended: bool,
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PistonHeadData {
    pub facing: Facing,
    pub short: bool,
    pub type_: PistonHeadType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MovingPistonData {
    pub facing: Facing,
    pub type_: MovingPistonType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TntData {
    pub unstable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WallTorchData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FireData {
    pub age: i32,
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OakStairsData {
    pub facing: Facing,
    pub half: Half,
    pub shape: Shape,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChestData {
    pub facing: Facing,
    pub type_: ChestType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RedstoneWireData {
    pub east: RedstoneWireEast,
    pub north: RedstoneWireNorth,
    pub power: i32,
    pub south: RedstoneWireSouth,
    pub west: RedstoneWireWest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WheatData {
    pub age: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FarmlandData {
    pub moisture: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FurnaceData {
    pub facing: Facing,
    pub lit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SignData {
    pub rotation: i32,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OakDoorData {
    pub facing: Facing,
    pub half: Half,
    pub hinge: Hinge,
    pub open: bool,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LadderData {
    pub facing: Facing,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RailData {
    pub shape: Shape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CobblestoneStairsData {
    pub facing: Facing,
    pub half: Half,
    pub shape: Shape,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WallSignData {
    pub facing: Facing,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LeverData {
    pub face: Face,
    pub facing: Facing,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StonePressurePlateData {
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IronDoorData {
    pub facing: Facing,
    pub half: Half,
    pub hinge: Hinge,
    pub open: bool,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OakPressurePlateData {
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SprucePressurePlateData {
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BirchPressurePlateData {
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JunglePressurePlateData {
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AcaciaPressurePlateData {
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DarkOakPressurePlateData {
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RedstoneOreData {
    pub lit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RedstoneTorchData {
    pub lit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RedstoneWallTorchData {
    pub facing: Facing,
    pub lit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StoneButtonData {
    pub face: Face,
    pub facing: Facing,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SnowData {
    pub layers: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CactusData {
    pub age: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SugarCaneData {
    pub age: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JukeboxData {
    pub has_record: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OakFenceData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetherPortalData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CarvedPumpkinData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JackOLanternData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CakeData {
    pub bites: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RepeaterData {
    pub delay: i32,
    pub facing: Facing,
    pub locked: bool,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OakTrapdoorData {
    pub facing: Facing,
    pub half: Half,
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpruceTrapdoorData {
    pub facing: Facing,
    pub half: Half,
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BirchTrapdoorData {
    pub facing: Facing,
    pub half: Half,
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JungleTrapdoorData {
    pub facing: Facing,
    pub half: Half,
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AcaciaTrapdoorData {
    pub facing: Facing,
    pub half: Half,
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DarkOakTrapdoorData {
    pub facing: Facing,
    pub half: Half,
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BrownMushroomBlockData {
    pub down: bool,
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RedMushroomBlockData {
    pub down: bool,
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MushroomStemData {
    pub down: bool,
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IronBarsData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlassPaneData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttachedPumpkinStemData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AttachedMelonStemData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PumpkinStemData {
    pub age: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MelonStemData {
    pub age: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VineData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OakFenceGateData {
    pub facing: Facing,
    pub in_wall: bool,
    pub open: bool,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BrickStairsData {
    pub facing: Facing,
    pub half: Half,
    pub shape: Shape,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StoneBrickStairsData {
    pub facing: Facing,
    pub half: Half,
    pub shape: Shape,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MyceliumData {
    pub snowy: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetherBrickFenceData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetherBrickStairsData {
    pub facing: Facing,
    pub half: Half,
    pub shape: Shape,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetherWartData {
    pub age: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BrewingStandData {
    pub has_bottle_0: bool,
    pub has_bottle_1: bool,
    pub has_bottle_2: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CauldronData {
    pub level: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EndPortalFrameData {
    pub eye: bool,
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RedstoneLampData {
    pub lit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CocoaData {
    pub age: i32,
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SandstoneStairsData {
    pub facing: Facing,
    pub half: Half,
    pub shape: Shape,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnderChestData {
    pub facing: Facing,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TripwireHookData {
    pub attached: bool,
    pub facing: Facing,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TripwireData {
    pub attached: bool,
    pub disarmed: bool,
    pub east: bool,
    pub north: bool,
    pub powered: bool,
    pub south: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpruceStairsData {
    pub facing: Facing,
    pub half: Half,
    pub shape: Shape,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BirchStairsData {
    pub facing: Facing,
    pub half: Half,
    pub shape: Shape,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JungleStairsData {
    pub facing: Facing,
    pub half: Half,
    pub shape: Shape,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CommandBlockData {
    pub conditional: bool,
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CobblestoneWallData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MossyCobblestoneWallData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CarrotsData {
    pub age: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PotatoesData {
    pub age: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OakButtonData {
    pub face: Face,
    pub facing: Facing,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpruceButtonData {
    pub face: Face,
    pub facing: Facing,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BirchButtonData {
    pub face: Face,
    pub facing: Facing,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JungleButtonData {
    pub face: Face,
    pub facing: Facing,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AcaciaButtonData {
    pub face: Face,
    pub facing: Facing,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DarkOakButtonData {
    pub face: Face,
    pub facing: Facing,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SkeletonWallSkullData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SkeletonSkullData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WitherSkeletonWallSkullData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WitherSkeletonSkullData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ZombieWallHeadData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ZombieHeadData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerWallHeadData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerHeadData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CreeperWallHeadData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CreeperHeadData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DragonWallHeadData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DragonHeadData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AnvilData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChippedAnvilData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DamagedAnvilData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TrappedChestData {
    pub facing: Facing,
    pub type_: TrappedChestType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LightWeightedPressurePlateData {
    pub power: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HeavyWeightedPressurePlateData {
    pub power: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComparatorData {
    pub facing: Facing,
    pub mode: ComparatorMode,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DaylightDetectorData {
    pub inverted: bool,
    pub power: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HopperData {
    pub enabled: bool,
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuartzPillarData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuartzStairsData {
    pub facing: Facing,
    pub half: Half,
    pub shape: Shape,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ActivatorRailData {
    pub powered: bool,
    pub shape: Shape,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DropperData {
    pub facing: Facing,
    pub triggered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WhiteStainedGlassPaneData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrangeStainedGlassPaneData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MagentaStainedGlassPaneData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LightBlueStainedGlassPaneData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct YellowStainedGlassPaneData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LimeStainedGlassPaneData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PinkStainedGlassPaneData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GrayStainedGlassPaneData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LightGrayStainedGlassPaneData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CyanStainedGlassPaneData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PurpleStainedGlassPaneData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlueStainedGlassPaneData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BrownStainedGlassPaneData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GreenStainedGlassPaneData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RedStainedGlassPaneData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlackStainedGlassPaneData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AcaciaStairsData {
    pub facing: Facing,
    pub half: Half,
    pub shape: Shape,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DarkOakStairsData {
    pub facing: Facing,
    pub half: Half,
    pub shape: Shape,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IronTrapdoorData {
    pub facing: Facing,
    pub half: Half,
    pub open: bool,
    pub powered: bool,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrismarineStairsData {
    pub facing: Facing,
    pub half: Half,
    pub shape: Shape,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrismarineBrickStairsData {
    pub facing: Facing,
    pub half: Half,
    pub shape: Shape,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DarkPrismarineStairsData {
    pub facing: Facing,
    pub half: Half,
    pub shape: Shape,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrismarineSlabData {
    pub type_: PrismarineSlabType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrismarineBrickSlabData {
    pub type_: PrismarineBrickSlabType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DarkPrismarineSlabData {
    pub type_: DarkPrismarineSlabType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HayBlockData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SunflowerData {
    pub half: Half,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LilacData {
    pub half: Half,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoseBushData {
    pub half: Half,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PeonyData {
    pub half: Half,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TallGrassData {
    pub half: Half,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LargeFernData {
    pub half: Half,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WhiteBannerData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrangeBannerData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MagentaBannerData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LightBlueBannerData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct YellowBannerData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LimeBannerData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PinkBannerData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GrayBannerData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LightGrayBannerData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CyanBannerData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PurpleBannerData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlueBannerData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BrownBannerData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GreenBannerData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RedBannerData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlackBannerData {
    pub rotation: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WhiteWallBannerData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrangeWallBannerData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MagentaWallBannerData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LightBlueWallBannerData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct YellowWallBannerData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LimeWallBannerData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PinkWallBannerData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GrayWallBannerData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LightGrayWallBannerData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CyanWallBannerData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PurpleWallBannerData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlueWallBannerData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BrownWallBannerData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GreenWallBannerData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RedWallBannerData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlackWallBannerData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RedSandstoneStairsData {
    pub facing: Facing,
    pub half: Half,
    pub shape: Shape,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OakSlabData {
    pub type_: OakSlabType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpruceSlabData {
    pub type_: SpruceSlabType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BirchSlabData {
    pub type_: BirchSlabType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JungleSlabData {
    pub type_: JungleSlabType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AcaciaSlabData {
    pub type_: AcaciaSlabType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DarkOakSlabData {
    pub type_: DarkOakSlabType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StoneSlabData {
    pub type_: StoneSlabType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SandstoneSlabData {
    pub type_: SandstoneSlabType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PetrifiedOakSlabData {
    pub type_: PetrifiedOakSlabType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CobblestoneSlabData {
    pub type_: CobblestoneSlabType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BrickSlabData {
    pub type_: BrickSlabType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StoneBrickSlabData {
    pub type_: StoneBrickSlabType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NetherBrickSlabData {
    pub type_: NetherBrickSlabType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuartzSlabData {
    pub type_: QuartzSlabType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RedSandstoneSlabData {
    pub type_: RedSandstoneSlabType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PurpurSlabData {
    pub type_: PurpurSlabType,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpruceFenceGateData {
    pub facing: Facing,
    pub in_wall: bool,
    pub open: bool,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BirchFenceGateData {
    pub facing: Facing,
    pub in_wall: bool,
    pub open: bool,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JungleFenceGateData {
    pub facing: Facing,
    pub in_wall: bool,
    pub open: bool,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AcaciaFenceGateData {
    pub facing: Facing,
    pub in_wall: bool,
    pub open: bool,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DarkOakFenceGateData {
    pub facing: Facing,
    pub in_wall: bool,
    pub open: bool,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpruceFenceData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BirchFenceData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JungleFenceData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AcaciaFenceData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DarkOakFenceData {
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub waterlogged: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpruceDoorData {
    pub facing: Facing,
    pub half: Half,
    pub hinge: Hinge,
    pub open: bool,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BirchDoorData {
    pub facing: Facing,
    pub half: Half,
    pub hinge: Hinge,
    pub open: bool,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JungleDoorData {
    pub facing: Facing,
    pub half: Half,
    pub hinge: Hinge,
    pub open: bool,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AcaciaDoorData {
    pub facing: Facing,
    pub half: Half,
    pub hinge: Hinge,
    pub open: bool,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DarkOakDoorData {
    pub facing: Facing,
    pub half: Half,
    pub hinge: Hinge,
    pub open: bool,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EndRodData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChorusPlantData {
    pub down: bool,
    pub east: bool,
    pub north: bool,
    pub south: bool,
    pub up: bool,
    pub west: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChorusFlowerData {
    pub age: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PurpurPillarData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PurpurStairsData {
    pub facing: Facing,
    pub half: Half,
    pub shape: Shape,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BeetrootsData {
    pub age: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RepeatingCommandBlockData {
    pub conditional: bool,
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChainCommandBlockData {
    pub conditional: bool,
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FrostedIceData {
    pub age: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BoneBlockData {
    pub axis: Axis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObserverData {
    pub facing: Facing,
    pub powered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShulkerBoxData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WhiteShulkerBoxData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrangeShulkerBoxData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MagentaShulkerBoxData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LightBlueShulkerBoxData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct YellowShulkerBoxData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LimeShulkerBoxData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PinkShulkerBoxData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GrayShulkerBoxData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LightGrayShulkerBoxData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CyanShulkerBoxData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PurpleShulkerBoxData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlueShulkerBoxData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BrownShulkerBoxData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GreenShulkerBoxData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RedShulkerBoxData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlackShulkerBoxData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WhiteGlazedTerracottaData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OrangeGlazedTerracottaData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MagentaGlazedTerracottaData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LightBlueGlazedTerracottaData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct YellowGlazedTerracottaData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LimeGlazedTerracottaData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PinkGlazedTerracottaData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GrayGlazedTerracottaData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LightGrayGlazedTerracottaData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CyanGlazedTerracottaData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PurpleGlazedTerracottaData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlueGlazedTerracottaData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BrownGlazedTerracottaData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GreenGlazedTerracottaData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RedGlazedTerracottaData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlackGlazedTerracottaData {
    pub facing: Facing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KelpData {
    pub age: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TurtleEggData {
    pub eggs: i32,
    pub hatch: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeadTubeCoralData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeadBrainCoralData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeadBubbleCoralData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeadFireCoralData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeadHornCoralData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TubeCoralData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BrainCoralData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BubbleCoralData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FireCoralData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HornCoralData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeadTubeCoralWallFanData {
    pub facing: Facing,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeadBrainCoralWallFanData {
    pub facing: Facing,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeadBubbleCoralWallFanData {
    pub facing: Facing,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeadFireCoralWallFanData {
    pub facing: Facing,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeadHornCoralWallFanData {
    pub facing: Facing,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TubeCoralWallFanData {
    pub facing: Facing,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BrainCoralWallFanData {
    pub facing: Facing,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BubbleCoralWallFanData {
    pub facing: Facing,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FireCoralWallFanData {
    pub facing: Facing,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HornCoralWallFanData {
    pub facing: Facing,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeadTubeCoralFanData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeadBrainCoralFanData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeadBubbleCoralFanData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeadFireCoralFanData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeadHornCoralFanData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TubeCoralFanData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BrainCoralFanData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BubbleCoralFanData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FireCoralFanData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HornCoralFanData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SeaPickleData {
    pub pickles: i32,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConduitData {
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BubbleColumnData {
    pub drag: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StructureBlockData {
    pub mode: StructureBlockMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Facing {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

impl Facing {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "north" => Some(Facing::North),
            "south" => Some(Facing::South),
            "east" => Some(Facing::East),
            "west" => Some(Facing::West),
            "up" => Some(Facing::Up),
            "down" => Some(Facing::Down),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "x" => Some(Axis::X),
            "y" => Some(Axis::Y),
            "z" => Some(Axis::Z),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Half {
    Upper,
    Lower,
    Top,
    Bottom,
}

impl Half {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "upper" => Some(Half::Upper),
            "lower" => Some(Half::Lower),
            "top" => Some(Half::Top),
            "bottom" => Some(Half::Bottom),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Face {
    Floor,
    Wall,
    Ceiling,
}

impl Face {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "floor" => Some(Face::Floor),
            "wall" => Some(Face::Wall),
            "ceiling" => Some(Face::Ceiling),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Shape {
    Straight,
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
    AscendingNorth,
    AscendingEast,
    AscendingSouth,
    AscendingWest,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    NorthSouth,
    EastWest,
}
impl Shape {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "straight" => Some(Shape::Straight),
            "inner_left" => Some(Shape::InnerLeft),
            "inner_right" => Some(Shape::InnerRight),
            "outer_left" => Some(Shape::OuterLeft),
            "ascending_north" => Some(Shape::AscendingNorth),
            "ascending_east" => Some(Shape::AscendingEast),
            "ascending_south" => Some(Shape::AscendingSouth),
            "ascending_west" => Some(Shape::AscendingWest),
            "north_east" => Some(Shape::NorthEast),
            "north_west" => Some(Shape::NorthWest),
            "south_east" => Some(Shape::SouthEast),
            "south_west" => Some(Shape::SouthWest),
            "north_south" => Some(Shape::NorthSouth),
            "east_west" => Some(Shape::EastWest),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Hinge {
    Left,
    Right,
}

impl Hinge {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "left" => Some(Hinge::Left),
            "right" => Some(Hinge::Right),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Part {
    Head,
    Foot,
}

impl Part {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "head" => Some(Part::Head),
            "foot" => Some(Part::Foot),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoteBlockInstrument {
    Harp,
    Basedrum,
    Snare,
    Hat,
    Bass,
    Flute,
    Bell,
    Guitar,
    Chime,
    Xylophone,
}

impl NoteBlockInstrument {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "harp" => Some(NoteBlockInstrument::Harp),
            "basedrum" => Some(NoteBlockInstrument::Basedrum),
            "snare" => Some(NoteBlockInstrument::Snare),
            "hat" => Some(NoteBlockInstrument::Hat),
            "bass" => Some(NoteBlockInstrument::Bass),
            "flute" => Some(NoteBlockInstrument::Flute),
            "bell" => Some(NoteBlockInstrument::Bell),
            "guitar" => Some(NoteBlockInstrument::Guitar),
            "chime" => Some(NoteBlockInstrument::Chime),
            "xylophone" => Some(NoteBlockInstrument::Xylophone),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PistonHeadType {
    Normal,
    Sticky,
}

impl PistonHeadType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "normal" => Some(PistonHeadType::Normal),
            "sticky" => Some(PistonHeadType::Sticky),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MovingPistonType {
    Normal,
    Sticky,
}

impl MovingPistonType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "normal" => Some(MovingPistonType::Normal),
            "sticky" => Some(MovingPistonType::Sticky),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChestType {
    Single,
    Left,
    Right,
}

impl ChestType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "single" => Some(ChestType::Single),
            "left" => Some(ChestType::Left),
            "right" => Some(ChestType::Right),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RedstoneWireEast {
    Up,
    Side,
    None,
}

impl RedstoneWireEast {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "up" => Some(RedstoneWireEast::Up),
            "side" => Some(RedstoneWireEast::Side),
            "none" => Some(RedstoneWireEast::None),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RedstoneWireNorth {
    Up,
    Side,
    None,
}

impl RedstoneWireNorth {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "up" => Some(RedstoneWireNorth::Up),
            "side" => Some(RedstoneWireNorth::Side),
            "none" => Some(RedstoneWireNorth::None),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RedstoneWireSouth {
    Up,
    Side,
    None,
}

impl RedstoneWireSouth {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "up" => Some(RedstoneWireSouth::Up),
            "side" => Some(RedstoneWireSouth::Side),
            "none" => Some(RedstoneWireSouth::None),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RedstoneWireWest {
    Up,
    Side,
    None,
}

impl RedstoneWireWest {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "up" => Some(RedstoneWireWest::Up),
            "side" => Some(RedstoneWireWest::Side),
            "none" => Some(RedstoneWireWest::None),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TrappedChestType {
    Single,
    Left,
    Right,
}

impl TrappedChestType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "single" => Some(TrappedChestType::Single),
            "left" => Some(TrappedChestType::Left),
            "right" => Some(TrappedChestType::Right),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComparatorMode {
    Compare,
    Subtract,
}

impl ComparatorMode {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "compare" => Some(ComparatorMode::Compare),
            "subtract" => Some(ComparatorMode::Subtract),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrismarineSlabType {
    Top,
    Bottom,
    Double,
}

impl PrismarineSlabType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "top" => Some(PrismarineSlabType::Top),
            "bottom" => Some(PrismarineSlabType::Bottom),
            "double" => Some(PrismarineSlabType::Double),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrismarineBrickSlabType {
    Top,
    Bottom,
    Double,
}

impl PrismarineBrickSlabType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "top" => Some(PrismarineBrickSlabType::Top),
            "bottom" => Some(PrismarineBrickSlabType::Bottom),
            "double" => Some(PrismarineBrickSlabType::Double),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DarkPrismarineSlabType {
    Top,
    Bottom,
    Double,
}

impl DarkPrismarineSlabType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "top" => Some(DarkPrismarineSlabType::Top),
            "bottom" => Some(DarkPrismarineSlabType::Bottom),
            "double" => Some(DarkPrismarineSlabType::Double),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OakSlabType {
    Top,
    Bottom,
    Double,
}

impl OakSlabType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "top" => Some(OakSlabType::Top),
            "bottom" => Some(OakSlabType::Bottom),
            "double" => Some(OakSlabType::Double),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpruceSlabType {
    Top,
    Bottom,
    Double,
}

impl SpruceSlabType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "top" => Some(SpruceSlabType::Top),
            "bottom" => Some(SpruceSlabType::Bottom),
            "double" => Some(SpruceSlabType::Double),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BirchSlabType {
    Top,
    Bottom,
    Double,
}

impl BirchSlabType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "top" => Some(BirchSlabType::Top),
            "bottom" => Some(BirchSlabType::Bottom),
            "double" => Some(BirchSlabType::Double),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JungleSlabType {
    Top,
    Bottom,
    Double,
}

impl JungleSlabType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "top" => Some(JungleSlabType::Top),
            "bottom" => Some(JungleSlabType::Bottom),
            "double" => Some(JungleSlabType::Double),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AcaciaSlabType {
    Top,
    Bottom,
    Double,
}

impl AcaciaSlabType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "top" => Some(AcaciaSlabType::Top),
            "bottom" => Some(AcaciaSlabType::Bottom),
            "double" => Some(AcaciaSlabType::Double),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DarkOakSlabType {
    Top,
    Bottom,
    Double,
}

impl DarkOakSlabType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "top" => Some(DarkOakSlabType::Top),
            "bottom" => Some(DarkOakSlabType::Bottom),
            "double" => Some(DarkOakSlabType::Double),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoneSlabType {
    Top,
    Bottom,
    Double,
}

impl StoneSlabType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "top" => Some(StoneSlabType::Top),
            "bottom" => Some(StoneSlabType::Bottom),
            "double" => Some(StoneSlabType::Double),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SandstoneSlabType {
    Top,
    Bottom,
    Double,
}

impl SandstoneSlabType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "top" => Some(SandstoneSlabType::Top),
            "bottom" => Some(SandstoneSlabType::Bottom),
            "double" => Some(SandstoneSlabType::Double),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PetrifiedOakSlabType {
    Top,
    Bottom,
    Double,
}

impl PetrifiedOakSlabType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "top" => Some(PetrifiedOakSlabType::Top),
            "bottom" => Some(PetrifiedOakSlabType::Bottom),
            "double" => Some(PetrifiedOakSlabType::Double),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CobblestoneSlabType {
    Top,
    Bottom,
    Double,
}

impl CobblestoneSlabType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "top" => Some(CobblestoneSlabType::Top),
            "bottom" => Some(CobblestoneSlabType::Bottom),
            "double" => Some(CobblestoneSlabType::Double),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BrickSlabType {
    Top,
    Bottom,
    Double,
}

impl BrickSlabType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "top" => Some(BrickSlabType::Top),
            "bottom" => Some(BrickSlabType::Bottom),
            "double" => Some(BrickSlabType::Double),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoneBrickSlabType {
    Top,
    Bottom,
    Double,
}

impl StoneBrickSlabType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "top" => Some(StoneBrickSlabType::Top),
            "bottom" => Some(StoneBrickSlabType::Bottom),
            "double" => Some(StoneBrickSlabType::Double),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NetherBrickSlabType {
    Top,
    Bottom,
    Double,
}

impl NetherBrickSlabType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "top" => Some(NetherBrickSlabType::Top),
            "bottom" => Some(NetherBrickSlabType::Bottom),
            "double" => Some(NetherBrickSlabType::Double),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuartzSlabType {
    Top,
    Bottom,
    Double,
}

impl QuartzSlabType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "top" => Some(QuartzSlabType::Top),
            "bottom" => Some(QuartzSlabType::Bottom),
            "double" => Some(QuartzSlabType::Double),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RedSandstoneSlabType {
    Top,
    Bottom,
    Double,
}

impl RedSandstoneSlabType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "top" => Some(RedSandstoneSlabType::Top),
            "bottom" => Some(RedSandstoneSlabType::Bottom),
            "double" => Some(RedSandstoneSlabType::Double),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PurpurSlabType {
    Top,
    Bottom,
    Double,
}

impl PurpurSlabType {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "top" => Some(PurpurSlabType::Top),
            "bottom" => Some(PurpurSlabType::Bottom),
            "double" => Some(PurpurSlabType::Double),
            _ => None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StructureBlockMode {
    Save,
    Load,
    Corner,
    Data,
}

impl StructureBlockMode {
    pub fn from_identifier(i: &str) -> Option<Self> {
        match i {
            "save" => Some(StructureBlockMode::Save),
            "load" => Some(StructureBlockMode::Load),
            "corner" => Some(StructureBlockMode::Corner),
            "data" => Some(StructureBlockMode::Data),
            _ => None,
        }
    }
}
