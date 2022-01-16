// This file is @generated. Please do not edit.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    serde :: Serialize,
    serde :: Deserialize,
)]
#[serde(try_from = "String", into = "&'static str")]
pub enum Item {
    Stone,
    Granite,
    PolishedGranite,
    Diorite,
    PolishedDiorite,
    Andesite,
    PolishedAndesite,
    Deepslate,
    CobbledDeepslate,
    PolishedDeepslate,
    Calcite,
    Tuff,
    DripstoneBlock,
    GrassBlock,
    Dirt,
    CoarseDirt,
    Podzol,
    RootedDirt,
    CrimsonNylium,
    WarpedNylium,
    Cobblestone,
    OakPlanks,
    SprucePlanks,
    BirchPlanks,
    JunglePlanks,
    AcaciaPlanks,
    DarkOakPlanks,
    CrimsonPlanks,
    WarpedPlanks,
    OakSapling,
    SpruceSapling,
    BirchSapling,
    JungleSapling,
    AcaciaSapling,
    DarkOakSapling,
    Bedrock,
    Sand,
    RedSand,
    Gravel,
    CoalOre,
    DeepslateCoalOre,
    IronOre,
    DeepslateIronOre,
    CopperOre,
    DeepslateCopperOre,
    GoldOre,
    DeepslateGoldOre,
    RedstoneOre,
    DeepslateRedstoneOre,
    EmeraldOre,
    DeepslateEmeraldOre,
    LapisOre,
    DeepslateLapisOre,
    DiamondOre,
    DeepslateDiamondOre,
    NetherGoldOre,
    NetherQuartzOre,
    AncientDebris,
    CoalBlock,
    RawIronBlock,
    RawCopperBlock,
    RawGoldBlock,
    AmethystBlock,
    BuddingAmethyst,
    IronBlock,
    CopperBlock,
    GoldBlock,
    DiamondBlock,
    NetheriteBlock,
    ExposedCopper,
    WeatheredCopper,
    OxidizedCopper,
    CutCopper,
    ExposedCutCopper,
    WeatheredCutCopper,
    OxidizedCutCopper,
    CutCopperStairs,
    ExposedCutCopperStairs,
    WeatheredCutCopperStairs,
    OxidizedCutCopperStairs,
    CutCopperSlab,
    ExposedCutCopperSlab,
    WeatheredCutCopperSlab,
    OxidizedCutCopperSlab,
    WaxedCopperBlock,
    WaxedExposedCopper,
    WaxedWeatheredCopper,
    WaxedOxidizedCopper,
    WaxedCutCopper,
    WaxedExposedCutCopper,
    WaxedWeatheredCutCopper,
    WaxedOxidizedCutCopper,
    WaxedCutCopperStairs,
    WaxedExposedCutCopperStairs,
    WaxedWeatheredCutCopperStairs,
    WaxedOxidizedCutCopperStairs,
    WaxedCutCopperSlab,
    WaxedExposedCutCopperSlab,
    WaxedWeatheredCutCopperSlab,
    WaxedOxidizedCutCopperSlab,
    OakLog,
    SpruceLog,
    BirchLog,
    JungleLog,
    AcaciaLog,
    DarkOakLog,
    CrimsonStem,
    WarpedStem,
    StrippedOakLog,
    StrippedSpruceLog,
    StrippedBirchLog,
    StrippedJungleLog,
    StrippedAcaciaLog,
    StrippedDarkOakLog,
    StrippedCrimsonStem,
    StrippedWarpedStem,
    StrippedOakWood,
    StrippedSpruceWood,
    StrippedBirchWood,
    StrippedJungleWood,
    StrippedAcaciaWood,
    StrippedDarkOakWood,
    StrippedCrimsonHyphae,
    StrippedWarpedHyphae,
    OakWood,
    SpruceWood,
    BirchWood,
    JungleWood,
    AcaciaWood,
    DarkOakWood,
    CrimsonHyphae,
    WarpedHyphae,
    OakLeaves,
    SpruceLeaves,
    BirchLeaves,
    JungleLeaves,
    AcaciaLeaves,
    DarkOakLeaves,
    AzaleaLeaves,
    FloweringAzaleaLeaves,
    Sponge,
    WetSponge,
    Glass,
    TintedGlass,
    LapisBlock,
    Sandstone,
    ChiseledSandstone,
    CutSandstone,
    Cobweb,
    Grass,
    Fern,
    Azalea,
    FloweringAzalea,
    DeadBush,
    Seagrass,
    SeaPickle,
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
    Cornflower,
    LilyOfTheValley,
    WitherRose,
    SporeBlossom,
    BrownMushroom,
    RedMushroom,
    CrimsonFungus,
    WarpedFungus,
    CrimsonRoots,
    WarpedRoots,
    NetherSprouts,
    WeepingVines,
    TwistingVines,
    SugarCane,
    Kelp,
    MossCarpet,
    MossBlock,
    HangingRoots,
    BigDripleaf,
    SmallDripleaf,
    Bamboo,
    OakSlab,
    SpruceSlab,
    BirchSlab,
    JungleSlab,
    AcaciaSlab,
    DarkOakSlab,
    CrimsonSlab,
    WarpedSlab,
    StoneSlab,
    SmoothStoneSlab,
    SandstoneSlab,
    CutSandstoneSlab,
    PetrifiedOakSlab,
    CobblestoneSlab,
    BrickSlab,
    StoneBrickSlab,
    NetherBrickSlab,
    QuartzSlab,
    RedSandstoneSlab,
    CutRedSandstoneSlab,
    PurpurSlab,
    PrismarineSlab,
    PrismarineBrickSlab,
    DarkPrismarineSlab,
    SmoothQuartz,
    SmoothRedSandstone,
    SmoothSandstone,
    SmoothStone,
    Bricks,
    Bookshelf,
    MossyCobblestone,
    Obsidian,
    Torch,
    EndRod,
    ChorusPlant,
    ChorusFlower,
    PurpurBlock,
    PurpurPillar,
    PurpurStairs,
    Spawner,
    OakStairs,
    Chest,
    CraftingTable,
    Farmland,
    Furnace,
    Ladder,
    CobblestoneStairs,
    Snow,
    Ice,
    SnowBlock,
    Cactus,
    Clay,
    Jukebox,
    OakFence,
    SpruceFence,
    BirchFence,
    JungleFence,
    AcaciaFence,
    DarkOakFence,
    CrimsonFence,
    WarpedFence,
    Pumpkin,
    CarvedPumpkin,
    JackOLantern,
    Netherrack,
    SoulSand,
    SoulSoil,
    Basalt,
    PolishedBasalt,
    SmoothBasalt,
    SoulTorch,
    Glowstone,
    InfestedStone,
    InfestedCobblestone,
    InfestedStoneBricks,
    InfestedMossyStoneBricks,
    InfestedCrackedStoneBricks,
    InfestedChiseledStoneBricks,
    InfestedDeepslate,
    StoneBricks,
    MossyStoneBricks,
    CrackedStoneBricks,
    ChiseledStoneBricks,
    DeepslateBricks,
    CrackedDeepslateBricks,
    DeepslateTiles,
    CrackedDeepslateTiles,
    ChiseledDeepslate,
    BrownMushroomBlock,
    RedMushroomBlock,
    MushroomStem,
    IronBars,
    Chain,
    GlassPane,
    Melon,
    Vine,
    GlowLichen,
    BrickStairs,
    StoneBrickStairs,
    Mycelium,
    LilyPad,
    NetherBricks,
    CrackedNetherBricks,
    ChiseledNetherBricks,
    NetherBrickFence,
    NetherBrickStairs,
    EnchantingTable,
    EndPortalFrame,
    EndStone,
    EndStoneBricks,
    DragonEgg,
    SandstoneStairs,
    EnderChest,
    EmeraldBlock,
    SpruceStairs,
    BirchStairs,
    JungleStairs,
    CrimsonStairs,
    WarpedStairs,
    CommandBlock,
    Beacon,
    CobblestoneWall,
    MossyCobblestoneWall,
    BrickWall,
    PrismarineWall,
    RedSandstoneWall,
    MossyStoneBrickWall,
    GraniteWall,
    StoneBrickWall,
    NetherBrickWall,
    AndesiteWall,
    RedNetherBrickWall,
    SandstoneWall,
    EndStoneBrickWall,
    DioriteWall,
    BlackstoneWall,
    PolishedBlackstoneWall,
    PolishedBlackstoneBrickWall,
    CobbledDeepslateWall,
    PolishedDeepslateWall,
    DeepslateBrickWall,
    DeepslateTileWall,
    Anvil,
    ChippedAnvil,
    DamagedAnvil,
    ChiseledQuartzBlock,
    QuartzBlock,
    QuartzBricks,
    QuartzPillar,
    QuartzStairs,
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
    Barrier,
    Light,
    HayBlock,
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
    PackedIce,
    AcaciaStairs,
    DarkOakStairs,
    DirtPath,
    Sunflower,
    Lilac,
    RoseBush,
    Peony,
    TallGrass,
    LargeFern,
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
    WhiteStainedGlassPane,
    OrangeStainedGlassPane,
    MagentaStainedGlassPane,
    LightBlueStainedGlassPane,
    YellowStainedGlassPane,
    LimeStainedGlassPane,
    PinkStainedGlassPane,
    GrayStainedGlassPane,
    LightGrayStainedGlassPane,
    CyanStainedGlassPane,
    PurpleStainedGlassPane,
    BlueStainedGlassPane,
    BrownStainedGlassPane,
    GreenStainedGlassPane,
    RedStainedGlassPane,
    BlackStainedGlassPane,
    Prismarine,
    PrismarineBricks,
    DarkPrismarine,
    PrismarineStairs,
    PrismarineBrickStairs,
    DarkPrismarineStairs,
    SeaLantern,
    RedSandstone,
    ChiseledRedSandstone,
    CutRedSandstone,
    RedSandstoneStairs,
    RepeatingCommandBlock,
    ChainCommandBlock,
    MagmaBlock,
    NetherWartBlock,
    WarpedWartBlock,
    RedNetherBricks,
    BoneBlock,
    StructureVoid,
    ShulkerBox,
    WhiteShulkerBox,
    OrangeShulkerBox,
    MagentaShulkerBox,
    LightBlueShulkerBox,
    YellowShulkerBox,
    LimeShulkerBox,
    PinkShulkerBox,
    GrayShulkerBox,
    LightGrayShulkerBox,
    CyanShulkerBox,
    PurpleShulkerBox,
    BlueShulkerBox,
    BrownShulkerBox,
    GreenShulkerBox,
    RedShulkerBox,
    BlackShulkerBox,
    WhiteGlazedTerracotta,
    OrangeGlazedTerracotta,
    MagentaGlazedTerracotta,
    LightBlueGlazedTerracotta,
    YellowGlazedTerracotta,
    LimeGlazedTerracotta,
    PinkGlazedTerracotta,
    GrayGlazedTerracotta,
    LightGrayGlazedTerracotta,
    CyanGlazedTerracotta,
    PurpleGlazedTerracotta,
    BlueGlazedTerracotta,
    BrownGlazedTerracotta,
    GreenGlazedTerracotta,
    RedGlazedTerracotta,
    BlackGlazedTerracotta,
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
    TurtleEgg,
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
    TubeCoral,
    BrainCoral,
    BubbleCoral,
    FireCoral,
    HornCoral,
    DeadBrainCoral,
    DeadBubbleCoral,
    DeadFireCoral,
    DeadHornCoral,
    DeadTubeCoral,
    TubeCoralFan,
    BrainCoralFan,
    BubbleCoralFan,
    FireCoralFan,
    HornCoralFan,
    DeadTubeCoralFan,
    DeadBrainCoralFan,
    DeadBubbleCoralFan,
    DeadFireCoralFan,
    DeadHornCoralFan,
    BlueIce,
    Conduit,
    PolishedGraniteStairs,
    SmoothRedSandstoneStairs,
    MossyStoneBrickStairs,
    PolishedDioriteStairs,
    MossyCobblestoneStairs,
    EndStoneBrickStairs,
    StoneStairs,
    SmoothSandstoneStairs,
    SmoothQuartzStairs,
    GraniteStairs,
    AndesiteStairs,
    RedNetherBrickStairs,
    PolishedAndesiteStairs,
    DioriteStairs,
    CobbledDeepslateStairs,
    PolishedDeepslateStairs,
    DeepslateBrickStairs,
    DeepslateTileStairs,
    PolishedGraniteSlab,
    SmoothRedSandstoneSlab,
    MossyStoneBrickSlab,
    PolishedDioriteSlab,
    MossyCobblestoneSlab,
    EndStoneBrickSlab,
    SmoothSandstoneSlab,
    SmoothQuartzSlab,
    GraniteSlab,
    AndesiteSlab,
    RedNetherBrickSlab,
    PolishedAndesiteSlab,
    DioriteSlab,
    CobbledDeepslateSlab,
    PolishedDeepslateSlab,
    DeepslateBrickSlab,
    DeepslateTileSlab,
    Scaffolding,
    Redstone,
    RedstoneTorch,
    RedstoneBlock,
    Repeater,
    Comparator,
    Piston,
    StickyPiston,
    SlimeBlock,
    HoneyBlock,
    Observer,
    Hopper,
    Dispenser,
    Dropper,
    Lectern,
    Target,
    Lever,
    LightningRod,
    DaylightDetector,
    SculkSensor,
    TripwireHook,
    TrappedChest,
    Tnt,
    RedstoneLamp,
    NoteBlock,
    StoneButton,
    PolishedBlackstoneButton,
    OakButton,
    SpruceButton,
    BirchButton,
    JungleButton,
    AcaciaButton,
    DarkOakButton,
    CrimsonButton,
    WarpedButton,
    StonePressurePlate,
    PolishedBlackstonePressurePlate,
    LightWeightedPressurePlate,
    HeavyWeightedPressurePlate,
    OakPressurePlate,
    SprucePressurePlate,
    BirchPressurePlate,
    JunglePressurePlate,
    AcaciaPressurePlate,
    DarkOakPressurePlate,
    CrimsonPressurePlate,
    WarpedPressurePlate,
    IronDoor,
    OakDoor,
    SpruceDoor,
    BirchDoor,
    JungleDoor,
    AcaciaDoor,
    DarkOakDoor,
    CrimsonDoor,
    WarpedDoor,
    IronTrapdoor,
    OakTrapdoor,
    SpruceTrapdoor,
    BirchTrapdoor,
    JungleTrapdoor,
    AcaciaTrapdoor,
    DarkOakTrapdoor,
    CrimsonTrapdoor,
    WarpedTrapdoor,
    OakFenceGate,
    SpruceFenceGate,
    BirchFenceGate,
    JungleFenceGate,
    AcaciaFenceGate,
    DarkOakFenceGate,
    CrimsonFenceGate,
    WarpedFenceGate,
    PoweredRail,
    DetectorRail,
    Rail,
    ActivatorRail,
    Saddle,
    Minecart,
    ChestMinecart,
    FurnaceMinecart,
    TntMinecart,
    HopperMinecart,
    CarrotOnAStick,
    WarpedFungusOnAStick,
    Elytra,
    OakBoat,
    SpruceBoat,
    BirchBoat,
    JungleBoat,
    AcaciaBoat,
    DarkOakBoat,
    StructureBlock,
    Jigsaw,
    TurtleHelmet,
    Scute,
    FlintAndSteel,
    Apple,
    Bow,
    Arrow,
    Coal,
    Charcoal,
    Diamond,
    Emerald,
    LapisLazuli,
    Quartz,
    AmethystShard,
    RawIron,
    IronIngot,
    RawCopper,
    CopperIngot,
    RawGold,
    GoldIngot,
    NetheriteIngot,
    NetheriteScrap,
    WoodenSword,
    WoodenShovel,
    WoodenPickaxe,
    WoodenAxe,
    WoodenHoe,
    StoneSword,
    StoneShovel,
    StonePickaxe,
    StoneAxe,
    StoneHoe,
    GoldenSword,
    GoldenShovel,
    GoldenPickaxe,
    GoldenAxe,
    GoldenHoe,
    IronSword,
    IronShovel,
    IronPickaxe,
    IronAxe,
    IronHoe,
    DiamondSword,
    DiamondShovel,
    DiamondPickaxe,
    DiamondAxe,
    DiamondHoe,
    NetheriteSword,
    NetheriteShovel,
    NetheritePickaxe,
    NetheriteAxe,
    NetheriteHoe,
    Stick,
    Bowl,
    MushroomStew,
    String,
    Feather,
    Gunpowder,
    WheatSeeds,
    Wheat,
    Bread,
    LeatherHelmet,
    LeatherChestplate,
    LeatherLeggings,
    LeatherBoots,
    ChainmailHelmet,
    ChainmailChestplate,
    ChainmailLeggings,
    ChainmailBoots,
    IronHelmet,
    IronChestplate,
    IronLeggings,
    IronBoots,
    DiamondHelmet,
    DiamondChestplate,
    DiamondLeggings,
    DiamondBoots,
    GoldenHelmet,
    GoldenChestplate,
    GoldenLeggings,
    GoldenBoots,
    NetheriteHelmet,
    NetheriteChestplate,
    NetheriteLeggings,
    NetheriteBoots,
    Flint,
    Porkchop,
    CookedPorkchop,
    Painting,
    GoldenApple,
    EnchantedGoldenApple,
    OakSign,
    SpruceSign,
    BirchSign,
    JungleSign,
    AcaciaSign,
    DarkOakSign,
    CrimsonSign,
    WarpedSign,
    Bucket,
    WaterBucket,
    LavaBucket,
    PowderSnowBucket,
    Snowball,
    Leather,
    MilkBucket,
    PufferfishBucket,
    SalmonBucket,
    CodBucket,
    TropicalFishBucket,
    AxolotlBucket,
    Brick,
    ClayBall,
    DriedKelpBlock,
    Paper,
    Book,
    SlimeBall,
    Egg,
    Compass,
    Bundle,
    FishingRod,
    Clock,
    Spyglass,
    GlowstoneDust,
    Cod,
    Salmon,
    TropicalFish,
    Pufferfish,
    CookedCod,
    CookedSalmon,
    InkSac,
    GlowInkSac,
    CocoaBeans,
    WhiteDye,
    OrangeDye,
    MagentaDye,
    LightBlueDye,
    YellowDye,
    LimeDye,
    PinkDye,
    GrayDye,
    LightGrayDye,
    CyanDye,
    PurpleDye,
    BlueDye,
    BrownDye,
    GreenDye,
    RedDye,
    BlackDye,
    BoneMeal,
    Bone,
    Sugar,
    Cake,
    WhiteBed,
    OrangeBed,
    MagentaBed,
    LightBlueBed,
    YellowBed,
    LimeBed,
    PinkBed,
    GrayBed,
    LightGrayBed,
    CyanBed,
    PurpleBed,
    BlueBed,
    BrownBed,
    GreenBed,
    RedBed,
    BlackBed,
    Cookie,
    FilledMap,
    Shears,
    MelonSlice,
    DriedKelp,
    PumpkinSeeds,
    MelonSeeds,
    Beef,
    CookedBeef,
    Chicken,
    CookedChicken,
    RottenFlesh,
    EnderPearl,
    BlazeRod,
    GhastTear,
    GoldNugget,
    NetherWart,
    Potion,
    GlassBottle,
    SpiderEye,
    FermentedSpiderEye,
    BlazePowder,
    MagmaCream,
    BrewingStand,
    Cauldron,
    EnderEye,
    GlisteringMelonSlice,
    AxolotlSpawnEgg,
    BatSpawnEgg,
    BeeSpawnEgg,
    BlazeSpawnEgg,
    CatSpawnEgg,
    CaveSpiderSpawnEgg,
    ChickenSpawnEgg,
    CodSpawnEgg,
    CowSpawnEgg,
    CreeperSpawnEgg,
    DolphinSpawnEgg,
    DonkeySpawnEgg,
    DrownedSpawnEgg,
    ElderGuardianSpawnEgg,
    EndermanSpawnEgg,
    EndermiteSpawnEgg,
    EvokerSpawnEgg,
    FoxSpawnEgg,
    GhastSpawnEgg,
    GlowSquidSpawnEgg,
    GoatSpawnEgg,
    GuardianSpawnEgg,
    HoglinSpawnEgg,
    HorseSpawnEgg,
    HuskSpawnEgg,
    LlamaSpawnEgg,
    MagmaCubeSpawnEgg,
    MooshroomSpawnEgg,
    MuleSpawnEgg,
    OcelotSpawnEgg,
    PandaSpawnEgg,
    ParrotSpawnEgg,
    PhantomSpawnEgg,
    PigSpawnEgg,
    PiglinSpawnEgg,
    PiglinBruteSpawnEgg,
    PillagerSpawnEgg,
    PolarBearSpawnEgg,
    PufferfishSpawnEgg,
    RabbitSpawnEgg,
    RavagerSpawnEgg,
    SalmonSpawnEgg,
    SheepSpawnEgg,
    ShulkerSpawnEgg,
    SilverfishSpawnEgg,
    SkeletonSpawnEgg,
    SkeletonHorseSpawnEgg,
    SlimeSpawnEgg,
    SpiderSpawnEgg,
    SquidSpawnEgg,
    StraySpawnEgg,
    StriderSpawnEgg,
    TraderLlamaSpawnEgg,
    TropicalFishSpawnEgg,
    TurtleSpawnEgg,
    VexSpawnEgg,
    VillagerSpawnEgg,
    VindicatorSpawnEgg,
    WanderingTraderSpawnEgg,
    WitchSpawnEgg,
    WitherSkeletonSpawnEgg,
    WolfSpawnEgg,
    ZoglinSpawnEgg,
    ZombieSpawnEgg,
    ZombieHorseSpawnEgg,
    ZombieVillagerSpawnEgg,
    ZombifiedPiglinSpawnEgg,
    ExperienceBottle,
    FireCharge,
    WritableBook,
    WrittenBook,
    ItemFrame,
    GlowItemFrame,
    FlowerPot,
    Carrot,
    Potato,
    BakedPotato,
    PoisonousPotato,
    Map,
    GoldenCarrot,
    SkeletonSkull,
    WitherSkeletonSkull,
    PlayerHead,
    ZombieHead,
    CreeperHead,
    DragonHead,
    NetherStar,
    PumpkinPie,
    FireworkRocket,
    FireworkStar,
    EnchantedBook,
    NetherBrick,
    PrismarineShard,
    PrismarineCrystals,
    Rabbit,
    CookedRabbit,
    RabbitStew,
    RabbitFoot,
    RabbitHide,
    ArmorStand,
    IronHorseArmor,
    GoldenHorseArmor,
    DiamondHorseArmor,
    LeatherHorseArmor,
    Lead,
    NameTag,
    CommandBlockMinecart,
    Mutton,
    CookedMutton,
    WhiteBanner,
    OrangeBanner,
    MagentaBanner,
    LightBlueBanner,
    YellowBanner,
    LimeBanner,
    PinkBanner,
    GrayBanner,
    LightGrayBanner,
    CyanBanner,
    PurpleBanner,
    BlueBanner,
    BrownBanner,
    GreenBanner,
    RedBanner,
    BlackBanner,
    EndCrystal,
    ChorusFruit,
    PoppedChorusFruit,
    Beetroot,
    BeetrootSeeds,
    BeetrootSoup,
    DragonBreath,
    SplashPotion,
    SpectralArrow,
    TippedArrow,
    LingeringPotion,
    Shield,
    TotemOfUndying,
    ShulkerShell,
    IronNugget,
    KnowledgeBook,
    DebugStick,
    MusicDisc13,
    MusicDiscCat,
    MusicDiscBlocks,
    MusicDiscChirp,
    MusicDiscFar,
    MusicDiscMall,
    MusicDiscMellohi,
    MusicDiscStal,
    MusicDiscStrad,
    MusicDiscWard,
    MusicDisc11,
    MusicDiscWait,
    MusicDiscOtherside,
    MusicDiscPigstep,
    Trident,
    PhantomMembrane,
    NautilusShell,
    HeartOfTheSea,
    Crossbow,
    SuspiciousStew,
    Loom,
    FlowerBannerPattern,
    CreeperBannerPattern,
    SkullBannerPattern,
    MojangBannerPattern,
    GlobeBannerPattern,
    PiglinBannerPattern,
    Composter,
    Barrel,
    Smoker,
    BlastFurnace,
    CartographyTable,
    FletchingTable,
    Grindstone,
    SmithingTable,
    Stonecutter,
    Bell,
    Lantern,
    SoulLantern,
    SweetBerries,
    GlowBerries,
    Campfire,
    SoulCampfire,
    Shroomlight,
    Honeycomb,
    BeeNest,
    Beehive,
    HoneyBottle,
    HoneycombBlock,
    Lodestone,
    CryingObsidian,
    Blackstone,
    BlackstoneSlab,
    BlackstoneStairs,
    GildedBlackstone,
    PolishedBlackstone,
    PolishedBlackstoneSlab,
    PolishedBlackstoneStairs,
    ChiseledPolishedBlackstone,
    PolishedBlackstoneBricks,
    PolishedBlackstoneBrickSlab,
    PolishedBlackstoneBrickStairs,
    CrackedPolishedBlackstoneBricks,
    RespawnAnchor,
    Candle,
    WhiteCandle,
    OrangeCandle,
    MagentaCandle,
    LightBlueCandle,
    YellowCandle,
    LimeCandle,
    PinkCandle,
    GrayCandle,
    LightGrayCandle,
    CyanCandle,
    PurpleCandle,
    BlueCandle,
    BrownCandle,
    GreenCandle,
    RedCandle,
    BlackCandle,
    SmallAmethystBud,
    MediumAmethystBud,
    LargeAmethystBud,
    AmethystCluster,
    PointedDripstone,
}
impl Item {
    #[inline]
    pub fn values() -> &'static [Item] {
        use Item::*;
        &[
            Stone,
            Granite,
            PolishedGranite,
            Diorite,
            PolishedDiorite,
            Andesite,
            PolishedAndesite,
            Deepslate,
            CobbledDeepslate,
            PolishedDeepslate,
            Calcite,
            Tuff,
            DripstoneBlock,
            GrassBlock,
            Dirt,
            CoarseDirt,
            Podzol,
            RootedDirt,
            CrimsonNylium,
            WarpedNylium,
            Cobblestone,
            OakPlanks,
            SprucePlanks,
            BirchPlanks,
            JunglePlanks,
            AcaciaPlanks,
            DarkOakPlanks,
            CrimsonPlanks,
            WarpedPlanks,
            OakSapling,
            SpruceSapling,
            BirchSapling,
            JungleSapling,
            AcaciaSapling,
            DarkOakSapling,
            Bedrock,
            Sand,
            RedSand,
            Gravel,
            CoalOre,
            DeepslateCoalOre,
            IronOre,
            DeepslateIronOre,
            CopperOre,
            DeepslateCopperOre,
            GoldOre,
            DeepslateGoldOre,
            RedstoneOre,
            DeepslateRedstoneOre,
            EmeraldOre,
            DeepslateEmeraldOre,
            LapisOre,
            DeepslateLapisOre,
            DiamondOre,
            DeepslateDiamondOre,
            NetherGoldOre,
            NetherQuartzOre,
            AncientDebris,
            CoalBlock,
            RawIronBlock,
            RawCopperBlock,
            RawGoldBlock,
            AmethystBlock,
            BuddingAmethyst,
            IronBlock,
            CopperBlock,
            GoldBlock,
            DiamondBlock,
            NetheriteBlock,
            ExposedCopper,
            WeatheredCopper,
            OxidizedCopper,
            CutCopper,
            ExposedCutCopper,
            WeatheredCutCopper,
            OxidizedCutCopper,
            CutCopperStairs,
            ExposedCutCopperStairs,
            WeatheredCutCopperStairs,
            OxidizedCutCopperStairs,
            CutCopperSlab,
            ExposedCutCopperSlab,
            WeatheredCutCopperSlab,
            OxidizedCutCopperSlab,
            WaxedCopperBlock,
            WaxedExposedCopper,
            WaxedWeatheredCopper,
            WaxedOxidizedCopper,
            WaxedCutCopper,
            WaxedExposedCutCopper,
            WaxedWeatheredCutCopper,
            WaxedOxidizedCutCopper,
            WaxedCutCopperStairs,
            WaxedExposedCutCopperStairs,
            WaxedWeatheredCutCopperStairs,
            WaxedOxidizedCutCopperStairs,
            WaxedCutCopperSlab,
            WaxedExposedCutCopperSlab,
            WaxedWeatheredCutCopperSlab,
            WaxedOxidizedCutCopperSlab,
            OakLog,
            SpruceLog,
            BirchLog,
            JungleLog,
            AcaciaLog,
            DarkOakLog,
            CrimsonStem,
            WarpedStem,
            StrippedOakLog,
            StrippedSpruceLog,
            StrippedBirchLog,
            StrippedJungleLog,
            StrippedAcaciaLog,
            StrippedDarkOakLog,
            StrippedCrimsonStem,
            StrippedWarpedStem,
            StrippedOakWood,
            StrippedSpruceWood,
            StrippedBirchWood,
            StrippedJungleWood,
            StrippedAcaciaWood,
            StrippedDarkOakWood,
            StrippedCrimsonHyphae,
            StrippedWarpedHyphae,
            OakWood,
            SpruceWood,
            BirchWood,
            JungleWood,
            AcaciaWood,
            DarkOakWood,
            CrimsonHyphae,
            WarpedHyphae,
            OakLeaves,
            SpruceLeaves,
            BirchLeaves,
            JungleLeaves,
            AcaciaLeaves,
            DarkOakLeaves,
            AzaleaLeaves,
            FloweringAzaleaLeaves,
            Sponge,
            WetSponge,
            Glass,
            TintedGlass,
            LapisBlock,
            Sandstone,
            ChiseledSandstone,
            CutSandstone,
            Cobweb,
            Grass,
            Fern,
            Azalea,
            FloweringAzalea,
            DeadBush,
            Seagrass,
            SeaPickle,
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
            Cornflower,
            LilyOfTheValley,
            WitherRose,
            SporeBlossom,
            BrownMushroom,
            RedMushroom,
            CrimsonFungus,
            WarpedFungus,
            CrimsonRoots,
            WarpedRoots,
            NetherSprouts,
            WeepingVines,
            TwistingVines,
            SugarCane,
            Kelp,
            MossCarpet,
            MossBlock,
            HangingRoots,
            BigDripleaf,
            SmallDripleaf,
            Bamboo,
            OakSlab,
            SpruceSlab,
            BirchSlab,
            JungleSlab,
            AcaciaSlab,
            DarkOakSlab,
            CrimsonSlab,
            WarpedSlab,
            StoneSlab,
            SmoothStoneSlab,
            SandstoneSlab,
            CutSandstoneSlab,
            PetrifiedOakSlab,
            CobblestoneSlab,
            BrickSlab,
            StoneBrickSlab,
            NetherBrickSlab,
            QuartzSlab,
            RedSandstoneSlab,
            CutRedSandstoneSlab,
            PurpurSlab,
            PrismarineSlab,
            PrismarineBrickSlab,
            DarkPrismarineSlab,
            SmoothQuartz,
            SmoothRedSandstone,
            SmoothSandstone,
            SmoothStone,
            Bricks,
            Bookshelf,
            MossyCobblestone,
            Obsidian,
            Torch,
            EndRod,
            ChorusPlant,
            ChorusFlower,
            PurpurBlock,
            PurpurPillar,
            PurpurStairs,
            Spawner,
            OakStairs,
            Chest,
            CraftingTable,
            Farmland,
            Furnace,
            Ladder,
            CobblestoneStairs,
            Snow,
            Ice,
            SnowBlock,
            Cactus,
            Clay,
            Jukebox,
            OakFence,
            SpruceFence,
            BirchFence,
            JungleFence,
            AcaciaFence,
            DarkOakFence,
            CrimsonFence,
            WarpedFence,
            Pumpkin,
            CarvedPumpkin,
            JackOLantern,
            Netherrack,
            SoulSand,
            SoulSoil,
            Basalt,
            PolishedBasalt,
            SmoothBasalt,
            SoulTorch,
            Glowstone,
            InfestedStone,
            InfestedCobblestone,
            InfestedStoneBricks,
            InfestedMossyStoneBricks,
            InfestedCrackedStoneBricks,
            InfestedChiseledStoneBricks,
            InfestedDeepslate,
            StoneBricks,
            MossyStoneBricks,
            CrackedStoneBricks,
            ChiseledStoneBricks,
            DeepslateBricks,
            CrackedDeepslateBricks,
            DeepslateTiles,
            CrackedDeepslateTiles,
            ChiseledDeepslate,
            BrownMushroomBlock,
            RedMushroomBlock,
            MushroomStem,
            IronBars,
            Chain,
            GlassPane,
            Melon,
            Vine,
            GlowLichen,
            BrickStairs,
            StoneBrickStairs,
            Mycelium,
            LilyPad,
            NetherBricks,
            CrackedNetherBricks,
            ChiseledNetherBricks,
            NetherBrickFence,
            NetherBrickStairs,
            EnchantingTable,
            EndPortalFrame,
            EndStone,
            EndStoneBricks,
            DragonEgg,
            SandstoneStairs,
            EnderChest,
            EmeraldBlock,
            SpruceStairs,
            BirchStairs,
            JungleStairs,
            CrimsonStairs,
            WarpedStairs,
            CommandBlock,
            Beacon,
            CobblestoneWall,
            MossyCobblestoneWall,
            BrickWall,
            PrismarineWall,
            RedSandstoneWall,
            MossyStoneBrickWall,
            GraniteWall,
            StoneBrickWall,
            NetherBrickWall,
            AndesiteWall,
            RedNetherBrickWall,
            SandstoneWall,
            EndStoneBrickWall,
            DioriteWall,
            BlackstoneWall,
            PolishedBlackstoneWall,
            PolishedBlackstoneBrickWall,
            CobbledDeepslateWall,
            PolishedDeepslateWall,
            DeepslateBrickWall,
            DeepslateTileWall,
            Anvil,
            ChippedAnvil,
            DamagedAnvil,
            ChiseledQuartzBlock,
            QuartzBlock,
            QuartzBricks,
            QuartzPillar,
            QuartzStairs,
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
            Barrier,
            Light,
            HayBlock,
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
            PackedIce,
            AcaciaStairs,
            DarkOakStairs,
            DirtPath,
            Sunflower,
            Lilac,
            RoseBush,
            Peony,
            TallGrass,
            LargeFern,
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
            WhiteStainedGlassPane,
            OrangeStainedGlassPane,
            MagentaStainedGlassPane,
            LightBlueStainedGlassPane,
            YellowStainedGlassPane,
            LimeStainedGlassPane,
            PinkStainedGlassPane,
            GrayStainedGlassPane,
            LightGrayStainedGlassPane,
            CyanStainedGlassPane,
            PurpleStainedGlassPane,
            BlueStainedGlassPane,
            BrownStainedGlassPane,
            GreenStainedGlassPane,
            RedStainedGlassPane,
            BlackStainedGlassPane,
            Prismarine,
            PrismarineBricks,
            DarkPrismarine,
            PrismarineStairs,
            PrismarineBrickStairs,
            DarkPrismarineStairs,
            SeaLantern,
            RedSandstone,
            ChiseledRedSandstone,
            CutRedSandstone,
            RedSandstoneStairs,
            RepeatingCommandBlock,
            ChainCommandBlock,
            MagmaBlock,
            NetherWartBlock,
            WarpedWartBlock,
            RedNetherBricks,
            BoneBlock,
            StructureVoid,
            ShulkerBox,
            WhiteShulkerBox,
            OrangeShulkerBox,
            MagentaShulkerBox,
            LightBlueShulkerBox,
            YellowShulkerBox,
            LimeShulkerBox,
            PinkShulkerBox,
            GrayShulkerBox,
            LightGrayShulkerBox,
            CyanShulkerBox,
            PurpleShulkerBox,
            BlueShulkerBox,
            BrownShulkerBox,
            GreenShulkerBox,
            RedShulkerBox,
            BlackShulkerBox,
            WhiteGlazedTerracotta,
            OrangeGlazedTerracotta,
            MagentaGlazedTerracotta,
            LightBlueGlazedTerracotta,
            YellowGlazedTerracotta,
            LimeGlazedTerracotta,
            PinkGlazedTerracotta,
            GrayGlazedTerracotta,
            LightGrayGlazedTerracotta,
            CyanGlazedTerracotta,
            PurpleGlazedTerracotta,
            BlueGlazedTerracotta,
            BrownGlazedTerracotta,
            GreenGlazedTerracotta,
            RedGlazedTerracotta,
            BlackGlazedTerracotta,
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
            TurtleEgg,
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
            TubeCoral,
            BrainCoral,
            BubbleCoral,
            FireCoral,
            HornCoral,
            DeadBrainCoral,
            DeadBubbleCoral,
            DeadFireCoral,
            DeadHornCoral,
            DeadTubeCoral,
            TubeCoralFan,
            BrainCoralFan,
            BubbleCoralFan,
            FireCoralFan,
            HornCoralFan,
            DeadTubeCoralFan,
            DeadBrainCoralFan,
            DeadBubbleCoralFan,
            DeadFireCoralFan,
            DeadHornCoralFan,
            BlueIce,
            Conduit,
            PolishedGraniteStairs,
            SmoothRedSandstoneStairs,
            MossyStoneBrickStairs,
            PolishedDioriteStairs,
            MossyCobblestoneStairs,
            EndStoneBrickStairs,
            StoneStairs,
            SmoothSandstoneStairs,
            SmoothQuartzStairs,
            GraniteStairs,
            AndesiteStairs,
            RedNetherBrickStairs,
            PolishedAndesiteStairs,
            DioriteStairs,
            CobbledDeepslateStairs,
            PolishedDeepslateStairs,
            DeepslateBrickStairs,
            DeepslateTileStairs,
            PolishedGraniteSlab,
            SmoothRedSandstoneSlab,
            MossyStoneBrickSlab,
            PolishedDioriteSlab,
            MossyCobblestoneSlab,
            EndStoneBrickSlab,
            SmoothSandstoneSlab,
            SmoothQuartzSlab,
            GraniteSlab,
            AndesiteSlab,
            RedNetherBrickSlab,
            PolishedAndesiteSlab,
            DioriteSlab,
            CobbledDeepslateSlab,
            PolishedDeepslateSlab,
            DeepslateBrickSlab,
            DeepslateTileSlab,
            Scaffolding,
            Redstone,
            RedstoneTorch,
            RedstoneBlock,
            Repeater,
            Comparator,
            Piston,
            StickyPiston,
            SlimeBlock,
            HoneyBlock,
            Observer,
            Hopper,
            Dispenser,
            Dropper,
            Lectern,
            Target,
            Lever,
            LightningRod,
            DaylightDetector,
            SculkSensor,
            TripwireHook,
            TrappedChest,
            Tnt,
            RedstoneLamp,
            NoteBlock,
            StoneButton,
            PolishedBlackstoneButton,
            OakButton,
            SpruceButton,
            BirchButton,
            JungleButton,
            AcaciaButton,
            DarkOakButton,
            CrimsonButton,
            WarpedButton,
            StonePressurePlate,
            PolishedBlackstonePressurePlate,
            LightWeightedPressurePlate,
            HeavyWeightedPressurePlate,
            OakPressurePlate,
            SprucePressurePlate,
            BirchPressurePlate,
            JunglePressurePlate,
            AcaciaPressurePlate,
            DarkOakPressurePlate,
            CrimsonPressurePlate,
            WarpedPressurePlate,
            IronDoor,
            OakDoor,
            SpruceDoor,
            BirchDoor,
            JungleDoor,
            AcaciaDoor,
            DarkOakDoor,
            CrimsonDoor,
            WarpedDoor,
            IronTrapdoor,
            OakTrapdoor,
            SpruceTrapdoor,
            BirchTrapdoor,
            JungleTrapdoor,
            AcaciaTrapdoor,
            DarkOakTrapdoor,
            CrimsonTrapdoor,
            WarpedTrapdoor,
            OakFenceGate,
            SpruceFenceGate,
            BirchFenceGate,
            JungleFenceGate,
            AcaciaFenceGate,
            DarkOakFenceGate,
            CrimsonFenceGate,
            WarpedFenceGate,
            PoweredRail,
            DetectorRail,
            Rail,
            ActivatorRail,
            Saddle,
            Minecart,
            ChestMinecart,
            FurnaceMinecart,
            TntMinecart,
            HopperMinecart,
            CarrotOnAStick,
            WarpedFungusOnAStick,
            Elytra,
            OakBoat,
            SpruceBoat,
            BirchBoat,
            JungleBoat,
            AcaciaBoat,
            DarkOakBoat,
            StructureBlock,
            Jigsaw,
            TurtleHelmet,
            Scute,
            FlintAndSteel,
            Apple,
            Bow,
            Arrow,
            Coal,
            Charcoal,
            Diamond,
            Emerald,
            LapisLazuli,
            Quartz,
            AmethystShard,
            RawIron,
            IronIngot,
            RawCopper,
            CopperIngot,
            RawGold,
            GoldIngot,
            NetheriteIngot,
            NetheriteScrap,
            WoodenSword,
            WoodenShovel,
            WoodenPickaxe,
            WoodenAxe,
            WoodenHoe,
            StoneSword,
            StoneShovel,
            StonePickaxe,
            StoneAxe,
            StoneHoe,
            GoldenSword,
            GoldenShovel,
            GoldenPickaxe,
            GoldenAxe,
            GoldenHoe,
            IronSword,
            IronShovel,
            IronPickaxe,
            IronAxe,
            IronHoe,
            DiamondSword,
            DiamondShovel,
            DiamondPickaxe,
            DiamondAxe,
            DiamondHoe,
            NetheriteSword,
            NetheriteShovel,
            NetheritePickaxe,
            NetheriteAxe,
            NetheriteHoe,
            Stick,
            Bowl,
            MushroomStew,
            String,
            Feather,
            Gunpowder,
            WheatSeeds,
            Wheat,
            Bread,
            LeatherHelmet,
            LeatherChestplate,
            LeatherLeggings,
            LeatherBoots,
            ChainmailHelmet,
            ChainmailChestplate,
            ChainmailLeggings,
            ChainmailBoots,
            IronHelmet,
            IronChestplate,
            IronLeggings,
            IronBoots,
            DiamondHelmet,
            DiamondChestplate,
            DiamondLeggings,
            DiamondBoots,
            GoldenHelmet,
            GoldenChestplate,
            GoldenLeggings,
            GoldenBoots,
            NetheriteHelmet,
            NetheriteChestplate,
            NetheriteLeggings,
            NetheriteBoots,
            Flint,
            Porkchop,
            CookedPorkchop,
            Painting,
            GoldenApple,
            EnchantedGoldenApple,
            OakSign,
            SpruceSign,
            BirchSign,
            JungleSign,
            AcaciaSign,
            DarkOakSign,
            CrimsonSign,
            WarpedSign,
            Bucket,
            WaterBucket,
            LavaBucket,
            PowderSnowBucket,
            Snowball,
            Leather,
            MilkBucket,
            PufferfishBucket,
            SalmonBucket,
            CodBucket,
            TropicalFishBucket,
            AxolotlBucket,
            Brick,
            ClayBall,
            DriedKelpBlock,
            Paper,
            Book,
            SlimeBall,
            Egg,
            Compass,
            Bundle,
            FishingRod,
            Clock,
            Spyglass,
            GlowstoneDust,
            Cod,
            Salmon,
            TropicalFish,
            Pufferfish,
            CookedCod,
            CookedSalmon,
            InkSac,
            GlowInkSac,
            CocoaBeans,
            WhiteDye,
            OrangeDye,
            MagentaDye,
            LightBlueDye,
            YellowDye,
            LimeDye,
            PinkDye,
            GrayDye,
            LightGrayDye,
            CyanDye,
            PurpleDye,
            BlueDye,
            BrownDye,
            GreenDye,
            RedDye,
            BlackDye,
            BoneMeal,
            Bone,
            Sugar,
            Cake,
            WhiteBed,
            OrangeBed,
            MagentaBed,
            LightBlueBed,
            YellowBed,
            LimeBed,
            PinkBed,
            GrayBed,
            LightGrayBed,
            CyanBed,
            PurpleBed,
            BlueBed,
            BrownBed,
            GreenBed,
            RedBed,
            BlackBed,
            Cookie,
            FilledMap,
            Shears,
            MelonSlice,
            DriedKelp,
            PumpkinSeeds,
            MelonSeeds,
            Beef,
            CookedBeef,
            Chicken,
            CookedChicken,
            RottenFlesh,
            EnderPearl,
            BlazeRod,
            GhastTear,
            GoldNugget,
            NetherWart,
            Potion,
            GlassBottle,
            SpiderEye,
            FermentedSpiderEye,
            BlazePowder,
            MagmaCream,
            BrewingStand,
            Cauldron,
            EnderEye,
            GlisteringMelonSlice,
            AxolotlSpawnEgg,
            BatSpawnEgg,
            BeeSpawnEgg,
            BlazeSpawnEgg,
            CatSpawnEgg,
            CaveSpiderSpawnEgg,
            ChickenSpawnEgg,
            CodSpawnEgg,
            CowSpawnEgg,
            CreeperSpawnEgg,
            DolphinSpawnEgg,
            DonkeySpawnEgg,
            DrownedSpawnEgg,
            ElderGuardianSpawnEgg,
            EndermanSpawnEgg,
            EndermiteSpawnEgg,
            EvokerSpawnEgg,
            FoxSpawnEgg,
            GhastSpawnEgg,
            GlowSquidSpawnEgg,
            GoatSpawnEgg,
            GuardianSpawnEgg,
            HoglinSpawnEgg,
            HorseSpawnEgg,
            HuskSpawnEgg,
            LlamaSpawnEgg,
            MagmaCubeSpawnEgg,
            MooshroomSpawnEgg,
            MuleSpawnEgg,
            OcelotSpawnEgg,
            PandaSpawnEgg,
            ParrotSpawnEgg,
            PhantomSpawnEgg,
            PigSpawnEgg,
            PiglinSpawnEgg,
            PiglinBruteSpawnEgg,
            PillagerSpawnEgg,
            PolarBearSpawnEgg,
            PufferfishSpawnEgg,
            RabbitSpawnEgg,
            RavagerSpawnEgg,
            SalmonSpawnEgg,
            SheepSpawnEgg,
            ShulkerSpawnEgg,
            SilverfishSpawnEgg,
            SkeletonSpawnEgg,
            SkeletonHorseSpawnEgg,
            SlimeSpawnEgg,
            SpiderSpawnEgg,
            SquidSpawnEgg,
            StraySpawnEgg,
            StriderSpawnEgg,
            TraderLlamaSpawnEgg,
            TropicalFishSpawnEgg,
            TurtleSpawnEgg,
            VexSpawnEgg,
            VillagerSpawnEgg,
            VindicatorSpawnEgg,
            WanderingTraderSpawnEgg,
            WitchSpawnEgg,
            WitherSkeletonSpawnEgg,
            WolfSpawnEgg,
            ZoglinSpawnEgg,
            ZombieSpawnEgg,
            ZombieHorseSpawnEgg,
            ZombieVillagerSpawnEgg,
            ZombifiedPiglinSpawnEgg,
            ExperienceBottle,
            FireCharge,
            WritableBook,
            WrittenBook,
            ItemFrame,
            GlowItemFrame,
            FlowerPot,
            Carrot,
            Potato,
            BakedPotato,
            PoisonousPotato,
            Map,
            GoldenCarrot,
            SkeletonSkull,
            WitherSkeletonSkull,
            PlayerHead,
            ZombieHead,
            CreeperHead,
            DragonHead,
            NetherStar,
            PumpkinPie,
            FireworkRocket,
            FireworkStar,
            EnchantedBook,
            NetherBrick,
            PrismarineShard,
            PrismarineCrystals,
            Rabbit,
            CookedRabbit,
            RabbitStew,
            RabbitFoot,
            RabbitHide,
            ArmorStand,
            IronHorseArmor,
            GoldenHorseArmor,
            DiamondHorseArmor,
            LeatherHorseArmor,
            Lead,
            NameTag,
            CommandBlockMinecart,
            Mutton,
            CookedMutton,
            WhiteBanner,
            OrangeBanner,
            MagentaBanner,
            LightBlueBanner,
            YellowBanner,
            LimeBanner,
            PinkBanner,
            GrayBanner,
            LightGrayBanner,
            CyanBanner,
            PurpleBanner,
            BlueBanner,
            BrownBanner,
            GreenBanner,
            RedBanner,
            BlackBanner,
            EndCrystal,
            ChorusFruit,
            PoppedChorusFruit,
            Beetroot,
            BeetrootSeeds,
            BeetrootSoup,
            DragonBreath,
            SplashPotion,
            SpectralArrow,
            TippedArrow,
            LingeringPotion,
            Shield,
            TotemOfUndying,
            ShulkerShell,
            IronNugget,
            KnowledgeBook,
            DebugStick,
            MusicDisc13,
            MusicDiscCat,
            MusicDiscBlocks,
            MusicDiscChirp,
            MusicDiscFar,
            MusicDiscMall,
            MusicDiscMellohi,
            MusicDiscStal,
            MusicDiscStrad,
            MusicDiscWard,
            MusicDisc11,
            MusicDiscWait,
            MusicDiscOtherside,
            MusicDiscPigstep,
            Trident,
            PhantomMembrane,
            NautilusShell,
            HeartOfTheSea,
            Crossbow,
            SuspiciousStew,
            Loom,
            FlowerBannerPattern,
            CreeperBannerPattern,
            SkullBannerPattern,
            MojangBannerPattern,
            GlobeBannerPattern,
            PiglinBannerPattern,
            Composter,
            Barrel,
            Smoker,
            BlastFurnace,
            CartographyTable,
            FletchingTable,
            Grindstone,
            SmithingTable,
            Stonecutter,
            Bell,
            Lantern,
            SoulLantern,
            SweetBerries,
            GlowBerries,
            Campfire,
            SoulCampfire,
            Shroomlight,
            Honeycomb,
            BeeNest,
            Beehive,
            HoneyBottle,
            HoneycombBlock,
            Lodestone,
            CryingObsidian,
            Blackstone,
            BlackstoneSlab,
            BlackstoneStairs,
            GildedBlackstone,
            PolishedBlackstone,
            PolishedBlackstoneSlab,
            PolishedBlackstoneStairs,
            ChiseledPolishedBlackstone,
            PolishedBlackstoneBricks,
            PolishedBlackstoneBrickSlab,
            PolishedBlackstoneBrickStairs,
            CrackedPolishedBlackstoneBricks,
            RespawnAnchor,
            Candle,
            WhiteCandle,
            OrangeCandle,
            MagentaCandle,
            LightBlueCandle,
            YellowCandle,
            LimeCandle,
            PinkCandle,
            GrayCandle,
            LightGrayCandle,
            CyanCandle,
            PurpleCandle,
            BlueCandle,
            BrownCandle,
            GreenCandle,
            RedCandle,
            BlackCandle,
            SmallAmethystBud,
            MediumAmethystBud,
            LargeAmethystBud,
            AmethystCluster,
            PointedDripstone,
        ]
    }
}
impl Item {
    #[doc = "Returns the `id` property of this `Item`."]
    #[inline]
    pub fn id(&self) -> u32 {
        match self {
            Item::Stone => 1u32,
            Item::Granite => 2u32,
            Item::PolishedGranite => 3u32,
            Item::Diorite => 4u32,
            Item::PolishedDiorite => 5u32,
            Item::Andesite => 6u32,
            Item::PolishedAndesite => 7u32,
            Item::Deepslate => 8u32,
            Item::CobbledDeepslate => 9u32,
            Item::PolishedDeepslate => 10u32,
            Item::Calcite => 11u32,
            Item::Tuff => 12u32,
            Item::DripstoneBlock => 13u32,
            Item::GrassBlock => 14u32,
            Item::Dirt => 15u32,
            Item::CoarseDirt => 16u32,
            Item::Podzol => 17u32,
            Item::RootedDirt => 18u32,
            Item::CrimsonNylium => 19u32,
            Item::WarpedNylium => 20u32,
            Item::Cobblestone => 21u32,
            Item::OakPlanks => 22u32,
            Item::SprucePlanks => 23u32,
            Item::BirchPlanks => 24u32,
            Item::JunglePlanks => 25u32,
            Item::AcaciaPlanks => 26u32,
            Item::DarkOakPlanks => 27u32,
            Item::CrimsonPlanks => 28u32,
            Item::WarpedPlanks => 29u32,
            Item::OakSapling => 30u32,
            Item::SpruceSapling => 31u32,
            Item::BirchSapling => 32u32,
            Item::JungleSapling => 33u32,
            Item::AcaciaSapling => 34u32,
            Item::DarkOakSapling => 35u32,
            Item::Bedrock => 36u32,
            Item::Sand => 37u32,
            Item::RedSand => 38u32,
            Item::Gravel => 39u32,
            Item::CoalOre => 40u32,
            Item::DeepslateCoalOre => 41u32,
            Item::IronOre => 42u32,
            Item::DeepslateIronOre => 43u32,
            Item::CopperOre => 44u32,
            Item::DeepslateCopperOre => 45u32,
            Item::GoldOre => 46u32,
            Item::DeepslateGoldOre => 47u32,
            Item::RedstoneOre => 48u32,
            Item::DeepslateRedstoneOre => 49u32,
            Item::EmeraldOre => 50u32,
            Item::DeepslateEmeraldOre => 51u32,
            Item::LapisOre => 52u32,
            Item::DeepslateLapisOre => 53u32,
            Item::DiamondOre => 54u32,
            Item::DeepslateDiamondOre => 55u32,
            Item::NetherGoldOre => 56u32,
            Item::NetherQuartzOre => 57u32,
            Item::AncientDebris => 58u32,
            Item::CoalBlock => 59u32,
            Item::RawIronBlock => 60u32,
            Item::RawCopperBlock => 61u32,
            Item::RawGoldBlock => 62u32,
            Item::AmethystBlock => 63u32,
            Item::BuddingAmethyst => 64u32,
            Item::IronBlock => 65u32,
            Item::CopperBlock => 66u32,
            Item::GoldBlock => 67u32,
            Item::DiamondBlock => 68u32,
            Item::NetheriteBlock => 69u32,
            Item::ExposedCopper => 70u32,
            Item::WeatheredCopper => 71u32,
            Item::OxidizedCopper => 72u32,
            Item::CutCopper => 73u32,
            Item::ExposedCutCopper => 74u32,
            Item::WeatheredCutCopper => 75u32,
            Item::OxidizedCutCopper => 76u32,
            Item::CutCopperStairs => 77u32,
            Item::ExposedCutCopperStairs => 78u32,
            Item::WeatheredCutCopperStairs => 79u32,
            Item::OxidizedCutCopperStairs => 80u32,
            Item::CutCopperSlab => 81u32,
            Item::ExposedCutCopperSlab => 82u32,
            Item::WeatheredCutCopperSlab => 83u32,
            Item::OxidizedCutCopperSlab => 84u32,
            Item::WaxedCopperBlock => 85u32,
            Item::WaxedExposedCopper => 86u32,
            Item::WaxedWeatheredCopper => 87u32,
            Item::WaxedOxidizedCopper => 88u32,
            Item::WaxedCutCopper => 89u32,
            Item::WaxedExposedCutCopper => 90u32,
            Item::WaxedWeatheredCutCopper => 91u32,
            Item::WaxedOxidizedCutCopper => 92u32,
            Item::WaxedCutCopperStairs => 93u32,
            Item::WaxedExposedCutCopperStairs => 94u32,
            Item::WaxedWeatheredCutCopperStairs => 95u32,
            Item::WaxedOxidizedCutCopperStairs => 96u32,
            Item::WaxedCutCopperSlab => 97u32,
            Item::WaxedExposedCutCopperSlab => 98u32,
            Item::WaxedWeatheredCutCopperSlab => 99u32,
            Item::WaxedOxidizedCutCopperSlab => 100u32,
            Item::OakLog => 101u32,
            Item::SpruceLog => 102u32,
            Item::BirchLog => 103u32,
            Item::JungleLog => 104u32,
            Item::AcaciaLog => 105u32,
            Item::DarkOakLog => 106u32,
            Item::CrimsonStem => 107u32,
            Item::WarpedStem => 108u32,
            Item::StrippedOakLog => 109u32,
            Item::StrippedSpruceLog => 110u32,
            Item::StrippedBirchLog => 111u32,
            Item::StrippedJungleLog => 112u32,
            Item::StrippedAcaciaLog => 113u32,
            Item::StrippedDarkOakLog => 114u32,
            Item::StrippedCrimsonStem => 115u32,
            Item::StrippedWarpedStem => 116u32,
            Item::StrippedOakWood => 117u32,
            Item::StrippedSpruceWood => 118u32,
            Item::StrippedBirchWood => 119u32,
            Item::StrippedJungleWood => 120u32,
            Item::StrippedAcaciaWood => 121u32,
            Item::StrippedDarkOakWood => 122u32,
            Item::StrippedCrimsonHyphae => 123u32,
            Item::StrippedWarpedHyphae => 124u32,
            Item::OakWood => 125u32,
            Item::SpruceWood => 126u32,
            Item::BirchWood => 127u32,
            Item::JungleWood => 128u32,
            Item::AcaciaWood => 129u32,
            Item::DarkOakWood => 130u32,
            Item::CrimsonHyphae => 131u32,
            Item::WarpedHyphae => 132u32,
            Item::OakLeaves => 133u32,
            Item::SpruceLeaves => 134u32,
            Item::BirchLeaves => 135u32,
            Item::JungleLeaves => 136u32,
            Item::AcaciaLeaves => 137u32,
            Item::DarkOakLeaves => 138u32,
            Item::AzaleaLeaves => 139u32,
            Item::FloweringAzaleaLeaves => 140u32,
            Item::Sponge => 141u32,
            Item::WetSponge => 142u32,
            Item::Glass => 143u32,
            Item::TintedGlass => 144u32,
            Item::LapisBlock => 145u32,
            Item::Sandstone => 146u32,
            Item::ChiseledSandstone => 147u32,
            Item::CutSandstone => 148u32,
            Item::Cobweb => 149u32,
            Item::Grass => 150u32,
            Item::Fern => 151u32,
            Item::Azalea => 152u32,
            Item::FloweringAzalea => 153u32,
            Item::DeadBush => 154u32,
            Item::Seagrass => 155u32,
            Item::SeaPickle => 156u32,
            Item::WhiteWool => 157u32,
            Item::OrangeWool => 158u32,
            Item::MagentaWool => 159u32,
            Item::LightBlueWool => 160u32,
            Item::YellowWool => 161u32,
            Item::LimeWool => 162u32,
            Item::PinkWool => 163u32,
            Item::GrayWool => 164u32,
            Item::LightGrayWool => 165u32,
            Item::CyanWool => 166u32,
            Item::PurpleWool => 167u32,
            Item::BlueWool => 168u32,
            Item::BrownWool => 169u32,
            Item::GreenWool => 170u32,
            Item::RedWool => 171u32,
            Item::BlackWool => 172u32,
            Item::Dandelion => 173u32,
            Item::Poppy => 174u32,
            Item::BlueOrchid => 175u32,
            Item::Allium => 176u32,
            Item::AzureBluet => 177u32,
            Item::RedTulip => 178u32,
            Item::OrangeTulip => 179u32,
            Item::WhiteTulip => 180u32,
            Item::PinkTulip => 181u32,
            Item::OxeyeDaisy => 182u32,
            Item::Cornflower => 183u32,
            Item::LilyOfTheValley => 184u32,
            Item::WitherRose => 185u32,
            Item::SporeBlossom => 186u32,
            Item::BrownMushroom => 187u32,
            Item::RedMushroom => 188u32,
            Item::CrimsonFungus => 189u32,
            Item::WarpedFungus => 190u32,
            Item::CrimsonRoots => 191u32,
            Item::WarpedRoots => 192u32,
            Item::NetherSprouts => 193u32,
            Item::WeepingVines => 194u32,
            Item::TwistingVines => 195u32,
            Item::SugarCane => 196u32,
            Item::Kelp => 197u32,
            Item::MossCarpet => 198u32,
            Item::MossBlock => 199u32,
            Item::HangingRoots => 200u32,
            Item::BigDripleaf => 201u32,
            Item::SmallDripleaf => 202u32,
            Item::Bamboo => 203u32,
            Item::OakSlab => 204u32,
            Item::SpruceSlab => 205u32,
            Item::BirchSlab => 206u32,
            Item::JungleSlab => 207u32,
            Item::AcaciaSlab => 208u32,
            Item::DarkOakSlab => 209u32,
            Item::CrimsonSlab => 210u32,
            Item::WarpedSlab => 211u32,
            Item::StoneSlab => 212u32,
            Item::SmoothStoneSlab => 213u32,
            Item::SandstoneSlab => 214u32,
            Item::CutSandstoneSlab => 215u32,
            Item::PetrifiedOakSlab => 216u32,
            Item::CobblestoneSlab => 217u32,
            Item::BrickSlab => 218u32,
            Item::StoneBrickSlab => 219u32,
            Item::NetherBrickSlab => 220u32,
            Item::QuartzSlab => 221u32,
            Item::RedSandstoneSlab => 222u32,
            Item::CutRedSandstoneSlab => 223u32,
            Item::PurpurSlab => 224u32,
            Item::PrismarineSlab => 225u32,
            Item::PrismarineBrickSlab => 226u32,
            Item::DarkPrismarineSlab => 227u32,
            Item::SmoothQuartz => 228u32,
            Item::SmoothRedSandstone => 229u32,
            Item::SmoothSandstone => 230u32,
            Item::SmoothStone => 231u32,
            Item::Bricks => 232u32,
            Item::Bookshelf => 233u32,
            Item::MossyCobblestone => 234u32,
            Item::Obsidian => 235u32,
            Item::Torch => 236u32,
            Item::EndRod => 237u32,
            Item::ChorusPlant => 238u32,
            Item::ChorusFlower => 239u32,
            Item::PurpurBlock => 240u32,
            Item::PurpurPillar => 241u32,
            Item::PurpurStairs => 242u32,
            Item::Spawner => 243u32,
            Item::OakStairs => 244u32,
            Item::Chest => 245u32,
            Item::CraftingTable => 246u32,
            Item::Farmland => 247u32,
            Item::Furnace => 248u32,
            Item::Ladder => 249u32,
            Item::CobblestoneStairs => 250u32,
            Item::Snow => 251u32,
            Item::Ice => 252u32,
            Item::SnowBlock => 253u32,
            Item::Cactus => 254u32,
            Item::Clay => 255u32,
            Item::Jukebox => 256u32,
            Item::OakFence => 257u32,
            Item::SpruceFence => 258u32,
            Item::BirchFence => 259u32,
            Item::JungleFence => 260u32,
            Item::AcaciaFence => 261u32,
            Item::DarkOakFence => 262u32,
            Item::CrimsonFence => 263u32,
            Item::WarpedFence => 264u32,
            Item::Pumpkin => 265u32,
            Item::CarvedPumpkin => 266u32,
            Item::JackOLantern => 267u32,
            Item::Netherrack => 268u32,
            Item::SoulSand => 269u32,
            Item::SoulSoil => 270u32,
            Item::Basalt => 271u32,
            Item::PolishedBasalt => 272u32,
            Item::SmoothBasalt => 273u32,
            Item::SoulTorch => 274u32,
            Item::Glowstone => 275u32,
            Item::InfestedStone => 276u32,
            Item::InfestedCobblestone => 277u32,
            Item::InfestedStoneBricks => 278u32,
            Item::InfestedMossyStoneBricks => 279u32,
            Item::InfestedCrackedStoneBricks => 280u32,
            Item::InfestedChiseledStoneBricks => 281u32,
            Item::InfestedDeepslate => 282u32,
            Item::StoneBricks => 283u32,
            Item::MossyStoneBricks => 284u32,
            Item::CrackedStoneBricks => 285u32,
            Item::ChiseledStoneBricks => 286u32,
            Item::DeepslateBricks => 287u32,
            Item::CrackedDeepslateBricks => 288u32,
            Item::DeepslateTiles => 289u32,
            Item::CrackedDeepslateTiles => 290u32,
            Item::ChiseledDeepslate => 291u32,
            Item::BrownMushroomBlock => 292u32,
            Item::RedMushroomBlock => 293u32,
            Item::MushroomStem => 294u32,
            Item::IronBars => 295u32,
            Item::Chain => 296u32,
            Item::GlassPane => 297u32,
            Item::Melon => 298u32,
            Item::Vine => 299u32,
            Item::GlowLichen => 300u32,
            Item::BrickStairs => 301u32,
            Item::StoneBrickStairs => 302u32,
            Item::Mycelium => 303u32,
            Item::LilyPad => 304u32,
            Item::NetherBricks => 305u32,
            Item::CrackedNetherBricks => 306u32,
            Item::ChiseledNetherBricks => 307u32,
            Item::NetherBrickFence => 308u32,
            Item::NetherBrickStairs => 309u32,
            Item::EnchantingTable => 310u32,
            Item::EndPortalFrame => 311u32,
            Item::EndStone => 312u32,
            Item::EndStoneBricks => 313u32,
            Item::DragonEgg => 314u32,
            Item::SandstoneStairs => 315u32,
            Item::EnderChest => 316u32,
            Item::EmeraldBlock => 317u32,
            Item::SpruceStairs => 318u32,
            Item::BirchStairs => 319u32,
            Item::JungleStairs => 320u32,
            Item::CrimsonStairs => 321u32,
            Item::WarpedStairs => 322u32,
            Item::CommandBlock => 323u32,
            Item::Beacon => 324u32,
            Item::CobblestoneWall => 325u32,
            Item::MossyCobblestoneWall => 326u32,
            Item::BrickWall => 327u32,
            Item::PrismarineWall => 328u32,
            Item::RedSandstoneWall => 329u32,
            Item::MossyStoneBrickWall => 330u32,
            Item::GraniteWall => 331u32,
            Item::StoneBrickWall => 332u32,
            Item::NetherBrickWall => 333u32,
            Item::AndesiteWall => 334u32,
            Item::RedNetherBrickWall => 335u32,
            Item::SandstoneWall => 336u32,
            Item::EndStoneBrickWall => 337u32,
            Item::DioriteWall => 338u32,
            Item::BlackstoneWall => 339u32,
            Item::PolishedBlackstoneWall => 340u32,
            Item::PolishedBlackstoneBrickWall => 341u32,
            Item::CobbledDeepslateWall => 342u32,
            Item::PolishedDeepslateWall => 343u32,
            Item::DeepslateBrickWall => 344u32,
            Item::DeepslateTileWall => 345u32,
            Item::Anvil => 346u32,
            Item::ChippedAnvil => 347u32,
            Item::DamagedAnvil => 348u32,
            Item::ChiseledQuartzBlock => 349u32,
            Item::QuartzBlock => 350u32,
            Item::QuartzBricks => 351u32,
            Item::QuartzPillar => 352u32,
            Item::QuartzStairs => 353u32,
            Item::WhiteTerracotta => 354u32,
            Item::OrangeTerracotta => 355u32,
            Item::MagentaTerracotta => 356u32,
            Item::LightBlueTerracotta => 357u32,
            Item::YellowTerracotta => 358u32,
            Item::LimeTerracotta => 359u32,
            Item::PinkTerracotta => 360u32,
            Item::GrayTerracotta => 361u32,
            Item::LightGrayTerracotta => 362u32,
            Item::CyanTerracotta => 363u32,
            Item::PurpleTerracotta => 364u32,
            Item::BlueTerracotta => 365u32,
            Item::BrownTerracotta => 366u32,
            Item::GreenTerracotta => 367u32,
            Item::RedTerracotta => 368u32,
            Item::BlackTerracotta => 369u32,
            Item::Barrier => 370u32,
            Item::Light => 371u32,
            Item::HayBlock => 372u32,
            Item::WhiteCarpet => 373u32,
            Item::OrangeCarpet => 374u32,
            Item::MagentaCarpet => 375u32,
            Item::LightBlueCarpet => 376u32,
            Item::YellowCarpet => 377u32,
            Item::LimeCarpet => 378u32,
            Item::PinkCarpet => 379u32,
            Item::GrayCarpet => 380u32,
            Item::LightGrayCarpet => 381u32,
            Item::CyanCarpet => 382u32,
            Item::PurpleCarpet => 383u32,
            Item::BlueCarpet => 384u32,
            Item::BrownCarpet => 385u32,
            Item::GreenCarpet => 386u32,
            Item::RedCarpet => 387u32,
            Item::BlackCarpet => 388u32,
            Item::Terracotta => 389u32,
            Item::PackedIce => 390u32,
            Item::AcaciaStairs => 391u32,
            Item::DarkOakStairs => 392u32,
            Item::DirtPath => 393u32,
            Item::Sunflower => 394u32,
            Item::Lilac => 395u32,
            Item::RoseBush => 396u32,
            Item::Peony => 397u32,
            Item::TallGrass => 398u32,
            Item::LargeFern => 399u32,
            Item::WhiteStainedGlass => 400u32,
            Item::OrangeStainedGlass => 401u32,
            Item::MagentaStainedGlass => 402u32,
            Item::LightBlueStainedGlass => 403u32,
            Item::YellowStainedGlass => 404u32,
            Item::LimeStainedGlass => 405u32,
            Item::PinkStainedGlass => 406u32,
            Item::GrayStainedGlass => 407u32,
            Item::LightGrayStainedGlass => 408u32,
            Item::CyanStainedGlass => 409u32,
            Item::PurpleStainedGlass => 410u32,
            Item::BlueStainedGlass => 411u32,
            Item::BrownStainedGlass => 412u32,
            Item::GreenStainedGlass => 413u32,
            Item::RedStainedGlass => 414u32,
            Item::BlackStainedGlass => 415u32,
            Item::WhiteStainedGlassPane => 416u32,
            Item::OrangeStainedGlassPane => 417u32,
            Item::MagentaStainedGlassPane => 418u32,
            Item::LightBlueStainedGlassPane => 419u32,
            Item::YellowStainedGlassPane => 420u32,
            Item::LimeStainedGlassPane => 421u32,
            Item::PinkStainedGlassPane => 422u32,
            Item::GrayStainedGlassPane => 423u32,
            Item::LightGrayStainedGlassPane => 424u32,
            Item::CyanStainedGlassPane => 425u32,
            Item::PurpleStainedGlassPane => 426u32,
            Item::BlueStainedGlassPane => 427u32,
            Item::BrownStainedGlassPane => 428u32,
            Item::GreenStainedGlassPane => 429u32,
            Item::RedStainedGlassPane => 430u32,
            Item::BlackStainedGlassPane => 431u32,
            Item::Prismarine => 432u32,
            Item::PrismarineBricks => 433u32,
            Item::DarkPrismarine => 434u32,
            Item::PrismarineStairs => 435u32,
            Item::PrismarineBrickStairs => 436u32,
            Item::DarkPrismarineStairs => 437u32,
            Item::SeaLantern => 438u32,
            Item::RedSandstone => 439u32,
            Item::ChiseledRedSandstone => 440u32,
            Item::CutRedSandstone => 441u32,
            Item::RedSandstoneStairs => 442u32,
            Item::RepeatingCommandBlock => 443u32,
            Item::ChainCommandBlock => 444u32,
            Item::MagmaBlock => 445u32,
            Item::NetherWartBlock => 446u32,
            Item::WarpedWartBlock => 447u32,
            Item::RedNetherBricks => 448u32,
            Item::BoneBlock => 449u32,
            Item::StructureVoid => 450u32,
            Item::ShulkerBox => 451u32,
            Item::WhiteShulkerBox => 452u32,
            Item::OrangeShulkerBox => 453u32,
            Item::MagentaShulkerBox => 454u32,
            Item::LightBlueShulkerBox => 455u32,
            Item::YellowShulkerBox => 456u32,
            Item::LimeShulkerBox => 457u32,
            Item::PinkShulkerBox => 458u32,
            Item::GrayShulkerBox => 459u32,
            Item::LightGrayShulkerBox => 460u32,
            Item::CyanShulkerBox => 461u32,
            Item::PurpleShulkerBox => 462u32,
            Item::BlueShulkerBox => 463u32,
            Item::BrownShulkerBox => 464u32,
            Item::GreenShulkerBox => 465u32,
            Item::RedShulkerBox => 466u32,
            Item::BlackShulkerBox => 467u32,
            Item::WhiteGlazedTerracotta => 468u32,
            Item::OrangeGlazedTerracotta => 469u32,
            Item::MagentaGlazedTerracotta => 470u32,
            Item::LightBlueGlazedTerracotta => 471u32,
            Item::YellowGlazedTerracotta => 472u32,
            Item::LimeGlazedTerracotta => 473u32,
            Item::PinkGlazedTerracotta => 474u32,
            Item::GrayGlazedTerracotta => 475u32,
            Item::LightGrayGlazedTerracotta => 476u32,
            Item::CyanGlazedTerracotta => 477u32,
            Item::PurpleGlazedTerracotta => 478u32,
            Item::BlueGlazedTerracotta => 479u32,
            Item::BrownGlazedTerracotta => 480u32,
            Item::GreenGlazedTerracotta => 481u32,
            Item::RedGlazedTerracotta => 482u32,
            Item::BlackGlazedTerracotta => 483u32,
            Item::WhiteConcrete => 484u32,
            Item::OrangeConcrete => 485u32,
            Item::MagentaConcrete => 486u32,
            Item::LightBlueConcrete => 487u32,
            Item::YellowConcrete => 488u32,
            Item::LimeConcrete => 489u32,
            Item::PinkConcrete => 490u32,
            Item::GrayConcrete => 491u32,
            Item::LightGrayConcrete => 492u32,
            Item::CyanConcrete => 493u32,
            Item::PurpleConcrete => 494u32,
            Item::BlueConcrete => 495u32,
            Item::BrownConcrete => 496u32,
            Item::GreenConcrete => 497u32,
            Item::RedConcrete => 498u32,
            Item::BlackConcrete => 499u32,
            Item::WhiteConcretePowder => 500u32,
            Item::OrangeConcretePowder => 501u32,
            Item::MagentaConcretePowder => 502u32,
            Item::LightBlueConcretePowder => 503u32,
            Item::YellowConcretePowder => 504u32,
            Item::LimeConcretePowder => 505u32,
            Item::PinkConcretePowder => 506u32,
            Item::GrayConcretePowder => 507u32,
            Item::LightGrayConcretePowder => 508u32,
            Item::CyanConcretePowder => 509u32,
            Item::PurpleConcretePowder => 510u32,
            Item::BlueConcretePowder => 511u32,
            Item::BrownConcretePowder => 512u32,
            Item::GreenConcretePowder => 513u32,
            Item::RedConcretePowder => 514u32,
            Item::BlackConcretePowder => 515u32,
            Item::TurtleEgg => 516u32,
            Item::DeadTubeCoralBlock => 517u32,
            Item::DeadBrainCoralBlock => 518u32,
            Item::DeadBubbleCoralBlock => 519u32,
            Item::DeadFireCoralBlock => 520u32,
            Item::DeadHornCoralBlock => 521u32,
            Item::TubeCoralBlock => 522u32,
            Item::BrainCoralBlock => 523u32,
            Item::BubbleCoralBlock => 524u32,
            Item::FireCoralBlock => 525u32,
            Item::HornCoralBlock => 526u32,
            Item::TubeCoral => 527u32,
            Item::BrainCoral => 528u32,
            Item::BubbleCoral => 529u32,
            Item::FireCoral => 530u32,
            Item::HornCoral => 531u32,
            Item::DeadBrainCoral => 532u32,
            Item::DeadBubbleCoral => 533u32,
            Item::DeadFireCoral => 534u32,
            Item::DeadHornCoral => 535u32,
            Item::DeadTubeCoral => 536u32,
            Item::TubeCoralFan => 537u32,
            Item::BrainCoralFan => 538u32,
            Item::BubbleCoralFan => 539u32,
            Item::FireCoralFan => 540u32,
            Item::HornCoralFan => 541u32,
            Item::DeadTubeCoralFan => 542u32,
            Item::DeadBrainCoralFan => 543u32,
            Item::DeadBubbleCoralFan => 544u32,
            Item::DeadFireCoralFan => 545u32,
            Item::DeadHornCoralFan => 546u32,
            Item::BlueIce => 547u32,
            Item::Conduit => 548u32,
            Item::PolishedGraniteStairs => 549u32,
            Item::SmoothRedSandstoneStairs => 550u32,
            Item::MossyStoneBrickStairs => 551u32,
            Item::PolishedDioriteStairs => 552u32,
            Item::MossyCobblestoneStairs => 553u32,
            Item::EndStoneBrickStairs => 554u32,
            Item::StoneStairs => 555u32,
            Item::SmoothSandstoneStairs => 556u32,
            Item::SmoothQuartzStairs => 557u32,
            Item::GraniteStairs => 558u32,
            Item::AndesiteStairs => 559u32,
            Item::RedNetherBrickStairs => 560u32,
            Item::PolishedAndesiteStairs => 561u32,
            Item::DioriteStairs => 562u32,
            Item::CobbledDeepslateStairs => 563u32,
            Item::PolishedDeepslateStairs => 564u32,
            Item::DeepslateBrickStairs => 565u32,
            Item::DeepslateTileStairs => 566u32,
            Item::PolishedGraniteSlab => 567u32,
            Item::SmoothRedSandstoneSlab => 568u32,
            Item::MossyStoneBrickSlab => 569u32,
            Item::PolishedDioriteSlab => 570u32,
            Item::MossyCobblestoneSlab => 571u32,
            Item::EndStoneBrickSlab => 572u32,
            Item::SmoothSandstoneSlab => 573u32,
            Item::SmoothQuartzSlab => 574u32,
            Item::GraniteSlab => 575u32,
            Item::AndesiteSlab => 576u32,
            Item::RedNetherBrickSlab => 577u32,
            Item::PolishedAndesiteSlab => 578u32,
            Item::DioriteSlab => 579u32,
            Item::CobbledDeepslateSlab => 580u32,
            Item::PolishedDeepslateSlab => 581u32,
            Item::DeepslateBrickSlab => 582u32,
            Item::DeepslateTileSlab => 583u32,
            Item::Scaffolding => 584u32,
            Item::Redstone => 585u32,
            Item::RedstoneTorch => 586u32,
            Item::RedstoneBlock => 587u32,
            Item::Repeater => 588u32,
            Item::Comparator => 589u32,
            Item::Piston => 590u32,
            Item::StickyPiston => 591u32,
            Item::SlimeBlock => 592u32,
            Item::HoneyBlock => 593u32,
            Item::Observer => 594u32,
            Item::Hopper => 595u32,
            Item::Dispenser => 596u32,
            Item::Dropper => 597u32,
            Item::Lectern => 598u32,
            Item::Target => 599u32,
            Item::Lever => 600u32,
            Item::LightningRod => 601u32,
            Item::DaylightDetector => 602u32,
            Item::SculkSensor => 603u32,
            Item::TripwireHook => 604u32,
            Item::TrappedChest => 605u32,
            Item::Tnt => 606u32,
            Item::RedstoneLamp => 607u32,
            Item::NoteBlock => 608u32,
            Item::StoneButton => 609u32,
            Item::PolishedBlackstoneButton => 610u32,
            Item::OakButton => 611u32,
            Item::SpruceButton => 612u32,
            Item::BirchButton => 613u32,
            Item::JungleButton => 614u32,
            Item::AcaciaButton => 615u32,
            Item::DarkOakButton => 616u32,
            Item::CrimsonButton => 617u32,
            Item::WarpedButton => 618u32,
            Item::StonePressurePlate => 619u32,
            Item::PolishedBlackstonePressurePlate => 620u32,
            Item::LightWeightedPressurePlate => 621u32,
            Item::HeavyWeightedPressurePlate => 622u32,
            Item::OakPressurePlate => 623u32,
            Item::SprucePressurePlate => 624u32,
            Item::BirchPressurePlate => 625u32,
            Item::JunglePressurePlate => 626u32,
            Item::AcaciaPressurePlate => 627u32,
            Item::DarkOakPressurePlate => 628u32,
            Item::CrimsonPressurePlate => 629u32,
            Item::WarpedPressurePlate => 630u32,
            Item::IronDoor => 631u32,
            Item::OakDoor => 632u32,
            Item::SpruceDoor => 633u32,
            Item::BirchDoor => 634u32,
            Item::JungleDoor => 635u32,
            Item::AcaciaDoor => 636u32,
            Item::DarkOakDoor => 637u32,
            Item::CrimsonDoor => 638u32,
            Item::WarpedDoor => 639u32,
            Item::IronTrapdoor => 640u32,
            Item::OakTrapdoor => 641u32,
            Item::SpruceTrapdoor => 642u32,
            Item::BirchTrapdoor => 643u32,
            Item::JungleTrapdoor => 644u32,
            Item::AcaciaTrapdoor => 645u32,
            Item::DarkOakTrapdoor => 646u32,
            Item::CrimsonTrapdoor => 647u32,
            Item::WarpedTrapdoor => 648u32,
            Item::OakFenceGate => 649u32,
            Item::SpruceFenceGate => 650u32,
            Item::BirchFenceGate => 651u32,
            Item::JungleFenceGate => 652u32,
            Item::AcaciaFenceGate => 653u32,
            Item::DarkOakFenceGate => 654u32,
            Item::CrimsonFenceGate => 655u32,
            Item::WarpedFenceGate => 656u32,
            Item::PoweredRail => 657u32,
            Item::DetectorRail => 658u32,
            Item::Rail => 659u32,
            Item::ActivatorRail => 660u32,
            Item::Saddle => 661u32,
            Item::Minecart => 662u32,
            Item::ChestMinecart => 663u32,
            Item::FurnaceMinecart => 664u32,
            Item::TntMinecart => 665u32,
            Item::HopperMinecart => 666u32,
            Item::CarrotOnAStick => 667u32,
            Item::WarpedFungusOnAStick => 668u32,
            Item::Elytra => 669u32,
            Item::OakBoat => 670u32,
            Item::SpruceBoat => 671u32,
            Item::BirchBoat => 672u32,
            Item::JungleBoat => 673u32,
            Item::AcaciaBoat => 674u32,
            Item::DarkOakBoat => 675u32,
            Item::StructureBlock => 676u32,
            Item::Jigsaw => 677u32,
            Item::TurtleHelmet => 678u32,
            Item::Scute => 679u32,
            Item::FlintAndSteel => 680u32,
            Item::Apple => 681u32,
            Item::Bow => 682u32,
            Item::Arrow => 683u32,
            Item::Coal => 684u32,
            Item::Charcoal => 685u32,
            Item::Diamond => 686u32,
            Item::Emerald => 687u32,
            Item::LapisLazuli => 688u32,
            Item::Quartz => 689u32,
            Item::AmethystShard => 690u32,
            Item::RawIron => 691u32,
            Item::IronIngot => 692u32,
            Item::RawCopper => 693u32,
            Item::CopperIngot => 694u32,
            Item::RawGold => 695u32,
            Item::GoldIngot => 696u32,
            Item::NetheriteIngot => 697u32,
            Item::NetheriteScrap => 698u32,
            Item::WoodenSword => 699u32,
            Item::WoodenShovel => 700u32,
            Item::WoodenPickaxe => 701u32,
            Item::WoodenAxe => 702u32,
            Item::WoodenHoe => 703u32,
            Item::StoneSword => 704u32,
            Item::StoneShovel => 705u32,
            Item::StonePickaxe => 706u32,
            Item::StoneAxe => 707u32,
            Item::StoneHoe => 708u32,
            Item::GoldenSword => 709u32,
            Item::GoldenShovel => 710u32,
            Item::GoldenPickaxe => 711u32,
            Item::GoldenAxe => 712u32,
            Item::GoldenHoe => 713u32,
            Item::IronSword => 714u32,
            Item::IronShovel => 715u32,
            Item::IronPickaxe => 716u32,
            Item::IronAxe => 717u32,
            Item::IronHoe => 718u32,
            Item::DiamondSword => 719u32,
            Item::DiamondShovel => 720u32,
            Item::DiamondPickaxe => 721u32,
            Item::DiamondAxe => 722u32,
            Item::DiamondHoe => 723u32,
            Item::NetheriteSword => 724u32,
            Item::NetheriteShovel => 725u32,
            Item::NetheritePickaxe => 726u32,
            Item::NetheriteAxe => 727u32,
            Item::NetheriteHoe => 728u32,
            Item::Stick => 729u32,
            Item::Bowl => 730u32,
            Item::MushroomStew => 731u32,
            Item::String => 732u32,
            Item::Feather => 733u32,
            Item::Gunpowder => 734u32,
            Item::WheatSeeds => 735u32,
            Item::Wheat => 736u32,
            Item::Bread => 737u32,
            Item::LeatherHelmet => 738u32,
            Item::LeatherChestplate => 739u32,
            Item::LeatherLeggings => 740u32,
            Item::LeatherBoots => 741u32,
            Item::ChainmailHelmet => 742u32,
            Item::ChainmailChestplate => 743u32,
            Item::ChainmailLeggings => 744u32,
            Item::ChainmailBoots => 745u32,
            Item::IronHelmet => 746u32,
            Item::IronChestplate => 747u32,
            Item::IronLeggings => 748u32,
            Item::IronBoots => 749u32,
            Item::DiamondHelmet => 750u32,
            Item::DiamondChestplate => 751u32,
            Item::DiamondLeggings => 752u32,
            Item::DiamondBoots => 753u32,
            Item::GoldenHelmet => 754u32,
            Item::GoldenChestplate => 755u32,
            Item::GoldenLeggings => 756u32,
            Item::GoldenBoots => 757u32,
            Item::NetheriteHelmet => 758u32,
            Item::NetheriteChestplate => 759u32,
            Item::NetheriteLeggings => 760u32,
            Item::NetheriteBoots => 761u32,
            Item::Flint => 762u32,
            Item::Porkchop => 763u32,
            Item::CookedPorkchop => 764u32,
            Item::Painting => 765u32,
            Item::GoldenApple => 766u32,
            Item::EnchantedGoldenApple => 767u32,
            Item::OakSign => 768u32,
            Item::SpruceSign => 769u32,
            Item::BirchSign => 770u32,
            Item::JungleSign => 771u32,
            Item::AcaciaSign => 772u32,
            Item::DarkOakSign => 773u32,
            Item::CrimsonSign => 774u32,
            Item::WarpedSign => 775u32,
            Item::Bucket => 776u32,
            Item::WaterBucket => 777u32,
            Item::LavaBucket => 778u32,
            Item::PowderSnowBucket => 779u32,
            Item::Snowball => 780u32,
            Item::Leather => 781u32,
            Item::MilkBucket => 782u32,
            Item::PufferfishBucket => 783u32,
            Item::SalmonBucket => 784u32,
            Item::CodBucket => 785u32,
            Item::TropicalFishBucket => 786u32,
            Item::AxolotlBucket => 787u32,
            Item::Brick => 788u32,
            Item::ClayBall => 789u32,
            Item::DriedKelpBlock => 790u32,
            Item::Paper => 791u32,
            Item::Book => 792u32,
            Item::SlimeBall => 793u32,
            Item::Egg => 794u32,
            Item::Compass => 795u32,
            Item::Bundle => 796u32,
            Item::FishingRod => 797u32,
            Item::Clock => 798u32,
            Item::Spyglass => 799u32,
            Item::GlowstoneDust => 800u32,
            Item::Cod => 801u32,
            Item::Salmon => 802u32,
            Item::TropicalFish => 803u32,
            Item::Pufferfish => 804u32,
            Item::CookedCod => 805u32,
            Item::CookedSalmon => 806u32,
            Item::InkSac => 807u32,
            Item::GlowInkSac => 808u32,
            Item::CocoaBeans => 809u32,
            Item::WhiteDye => 810u32,
            Item::OrangeDye => 811u32,
            Item::MagentaDye => 812u32,
            Item::LightBlueDye => 813u32,
            Item::YellowDye => 814u32,
            Item::LimeDye => 815u32,
            Item::PinkDye => 816u32,
            Item::GrayDye => 817u32,
            Item::LightGrayDye => 818u32,
            Item::CyanDye => 819u32,
            Item::PurpleDye => 820u32,
            Item::BlueDye => 821u32,
            Item::BrownDye => 822u32,
            Item::GreenDye => 823u32,
            Item::RedDye => 824u32,
            Item::BlackDye => 825u32,
            Item::BoneMeal => 826u32,
            Item::Bone => 827u32,
            Item::Sugar => 828u32,
            Item::Cake => 829u32,
            Item::WhiteBed => 830u32,
            Item::OrangeBed => 831u32,
            Item::MagentaBed => 832u32,
            Item::LightBlueBed => 833u32,
            Item::YellowBed => 834u32,
            Item::LimeBed => 835u32,
            Item::PinkBed => 836u32,
            Item::GrayBed => 837u32,
            Item::LightGrayBed => 838u32,
            Item::CyanBed => 839u32,
            Item::PurpleBed => 840u32,
            Item::BlueBed => 841u32,
            Item::BrownBed => 842u32,
            Item::GreenBed => 843u32,
            Item::RedBed => 844u32,
            Item::BlackBed => 845u32,
            Item::Cookie => 846u32,
            Item::FilledMap => 847u32,
            Item::Shears => 848u32,
            Item::MelonSlice => 849u32,
            Item::DriedKelp => 850u32,
            Item::PumpkinSeeds => 851u32,
            Item::MelonSeeds => 852u32,
            Item::Beef => 853u32,
            Item::CookedBeef => 854u32,
            Item::Chicken => 855u32,
            Item::CookedChicken => 856u32,
            Item::RottenFlesh => 857u32,
            Item::EnderPearl => 858u32,
            Item::BlazeRod => 859u32,
            Item::GhastTear => 860u32,
            Item::GoldNugget => 861u32,
            Item::NetherWart => 862u32,
            Item::Potion => 863u32,
            Item::GlassBottle => 864u32,
            Item::SpiderEye => 865u32,
            Item::FermentedSpiderEye => 866u32,
            Item::BlazePowder => 867u32,
            Item::MagmaCream => 868u32,
            Item::BrewingStand => 869u32,
            Item::Cauldron => 870u32,
            Item::EnderEye => 871u32,
            Item::GlisteringMelonSlice => 872u32,
            Item::AxolotlSpawnEgg => 873u32,
            Item::BatSpawnEgg => 874u32,
            Item::BeeSpawnEgg => 875u32,
            Item::BlazeSpawnEgg => 876u32,
            Item::CatSpawnEgg => 877u32,
            Item::CaveSpiderSpawnEgg => 878u32,
            Item::ChickenSpawnEgg => 879u32,
            Item::CodSpawnEgg => 880u32,
            Item::CowSpawnEgg => 881u32,
            Item::CreeperSpawnEgg => 882u32,
            Item::DolphinSpawnEgg => 883u32,
            Item::DonkeySpawnEgg => 884u32,
            Item::DrownedSpawnEgg => 885u32,
            Item::ElderGuardianSpawnEgg => 886u32,
            Item::EndermanSpawnEgg => 887u32,
            Item::EndermiteSpawnEgg => 888u32,
            Item::EvokerSpawnEgg => 889u32,
            Item::FoxSpawnEgg => 890u32,
            Item::GhastSpawnEgg => 891u32,
            Item::GlowSquidSpawnEgg => 892u32,
            Item::GoatSpawnEgg => 893u32,
            Item::GuardianSpawnEgg => 894u32,
            Item::HoglinSpawnEgg => 895u32,
            Item::HorseSpawnEgg => 896u32,
            Item::HuskSpawnEgg => 897u32,
            Item::LlamaSpawnEgg => 898u32,
            Item::MagmaCubeSpawnEgg => 899u32,
            Item::MooshroomSpawnEgg => 900u32,
            Item::MuleSpawnEgg => 901u32,
            Item::OcelotSpawnEgg => 902u32,
            Item::PandaSpawnEgg => 903u32,
            Item::ParrotSpawnEgg => 904u32,
            Item::PhantomSpawnEgg => 905u32,
            Item::PigSpawnEgg => 906u32,
            Item::PiglinSpawnEgg => 907u32,
            Item::PiglinBruteSpawnEgg => 908u32,
            Item::PillagerSpawnEgg => 909u32,
            Item::PolarBearSpawnEgg => 910u32,
            Item::PufferfishSpawnEgg => 911u32,
            Item::RabbitSpawnEgg => 912u32,
            Item::RavagerSpawnEgg => 913u32,
            Item::SalmonSpawnEgg => 914u32,
            Item::SheepSpawnEgg => 915u32,
            Item::ShulkerSpawnEgg => 916u32,
            Item::SilverfishSpawnEgg => 917u32,
            Item::SkeletonSpawnEgg => 918u32,
            Item::SkeletonHorseSpawnEgg => 919u32,
            Item::SlimeSpawnEgg => 920u32,
            Item::SpiderSpawnEgg => 921u32,
            Item::SquidSpawnEgg => 922u32,
            Item::StraySpawnEgg => 923u32,
            Item::StriderSpawnEgg => 924u32,
            Item::TraderLlamaSpawnEgg => 925u32,
            Item::TropicalFishSpawnEgg => 926u32,
            Item::TurtleSpawnEgg => 927u32,
            Item::VexSpawnEgg => 928u32,
            Item::VillagerSpawnEgg => 929u32,
            Item::VindicatorSpawnEgg => 930u32,
            Item::WanderingTraderSpawnEgg => 931u32,
            Item::WitchSpawnEgg => 932u32,
            Item::WitherSkeletonSpawnEgg => 933u32,
            Item::WolfSpawnEgg => 934u32,
            Item::ZoglinSpawnEgg => 935u32,
            Item::ZombieSpawnEgg => 936u32,
            Item::ZombieHorseSpawnEgg => 937u32,
            Item::ZombieVillagerSpawnEgg => 938u32,
            Item::ZombifiedPiglinSpawnEgg => 939u32,
            Item::ExperienceBottle => 940u32,
            Item::FireCharge => 941u32,
            Item::WritableBook => 942u32,
            Item::WrittenBook => 943u32,
            Item::ItemFrame => 944u32,
            Item::GlowItemFrame => 945u32,
            Item::FlowerPot => 946u32,
            Item::Carrot => 947u32,
            Item::Potato => 948u32,
            Item::BakedPotato => 949u32,
            Item::PoisonousPotato => 950u32,
            Item::Map => 951u32,
            Item::GoldenCarrot => 952u32,
            Item::SkeletonSkull => 953u32,
            Item::WitherSkeletonSkull => 954u32,
            Item::PlayerHead => 955u32,
            Item::ZombieHead => 956u32,
            Item::CreeperHead => 957u32,
            Item::DragonHead => 958u32,
            Item::NetherStar => 959u32,
            Item::PumpkinPie => 960u32,
            Item::FireworkRocket => 961u32,
            Item::FireworkStar => 962u32,
            Item::EnchantedBook => 963u32,
            Item::NetherBrick => 964u32,
            Item::PrismarineShard => 965u32,
            Item::PrismarineCrystals => 966u32,
            Item::Rabbit => 967u32,
            Item::CookedRabbit => 968u32,
            Item::RabbitStew => 969u32,
            Item::RabbitFoot => 970u32,
            Item::RabbitHide => 971u32,
            Item::ArmorStand => 972u32,
            Item::IronHorseArmor => 973u32,
            Item::GoldenHorseArmor => 974u32,
            Item::DiamondHorseArmor => 975u32,
            Item::LeatherHorseArmor => 976u32,
            Item::Lead => 977u32,
            Item::NameTag => 978u32,
            Item::CommandBlockMinecart => 979u32,
            Item::Mutton => 980u32,
            Item::CookedMutton => 981u32,
            Item::WhiteBanner => 982u32,
            Item::OrangeBanner => 983u32,
            Item::MagentaBanner => 984u32,
            Item::LightBlueBanner => 985u32,
            Item::YellowBanner => 986u32,
            Item::LimeBanner => 987u32,
            Item::PinkBanner => 988u32,
            Item::GrayBanner => 989u32,
            Item::LightGrayBanner => 990u32,
            Item::CyanBanner => 991u32,
            Item::PurpleBanner => 992u32,
            Item::BlueBanner => 993u32,
            Item::BrownBanner => 994u32,
            Item::GreenBanner => 995u32,
            Item::RedBanner => 996u32,
            Item::BlackBanner => 997u32,
            Item::EndCrystal => 998u32,
            Item::ChorusFruit => 999u32,
            Item::PoppedChorusFruit => 1000u32,
            Item::Beetroot => 1001u32,
            Item::BeetrootSeeds => 1002u32,
            Item::BeetrootSoup => 1003u32,
            Item::DragonBreath => 1004u32,
            Item::SplashPotion => 1005u32,
            Item::SpectralArrow => 1006u32,
            Item::TippedArrow => 1007u32,
            Item::LingeringPotion => 1008u32,
            Item::Shield => 1009u32,
            Item::TotemOfUndying => 1010u32,
            Item::ShulkerShell => 1011u32,
            Item::IronNugget => 1012u32,
            Item::KnowledgeBook => 1013u32,
            Item::DebugStick => 1014u32,
            Item::MusicDisc13 => 1015u32,
            Item::MusicDiscCat => 1016u32,
            Item::MusicDiscBlocks => 1017u32,
            Item::MusicDiscChirp => 1018u32,
            Item::MusicDiscFar => 1019u32,
            Item::MusicDiscMall => 1020u32,
            Item::MusicDiscMellohi => 1021u32,
            Item::MusicDiscStal => 1022u32,
            Item::MusicDiscStrad => 1023u32,
            Item::MusicDiscWard => 1024u32,
            Item::MusicDisc11 => 1025u32,
            Item::MusicDiscWait => 1026u32,
            Item::MusicDiscOtherside => 1027u32,
            Item::MusicDiscPigstep => 1028u32,
            Item::Trident => 1029u32,
            Item::PhantomMembrane => 1030u32,
            Item::NautilusShell => 1031u32,
            Item::HeartOfTheSea => 1032u32,
            Item::Crossbow => 1033u32,
            Item::SuspiciousStew => 1034u32,
            Item::Loom => 1035u32,
            Item::FlowerBannerPattern => 1036u32,
            Item::CreeperBannerPattern => 1037u32,
            Item::SkullBannerPattern => 1038u32,
            Item::MojangBannerPattern => 1039u32,
            Item::GlobeBannerPattern => 1040u32,
            Item::PiglinBannerPattern => 1041u32,
            Item::Composter => 1042u32,
            Item::Barrel => 1043u32,
            Item::Smoker => 1044u32,
            Item::BlastFurnace => 1045u32,
            Item::CartographyTable => 1046u32,
            Item::FletchingTable => 1047u32,
            Item::Grindstone => 1048u32,
            Item::SmithingTable => 1049u32,
            Item::Stonecutter => 1050u32,
            Item::Bell => 1051u32,
            Item::Lantern => 1052u32,
            Item::SoulLantern => 1053u32,
            Item::SweetBerries => 1054u32,
            Item::GlowBerries => 1055u32,
            Item::Campfire => 1056u32,
            Item::SoulCampfire => 1057u32,
            Item::Shroomlight => 1058u32,
            Item::Honeycomb => 1059u32,
            Item::BeeNest => 1060u32,
            Item::Beehive => 1061u32,
            Item::HoneyBottle => 1062u32,
            Item::HoneycombBlock => 1063u32,
            Item::Lodestone => 1064u32,
            Item::CryingObsidian => 1065u32,
            Item::Blackstone => 1066u32,
            Item::BlackstoneSlab => 1067u32,
            Item::BlackstoneStairs => 1068u32,
            Item::GildedBlackstone => 1069u32,
            Item::PolishedBlackstone => 1070u32,
            Item::PolishedBlackstoneSlab => 1071u32,
            Item::PolishedBlackstoneStairs => 1072u32,
            Item::ChiseledPolishedBlackstone => 1073u32,
            Item::PolishedBlackstoneBricks => 1074u32,
            Item::PolishedBlackstoneBrickSlab => 1075u32,
            Item::PolishedBlackstoneBrickStairs => 1076u32,
            Item::CrackedPolishedBlackstoneBricks => 1077u32,
            Item::RespawnAnchor => 1078u32,
            Item::Candle => 1079u32,
            Item::WhiteCandle => 1080u32,
            Item::OrangeCandle => 1081u32,
            Item::MagentaCandle => 1082u32,
            Item::LightBlueCandle => 1083u32,
            Item::YellowCandle => 1084u32,
            Item::LimeCandle => 1085u32,
            Item::PinkCandle => 1086u32,
            Item::GrayCandle => 1087u32,
            Item::LightGrayCandle => 1088u32,
            Item::CyanCandle => 1089u32,
            Item::PurpleCandle => 1090u32,
            Item::BlueCandle => 1091u32,
            Item::BrownCandle => 1092u32,
            Item::GreenCandle => 1093u32,
            Item::RedCandle => 1094u32,
            Item::BlackCandle => 1095u32,
            Item::SmallAmethystBud => 1096u32,
            Item::MediumAmethystBud => 1097u32,
            Item::LargeAmethystBud => 1098u32,
            Item::AmethystCluster => 1099u32,
            Item::PointedDripstone => 1100u32,
        }
    }
    #[doc = "Gets a `Item` by its `id`."]
    #[inline]
    pub fn from_id(id: u32) -> Option<Self> {
        match id {
            1u32 => Some(Item::Stone),
            2u32 => Some(Item::Granite),
            3u32 => Some(Item::PolishedGranite),
            4u32 => Some(Item::Diorite),
            5u32 => Some(Item::PolishedDiorite),
            6u32 => Some(Item::Andesite),
            7u32 => Some(Item::PolishedAndesite),
            8u32 => Some(Item::Deepslate),
            9u32 => Some(Item::CobbledDeepslate),
            10u32 => Some(Item::PolishedDeepslate),
            11u32 => Some(Item::Calcite),
            12u32 => Some(Item::Tuff),
            13u32 => Some(Item::DripstoneBlock),
            14u32 => Some(Item::GrassBlock),
            15u32 => Some(Item::Dirt),
            16u32 => Some(Item::CoarseDirt),
            17u32 => Some(Item::Podzol),
            18u32 => Some(Item::RootedDirt),
            19u32 => Some(Item::CrimsonNylium),
            20u32 => Some(Item::WarpedNylium),
            21u32 => Some(Item::Cobblestone),
            22u32 => Some(Item::OakPlanks),
            23u32 => Some(Item::SprucePlanks),
            24u32 => Some(Item::BirchPlanks),
            25u32 => Some(Item::JunglePlanks),
            26u32 => Some(Item::AcaciaPlanks),
            27u32 => Some(Item::DarkOakPlanks),
            28u32 => Some(Item::CrimsonPlanks),
            29u32 => Some(Item::WarpedPlanks),
            30u32 => Some(Item::OakSapling),
            31u32 => Some(Item::SpruceSapling),
            32u32 => Some(Item::BirchSapling),
            33u32 => Some(Item::JungleSapling),
            34u32 => Some(Item::AcaciaSapling),
            35u32 => Some(Item::DarkOakSapling),
            36u32 => Some(Item::Bedrock),
            37u32 => Some(Item::Sand),
            38u32 => Some(Item::RedSand),
            39u32 => Some(Item::Gravel),
            40u32 => Some(Item::CoalOre),
            41u32 => Some(Item::DeepslateCoalOre),
            42u32 => Some(Item::IronOre),
            43u32 => Some(Item::DeepslateIronOre),
            44u32 => Some(Item::CopperOre),
            45u32 => Some(Item::DeepslateCopperOre),
            46u32 => Some(Item::GoldOre),
            47u32 => Some(Item::DeepslateGoldOre),
            48u32 => Some(Item::RedstoneOre),
            49u32 => Some(Item::DeepslateRedstoneOre),
            50u32 => Some(Item::EmeraldOre),
            51u32 => Some(Item::DeepslateEmeraldOre),
            52u32 => Some(Item::LapisOre),
            53u32 => Some(Item::DeepslateLapisOre),
            54u32 => Some(Item::DiamondOre),
            55u32 => Some(Item::DeepslateDiamondOre),
            56u32 => Some(Item::NetherGoldOre),
            57u32 => Some(Item::NetherQuartzOre),
            58u32 => Some(Item::AncientDebris),
            59u32 => Some(Item::CoalBlock),
            60u32 => Some(Item::RawIronBlock),
            61u32 => Some(Item::RawCopperBlock),
            62u32 => Some(Item::RawGoldBlock),
            63u32 => Some(Item::AmethystBlock),
            64u32 => Some(Item::BuddingAmethyst),
            65u32 => Some(Item::IronBlock),
            66u32 => Some(Item::CopperBlock),
            67u32 => Some(Item::GoldBlock),
            68u32 => Some(Item::DiamondBlock),
            69u32 => Some(Item::NetheriteBlock),
            70u32 => Some(Item::ExposedCopper),
            71u32 => Some(Item::WeatheredCopper),
            72u32 => Some(Item::OxidizedCopper),
            73u32 => Some(Item::CutCopper),
            74u32 => Some(Item::ExposedCutCopper),
            75u32 => Some(Item::WeatheredCutCopper),
            76u32 => Some(Item::OxidizedCutCopper),
            77u32 => Some(Item::CutCopperStairs),
            78u32 => Some(Item::ExposedCutCopperStairs),
            79u32 => Some(Item::WeatheredCutCopperStairs),
            80u32 => Some(Item::OxidizedCutCopperStairs),
            81u32 => Some(Item::CutCopperSlab),
            82u32 => Some(Item::ExposedCutCopperSlab),
            83u32 => Some(Item::WeatheredCutCopperSlab),
            84u32 => Some(Item::OxidizedCutCopperSlab),
            85u32 => Some(Item::WaxedCopperBlock),
            86u32 => Some(Item::WaxedExposedCopper),
            87u32 => Some(Item::WaxedWeatheredCopper),
            88u32 => Some(Item::WaxedOxidizedCopper),
            89u32 => Some(Item::WaxedCutCopper),
            90u32 => Some(Item::WaxedExposedCutCopper),
            91u32 => Some(Item::WaxedWeatheredCutCopper),
            92u32 => Some(Item::WaxedOxidizedCutCopper),
            93u32 => Some(Item::WaxedCutCopperStairs),
            94u32 => Some(Item::WaxedExposedCutCopperStairs),
            95u32 => Some(Item::WaxedWeatheredCutCopperStairs),
            96u32 => Some(Item::WaxedOxidizedCutCopperStairs),
            97u32 => Some(Item::WaxedCutCopperSlab),
            98u32 => Some(Item::WaxedExposedCutCopperSlab),
            99u32 => Some(Item::WaxedWeatheredCutCopperSlab),
            100u32 => Some(Item::WaxedOxidizedCutCopperSlab),
            101u32 => Some(Item::OakLog),
            102u32 => Some(Item::SpruceLog),
            103u32 => Some(Item::BirchLog),
            104u32 => Some(Item::JungleLog),
            105u32 => Some(Item::AcaciaLog),
            106u32 => Some(Item::DarkOakLog),
            107u32 => Some(Item::CrimsonStem),
            108u32 => Some(Item::WarpedStem),
            109u32 => Some(Item::StrippedOakLog),
            110u32 => Some(Item::StrippedSpruceLog),
            111u32 => Some(Item::StrippedBirchLog),
            112u32 => Some(Item::StrippedJungleLog),
            113u32 => Some(Item::StrippedAcaciaLog),
            114u32 => Some(Item::StrippedDarkOakLog),
            115u32 => Some(Item::StrippedCrimsonStem),
            116u32 => Some(Item::StrippedWarpedStem),
            117u32 => Some(Item::StrippedOakWood),
            118u32 => Some(Item::StrippedSpruceWood),
            119u32 => Some(Item::StrippedBirchWood),
            120u32 => Some(Item::StrippedJungleWood),
            121u32 => Some(Item::StrippedAcaciaWood),
            122u32 => Some(Item::StrippedDarkOakWood),
            123u32 => Some(Item::StrippedCrimsonHyphae),
            124u32 => Some(Item::StrippedWarpedHyphae),
            125u32 => Some(Item::OakWood),
            126u32 => Some(Item::SpruceWood),
            127u32 => Some(Item::BirchWood),
            128u32 => Some(Item::JungleWood),
            129u32 => Some(Item::AcaciaWood),
            130u32 => Some(Item::DarkOakWood),
            131u32 => Some(Item::CrimsonHyphae),
            132u32 => Some(Item::WarpedHyphae),
            133u32 => Some(Item::OakLeaves),
            134u32 => Some(Item::SpruceLeaves),
            135u32 => Some(Item::BirchLeaves),
            136u32 => Some(Item::JungleLeaves),
            137u32 => Some(Item::AcaciaLeaves),
            138u32 => Some(Item::DarkOakLeaves),
            139u32 => Some(Item::AzaleaLeaves),
            140u32 => Some(Item::FloweringAzaleaLeaves),
            141u32 => Some(Item::Sponge),
            142u32 => Some(Item::WetSponge),
            143u32 => Some(Item::Glass),
            144u32 => Some(Item::TintedGlass),
            145u32 => Some(Item::LapisBlock),
            146u32 => Some(Item::Sandstone),
            147u32 => Some(Item::ChiseledSandstone),
            148u32 => Some(Item::CutSandstone),
            149u32 => Some(Item::Cobweb),
            150u32 => Some(Item::Grass),
            151u32 => Some(Item::Fern),
            152u32 => Some(Item::Azalea),
            153u32 => Some(Item::FloweringAzalea),
            154u32 => Some(Item::DeadBush),
            155u32 => Some(Item::Seagrass),
            156u32 => Some(Item::SeaPickle),
            157u32 => Some(Item::WhiteWool),
            158u32 => Some(Item::OrangeWool),
            159u32 => Some(Item::MagentaWool),
            160u32 => Some(Item::LightBlueWool),
            161u32 => Some(Item::YellowWool),
            162u32 => Some(Item::LimeWool),
            163u32 => Some(Item::PinkWool),
            164u32 => Some(Item::GrayWool),
            165u32 => Some(Item::LightGrayWool),
            166u32 => Some(Item::CyanWool),
            167u32 => Some(Item::PurpleWool),
            168u32 => Some(Item::BlueWool),
            169u32 => Some(Item::BrownWool),
            170u32 => Some(Item::GreenWool),
            171u32 => Some(Item::RedWool),
            172u32 => Some(Item::BlackWool),
            173u32 => Some(Item::Dandelion),
            174u32 => Some(Item::Poppy),
            175u32 => Some(Item::BlueOrchid),
            176u32 => Some(Item::Allium),
            177u32 => Some(Item::AzureBluet),
            178u32 => Some(Item::RedTulip),
            179u32 => Some(Item::OrangeTulip),
            180u32 => Some(Item::WhiteTulip),
            181u32 => Some(Item::PinkTulip),
            182u32 => Some(Item::OxeyeDaisy),
            183u32 => Some(Item::Cornflower),
            184u32 => Some(Item::LilyOfTheValley),
            185u32 => Some(Item::WitherRose),
            186u32 => Some(Item::SporeBlossom),
            187u32 => Some(Item::BrownMushroom),
            188u32 => Some(Item::RedMushroom),
            189u32 => Some(Item::CrimsonFungus),
            190u32 => Some(Item::WarpedFungus),
            191u32 => Some(Item::CrimsonRoots),
            192u32 => Some(Item::WarpedRoots),
            193u32 => Some(Item::NetherSprouts),
            194u32 => Some(Item::WeepingVines),
            195u32 => Some(Item::TwistingVines),
            196u32 => Some(Item::SugarCane),
            197u32 => Some(Item::Kelp),
            198u32 => Some(Item::MossCarpet),
            199u32 => Some(Item::MossBlock),
            200u32 => Some(Item::HangingRoots),
            201u32 => Some(Item::BigDripleaf),
            202u32 => Some(Item::SmallDripleaf),
            203u32 => Some(Item::Bamboo),
            204u32 => Some(Item::OakSlab),
            205u32 => Some(Item::SpruceSlab),
            206u32 => Some(Item::BirchSlab),
            207u32 => Some(Item::JungleSlab),
            208u32 => Some(Item::AcaciaSlab),
            209u32 => Some(Item::DarkOakSlab),
            210u32 => Some(Item::CrimsonSlab),
            211u32 => Some(Item::WarpedSlab),
            212u32 => Some(Item::StoneSlab),
            213u32 => Some(Item::SmoothStoneSlab),
            214u32 => Some(Item::SandstoneSlab),
            215u32 => Some(Item::CutSandstoneSlab),
            216u32 => Some(Item::PetrifiedOakSlab),
            217u32 => Some(Item::CobblestoneSlab),
            218u32 => Some(Item::BrickSlab),
            219u32 => Some(Item::StoneBrickSlab),
            220u32 => Some(Item::NetherBrickSlab),
            221u32 => Some(Item::QuartzSlab),
            222u32 => Some(Item::RedSandstoneSlab),
            223u32 => Some(Item::CutRedSandstoneSlab),
            224u32 => Some(Item::PurpurSlab),
            225u32 => Some(Item::PrismarineSlab),
            226u32 => Some(Item::PrismarineBrickSlab),
            227u32 => Some(Item::DarkPrismarineSlab),
            228u32 => Some(Item::SmoothQuartz),
            229u32 => Some(Item::SmoothRedSandstone),
            230u32 => Some(Item::SmoothSandstone),
            231u32 => Some(Item::SmoothStone),
            232u32 => Some(Item::Bricks),
            233u32 => Some(Item::Bookshelf),
            234u32 => Some(Item::MossyCobblestone),
            235u32 => Some(Item::Obsidian),
            236u32 => Some(Item::Torch),
            237u32 => Some(Item::EndRod),
            238u32 => Some(Item::ChorusPlant),
            239u32 => Some(Item::ChorusFlower),
            240u32 => Some(Item::PurpurBlock),
            241u32 => Some(Item::PurpurPillar),
            242u32 => Some(Item::PurpurStairs),
            243u32 => Some(Item::Spawner),
            244u32 => Some(Item::OakStairs),
            245u32 => Some(Item::Chest),
            246u32 => Some(Item::CraftingTable),
            247u32 => Some(Item::Farmland),
            248u32 => Some(Item::Furnace),
            249u32 => Some(Item::Ladder),
            250u32 => Some(Item::CobblestoneStairs),
            251u32 => Some(Item::Snow),
            252u32 => Some(Item::Ice),
            253u32 => Some(Item::SnowBlock),
            254u32 => Some(Item::Cactus),
            255u32 => Some(Item::Clay),
            256u32 => Some(Item::Jukebox),
            257u32 => Some(Item::OakFence),
            258u32 => Some(Item::SpruceFence),
            259u32 => Some(Item::BirchFence),
            260u32 => Some(Item::JungleFence),
            261u32 => Some(Item::AcaciaFence),
            262u32 => Some(Item::DarkOakFence),
            263u32 => Some(Item::CrimsonFence),
            264u32 => Some(Item::WarpedFence),
            265u32 => Some(Item::Pumpkin),
            266u32 => Some(Item::CarvedPumpkin),
            267u32 => Some(Item::JackOLantern),
            268u32 => Some(Item::Netherrack),
            269u32 => Some(Item::SoulSand),
            270u32 => Some(Item::SoulSoil),
            271u32 => Some(Item::Basalt),
            272u32 => Some(Item::PolishedBasalt),
            273u32 => Some(Item::SmoothBasalt),
            274u32 => Some(Item::SoulTorch),
            275u32 => Some(Item::Glowstone),
            276u32 => Some(Item::InfestedStone),
            277u32 => Some(Item::InfestedCobblestone),
            278u32 => Some(Item::InfestedStoneBricks),
            279u32 => Some(Item::InfestedMossyStoneBricks),
            280u32 => Some(Item::InfestedCrackedStoneBricks),
            281u32 => Some(Item::InfestedChiseledStoneBricks),
            282u32 => Some(Item::InfestedDeepslate),
            283u32 => Some(Item::StoneBricks),
            284u32 => Some(Item::MossyStoneBricks),
            285u32 => Some(Item::CrackedStoneBricks),
            286u32 => Some(Item::ChiseledStoneBricks),
            287u32 => Some(Item::DeepslateBricks),
            288u32 => Some(Item::CrackedDeepslateBricks),
            289u32 => Some(Item::DeepslateTiles),
            290u32 => Some(Item::CrackedDeepslateTiles),
            291u32 => Some(Item::ChiseledDeepslate),
            292u32 => Some(Item::BrownMushroomBlock),
            293u32 => Some(Item::RedMushroomBlock),
            294u32 => Some(Item::MushroomStem),
            295u32 => Some(Item::IronBars),
            296u32 => Some(Item::Chain),
            297u32 => Some(Item::GlassPane),
            298u32 => Some(Item::Melon),
            299u32 => Some(Item::Vine),
            300u32 => Some(Item::GlowLichen),
            301u32 => Some(Item::BrickStairs),
            302u32 => Some(Item::StoneBrickStairs),
            303u32 => Some(Item::Mycelium),
            304u32 => Some(Item::LilyPad),
            305u32 => Some(Item::NetherBricks),
            306u32 => Some(Item::CrackedNetherBricks),
            307u32 => Some(Item::ChiseledNetherBricks),
            308u32 => Some(Item::NetherBrickFence),
            309u32 => Some(Item::NetherBrickStairs),
            310u32 => Some(Item::EnchantingTable),
            311u32 => Some(Item::EndPortalFrame),
            312u32 => Some(Item::EndStone),
            313u32 => Some(Item::EndStoneBricks),
            314u32 => Some(Item::DragonEgg),
            315u32 => Some(Item::SandstoneStairs),
            316u32 => Some(Item::EnderChest),
            317u32 => Some(Item::EmeraldBlock),
            318u32 => Some(Item::SpruceStairs),
            319u32 => Some(Item::BirchStairs),
            320u32 => Some(Item::JungleStairs),
            321u32 => Some(Item::CrimsonStairs),
            322u32 => Some(Item::WarpedStairs),
            323u32 => Some(Item::CommandBlock),
            324u32 => Some(Item::Beacon),
            325u32 => Some(Item::CobblestoneWall),
            326u32 => Some(Item::MossyCobblestoneWall),
            327u32 => Some(Item::BrickWall),
            328u32 => Some(Item::PrismarineWall),
            329u32 => Some(Item::RedSandstoneWall),
            330u32 => Some(Item::MossyStoneBrickWall),
            331u32 => Some(Item::GraniteWall),
            332u32 => Some(Item::StoneBrickWall),
            333u32 => Some(Item::NetherBrickWall),
            334u32 => Some(Item::AndesiteWall),
            335u32 => Some(Item::RedNetherBrickWall),
            336u32 => Some(Item::SandstoneWall),
            337u32 => Some(Item::EndStoneBrickWall),
            338u32 => Some(Item::DioriteWall),
            339u32 => Some(Item::BlackstoneWall),
            340u32 => Some(Item::PolishedBlackstoneWall),
            341u32 => Some(Item::PolishedBlackstoneBrickWall),
            342u32 => Some(Item::CobbledDeepslateWall),
            343u32 => Some(Item::PolishedDeepslateWall),
            344u32 => Some(Item::DeepslateBrickWall),
            345u32 => Some(Item::DeepslateTileWall),
            346u32 => Some(Item::Anvil),
            347u32 => Some(Item::ChippedAnvil),
            348u32 => Some(Item::DamagedAnvil),
            349u32 => Some(Item::ChiseledQuartzBlock),
            350u32 => Some(Item::QuartzBlock),
            351u32 => Some(Item::QuartzBricks),
            352u32 => Some(Item::QuartzPillar),
            353u32 => Some(Item::QuartzStairs),
            354u32 => Some(Item::WhiteTerracotta),
            355u32 => Some(Item::OrangeTerracotta),
            356u32 => Some(Item::MagentaTerracotta),
            357u32 => Some(Item::LightBlueTerracotta),
            358u32 => Some(Item::YellowTerracotta),
            359u32 => Some(Item::LimeTerracotta),
            360u32 => Some(Item::PinkTerracotta),
            361u32 => Some(Item::GrayTerracotta),
            362u32 => Some(Item::LightGrayTerracotta),
            363u32 => Some(Item::CyanTerracotta),
            364u32 => Some(Item::PurpleTerracotta),
            365u32 => Some(Item::BlueTerracotta),
            366u32 => Some(Item::BrownTerracotta),
            367u32 => Some(Item::GreenTerracotta),
            368u32 => Some(Item::RedTerracotta),
            369u32 => Some(Item::BlackTerracotta),
            370u32 => Some(Item::Barrier),
            371u32 => Some(Item::Light),
            372u32 => Some(Item::HayBlock),
            373u32 => Some(Item::WhiteCarpet),
            374u32 => Some(Item::OrangeCarpet),
            375u32 => Some(Item::MagentaCarpet),
            376u32 => Some(Item::LightBlueCarpet),
            377u32 => Some(Item::YellowCarpet),
            378u32 => Some(Item::LimeCarpet),
            379u32 => Some(Item::PinkCarpet),
            380u32 => Some(Item::GrayCarpet),
            381u32 => Some(Item::LightGrayCarpet),
            382u32 => Some(Item::CyanCarpet),
            383u32 => Some(Item::PurpleCarpet),
            384u32 => Some(Item::BlueCarpet),
            385u32 => Some(Item::BrownCarpet),
            386u32 => Some(Item::GreenCarpet),
            387u32 => Some(Item::RedCarpet),
            388u32 => Some(Item::BlackCarpet),
            389u32 => Some(Item::Terracotta),
            390u32 => Some(Item::PackedIce),
            391u32 => Some(Item::AcaciaStairs),
            392u32 => Some(Item::DarkOakStairs),
            393u32 => Some(Item::DirtPath),
            394u32 => Some(Item::Sunflower),
            395u32 => Some(Item::Lilac),
            396u32 => Some(Item::RoseBush),
            397u32 => Some(Item::Peony),
            398u32 => Some(Item::TallGrass),
            399u32 => Some(Item::LargeFern),
            400u32 => Some(Item::WhiteStainedGlass),
            401u32 => Some(Item::OrangeStainedGlass),
            402u32 => Some(Item::MagentaStainedGlass),
            403u32 => Some(Item::LightBlueStainedGlass),
            404u32 => Some(Item::YellowStainedGlass),
            405u32 => Some(Item::LimeStainedGlass),
            406u32 => Some(Item::PinkStainedGlass),
            407u32 => Some(Item::GrayStainedGlass),
            408u32 => Some(Item::LightGrayStainedGlass),
            409u32 => Some(Item::CyanStainedGlass),
            410u32 => Some(Item::PurpleStainedGlass),
            411u32 => Some(Item::BlueStainedGlass),
            412u32 => Some(Item::BrownStainedGlass),
            413u32 => Some(Item::GreenStainedGlass),
            414u32 => Some(Item::RedStainedGlass),
            415u32 => Some(Item::BlackStainedGlass),
            416u32 => Some(Item::WhiteStainedGlassPane),
            417u32 => Some(Item::OrangeStainedGlassPane),
            418u32 => Some(Item::MagentaStainedGlassPane),
            419u32 => Some(Item::LightBlueStainedGlassPane),
            420u32 => Some(Item::YellowStainedGlassPane),
            421u32 => Some(Item::LimeStainedGlassPane),
            422u32 => Some(Item::PinkStainedGlassPane),
            423u32 => Some(Item::GrayStainedGlassPane),
            424u32 => Some(Item::LightGrayStainedGlassPane),
            425u32 => Some(Item::CyanStainedGlassPane),
            426u32 => Some(Item::PurpleStainedGlassPane),
            427u32 => Some(Item::BlueStainedGlassPane),
            428u32 => Some(Item::BrownStainedGlassPane),
            429u32 => Some(Item::GreenStainedGlassPane),
            430u32 => Some(Item::RedStainedGlassPane),
            431u32 => Some(Item::BlackStainedGlassPane),
            432u32 => Some(Item::Prismarine),
            433u32 => Some(Item::PrismarineBricks),
            434u32 => Some(Item::DarkPrismarine),
            435u32 => Some(Item::PrismarineStairs),
            436u32 => Some(Item::PrismarineBrickStairs),
            437u32 => Some(Item::DarkPrismarineStairs),
            438u32 => Some(Item::SeaLantern),
            439u32 => Some(Item::RedSandstone),
            440u32 => Some(Item::ChiseledRedSandstone),
            441u32 => Some(Item::CutRedSandstone),
            442u32 => Some(Item::RedSandstoneStairs),
            443u32 => Some(Item::RepeatingCommandBlock),
            444u32 => Some(Item::ChainCommandBlock),
            445u32 => Some(Item::MagmaBlock),
            446u32 => Some(Item::NetherWartBlock),
            447u32 => Some(Item::WarpedWartBlock),
            448u32 => Some(Item::RedNetherBricks),
            449u32 => Some(Item::BoneBlock),
            450u32 => Some(Item::StructureVoid),
            451u32 => Some(Item::ShulkerBox),
            452u32 => Some(Item::WhiteShulkerBox),
            453u32 => Some(Item::OrangeShulkerBox),
            454u32 => Some(Item::MagentaShulkerBox),
            455u32 => Some(Item::LightBlueShulkerBox),
            456u32 => Some(Item::YellowShulkerBox),
            457u32 => Some(Item::LimeShulkerBox),
            458u32 => Some(Item::PinkShulkerBox),
            459u32 => Some(Item::GrayShulkerBox),
            460u32 => Some(Item::LightGrayShulkerBox),
            461u32 => Some(Item::CyanShulkerBox),
            462u32 => Some(Item::PurpleShulkerBox),
            463u32 => Some(Item::BlueShulkerBox),
            464u32 => Some(Item::BrownShulkerBox),
            465u32 => Some(Item::GreenShulkerBox),
            466u32 => Some(Item::RedShulkerBox),
            467u32 => Some(Item::BlackShulkerBox),
            468u32 => Some(Item::WhiteGlazedTerracotta),
            469u32 => Some(Item::OrangeGlazedTerracotta),
            470u32 => Some(Item::MagentaGlazedTerracotta),
            471u32 => Some(Item::LightBlueGlazedTerracotta),
            472u32 => Some(Item::YellowGlazedTerracotta),
            473u32 => Some(Item::LimeGlazedTerracotta),
            474u32 => Some(Item::PinkGlazedTerracotta),
            475u32 => Some(Item::GrayGlazedTerracotta),
            476u32 => Some(Item::LightGrayGlazedTerracotta),
            477u32 => Some(Item::CyanGlazedTerracotta),
            478u32 => Some(Item::PurpleGlazedTerracotta),
            479u32 => Some(Item::BlueGlazedTerracotta),
            480u32 => Some(Item::BrownGlazedTerracotta),
            481u32 => Some(Item::GreenGlazedTerracotta),
            482u32 => Some(Item::RedGlazedTerracotta),
            483u32 => Some(Item::BlackGlazedTerracotta),
            484u32 => Some(Item::WhiteConcrete),
            485u32 => Some(Item::OrangeConcrete),
            486u32 => Some(Item::MagentaConcrete),
            487u32 => Some(Item::LightBlueConcrete),
            488u32 => Some(Item::YellowConcrete),
            489u32 => Some(Item::LimeConcrete),
            490u32 => Some(Item::PinkConcrete),
            491u32 => Some(Item::GrayConcrete),
            492u32 => Some(Item::LightGrayConcrete),
            493u32 => Some(Item::CyanConcrete),
            494u32 => Some(Item::PurpleConcrete),
            495u32 => Some(Item::BlueConcrete),
            496u32 => Some(Item::BrownConcrete),
            497u32 => Some(Item::GreenConcrete),
            498u32 => Some(Item::RedConcrete),
            499u32 => Some(Item::BlackConcrete),
            500u32 => Some(Item::WhiteConcretePowder),
            501u32 => Some(Item::OrangeConcretePowder),
            502u32 => Some(Item::MagentaConcretePowder),
            503u32 => Some(Item::LightBlueConcretePowder),
            504u32 => Some(Item::YellowConcretePowder),
            505u32 => Some(Item::LimeConcretePowder),
            506u32 => Some(Item::PinkConcretePowder),
            507u32 => Some(Item::GrayConcretePowder),
            508u32 => Some(Item::LightGrayConcretePowder),
            509u32 => Some(Item::CyanConcretePowder),
            510u32 => Some(Item::PurpleConcretePowder),
            511u32 => Some(Item::BlueConcretePowder),
            512u32 => Some(Item::BrownConcretePowder),
            513u32 => Some(Item::GreenConcretePowder),
            514u32 => Some(Item::RedConcretePowder),
            515u32 => Some(Item::BlackConcretePowder),
            516u32 => Some(Item::TurtleEgg),
            517u32 => Some(Item::DeadTubeCoralBlock),
            518u32 => Some(Item::DeadBrainCoralBlock),
            519u32 => Some(Item::DeadBubbleCoralBlock),
            520u32 => Some(Item::DeadFireCoralBlock),
            521u32 => Some(Item::DeadHornCoralBlock),
            522u32 => Some(Item::TubeCoralBlock),
            523u32 => Some(Item::BrainCoralBlock),
            524u32 => Some(Item::BubbleCoralBlock),
            525u32 => Some(Item::FireCoralBlock),
            526u32 => Some(Item::HornCoralBlock),
            527u32 => Some(Item::TubeCoral),
            528u32 => Some(Item::BrainCoral),
            529u32 => Some(Item::BubbleCoral),
            530u32 => Some(Item::FireCoral),
            531u32 => Some(Item::HornCoral),
            532u32 => Some(Item::DeadBrainCoral),
            533u32 => Some(Item::DeadBubbleCoral),
            534u32 => Some(Item::DeadFireCoral),
            535u32 => Some(Item::DeadHornCoral),
            536u32 => Some(Item::DeadTubeCoral),
            537u32 => Some(Item::TubeCoralFan),
            538u32 => Some(Item::BrainCoralFan),
            539u32 => Some(Item::BubbleCoralFan),
            540u32 => Some(Item::FireCoralFan),
            541u32 => Some(Item::HornCoralFan),
            542u32 => Some(Item::DeadTubeCoralFan),
            543u32 => Some(Item::DeadBrainCoralFan),
            544u32 => Some(Item::DeadBubbleCoralFan),
            545u32 => Some(Item::DeadFireCoralFan),
            546u32 => Some(Item::DeadHornCoralFan),
            547u32 => Some(Item::BlueIce),
            548u32 => Some(Item::Conduit),
            549u32 => Some(Item::PolishedGraniteStairs),
            550u32 => Some(Item::SmoothRedSandstoneStairs),
            551u32 => Some(Item::MossyStoneBrickStairs),
            552u32 => Some(Item::PolishedDioriteStairs),
            553u32 => Some(Item::MossyCobblestoneStairs),
            554u32 => Some(Item::EndStoneBrickStairs),
            555u32 => Some(Item::StoneStairs),
            556u32 => Some(Item::SmoothSandstoneStairs),
            557u32 => Some(Item::SmoothQuartzStairs),
            558u32 => Some(Item::GraniteStairs),
            559u32 => Some(Item::AndesiteStairs),
            560u32 => Some(Item::RedNetherBrickStairs),
            561u32 => Some(Item::PolishedAndesiteStairs),
            562u32 => Some(Item::DioriteStairs),
            563u32 => Some(Item::CobbledDeepslateStairs),
            564u32 => Some(Item::PolishedDeepslateStairs),
            565u32 => Some(Item::DeepslateBrickStairs),
            566u32 => Some(Item::DeepslateTileStairs),
            567u32 => Some(Item::PolishedGraniteSlab),
            568u32 => Some(Item::SmoothRedSandstoneSlab),
            569u32 => Some(Item::MossyStoneBrickSlab),
            570u32 => Some(Item::PolishedDioriteSlab),
            571u32 => Some(Item::MossyCobblestoneSlab),
            572u32 => Some(Item::EndStoneBrickSlab),
            573u32 => Some(Item::SmoothSandstoneSlab),
            574u32 => Some(Item::SmoothQuartzSlab),
            575u32 => Some(Item::GraniteSlab),
            576u32 => Some(Item::AndesiteSlab),
            577u32 => Some(Item::RedNetherBrickSlab),
            578u32 => Some(Item::PolishedAndesiteSlab),
            579u32 => Some(Item::DioriteSlab),
            580u32 => Some(Item::CobbledDeepslateSlab),
            581u32 => Some(Item::PolishedDeepslateSlab),
            582u32 => Some(Item::DeepslateBrickSlab),
            583u32 => Some(Item::DeepslateTileSlab),
            584u32 => Some(Item::Scaffolding),
            585u32 => Some(Item::Redstone),
            586u32 => Some(Item::RedstoneTorch),
            587u32 => Some(Item::RedstoneBlock),
            588u32 => Some(Item::Repeater),
            589u32 => Some(Item::Comparator),
            590u32 => Some(Item::Piston),
            591u32 => Some(Item::StickyPiston),
            592u32 => Some(Item::SlimeBlock),
            593u32 => Some(Item::HoneyBlock),
            594u32 => Some(Item::Observer),
            595u32 => Some(Item::Hopper),
            596u32 => Some(Item::Dispenser),
            597u32 => Some(Item::Dropper),
            598u32 => Some(Item::Lectern),
            599u32 => Some(Item::Target),
            600u32 => Some(Item::Lever),
            601u32 => Some(Item::LightningRod),
            602u32 => Some(Item::DaylightDetector),
            603u32 => Some(Item::SculkSensor),
            604u32 => Some(Item::TripwireHook),
            605u32 => Some(Item::TrappedChest),
            606u32 => Some(Item::Tnt),
            607u32 => Some(Item::RedstoneLamp),
            608u32 => Some(Item::NoteBlock),
            609u32 => Some(Item::StoneButton),
            610u32 => Some(Item::PolishedBlackstoneButton),
            611u32 => Some(Item::OakButton),
            612u32 => Some(Item::SpruceButton),
            613u32 => Some(Item::BirchButton),
            614u32 => Some(Item::JungleButton),
            615u32 => Some(Item::AcaciaButton),
            616u32 => Some(Item::DarkOakButton),
            617u32 => Some(Item::CrimsonButton),
            618u32 => Some(Item::WarpedButton),
            619u32 => Some(Item::StonePressurePlate),
            620u32 => Some(Item::PolishedBlackstonePressurePlate),
            621u32 => Some(Item::LightWeightedPressurePlate),
            622u32 => Some(Item::HeavyWeightedPressurePlate),
            623u32 => Some(Item::OakPressurePlate),
            624u32 => Some(Item::SprucePressurePlate),
            625u32 => Some(Item::BirchPressurePlate),
            626u32 => Some(Item::JunglePressurePlate),
            627u32 => Some(Item::AcaciaPressurePlate),
            628u32 => Some(Item::DarkOakPressurePlate),
            629u32 => Some(Item::CrimsonPressurePlate),
            630u32 => Some(Item::WarpedPressurePlate),
            631u32 => Some(Item::IronDoor),
            632u32 => Some(Item::OakDoor),
            633u32 => Some(Item::SpruceDoor),
            634u32 => Some(Item::BirchDoor),
            635u32 => Some(Item::JungleDoor),
            636u32 => Some(Item::AcaciaDoor),
            637u32 => Some(Item::DarkOakDoor),
            638u32 => Some(Item::CrimsonDoor),
            639u32 => Some(Item::WarpedDoor),
            640u32 => Some(Item::IronTrapdoor),
            641u32 => Some(Item::OakTrapdoor),
            642u32 => Some(Item::SpruceTrapdoor),
            643u32 => Some(Item::BirchTrapdoor),
            644u32 => Some(Item::JungleTrapdoor),
            645u32 => Some(Item::AcaciaTrapdoor),
            646u32 => Some(Item::DarkOakTrapdoor),
            647u32 => Some(Item::CrimsonTrapdoor),
            648u32 => Some(Item::WarpedTrapdoor),
            649u32 => Some(Item::OakFenceGate),
            650u32 => Some(Item::SpruceFenceGate),
            651u32 => Some(Item::BirchFenceGate),
            652u32 => Some(Item::JungleFenceGate),
            653u32 => Some(Item::AcaciaFenceGate),
            654u32 => Some(Item::DarkOakFenceGate),
            655u32 => Some(Item::CrimsonFenceGate),
            656u32 => Some(Item::WarpedFenceGate),
            657u32 => Some(Item::PoweredRail),
            658u32 => Some(Item::DetectorRail),
            659u32 => Some(Item::Rail),
            660u32 => Some(Item::ActivatorRail),
            661u32 => Some(Item::Saddle),
            662u32 => Some(Item::Minecart),
            663u32 => Some(Item::ChestMinecart),
            664u32 => Some(Item::FurnaceMinecart),
            665u32 => Some(Item::TntMinecart),
            666u32 => Some(Item::HopperMinecart),
            667u32 => Some(Item::CarrotOnAStick),
            668u32 => Some(Item::WarpedFungusOnAStick),
            669u32 => Some(Item::Elytra),
            670u32 => Some(Item::OakBoat),
            671u32 => Some(Item::SpruceBoat),
            672u32 => Some(Item::BirchBoat),
            673u32 => Some(Item::JungleBoat),
            674u32 => Some(Item::AcaciaBoat),
            675u32 => Some(Item::DarkOakBoat),
            676u32 => Some(Item::StructureBlock),
            677u32 => Some(Item::Jigsaw),
            678u32 => Some(Item::TurtleHelmet),
            679u32 => Some(Item::Scute),
            680u32 => Some(Item::FlintAndSteel),
            681u32 => Some(Item::Apple),
            682u32 => Some(Item::Bow),
            683u32 => Some(Item::Arrow),
            684u32 => Some(Item::Coal),
            685u32 => Some(Item::Charcoal),
            686u32 => Some(Item::Diamond),
            687u32 => Some(Item::Emerald),
            688u32 => Some(Item::LapisLazuli),
            689u32 => Some(Item::Quartz),
            690u32 => Some(Item::AmethystShard),
            691u32 => Some(Item::RawIron),
            692u32 => Some(Item::IronIngot),
            693u32 => Some(Item::RawCopper),
            694u32 => Some(Item::CopperIngot),
            695u32 => Some(Item::RawGold),
            696u32 => Some(Item::GoldIngot),
            697u32 => Some(Item::NetheriteIngot),
            698u32 => Some(Item::NetheriteScrap),
            699u32 => Some(Item::WoodenSword),
            700u32 => Some(Item::WoodenShovel),
            701u32 => Some(Item::WoodenPickaxe),
            702u32 => Some(Item::WoodenAxe),
            703u32 => Some(Item::WoodenHoe),
            704u32 => Some(Item::StoneSword),
            705u32 => Some(Item::StoneShovel),
            706u32 => Some(Item::StonePickaxe),
            707u32 => Some(Item::StoneAxe),
            708u32 => Some(Item::StoneHoe),
            709u32 => Some(Item::GoldenSword),
            710u32 => Some(Item::GoldenShovel),
            711u32 => Some(Item::GoldenPickaxe),
            712u32 => Some(Item::GoldenAxe),
            713u32 => Some(Item::GoldenHoe),
            714u32 => Some(Item::IronSword),
            715u32 => Some(Item::IronShovel),
            716u32 => Some(Item::IronPickaxe),
            717u32 => Some(Item::IronAxe),
            718u32 => Some(Item::IronHoe),
            719u32 => Some(Item::DiamondSword),
            720u32 => Some(Item::DiamondShovel),
            721u32 => Some(Item::DiamondPickaxe),
            722u32 => Some(Item::DiamondAxe),
            723u32 => Some(Item::DiamondHoe),
            724u32 => Some(Item::NetheriteSword),
            725u32 => Some(Item::NetheriteShovel),
            726u32 => Some(Item::NetheritePickaxe),
            727u32 => Some(Item::NetheriteAxe),
            728u32 => Some(Item::NetheriteHoe),
            729u32 => Some(Item::Stick),
            730u32 => Some(Item::Bowl),
            731u32 => Some(Item::MushroomStew),
            732u32 => Some(Item::String),
            733u32 => Some(Item::Feather),
            734u32 => Some(Item::Gunpowder),
            735u32 => Some(Item::WheatSeeds),
            736u32 => Some(Item::Wheat),
            737u32 => Some(Item::Bread),
            738u32 => Some(Item::LeatherHelmet),
            739u32 => Some(Item::LeatherChestplate),
            740u32 => Some(Item::LeatherLeggings),
            741u32 => Some(Item::LeatherBoots),
            742u32 => Some(Item::ChainmailHelmet),
            743u32 => Some(Item::ChainmailChestplate),
            744u32 => Some(Item::ChainmailLeggings),
            745u32 => Some(Item::ChainmailBoots),
            746u32 => Some(Item::IronHelmet),
            747u32 => Some(Item::IronChestplate),
            748u32 => Some(Item::IronLeggings),
            749u32 => Some(Item::IronBoots),
            750u32 => Some(Item::DiamondHelmet),
            751u32 => Some(Item::DiamondChestplate),
            752u32 => Some(Item::DiamondLeggings),
            753u32 => Some(Item::DiamondBoots),
            754u32 => Some(Item::GoldenHelmet),
            755u32 => Some(Item::GoldenChestplate),
            756u32 => Some(Item::GoldenLeggings),
            757u32 => Some(Item::GoldenBoots),
            758u32 => Some(Item::NetheriteHelmet),
            759u32 => Some(Item::NetheriteChestplate),
            760u32 => Some(Item::NetheriteLeggings),
            761u32 => Some(Item::NetheriteBoots),
            762u32 => Some(Item::Flint),
            763u32 => Some(Item::Porkchop),
            764u32 => Some(Item::CookedPorkchop),
            765u32 => Some(Item::Painting),
            766u32 => Some(Item::GoldenApple),
            767u32 => Some(Item::EnchantedGoldenApple),
            768u32 => Some(Item::OakSign),
            769u32 => Some(Item::SpruceSign),
            770u32 => Some(Item::BirchSign),
            771u32 => Some(Item::JungleSign),
            772u32 => Some(Item::AcaciaSign),
            773u32 => Some(Item::DarkOakSign),
            774u32 => Some(Item::CrimsonSign),
            775u32 => Some(Item::WarpedSign),
            776u32 => Some(Item::Bucket),
            777u32 => Some(Item::WaterBucket),
            778u32 => Some(Item::LavaBucket),
            779u32 => Some(Item::PowderSnowBucket),
            780u32 => Some(Item::Snowball),
            781u32 => Some(Item::Leather),
            782u32 => Some(Item::MilkBucket),
            783u32 => Some(Item::PufferfishBucket),
            784u32 => Some(Item::SalmonBucket),
            785u32 => Some(Item::CodBucket),
            786u32 => Some(Item::TropicalFishBucket),
            787u32 => Some(Item::AxolotlBucket),
            788u32 => Some(Item::Brick),
            789u32 => Some(Item::ClayBall),
            790u32 => Some(Item::DriedKelpBlock),
            791u32 => Some(Item::Paper),
            792u32 => Some(Item::Book),
            793u32 => Some(Item::SlimeBall),
            794u32 => Some(Item::Egg),
            795u32 => Some(Item::Compass),
            796u32 => Some(Item::Bundle),
            797u32 => Some(Item::FishingRod),
            798u32 => Some(Item::Clock),
            799u32 => Some(Item::Spyglass),
            800u32 => Some(Item::GlowstoneDust),
            801u32 => Some(Item::Cod),
            802u32 => Some(Item::Salmon),
            803u32 => Some(Item::TropicalFish),
            804u32 => Some(Item::Pufferfish),
            805u32 => Some(Item::CookedCod),
            806u32 => Some(Item::CookedSalmon),
            807u32 => Some(Item::InkSac),
            808u32 => Some(Item::GlowInkSac),
            809u32 => Some(Item::CocoaBeans),
            810u32 => Some(Item::WhiteDye),
            811u32 => Some(Item::OrangeDye),
            812u32 => Some(Item::MagentaDye),
            813u32 => Some(Item::LightBlueDye),
            814u32 => Some(Item::YellowDye),
            815u32 => Some(Item::LimeDye),
            816u32 => Some(Item::PinkDye),
            817u32 => Some(Item::GrayDye),
            818u32 => Some(Item::LightGrayDye),
            819u32 => Some(Item::CyanDye),
            820u32 => Some(Item::PurpleDye),
            821u32 => Some(Item::BlueDye),
            822u32 => Some(Item::BrownDye),
            823u32 => Some(Item::GreenDye),
            824u32 => Some(Item::RedDye),
            825u32 => Some(Item::BlackDye),
            826u32 => Some(Item::BoneMeal),
            827u32 => Some(Item::Bone),
            828u32 => Some(Item::Sugar),
            829u32 => Some(Item::Cake),
            830u32 => Some(Item::WhiteBed),
            831u32 => Some(Item::OrangeBed),
            832u32 => Some(Item::MagentaBed),
            833u32 => Some(Item::LightBlueBed),
            834u32 => Some(Item::YellowBed),
            835u32 => Some(Item::LimeBed),
            836u32 => Some(Item::PinkBed),
            837u32 => Some(Item::GrayBed),
            838u32 => Some(Item::LightGrayBed),
            839u32 => Some(Item::CyanBed),
            840u32 => Some(Item::PurpleBed),
            841u32 => Some(Item::BlueBed),
            842u32 => Some(Item::BrownBed),
            843u32 => Some(Item::GreenBed),
            844u32 => Some(Item::RedBed),
            845u32 => Some(Item::BlackBed),
            846u32 => Some(Item::Cookie),
            847u32 => Some(Item::FilledMap),
            848u32 => Some(Item::Shears),
            849u32 => Some(Item::MelonSlice),
            850u32 => Some(Item::DriedKelp),
            851u32 => Some(Item::PumpkinSeeds),
            852u32 => Some(Item::MelonSeeds),
            853u32 => Some(Item::Beef),
            854u32 => Some(Item::CookedBeef),
            855u32 => Some(Item::Chicken),
            856u32 => Some(Item::CookedChicken),
            857u32 => Some(Item::RottenFlesh),
            858u32 => Some(Item::EnderPearl),
            859u32 => Some(Item::BlazeRod),
            860u32 => Some(Item::GhastTear),
            861u32 => Some(Item::GoldNugget),
            862u32 => Some(Item::NetherWart),
            863u32 => Some(Item::Potion),
            864u32 => Some(Item::GlassBottle),
            865u32 => Some(Item::SpiderEye),
            866u32 => Some(Item::FermentedSpiderEye),
            867u32 => Some(Item::BlazePowder),
            868u32 => Some(Item::MagmaCream),
            869u32 => Some(Item::BrewingStand),
            870u32 => Some(Item::Cauldron),
            871u32 => Some(Item::EnderEye),
            872u32 => Some(Item::GlisteringMelonSlice),
            873u32 => Some(Item::AxolotlSpawnEgg),
            874u32 => Some(Item::BatSpawnEgg),
            875u32 => Some(Item::BeeSpawnEgg),
            876u32 => Some(Item::BlazeSpawnEgg),
            877u32 => Some(Item::CatSpawnEgg),
            878u32 => Some(Item::CaveSpiderSpawnEgg),
            879u32 => Some(Item::ChickenSpawnEgg),
            880u32 => Some(Item::CodSpawnEgg),
            881u32 => Some(Item::CowSpawnEgg),
            882u32 => Some(Item::CreeperSpawnEgg),
            883u32 => Some(Item::DolphinSpawnEgg),
            884u32 => Some(Item::DonkeySpawnEgg),
            885u32 => Some(Item::DrownedSpawnEgg),
            886u32 => Some(Item::ElderGuardianSpawnEgg),
            887u32 => Some(Item::EndermanSpawnEgg),
            888u32 => Some(Item::EndermiteSpawnEgg),
            889u32 => Some(Item::EvokerSpawnEgg),
            890u32 => Some(Item::FoxSpawnEgg),
            891u32 => Some(Item::GhastSpawnEgg),
            892u32 => Some(Item::GlowSquidSpawnEgg),
            893u32 => Some(Item::GoatSpawnEgg),
            894u32 => Some(Item::GuardianSpawnEgg),
            895u32 => Some(Item::HoglinSpawnEgg),
            896u32 => Some(Item::HorseSpawnEgg),
            897u32 => Some(Item::HuskSpawnEgg),
            898u32 => Some(Item::LlamaSpawnEgg),
            899u32 => Some(Item::MagmaCubeSpawnEgg),
            900u32 => Some(Item::MooshroomSpawnEgg),
            901u32 => Some(Item::MuleSpawnEgg),
            902u32 => Some(Item::OcelotSpawnEgg),
            903u32 => Some(Item::PandaSpawnEgg),
            904u32 => Some(Item::ParrotSpawnEgg),
            905u32 => Some(Item::PhantomSpawnEgg),
            906u32 => Some(Item::PigSpawnEgg),
            907u32 => Some(Item::PiglinSpawnEgg),
            908u32 => Some(Item::PiglinBruteSpawnEgg),
            909u32 => Some(Item::PillagerSpawnEgg),
            910u32 => Some(Item::PolarBearSpawnEgg),
            911u32 => Some(Item::PufferfishSpawnEgg),
            912u32 => Some(Item::RabbitSpawnEgg),
            913u32 => Some(Item::RavagerSpawnEgg),
            914u32 => Some(Item::SalmonSpawnEgg),
            915u32 => Some(Item::SheepSpawnEgg),
            916u32 => Some(Item::ShulkerSpawnEgg),
            917u32 => Some(Item::SilverfishSpawnEgg),
            918u32 => Some(Item::SkeletonSpawnEgg),
            919u32 => Some(Item::SkeletonHorseSpawnEgg),
            920u32 => Some(Item::SlimeSpawnEgg),
            921u32 => Some(Item::SpiderSpawnEgg),
            922u32 => Some(Item::SquidSpawnEgg),
            923u32 => Some(Item::StraySpawnEgg),
            924u32 => Some(Item::StriderSpawnEgg),
            925u32 => Some(Item::TraderLlamaSpawnEgg),
            926u32 => Some(Item::TropicalFishSpawnEgg),
            927u32 => Some(Item::TurtleSpawnEgg),
            928u32 => Some(Item::VexSpawnEgg),
            929u32 => Some(Item::VillagerSpawnEgg),
            930u32 => Some(Item::VindicatorSpawnEgg),
            931u32 => Some(Item::WanderingTraderSpawnEgg),
            932u32 => Some(Item::WitchSpawnEgg),
            933u32 => Some(Item::WitherSkeletonSpawnEgg),
            934u32 => Some(Item::WolfSpawnEgg),
            935u32 => Some(Item::ZoglinSpawnEgg),
            936u32 => Some(Item::ZombieSpawnEgg),
            937u32 => Some(Item::ZombieHorseSpawnEgg),
            938u32 => Some(Item::ZombieVillagerSpawnEgg),
            939u32 => Some(Item::ZombifiedPiglinSpawnEgg),
            940u32 => Some(Item::ExperienceBottle),
            941u32 => Some(Item::FireCharge),
            942u32 => Some(Item::WritableBook),
            943u32 => Some(Item::WrittenBook),
            944u32 => Some(Item::ItemFrame),
            945u32 => Some(Item::GlowItemFrame),
            946u32 => Some(Item::FlowerPot),
            947u32 => Some(Item::Carrot),
            948u32 => Some(Item::Potato),
            949u32 => Some(Item::BakedPotato),
            950u32 => Some(Item::PoisonousPotato),
            951u32 => Some(Item::Map),
            952u32 => Some(Item::GoldenCarrot),
            953u32 => Some(Item::SkeletonSkull),
            954u32 => Some(Item::WitherSkeletonSkull),
            955u32 => Some(Item::PlayerHead),
            956u32 => Some(Item::ZombieHead),
            957u32 => Some(Item::CreeperHead),
            958u32 => Some(Item::DragonHead),
            959u32 => Some(Item::NetherStar),
            960u32 => Some(Item::PumpkinPie),
            961u32 => Some(Item::FireworkRocket),
            962u32 => Some(Item::FireworkStar),
            963u32 => Some(Item::EnchantedBook),
            964u32 => Some(Item::NetherBrick),
            965u32 => Some(Item::PrismarineShard),
            966u32 => Some(Item::PrismarineCrystals),
            967u32 => Some(Item::Rabbit),
            968u32 => Some(Item::CookedRabbit),
            969u32 => Some(Item::RabbitStew),
            970u32 => Some(Item::RabbitFoot),
            971u32 => Some(Item::RabbitHide),
            972u32 => Some(Item::ArmorStand),
            973u32 => Some(Item::IronHorseArmor),
            974u32 => Some(Item::GoldenHorseArmor),
            975u32 => Some(Item::DiamondHorseArmor),
            976u32 => Some(Item::LeatherHorseArmor),
            977u32 => Some(Item::Lead),
            978u32 => Some(Item::NameTag),
            979u32 => Some(Item::CommandBlockMinecart),
            980u32 => Some(Item::Mutton),
            981u32 => Some(Item::CookedMutton),
            982u32 => Some(Item::WhiteBanner),
            983u32 => Some(Item::OrangeBanner),
            984u32 => Some(Item::MagentaBanner),
            985u32 => Some(Item::LightBlueBanner),
            986u32 => Some(Item::YellowBanner),
            987u32 => Some(Item::LimeBanner),
            988u32 => Some(Item::PinkBanner),
            989u32 => Some(Item::GrayBanner),
            990u32 => Some(Item::LightGrayBanner),
            991u32 => Some(Item::CyanBanner),
            992u32 => Some(Item::PurpleBanner),
            993u32 => Some(Item::BlueBanner),
            994u32 => Some(Item::BrownBanner),
            995u32 => Some(Item::GreenBanner),
            996u32 => Some(Item::RedBanner),
            997u32 => Some(Item::BlackBanner),
            998u32 => Some(Item::EndCrystal),
            999u32 => Some(Item::ChorusFruit),
            1000u32 => Some(Item::PoppedChorusFruit),
            1001u32 => Some(Item::Beetroot),
            1002u32 => Some(Item::BeetrootSeeds),
            1003u32 => Some(Item::BeetrootSoup),
            1004u32 => Some(Item::DragonBreath),
            1005u32 => Some(Item::SplashPotion),
            1006u32 => Some(Item::SpectralArrow),
            1007u32 => Some(Item::TippedArrow),
            1008u32 => Some(Item::LingeringPotion),
            1009u32 => Some(Item::Shield),
            1010u32 => Some(Item::TotemOfUndying),
            1011u32 => Some(Item::ShulkerShell),
            1012u32 => Some(Item::IronNugget),
            1013u32 => Some(Item::KnowledgeBook),
            1014u32 => Some(Item::DebugStick),
            1015u32 => Some(Item::MusicDisc13),
            1016u32 => Some(Item::MusicDiscCat),
            1017u32 => Some(Item::MusicDiscBlocks),
            1018u32 => Some(Item::MusicDiscChirp),
            1019u32 => Some(Item::MusicDiscFar),
            1020u32 => Some(Item::MusicDiscMall),
            1021u32 => Some(Item::MusicDiscMellohi),
            1022u32 => Some(Item::MusicDiscStal),
            1023u32 => Some(Item::MusicDiscStrad),
            1024u32 => Some(Item::MusicDiscWard),
            1025u32 => Some(Item::MusicDisc11),
            1026u32 => Some(Item::MusicDiscWait),
            1027u32 => Some(Item::MusicDiscOtherside),
            1028u32 => Some(Item::MusicDiscPigstep),
            1029u32 => Some(Item::Trident),
            1030u32 => Some(Item::PhantomMembrane),
            1031u32 => Some(Item::NautilusShell),
            1032u32 => Some(Item::HeartOfTheSea),
            1033u32 => Some(Item::Crossbow),
            1034u32 => Some(Item::SuspiciousStew),
            1035u32 => Some(Item::Loom),
            1036u32 => Some(Item::FlowerBannerPattern),
            1037u32 => Some(Item::CreeperBannerPattern),
            1038u32 => Some(Item::SkullBannerPattern),
            1039u32 => Some(Item::MojangBannerPattern),
            1040u32 => Some(Item::GlobeBannerPattern),
            1041u32 => Some(Item::PiglinBannerPattern),
            1042u32 => Some(Item::Composter),
            1043u32 => Some(Item::Barrel),
            1044u32 => Some(Item::Smoker),
            1045u32 => Some(Item::BlastFurnace),
            1046u32 => Some(Item::CartographyTable),
            1047u32 => Some(Item::FletchingTable),
            1048u32 => Some(Item::Grindstone),
            1049u32 => Some(Item::SmithingTable),
            1050u32 => Some(Item::Stonecutter),
            1051u32 => Some(Item::Bell),
            1052u32 => Some(Item::Lantern),
            1053u32 => Some(Item::SoulLantern),
            1054u32 => Some(Item::SweetBerries),
            1055u32 => Some(Item::GlowBerries),
            1056u32 => Some(Item::Campfire),
            1057u32 => Some(Item::SoulCampfire),
            1058u32 => Some(Item::Shroomlight),
            1059u32 => Some(Item::Honeycomb),
            1060u32 => Some(Item::BeeNest),
            1061u32 => Some(Item::Beehive),
            1062u32 => Some(Item::HoneyBottle),
            1063u32 => Some(Item::HoneycombBlock),
            1064u32 => Some(Item::Lodestone),
            1065u32 => Some(Item::CryingObsidian),
            1066u32 => Some(Item::Blackstone),
            1067u32 => Some(Item::BlackstoneSlab),
            1068u32 => Some(Item::BlackstoneStairs),
            1069u32 => Some(Item::GildedBlackstone),
            1070u32 => Some(Item::PolishedBlackstone),
            1071u32 => Some(Item::PolishedBlackstoneSlab),
            1072u32 => Some(Item::PolishedBlackstoneStairs),
            1073u32 => Some(Item::ChiseledPolishedBlackstone),
            1074u32 => Some(Item::PolishedBlackstoneBricks),
            1075u32 => Some(Item::PolishedBlackstoneBrickSlab),
            1076u32 => Some(Item::PolishedBlackstoneBrickStairs),
            1077u32 => Some(Item::CrackedPolishedBlackstoneBricks),
            1078u32 => Some(Item::RespawnAnchor),
            1079u32 => Some(Item::Candle),
            1080u32 => Some(Item::WhiteCandle),
            1081u32 => Some(Item::OrangeCandle),
            1082u32 => Some(Item::MagentaCandle),
            1083u32 => Some(Item::LightBlueCandle),
            1084u32 => Some(Item::YellowCandle),
            1085u32 => Some(Item::LimeCandle),
            1086u32 => Some(Item::PinkCandle),
            1087u32 => Some(Item::GrayCandle),
            1088u32 => Some(Item::LightGrayCandle),
            1089u32 => Some(Item::CyanCandle),
            1090u32 => Some(Item::PurpleCandle),
            1091u32 => Some(Item::BlueCandle),
            1092u32 => Some(Item::BrownCandle),
            1093u32 => Some(Item::GreenCandle),
            1094u32 => Some(Item::RedCandle),
            1095u32 => Some(Item::BlackCandle),
            1096u32 => Some(Item::SmallAmethystBud),
            1097u32 => Some(Item::MediumAmethystBud),
            1098u32 => Some(Item::LargeAmethystBud),
            1099u32 => Some(Item::AmethystCluster),
            1100u32 => Some(Item::PointedDripstone),
            _ => None,
        }
    }
}
impl Item {
    #[doc = "Returns the `name` property of this `Item`."]
    #[inline]
    pub fn name(&self) -> &'static str {
        match self {
            Item::Stone => "stone",
            Item::Granite => "granite",
            Item::PolishedGranite => "polished_granite",
            Item::Diorite => "diorite",
            Item::PolishedDiorite => "polished_diorite",
            Item::Andesite => "andesite",
            Item::PolishedAndesite => "polished_andesite",
            Item::Deepslate => "deepslate",
            Item::CobbledDeepslate => "cobbled_deepslate",
            Item::PolishedDeepslate => "polished_deepslate",
            Item::Calcite => "calcite",
            Item::Tuff => "tuff",
            Item::DripstoneBlock => "dripstone_block",
            Item::GrassBlock => "grass_block",
            Item::Dirt => "dirt",
            Item::CoarseDirt => "coarse_dirt",
            Item::Podzol => "podzol",
            Item::RootedDirt => "rooted_dirt",
            Item::CrimsonNylium => "crimson_nylium",
            Item::WarpedNylium => "warped_nylium",
            Item::Cobblestone => "cobblestone",
            Item::OakPlanks => "oak_planks",
            Item::SprucePlanks => "spruce_planks",
            Item::BirchPlanks => "birch_planks",
            Item::JunglePlanks => "jungle_planks",
            Item::AcaciaPlanks => "acacia_planks",
            Item::DarkOakPlanks => "dark_oak_planks",
            Item::CrimsonPlanks => "crimson_planks",
            Item::WarpedPlanks => "warped_planks",
            Item::OakSapling => "oak_sapling",
            Item::SpruceSapling => "spruce_sapling",
            Item::BirchSapling => "birch_sapling",
            Item::JungleSapling => "jungle_sapling",
            Item::AcaciaSapling => "acacia_sapling",
            Item::DarkOakSapling => "dark_oak_sapling",
            Item::Bedrock => "bedrock",
            Item::Sand => "sand",
            Item::RedSand => "red_sand",
            Item::Gravel => "gravel",
            Item::CoalOre => "coal_ore",
            Item::DeepslateCoalOre => "deepslate_coal_ore",
            Item::IronOre => "iron_ore",
            Item::DeepslateIronOre => "deepslate_iron_ore",
            Item::CopperOre => "copper_ore",
            Item::DeepslateCopperOre => "deepslate_copper_ore",
            Item::GoldOre => "gold_ore",
            Item::DeepslateGoldOre => "deepslate_gold_ore",
            Item::RedstoneOre => "redstone_ore",
            Item::DeepslateRedstoneOre => "deepslate_redstone_ore",
            Item::EmeraldOre => "emerald_ore",
            Item::DeepslateEmeraldOre => "deepslate_emerald_ore",
            Item::LapisOre => "lapis_ore",
            Item::DeepslateLapisOre => "deepslate_lapis_ore",
            Item::DiamondOre => "diamond_ore",
            Item::DeepslateDiamondOre => "deepslate_diamond_ore",
            Item::NetherGoldOre => "nether_gold_ore",
            Item::NetherQuartzOre => "nether_quartz_ore",
            Item::AncientDebris => "ancient_debris",
            Item::CoalBlock => "coal_block",
            Item::RawIronBlock => "raw_iron_block",
            Item::RawCopperBlock => "raw_copper_block",
            Item::RawGoldBlock => "raw_gold_block",
            Item::AmethystBlock => "amethyst_block",
            Item::BuddingAmethyst => "budding_amethyst",
            Item::IronBlock => "iron_block",
            Item::CopperBlock => "copper_block",
            Item::GoldBlock => "gold_block",
            Item::DiamondBlock => "diamond_block",
            Item::NetheriteBlock => "netherite_block",
            Item::ExposedCopper => "exposed_copper",
            Item::WeatheredCopper => "weathered_copper",
            Item::OxidizedCopper => "oxidized_copper",
            Item::CutCopper => "cut_copper",
            Item::ExposedCutCopper => "exposed_cut_copper",
            Item::WeatheredCutCopper => "weathered_cut_copper",
            Item::OxidizedCutCopper => "oxidized_cut_copper",
            Item::CutCopperStairs => "cut_copper_stairs",
            Item::ExposedCutCopperStairs => "exposed_cut_copper_stairs",
            Item::WeatheredCutCopperStairs => "weathered_cut_copper_stairs",
            Item::OxidizedCutCopperStairs => "oxidized_cut_copper_stairs",
            Item::CutCopperSlab => "cut_copper_slab",
            Item::ExposedCutCopperSlab => "exposed_cut_copper_slab",
            Item::WeatheredCutCopperSlab => "weathered_cut_copper_slab",
            Item::OxidizedCutCopperSlab => "oxidized_cut_copper_slab",
            Item::WaxedCopperBlock => "waxed_copper_block",
            Item::WaxedExposedCopper => "waxed_exposed_copper",
            Item::WaxedWeatheredCopper => "waxed_weathered_copper",
            Item::WaxedOxidizedCopper => "waxed_oxidized_copper",
            Item::WaxedCutCopper => "waxed_cut_copper",
            Item::WaxedExposedCutCopper => "waxed_exposed_cut_copper",
            Item::WaxedWeatheredCutCopper => "waxed_weathered_cut_copper",
            Item::WaxedOxidizedCutCopper => "waxed_oxidized_cut_copper",
            Item::WaxedCutCopperStairs => "waxed_cut_copper_stairs",
            Item::WaxedExposedCutCopperStairs => "waxed_exposed_cut_copper_stairs",
            Item::WaxedWeatheredCutCopperStairs => "waxed_weathered_cut_copper_stairs",
            Item::WaxedOxidizedCutCopperStairs => "waxed_oxidized_cut_copper_stairs",
            Item::WaxedCutCopperSlab => "waxed_cut_copper_slab",
            Item::WaxedExposedCutCopperSlab => "waxed_exposed_cut_copper_slab",
            Item::WaxedWeatheredCutCopperSlab => "waxed_weathered_cut_copper_slab",
            Item::WaxedOxidizedCutCopperSlab => "waxed_oxidized_cut_copper_slab",
            Item::OakLog => "oak_log",
            Item::SpruceLog => "spruce_log",
            Item::BirchLog => "birch_log",
            Item::JungleLog => "jungle_log",
            Item::AcaciaLog => "acacia_log",
            Item::DarkOakLog => "dark_oak_log",
            Item::CrimsonStem => "crimson_stem",
            Item::WarpedStem => "warped_stem",
            Item::StrippedOakLog => "stripped_oak_log",
            Item::StrippedSpruceLog => "stripped_spruce_log",
            Item::StrippedBirchLog => "stripped_birch_log",
            Item::StrippedJungleLog => "stripped_jungle_log",
            Item::StrippedAcaciaLog => "stripped_acacia_log",
            Item::StrippedDarkOakLog => "stripped_dark_oak_log",
            Item::StrippedCrimsonStem => "stripped_crimson_stem",
            Item::StrippedWarpedStem => "stripped_warped_stem",
            Item::StrippedOakWood => "stripped_oak_wood",
            Item::StrippedSpruceWood => "stripped_spruce_wood",
            Item::StrippedBirchWood => "stripped_birch_wood",
            Item::StrippedJungleWood => "stripped_jungle_wood",
            Item::StrippedAcaciaWood => "stripped_acacia_wood",
            Item::StrippedDarkOakWood => "stripped_dark_oak_wood",
            Item::StrippedCrimsonHyphae => "stripped_crimson_hyphae",
            Item::StrippedWarpedHyphae => "stripped_warped_hyphae",
            Item::OakWood => "oak_wood",
            Item::SpruceWood => "spruce_wood",
            Item::BirchWood => "birch_wood",
            Item::JungleWood => "jungle_wood",
            Item::AcaciaWood => "acacia_wood",
            Item::DarkOakWood => "dark_oak_wood",
            Item::CrimsonHyphae => "crimson_hyphae",
            Item::WarpedHyphae => "warped_hyphae",
            Item::OakLeaves => "oak_leaves",
            Item::SpruceLeaves => "spruce_leaves",
            Item::BirchLeaves => "birch_leaves",
            Item::JungleLeaves => "jungle_leaves",
            Item::AcaciaLeaves => "acacia_leaves",
            Item::DarkOakLeaves => "dark_oak_leaves",
            Item::AzaleaLeaves => "azalea_leaves",
            Item::FloweringAzaleaLeaves => "flowering_azalea_leaves",
            Item::Sponge => "sponge",
            Item::WetSponge => "wet_sponge",
            Item::Glass => "glass",
            Item::TintedGlass => "tinted_glass",
            Item::LapisBlock => "lapis_block",
            Item::Sandstone => "sandstone",
            Item::ChiseledSandstone => "chiseled_sandstone",
            Item::CutSandstone => "cut_sandstone",
            Item::Cobweb => "cobweb",
            Item::Grass => "grass",
            Item::Fern => "fern",
            Item::Azalea => "azalea",
            Item::FloweringAzalea => "flowering_azalea",
            Item::DeadBush => "dead_bush",
            Item::Seagrass => "seagrass",
            Item::SeaPickle => "sea_pickle",
            Item::WhiteWool => "white_wool",
            Item::OrangeWool => "orange_wool",
            Item::MagentaWool => "magenta_wool",
            Item::LightBlueWool => "light_blue_wool",
            Item::YellowWool => "yellow_wool",
            Item::LimeWool => "lime_wool",
            Item::PinkWool => "pink_wool",
            Item::GrayWool => "gray_wool",
            Item::LightGrayWool => "light_gray_wool",
            Item::CyanWool => "cyan_wool",
            Item::PurpleWool => "purple_wool",
            Item::BlueWool => "blue_wool",
            Item::BrownWool => "brown_wool",
            Item::GreenWool => "green_wool",
            Item::RedWool => "red_wool",
            Item::BlackWool => "black_wool",
            Item::Dandelion => "dandelion",
            Item::Poppy => "poppy",
            Item::BlueOrchid => "blue_orchid",
            Item::Allium => "allium",
            Item::AzureBluet => "azure_bluet",
            Item::RedTulip => "red_tulip",
            Item::OrangeTulip => "orange_tulip",
            Item::WhiteTulip => "white_tulip",
            Item::PinkTulip => "pink_tulip",
            Item::OxeyeDaisy => "oxeye_daisy",
            Item::Cornflower => "cornflower",
            Item::LilyOfTheValley => "lily_of_the_valley",
            Item::WitherRose => "wither_rose",
            Item::SporeBlossom => "spore_blossom",
            Item::BrownMushroom => "brown_mushroom",
            Item::RedMushroom => "red_mushroom",
            Item::CrimsonFungus => "crimson_fungus",
            Item::WarpedFungus => "warped_fungus",
            Item::CrimsonRoots => "crimson_roots",
            Item::WarpedRoots => "warped_roots",
            Item::NetherSprouts => "nether_sprouts",
            Item::WeepingVines => "weeping_vines",
            Item::TwistingVines => "twisting_vines",
            Item::SugarCane => "sugar_cane",
            Item::Kelp => "kelp",
            Item::MossCarpet => "moss_carpet",
            Item::MossBlock => "moss_block",
            Item::HangingRoots => "hanging_roots",
            Item::BigDripleaf => "big_dripleaf",
            Item::SmallDripleaf => "small_dripleaf",
            Item::Bamboo => "bamboo",
            Item::OakSlab => "oak_slab",
            Item::SpruceSlab => "spruce_slab",
            Item::BirchSlab => "birch_slab",
            Item::JungleSlab => "jungle_slab",
            Item::AcaciaSlab => "acacia_slab",
            Item::DarkOakSlab => "dark_oak_slab",
            Item::CrimsonSlab => "crimson_slab",
            Item::WarpedSlab => "warped_slab",
            Item::StoneSlab => "stone_slab",
            Item::SmoothStoneSlab => "smooth_stone_slab",
            Item::SandstoneSlab => "sandstone_slab",
            Item::CutSandstoneSlab => "cut_sandstone_slab",
            Item::PetrifiedOakSlab => "petrified_oak_slab",
            Item::CobblestoneSlab => "cobblestone_slab",
            Item::BrickSlab => "brick_slab",
            Item::StoneBrickSlab => "stone_brick_slab",
            Item::NetherBrickSlab => "nether_brick_slab",
            Item::QuartzSlab => "quartz_slab",
            Item::RedSandstoneSlab => "red_sandstone_slab",
            Item::CutRedSandstoneSlab => "cut_red_sandstone_slab",
            Item::PurpurSlab => "purpur_slab",
            Item::PrismarineSlab => "prismarine_slab",
            Item::PrismarineBrickSlab => "prismarine_brick_slab",
            Item::DarkPrismarineSlab => "dark_prismarine_slab",
            Item::SmoothQuartz => "smooth_quartz",
            Item::SmoothRedSandstone => "smooth_red_sandstone",
            Item::SmoothSandstone => "smooth_sandstone",
            Item::SmoothStone => "smooth_stone",
            Item::Bricks => "bricks",
            Item::Bookshelf => "bookshelf",
            Item::MossyCobblestone => "mossy_cobblestone",
            Item::Obsidian => "obsidian",
            Item::Torch => "torch",
            Item::EndRod => "end_rod",
            Item::ChorusPlant => "chorus_plant",
            Item::ChorusFlower => "chorus_flower",
            Item::PurpurBlock => "purpur_block",
            Item::PurpurPillar => "purpur_pillar",
            Item::PurpurStairs => "purpur_stairs",
            Item::Spawner => "spawner",
            Item::OakStairs => "oak_stairs",
            Item::Chest => "chest",
            Item::CraftingTable => "crafting_table",
            Item::Farmland => "farmland",
            Item::Furnace => "furnace",
            Item::Ladder => "ladder",
            Item::CobblestoneStairs => "cobblestone_stairs",
            Item::Snow => "snow",
            Item::Ice => "ice",
            Item::SnowBlock => "snow_block",
            Item::Cactus => "cactus",
            Item::Clay => "clay",
            Item::Jukebox => "jukebox",
            Item::OakFence => "oak_fence",
            Item::SpruceFence => "spruce_fence",
            Item::BirchFence => "birch_fence",
            Item::JungleFence => "jungle_fence",
            Item::AcaciaFence => "acacia_fence",
            Item::DarkOakFence => "dark_oak_fence",
            Item::CrimsonFence => "crimson_fence",
            Item::WarpedFence => "warped_fence",
            Item::Pumpkin => "pumpkin",
            Item::CarvedPumpkin => "carved_pumpkin",
            Item::JackOLantern => "jack_o_lantern",
            Item::Netherrack => "netherrack",
            Item::SoulSand => "soul_sand",
            Item::SoulSoil => "soul_soil",
            Item::Basalt => "basalt",
            Item::PolishedBasalt => "polished_basalt",
            Item::SmoothBasalt => "smooth_basalt",
            Item::SoulTorch => "soul_torch",
            Item::Glowstone => "glowstone",
            Item::InfestedStone => "infested_stone",
            Item::InfestedCobblestone => "infested_cobblestone",
            Item::InfestedStoneBricks => "infested_stone_bricks",
            Item::InfestedMossyStoneBricks => "infested_mossy_stone_bricks",
            Item::InfestedCrackedStoneBricks => "infested_cracked_stone_bricks",
            Item::InfestedChiseledStoneBricks => "infested_chiseled_stone_bricks",
            Item::InfestedDeepslate => "infested_deepslate",
            Item::StoneBricks => "stone_bricks",
            Item::MossyStoneBricks => "mossy_stone_bricks",
            Item::CrackedStoneBricks => "cracked_stone_bricks",
            Item::ChiseledStoneBricks => "chiseled_stone_bricks",
            Item::DeepslateBricks => "deepslate_bricks",
            Item::CrackedDeepslateBricks => "cracked_deepslate_bricks",
            Item::DeepslateTiles => "deepslate_tiles",
            Item::CrackedDeepslateTiles => "cracked_deepslate_tiles",
            Item::ChiseledDeepslate => "chiseled_deepslate",
            Item::BrownMushroomBlock => "brown_mushroom_block",
            Item::RedMushroomBlock => "red_mushroom_block",
            Item::MushroomStem => "mushroom_stem",
            Item::IronBars => "iron_bars",
            Item::Chain => "chain",
            Item::GlassPane => "glass_pane",
            Item::Melon => "melon",
            Item::Vine => "vine",
            Item::GlowLichen => "glow_lichen",
            Item::BrickStairs => "brick_stairs",
            Item::StoneBrickStairs => "stone_brick_stairs",
            Item::Mycelium => "mycelium",
            Item::LilyPad => "lily_pad",
            Item::NetherBricks => "nether_bricks",
            Item::CrackedNetherBricks => "cracked_nether_bricks",
            Item::ChiseledNetherBricks => "chiseled_nether_bricks",
            Item::NetherBrickFence => "nether_brick_fence",
            Item::NetherBrickStairs => "nether_brick_stairs",
            Item::EnchantingTable => "enchanting_table",
            Item::EndPortalFrame => "end_portal_frame",
            Item::EndStone => "end_stone",
            Item::EndStoneBricks => "end_stone_bricks",
            Item::DragonEgg => "dragon_egg",
            Item::SandstoneStairs => "sandstone_stairs",
            Item::EnderChest => "ender_chest",
            Item::EmeraldBlock => "emerald_block",
            Item::SpruceStairs => "spruce_stairs",
            Item::BirchStairs => "birch_stairs",
            Item::JungleStairs => "jungle_stairs",
            Item::CrimsonStairs => "crimson_stairs",
            Item::WarpedStairs => "warped_stairs",
            Item::CommandBlock => "command_block",
            Item::Beacon => "beacon",
            Item::CobblestoneWall => "cobblestone_wall",
            Item::MossyCobblestoneWall => "mossy_cobblestone_wall",
            Item::BrickWall => "brick_wall",
            Item::PrismarineWall => "prismarine_wall",
            Item::RedSandstoneWall => "red_sandstone_wall",
            Item::MossyStoneBrickWall => "mossy_stone_brick_wall",
            Item::GraniteWall => "granite_wall",
            Item::StoneBrickWall => "stone_brick_wall",
            Item::NetherBrickWall => "nether_brick_wall",
            Item::AndesiteWall => "andesite_wall",
            Item::RedNetherBrickWall => "red_nether_brick_wall",
            Item::SandstoneWall => "sandstone_wall",
            Item::EndStoneBrickWall => "end_stone_brick_wall",
            Item::DioriteWall => "diorite_wall",
            Item::BlackstoneWall => "blackstone_wall",
            Item::PolishedBlackstoneWall => "polished_blackstone_wall",
            Item::PolishedBlackstoneBrickWall => "polished_blackstone_brick_wall",
            Item::CobbledDeepslateWall => "cobbled_deepslate_wall",
            Item::PolishedDeepslateWall => "polished_deepslate_wall",
            Item::DeepslateBrickWall => "deepslate_brick_wall",
            Item::DeepslateTileWall => "deepslate_tile_wall",
            Item::Anvil => "anvil",
            Item::ChippedAnvil => "chipped_anvil",
            Item::DamagedAnvil => "damaged_anvil",
            Item::ChiseledQuartzBlock => "chiseled_quartz_block",
            Item::QuartzBlock => "quartz_block",
            Item::QuartzBricks => "quartz_bricks",
            Item::QuartzPillar => "quartz_pillar",
            Item::QuartzStairs => "quartz_stairs",
            Item::WhiteTerracotta => "white_terracotta",
            Item::OrangeTerracotta => "orange_terracotta",
            Item::MagentaTerracotta => "magenta_terracotta",
            Item::LightBlueTerracotta => "light_blue_terracotta",
            Item::YellowTerracotta => "yellow_terracotta",
            Item::LimeTerracotta => "lime_terracotta",
            Item::PinkTerracotta => "pink_terracotta",
            Item::GrayTerracotta => "gray_terracotta",
            Item::LightGrayTerracotta => "light_gray_terracotta",
            Item::CyanTerracotta => "cyan_terracotta",
            Item::PurpleTerracotta => "purple_terracotta",
            Item::BlueTerracotta => "blue_terracotta",
            Item::BrownTerracotta => "brown_terracotta",
            Item::GreenTerracotta => "green_terracotta",
            Item::RedTerracotta => "red_terracotta",
            Item::BlackTerracotta => "black_terracotta",
            Item::Barrier => "barrier",
            Item::Light => "light",
            Item::HayBlock => "hay_block",
            Item::WhiteCarpet => "white_carpet",
            Item::OrangeCarpet => "orange_carpet",
            Item::MagentaCarpet => "magenta_carpet",
            Item::LightBlueCarpet => "light_blue_carpet",
            Item::YellowCarpet => "yellow_carpet",
            Item::LimeCarpet => "lime_carpet",
            Item::PinkCarpet => "pink_carpet",
            Item::GrayCarpet => "gray_carpet",
            Item::LightGrayCarpet => "light_gray_carpet",
            Item::CyanCarpet => "cyan_carpet",
            Item::PurpleCarpet => "purple_carpet",
            Item::BlueCarpet => "blue_carpet",
            Item::BrownCarpet => "brown_carpet",
            Item::GreenCarpet => "green_carpet",
            Item::RedCarpet => "red_carpet",
            Item::BlackCarpet => "black_carpet",
            Item::Terracotta => "terracotta",
            Item::PackedIce => "packed_ice",
            Item::AcaciaStairs => "acacia_stairs",
            Item::DarkOakStairs => "dark_oak_stairs",
            Item::DirtPath => "dirt_path",
            Item::Sunflower => "sunflower",
            Item::Lilac => "lilac",
            Item::RoseBush => "rose_bush",
            Item::Peony => "peony",
            Item::TallGrass => "tall_grass",
            Item::LargeFern => "large_fern",
            Item::WhiteStainedGlass => "white_stained_glass",
            Item::OrangeStainedGlass => "orange_stained_glass",
            Item::MagentaStainedGlass => "magenta_stained_glass",
            Item::LightBlueStainedGlass => "light_blue_stained_glass",
            Item::YellowStainedGlass => "yellow_stained_glass",
            Item::LimeStainedGlass => "lime_stained_glass",
            Item::PinkStainedGlass => "pink_stained_glass",
            Item::GrayStainedGlass => "gray_stained_glass",
            Item::LightGrayStainedGlass => "light_gray_stained_glass",
            Item::CyanStainedGlass => "cyan_stained_glass",
            Item::PurpleStainedGlass => "purple_stained_glass",
            Item::BlueStainedGlass => "blue_stained_glass",
            Item::BrownStainedGlass => "brown_stained_glass",
            Item::GreenStainedGlass => "green_stained_glass",
            Item::RedStainedGlass => "red_stained_glass",
            Item::BlackStainedGlass => "black_stained_glass",
            Item::WhiteStainedGlassPane => "white_stained_glass_pane",
            Item::OrangeStainedGlassPane => "orange_stained_glass_pane",
            Item::MagentaStainedGlassPane => "magenta_stained_glass_pane",
            Item::LightBlueStainedGlassPane => "light_blue_stained_glass_pane",
            Item::YellowStainedGlassPane => "yellow_stained_glass_pane",
            Item::LimeStainedGlassPane => "lime_stained_glass_pane",
            Item::PinkStainedGlassPane => "pink_stained_glass_pane",
            Item::GrayStainedGlassPane => "gray_stained_glass_pane",
            Item::LightGrayStainedGlassPane => "light_gray_stained_glass_pane",
            Item::CyanStainedGlassPane => "cyan_stained_glass_pane",
            Item::PurpleStainedGlassPane => "purple_stained_glass_pane",
            Item::BlueStainedGlassPane => "blue_stained_glass_pane",
            Item::BrownStainedGlassPane => "brown_stained_glass_pane",
            Item::GreenStainedGlassPane => "green_stained_glass_pane",
            Item::RedStainedGlassPane => "red_stained_glass_pane",
            Item::BlackStainedGlassPane => "black_stained_glass_pane",
            Item::Prismarine => "prismarine",
            Item::PrismarineBricks => "prismarine_bricks",
            Item::DarkPrismarine => "dark_prismarine",
            Item::PrismarineStairs => "prismarine_stairs",
            Item::PrismarineBrickStairs => "prismarine_brick_stairs",
            Item::DarkPrismarineStairs => "dark_prismarine_stairs",
            Item::SeaLantern => "sea_lantern",
            Item::RedSandstone => "red_sandstone",
            Item::ChiseledRedSandstone => "chiseled_red_sandstone",
            Item::CutRedSandstone => "cut_red_sandstone",
            Item::RedSandstoneStairs => "red_sandstone_stairs",
            Item::RepeatingCommandBlock => "repeating_command_block",
            Item::ChainCommandBlock => "chain_command_block",
            Item::MagmaBlock => "magma_block",
            Item::NetherWartBlock => "nether_wart_block",
            Item::WarpedWartBlock => "warped_wart_block",
            Item::RedNetherBricks => "red_nether_bricks",
            Item::BoneBlock => "bone_block",
            Item::StructureVoid => "structure_void",
            Item::ShulkerBox => "shulker_box",
            Item::WhiteShulkerBox => "white_shulker_box",
            Item::OrangeShulkerBox => "orange_shulker_box",
            Item::MagentaShulkerBox => "magenta_shulker_box",
            Item::LightBlueShulkerBox => "light_blue_shulker_box",
            Item::YellowShulkerBox => "yellow_shulker_box",
            Item::LimeShulkerBox => "lime_shulker_box",
            Item::PinkShulkerBox => "pink_shulker_box",
            Item::GrayShulkerBox => "gray_shulker_box",
            Item::LightGrayShulkerBox => "light_gray_shulker_box",
            Item::CyanShulkerBox => "cyan_shulker_box",
            Item::PurpleShulkerBox => "purple_shulker_box",
            Item::BlueShulkerBox => "blue_shulker_box",
            Item::BrownShulkerBox => "brown_shulker_box",
            Item::GreenShulkerBox => "green_shulker_box",
            Item::RedShulkerBox => "red_shulker_box",
            Item::BlackShulkerBox => "black_shulker_box",
            Item::WhiteGlazedTerracotta => "white_glazed_terracotta",
            Item::OrangeGlazedTerracotta => "orange_glazed_terracotta",
            Item::MagentaGlazedTerracotta => "magenta_glazed_terracotta",
            Item::LightBlueGlazedTerracotta => "light_blue_glazed_terracotta",
            Item::YellowGlazedTerracotta => "yellow_glazed_terracotta",
            Item::LimeGlazedTerracotta => "lime_glazed_terracotta",
            Item::PinkGlazedTerracotta => "pink_glazed_terracotta",
            Item::GrayGlazedTerracotta => "gray_glazed_terracotta",
            Item::LightGrayGlazedTerracotta => "light_gray_glazed_terracotta",
            Item::CyanGlazedTerracotta => "cyan_glazed_terracotta",
            Item::PurpleGlazedTerracotta => "purple_glazed_terracotta",
            Item::BlueGlazedTerracotta => "blue_glazed_terracotta",
            Item::BrownGlazedTerracotta => "brown_glazed_terracotta",
            Item::GreenGlazedTerracotta => "green_glazed_terracotta",
            Item::RedGlazedTerracotta => "red_glazed_terracotta",
            Item::BlackGlazedTerracotta => "black_glazed_terracotta",
            Item::WhiteConcrete => "white_concrete",
            Item::OrangeConcrete => "orange_concrete",
            Item::MagentaConcrete => "magenta_concrete",
            Item::LightBlueConcrete => "light_blue_concrete",
            Item::YellowConcrete => "yellow_concrete",
            Item::LimeConcrete => "lime_concrete",
            Item::PinkConcrete => "pink_concrete",
            Item::GrayConcrete => "gray_concrete",
            Item::LightGrayConcrete => "light_gray_concrete",
            Item::CyanConcrete => "cyan_concrete",
            Item::PurpleConcrete => "purple_concrete",
            Item::BlueConcrete => "blue_concrete",
            Item::BrownConcrete => "brown_concrete",
            Item::GreenConcrete => "green_concrete",
            Item::RedConcrete => "red_concrete",
            Item::BlackConcrete => "black_concrete",
            Item::WhiteConcretePowder => "white_concrete_powder",
            Item::OrangeConcretePowder => "orange_concrete_powder",
            Item::MagentaConcretePowder => "magenta_concrete_powder",
            Item::LightBlueConcretePowder => "light_blue_concrete_powder",
            Item::YellowConcretePowder => "yellow_concrete_powder",
            Item::LimeConcretePowder => "lime_concrete_powder",
            Item::PinkConcretePowder => "pink_concrete_powder",
            Item::GrayConcretePowder => "gray_concrete_powder",
            Item::LightGrayConcretePowder => "light_gray_concrete_powder",
            Item::CyanConcretePowder => "cyan_concrete_powder",
            Item::PurpleConcretePowder => "purple_concrete_powder",
            Item::BlueConcretePowder => "blue_concrete_powder",
            Item::BrownConcretePowder => "brown_concrete_powder",
            Item::GreenConcretePowder => "green_concrete_powder",
            Item::RedConcretePowder => "red_concrete_powder",
            Item::BlackConcretePowder => "black_concrete_powder",
            Item::TurtleEgg => "turtle_egg",
            Item::DeadTubeCoralBlock => "dead_tube_coral_block",
            Item::DeadBrainCoralBlock => "dead_brain_coral_block",
            Item::DeadBubbleCoralBlock => "dead_bubble_coral_block",
            Item::DeadFireCoralBlock => "dead_fire_coral_block",
            Item::DeadHornCoralBlock => "dead_horn_coral_block",
            Item::TubeCoralBlock => "tube_coral_block",
            Item::BrainCoralBlock => "brain_coral_block",
            Item::BubbleCoralBlock => "bubble_coral_block",
            Item::FireCoralBlock => "fire_coral_block",
            Item::HornCoralBlock => "horn_coral_block",
            Item::TubeCoral => "tube_coral",
            Item::BrainCoral => "brain_coral",
            Item::BubbleCoral => "bubble_coral",
            Item::FireCoral => "fire_coral",
            Item::HornCoral => "horn_coral",
            Item::DeadBrainCoral => "dead_brain_coral",
            Item::DeadBubbleCoral => "dead_bubble_coral",
            Item::DeadFireCoral => "dead_fire_coral",
            Item::DeadHornCoral => "dead_horn_coral",
            Item::DeadTubeCoral => "dead_tube_coral",
            Item::TubeCoralFan => "tube_coral_fan",
            Item::BrainCoralFan => "brain_coral_fan",
            Item::BubbleCoralFan => "bubble_coral_fan",
            Item::FireCoralFan => "fire_coral_fan",
            Item::HornCoralFan => "horn_coral_fan",
            Item::DeadTubeCoralFan => "dead_tube_coral_fan",
            Item::DeadBrainCoralFan => "dead_brain_coral_fan",
            Item::DeadBubbleCoralFan => "dead_bubble_coral_fan",
            Item::DeadFireCoralFan => "dead_fire_coral_fan",
            Item::DeadHornCoralFan => "dead_horn_coral_fan",
            Item::BlueIce => "blue_ice",
            Item::Conduit => "conduit",
            Item::PolishedGraniteStairs => "polished_granite_stairs",
            Item::SmoothRedSandstoneStairs => "smooth_red_sandstone_stairs",
            Item::MossyStoneBrickStairs => "mossy_stone_brick_stairs",
            Item::PolishedDioriteStairs => "polished_diorite_stairs",
            Item::MossyCobblestoneStairs => "mossy_cobblestone_stairs",
            Item::EndStoneBrickStairs => "end_stone_brick_stairs",
            Item::StoneStairs => "stone_stairs",
            Item::SmoothSandstoneStairs => "smooth_sandstone_stairs",
            Item::SmoothQuartzStairs => "smooth_quartz_stairs",
            Item::GraniteStairs => "granite_stairs",
            Item::AndesiteStairs => "andesite_stairs",
            Item::RedNetherBrickStairs => "red_nether_brick_stairs",
            Item::PolishedAndesiteStairs => "polished_andesite_stairs",
            Item::DioriteStairs => "diorite_stairs",
            Item::CobbledDeepslateStairs => "cobbled_deepslate_stairs",
            Item::PolishedDeepslateStairs => "polished_deepslate_stairs",
            Item::DeepslateBrickStairs => "deepslate_brick_stairs",
            Item::DeepslateTileStairs => "deepslate_tile_stairs",
            Item::PolishedGraniteSlab => "polished_granite_slab",
            Item::SmoothRedSandstoneSlab => "smooth_red_sandstone_slab",
            Item::MossyStoneBrickSlab => "mossy_stone_brick_slab",
            Item::PolishedDioriteSlab => "polished_diorite_slab",
            Item::MossyCobblestoneSlab => "mossy_cobblestone_slab",
            Item::EndStoneBrickSlab => "end_stone_brick_slab",
            Item::SmoothSandstoneSlab => "smooth_sandstone_slab",
            Item::SmoothQuartzSlab => "smooth_quartz_slab",
            Item::GraniteSlab => "granite_slab",
            Item::AndesiteSlab => "andesite_slab",
            Item::RedNetherBrickSlab => "red_nether_brick_slab",
            Item::PolishedAndesiteSlab => "polished_andesite_slab",
            Item::DioriteSlab => "diorite_slab",
            Item::CobbledDeepslateSlab => "cobbled_deepslate_slab",
            Item::PolishedDeepslateSlab => "polished_deepslate_slab",
            Item::DeepslateBrickSlab => "deepslate_brick_slab",
            Item::DeepslateTileSlab => "deepslate_tile_slab",
            Item::Scaffolding => "scaffolding",
            Item::Redstone => "redstone",
            Item::RedstoneTorch => "redstone_torch",
            Item::RedstoneBlock => "redstone_block",
            Item::Repeater => "repeater",
            Item::Comparator => "comparator",
            Item::Piston => "piston",
            Item::StickyPiston => "sticky_piston",
            Item::SlimeBlock => "slime_block",
            Item::HoneyBlock => "honey_block",
            Item::Observer => "observer",
            Item::Hopper => "hopper",
            Item::Dispenser => "dispenser",
            Item::Dropper => "dropper",
            Item::Lectern => "lectern",
            Item::Target => "target",
            Item::Lever => "lever",
            Item::LightningRod => "lightning_rod",
            Item::DaylightDetector => "daylight_detector",
            Item::SculkSensor => "sculk_sensor",
            Item::TripwireHook => "tripwire_hook",
            Item::TrappedChest => "trapped_chest",
            Item::Tnt => "tnt",
            Item::RedstoneLamp => "redstone_lamp",
            Item::NoteBlock => "note_block",
            Item::StoneButton => "stone_button",
            Item::PolishedBlackstoneButton => "polished_blackstone_button",
            Item::OakButton => "oak_button",
            Item::SpruceButton => "spruce_button",
            Item::BirchButton => "birch_button",
            Item::JungleButton => "jungle_button",
            Item::AcaciaButton => "acacia_button",
            Item::DarkOakButton => "dark_oak_button",
            Item::CrimsonButton => "crimson_button",
            Item::WarpedButton => "warped_button",
            Item::StonePressurePlate => "stone_pressure_plate",
            Item::PolishedBlackstonePressurePlate => "polished_blackstone_pressure_plate",
            Item::LightWeightedPressurePlate => "light_weighted_pressure_plate",
            Item::HeavyWeightedPressurePlate => "heavy_weighted_pressure_plate",
            Item::OakPressurePlate => "oak_pressure_plate",
            Item::SprucePressurePlate => "spruce_pressure_plate",
            Item::BirchPressurePlate => "birch_pressure_plate",
            Item::JunglePressurePlate => "jungle_pressure_plate",
            Item::AcaciaPressurePlate => "acacia_pressure_plate",
            Item::DarkOakPressurePlate => "dark_oak_pressure_plate",
            Item::CrimsonPressurePlate => "crimson_pressure_plate",
            Item::WarpedPressurePlate => "warped_pressure_plate",
            Item::IronDoor => "iron_door",
            Item::OakDoor => "oak_door",
            Item::SpruceDoor => "spruce_door",
            Item::BirchDoor => "birch_door",
            Item::JungleDoor => "jungle_door",
            Item::AcaciaDoor => "acacia_door",
            Item::DarkOakDoor => "dark_oak_door",
            Item::CrimsonDoor => "crimson_door",
            Item::WarpedDoor => "warped_door",
            Item::IronTrapdoor => "iron_trapdoor",
            Item::OakTrapdoor => "oak_trapdoor",
            Item::SpruceTrapdoor => "spruce_trapdoor",
            Item::BirchTrapdoor => "birch_trapdoor",
            Item::JungleTrapdoor => "jungle_trapdoor",
            Item::AcaciaTrapdoor => "acacia_trapdoor",
            Item::DarkOakTrapdoor => "dark_oak_trapdoor",
            Item::CrimsonTrapdoor => "crimson_trapdoor",
            Item::WarpedTrapdoor => "warped_trapdoor",
            Item::OakFenceGate => "oak_fence_gate",
            Item::SpruceFenceGate => "spruce_fence_gate",
            Item::BirchFenceGate => "birch_fence_gate",
            Item::JungleFenceGate => "jungle_fence_gate",
            Item::AcaciaFenceGate => "acacia_fence_gate",
            Item::DarkOakFenceGate => "dark_oak_fence_gate",
            Item::CrimsonFenceGate => "crimson_fence_gate",
            Item::WarpedFenceGate => "warped_fence_gate",
            Item::PoweredRail => "powered_rail",
            Item::DetectorRail => "detector_rail",
            Item::Rail => "rail",
            Item::ActivatorRail => "activator_rail",
            Item::Saddle => "saddle",
            Item::Minecart => "minecart",
            Item::ChestMinecart => "chest_minecart",
            Item::FurnaceMinecart => "furnace_minecart",
            Item::TntMinecart => "tnt_minecart",
            Item::HopperMinecart => "hopper_minecart",
            Item::CarrotOnAStick => "carrot_on_a_stick",
            Item::WarpedFungusOnAStick => "warped_fungus_on_a_stick",
            Item::Elytra => "elytra",
            Item::OakBoat => "oak_boat",
            Item::SpruceBoat => "spruce_boat",
            Item::BirchBoat => "birch_boat",
            Item::JungleBoat => "jungle_boat",
            Item::AcaciaBoat => "acacia_boat",
            Item::DarkOakBoat => "dark_oak_boat",
            Item::StructureBlock => "structure_block",
            Item::Jigsaw => "jigsaw",
            Item::TurtleHelmet => "turtle_helmet",
            Item::Scute => "scute",
            Item::FlintAndSteel => "flint_and_steel",
            Item::Apple => "apple",
            Item::Bow => "bow",
            Item::Arrow => "arrow",
            Item::Coal => "coal",
            Item::Charcoal => "charcoal",
            Item::Diamond => "diamond",
            Item::Emerald => "emerald",
            Item::LapisLazuli => "lapis_lazuli",
            Item::Quartz => "quartz",
            Item::AmethystShard => "amethyst_shard",
            Item::RawIron => "raw_iron",
            Item::IronIngot => "iron_ingot",
            Item::RawCopper => "raw_copper",
            Item::CopperIngot => "copper_ingot",
            Item::RawGold => "raw_gold",
            Item::GoldIngot => "gold_ingot",
            Item::NetheriteIngot => "netherite_ingot",
            Item::NetheriteScrap => "netherite_scrap",
            Item::WoodenSword => "wooden_sword",
            Item::WoodenShovel => "wooden_shovel",
            Item::WoodenPickaxe => "wooden_pickaxe",
            Item::WoodenAxe => "wooden_axe",
            Item::WoodenHoe => "wooden_hoe",
            Item::StoneSword => "stone_sword",
            Item::StoneShovel => "stone_shovel",
            Item::StonePickaxe => "stone_pickaxe",
            Item::StoneAxe => "stone_axe",
            Item::StoneHoe => "stone_hoe",
            Item::GoldenSword => "golden_sword",
            Item::GoldenShovel => "golden_shovel",
            Item::GoldenPickaxe => "golden_pickaxe",
            Item::GoldenAxe => "golden_axe",
            Item::GoldenHoe => "golden_hoe",
            Item::IronSword => "iron_sword",
            Item::IronShovel => "iron_shovel",
            Item::IronPickaxe => "iron_pickaxe",
            Item::IronAxe => "iron_axe",
            Item::IronHoe => "iron_hoe",
            Item::DiamondSword => "diamond_sword",
            Item::DiamondShovel => "diamond_shovel",
            Item::DiamondPickaxe => "diamond_pickaxe",
            Item::DiamondAxe => "diamond_axe",
            Item::DiamondHoe => "diamond_hoe",
            Item::NetheriteSword => "netherite_sword",
            Item::NetheriteShovel => "netherite_shovel",
            Item::NetheritePickaxe => "netherite_pickaxe",
            Item::NetheriteAxe => "netherite_axe",
            Item::NetheriteHoe => "netherite_hoe",
            Item::Stick => "stick",
            Item::Bowl => "bowl",
            Item::MushroomStew => "mushroom_stew",
            Item::String => "string",
            Item::Feather => "feather",
            Item::Gunpowder => "gunpowder",
            Item::WheatSeeds => "wheat_seeds",
            Item::Wheat => "wheat",
            Item::Bread => "bread",
            Item::LeatherHelmet => "leather_helmet",
            Item::LeatherChestplate => "leather_chestplate",
            Item::LeatherLeggings => "leather_leggings",
            Item::LeatherBoots => "leather_boots",
            Item::ChainmailHelmet => "chainmail_helmet",
            Item::ChainmailChestplate => "chainmail_chestplate",
            Item::ChainmailLeggings => "chainmail_leggings",
            Item::ChainmailBoots => "chainmail_boots",
            Item::IronHelmet => "iron_helmet",
            Item::IronChestplate => "iron_chestplate",
            Item::IronLeggings => "iron_leggings",
            Item::IronBoots => "iron_boots",
            Item::DiamondHelmet => "diamond_helmet",
            Item::DiamondChestplate => "diamond_chestplate",
            Item::DiamondLeggings => "diamond_leggings",
            Item::DiamondBoots => "diamond_boots",
            Item::GoldenHelmet => "golden_helmet",
            Item::GoldenChestplate => "golden_chestplate",
            Item::GoldenLeggings => "golden_leggings",
            Item::GoldenBoots => "golden_boots",
            Item::NetheriteHelmet => "netherite_helmet",
            Item::NetheriteChestplate => "netherite_chestplate",
            Item::NetheriteLeggings => "netherite_leggings",
            Item::NetheriteBoots => "netherite_boots",
            Item::Flint => "flint",
            Item::Porkchop => "porkchop",
            Item::CookedPorkchop => "cooked_porkchop",
            Item::Painting => "painting",
            Item::GoldenApple => "golden_apple",
            Item::EnchantedGoldenApple => "enchanted_golden_apple",
            Item::OakSign => "oak_sign",
            Item::SpruceSign => "spruce_sign",
            Item::BirchSign => "birch_sign",
            Item::JungleSign => "jungle_sign",
            Item::AcaciaSign => "acacia_sign",
            Item::DarkOakSign => "dark_oak_sign",
            Item::CrimsonSign => "crimson_sign",
            Item::WarpedSign => "warped_sign",
            Item::Bucket => "bucket",
            Item::WaterBucket => "water_bucket",
            Item::LavaBucket => "lava_bucket",
            Item::PowderSnowBucket => "powder_snow_bucket",
            Item::Snowball => "snowball",
            Item::Leather => "leather",
            Item::MilkBucket => "milk_bucket",
            Item::PufferfishBucket => "pufferfish_bucket",
            Item::SalmonBucket => "salmon_bucket",
            Item::CodBucket => "cod_bucket",
            Item::TropicalFishBucket => "tropical_fish_bucket",
            Item::AxolotlBucket => "axolotl_bucket",
            Item::Brick => "brick",
            Item::ClayBall => "clay_ball",
            Item::DriedKelpBlock => "dried_kelp_block",
            Item::Paper => "paper",
            Item::Book => "book",
            Item::SlimeBall => "slime_ball",
            Item::Egg => "egg",
            Item::Compass => "compass",
            Item::Bundle => "bundle",
            Item::FishingRod => "fishing_rod",
            Item::Clock => "clock",
            Item::Spyglass => "spyglass",
            Item::GlowstoneDust => "glowstone_dust",
            Item::Cod => "cod",
            Item::Salmon => "salmon",
            Item::TropicalFish => "tropical_fish",
            Item::Pufferfish => "pufferfish",
            Item::CookedCod => "cooked_cod",
            Item::CookedSalmon => "cooked_salmon",
            Item::InkSac => "ink_sac",
            Item::GlowInkSac => "glow_ink_sac",
            Item::CocoaBeans => "cocoa_beans",
            Item::WhiteDye => "white_dye",
            Item::OrangeDye => "orange_dye",
            Item::MagentaDye => "magenta_dye",
            Item::LightBlueDye => "light_blue_dye",
            Item::YellowDye => "yellow_dye",
            Item::LimeDye => "lime_dye",
            Item::PinkDye => "pink_dye",
            Item::GrayDye => "gray_dye",
            Item::LightGrayDye => "light_gray_dye",
            Item::CyanDye => "cyan_dye",
            Item::PurpleDye => "purple_dye",
            Item::BlueDye => "blue_dye",
            Item::BrownDye => "brown_dye",
            Item::GreenDye => "green_dye",
            Item::RedDye => "red_dye",
            Item::BlackDye => "black_dye",
            Item::BoneMeal => "bone_meal",
            Item::Bone => "bone",
            Item::Sugar => "sugar",
            Item::Cake => "cake",
            Item::WhiteBed => "white_bed",
            Item::OrangeBed => "orange_bed",
            Item::MagentaBed => "magenta_bed",
            Item::LightBlueBed => "light_blue_bed",
            Item::YellowBed => "yellow_bed",
            Item::LimeBed => "lime_bed",
            Item::PinkBed => "pink_bed",
            Item::GrayBed => "gray_bed",
            Item::LightGrayBed => "light_gray_bed",
            Item::CyanBed => "cyan_bed",
            Item::PurpleBed => "purple_bed",
            Item::BlueBed => "blue_bed",
            Item::BrownBed => "brown_bed",
            Item::GreenBed => "green_bed",
            Item::RedBed => "red_bed",
            Item::BlackBed => "black_bed",
            Item::Cookie => "cookie",
            Item::FilledMap => "filled_map",
            Item::Shears => "shears",
            Item::MelonSlice => "melon_slice",
            Item::DriedKelp => "dried_kelp",
            Item::PumpkinSeeds => "pumpkin_seeds",
            Item::MelonSeeds => "melon_seeds",
            Item::Beef => "beef",
            Item::CookedBeef => "cooked_beef",
            Item::Chicken => "chicken",
            Item::CookedChicken => "cooked_chicken",
            Item::RottenFlesh => "rotten_flesh",
            Item::EnderPearl => "ender_pearl",
            Item::BlazeRod => "blaze_rod",
            Item::GhastTear => "ghast_tear",
            Item::GoldNugget => "gold_nugget",
            Item::NetherWart => "nether_wart",
            Item::Potion => "potion",
            Item::GlassBottle => "glass_bottle",
            Item::SpiderEye => "spider_eye",
            Item::FermentedSpiderEye => "fermented_spider_eye",
            Item::BlazePowder => "blaze_powder",
            Item::MagmaCream => "magma_cream",
            Item::BrewingStand => "brewing_stand",
            Item::Cauldron => "cauldron",
            Item::EnderEye => "ender_eye",
            Item::GlisteringMelonSlice => "glistering_melon_slice",
            Item::AxolotlSpawnEgg => "axolotl_spawn_egg",
            Item::BatSpawnEgg => "bat_spawn_egg",
            Item::BeeSpawnEgg => "bee_spawn_egg",
            Item::BlazeSpawnEgg => "blaze_spawn_egg",
            Item::CatSpawnEgg => "cat_spawn_egg",
            Item::CaveSpiderSpawnEgg => "cave_spider_spawn_egg",
            Item::ChickenSpawnEgg => "chicken_spawn_egg",
            Item::CodSpawnEgg => "cod_spawn_egg",
            Item::CowSpawnEgg => "cow_spawn_egg",
            Item::CreeperSpawnEgg => "creeper_spawn_egg",
            Item::DolphinSpawnEgg => "dolphin_spawn_egg",
            Item::DonkeySpawnEgg => "donkey_spawn_egg",
            Item::DrownedSpawnEgg => "drowned_spawn_egg",
            Item::ElderGuardianSpawnEgg => "elder_guardian_spawn_egg",
            Item::EndermanSpawnEgg => "enderman_spawn_egg",
            Item::EndermiteSpawnEgg => "endermite_spawn_egg",
            Item::EvokerSpawnEgg => "evoker_spawn_egg",
            Item::FoxSpawnEgg => "fox_spawn_egg",
            Item::GhastSpawnEgg => "ghast_spawn_egg",
            Item::GlowSquidSpawnEgg => "glow_squid_spawn_egg",
            Item::GoatSpawnEgg => "goat_spawn_egg",
            Item::GuardianSpawnEgg => "guardian_spawn_egg",
            Item::HoglinSpawnEgg => "hoglin_spawn_egg",
            Item::HorseSpawnEgg => "horse_spawn_egg",
            Item::HuskSpawnEgg => "husk_spawn_egg",
            Item::LlamaSpawnEgg => "llama_spawn_egg",
            Item::MagmaCubeSpawnEgg => "magma_cube_spawn_egg",
            Item::MooshroomSpawnEgg => "mooshroom_spawn_egg",
            Item::MuleSpawnEgg => "mule_spawn_egg",
            Item::OcelotSpawnEgg => "ocelot_spawn_egg",
            Item::PandaSpawnEgg => "panda_spawn_egg",
            Item::ParrotSpawnEgg => "parrot_spawn_egg",
            Item::PhantomSpawnEgg => "phantom_spawn_egg",
            Item::PigSpawnEgg => "pig_spawn_egg",
            Item::PiglinSpawnEgg => "piglin_spawn_egg",
            Item::PiglinBruteSpawnEgg => "piglin_brute_spawn_egg",
            Item::PillagerSpawnEgg => "pillager_spawn_egg",
            Item::PolarBearSpawnEgg => "polar_bear_spawn_egg",
            Item::PufferfishSpawnEgg => "pufferfish_spawn_egg",
            Item::RabbitSpawnEgg => "rabbit_spawn_egg",
            Item::RavagerSpawnEgg => "ravager_spawn_egg",
            Item::SalmonSpawnEgg => "salmon_spawn_egg",
            Item::SheepSpawnEgg => "sheep_spawn_egg",
            Item::ShulkerSpawnEgg => "shulker_spawn_egg",
            Item::SilverfishSpawnEgg => "silverfish_spawn_egg",
            Item::SkeletonSpawnEgg => "skeleton_spawn_egg",
            Item::SkeletonHorseSpawnEgg => "skeleton_horse_spawn_egg",
            Item::SlimeSpawnEgg => "slime_spawn_egg",
            Item::SpiderSpawnEgg => "spider_spawn_egg",
            Item::SquidSpawnEgg => "squid_spawn_egg",
            Item::StraySpawnEgg => "stray_spawn_egg",
            Item::StriderSpawnEgg => "strider_spawn_egg",
            Item::TraderLlamaSpawnEgg => "trader_llama_spawn_egg",
            Item::TropicalFishSpawnEgg => "tropical_fish_spawn_egg",
            Item::TurtleSpawnEgg => "turtle_spawn_egg",
            Item::VexSpawnEgg => "vex_spawn_egg",
            Item::VillagerSpawnEgg => "villager_spawn_egg",
            Item::VindicatorSpawnEgg => "vindicator_spawn_egg",
            Item::WanderingTraderSpawnEgg => "wandering_trader_spawn_egg",
            Item::WitchSpawnEgg => "witch_spawn_egg",
            Item::WitherSkeletonSpawnEgg => "wither_skeleton_spawn_egg",
            Item::WolfSpawnEgg => "wolf_spawn_egg",
            Item::ZoglinSpawnEgg => "zoglin_spawn_egg",
            Item::ZombieSpawnEgg => "zombie_spawn_egg",
            Item::ZombieHorseSpawnEgg => "zombie_horse_spawn_egg",
            Item::ZombieVillagerSpawnEgg => "zombie_villager_spawn_egg",
            Item::ZombifiedPiglinSpawnEgg => "zombified_piglin_spawn_egg",
            Item::ExperienceBottle => "experience_bottle",
            Item::FireCharge => "fire_charge",
            Item::WritableBook => "writable_book",
            Item::WrittenBook => "written_book",
            Item::ItemFrame => "item_frame",
            Item::GlowItemFrame => "glow_item_frame",
            Item::FlowerPot => "flower_pot",
            Item::Carrot => "carrot",
            Item::Potato => "potato",
            Item::BakedPotato => "baked_potato",
            Item::PoisonousPotato => "poisonous_potato",
            Item::Map => "map",
            Item::GoldenCarrot => "golden_carrot",
            Item::SkeletonSkull => "skeleton_skull",
            Item::WitherSkeletonSkull => "wither_skeleton_skull",
            Item::PlayerHead => "player_head",
            Item::ZombieHead => "zombie_head",
            Item::CreeperHead => "creeper_head",
            Item::DragonHead => "dragon_head",
            Item::NetherStar => "nether_star",
            Item::PumpkinPie => "pumpkin_pie",
            Item::FireworkRocket => "firework_rocket",
            Item::FireworkStar => "firework_star",
            Item::EnchantedBook => "enchanted_book",
            Item::NetherBrick => "nether_brick",
            Item::PrismarineShard => "prismarine_shard",
            Item::PrismarineCrystals => "prismarine_crystals",
            Item::Rabbit => "rabbit",
            Item::CookedRabbit => "cooked_rabbit",
            Item::RabbitStew => "rabbit_stew",
            Item::RabbitFoot => "rabbit_foot",
            Item::RabbitHide => "rabbit_hide",
            Item::ArmorStand => "armor_stand",
            Item::IronHorseArmor => "iron_horse_armor",
            Item::GoldenHorseArmor => "golden_horse_armor",
            Item::DiamondHorseArmor => "diamond_horse_armor",
            Item::LeatherHorseArmor => "leather_horse_armor",
            Item::Lead => "lead",
            Item::NameTag => "name_tag",
            Item::CommandBlockMinecart => "command_block_minecart",
            Item::Mutton => "mutton",
            Item::CookedMutton => "cooked_mutton",
            Item::WhiteBanner => "white_banner",
            Item::OrangeBanner => "orange_banner",
            Item::MagentaBanner => "magenta_banner",
            Item::LightBlueBanner => "light_blue_banner",
            Item::YellowBanner => "yellow_banner",
            Item::LimeBanner => "lime_banner",
            Item::PinkBanner => "pink_banner",
            Item::GrayBanner => "gray_banner",
            Item::LightGrayBanner => "light_gray_banner",
            Item::CyanBanner => "cyan_banner",
            Item::PurpleBanner => "purple_banner",
            Item::BlueBanner => "blue_banner",
            Item::BrownBanner => "brown_banner",
            Item::GreenBanner => "green_banner",
            Item::RedBanner => "red_banner",
            Item::BlackBanner => "black_banner",
            Item::EndCrystal => "end_crystal",
            Item::ChorusFruit => "chorus_fruit",
            Item::PoppedChorusFruit => "popped_chorus_fruit",
            Item::Beetroot => "beetroot",
            Item::BeetrootSeeds => "beetroot_seeds",
            Item::BeetrootSoup => "beetroot_soup",
            Item::DragonBreath => "dragon_breath",
            Item::SplashPotion => "splash_potion",
            Item::SpectralArrow => "spectral_arrow",
            Item::TippedArrow => "tipped_arrow",
            Item::LingeringPotion => "lingering_potion",
            Item::Shield => "shield",
            Item::TotemOfUndying => "totem_of_undying",
            Item::ShulkerShell => "shulker_shell",
            Item::IronNugget => "iron_nugget",
            Item::KnowledgeBook => "knowledge_book",
            Item::DebugStick => "debug_stick",
            Item::MusicDisc13 => "music_disc_13",
            Item::MusicDiscCat => "music_disc_cat",
            Item::MusicDiscBlocks => "music_disc_blocks",
            Item::MusicDiscChirp => "music_disc_chirp",
            Item::MusicDiscFar => "music_disc_far",
            Item::MusicDiscMall => "music_disc_mall",
            Item::MusicDiscMellohi => "music_disc_mellohi",
            Item::MusicDiscStal => "music_disc_stal",
            Item::MusicDiscStrad => "music_disc_strad",
            Item::MusicDiscWard => "music_disc_ward",
            Item::MusicDisc11 => "music_disc_11",
            Item::MusicDiscWait => "music_disc_wait",
            Item::MusicDiscOtherside => "music_disc_otherside",
            Item::MusicDiscPigstep => "music_disc_pigstep",
            Item::Trident => "trident",
            Item::PhantomMembrane => "phantom_membrane",
            Item::NautilusShell => "nautilus_shell",
            Item::HeartOfTheSea => "heart_of_the_sea",
            Item::Crossbow => "crossbow",
            Item::SuspiciousStew => "suspicious_stew",
            Item::Loom => "loom",
            Item::FlowerBannerPattern => "flower_banner_pattern",
            Item::CreeperBannerPattern => "creeper_banner_pattern",
            Item::SkullBannerPattern => "skull_banner_pattern",
            Item::MojangBannerPattern => "mojang_banner_pattern",
            Item::GlobeBannerPattern => "globe_banner_pattern",
            Item::PiglinBannerPattern => "piglin_banner_pattern",
            Item::Composter => "composter",
            Item::Barrel => "barrel",
            Item::Smoker => "smoker",
            Item::BlastFurnace => "blast_furnace",
            Item::CartographyTable => "cartography_table",
            Item::FletchingTable => "fletching_table",
            Item::Grindstone => "grindstone",
            Item::SmithingTable => "smithing_table",
            Item::Stonecutter => "stonecutter",
            Item::Bell => "bell",
            Item::Lantern => "lantern",
            Item::SoulLantern => "soul_lantern",
            Item::SweetBerries => "sweet_berries",
            Item::GlowBerries => "glow_berries",
            Item::Campfire => "campfire",
            Item::SoulCampfire => "soul_campfire",
            Item::Shroomlight => "shroomlight",
            Item::Honeycomb => "honeycomb",
            Item::BeeNest => "bee_nest",
            Item::Beehive => "beehive",
            Item::HoneyBottle => "honey_bottle",
            Item::HoneycombBlock => "honeycomb_block",
            Item::Lodestone => "lodestone",
            Item::CryingObsidian => "crying_obsidian",
            Item::Blackstone => "blackstone",
            Item::BlackstoneSlab => "blackstone_slab",
            Item::BlackstoneStairs => "blackstone_stairs",
            Item::GildedBlackstone => "gilded_blackstone",
            Item::PolishedBlackstone => "polished_blackstone",
            Item::PolishedBlackstoneSlab => "polished_blackstone_slab",
            Item::PolishedBlackstoneStairs => "polished_blackstone_stairs",
            Item::ChiseledPolishedBlackstone => "chiseled_polished_blackstone",
            Item::PolishedBlackstoneBricks => "polished_blackstone_bricks",
            Item::PolishedBlackstoneBrickSlab => "polished_blackstone_brick_slab",
            Item::PolishedBlackstoneBrickStairs => "polished_blackstone_brick_stairs",
            Item::CrackedPolishedBlackstoneBricks => "cracked_polished_blackstone_bricks",
            Item::RespawnAnchor => "respawn_anchor",
            Item::Candle => "candle",
            Item::WhiteCandle => "white_candle",
            Item::OrangeCandle => "orange_candle",
            Item::MagentaCandle => "magenta_candle",
            Item::LightBlueCandle => "light_blue_candle",
            Item::YellowCandle => "yellow_candle",
            Item::LimeCandle => "lime_candle",
            Item::PinkCandle => "pink_candle",
            Item::GrayCandle => "gray_candle",
            Item::LightGrayCandle => "light_gray_candle",
            Item::CyanCandle => "cyan_candle",
            Item::PurpleCandle => "purple_candle",
            Item::BlueCandle => "blue_candle",
            Item::BrownCandle => "brown_candle",
            Item::GreenCandle => "green_candle",
            Item::RedCandle => "red_candle",
            Item::BlackCandle => "black_candle",
            Item::SmallAmethystBud => "small_amethyst_bud",
            Item::MediumAmethystBud => "medium_amethyst_bud",
            Item::LargeAmethystBud => "large_amethyst_bud",
            Item::AmethystCluster => "amethyst_cluster",
            Item::PointedDripstone => "pointed_dripstone",
        }
    }
    #[doc = "Gets a `Item` by its `name`."]
    #[inline]
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "stone" => Some(Item::Stone),
            "granite" => Some(Item::Granite),
            "polished_granite" => Some(Item::PolishedGranite),
            "diorite" => Some(Item::Diorite),
            "polished_diorite" => Some(Item::PolishedDiorite),
            "andesite" => Some(Item::Andesite),
            "polished_andesite" => Some(Item::PolishedAndesite),
            "deepslate" => Some(Item::Deepslate),
            "cobbled_deepslate" => Some(Item::CobbledDeepslate),
            "polished_deepslate" => Some(Item::PolishedDeepslate),
            "calcite" => Some(Item::Calcite),
            "tuff" => Some(Item::Tuff),
            "dripstone_block" => Some(Item::DripstoneBlock),
            "grass_block" => Some(Item::GrassBlock),
            "dirt" => Some(Item::Dirt),
            "coarse_dirt" => Some(Item::CoarseDirt),
            "podzol" => Some(Item::Podzol),
            "rooted_dirt" => Some(Item::RootedDirt),
            "crimson_nylium" => Some(Item::CrimsonNylium),
            "warped_nylium" => Some(Item::WarpedNylium),
            "cobblestone" => Some(Item::Cobblestone),
            "oak_planks" => Some(Item::OakPlanks),
            "spruce_planks" => Some(Item::SprucePlanks),
            "birch_planks" => Some(Item::BirchPlanks),
            "jungle_planks" => Some(Item::JunglePlanks),
            "acacia_planks" => Some(Item::AcaciaPlanks),
            "dark_oak_planks" => Some(Item::DarkOakPlanks),
            "crimson_planks" => Some(Item::CrimsonPlanks),
            "warped_planks" => Some(Item::WarpedPlanks),
            "oak_sapling" => Some(Item::OakSapling),
            "spruce_sapling" => Some(Item::SpruceSapling),
            "birch_sapling" => Some(Item::BirchSapling),
            "jungle_sapling" => Some(Item::JungleSapling),
            "acacia_sapling" => Some(Item::AcaciaSapling),
            "dark_oak_sapling" => Some(Item::DarkOakSapling),
            "bedrock" => Some(Item::Bedrock),
            "sand" => Some(Item::Sand),
            "red_sand" => Some(Item::RedSand),
            "gravel" => Some(Item::Gravel),
            "coal_ore" => Some(Item::CoalOre),
            "deepslate_coal_ore" => Some(Item::DeepslateCoalOre),
            "iron_ore" => Some(Item::IronOre),
            "deepslate_iron_ore" => Some(Item::DeepslateIronOre),
            "copper_ore" => Some(Item::CopperOre),
            "deepslate_copper_ore" => Some(Item::DeepslateCopperOre),
            "gold_ore" => Some(Item::GoldOre),
            "deepslate_gold_ore" => Some(Item::DeepslateGoldOre),
            "redstone_ore" => Some(Item::RedstoneOre),
            "deepslate_redstone_ore" => Some(Item::DeepslateRedstoneOre),
            "emerald_ore" => Some(Item::EmeraldOre),
            "deepslate_emerald_ore" => Some(Item::DeepslateEmeraldOre),
            "lapis_ore" => Some(Item::LapisOre),
            "deepslate_lapis_ore" => Some(Item::DeepslateLapisOre),
            "diamond_ore" => Some(Item::DiamondOre),
            "deepslate_diamond_ore" => Some(Item::DeepslateDiamondOre),
            "nether_gold_ore" => Some(Item::NetherGoldOre),
            "nether_quartz_ore" => Some(Item::NetherQuartzOre),
            "ancient_debris" => Some(Item::AncientDebris),
            "coal_block" => Some(Item::CoalBlock),
            "raw_iron_block" => Some(Item::RawIronBlock),
            "raw_copper_block" => Some(Item::RawCopperBlock),
            "raw_gold_block" => Some(Item::RawGoldBlock),
            "amethyst_block" => Some(Item::AmethystBlock),
            "budding_amethyst" => Some(Item::BuddingAmethyst),
            "iron_block" => Some(Item::IronBlock),
            "copper_block" => Some(Item::CopperBlock),
            "gold_block" => Some(Item::GoldBlock),
            "diamond_block" => Some(Item::DiamondBlock),
            "netherite_block" => Some(Item::NetheriteBlock),
            "exposed_copper" => Some(Item::ExposedCopper),
            "weathered_copper" => Some(Item::WeatheredCopper),
            "oxidized_copper" => Some(Item::OxidizedCopper),
            "cut_copper" => Some(Item::CutCopper),
            "exposed_cut_copper" => Some(Item::ExposedCutCopper),
            "weathered_cut_copper" => Some(Item::WeatheredCutCopper),
            "oxidized_cut_copper" => Some(Item::OxidizedCutCopper),
            "cut_copper_stairs" => Some(Item::CutCopperStairs),
            "exposed_cut_copper_stairs" => Some(Item::ExposedCutCopperStairs),
            "weathered_cut_copper_stairs" => Some(Item::WeatheredCutCopperStairs),
            "oxidized_cut_copper_stairs" => Some(Item::OxidizedCutCopperStairs),
            "cut_copper_slab" => Some(Item::CutCopperSlab),
            "exposed_cut_copper_slab" => Some(Item::ExposedCutCopperSlab),
            "weathered_cut_copper_slab" => Some(Item::WeatheredCutCopperSlab),
            "oxidized_cut_copper_slab" => Some(Item::OxidizedCutCopperSlab),
            "waxed_copper_block" => Some(Item::WaxedCopperBlock),
            "waxed_exposed_copper" => Some(Item::WaxedExposedCopper),
            "waxed_weathered_copper" => Some(Item::WaxedWeatheredCopper),
            "waxed_oxidized_copper" => Some(Item::WaxedOxidizedCopper),
            "waxed_cut_copper" => Some(Item::WaxedCutCopper),
            "waxed_exposed_cut_copper" => Some(Item::WaxedExposedCutCopper),
            "waxed_weathered_cut_copper" => Some(Item::WaxedWeatheredCutCopper),
            "waxed_oxidized_cut_copper" => Some(Item::WaxedOxidizedCutCopper),
            "waxed_cut_copper_stairs" => Some(Item::WaxedCutCopperStairs),
            "waxed_exposed_cut_copper_stairs" => Some(Item::WaxedExposedCutCopperStairs),
            "waxed_weathered_cut_copper_stairs" => Some(Item::WaxedWeatheredCutCopperStairs),
            "waxed_oxidized_cut_copper_stairs" => Some(Item::WaxedOxidizedCutCopperStairs),
            "waxed_cut_copper_slab" => Some(Item::WaxedCutCopperSlab),
            "waxed_exposed_cut_copper_slab" => Some(Item::WaxedExposedCutCopperSlab),
            "waxed_weathered_cut_copper_slab" => Some(Item::WaxedWeatheredCutCopperSlab),
            "waxed_oxidized_cut_copper_slab" => Some(Item::WaxedOxidizedCutCopperSlab),
            "oak_log" => Some(Item::OakLog),
            "spruce_log" => Some(Item::SpruceLog),
            "birch_log" => Some(Item::BirchLog),
            "jungle_log" => Some(Item::JungleLog),
            "acacia_log" => Some(Item::AcaciaLog),
            "dark_oak_log" => Some(Item::DarkOakLog),
            "crimson_stem" => Some(Item::CrimsonStem),
            "warped_stem" => Some(Item::WarpedStem),
            "stripped_oak_log" => Some(Item::StrippedOakLog),
            "stripped_spruce_log" => Some(Item::StrippedSpruceLog),
            "stripped_birch_log" => Some(Item::StrippedBirchLog),
            "stripped_jungle_log" => Some(Item::StrippedJungleLog),
            "stripped_acacia_log" => Some(Item::StrippedAcaciaLog),
            "stripped_dark_oak_log" => Some(Item::StrippedDarkOakLog),
            "stripped_crimson_stem" => Some(Item::StrippedCrimsonStem),
            "stripped_warped_stem" => Some(Item::StrippedWarpedStem),
            "stripped_oak_wood" => Some(Item::StrippedOakWood),
            "stripped_spruce_wood" => Some(Item::StrippedSpruceWood),
            "stripped_birch_wood" => Some(Item::StrippedBirchWood),
            "stripped_jungle_wood" => Some(Item::StrippedJungleWood),
            "stripped_acacia_wood" => Some(Item::StrippedAcaciaWood),
            "stripped_dark_oak_wood" => Some(Item::StrippedDarkOakWood),
            "stripped_crimson_hyphae" => Some(Item::StrippedCrimsonHyphae),
            "stripped_warped_hyphae" => Some(Item::StrippedWarpedHyphae),
            "oak_wood" => Some(Item::OakWood),
            "spruce_wood" => Some(Item::SpruceWood),
            "birch_wood" => Some(Item::BirchWood),
            "jungle_wood" => Some(Item::JungleWood),
            "acacia_wood" => Some(Item::AcaciaWood),
            "dark_oak_wood" => Some(Item::DarkOakWood),
            "crimson_hyphae" => Some(Item::CrimsonHyphae),
            "warped_hyphae" => Some(Item::WarpedHyphae),
            "oak_leaves" => Some(Item::OakLeaves),
            "spruce_leaves" => Some(Item::SpruceLeaves),
            "birch_leaves" => Some(Item::BirchLeaves),
            "jungle_leaves" => Some(Item::JungleLeaves),
            "acacia_leaves" => Some(Item::AcaciaLeaves),
            "dark_oak_leaves" => Some(Item::DarkOakLeaves),
            "azalea_leaves" => Some(Item::AzaleaLeaves),
            "flowering_azalea_leaves" => Some(Item::FloweringAzaleaLeaves),
            "sponge" => Some(Item::Sponge),
            "wet_sponge" => Some(Item::WetSponge),
            "glass" => Some(Item::Glass),
            "tinted_glass" => Some(Item::TintedGlass),
            "lapis_block" => Some(Item::LapisBlock),
            "sandstone" => Some(Item::Sandstone),
            "chiseled_sandstone" => Some(Item::ChiseledSandstone),
            "cut_sandstone" => Some(Item::CutSandstone),
            "cobweb" => Some(Item::Cobweb),
            "grass" => Some(Item::Grass),
            "fern" => Some(Item::Fern),
            "azalea" => Some(Item::Azalea),
            "flowering_azalea" => Some(Item::FloweringAzalea),
            "dead_bush" => Some(Item::DeadBush),
            "seagrass" => Some(Item::Seagrass),
            "sea_pickle" => Some(Item::SeaPickle),
            "white_wool" => Some(Item::WhiteWool),
            "orange_wool" => Some(Item::OrangeWool),
            "magenta_wool" => Some(Item::MagentaWool),
            "light_blue_wool" => Some(Item::LightBlueWool),
            "yellow_wool" => Some(Item::YellowWool),
            "lime_wool" => Some(Item::LimeWool),
            "pink_wool" => Some(Item::PinkWool),
            "gray_wool" => Some(Item::GrayWool),
            "light_gray_wool" => Some(Item::LightGrayWool),
            "cyan_wool" => Some(Item::CyanWool),
            "purple_wool" => Some(Item::PurpleWool),
            "blue_wool" => Some(Item::BlueWool),
            "brown_wool" => Some(Item::BrownWool),
            "green_wool" => Some(Item::GreenWool),
            "red_wool" => Some(Item::RedWool),
            "black_wool" => Some(Item::BlackWool),
            "dandelion" => Some(Item::Dandelion),
            "poppy" => Some(Item::Poppy),
            "blue_orchid" => Some(Item::BlueOrchid),
            "allium" => Some(Item::Allium),
            "azure_bluet" => Some(Item::AzureBluet),
            "red_tulip" => Some(Item::RedTulip),
            "orange_tulip" => Some(Item::OrangeTulip),
            "white_tulip" => Some(Item::WhiteTulip),
            "pink_tulip" => Some(Item::PinkTulip),
            "oxeye_daisy" => Some(Item::OxeyeDaisy),
            "cornflower" => Some(Item::Cornflower),
            "lily_of_the_valley" => Some(Item::LilyOfTheValley),
            "wither_rose" => Some(Item::WitherRose),
            "spore_blossom" => Some(Item::SporeBlossom),
            "brown_mushroom" => Some(Item::BrownMushroom),
            "red_mushroom" => Some(Item::RedMushroom),
            "crimson_fungus" => Some(Item::CrimsonFungus),
            "warped_fungus" => Some(Item::WarpedFungus),
            "crimson_roots" => Some(Item::CrimsonRoots),
            "warped_roots" => Some(Item::WarpedRoots),
            "nether_sprouts" => Some(Item::NetherSprouts),
            "weeping_vines" => Some(Item::WeepingVines),
            "twisting_vines" => Some(Item::TwistingVines),
            "sugar_cane" => Some(Item::SugarCane),
            "kelp" => Some(Item::Kelp),
            "moss_carpet" => Some(Item::MossCarpet),
            "moss_block" => Some(Item::MossBlock),
            "hanging_roots" => Some(Item::HangingRoots),
            "big_dripleaf" => Some(Item::BigDripleaf),
            "small_dripleaf" => Some(Item::SmallDripleaf),
            "bamboo" => Some(Item::Bamboo),
            "oak_slab" => Some(Item::OakSlab),
            "spruce_slab" => Some(Item::SpruceSlab),
            "birch_slab" => Some(Item::BirchSlab),
            "jungle_slab" => Some(Item::JungleSlab),
            "acacia_slab" => Some(Item::AcaciaSlab),
            "dark_oak_slab" => Some(Item::DarkOakSlab),
            "crimson_slab" => Some(Item::CrimsonSlab),
            "warped_slab" => Some(Item::WarpedSlab),
            "stone_slab" => Some(Item::StoneSlab),
            "smooth_stone_slab" => Some(Item::SmoothStoneSlab),
            "sandstone_slab" => Some(Item::SandstoneSlab),
            "cut_sandstone_slab" => Some(Item::CutSandstoneSlab),
            "petrified_oak_slab" => Some(Item::PetrifiedOakSlab),
            "cobblestone_slab" => Some(Item::CobblestoneSlab),
            "brick_slab" => Some(Item::BrickSlab),
            "stone_brick_slab" => Some(Item::StoneBrickSlab),
            "nether_brick_slab" => Some(Item::NetherBrickSlab),
            "quartz_slab" => Some(Item::QuartzSlab),
            "red_sandstone_slab" => Some(Item::RedSandstoneSlab),
            "cut_red_sandstone_slab" => Some(Item::CutRedSandstoneSlab),
            "purpur_slab" => Some(Item::PurpurSlab),
            "prismarine_slab" => Some(Item::PrismarineSlab),
            "prismarine_brick_slab" => Some(Item::PrismarineBrickSlab),
            "dark_prismarine_slab" => Some(Item::DarkPrismarineSlab),
            "smooth_quartz" => Some(Item::SmoothQuartz),
            "smooth_red_sandstone" => Some(Item::SmoothRedSandstone),
            "smooth_sandstone" => Some(Item::SmoothSandstone),
            "smooth_stone" => Some(Item::SmoothStone),
            "bricks" => Some(Item::Bricks),
            "bookshelf" => Some(Item::Bookshelf),
            "mossy_cobblestone" => Some(Item::MossyCobblestone),
            "obsidian" => Some(Item::Obsidian),
            "torch" => Some(Item::Torch),
            "end_rod" => Some(Item::EndRod),
            "chorus_plant" => Some(Item::ChorusPlant),
            "chorus_flower" => Some(Item::ChorusFlower),
            "purpur_block" => Some(Item::PurpurBlock),
            "purpur_pillar" => Some(Item::PurpurPillar),
            "purpur_stairs" => Some(Item::PurpurStairs),
            "spawner" => Some(Item::Spawner),
            "oak_stairs" => Some(Item::OakStairs),
            "chest" => Some(Item::Chest),
            "crafting_table" => Some(Item::CraftingTable),
            "farmland" => Some(Item::Farmland),
            "furnace" => Some(Item::Furnace),
            "ladder" => Some(Item::Ladder),
            "cobblestone_stairs" => Some(Item::CobblestoneStairs),
            "snow" => Some(Item::Snow),
            "ice" => Some(Item::Ice),
            "snow_block" => Some(Item::SnowBlock),
            "cactus" => Some(Item::Cactus),
            "clay" => Some(Item::Clay),
            "jukebox" => Some(Item::Jukebox),
            "oak_fence" => Some(Item::OakFence),
            "spruce_fence" => Some(Item::SpruceFence),
            "birch_fence" => Some(Item::BirchFence),
            "jungle_fence" => Some(Item::JungleFence),
            "acacia_fence" => Some(Item::AcaciaFence),
            "dark_oak_fence" => Some(Item::DarkOakFence),
            "crimson_fence" => Some(Item::CrimsonFence),
            "warped_fence" => Some(Item::WarpedFence),
            "pumpkin" => Some(Item::Pumpkin),
            "carved_pumpkin" => Some(Item::CarvedPumpkin),
            "jack_o_lantern" => Some(Item::JackOLantern),
            "netherrack" => Some(Item::Netherrack),
            "soul_sand" => Some(Item::SoulSand),
            "soul_soil" => Some(Item::SoulSoil),
            "basalt" => Some(Item::Basalt),
            "polished_basalt" => Some(Item::PolishedBasalt),
            "smooth_basalt" => Some(Item::SmoothBasalt),
            "soul_torch" => Some(Item::SoulTorch),
            "glowstone" => Some(Item::Glowstone),
            "infested_stone" => Some(Item::InfestedStone),
            "infested_cobblestone" => Some(Item::InfestedCobblestone),
            "infested_stone_bricks" => Some(Item::InfestedStoneBricks),
            "infested_mossy_stone_bricks" => Some(Item::InfestedMossyStoneBricks),
            "infested_cracked_stone_bricks" => Some(Item::InfestedCrackedStoneBricks),
            "infested_chiseled_stone_bricks" => Some(Item::InfestedChiseledStoneBricks),
            "infested_deepslate" => Some(Item::InfestedDeepslate),
            "stone_bricks" => Some(Item::StoneBricks),
            "mossy_stone_bricks" => Some(Item::MossyStoneBricks),
            "cracked_stone_bricks" => Some(Item::CrackedStoneBricks),
            "chiseled_stone_bricks" => Some(Item::ChiseledStoneBricks),
            "deepslate_bricks" => Some(Item::DeepslateBricks),
            "cracked_deepslate_bricks" => Some(Item::CrackedDeepslateBricks),
            "deepslate_tiles" => Some(Item::DeepslateTiles),
            "cracked_deepslate_tiles" => Some(Item::CrackedDeepslateTiles),
            "chiseled_deepslate" => Some(Item::ChiseledDeepslate),
            "brown_mushroom_block" => Some(Item::BrownMushroomBlock),
            "red_mushroom_block" => Some(Item::RedMushroomBlock),
            "mushroom_stem" => Some(Item::MushroomStem),
            "iron_bars" => Some(Item::IronBars),
            "chain" => Some(Item::Chain),
            "glass_pane" => Some(Item::GlassPane),
            "melon" => Some(Item::Melon),
            "vine" => Some(Item::Vine),
            "glow_lichen" => Some(Item::GlowLichen),
            "brick_stairs" => Some(Item::BrickStairs),
            "stone_brick_stairs" => Some(Item::StoneBrickStairs),
            "mycelium" => Some(Item::Mycelium),
            "lily_pad" => Some(Item::LilyPad),
            "nether_bricks" => Some(Item::NetherBricks),
            "cracked_nether_bricks" => Some(Item::CrackedNetherBricks),
            "chiseled_nether_bricks" => Some(Item::ChiseledNetherBricks),
            "nether_brick_fence" => Some(Item::NetherBrickFence),
            "nether_brick_stairs" => Some(Item::NetherBrickStairs),
            "enchanting_table" => Some(Item::EnchantingTable),
            "end_portal_frame" => Some(Item::EndPortalFrame),
            "end_stone" => Some(Item::EndStone),
            "end_stone_bricks" => Some(Item::EndStoneBricks),
            "dragon_egg" => Some(Item::DragonEgg),
            "sandstone_stairs" => Some(Item::SandstoneStairs),
            "ender_chest" => Some(Item::EnderChest),
            "emerald_block" => Some(Item::EmeraldBlock),
            "spruce_stairs" => Some(Item::SpruceStairs),
            "birch_stairs" => Some(Item::BirchStairs),
            "jungle_stairs" => Some(Item::JungleStairs),
            "crimson_stairs" => Some(Item::CrimsonStairs),
            "warped_stairs" => Some(Item::WarpedStairs),
            "command_block" => Some(Item::CommandBlock),
            "beacon" => Some(Item::Beacon),
            "cobblestone_wall" => Some(Item::CobblestoneWall),
            "mossy_cobblestone_wall" => Some(Item::MossyCobblestoneWall),
            "brick_wall" => Some(Item::BrickWall),
            "prismarine_wall" => Some(Item::PrismarineWall),
            "red_sandstone_wall" => Some(Item::RedSandstoneWall),
            "mossy_stone_brick_wall" => Some(Item::MossyStoneBrickWall),
            "granite_wall" => Some(Item::GraniteWall),
            "stone_brick_wall" => Some(Item::StoneBrickWall),
            "nether_brick_wall" => Some(Item::NetherBrickWall),
            "andesite_wall" => Some(Item::AndesiteWall),
            "red_nether_brick_wall" => Some(Item::RedNetherBrickWall),
            "sandstone_wall" => Some(Item::SandstoneWall),
            "end_stone_brick_wall" => Some(Item::EndStoneBrickWall),
            "diorite_wall" => Some(Item::DioriteWall),
            "blackstone_wall" => Some(Item::BlackstoneWall),
            "polished_blackstone_wall" => Some(Item::PolishedBlackstoneWall),
            "polished_blackstone_brick_wall" => Some(Item::PolishedBlackstoneBrickWall),
            "cobbled_deepslate_wall" => Some(Item::CobbledDeepslateWall),
            "polished_deepslate_wall" => Some(Item::PolishedDeepslateWall),
            "deepslate_brick_wall" => Some(Item::DeepslateBrickWall),
            "deepslate_tile_wall" => Some(Item::DeepslateTileWall),
            "anvil" => Some(Item::Anvil),
            "chipped_anvil" => Some(Item::ChippedAnvil),
            "damaged_anvil" => Some(Item::DamagedAnvil),
            "chiseled_quartz_block" => Some(Item::ChiseledQuartzBlock),
            "quartz_block" => Some(Item::QuartzBlock),
            "quartz_bricks" => Some(Item::QuartzBricks),
            "quartz_pillar" => Some(Item::QuartzPillar),
            "quartz_stairs" => Some(Item::QuartzStairs),
            "white_terracotta" => Some(Item::WhiteTerracotta),
            "orange_terracotta" => Some(Item::OrangeTerracotta),
            "magenta_terracotta" => Some(Item::MagentaTerracotta),
            "light_blue_terracotta" => Some(Item::LightBlueTerracotta),
            "yellow_terracotta" => Some(Item::YellowTerracotta),
            "lime_terracotta" => Some(Item::LimeTerracotta),
            "pink_terracotta" => Some(Item::PinkTerracotta),
            "gray_terracotta" => Some(Item::GrayTerracotta),
            "light_gray_terracotta" => Some(Item::LightGrayTerracotta),
            "cyan_terracotta" => Some(Item::CyanTerracotta),
            "purple_terracotta" => Some(Item::PurpleTerracotta),
            "blue_terracotta" => Some(Item::BlueTerracotta),
            "brown_terracotta" => Some(Item::BrownTerracotta),
            "green_terracotta" => Some(Item::GreenTerracotta),
            "red_terracotta" => Some(Item::RedTerracotta),
            "black_terracotta" => Some(Item::BlackTerracotta),
            "barrier" => Some(Item::Barrier),
            "light" => Some(Item::Light),
            "hay_block" => Some(Item::HayBlock),
            "white_carpet" => Some(Item::WhiteCarpet),
            "orange_carpet" => Some(Item::OrangeCarpet),
            "magenta_carpet" => Some(Item::MagentaCarpet),
            "light_blue_carpet" => Some(Item::LightBlueCarpet),
            "yellow_carpet" => Some(Item::YellowCarpet),
            "lime_carpet" => Some(Item::LimeCarpet),
            "pink_carpet" => Some(Item::PinkCarpet),
            "gray_carpet" => Some(Item::GrayCarpet),
            "light_gray_carpet" => Some(Item::LightGrayCarpet),
            "cyan_carpet" => Some(Item::CyanCarpet),
            "purple_carpet" => Some(Item::PurpleCarpet),
            "blue_carpet" => Some(Item::BlueCarpet),
            "brown_carpet" => Some(Item::BrownCarpet),
            "green_carpet" => Some(Item::GreenCarpet),
            "red_carpet" => Some(Item::RedCarpet),
            "black_carpet" => Some(Item::BlackCarpet),
            "terracotta" => Some(Item::Terracotta),
            "packed_ice" => Some(Item::PackedIce),
            "acacia_stairs" => Some(Item::AcaciaStairs),
            "dark_oak_stairs" => Some(Item::DarkOakStairs),
            "dirt_path" => Some(Item::DirtPath),
            "sunflower" => Some(Item::Sunflower),
            "lilac" => Some(Item::Lilac),
            "rose_bush" => Some(Item::RoseBush),
            "peony" => Some(Item::Peony),
            "tall_grass" => Some(Item::TallGrass),
            "large_fern" => Some(Item::LargeFern),
            "white_stained_glass" => Some(Item::WhiteStainedGlass),
            "orange_stained_glass" => Some(Item::OrangeStainedGlass),
            "magenta_stained_glass" => Some(Item::MagentaStainedGlass),
            "light_blue_stained_glass" => Some(Item::LightBlueStainedGlass),
            "yellow_stained_glass" => Some(Item::YellowStainedGlass),
            "lime_stained_glass" => Some(Item::LimeStainedGlass),
            "pink_stained_glass" => Some(Item::PinkStainedGlass),
            "gray_stained_glass" => Some(Item::GrayStainedGlass),
            "light_gray_stained_glass" => Some(Item::LightGrayStainedGlass),
            "cyan_stained_glass" => Some(Item::CyanStainedGlass),
            "purple_stained_glass" => Some(Item::PurpleStainedGlass),
            "blue_stained_glass" => Some(Item::BlueStainedGlass),
            "brown_stained_glass" => Some(Item::BrownStainedGlass),
            "green_stained_glass" => Some(Item::GreenStainedGlass),
            "red_stained_glass" => Some(Item::RedStainedGlass),
            "black_stained_glass" => Some(Item::BlackStainedGlass),
            "white_stained_glass_pane" => Some(Item::WhiteStainedGlassPane),
            "orange_stained_glass_pane" => Some(Item::OrangeStainedGlassPane),
            "magenta_stained_glass_pane" => Some(Item::MagentaStainedGlassPane),
            "light_blue_stained_glass_pane" => Some(Item::LightBlueStainedGlassPane),
            "yellow_stained_glass_pane" => Some(Item::YellowStainedGlassPane),
            "lime_stained_glass_pane" => Some(Item::LimeStainedGlassPane),
            "pink_stained_glass_pane" => Some(Item::PinkStainedGlassPane),
            "gray_stained_glass_pane" => Some(Item::GrayStainedGlassPane),
            "light_gray_stained_glass_pane" => Some(Item::LightGrayStainedGlassPane),
            "cyan_stained_glass_pane" => Some(Item::CyanStainedGlassPane),
            "purple_stained_glass_pane" => Some(Item::PurpleStainedGlassPane),
            "blue_stained_glass_pane" => Some(Item::BlueStainedGlassPane),
            "brown_stained_glass_pane" => Some(Item::BrownStainedGlassPane),
            "green_stained_glass_pane" => Some(Item::GreenStainedGlassPane),
            "red_stained_glass_pane" => Some(Item::RedStainedGlassPane),
            "black_stained_glass_pane" => Some(Item::BlackStainedGlassPane),
            "prismarine" => Some(Item::Prismarine),
            "prismarine_bricks" => Some(Item::PrismarineBricks),
            "dark_prismarine" => Some(Item::DarkPrismarine),
            "prismarine_stairs" => Some(Item::PrismarineStairs),
            "prismarine_brick_stairs" => Some(Item::PrismarineBrickStairs),
            "dark_prismarine_stairs" => Some(Item::DarkPrismarineStairs),
            "sea_lantern" => Some(Item::SeaLantern),
            "red_sandstone" => Some(Item::RedSandstone),
            "chiseled_red_sandstone" => Some(Item::ChiseledRedSandstone),
            "cut_red_sandstone" => Some(Item::CutRedSandstone),
            "red_sandstone_stairs" => Some(Item::RedSandstoneStairs),
            "repeating_command_block" => Some(Item::RepeatingCommandBlock),
            "chain_command_block" => Some(Item::ChainCommandBlock),
            "magma_block" => Some(Item::MagmaBlock),
            "nether_wart_block" => Some(Item::NetherWartBlock),
            "warped_wart_block" => Some(Item::WarpedWartBlock),
            "red_nether_bricks" => Some(Item::RedNetherBricks),
            "bone_block" => Some(Item::BoneBlock),
            "structure_void" => Some(Item::StructureVoid),
            "shulker_box" => Some(Item::ShulkerBox),
            "white_shulker_box" => Some(Item::WhiteShulkerBox),
            "orange_shulker_box" => Some(Item::OrangeShulkerBox),
            "magenta_shulker_box" => Some(Item::MagentaShulkerBox),
            "light_blue_shulker_box" => Some(Item::LightBlueShulkerBox),
            "yellow_shulker_box" => Some(Item::YellowShulkerBox),
            "lime_shulker_box" => Some(Item::LimeShulkerBox),
            "pink_shulker_box" => Some(Item::PinkShulkerBox),
            "gray_shulker_box" => Some(Item::GrayShulkerBox),
            "light_gray_shulker_box" => Some(Item::LightGrayShulkerBox),
            "cyan_shulker_box" => Some(Item::CyanShulkerBox),
            "purple_shulker_box" => Some(Item::PurpleShulkerBox),
            "blue_shulker_box" => Some(Item::BlueShulkerBox),
            "brown_shulker_box" => Some(Item::BrownShulkerBox),
            "green_shulker_box" => Some(Item::GreenShulkerBox),
            "red_shulker_box" => Some(Item::RedShulkerBox),
            "black_shulker_box" => Some(Item::BlackShulkerBox),
            "white_glazed_terracotta" => Some(Item::WhiteGlazedTerracotta),
            "orange_glazed_terracotta" => Some(Item::OrangeGlazedTerracotta),
            "magenta_glazed_terracotta" => Some(Item::MagentaGlazedTerracotta),
            "light_blue_glazed_terracotta" => Some(Item::LightBlueGlazedTerracotta),
            "yellow_glazed_terracotta" => Some(Item::YellowGlazedTerracotta),
            "lime_glazed_terracotta" => Some(Item::LimeGlazedTerracotta),
            "pink_glazed_terracotta" => Some(Item::PinkGlazedTerracotta),
            "gray_glazed_terracotta" => Some(Item::GrayGlazedTerracotta),
            "light_gray_glazed_terracotta" => Some(Item::LightGrayGlazedTerracotta),
            "cyan_glazed_terracotta" => Some(Item::CyanGlazedTerracotta),
            "purple_glazed_terracotta" => Some(Item::PurpleGlazedTerracotta),
            "blue_glazed_terracotta" => Some(Item::BlueGlazedTerracotta),
            "brown_glazed_terracotta" => Some(Item::BrownGlazedTerracotta),
            "green_glazed_terracotta" => Some(Item::GreenGlazedTerracotta),
            "red_glazed_terracotta" => Some(Item::RedGlazedTerracotta),
            "black_glazed_terracotta" => Some(Item::BlackGlazedTerracotta),
            "white_concrete" => Some(Item::WhiteConcrete),
            "orange_concrete" => Some(Item::OrangeConcrete),
            "magenta_concrete" => Some(Item::MagentaConcrete),
            "light_blue_concrete" => Some(Item::LightBlueConcrete),
            "yellow_concrete" => Some(Item::YellowConcrete),
            "lime_concrete" => Some(Item::LimeConcrete),
            "pink_concrete" => Some(Item::PinkConcrete),
            "gray_concrete" => Some(Item::GrayConcrete),
            "light_gray_concrete" => Some(Item::LightGrayConcrete),
            "cyan_concrete" => Some(Item::CyanConcrete),
            "purple_concrete" => Some(Item::PurpleConcrete),
            "blue_concrete" => Some(Item::BlueConcrete),
            "brown_concrete" => Some(Item::BrownConcrete),
            "green_concrete" => Some(Item::GreenConcrete),
            "red_concrete" => Some(Item::RedConcrete),
            "black_concrete" => Some(Item::BlackConcrete),
            "white_concrete_powder" => Some(Item::WhiteConcretePowder),
            "orange_concrete_powder" => Some(Item::OrangeConcretePowder),
            "magenta_concrete_powder" => Some(Item::MagentaConcretePowder),
            "light_blue_concrete_powder" => Some(Item::LightBlueConcretePowder),
            "yellow_concrete_powder" => Some(Item::YellowConcretePowder),
            "lime_concrete_powder" => Some(Item::LimeConcretePowder),
            "pink_concrete_powder" => Some(Item::PinkConcretePowder),
            "gray_concrete_powder" => Some(Item::GrayConcretePowder),
            "light_gray_concrete_powder" => Some(Item::LightGrayConcretePowder),
            "cyan_concrete_powder" => Some(Item::CyanConcretePowder),
            "purple_concrete_powder" => Some(Item::PurpleConcretePowder),
            "blue_concrete_powder" => Some(Item::BlueConcretePowder),
            "brown_concrete_powder" => Some(Item::BrownConcretePowder),
            "green_concrete_powder" => Some(Item::GreenConcretePowder),
            "red_concrete_powder" => Some(Item::RedConcretePowder),
            "black_concrete_powder" => Some(Item::BlackConcretePowder),
            "turtle_egg" => Some(Item::TurtleEgg),
            "dead_tube_coral_block" => Some(Item::DeadTubeCoralBlock),
            "dead_brain_coral_block" => Some(Item::DeadBrainCoralBlock),
            "dead_bubble_coral_block" => Some(Item::DeadBubbleCoralBlock),
            "dead_fire_coral_block" => Some(Item::DeadFireCoralBlock),
            "dead_horn_coral_block" => Some(Item::DeadHornCoralBlock),
            "tube_coral_block" => Some(Item::TubeCoralBlock),
            "brain_coral_block" => Some(Item::BrainCoralBlock),
            "bubble_coral_block" => Some(Item::BubbleCoralBlock),
            "fire_coral_block" => Some(Item::FireCoralBlock),
            "horn_coral_block" => Some(Item::HornCoralBlock),
            "tube_coral" => Some(Item::TubeCoral),
            "brain_coral" => Some(Item::BrainCoral),
            "bubble_coral" => Some(Item::BubbleCoral),
            "fire_coral" => Some(Item::FireCoral),
            "horn_coral" => Some(Item::HornCoral),
            "dead_brain_coral" => Some(Item::DeadBrainCoral),
            "dead_bubble_coral" => Some(Item::DeadBubbleCoral),
            "dead_fire_coral" => Some(Item::DeadFireCoral),
            "dead_horn_coral" => Some(Item::DeadHornCoral),
            "dead_tube_coral" => Some(Item::DeadTubeCoral),
            "tube_coral_fan" => Some(Item::TubeCoralFan),
            "brain_coral_fan" => Some(Item::BrainCoralFan),
            "bubble_coral_fan" => Some(Item::BubbleCoralFan),
            "fire_coral_fan" => Some(Item::FireCoralFan),
            "horn_coral_fan" => Some(Item::HornCoralFan),
            "dead_tube_coral_fan" => Some(Item::DeadTubeCoralFan),
            "dead_brain_coral_fan" => Some(Item::DeadBrainCoralFan),
            "dead_bubble_coral_fan" => Some(Item::DeadBubbleCoralFan),
            "dead_fire_coral_fan" => Some(Item::DeadFireCoralFan),
            "dead_horn_coral_fan" => Some(Item::DeadHornCoralFan),
            "blue_ice" => Some(Item::BlueIce),
            "conduit" => Some(Item::Conduit),
            "polished_granite_stairs" => Some(Item::PolishedGraniteStairs),
            "smooth_red_sandstone_stairs" => Some(Item::SmoothRedSandstoneStairs),
            "mossy_stone_brick_stairs" => Some(Item::MossyStoneBrickStairs),
            "polished_diorite_stairs" => Some(Item::PolishedDioriteStairs),
            "mossy_cobblestone_stairs" => Some(Item::MossyCobblestoneStairs),
            "end_stone_brick_stairs" => Some(Item::EndStoneBrickStairs),
            "stone_stairs" => Some(Item::StoneStairs),
            "smooth_sandstone_stairs" => Some(Item::SmoothSandstoneStairs),
            "smooth_quartz_stairs" => Some(Item::SmoothQuartzStairs),
            "granite_stairs" => Some(Item::GraniteStairs),
            "andesite_stairs" => Some(Item::AndesiteStairs),
            "red_nether_brick_stairs" => Some(Item::RedNetherBrickStairs),
            "polished_andesite_stairs" => Some(Item::PolishedAndesiteStairs),
            "diorite_stairs" => Some(Item::DioriteStairs),
            "cobbled_deepslate_stairs" => Some(Item::CobbledDeepslateStairs),
            "polished_deepslate_stairs" => Some(Item::PolishedDeepslateStairs),
            "deepslate_brick_stairs" => Some(Item::DeepslateBrickStairs),
            "deepslate_tile_stairs" => Some(Item::DeepslateTileStairs),
            "polished_granite_slab" => Some(Item::PolishedGraniteSlab),
            "smooth_red_sandstone_slab" => Some(Item::SmoothRedSandstoneSlab),
            "mossy_stone_brick_slab" => Some(Item::MossyStoneBrickSlab),
            "polished_diorite_slab" => Some(Item::PolishedDioriteSlab),
            "mossy_cobblestone_slab" => Some(Item::MossyCobblestoneSlab),
            "end_stone_brick_slab" => Some(Item::EndStoneBrickSlab),
            "smooth_sandstone_slab" => Some(Item::SmoothSandstoneSlab),
            "smooth_quartz_slab" => Some(Item::SmoothQuartzSlab),
            "granite_slab" => Some(Item::GraniteSlab),
            "andesite_slab" => Some(Item::AndesiteSlab),
            "red_nether_brick_slab" => Some(Item::RedNetherBrickSlab),
            "polished_andesite_slab" => Some(Item::PolishedAndesiteSlab),
            "diorite_slab" => Some(Item::DioriteSlab),
            "cobbled_deepslate_slab" => Some(Item::CobbledDeepslateSlab),
            "polished_deepslate_slab" => Some(Item::PolishedDeepslateSlab),
            "deepslate_brick_slab" => Some(Item::DeepslateBrickSlab),
            "deepslate_tile_slab" => Some(Item::DeepslateTileSlab),
            "scaffolding" => Some(Item::Scaffolding),
            "redstone" => Some(Item::Redstone),
            "redstone_torch" => Some(Item::RedstoneTorch),
            "redstone_block" => Some(Item::RedstoneBlock),
            "repeater" => Some(Item::Repeater),
            "comparator" => Some(Item::Comparator),
            "piston" => Some(Item::Piston),
            "sticky_piston" => Some(Item::StickyPiston),
            "slime_block" => Some(Item::SlimeBlock),
            "honey_block" => Some(Item::HoneyBlock),
            "observer" => Some(Item::Observer),
            "hopper" => Some(Item::Hopper),
            "dispenser" => Some(Item::Dispenser),
            "dropper" => Some(Item::Dropper),
            "lectern" => Some(Item::Lectern),
            "target" => Some(Item::Target),
            "lever" => Some(Item::Lever),
            "lightning_rod" => Some(Item::LightningRod),
            "daylight_detector" => Some(Item::DaylightDetector),
            "sculk_sensor" => Some(Item::SculkSensor),
            "tripwire_hook" => Some(Item::TripwireHook),
            "trapped_chest" => Some(Item::TrappedChest),
            "tnt" => Some(Item::Tnt),
            "redstone_lamp" => Some(Item::RedstoneLamp),
            "note_block" => Some(Item::NoteBlock),
            "stone_button" => Some(Item::StoneButton),
            "polished_blackstone_button" => Some(Item::PolishedBlackstoneButton),
            "oak_button" => Some(Item::OakButton),
            "spruce_button" => Some(Item::SpruceButton),
            "birch_button" => Some(Item::BirchButton),
            "jungle_button" => Some(Item::JungleButton),
            "acacia_button" => Some(Item::AcaciaButton),
            "dark_oak_button" => Some(Item::DarkOakButton),
            "crimson_button" => Some(Item::CrimsonButton),
            "warped_button" => Some(Item::WarpedButton),
            "stone_pressure_plate" => Some(Item::StonePressurePlate),
            "polished_blackstone_pressure_plate" => Some(Item::PolishedBlackstonePressurePlate),
            "light_weighted_pressure_plate" => Some(Item::LightWeightedPressurePlate),
            "heavy_weighted_pressure_plate" => Some(Item::HeavyWeightedPressurePlate),
            "oak_pressure_plate" => Some(Item::OakPressurePlate),
            "spruce_pressure_plate" => Some(Item::SprucePressurePlate),
            "birch_pressure_plate" => Some(Item::BirchPressurePlate),
            "jungle_pressure_plate" => Some(Item::JunglePressurePlate),
            "acacia_pressure_plate" => Some(Item::AcaciaPressurePlate),
            "dark_oak_pressure_plate" => Some(Item::DarkOakPressurePlate),
            "crimson_pressure_plate" => Some(Item::CrimsonPressurePlate),
            "warped_pressure_plate" => Some(Item::WarpedPressurePlate),
            "iron_door" => Some(Item::IronDoor),
            "oak_door" => Some(Item::OakDoor),
            "spruce_door" => Some(Item::SpruceDoor),
            "birch_door" => Some(Item::BirchDoor),
            "jungle_door" => Some(Item::JungleDoor),
            "acacia_door" => Some(Item::AcaciaDoor),
            "dark_oak_door" => Some(Item::DarkOakDoor),
            "crimson_door" => Some(Item::CrimsonDoor),
            "warped_door" => Some(Item::WarpedDoor),
            "iron_trapdoor" => Some(Item::IronTrapdoor),
            "oak_trapdoor" => Some(Item::OakTrapdoor),
            "spruce_trapdoor" => Some(Item::SpruceTrapdoor),
            "birch_trapdoor" => Some(Item::BirchTrapdoor),
            "jungle_trapdoor" => Some(Item::JungleTrapdoor),
            "acacia_trapdoor" => Some(Item::AcaciaTrapdoor),
            "dark_oak_trapdoor" => Some(Item::DarkOakTrapdoor),
            "crimson_trapdoor" => Some(Item::CrimsonTrapdoor),
            "warped_trapdoor" => Some(Item::WarpedTrapdoor),
            "oak_fence_gate" => Some(Item::OakFenceGate),
            "spruce_fence_gate" => Some(Item::SpruceFenceGate),
            "birch_fence_gate" => Some(Item::BirchFenceGate),
            "jungle_fence_gate" => Some(Item::JungleFenceGate),
            "acacia_fence_gate" => Some(Item::AcaciaFenceGate),
            "dark_oak_fence_gate" => Some(Item::DarkOakFenceGate),
            "crimson_fence_gate" => Some(Item::CrimsonFenceGate),
            "warped_fence_gate" => Some(Item::WarpedFenceGate),
            "powered_rail" => Some(Item::PoweredRail),
            "detector_rail" => Some(Item::DetectorRail),
            "rail" => Some(Item::Rail),
            "activator_rail" => Some(Item::ActivatorRail),
            "saddle" => Some(Item::Saddle),
            "minecart" => Some(Item::Minecart),
            "chest_minecart" => Some(Item::ChestMinecart),
            "furnace_minecart" => Some(Item::FurnaceMinecart),
            "tnt_minecart" => Some(Item::TntMinecart),
            "hopper_minecart" => Some(Item::HopperMinecart),
            "carrot_on_a_stick" => Some(Item::CarrotOnAStick),
            "warped_fungus_on_a_stick" => Some(Item::WarpedFungusOnAStick),
            "elytra" => Some(Item::Elytra),
            "oak_boat" => Some(Item::OakBoat),
            "spruce_boat" => Some(Item::SpruceBoat),
            "birch_boat" => Some(Item::BirchBoat),
            "jungle_boat" => Some(Item::JungleBoat),
            "acacia_boat" => Some(Item::AcaciaBoat),
            "dark_oak_boat" => Some(Item::DarkOakBoat),
            "structure_block" => Some(Item::StructureBlock),
            "jigsaw" => Some(Item::Jigsaw),
            "turtle_helmet" => Some(Item::TurtleHelmet),
            "scute" => Some(Item::Scute),
            "flint_and_steel" => Some(Item::FlintAndSteel),
            "apple" => Some(Item::Apple),
            "bow" => Some(Item::Bow),
            "arrow" => Some(Item::Arrow),
            "coal" => Some(Item::Coal),
            "charcoal" => Some(Item::Charcoal),
            "diamond" => Some(Item::Diamond),
            "emerald" => Some(Item::Emerald),
            "lapis_lazuli" => Some(Item::LapisLazuli),
            "quartz" => Some(Item::Quartz),
            "amethyst_shard" => Some(Item::AmethystShard),
            "raw_iron" => Some(Item::RawIron),
            "iron_ingot" => Some(Item::IronIngot),
            "raw_copper" => Some(Item::RawCopper),
            "copper_ingot" => Some(Item::CopperIngot),
            "raw_gold" => Some(Item::RawGold),
            "gold_ingot" => Some(Item::GoldIngot),
            "netherite_ingot" => Some(Item::NetheriteIngot),
            "netherite_scrap" => Some(Item::NetheriteScrap),
            "wooden_sword" => Some(Item::WoodenSword),
            "wooden_shovel" => Some(Item::WoodenShovel),
            "wooden_pickaxe" => Some(Item::WoodenPickaxe),
            "wooden_axe" => Some(Item::WoodenAxe),
            "wooden_hoe" => Some(Item::WoodenHoe),
            "stone_sword" => Some(Item::StoneSword),
            "stone_shovel" => Some(Item::StoneShovel),
            "stone_pickaxe" => Some(Item::StonePickaxe),
            "stone_axe" => Some(Item::StoneAxe),
            "stone_hoe" => Some(Item::StoneHoe),
            "golden_sword" => Some(Item::GoldenSword),
            "golden_shovel" => Some(Item::GoldenShovel),
            "golden_pickaxe" => Some(Item::GoldenPickaxe),
            "golden_axe" => Some(Item::GoldenAxe),
            "golden_hoe" => Some(Item::GoldenHoe),
            "iron_sword" => Some(Item::IronSword),
            "iron_shovel" => Some(Item::IronShovel),
            "iron_pickaxe" => Some(Item::IronPickaxe),
            "iron_axe" => Some(Item::IronAxe),
            "iron_hoe" => Some(Item::IronHoe),
            "diamond_sword" => Some(Item::DiamondSword),
            "diamond_shovel" => Some(Item::DiamondShovel),
            "diamond_pickaxe" => Some(Item::DiamondPickaxe),
            "diamond_axe" => Some(Item::DiamondAxe),
            "diamond_hoe" => Some(Item::DiamondHoe),
            "netherite_sword" => Some(Item::NetheriteSword),
            "netherite_shovel" => Some(Item::NetheriteShovel),
            "netherite_pickaxe" => Some(Item::NetheritePickaxe),
            "netherite_axe" => Some(Item::NetheriteAxe),
            "netherite_hoe" => Some(Item::NetheriteHoe),
            "stick" => Some(Item::Stick),
            "bowl" => Some(Item::Bowl),
            "mushroom_stew" => Some(Item::MushroomStew),
            "string" => Some(Item::String),
            "feather" => Some(Item::Feather),
            "gunpowder" => Some(Item::Gunpowder),
            "wheat_seeds" => Some(Item::WheatSeeds),
            "wheat" => Some(Item::Wheat),
            "bread" => Some(Item::Bread),
            "leather_helmet" => Some(Item::LeatherHelmet),
            "leather_chestplate" => Some(Item::LeatherChestplate),
            "leather_leggings" => Some(Item::LeatherLeggings),
            "leather_boots" => Some(Item::LeatherBoots),
            "chainmail_helmet" => Some(Item::ChainmailHelmet),
            "chainmail_chestplate" => Some(Item::ChainmailChestplate),
            "chainmail_leggings" => Some(Item::ChainmailLeggings),
            "chainmail_boots" => Some(Item::ChainmailBoots),
            "iron_helmet" => Some(Item::IronHelmet),
            "iron_chestplate" => Some(Item::IronChestplate),
            "iron_leggings" => Some(Item::IronLeggings),
            "iron_boots" => Some(Item::IronBoots),
            "diamond_helmet" => Some(Item::DiamondHelmet),
            "diamond_chestplate" => Some(Item::DiamondChestplate),
            "diamond_leggings" => Some(Item::DiamondLeggings),
            "diamond_boots" => Some(Item::DiamondBoots),
            "golden_helmet" => Some(Item::GoldenHelmet),
            "golden_chestplate" => Some(Item::GoldenChestplate),
            "golden_leggings" => Some(Item::GoldenLeggings),
            "golden_boots" => Some(Item::GoldenBoots),
            "netherite_helmet" => Some(Item::NetheriteHelmet),
            "netherite_chestplate" => Some(Item::NetheriteChestplate),
            "netherite_leggings" => Some(Item::NetheriteLeggings),
            "netherite_boots" => Some(Item::NetheriteBoots),
            "flint" => Some(Item::Flint),
            "porkchop" => Some(Item::Porkchop),
            "cooked_porkchop" => Some(Item::CookedPorkchop),
            "painting" => Some(Item::Painting),
            "golden_apple" => Some(Item::GoldenApple),
            "enchanted_golden_apple" => Some(Item::EnchantedGoldenApple),
            "oak_sign" => Some(Item::OakSign),
            "spruce_sign" => Some(Item::SpruceSign),
            "birch_sign" => Some(Item::BirchSign),
            "jungle_sign" => Some(Item::JungleSign),
            "acacia_sign" => Some(Item::AcaciaSign),
            "dark_oak_sign" => Some(Item::DarkOakSign),
            "crimson_sign" => Some(Item::CrimsonSign),
            "warped_sign" => Some(Item::WarpedSign),
            "bucket" => Some(Item::Bucket),
            "water_bucket" => Some(Item::WaterBucket),
            "lava_bucket" => Some(Item::LavaBucket),
            "powder_snow_bucket" => Some(Item::PowderSnowBucket),
            "snowball" => Some(Item::Snowball),
            "leather" => Some(Item::Leather),
            "milk_bucket" => Some(Item::MilkBucket),
            "pufferfish_bucket" => Some(Item::PufferfishBucket),
            "salmon_bucket" => Some(Item::SalmonBucket),
            "cod_bucket" => Some(Item::CodBucket),
            "tropical_fish_bucket" => Some(Item::TropicalFishBucket),
            "axolotl_bucket" => Some(Item::AxolotlBucket),
            "brick" => Some(Item::Brick),
            "clay_ball" => Some(Item::ClayBall),
            "dried_kelp_block" => Some(Item::DriedKelpBlock),
            "paper" => Some(Item::Paper),
            "book" => Some(Item::Book),
            "slime_ball" => Some(Item::SlimeBall),
            "egg" => Some(Item::Egg),
            "compass" => Some(Item::Compass),
            "bundle" => Some(Item::Bundle),
            "fishing_rod" => Some(Item::FishingRod),
            "clock" => Some(Item::Clock),
            "spyglass" => Some(Item::Spyglass),
            "glowstone_dust" => Some(Item::GlowstoneDust),
            "cod" => Some(Item::Cod),
            "salmon" => Some(Item::Salmon),
            "tropical_fish" => Some(Item::TropicalFish),
            "pufferfish" => Some(Item::Pufferfish),
            "cooked_cod" => Some(Item::CookedCod),
            "cooked_salmon" => Some(Item::CookedSalmon),
            "ink_sac" => Some(Item::InkSac),
            "glow_ink_sac" => Some(Item::GlowInkSac),
            "cocoa_beans" => Some(Item::CocoaBeans),
            "white_dye" => Some(Item::WhiteDye),
            "orange_dye" => Some(Item::OrangeDye),
            "magenta_dye" => Some(Item::MagentaDye),
            "light_blue_dye" => Some(Item::LightBlueDye),
            "yellow_dye" => Some(Item::YellowDye),
            "lime_dye" => Some(Item::LimeDye),
            "pink_dye" => Some(Item::PinkDye),
            "gray_dye" => Some(Item::GrayDye),
            "light_gray_dye" => Some(Item::LightGrayDye),
            "cyan_dye" => Some(Item::CyanDye),
            "purple_dye" => Some(Item::PurpleDye),
            "blue_dye" => Some(Item::BlueDye),
            "brown_dye" => Some(Item::BrownDye),
            "green_dye" => Some(Item::GreenDye),
            "red_dye" => Some(Item::RedDye),
            "black_dye" => Some(Item::BlackDye),
            "bone_meal" => Some(Item::BoneMeal),
            "bone" => Some(Item::Bone),
            "sugar" => Some(Item::Sugar),
            "cake" => Some(Item::Cake),
            "white_bed" => Some(Item::WhiteBed),
            "orange_bed" => Some(Item::OrangeBed),
            "magenta_bed" => Some(Item::MagentaBed),
            "light_blue_bed" => Some(Item::LightBlueBed),
            "yellow_bed" => Some(Item::YellowBed),
            "lime_bed" => Some(Item::LimeBed),
            "pink_bed" => Some(Item::PinkBed),
            "gray_bed" => Some(Item::GrayBed),
            "light_gray_bed" => Some(Item::LightGrayBed),
            "cyan_bed" => Some(Item::CyanBed),
            "purple_bed" => Some(Item::PurpleBed),
            "blue_bed" => Some(Item::BlueBed),
            "brown_bed" => Some(Item::BrownBed),
            "green_bed" => Some(Item::GreenBed),
            "red_bed" => Some(Item::RedBed),
            "black_bed" => Some(Item::BlackBed),
            "cookie" => Some(Item::Cookie),
            "filled_map" => Some(Item::FilledMap),
            "shears" => Some(Item::Shears),
            "melon_slice" => Some(Item::MelonSlice),
            "dried_kelp" => Some(Item::DriedKelp),
            "pumpkin_seeds" => Some(Item::PumpkinSeeds),
            "melon_seeds" => Some(Item::MelonSeeds),
            "beef" => Some(Item::Beef),
            "cooked_beef" => Some(Item::CookedBeef),
            "chicken" => Some(Item::Chicken),
            "cooked_chicken" => Some(Item::CookedChicken),
            "rotten_flesh" => Some(Item::RottenFlesh),
            "ender_pearl" => Some(Item::EnderPearl),
            "blaze_rod" => Some(Item::BlazeRod),
            "ghast_tear" => Some(Item::GhastTear),
            "gold_nugget" => Some(Item::GoldNugget),
            "nether_wart" => Some(Item::NetherWart),
            "potion" => Some(Item::Potion),
            "glass_bottle" => Some(Item::GlassBottle),
            "spider_eye" => Some(Item::SpiderEye),
            "fermented_spider_eye" => Some(Item::FermentedSpiderEye),
            "blaze_powder" => Some(Item::BlazePowder),
            "magma_cream" => Some(Item::MagmaCream),
            "brewing_stand" => Some(Item::BrewingStand),
            "cauldron" => Some(Item::Cauldron),
            "ender_eye" => Some(Item::EnderEye),
            "glistering_melon_slice" => Some(Item::GlisteringMelonSlice),
            "axolotl_spawn_egg" => Some(Item::AxolotlSpawnEgg),
            "bat_spawn_egg" => Some(Item::BatSpawnEgg),
            "bee_spawn_egg" => Some(Item::BeeSpawnEgg),
            "blaze_spawn_egg" => Some(Item::BlazeSpawnEgg),
            "cat_spawn_egg" => Some(Item::CatSpawnEgg),
            "cave_spider_spawn_egg" => Some(Item::CaveSpiderSpawnEgg),
            "chicken_spawn_egg" => Some(Item::ChickenSpawnEgg),
            "cod_spawn_egg" => Some(Item::CodSpawnEgg),
            "cow_spawn_egg" => Some(Item::CowSpawnEgg),
            "creeper_spawn_egg" => Some(Item::CreeperSpawnEgg),
            "dolphin_spawn_egg" => Some(Item::DolphinSpawnEgg),
            "donkey_spawn_egg" => Some(Item::DonkeySpawnEgg),
            "drowned_spawn_egg" => Some(Item::DrownedSpawnEgg),
            "elder_guardian_spawn_egg" => Some(Item::ElderGuardianSpawnEgg),
            "enderman_spawn_egg" => Some(Item::EndermanSpawnEgg),
            "endermite_spawn_egg" => Some(Item::EndermiteSpawnEgg),
            "evoker_spawn_egg" => Some(Item::EvokerSpawnEgg),
            "fox_spawn_egg" => Some(Item::FoxSpawnEgg),
            "ghast_spawn_egg" => Some(Item::GhastSpawnEgg),
            "glow_squid_spawn_egg" => Some(Item::GlowSquidSpawnEgg),
            "goat_spawn_egg" => Some(Item::GoatSpawnEgg),
            "guardian_spawn_egg" => Some(Item::GuardianSpawnEgg),
            "hoglin_spawn_egg" => Some(Item::HoglinSpawnEgg),
            "horse_spawn_egg" => Some(Item::HorseSpawnEgg),
            "husk_spawn_egg" => Some(Item::HuskSpawnEgg),
            "llama_spawn_egg" => Some(Item::LlamaSpawnEgg),
            "magma_cube_spawn_egg" => Some(Item::MagmaCubeSpawnEgg),
            "mooshroom_spawn_egg" => Some(Item::MooshroomSpawnEgg),
            "mule_spawn_egg" => Some(Item::MuleSpawnEgg),
            "ocelot_spawn_egg" => Some(Item::OcelotSpawnEgg),
            "panda_spawn_egg" => Some(Item::PandaSpawnEgg),
            "parrot_spawn_egg" => Some(Item::ParrotSpawnEgg),
            "phantom_spawn_egg" => Some(Item::PhantomSpawnEgg),
            "pig_spawn_egg" => Some(Item::PigSpawnEgg),
            "piglin_spawn_egg" => Some(Item::PiglinSpawnEgg),
            "piglin_brute_spawn_egg" => Some(Item::PiglinBruteSpawnEgg),
            "pillager_spawn_egg" => Some(Item::PillagerSpawnEgg),
            "polar_bear_spawn_egg" => Some(Item::PolarBearSpawnEgg),
            "pufferfish_spawn_egg" => Some(Item::PufferfishSpawnEgg),
            "rabbit_spawn_egg" => Some(Item::RabbitSpawnEgg),
            "ravager_spawn_egg" => Some(Item::RavagerSpawnEgg),
            "salmon_spawn_egg" => Some(Item::SalmonSpawnEgg),
            "sheep_spawn_egg" => Some(Item::SheepSpawnEgg),
            "shulker_spawn_egg" => Some(Item::ShulkerSpawnEgg),
            "silverfish_spawn_egg" => Some(Item::SilverfishSpawnEgg),
            "skeleton_spawn_egg" => Some(Item::SkeletonSpawnEgg),
            "skeleton_horse_spawn_egg" => Some(Item::SkeletonHorseSpawnEgg),
            "slime_spawn_egg" => Some(Item::SlimeSpawnEgg),
            "spider_spawn_egg" => Some(Item::SpiderSpawnEgg),
            "squid_spawn_egg" => Some(Item::SquidSpawnEgg),
            "stray_spawn_egg" => Some(Item::StraySpawnEgg),
            "strider_spawn_egg" => Some(Item::StriderSpawnEgg),
            "trader_llama_spawn_egg" => Some(Item::TraderLlamaSpawnEgg),
            "tropical_fish_spawn_egg" => Some(Item::TropicalFishSpawnEgg),
            "turtle_spawn_egg" => Some(Item::TurtleSpawnEgg),
            "vex_spawn_egg" => Some(Item::VexSpawnEgg),
            "villager_spawn_egg" => Some(Item::VillagerSpawnEgg),
            "vindicator_spawn_egg" => Some(Item::VindicatorSpawnEgg),
            "wandering_trader_spawn_egg" => Some(Item::WanderingTraderSpawnEgg),
            "witch_spawn_egg" => Some(Item::WitchSpawnEgg),
            "wither_skeleton_spawn_egg" => Some(Item::WitherSkeletonSpawnEgg),
            "wolf_spawn_egg" => Some(Item::WolfSpawnEgg),
            "zoglin_spawn_egg" => Some(Item::ZoglinSpawnEgg),
            "zombie_spawn_egg" => Some(Item::ZombieSpawnEgg),
            "zombie_horse_spawn_egg" => Some(Item::ZombieHorseSpawnEgg),
            "zombie_villager_spawn_egg" => Some(Item::ZombieVillagerSpawnEgg),
            "zombified_piglin_spawn_egg" => Some(Item::ZombifiedPiglinSpawnEgg),
            "experience_bottle" => Some(Item::ExperienceBottle),
            "fire_charge" => Some(Item::FireCharge),
            "writable_book" => Some(Item::WritableBook),
            "written_book" => Some(Item::WrittenBook),
            "item_frame" => Some(Item::ItemFrame),
            "glow_item_frame" => Some(Item::GlowItemFrame),
            "flower_pot" => Some(Item::FlowerPot),
            "carrot" => Some(Item::Carrot),
            "potato" => Some(Item::Potato),
            "baked_potato" => Some(Item::BakedPotato),
            "poisonous_potato" => Some(Item::PoisonousPotato),
            "map" => Some(Item::Map),
            "golden_carrot" => Some(Item::GoldenCarrot),
            "skeleton_skull" => Some(Item::SkeletonSkull),
            "wither_skeleton_skull" => Some(Item::WitherSkeletonSkull),
            "player_head" => Some(Item::PlayerHead),
            "zombie_head" => Some(Item::ZombieHead),
            "creeper_head" => Some(Item::CreeperHead),
            "dragon_head" => Some(Item::DragonHead),
            "nether_star" => Some(Item::NetherStar),
            "pumpkin_pie" => Some(Item::PumpkinPie),
            "firework_rocket" => Some(Item::FireworkRocket),
            "firework_star" => Some(Item::FireworkStar),
            "enchanted_book" => Some(Item::EnchantedBook),
            "nether_brick" => Some(Item::NetherBrick),
            "prismarine_shard" => Some(Item::PrismarineShard),
            "prismarine_crystals" => Some(Item::PrismarineCrystals),
            "rabbit" => Some(Item::Rabbit),
            "cooked_rabbit" => Some(Item::CookedRabbit),
            "rabbit_stew" => Some(Item::RabbitStew),
            "rabbit_foot" => Some(Item::RabbitFoot),
            "rabbit_hide" => Some(Item::RabbitHide),
            "armor_stand" => Some(Item::ArmorStand),
            "iron_horse_armor" => Some(Item::IronHorseArmor),
            "golden_horse_armor" => Some(Item::GoldenHorseArmor),
            "diamond_horse_armor" => Some(Item::DiamondHorseArmor),
            "leather_horse_armor" => Some(Item::LeatherHorseArmor),
            "lead" => Some(Item::Lead),
            "name_tag" => Some(Item::NameTag),
            "command_block_minecart" => Some(Item::CommandBlockMinecart),
            "mutton" => Some(Item::Mutton),
            "cooked_mutton" => Some(Item::CookedMutton),
            "white_banner" => Some(Item::WhiteBanner),
            "orange_banner" => Some(Item::OrangeBanner),
            "magenta_banner" => Some(Item::MagentaBanner),
            "light_blue_banner" => Some(Item::LightBlueBanner),
            "yellow_banner" => Some(Item::YellowBanner),
            "lime_banner" => Some(Item::LimeBanner),
            "pink_banner" => Some(Item::PinkBanner),
            "gray_banner" => Some(Item::GrayBanner),
            "light_gray_banner" => Some(Item::LightGrayBanner),
            "cyan_banner" => Some(Item::CyanBanner),
            "purple_banner" => Some(Item::PurpleBanner),
            "blue_banner" => Some(Item::BlueBanner),
            "brown_banner" => Some(Item::BrownBanner),
            "green_banner" => Some(Item::GreenBanner),
            "red_banner" => Some(Item::RedBanner),
            "black_banner" => Some(Item::BlackBanner),
            "end_crystal" => Some(Item::EndCrystal),
            "chorus_fruit" => Some(Item::ChorusFruit),
            "popped_chorus_fruit" => Some(Item::PoppedChorusFruit),
            "beetroot" => Some(Item::Beetroot),
            "beetroot_seeds" => Some(Item::BeetrootSeeds),
            "beetroot_soup" => Some(Item::BeetrootSoup),
            "dragon_breath" => Some(Item::DragonBreath),
            "splash_potion" => Some(Item::SplashPotion),
            "spectral_arrow" => Some(Item::SpectralArrow),
            "tipped_arrow" => Some(Item::TippedArrow),
            "lingering_potion" => Some(Item::LingeringPotion),
            "shield" => Some(Item::Shield),
            "totem_of_undying" => Some(Item::TotemOfUndying),
            "shulker_shell" => Some(Item::ShulkerShell),
            "iron_nugget" => Some(Item::IronNugget),
            "knowledge_book" => Some(Item::KnowledgeBook),
            "debug_stick" => Some(Item::DebugStick),
            "music_disc_13" => Some(Item::MusicDisc13),
            "music_disc_cat" => Some(Item::MusicDiscCat),
            "music_disc_blocks" => Some(Item::MusicDiscBlocks),
            "music_disc_chirp" => Some(Item::MusicDiscChirp),
            "music_disc_far" => Some(Item::MusicDiscFar),
            "music_disc_mall" => Some(Item::MusicDiscMall),
            "music_disc_mellohi" => Some(Item::MusicDiscMellohi),
            "music_disc_stal" => Some(Item::MusicDiscStal),
            "music_disc_strad" => Some(Item::MusicDiscStrad),
            "music_disc_ward" => Some(Item::MusicDiscWard),
            "music_disc_11" => Some(Item::MusicDisc11),
            "music_disc_wait" => Some(Item::MusicDiscWait),
            "music_disc_otherside" => Some(Item::MusicDiscOtherside),
            "music_disc_pigstep" => Some(Item::MusicDiscPigstep),
            "trident" => Some(Item::Trident),
            "phantom_membrane" => Some(Item::PhantomMembrane),
            "nautilus_shell" => Some(Item::NautilusShell),
            "heart_of_the_sea" => Some(Item::HeartOfTheSea),
            "crossbow" => Some(Item::Crossbow),
            "suspicious_stew" => Some(Item::SuspiciousStew),
            "loom" => Some(Item::Loom),
            "flower_banner_pattern" => Some(Item::FlowerBannerPattern),
            "creeper_banner_pattern" => Some(Item::CreeperBannerPattern),
            "skull_banner_pattern" => Some(Item::SkullBannerPattern),
            "mojang_banner_pattern" => Some(Item::MojangBannerPattern),
            "globe_banner_pattern" => Some(Item::GlobeBannerPattern),
            "piglin_banner_pattern" => Some(Item::PiglinBannerPattern),
            "composter" => Some(Item::Composter),
            "barrel" => Some(Item::Barrel),
            "smoker" => Some(Item::Smoker),
            "blast_furnace" => Some(Item::BlastFurnace),
            "cartography_table" => Some(Item::CartographyTable),
            "fletching_table" => Some(Item::FletchingTable),
            "grindstone" => Some(Item::Grindstone),
            "smithing_table" => Some(Item::SmithingTable),
            "stonecutter" => Some(Item::Stonecutter),
            "bell" => Some(Item::Bell),
            "lantern" => Some(Item::Lantern),
            "soul_lantern" => Some(Item::SoulLantern),
            "sweet_berries" => Some(Item::SweetBerries),
            "glow_berries" => Some(Item::GlowBerries),
            "campfire" => Some(Item::Campfire),
            "soul_campfire" => Some(Item::SoulCampfire),
            "shroomlight" => Some(Item::Shroomlight),
            "honeycomb" => Some(Item::Honeycomb),
            "bee_nest" => Some(Item::BeeNest),
            "beehive" => Some(Item::Beehive),
            "honey_bottle" => Some(Item::HoneyBottle),
            "honeycomb_block" => Some(Item::HoneycombBlock),
            "lodestone" => Some(Item::Lodestone),
            "crying_obsidian" => Some(Item::CryingObsidian),
            "blackstone" => Some(Item::Blackstone),
            "blackstone_slab" => Some(Item::BlackstoneSlab),
            "blackstone_stairs" => Some(Item::BlackstoneStairs),
            "gilded_blackstone" => Some(Item::GildedBlackstone),
            "polished_blackstone" => Some(Item::PolishedBlackstone),
            "polished_blackstone_slab" => Some(Item::PolishedBlackstoneSlab),
            "polished_blackstone_stairs" => Some(Item::PolishedBlackstoneStairs),
            "chiseled_polished_blackstone" => Some(Item::ChiseledPolishedBlackstone),
            "polished_blackstone_bricks" => Some(Item::PolishedBlackstoneBricks),
            "polished_blackstone_brick_slab" => Some(Item::PolishedBlackstoneBrickSlab),
            "polished_blackstone_brick_stairs" => Some(Item::PolishedBlackstoneBrickStairs),
            "cracked_polished_blackstone_bricks" => Some(Item::CrackedPolishedBlackstoneBricks),
            "respawn_anchor" => Some(Item::RespawnAnchor),
            "candle" => Some(Item::Candle),
            "white_candle" => Some(Item::WhiteCandle),
            "orange_candle" => Some(Item::OrangeCandle),
            "magenta_candle" => Some(Item::MagentaCandle),
            "light_blue_candle" => Some(Item::LightBlueCandle),
            "yellow_candle" => Some(Item::YellowCandle),
            "lime_candle" => Some(Item::LimeCandle),
            "pink_candle" => Some(Item::PinkCandle),
            "gray_candle" => Some(Item::GrayCandle),
            "light_gray_candle" => Some(Item::LightGrayCandle),
            "cyan_candle" => Some(Item::CyanCandle),
            "purple_candle" => Some(Item::PurpleCandle),
            "blue_candle" => Some(Item::BlueCandle),
            "brown_candle" => Some(Item::BrownCandle),
            "green_candle" => Some(Item::GreenCandle),
            "red_candle" => Some(Item::RedCandle),
            "black_candle" => Some(Item::BlackCandle),
            "small_amethyst_bud" => Some(Item::SmallAmethystBud),
            "medium_amethyst_bud" => Some(Item::MediumAmethystBud),
            "large_amethyst_bud" => Some(Item::LargeAmethystBud),
            "amethyst_cluster" => Some(Item::AmethystCluster),
            "pointed_dripstone" => Some(Item::PointedDripstone),
            _ => None,
        }
    }
}
impl Item {
    #[doc = "Returns the `namespaced_id` property of this `Item`."]
    #[inline]
    pub fn namespaced_id(&self) -> &'static str {
        match self {
            Item::Stone => "minecraft:stone",
            Item::Granite => "minecraft:granite",
            Item::PolishedGranite => "minecraft:polished_granite",
            Item::Diorite => "minecraft:diorite",
            Item::PolishedDiorite => "minecraft:polished_diorite",
            Item::Andesite => "minecraft:andesite",
            Item::PolishedAndesite => "minecraft:polished_andesite",
            Item::Deepslate => "minecraft:deepslate",
            Item::CobbledDeepslate => "minecraft:cobbled_deepslate",
            Item::PolishedDeepslate => "minecraft:polished_deepslate",
            Item::Calcite => "minecraft:calcite",
            Item::Tuff => "minecraft:tuff",
            Item::DripstoneBlock => "minecraft:dripstone_block",
            Item::GrassBlock => "minecraft:grass_block",
            Item::Dirt => "minecraft:dirt",
            Item::CoarseDirt => "minecraft:coarse_dirt",
            Item::Podzol => "minecraft:podzol",
            Item::RootedDirt => "minecraft:rooted_dirt",
            Item::CrimsonNylium => "minecraft:crimson_nylium",
            Item::WarpedNylium => "minecraft:warped_nylium",
            Item::Cobblestone => "minecraft:cobblestone",
            Item::OakPlanks => "minecraft:oak_planks",
            Item::SprucePlanks => "minecraft:spruce_planks",
            Item::BirchPlanks => "minecraft:birch_planks",
            Item::JunglePlanks => "minecraft:jungle_planks",
            Item::AcaciaPlanks => "minecraft:acacia_planks",
            Item::DarkOakPlanks => "minecraft:dark_oak_planks",
            Item::CrimsonPlanks => "minecraft:crimson_planks",
            Item::WarpedPlanks => "minecraft:warped_planks",
            Item::OakSapling => "minecraft:oak_sapling",
            Item::SpruceSapling => "minecraft:spruce_sapling",
            Item::BirchSapling => "minecraft:birch_sapling",
            Item::JungleSapling => "minecraft:jungle_sapling",
            Item::AcaciaSapling => "minecraft:acacia_sapling",
            Item::DarkOakSapling => "minecraft:dark_oak_sapling",
            Item::Bedrock => "minecraft:bedrock",
            Item::Sand => "minecraft:sand",
            Item::RedSand => "minecraft:red_sand",
            Item::Gravel => "minecraft:gravel",
            Item::CoalOre => "minecraft:coal_ore",
            Item::DeepslateCoalOre => "minecraft:deepslate_coal_ore",
            Item::IronOre => "minecraft:iron_ore",
            Item::DeepslateIronOre => "minecraft:deepslate_iron_ore",
            Item::CopperOre => "minecraft:copper_ore",
            Item::DeepslateCopperOre => "minecraft:deepslate_copper_ore",
            Item::GoldOre => "minecraft:gold_ore",
            Item::DeepslateGoldOre => "minecraft:deepslate_gold_ore",
            Item::RedstoneOre => "minecraft:redstone_ore",
            Item::DeepslateRedstoneOre => "minecraft:deepslate_redstone_ore",
            Item::EmeraldOre => "minecraft:emerald_ore",
            Item::DeepslateEmeraldOre => "minecraft:deepslate_emerald_ore",
            Item::LapisOre => "minecraft:lapis_ore",
            Item::DeepslateLapisOre => "minecraft:deepslate_lapis_ore",
            Item::DiamondOre => "minecraft:diamond_ore",
            Item::DeepslateDiamondOre => "minecraft:deepslate_diamond_ore",
            Item::NetherGoldOre => "minecraft:nether_gold_ore",
            Item::NetherQuartzOre => "minecraft:nether_quartz_ore",
            Item::AncientDebris => "minecraft:ancient_debris",
            Item::CoalBlock => "minecraft:coal_block",
            Item::RawIronBlock => "minecraft:raw_iron_block",
            Item::RawCopperBlock => "minecraft:raw_copper_block",
            Item::RawGoldBlock => "minecraft:raw_gold_block",
            Item::AmethystBlock => "minecraft:amethyst_block",
            Item::BuddingAmethyst => "minecraft:budding_amethyst",
            Item::IronBlock => "minecraft:iron_block",
            Item::CopperBlock => "minecraft:copper_block",
            Item::GoldBlock => "minecraft:gold_block",
            Item::DiamondBlock => "minecraft:diamond_block",
            Item::NetheriteBlock => "minecraft:netherite_block",
            Item::ExposedCopper => "minecraft:exposed_copper",
            Item::WeatheredCopper => "minecraft:weathered_copper",
            Item::OxidizedCopper => "minecraft:oxidized_copper",
            Item::CutCopper => "minecraft:cut_copper",
            Item::ExposedCutCopper => "minecraft:exposed_cut_copper",
            Item::WeatheredCutCopper => "minecraft:weathered_cut_copper",
            Item::OxidizedCutCopper => "minecraft:oxidized_cut_copper",
            Item::CutCopperStairs => "minecraft:cut_copper_stairs",
            Item::ExposedCutCopperStairs => "minecraft:exposed_cut_copper_stairs",
            Item::WeatheredCutCopperStairs => "minecraft:weathered_cut_copper_stairs",
            Item::OxidizedCutCopperStairs => "minecraft:oxidized_cut_copper_stairs",
            Item::CutCopperSlab => "minecraft:cut_copper_slab",
            Item::ExposedCutCopperSlab => "minecraft:exposed_cut_copper_slab",
            Item::WeatheredCutCopperSlab => "minecraft:weathered_cut_copper_slab",
            Item::OxidizedCutCopperSlab => "minecraft:oxidized_cut_copper_slab",
            Item::WaxedCopperBlock => "minecraft:waxed_copper_block",
            Item::WaxedExposedCopper => "minecraft:waxed_exposed_copper",
            Item::WaxedWeatheredCopper => "minecraft:waxed_weathered_copper",
            Item::WaxedOxidizedCopper => "minecraft:waxed_oxidized_copper",
            Item::WaxedCutCopper => "minecraft:waxed_cut_copper",
            Item::WaxedExposedCutCopper => "minecraft:waxed_exposed_cut_copper",
            Item::WaxedWeatheredCutCopper => "minecraft:waxed_weathered_cut_copper",
            Item::WaxedOxidizedCutCopper => "minecraft:waxed_oxidized_cut_copper",
            Item::WaxedCutCopperStairs => "minecraft:waxed_cut_copper_stairs",
            Item::WaxedExposedCutCopperStairs => "minecraft:waxed_exposed_cut_copper_stairs",
            Item::WaxedWeatheredCutCopperStairs => "minecraft:waxed_weathered_cut_copper_stairs",
            Item::WaxedOxidizedCutCopperStairs => "minecraft:waxed_oxidized_cut_copper_stairs",
            Item::WaxedCutCopperSlab => "minecraft:waxed_cut_copper_slab",
            Item::WaxedExposedCutCopperSlab => "minecraft:waxed_exposed_cut_copper_slab",
            Item::WaxedWeatheredCutCopperSlab => "minecraft:waxed_weathered_cut_copper_slab",
            Item::WaxedOxidizedCutCopperSlab => "minecraft:waxed_oxidized_cut_copper_slab",
            Item::OakLog => "minecraft:oak_log",
            Item::SpruceLog => "minecraft:spruce_log",
            Item::BirchLog => "minecraft:birch_log",
            Item::JungleLog => "minecraft:jungle_log",
            Item::AcaciaLog => "minecraft:acacia_log",
            Item::DarkOakLog => "minecraft:dark_oak_log",
            Item::CrimsonStem => "minecraft:crimson_stem",
            Item::WarpedStem => "minecraft:warped_stem",
            Item::StrippedOakLog => "minecraft:stripped_oak_log",
            Item::StrippedSpruceLog => "minecraft:stripped_spruce_log",
            Item::StrippedBirchLog => "minecraft:stripped_birch_log",
            Item::StrippedJungleLog => "minecraft:stripped_jungle_log",
            Item::StrippedAcaciaLog => "minecraft:stripped_acacia_log",
            Item::StrippedDarkOakLog => "minecraft:stripped_dark_oak_log",
            Item::StrippedCrimsonStem => "minecraft:stripped_crimson_stem",
            Item::StrippedWarpedStem => "minecraft:stripped_warped_stem",
            Item::StrippedOakWood => "minecraft:stripped_oak_wood",
            Item::StrippedSpruceWood => "minecraft:stripped_spruce_wood",
            Item::StrippedBirchWood => "minecraft:stripped_birch_wood",
            Item::StrippedJungleWood => "minecraft:stripped_jungle_wood",
            Item::StrippedAcaciaWood => "minecraft:stripped_acacia_wood",
            Item::StrippedDarkOakWood => "minecraft:stripped_dark_oak_wood",
            Item::StrippedCrimsonHyphae => "minecraft:stripped_crimson_hyphae",
            Item::StrippedWarpedHyphae => "minecraft:stripped_warped_hyphae",
            Item::OakWood => "minecraft:oak_wood",
            Item::SpruceWood => "minecraft:spruce_wood",
            Item::BirchWood => "minecraft:birch_wood",
            Item::JungleWood => "minecraft:jungle_wood",
            Item::AcaciaWood => "minecraft:acacia_wood",
            Item::DarkOakWood => "minecraft:dark_oak_wood",
            Item::CrimsonHyphae => "minecraft:crimson_hyphae",
            Item::WarpedHyphae => "minecraft:warped_hyphae",
            Item::OakLeaves => "minecraft:oak_leaves",
            Item::SpruceLeaves => "minecraft:spruce_leaves",
            Item::BirchLeaves => "minecraft:birch_leaves",
            Item::JungleLeaves => "minecraft:jungle_leaves",
            Item::AcaciaLeaves => "minecraft:acacia_leaves",
            Item::DarkOakLeaves => "minecraft:dark_oak_leaves",
            Item::AzaleaLeaves => "minecraft:azalea_leaves",
            Item::FloweringAzaleaLeaves => "minecraft:flowering_azalea_leaves",
            Item::Sponge => "minecraft:sponge",
            Item::WetSponge => "minecraft:wet_sponge",
            Item::Glass => "minecraft:glass",
            Item::TintedGlass => "minecraft:tinted_glass",
            Item::LapisBlock => "minecraft:lapis_block",
            Item::Sandstone => "minecraft:sandstone",
            Item::ChiseledSandstone => "minecraft:chiseled_sandstone",
            Item::CutSandstone => "minecraft:cut_sandstone",
            Item::Cobweb => "minecraft:cobweb",
            Item::Grass => "minecraft:grass",
            Item::Fern => "minecraft:fern",
            Item::Azalea => "minecraft:azalea",
            Item::FloweringAzalea => "minecraft:flowering_azalea",
            Item::DeadBush => "minecraft:dead_bush",
            Item::Seagrass => "minecraft:seagrass",
            Item::SeaPickle => "minecraft:sea_pickle",
            Item::WhiteWool => "minecraft:white_wool",
            Item::OrangeWool => "minecraft:orange_wool",
            Item::MagentaWool => "minecraft:magenta_wool",
            Item::LightBlueWool => "minecraft:light_blue_wool",
            Item::YellowWool => "minecraft:yellow_wool",
            Item::LimeWool => "minecraft:lime_wool",
            Item::PinkWool => "minecraft:pink_wool",
            Item::GrayWool => "minecraft:gray_wool",
            Item::LightGrayWool => "minecraft:light_gray_wool",
            Item::CyanWool => "minecraft:cyan_wool",
            Item::PurpleWool => "minecraft:purple_wool",
            Item::BlueWool => "minecraft:blue_wool",
            Item::BrownWool => "minecraft:brown_wool",
            Item::GreenWool => "minecraft:green_wool",
            Item::RedWool => "minecraft:red_wool",
            Item::BlackWool => "minecraft:black_wool",
            Item::Dandelion => "minecraft:dandelion",
            Item::Poppy => "minecraft:poppy",
            Item::BlueOrchid => "minecraft:blue_orchid",
            Item::Allium => "minecraft:allium",
            Item::AzureBluet => "minecraft:azure_bluet",
            Item::RedTulip => "minecraft:red_tulip",
            Item::OrangeTulip => "minecraft:orange_tulip",
            Item::WhiteTulip => "minecraft:white_tulip",
            Item::PinkTulip => "minecraft:pink_tulip",
            Item::OxeyeDaisy => "minecraft:oxeye_daisy",
            Item::Cornflower => "minecraft:cornflower",
            Item::LilyOfTheValley => "minecraft:lily_of_the_valley",
            Item::WitherRose => "minecraft:wither_rose",
            Item::SporeBlossom => "minecraft:spore_blossom",
            Item::BrownMushroom => "minecraft:brown_mushroom",
            Item::RedMushroom => "minecraft:red_mushroom",
            Item::CrimsonFungus => "minecraft:crimson_fungus",
            Item::WarpedFungus => "minecraft:warped_fungus",
            Item::CrimsonRoots => "minecraft:crimson_roots",
            Item::WarpedRoots => "minecraft:warped_roots",
            Item::NetherSprouts => "minecraft:nether_sprouts",
            Item::WeepingVines => "minecraft:weeping_vines",
            Item::TwistingVines => "minecraft:twisting_vines",
            Item::SugarCane => "minecraft:sugar_cane",
            Item::Kelp => "minecraft:kelp",
            Item::MossCarpet => "minecraft:moss_carpet",
            Item::MossBlock => "minecraft:moss_block",
            Item::HangingRoots => "minecraft:hanging_roots",
            Item::BigDripleaf => "minecraft:big_dripleaf",
            Item::SmallDripleaf => "minecraft:small_dripleaf",
            Item::Bamboo => "minecraft:bamboo",
            Item::OakSlab => "minecraft:oak_slab",
            Item::SpruceSlab => "minecraft:spruce_slab",
            Item::BirchSlab => "minecraft:birch_slab",
            Item::JungleSlab => "minecraft:jungle_slab",
            Item::AcaciaSlab => "minecraft:acacia_slab",
            Item::DarkOakSlab => "minecraft:dark_oak_slab",
            Item::CrimsonSlab => "minecraft:crimson_slab",
            Item::WarpedSlab => "minecraft:warped_slab",
            Item::StoneSlab => "minecraft:stone_slab",
            Item::SmoothStoneSlab => "minecraft:smooth_stone_slab",
            Item::SandstoneSlab => "minecraft:sandstone_slab",
            Item::CutSandstoneSlab => "minecraft:cut_sandstone_slab",
            Item::PetrifiedOakSlab => "minecraft:petrified_oak_slab",
            Item::CobblestoneSlab => "minecraft:cobblestone_slab",
            Item::BrickSlab => "minecraft:brick_slab",
            Item::StoneBrickSlab => "minecraft:stone_brick_slab",
            Item::NetherBrickSlab => "minecraft:nether_brick_slab",
            Item::QuartzSlab => "minecraft:quartz_slab",
            Item::RedSandstoneSlab => "minecraft:red_sandstone_slab",
            Item::CutRedSandstoneSlab => "minecraft:cut_red_sandstone_slab",
            Item::PurpurSlab => "minecraft:purpur_slab",
            Item::PrismarineSlab => "minecraft:prismarine_slab",
            Item::PrismarineBrickSlab => "minecraft:prismarine_brick_slab",
            Item::DarkPrismarineSlab => "minecraft:dark_prismarine_slab",
            Item::SmoothQuartz => "minecraft:smooth_quartz",
            Item::SmoothRedSandstone => "minecraft:smooth_red_sandstone",
            Item::SmoothSandstone => "minecraft:smooth_sandstone",
            Item::SmoothStone => "minecraft:smooth_stone",
            Item::Bricks => "minecraft:bricks",
            Item::Bookshelf => "minecraft:bookshelf",
            Item::MossyCobblestone => "minecraft:mossy_cobblestone",
            Item::Obsidian => "minecraft:obsidian",
            Item::Torch => "minecraft:torch",
            Item::EndRod => "minecraft:end_rod",
            Item::ChorusPlant => "minecraft:chorus_plant",
            Item::ChorusFlower => "minecraft:chorus_flower",
            Item::PurpurBlock => "minecraft:purpur_block",
            Item::PurpurPillar => "minecraft:purpur_pillar",
            Item::PurpurStairs => "minecraft:purpur_stairs",
            Item::Spawner => "minecraft:spawner",
            Item::OakStairs => "minecraft:oak_stairs",
            Item::Chest => "minecraft:chest",
            Item::CraftingTable => "minecraft:crafting_table",
            Item::Farmland => "minecraft:farmland",
            Item::Furnace => "minecraft:furnace",
            Item::Ladder => "minecraft:ladder",
            Item::CobblestoneStairs => "minecraft:cobblestone_stairs",
            Item::Snow => "minecraft:snow",
            Item::Ice => "minecraft:ice",
            Item::SnowBlock => "minecraft:snow_block",
            Item::Cactus => "minecraft:cactus",
            Item::Clay => "minecraft:clay",
            Item::Jukebox => "minecraft:jukebox",
            Item::OakFence => "minecraft:oak_fence",
            Item::SpruceFence => "minecraft:spruce_fence",
            Item::BirchFence => "minecraft:birch_fence",
            Item::JungleFence => "minecraft:jungle_fence",
            Item::AcaciaFence => "minecraft:acacia_fence",
            Item::DarkOakFence => "minecraft:dark_oak_fence",
            Item::CrimsonFence => "minecraft:crimson_fence",
            Item::WarpedFence => "minecraft:warped_fence",
            Item::Pumpkin => "minecraft:pumpkin",
            Item::CarvedPumpkin => "minecraft:carved_pumpkin",
            Item::JackOLantern => "minecraft:jack_o_lantern",
            Item::Netherrack => "minecraft:netherrack",
            Item::SoulSand => "minecraft:soul_sand",
            Item::SoulSoil => "minecraft:soul_soil",
            Item::Basalt => "minecraft:basalt",
            Item::PolishedBasalt => "minecraft:polished_basalt",
            Item::SmoothBasalt => "minecraft:smooth_basalt",
            Item::SoulTorch => "minecraft:soul_torch",
            Item::Glowstone => "minecraft:glowstone",
            Item::InfestedStone => "minecraft:infested_stone",
            Item::InfestedCobblestone => "minecraft:infested_cobblestone",
            Item::InfestedStoneBricks => "minecraft:infested_stone_bricks",
            Item::InfestedMossyStoneBricks => "minecraft:infested_mossy_stone_bricks",
            Item::InfestedCrackedStoneBricks => "minecraft:infested_cracked_stone_bricks",
            Item::InfestedChiseledStoneBricks => "minecraft:infested_chiseled_stone_bricks",
            Item::InfestedDeepslate => "minecraft:infested_deepslate",
            Item::StoneBricks => "minecraft:stone_bricks",
            Item::MossyStoneBricks => "minecraft:mossy_stone_bricks",
            Item::CrackedStoneBricks => "minecraft:cracked_stone_bricks",
            Item::ChiseledStoneBricks => "minecraft:chiseled_stone_bricks",
            Item::DeepslateBricks => "minecraft:deepslate_bricks",
            Item::CrackedDeepslateBricks => "minecraft:cracked_deepslate_bricks",
            Item::DeepslateTiles => "minecraft:deepslate_tiles",
            Item::CrackedDeepslateTiles => "minecraft:cracked_deepslate_tiles",
            Item::ChiseledDeepslate => "minecraft:chiseled_deepslate",
            Item::BrownMushroomBlock => "minecraft:brown_mushroom_block",
            Item::RedMushroomBlock => "minecraft:red_mushroom_block",
            Item::MushroomStem => "minecraft:mushroom_stem",
            Item::IronBars => "minecraft:iron_bars",
            Item::Chain => "minecraft:chain",
            Item::GlassPane => "minecraft:glass_pane",
            Item::Melon => "minecraft:melon",
            Item::Vine => "minecraft:vine",
            Item::GlowLichen => "minecraft:glow_lichen",
            Item::BrickStairs => "minecraft:brick_stairs",
            Item::StoneBrickStairs => "minecraft:stone_brick_stairs",
            Item::Mycelium => "minecraft:mycelium",
            Item::LilyPad => "minecraft:lily_pad",
            Item::NetherBricks => "minecraft:nether_bricks",
            Item::CrackedNetherBricks => "minecraft:cracked_nether_bricks",
            Item::ChiseledNetherBricks => "minecraft:chiseled_nether_bricks",
            Item::NetherBrickFence => "minecraft:nether_brick_fence",
            Item::NetherBrickStairs => "minecraft:nether_brick_stairs",
            Item::EnchantingTable => "minecraft:enchanting_table",
            Item::EndPortalFrame => "minecraft:end_portal_frame",
            Item::EndStone => "minecraft:end_stone",
            Item::EndStoneBricks => "minecraft:end_stone_bricks",
            Item::DragonEgg => "minecraft:dragon_egg",
            Item::SandstoneStairs => "minecraft:sandstone_stairs",
            Item::EnderChest => "minecraft:ender_chest",
            Item::EmeraldBlock => "minecraft:emerald_block",
            Item::SpruceStairs => "minecraft:spruce_stairs",
            Item::BirchStairs => "minecraft:birch_stairs",
            Item::JungleStairs => "minecraft:jungle_stairs",
            Item::CrimsonStairs => "minecraft:crimson_stairs",
            Item::WarpedStairs => "minecraft:warped_stairs",
            Item::CommandBlock => "minecraft:command_block",
            Item::Beacon => "minecraft:beacon",
            Item::CobblestoneWall => "minecraft:cobblestone_wall",
            Item::MossyCobblestoneWall => "minecraft:mossy_cobblestone_wall",
            Item::BrickWall => "minecraft:brick_wall",
            Item::PrismarineWall => "minecraft:prismarine_wall",
            Item::RedSandstoneWall => "minecraft:red_sandstone_wall",
            Item::MossyStoneBrickWall => "minecraft:mossy_stone_brick_wall",
            Item::GraniteWall => "minecraft:granite_wall",
            Item::StoneBrickWall => "minecraft:stone_brick_wall",
            Item::NetherBrickWall => "minecraft:nether_brick_wall",
            Item::AndesiteWall => "minecraft:andesite_wall",
            Item::RedNetherBrickWall => "minecraft:red_nether_brick_wall",
            Item::SandstoneWall => "minecraft:sandstone_wall",
            Item::EndStoneBrickWall => "minecraft:end_stone_brick_wall",
            Item::DioriteWall => "minecraft:diorite_wall",
            Item::BlackstoneWall => "minecraft:blackstone_wall",
            Item::PolishedBlackstoneWall => "minecraft:polished_blackstone_wall",
            Item::PolishedBlackstoneBrickWall => "minecraft:polished_blackstone_brick_wall",
            Item::CobbledDeepslateWall => "minecraft:cobbled_deepslate_wall",
            Item::PolishedDeepslateWall => "minecraft:polished_deepslate_wall",
            Item::DeepslateBrickWall => "minecraft:deepslate_brick_wall",
            Item::DeepslateTileWall => "minecraft:deepslate_tile_wall",
            Item::Anvil => "minecraft:anvil",
            Item::ChippedAnvil => "minecraft:chipped_anvil",
            Item::DamagedAnvil => "minecraft:damaged_anvil",
            Item::ChiseledQuartzBlock => "minecraft:chiseled_quartz_block",
            Item::QuartzBlock => "minecraft:quartz_block",
            Item::QuartzBricks => "minecraft:quartz_bricks",
            Item::QuartzPillar => "minecraft:quartz_pillar",
            Item::QuartzStairs => "minecraft:quartz_stairs",
            Item::WhiteTerracotta => "minecraft:white_terracotta",
            Item::OrangeTerracotta => "minecraft:orange_terracotta",
            Item::MagentaTerracotta => "minecraft:magenta_terracotta",
            Item::LightBlueTerracotta => "minecraft:light_blue_terracotta",
            Item::YellowTerracotta => "minecraft:yellow_terracotta",
            Item::LimeTerracotta => "minecraft:lime_terracotta",
            Item::PinkTerracotta => "minecraft:pink_terracotta",
            Item::GrayTerracotta => "minecraft:gray_terracotta",
            Item::LightGrayTerracotta => "minecraft:light_gray_terracotta",
            Item::CyanTerracotta => "minecraft:cyan_terracotta",
            Item::PurpleTerracotta => "minecraft:purple_terracotta",
            Item::BlueTerracotta => "minecraft:blue_terracotta",
            Item::BrownTerracotta => "minecraft:brown_terracotta",
            Item::GreenTerracotta => "minecraft:green_terracotta",
            Item::RedTerracotta => "minecraft:red_terracotta",
            Item::BlackTerracotta => "minecraft:black_terracotta",
            Item::Barrier => "minecraft:barrier",
            Item::Light => "minecraft:light",
            Item::HayBlock => "minecraft:hay_block",
            Item::WhiteCarpet => "minecraft:white_carpet",
            Item::OrangeCarpet => "minecraft:orange_carpet",
            Item::MagentaCarpet => "minecraft:magenta_carpet",
            Item::LightBlueCarpet => "minecraft:light_blue_carpet",
            Item::YellowCarpet => "minecraft:yellow_carpet",
            Item::LimeCarpet => "minecraft:lime_carpet",
            Item::PinkCarpet => "minecraft:pink_carpet",
            Item::GrayCarpet => "minecraft:gray_carpet",
            Item::LightGrayCarpet => "minecraft:light_gray_carpet",
            Item::CyanCarpet => "minecraft:cyan_carpet",
            Item::PurpleCarpet => "minecraft:purple_carpet",
            Item::BlueCarpet => "minecraft:blue_carpet",
            Item::BrownCarpet => "minecraft:brown_carpet",
            Item::GreenCarpet => "minecraft:green_carpet",
            Item::RedCarpet => "minecraft:red_carpet",
            Item::BlackCarpet => "minecraft:black_carpet",
            Item::Terracotta => "minecraft:terracotta",
            Item::PackedIce => "minecraft:packed_ice",
            Item::AcaciaStairs => "minecraft:acacia_stairs",
            Item::DarkOakStairs => "minecraft:dark_oak_stairs",
            Item::DirtPath => "minecraft:dirt_path",
            Item::Sunflower => "minecraft:sunflower",
            Item::Lilac => "minecraft:lilac",
            Item::RoseBush => "minecraft:rose_bush",
            Item::Peony => "minecraft:peony",
            Item::TallGrass => "minecraft:tall_grass",
            Item::LargeFern => "minecraft:large_fern",
            Item::WhiteStainedGlass => "minecraft:white_stained_glass",
            Item::OrangeStainedGlass => "minecraft:orange_stained_glass",
            Item::MagentaStainedGlass => "minecraft:magenta_stained_glass",
            Item::LightBlueStainedGlass => "minecraft:light_blue_stained_glass",
            Item::YellowStainedGlass => "minecraft:yellow_stained_glass",
            Item::LimeStainedGlass => "minecraft:lime_stained_glass",
            Item::PinkStainedGlass => "minecraft:pink_stained_glass",
            Item::GrayStainedGlass => "minecraft:gray_stained_glass",
            Item::LightGrayStainedGlass => "minecraft:light_gray_stained_glass",
            Item::CyanStainedGlass => "minecraft:cyan_stained_glass",
            Item::PurpleStainedGlass => "minecraft:purple_stained_glass",
            Item::BlueStainedGlass => "minecraft:blue_stained_glass",
            Item::BrownStainedGlass => "minecraft:brown_stained_glass",
            Item::GreenStainedGlass => "minecraft:green_stained_glass",
            Item::RedStainedGlass => "minecraft:red_stained_glass",
            Item::BlackStainedGlass => "minecraft:black_stained_glass",
            Item::WhiteStainedGlassPane => "minecraft:white_stained_glass_pane",
            Item::OrangeStainedGlassPane => "minecraft:orange_stained_glass_pane",
            Item::MagentaStainedGlassPane => "minecraft:magenta_stained_glass_pane",
            Item::LightBlueStainedGlassPane => "minecraft:light_blue_stained_glass_pane",
            Item::YellowStainedGlassPane => "minecraft:yellow_stained_glass_pane",
            Item::LimeStainedGlassPane => "minecraft:lime_stained_glass_pane",
            Item::PinkStainedGlassPane => "minecraft:pink_stained_glass_pane",
            Item::GrayStainedGlassPane => "minecraft:gray_stained_glass_pane",
            Item::LightGrayStainedGlassPane => "minecraft:light_gray_stained_glass_pane",
            Item::CyanStainedGlassPane => "minecraft:cyan_stained_glass_pane",
            Item::PurpleStainedGlassPane => "minecraft:purple_stained_glass_pane",
            Item::BlueStainedGlassPane => "minecraft:blue_stained_glass_pane",
            Item::BrownStainedGlassPane => "minecraft:brown_stained_glass_pane",
            Item::GreenStainedGlassPane => "minecraft:green_stained_glass_pane",
            Item::RedStainedGlassPane => "minecraft:red_stained_glass_pane",
            Item::BlackStainedGlassPane => "minecraft:black_stained_glass_pane",
            Item::Prismarine => "minecraft:prismarine",
            Item::PrismarineBricks => "minecraft:prismarine_bricks",
            Item::DarkPrismarine => "minecraft:dark_prismarine",
            Item::PrismarineStairs => "minecraft:prismarine_stairs",
            Item::PrismarineBrickStairs => "minecraft:prismarine_brick_stairs",
            Item::DarkPrismarineStairs => "minecraft:dark_prismarine_stairs",
            Item::SeaLantern => "minecraft:sea_lantern",
            Item::RedSandstone => "minecraft:red_sandstone",
            Item::ChiseledRedSandstone => "minecraft:chiseled_red_sandstone",
            Item::CutRedSandstone => "minecraft:cut_red_sandstone",
            Item::RedSandstoneStairs => "minecraft:red_sandstone_stairs",
            Item::RepeatingCommandBlock => "minecraft:repeating_command_block",
            Item::ChainCommandBlock => "minecraft:chain_command_block",
            Item::MagmaBlock => "minecraft:magma_block",
            Item::NetherWartBlock => "minecraft:nether_wart_block",
            Item::WarpedWartBlock => "minecraft:warped_wart_block",
            Item::RedNetherBricks => "minecraft:red_nether_bricks",
            Item::BoneBlock => "minecraft:bone_block",
            Item::StructureVoid => "minecraft:structure_void",
            Item::ShulkerBox => "minecraft:shulker_box",
            Item::WhiteShulkerBox => "minecraft:white_shulker_box",
            Item::OrangeShulkerBox => "minecraft:orange_shulker_box",
            Item::MagentaShulkerBox => "minecraft:magenta_shulker_box",
            Item::LightBlueShulkerBox => "minecraft:light_blue_shulker_box",
            Item::YellowShulkerBox => "minecraft:yellow_shulker_box",
            Item::LimeShulkerBox => "minecraft:lime_shulker_box",
            Item::PinkShulkerBox => "minecraft:pink_shulker_box",
            Item::GrayShulkerBox => "minecraft:gray_shulker_box",
            Item::LightGrayShulkerBox => "minecraft:light_gray_shulker_box",
            Item::CyanShulkerBox => "minecraft:cyan_shulker_box",
            Item::PurpleShulkerBox => "minecraft:purple_shulker_box",
            Item::BlueShulkerBox => "minecraft:blue_shulker_box",
            Item::BrownShulkerBox => "minecraft:brown_shulker_box",
            Item::GreenShulkerBox => "minecraft:green_shulker_box",
            Item::RedShulkerBox => "minecraft:red_shulker_box",
            Item::BlackShulkerBox => "minecraft:black_shulker_box",
            Item::WhiteGlazedTerracotta => "minecraft:white_glazed_terracotta",
            Item::OrangeGlazedTerracotta => "minecraft:orange_glazed_terracotta",
            Item::MagentaGlazedTerracotta => "minecraft:magenta_glazed_terracotta",
            Item::LightBlueGlazedTerracotta => "minecraft:light_blue_glazed_terracotta",
            Item::YellowGlazedTerracotta => "minecraft:yellow_glazed_terracotta",
            Item::LimeGlazedTerracotta => "minecraft:lime_glazed_terracotta",
            Item::PinkGlazedTerracotta => "minecraft:pink_glazed_terracotta",
            Item::GrayGlazedTerracotta => "minecraft:gray_glazed_terracotta",
            Item::LightGrayGlazedTerracotta => "minecraft:light_gray_glazed_terracotta",
            Item::CyanGlazedTerracotta => "minecraft:cyan_glazed_terracotta",
            Item::PurpleGlazedTerracotta => "minecraft:purple_glazed_terracotta",
            Item::BlueGlazedTerracotta => "minecraft:blue_glazed_terracotta",
            Item::BrownGlazedTerracotta => "minecraft:brown_glazed_terracotta",
            Item::GreenGlazedTerracotta => "minecraft:green_glazed_terracotta",
            Item::RedGlazedTerracotta => "minecraft:red_glazed_terracotta",
            Item::BlackGlazedTerracotta => "minecraft:black_glazed_terracotta",
            Item::WhiteConcrete => "minecraft:white_concrete",
            Item::OrangeConcrete => "minecraft:orange_concrete",
            Item::MagentaConcrete => "minecraft:magenta_concrete",
            Item::LightBlueConcrete => "minecraft:light_blue_concrete",
            Item::YellowConcrete => "minecraft:yellow_concrete",
            Item::LimeConcrete => "minecraft:lime_concrete",
            Item::PinkConcrete => "minecraft:pink_concrete",
            Item::GrayConcrete => "minecraft:gray_concrete",
            Item::LightGrayConcrete => "minecraft:light_gray_concrete",
            Item::CyanConcrete => "minecraft:cyan_concrete",
            Item::PurpleConcrete => "minecraft:purple_concrete",
            Item::BlueConcrete => "minecraft:blue_concrete",
            Item::BrownConcrete => "minecraft:brown_concrete",
            Item::GreenConcrete => "minecraft:green_concrete",
            Item::RedConcrete => "minecraft:red_concrete",
            Item::BlackConcrete => "minecraft:black_concrete",
            Item::WhiteConcretePowder => "minecraft:white_concrete_powder",
            Item::OrangeConcretePowder => "minecraft:orange_concrete_powder",
            Item::MagentaConcretePowder => "minecraft:magenta_concrete_powder",
            Item::LightBlueConcretePowder => "minecraft:light_blue_concrete_powder",
            Item::YellowConcretePowder => "minecraft:yellow_concrete_powder",
            Item::LimeConcretePowder => "minecraft:lime_concrete_powder",
            Item::PinkConcretePowder => "minecraft:pink_concrete_powder",
            Item::GrayConcretePowder => "minecraft:gray_concrete_powder",
            Item::LightGrayConcretePowder => "minecraft:light_gray_concrete_powder",
            Item::CyanConcretePowder => "minecraft:cyan_concrete_powder",
            Item::PurpleConcretePowder => "minecraft:purple_concrete_powder",
            Item::BlueConcretePowder => "minecraft:blue_concrete_powder",
            Item::BrownConcretePowder => "minecraft:brown_concrete_powder",
            Item::GreenConcretePowder => "minecraft:green_concrete_powder",
            Item::RedConcretePowder => "minecraft:red_concrete_powder",
            Item::BlackConcretePowder => "minecraft:black_concrete_powder",
            Item::TurtleEgg => "minecraft:turtle_egg",
            Item::DeadTubeCoralBlock => "minecraft:dead_tube_coral_block",
            Item::DeadBrainCoralBlock => "minecraft:dead_brain_coral_block",
            Item::DeadBubbleCoralBlock => "minecraft:dead_bubble_coral_block",
            Item::DeadFireCoralBlock => "minecraft:dead_fire_coral_block",
            Item::DeadHornCoralBlock => "minecraft:dead_horn_coral_block",
            Item::TubeCoralBlock => "minecraft:tube_coral_block",
            Item::BrainCoralBlock => "minecraft:brain_coral_block",
            Item::BubbleCoralBlock => "minecraft:bubble_coral_block",
            Item::FireCoralBlock => "minecraft:fire_coral_block",
            Item::HornCoralBlock => "minecraft:horn_coral_block",
            Item::TubeCoral => "minecraft:tube_coral",
            Item::BrainCoral => "minecraft:brain_coral",
            Item::BubbleCoral => "minecraft:bubble_coral",
            Item::FireCoral => "minecraft:fire_coral",
            Item::HornCoral => "minecraft:horn_coral",
            Item::DeadBrainCoral => "minecraft:dead_brain_coral",
            Item::DeadBubbleCoral => "minecraft:dead_bubble_coral",
            Item::DeadFireCoral => "minecraft:dead_fire_coral",
            Item::DeadHornCoral => "minecraft:dead_horn_coral",
            Item::DeadTubeCoral => "minecraft:dead_tube_coral",
            Item::TubeCoralFan => "minecraft:tube_coral_fan",
            Item::BrainCoralFan => "minecraft:brain_coral_fan",
            Item::BubbleCoralFan => "minecraft:bubble_coral_fan",
            Item::FireCoralFan => "minecraft:fire_coral_fan",
            Item::HornCoralFan => "minecraft:horn_coral_fan",
            Item::DeadTubeCoralFan => "minecraft:dead_tube_coral_fan",
            Item::DeadBrainCoralFan => "minecraft:dead_brain_coral_fan",
            Item::DeadBubbleCoralFan => "minecraft:dead_bubble_coral_fan",
            Item::DeadFireCoralFan => "minecraft:dead_fire_coral_fan",
            Item::DeadHornCoralFan => "minecraft:dead_horn_coral_fan",
            Item::BlueIce => "minecraft:blue_ice",
            Item::Conduit => "minecraft:conduit",
            Item::PolishedGraniteStairs => "minecraft:polished_granite_stairs",
            Item::SmoothRedSandstoneStairs => "minecraft:smooth_red_sandstone_stairs",
            Item::MossyStoneBrickStairs => "minecraft:mossy_stone_brick_stairs",
            Item::PolishedDioriteStairs => "minecraft:polished_diorite_stairs",
            Item::MossyCobblestoneStairs => "minecraft:mossy_cobblestone_stairs",
            Item::EndStoneBrickStairs => "minecraft:end_stone_brick_stairs",
            Item::StoneStairs => "minecraft:stone_stairs",
            Item::SmoothSandstoneStairs => "minecraft:smooth_sandstone_stairs",
            Item::SmoothQuartzStairs => "minecraft:smooth_quartz_stairs",
            Item::GraniteStairs => "minecraft:granite_stairs",
            Item::AndesiteStairs => "minecraft:andesite_stairs",
            Item::RedNetherBrickStairs => "minecraft:red_nether_brick_stairs",
            Item::PolishedAndesiteStairs => "minecraft:polished_andesite_stairs",
            Item::DioriteStairs => "minecraft:diorite_stairs",
            Item::CobbledDeepslateStairs => "minecraft:cobbled_deepslate_stairs",
            Item::PolishedDeepslateStairs => "minecraft:polished_deepslate_stairs",
            Item::DeepslateBrickStairs => "minecraft:deepslate_brick_stairs",
            Item::DeepslateTileStairs => "minecraft:deepslate_tile_stairs",
            Item::PolishedGraniteSlab => "minecraft:polished_granite_slab",
            Item::SmoothRedSandstoneSlab => "minecraft:smooth_red_sandstone_slab",
            Item::MossyStoneBrickSlab => "minecraft:mossy_stone_brick_slab",
            Item::PolishedDioriteSlab => "minecraft:polished_diorite_slab",
            Item::MossyCobblestoneSlab => "minecraft:mossy_cobblestone_slab",
            Item::EndStoneBrickSlab => "minecraft:end_stone_brick_slab",
            Item::SmoothSandstoneSlab => "minecraft:smooth_sandstone_slab",
            Item::SmoothQuartzSlab => "minecraft:smooth_quartz_slab",
            Item::GraniteSlab => "minecraft:granite_slab",
            Item::AndesiteSlab => "minecraft:andesite_slab",
            Item::RedNetherBrickSlab => "minecraft:red_nether_brick_slab",
            Item::PolishedAndesiteSlab => "minecraft:polished_andesite_slab",
            Item::DioriteSlab => "minecraft:diorite_slab",
            Item::CobbledDeepslateSlab => "minecraft:cobbled_deepslate_slab",
            Item::PolishedDeepslateSlab => "minecraft:polished_deepslate_slab",
            Item::DeepslateBrickSlab => "minecraft:deepslate_brick_slab",
            Item::DeepslateTileSlab => "minecraft:deepslate_tile_slab",
            Item::Scaffolding => "minecraft:scaffolding",
            Item::Redstone => "minecraft:redstone",
            Item::RedstoneTorch => "minecraft:redstone_torch",
            Item::RedstoneBlock => "minecraft:redstone_block",
            Item::Repeater => "minecraft:repeater",
            Item::Comparator => "minecraft:comparator",
            Item::Piston => "minecraft:piston",
            Item::StickyPiston => "minecraft:sticky_piston",
            Item::SlimeBlock => "minecraft:slime_block",
            Item::HoneyBlock => "minecraft:honey_block",
            Item::Observer => "minecraft:observer",
            Item::Hopper => "minecraft:hopper",
            Item::Dispenser => "minecraft:dispenser",
            Item::Dropper => "minecraft:dropper",
            Item::Lectern => "minecraft:lectern",
            Item::Target => "minecraft:target",
            Item::Lever => "minecraft:lever",
            Item::LightningRod => "minecraft:lightning_rod",
            Item::DaylightDetector => "minecraft:daylight_detector",
            Item::SculkSensor => "minecraft:sculk_sensor",
            Item::TripwireHook => "minecraft:tripwire_hook",
            Item::TrappedChest => "minecraft:trapped_chest",
            Item::Tnt => "minecraft:tnt",
            Item::RedstoneLamp => "minecraft:redstone_lamp",
            Item::NoteBlock => "minecraft:note_block",
            Item::StoneButton => "minecraft:stone_button",
            Item::PolishedBlackstoneButton => "minecraft:polished_blackstone_button",
            Item::OakButton => "minecraft:oak_button",
            Item::SpruceButton => "minecraft:spruce_button",
            Item::BirchButton => "minecraft:birch_button",
            Item::JungleButton => "minecraft:jungle_button",
            Item::AcaciaButton => "minecraft:acacia_button",
            Item::DarkOakButton => "minecraft:dark_oak_button",
            Item::CrimsonButton => "minecraft:crimson_button",
            Item::WarpedButton => "minecraft:warped_button",
            Item::StonePressurePlate => "minecraft:stone_pressure_plate",
            Item::PolishedBlackstonePressurePlate => "minecraft:polished_blackstone_pressure_plate",
            Item::LightWeightedPressurePlate => "minecraft:light_weighted_pressure_plate",
            Item::HeavyWeightedPressurePlate => "minecraft:heavy_weighted_pressure_plate",
            Item::OakPressurePlate => "minecraft:oak_pressure_plate",
            Item::SprucePressurePlate => "minecraft:spruce_pressure_plate",
            Item::BirchPressurePlate => "minecraft:birch_pressure_plate",
            Item::JunglePressurePlate => "minecraft:jungle_pressure_plate",
            Item::AcaciaPressurePlate => "minecraft:acacia_pressure_plate",
            Item::DarkOakPressurePlate => "minecraft:dark_oak_pressure_plate",
            Item::CrimsonPressurePlate => "minecraft:crimson_pressure_plate",
            Item::WarpedPressurePlate => "minecraft:warped_pressure_plate",
            Item::IronDoor => "minecraft:iron_door",
            Item::OakDoor => "minecraft:oak_door",
            Item::SpruceDoor => "minecraft:spruce_door",
            Item::BirchDoor => "minecraft:birch_door",
            Item::JungleDoor => "minecraft:jungle_door",
            Item::AcaciaDoor => "minecraft:acacia_door",
            Item::DarkOakDoor => "minecraft:dark_oak_door",
            Item::CrimsonDoor => "minecraft:crimson_door",
            Item::WarpedDoor => "minecraft:warped_door",
            Item::IronTrapdoor => "minecraft:iron_trapdoor",
            Item::OakTrapdoor => "minecraft:oak_trapdoor",
            Item::SpruceTrapdoor => "minecraft:spruce_trapdoor",
            Item::BirchTrapdoor => "minecraft:birch_trapdoor",
            Item::JungleTrapdoor => "minecraft:jungle_trapdoor",
            Item::AcaciaTrapdoor => "minecraft:acacia_trapdoor",
            Item::DarkOakTrapdoor => "minecraft:dark_oak_trapdoor",
            Item::CrimsonTrapdoor => "minecraft:crimson_trapdoor",
            Item::WarpedTrapdoor => "minecraft:warped_trapdoor",
            Item::OakFenceGate => "minecraft:oak_fence_gate",
            Item::SpruceFenceGate => "minecraft:spruce_fence_gate",
            Item::BirchFenceGate => "minecraft:birch_fence_gate",
            Item::JungleFenceGate => "minecraft:jungle_fence_gate",
            Item::AcaciaFenceGate => "minecraft:acacia_fence_gate",
            Item::DarkOakFenceGate => "minecraft:dark_oak_fence_gate",
            Item::CrimsonFenceGate => "minecraft:crimson_fence_gate",
            Item::WarpedFenceGate => "minecraft:warped_fence_gate",
            Item::PoweredRail => "minecraft:powered_rail",
            Item::DetectorRail => "minecraft:detector_rail",
            Item::Rail => "minecraft:rail",
            Item::ActivatorRail => "minecraft:activator_rail",
            Item::Saddle => "minecraft:saddle",
            Item::Minecart => "minecraft:minecart",
            Item::ChestMinecart => "minecraft:chest_minecart",
            Item::FurnaceMinecart => "minecraft:furnace_minecart",
            Item::TntMinecart => "minecraft:tnt_minecart",
            Item::HopperMinecart => "minecraft:hopper_minecart",
            Item::CarrotOnAStick => "minecraft:carrot_on_a_stick",
            Item::WarpedFungusOnAStick => "minecraft:warped_fungus_on_a_stick",
            Item::Elytra => "minecraft:elytra",
            Item::OakBoat => "minecraft:oak_boat",
            Item::SpruceBoat => "minecraft:spruce_boat",
            Item::BirchBoat => "minecraft:birch_boat",
            Item::JungleBoat => "minecraft:jungle_boat",
            Item::AcaciaBoat => "minecraft:acacia_boat",
            Item::DarkOakBoat => "minecraft:dark_oak_boat",
            Item::StructureBlock => "minecraft:structure_block",
            Item::Jigsaw => "minecraft:jigsaw",
            Item::TurtleHelmet => "minecraft:turtle_helmet",
            Item::Scute => "minecraft:scute",
            Item::FlintAndSteel => "minecraft:flint_and_steel",
            Item::Apple => "minecraft:apple",
            Item::Bow => "minecraft:bow",
            Item::Arrow => "minecraft:arrow",
            Item::Coal => "minecraft:coal",
            Item::Charcoal => "minecraft:charcoal",
            Item::Diamond => "minecraft:diamond",
            Item::Emerald => "minecraft:emerald",
            Item::LapisLazuli => "minecraft:lapis_lazuli",
            Item::Quartz => "minecraft:quartz",
            Item::AmethystShard => "minecraft:amethyst_shard",
            Item::RawIron => "minecraft:raw_iron",
            Item::IronIngot => "minecraft:iron_ingot",
            Item::RawCopper => "minecraft:raw_copper",
            Item::CopperIngot => "minecraft:copper_ingot",
            Item::RawGold => "minecraft:raw_gold",
            Item::GoldIngot => "minecraft:gold_ingot",
            Item::NetheriteIngot => "minecraft:netherite_ingot",
            Item::NetheriteScrap => "minecraft:netherite_scrap",
            Item::WoodenSword => "minecraft:wooden_sword",
            Item::WoodenShovel => "minecraft:wooden_shovel",
            Item::WoodenPickaxe => "minecraft:wooden_pickaxe",
            Item::WoodenAxe => "minecraft:wooden_axe",
            Item::WoodenHoe => "minecraft:wooden_hoe",
            Item::StoneSword => "minecraft:stone_sword",
            Item::StoneShovel => "minecraft:stone_shovel",
            Item::StonePickaxe => "minecraft:stone_pickaxe",
            Item::StoneAxe => "minecraft:stone_axe",
            Item::StoneHoe => "minecraft:stone_hoe",
            Item::GoldenSword => "minecraft:golden_sword",
            Item::GoldenShovel => "minecraft:golden_shovel",
            Item::GoldenPickaxe => "minecraft:golden_pickaxe",
            Item::GoldenAxe => "minecraft:golden_axe",
            Item::GoldenHoe => "minecraft:golden_hoe",
            Item::IronSword => "minecraft:iron_sword",
            Item::IronShovel => "minecraft:iron_shovel",
            Item::IronPickaxe => "minecraft:iron_pickaxe",
            Item::IronAxe => "minecraft:iron_axe",
            Item::IronHoe => "minecraft:iron_hoe",
            Item::DiamondSword => "minecraft:diamond_sword",
            Item::DiamondShovel => "minecraft:diamond_shovel",
            Item::DiamondPickaxe => "minecraft:diamond_pickaxe",
            Item::DiamondAxe => "minecraft:diamond_axe",
            Item::DiamondHoe => "minecraft:diamond_hoe",
            Item::NetheriteSword => "minecraft:netherite_sword",
            Item::NetheriteShovel => "minecraft:netherite_shovel",
            Item::NetheritePickaxe => "minecraft:netherite_pickaxe",
            Item::NetheriteAxe => "minecraft:netherite_axe",
            Item::NetheriteHoe => "minecraft:netherite_hoe",
            Item::Stick => "minecraft:stick",
            Item::Bowl => "minecraft:bowl",
            Item::MushroomStew => "minecraft:mushroom_stew",
            Item::String => "minecraft:string",
            Item::Feather => "minecraft:feather",
            Item::Gunpowder => "minecraft:gunpowder",
            Item::WheatSeeds => "minecraft:wheat_seeds",
            Item::Wheat => "minecraft:wheat",
            Item::Bread => "minecraft:bread",
            Item::LeatherHelmet => "minecraft:leather_helmet",
            Item::LeatherChestplate => "minecraft:leather_chestplate",
            Item::LeatherLeggings => "minecraft:leather_leggings",
            Item::LeatherBoots => "minecraft:leather_boots",
            Item::ChainmailHelmet => "minecraft:chainmail_helmet",
            Item::ChainmailChestplate => "minecraft:chainmail_chestplate",
            Item::ChainmailLeggings => "minecraft:chainmail_leggings",
            Item::ChainmailBoots => "minecraft:chainmail_boots",
            Item::IronHelmet => "minecraft:iron_helmet",
            Item::IronChestplate => "minecraft:iron_chestplate",
            Item::IronLeggings => "minecraft:iron_leggings",
            Item::IronBoots => "minecraft:iron_boots",
            Item::DiamondHelmet => "minecraft:diamond_helmet",
            Item::DiamondChestplate => "minecraft:diamond_chestplate",
            Item::DiamondLeggings => "minecraft:diamond_leggings",
            Item::DiamondBoots => "minecraft:diamond_boots",
            Item::GoldenHelmet => "minecraft:golden_helmet",
            Item::GoldenChestplate => "minecraft:golden_chestplate",
            Item::GoldenLeggings => "minecraft:golden_leggings",
            Item::GoldenBoots => "minecraft:golden_boots",
            Item::NetheriteHelmet => "minecraft:netherite_helmet",
            Item::NetheriteChestplate => "minecraft:netherite_chestplate",
            Item::NetheriteLeggings => "minecraft:netherite_leggings",
            Item::NetheriteBoots => "minecraft:netherite_boots",
            Item::Flint => "minecraft:flint",
            Item::Porkchop => "minecraft:porkchop",
            Item::CookedPorkchop => "minecraft:cooked_porkchop",
            Item::Painting => "minecraft:painting",
            Item::GoldenApple => "minecraft:golden_apple",
            Item::EnchantedGoldenApple => "minecraft:enchanted_golden_apple",
            Item::OakSign => "minecraft:oak_sign",
            Item::SpruceSign => "minecraft:spruce_sign",
            Item::BirchSign => "minecraft:birch_sign",
            Item::JungleSign => "minecraft:jungle_sign",
            Item::AcaciaSign => "minecraft:acacia_sign",
            Item::DarkOakSign => "minecraft:dark_oak_sign",
            Item::CrimsonSign => "minecraft:crimson_sign",
            Item::WarpedSign => "minecraft:warped_sign",
            Item::Bucket => "minecraft:bucket",
            Item::WaterBucket => "minecraft:water_bucket",
            Item::LavaBucket => "minecraft:lava_bucket",
            Item::PowderSnowBucket => "minecraft:powder_snow_bucket",
            Item::Snowball => "minecraft:snowball",
            Item::Leather => "minecraft:leather",
            Item::MilkBucket => "minecraft:milk_bucket",
            Item::PufferfishBucket => "minecraft:pufferfish_bucket",
            Item::SalmonBucket => "minecraft:salmon_bucket",
            Item::CodBucket => "minecraft:cod_bucket",
            Item::TropicalFishBucket => "minecraft:tropical_fish_bucket",
            Item::AxolotlBucket => "minecraft:axolotl_bucket",
            Item::Brick => "minecraft:brick",
            Item::ClayBall => "minecraft:clay_ball",
            Item::DriedKelpBlock => "minecraft:dried_kelp_block",
            Item::Paper => "minecraft:paper",
            Item::Book => "minecraft:book",
            Item::SlimeBall => "minecraft:slime_ball",
            Item::Egg => "minecraft:egg",
            Item::Compass => "minecraft:compass",
            Item::Bundle => "minecraft:bundle",
            Item::FishingRod => "minecraft:fishing_rod",
            Item::Clock => "minecraft:clock",
            Item::Spyglass => "minecraft:spyglass",
            Item::GlowstoneDust => "minecraft:glowstone_dust",
            Item::Cod => "minecraft:cod",
            Item::Salmon => "minecraft:salmon",
            Item::TropicalFish => "minecraft:tropical_fish",
            Item::Pufferfish => "minecraft:pufferfish",
            Item::CookedCod => "minecraft:cooked_cod",
            Item::CookedSalmon => "minecraft:cooked_salmon",
            Item::InkSac => "minecraft:ink_sac",
            Item::GlowInkSac => "minecraft:glow_ink_sac",
            Item::CocoaBeans => "minecraft:cocoa_beans",
            Item::WhiteDye => "minecraft:white_dye",
            Item::OrangeDye => "minecraft:orange_dye",
            Item::MagentaDye => "minecraft:magenta_dye",
            Item::LightBlueDye => "minecraft:light_blue_dye",
            Item::YellowDye => "minecraft:yellow_dye",
            Item::LimeDye => "minecraft:lime_dye",
            Item::PinkDye => "minecraft:pink_dye",
            Item::GrayDye => "minecraft:gray_dye",
            Item::LightGrayDye => "minecraft:light_gray_dye",
            Item::CyanDye => "minecraft:cyan_dye",
            Item::PurpleDye => "minecraft:purple_dye",
            Item::BlueDye => "minecraft:blue_dye",
            Item::BrownDye => "minecraft:brown_dye",
            Item::GreenDye => "minecraft:green_dye",
            Item::RedDye => "minecraft:red_dye",
            Item::BlackDye => "minecraft:black_dye",
            Item::BoneMeal => "minecraft:bone_meal",
            Item::Bone => "minecraft:bone",
            Item::Sugar => "minecraft:sugar",
            Item::Cake => "minecraft:cake",
            Item::WhiteBed => "minecraft:white_bed",
            Item::OrangeBed => "minecraft:orange_bed",
            Item::MagentaBed => "minecraft:magenta_bed",
            Item::LightBlueBed => "minecraft:light_blue_bed",
            Item::YellowBed => "minecraft:yellow_bed",
            Item::LimeBed => "minecraft:lime_bed",
            Item::PinkBed => "minecraft:pink_bed",
            Item::GrayBed => "minecraft:gray_bed",
            Item::LightGrayBed => "minecraft:light_gray_bed",
            Item::CyanBed => "minecraft:cyan_bed",
            Item::PurpleBed => "minecraft:purple_bed",
            Item::BlueBed => "minecraft:blue_bed",
            Item::BrownBed => "minecraft:brown_bed",
            Item::GreenBed => "minecraft:green_bed",
            Item::RedBed => "minecraft:red_bed",
            Item::BlackBed => "minecraft:black_bed",
            Item::Cookie => "minecraft:cookie",
            Item::FilledMap => "minecraft:filled_map",
            Item::Shears => "minecraft:shears",
            Item::MelonSlice => "minecraft:melon_slice",
            Item::DriedKelp => "minecraft:dried_kelp",
            Item::PumpkinSeeds => "minecraft:pumpkin_seeds",
            Item::MelonSeeds => "minecraft:melon_seeds",
            Item::Beef => "minecraft:beef",
            Item::CookedBeef => "minecraft:cooked_beef",
            Item::Chicken => "minecraft:chicken",
            Item::CookedChicken => "minecraft:cooked_chicken",
            Item::RottenFlesh => "minecraft:rotten_flesh",
            Item::EnderPearl => "minecraft:ender_pearl",
            Item::BlazeRod => "minecraft:blaze_rod",
            Item::GhastTear => "minecraft:ghast_tear",
            Item::GoldNugget => "minecraft:gold_nugget",
            Item::NetherWart => "minecraft:nether_wart",
            Item::Potion => "minecraft:potion",
            Item::GlassBottle => "minecraft:glass_bottle",
            Item::SpiderEye => "minecraft:spider_eye",
            Item::FermentedSpiderEye => "minecraft:fermented_spider_eye",
            Item::BlazePowder => "minecraft:blaze_powder",
            Item::MagmaCream => "minecraft:magma_cream",
            Item::BrewingStand => "minecraft:brewing_stand",
            Item::Cauldron => "minecraft:cauldron",
            Item::EnderEye => "minecraft:ender_eye",
            Item::GlisteringMelonSlice => "minecraft:glistering_melon_slice",
            Item::AxolotlSpawnEgg => "minecraft:axolotl_spawn_egg",
            Item::BatSpawnEgg => "minecraft:bat_spawn_egg",
            Item::BeeSpawnEgg => "minecraft:bee_spawn_egg",
            Item::BlazeSpawnEgg => "minecraft:blaze_spawn_egg",
            Item::CatSpawnEgg => "minecraft:cat_spawn_egg",
            Item::CaveSpiderSpawnEgg => "minecraft:cave_spider_spawn_egg",
            Item::ChickenSpawnEgg => "minecraft:chicken_spawn_egg",
            Item::CodSpawnEgg => "minecraft:cod_spawn_egg",
            Item::CowSpawnEgg => "minecraft:cow_spawn_egg",
            Item::CreeperSpawnEgg => "minecraft:creeper_spawn_egg",
            Item::DolphinSpawnEgg => "minecraft:dolphin_spawn_egg",
            Item::DonkeySpawnEgg => "minecraft:donkey_spawn_egg",
            Item::DrownedSpawnEgg => "minecraft:drowned_spawn_egg",
            Item::ElderGuardianSpawnEgg => "minecraft:elder_guardian_spawn_egg",
            Item::EndermanSpawnEgg => "minecraft:enderman_spawn_egg",
            Item::EndermiteSpawnEgg => "minecraft:endermite_spawn_egg",
            Item::EvokerSpawnEgg => "minecraft:evoker_spawn_egg",
            Item::FoxSpawnEgg => "minecraft:fox_spawn_egg",
            Item::GhastSpawnEgg => "minecraft:ghast_spawn_egg",
            Item::GlowSquidSpawnEgg => "minecraft:glow_squid_spawn_egg",
            Item::GoatSpawnEgg => "minecraft:goat_spawn_egg",
            Item::GuardianSpawnEgg => "minecraft:guardian_spawn_egg",
            Item::HoglinSpawnEgg => "minecraft:hoglin_spawn_egg",
            Item::HorseSpawnEgg => "minecraft:horse_spawn_egg",
            Item::HuskSpawnEgg => "minecraft:husk_spawn_egg",
            Item::LlamaSpawnEgg => "minecraft:llama_spawn_egg",
            Item::MagmaCubeSpawnEgg => "minecraft:magma_cube_spawn_egg",
            Item::MooshroomSpawnEgg => "minecraft:mooshroom_spawn_egg",
            Item::MuleSpawnEgg => "minecraft:mule_spawn_egg",
            Item::OcelotSpawnEgg => "minecraft:ocelot_spawn_egg",
            Item::PandaSpawnEgg => "minecraft:panda_spawn_egg",
            Item::ParrotSpawnEgg => "minecraft:parrot_spawn_egg",
            Item::PhantomSpawnEgg => "minecraft:phantom_spawn_egg",
            Item::PigSpawnEgg => "minecraft:pig_spawn_egg",
            Item::PiglinSpawnEgg => "minecraft:piglin_spawn_egg",
            Item::PiglinBruteSpawnEgg => "minecraft:piglin_brute_spawn_egg",
            Item::PillagerSpawnEgg => "minecraft:pillager_spawn_egg",
            Item::PolarBearSpawnEgg => "minecraft:polar_bear_spawn_egg",
            Item::PufferfishSpawnEgg => "minecraft:pufferfish_spawn_egg",
            Item::RabbitSpawnEgg => "minecraft:rabbit_spawn_egg",
            Item::RavagerSpawnEgg => "minecraft:ravager_spawn_egg",
            Item::SalmonSpawnEgg => "minecraft:salmon_spawn_egg",
            Item::SheepSpawnEgg => "minecraft:sheep_spawn_egg",
            Item::ShulkerSpawnEgg => "minecraft:shulker_spawn_egg",
            Item::SilverfishSpawnEgg => "minecraft:silverfish_spawn_egg",
            Item::SkeletonSpawnEgg => "minecraft:skeleton_spawn_egg",
            Item::SkeletonHorseSpawnEgg => "minecraft:skeleton_horse_spawn_egg",
            Item::SlimeSpawnEgg => "minecraft:slime_spawn_egg",
            Item::SpiderSpawnEgg => "minecraft:spider_spawn_egg",
            Item::SquidSpawnEgg => "minecraft:squid_spawn_egg",
            Item::StraySpawnEgg => "minecraft:stray_spawn_egg",
            Item::StriderSpawnEgg => "minecraft:strider_spawn_egg",
            Item::TraderLlamaSpawnEgg => "minecraft:trader_llama_spawn_egg",
            Item::TropicalFishSpawnEgg => "minecraft:tropical_fish_spawn_egg",
            Item::TurtleSpawnEgg => "minecraft:turtle_spawn_egg",
            Item::VexSpawnEgg => "minecraft:vex_spawn_egg",
            Item::VillagerSpawnEgg => "minecraft:villager_spawn_egg",
            Item::VindicatorSpawnEgg => "minecraft:vindicator_spawn_egg",
            Item::WanderingTraderSpawnEgg => "minecraft:wandering_trader_spawn_egg",
            Item::WitchSpawnEgg => "minecraft:witch_spawn_egg",
            Item::WitherSkeletonSpawnEgg => "minecraft:wither_skeleton_spawn_egg",
            Item::WolfSpawnEgg => "minecraft:wolf_spawn_egg",
            Item::ZoglinSpawnEgg => "minecraft:zoglin_spawn_egg",
            Item::ZombieSpawnEgg => "minecraft:zombie_spawn_egg",
            Item::ZombieHorseSpawnEgg => "minecraft:zombie_horse_spawn_egg",
            Item::ZombieVillagerSpawnEgg => "minecraft:zombie_villager_spawn_egg",
            Item::ZombifiedPiglinSpawnEgg => "minecraft:zombified_piglin_spawn_egg",
            Item::ExperienceBottle => "minecraft:experience_bottle",
            Item::FireCharge => "minecraft:fire_charge",
            Item::WritableBook => "minecraft:writable_book",
            Item::WrittenBook => "minecraft:written_book",
            Item::ItemFrame => "minecraft:item_frame",
            Item::GlowItemFrame => "minecraft:glow_item_frame",
            Item::FlowerPot => "minecraft:flower_pot",
            Item::Carrot => "minecraft:carrot",
            Item::Potato => "minecraft:potato",
            Item::BakedPotato => "minecraft:baked_potato",
            Item::PoisonousPotato => "minecraft:poisonous_potato",
            Item::Map => "minecraft:map",
            Item::GoldenCarrot => "minecraft:golden_carrot",
            Item::SkeletonSkull => "minecraft:skeleton_skull",
            Item::WitherSkeletonSkull => "minecraft:wither_skeleton_skull",
            Item::PlayerHead => "minecraft:player_head",
            Item::ZombieHead => "minecraft:zombie_head",
            Item::CreeperHead => "minecraft:creeper_head",
            Item::DragonHead => "minecraft:dragon_head",
            Item::NetherStar => "minecraft:nether_star",
            Item::PumpkinPie => "minecraft:pumpkin_pie",
            Item::FireworkRocket => "minecraft:firework_rocket",
            Item::FireworkStar => "minecraft:firework_star",
            Item::EnchantedBook => "minecraft:enchanted_book",
            Item::NetherBrick => "minecraft:nether_brick",
            Item::PrismarineShard => "minecraft:prismarine_shard",
            Item::PrismarineCrystals => "minecraft:prismarine_crystals",
            Item::Rabbit => "minecraft:rabbit",
            Item::CookedRabbit => "minecraft:cooked_rabbit",
            Item::RabbitStew => "minecraft:rabbit_stew",
            Item::RabbitFoot => "minecraft:rabbit_foot",
            Item::RabbitHide => "minecraft:rabbit_hide",
            Item::ArmorStand => "minecraft:armor_stand",
            Item::IronHorseArmor => "minecraft:iron_horse_armor",
            Item::GoldenHorseArmor => "minecraft:golden_horse_armor",
            Item::DiamondHorseArmor => "minecraft:diamond_horse_armor",
            Item::LeatherHorseArmor => "minecraft:leather_horse_armor",
            Item::Lead => "minecraft:lead",
            Item::NameTag => "minecraft:name_tag",
            Item::CommandBlockMinecart => "minecraft:command_block_minecart",
            Item::Mutton => "minecraft:mutton",
            Item::CookedMutton => "minecraft:cooked_mutton",
            Item::WhiteBanner => "minecraft:white_banner",
            Item::OrangeBanner => "minecraft:orange_banner",
            Item::MagentaBanner => "minecraft:magenta_banner",
            Item::LightBlueBanner => "minecraft:light_blue_banner",
            Item::YellowBanner => "minecraft:yellow_banner",
            Item::LimeBanner => "minecraft:lime_banner",
            Item::PinkBanner => "minecraft:pink_banner",
            Item::GrayBanner => "minecraft:gray_banner",
            Item::LightGrayBanner => "minecraft:light_gray_banner",
            Item::CyanBanner => "minecraft:cyan_banner",
            Item::PurpleBanner => "minecraft:purple_banner",
            Item::BlueBanner => "minecraft:blue_banner",
            Item::BrownBanner => "minecraft:brown_banner",
            Item::GreenBanner => "minecraft:green_banner",
            Item::RedBanner => "minecraft:red_banner",
            Item::BlackBanner => "minecraft:black_banner",
            Item::EndCrystal => "minecraft:end_crystal",
            Item::ChorusFruit => "minecraft:chorus_fruit",
            Item::PoppedChorusFruit => "minecraft:popped_chorus_fruit",
            Item::Beetroot => "minecraft:beetroot",
            Item::BeetrootSeeds => "minecraft:beetroot_seeds",
            Item::BeetrootSoup => "minecraft:beetroot_soup",
            Item::DragonBreath => "minecraft:dragon_breath",
            Item::SplashPotion => "minecraft:splash_potion",
            Item::SpectralArrow => "minecraft:spectral_arrow",
            Item::TippedArrow => "minecraft:tipped_arrow",
            Item::LingeringPotion => "minecraft:lingering_potion",
            Item::Shield => "minecraft:shield",
            Item::TotemOfUndying => "minecraft:totem_of_undying",
            Item::ShulkerShell => "minecraft:shulker_shell",
            Item::IronNugget => "minecraft:iron_nugget",
            Item::KnowledgeBook => "minecraft:knowledge_book",
            Item::DebugStick => "minecraft:debug_stick",
            Item::MusicDisc13 => "minecraft:music_disc_13",
            Item::MusicDiscCat => "minecraft:music_disc_cat",
            Item::MusicDiscBlocks => "minecraft:music_disc_blocks",
            Item::MusicDiscChirp => "minecraft:music_disc_chirp",
            Item::MusicDiscFar => "minecraft:music_disc_far",
            Item::MusicDiscMall => "minecraft:music_disc_mall",
            Item::MusicDiscMellohi => "minecraft:music_disc_mellohi",
            Item::MusicDiscStal => "minecraft:music_disc_stal",
            Item::MusicDiscStrad => "minecraft:music_disc_strad",
            Item::MusicDiscWard => "minecraft:music_disc_ward",
            Item::MusicDisc11 => "minecraft:music_disc_11",
            Item::MusicDiscWait => "minecraft:music_disc_wait",
            Item::MusicDiscOtherside => "minecraft:music_disc_otherside",
            Item::MusicDiscPigstep => "minecraft:music_disc_pigstep",
            Item::Trident => "minecraft:trident",
            Item::PhantomMembrane => "minecraft:phantom_membrane",
            Item::NautilusShell => "minecraft:nautilus_shell",
            Item::HeartOfTheSea => "minecraft:heart_of_the_sea",
            Item::Crossbow => "minecraft:crossbow",
            Item::SuspiciousStew => "minecraft:suspicious_stew",
            Item::Loom => "minecraft:loom",
            Item::FlowerBannerPattern => "minecraft:flower_banner_pattern",
            Item::CreeperBannerPattern => "minecraft:creeper_banner_pattern",
            Item::SkullBannerPattern => "minecraft:skull_banner_pattern",
            Item::MojangBannerPattern => "minecraft:mojang_banner_pattern",
            Item::GlobeBannerPattern => "minecraft:globe_banner_pattern",
            Item::PiglinBannerPattern => "minecraft:piglin_banner_pattern",
            Item::Composter => "minecraft:composter",
            Item::Barrel => "minecraft:barrel",
            Item::Smoker => "minecraft:smoker",
            Item::BlastFurnace => "minecraft:blast_furnace",
            Item::CartographyTable => "minecraft:cartography_table",
            Item::FletchingTable => "minecraft:fletching_table",
            Item::Grindstone => "minecraft:grindstone",
            Item::SmithingTable => "minecraft:smithing_table",
            Item::Stonecutter => "minecraft:stonecutter",
            Item::Bell => "minecraft:bell",
            Item::Lantern => "minecraft:lantern",
            Item::SoulLantern => "minecraft:soul_lantern",
            Item::SweetBerries => "minecraft:sweet_berries",
            Item::GlowBerries => "minecraft:glow_berries",
            Item::Campfire => "minecraft:campfire",
            Item::SoulCampfire => "minecraft:soul_campfire",
            Item::Shroomlight => "minecraft:shroomlight",
            Item::Honeycomb => "minecraft:honeycomb",
            Item::BeeNest => "minecraft:bee_nest",
            Item::Beehive => "minecraft:beehive",
            Item::HoneyBottle => "minecraft:honey_bottle",
            Item::HoneycombBlock => "minecraft:honeycomb_block",
            Item::Lodestone => "minecraft:lodestone",
            Item::CryingObsidian => "minecraft:crying_obsidian",
            Item::Blackstone => "minecraft:blackstone",
            Item::BlackstoneSlab => "minecraft:blackstone_slab",
            Item::BlackstoneStairs => "minecraft:blackstone_stairs",
            Item::GildedBlackstone => "minecraft:gilded_blackstone",
            Item::PolishedBlackstone => "minecraft:polished_blackstone",
            Item::PolishedBlackstoneSlab => "minecraft:polished_blackstone_slab",
            Item::PolishedBlackstoneStairs => "minecraft:polished_blackstone_stairs",
            Item::ChiseledPolishedBlackstone => "minecraft:chiseled_polished_blackstone",
            Item::PolishedBlackstoneBricks => "minecraft:polished_blackstone_bricks",
            Item::PolishedBlackstoneBrickSlab => "minecraft:polished_blackstone_brick_slab",
            Item::PolishedBlackstoneBrickStairs => "minecraft:polished_blackstone_brick_stairs",
            Item::CrackedPolishedBlackstoneBricks => "minecraft:cracked_polished_blackstone_bricks",
            Item::RespawnAnchor => "minecraft:respawn_anchor",
            Item::Candle => "minecraft:candle",
            Item::WhiteCandle => "minecraft:white_candle",
            Item::OrangeCandle => "minecraft:orange_candle",
            Item::MagentaCandle => "minecraft:magenta_candle",
            Item::LightBlueCandle => "minecraft:light_blue_candle",
            Item::YellowCandle => "minecraft:yellow_candle",
            Item::LimeCandle => "minecraft:lime_candle",
            Item::PinkCandle => "minecraft:pink_candle",
            Item::GrayCandle => "minecraft:gray_candle",
            Item::LightGrayCandle => "minecraft:light_gray_candle",
            Item::CyanCandle => "minecraft:cyan_candle",
            Item::PurpleCandle => "minecraft:purple_candle",
            Item::BlueCandle => "minecraft:blue_candle",
            Item::BrownCandle => "minecraft:brown_candle",
            Item::GreenCandle => "minecraft:green_candle",
            Item::RedCandle => "minecraft:red_candle",
            Item::BlackCandle => "minecraft:black_candle",
            Item::SmallAmethystBud => "minecraft:small_amethyst_bud",
            Item::MediumAmethystBud => "minecraft:medium_amethyst_bud",
            Item::LargeAmethystBud => "minecraft:large_amethyst_bud",
            Item::AmethystCluster => "minecraft:amethyst_cluster",
            Item::PointedDripstone => "minecraft:pointed_dripstone",
        }
    }
    #[doc = "Gets a `Item` by its `namespaced_id`."]
    #[inline]
    pub fn from_namespaced_id(namespaced_id: &str) -> Option<Self> {
        match namespaced_id {
            "minecraft:stone" => Some(Item::Stone),
            "minecraft:granite" => Some(Item::Granite),
            "minecraft:polished_granite" => Some(Item::PolishedGranite),
            "minecraft:diorite" => Some(Item::Diorite),
            "minecraft:polished_diorite" => Some(Item::PolishedDiorite),
            "minecraft:andesite" => Some(Item::Andesite),
            "minecraft:polished_andesite" => Some(Item::PolishedAndesite),
            "minecraft:deepslate" => Some(Item::Deepslate),
            "minecraft:cobbled_deepslate" => Some(Item::CobbledDeepslate),
            "minecraft:polished_deepslate" => Some(Item::PolishedDeepslate),
            "minecraft:calcite" => Some(Item::Calcite),
            "minecraft:tuff" => Some(Item::Tuff),
            "minecraft:dripstone_block" => Some(Item::DripstoneBlock),
            "minecraft:grass_block" => Some(Item::GrassBlock),
            "minecraft:dirt" => Some(Item::Dirt),
            "minecraft:coarse_dirt" => Some(Item::CoarseDirt),
            "minecraft:podzol" => Some(Item::Podzol),
            "minecraft:rooted_dirt" => Some(Item::RootedDirt),
            "minecraft:crimson_nylium" => Some(Item::CrimsonNylium),
            "minecraft:warped_nylium" => Some(Item::WarpedNylium),
            "minecraft:cobblestone" => Some(Item::Cobblestone),
            "minecraft:oak_planks" => Some(Item::OakPlanks),
            "minecraft:spruce_planks" => Some(Item::SprucePlanks),
            "minecraft:birch_planks" => Some(Item::BirchPlanks),
            "minecraft:jungle_planks" => Some(Item::JunglePlanks),
            "minecraft:acacia_planks" => Some(Item::AcaciaPlanks),
            "minecraft:dark_oak_planks" => Some(Item::DarkOakPlanks),
            "minecraft:crimson_planks" => Some(Item::CrimsonPlanks),
            "minecraft:warped_planks" => Some(Item::WarpedPlanks),
            "minecraft:oak_sapling" => Some(Item::OakSapling),
            "minecraft:spruce_sapling" => Some(Item::SpruceSapling),
            "minecraft:birch_sapling" => Some(Item::BirchSapling),
            "minecraft:jungle_sapling" => Some(Item::JungleSapling),
            "minecraft:acacia_sapling" => Some(Item::AcaciaSapling),
            "minecraft:dark_oak_sapling" => Some(Item::DarkOakSapling),
            "minecraft:bedrock" => Some(Item::Bedrock),
            "minecraft:sand" => Some(Item::Sand),
            "minecraft:red_sand" => Some(Item::RedSand),
            "minecraft:gravel" => Some(Item::Gravel),
            "minecraft:coal_ore" => Some(Item::CoalOre),
            "minecraft:deepslate_coal_ore" => Some(Item::DeepslateCoalOre),
            "minecraft:iron_ore" => Some(Item::IronOre),
            "minecraft:deepslate_iron_ore" => Some(Item::DeepslateIronOre),
            "minecraft:copper_ore" => Some(Item::CopperOre),
            "minecraft:deepslate_copper_ore" => Some(Item::DeepslateCopperOre),
            "minecraft:gold_ore" => Some(Item::GoldOre),
            "minecraft:deepslate_gold_ore" => Some(Item::DeepslateGoldOre),
            "minecraft:redstone_ore" => Some(Item::RedstoneOre),
            "minecraft:deepslate_redstone_ore" => Some(Item::DeepslateRedstoneOre),
            "minecraft:emerald_ore" => Some(Item::EmeraldOre),
            "minecraft:deepslate_emerald_ore" => Some(Item::DeepslateEmeraldOre),
            "minecraft:lapis_ore" => Some(Item::LapisOre),
            "minecraft:deepslate_lapis_ore" => Some(Item::DeepslateLapisOre),
            "minecraft:diamond_ore" => Some(Item::DiamondOre),
            "minecraft:deepslate_diamond_ore" => Some(Item::DeepslateDiamondOre),
            "minecraft:nether_gold_ore" => Some(Item::NetherGoldOre),
            "minecraft:nether_quartz_ore" => Some(Item::NetherQuartzOre),
            "minecraft:ancient_debris" => Some(Item::AncientDebris),
            "minecraft:coal_block" => Some(Item::CoalBlock),
            "minecraft:raw_iron_block" => Some(Item::RawIronBlock),
            "minecraft:raw_copper_block" => Some(Item::RawCopperBlock),
            "minecraft:raw_gold_block" => Some(Item::RawGoldBlock),
            "minecraft:amethyst_block" => Some(Item::AmethystBlock),
            "minecraft:budding_amethyst" => Some(Item::BuddingAmethyst),
            "minecraft:iron_block" => Some(Item::IronBlock),
            "minecraft:copper_block" => Some(Item::CopperBlock),
            "minecraft:gold_block" => Some(Item::GoldBlock),
            "minecraft:diamond_block" => Some(Item::DiamondBlock),
            "minecraft:netherite_block" => Some(Item::NetheriteBlock),
            "minecraft:exposed_copper" => Some(Item::ExposedCopper),
            "minecraft:weathered_copper" => Some(Item::WeatheredCopper),
            "minecraft:oxidized_copper" => Some(Item::OxidizedCopper),
            "minecraft:cut_copper" => Some(Item::CutCopper),
            "minecraft:exposed_cut_copper" => Some(Item::ExposedCutCopper),
            "minecraft:weathered_cut_copper" => Some(Item::WeatheredCutCopper),
            "minecraft:oxidized_cut_copper" => Some(Item::OxidizedCutCopper),
            "minecraft:cut_copper_stairs" => Some(Item::CutCopperStairs),
            "minecraft:exposed_cut_copper_stairs" => Some(Item::ExposedCutCopperStairs),
            "minecraft:weathered_cut_copper_stairs" => Some(Item::WeatheredCutCopperStairs),
            "minecraft:oxidized_cut_copper_stairs" => Some(Item::OxidizedCutCopperStairs),
            "minecraft:cut_copper_slab" => Some(Item::CutCopperSlab),
            "minecraft:exposed_cut_copper_slab" => Some(Item::ExposedCutCopperSlab),
            "minecraft:weathered_cut_copper_slab" => Some(Item::WeatheredCutCopperSlab),
            "minecraft:oxidized_cut_copper_slab" => Some(Item::OxidizedCutCopperSlab),
            "minecraft:waxed_copper_block" => Some(Item::WaxedCopperBlock),
            "minecraft:waxed_exposed_copper" => Some(Item::WaxedExposedCopper),
            "minecraft:waxed_weathered_copper" => Some(Item::WaxedWeatheredCopper),
            "minecraft:waxed_oxidized_copper" => Some(Item::WaxedOxidizedCopper),
            "minecraft:waxed_cut_copper" => Some(Item::WaxedCutCopper),
            "minecraft:waxed_exposed_cut_copper" => Some(Item::WaxedExposedCutCopper),
            "minecraft:waxed_weathered_cut_copper" => Some(Item::WaxedWeatheredCutCopper),
            "minecraft:waxed_oxidized_cut_copper" => Some(Item::WaxedOxidizedCutCopper),
            "minecraft:waxed_cut_copper_stairs" => Some(Item::WaxedCutCopperStairs),
            "minecraft:waxed_exposed_cut_copper_stairs" => Some(Item::WaxedExposedCutCopperStairs),
            "minecraft:waxed_weathered_cut_copper_stairs" => {
                Some(Item::WaxedWeatheredCutCopperStairs)
            }
            "minecraft:waxed_oxidized_cut_copper_stairs" => {
                Some(Item::WaxedOxidizedCutCopperStairs)
            }
            "minecraft:waxed_cut_copper_slab" => Some(Item::WaxedCutCopperSlab),
            "minecraft:waxed_exposed_cut_copper_slab" => Some(Item::WaxedExposedCutCopperSlab),
            "minecraft:waxed_weathered_cut_copper_slab" => Some(Item::WaxedWeatheredCutCopperSlab),
            "minecraft:waxed_oxidized_cut_copper_slab" => Some(Item::WaxedOxidizedCutCopperSlab),
            "minecraft:oak_log" => Some(Item::OakLog),
            "minecraft:spruce_log" => Some(Item::SpruceLog),
            "minecraft:birch_log" => Some(Item::BirchLog),
            "minecraft:jungle_log" => Some(Item::JungleLog),
            "minecraft:acacia_log" => Some(Item::AcaciaLog),
            "minecraft:dark_oak_log" => Some(Item::DarkOakLog),
            "minecraft:crimson_stem" => Some(Item::CrimsonStem),
            "minecraft:warped_stem" => Some(Item::WarpedStem),
            "minecraft:stripped_oak_log" => Some(Item::StrippedOakLog),
            "minecraft:stripped_spruce_log" => Some(Item::StrippedSpruceLog),
            "minecraft:stripped_birch_log" => Some(Item::StrippedBirchLog),
            "minecraft:stripped_jungle_log" => Some(Item::StrippedJungleLog),
            "minecraft:stripped_acacia_log" => Some(Item::StrippedAcaciaLog),
            "minecraft:stripped_dark_oak_log" => Some(Item::StrippedDarkOakLog),
            "minecraft:stripped_crimson_stem" => Some(Item::StrippedCrimsonStem),
            "minecraft:stripped_warped_stem" => Some(Item::StrippedWarpedStem),
            "minecraft:stripped_oak_wood" => Some(Item::StrippedOakWood),
            "minecraft:stripped_spruce_wood" => Some(Item::StrippedSpruceWood),
            "minecraft:stripped_birch_wood" => Some(Item::StrippedBirchWood),
            "minecraft:stripped_jungle_wood" => Some(Item::StrippedJungleWood),
            "minecraft:stripped_acacia_wood" => Some(Item::StrippedAcaciaWood),
            "minecraft:stripped_dark_oak_wood" => Some(Item::StrippedDarkOakWood),
            "minecraft:stripped_crimson_hyphae" => Some(Item::StrippedCrimsonHyphae),
            "minecraft:stripped_warped_hyphae" => Some(Item::StrippedWarpedHyphae),
            "minecraft:oak_wood" => Some(Item::OakWood),
            "minecraft:spruce_wood" => Some(Item::SpruceWood),
            "minecraft:birch_wood" => Some(Item::BirchWood),
            "minecraft:jungle_wood" => Some(Item::JungleWood),
            "minecraft:acacia_wood" => Some(Item::AcaciaWood),
            "minecraft:dark_oak_wood" => Some(Item::DarkOakWood),
            "minecraft:crimson_hyphae" => Some(Item::CrimsonHyphae),
            "minecraft:warped_hyphae" => Some(Item::WarpedHyphae),
            "minecraft:oak_leaves" => Some(Item::OakLeaves),
            "minecraft:spruce_leaves" => Some(Item::SpruceLeaves),
            "minecraft:birch_leaves" => Some(Item::BirchLeaves),
            "minecraft:jungle_leaves" => Some(Item::JungleLeaves),
            "minecraft:acacia_leaves" => Some(Item::AcaciaLeaves),
            "minecraft:dark_oak_leaves" => Some(Item::DarkOakLeaves),
            "minecraft:azalea_leaves" => Some(Item::AzaleaLeaves),
            "minecraft:flowering_azalea_leaves" => Some(Item::FloweringAzaleaLeaves),
            "minecraft:sponge" => Some(Item::Sponge),
            "minecraft:wet_sponge" => Some(Item::WetSponge),
            "minecraft:glass" => Some(Item::Glass),
            "minecraft:tinted_glass" => Some(Item::TintedGlass),
            "minecraft:lapis_block" => Some(Item::LapisBlock),
            "minecraft:sandstone" => Some(Item::Sandstone),
            "minecraft:chiseled_sandstone" => Some(Item::ChiseledSandstone),
            "minecraft:cut_sandstone" => Some(Item::CutSandstone),
            "minecraft:cobweb" => Some(Item::Cobweb),
            "minecraft:grass" => Some(Item::Grass),
            "minecraft:fern" => Some(Item::Fern),
            "minecraft:azalea" => Some(Item::Azalea),
            "minecraft:flowering_azalea" => Some(Item::FloweringAzalea),
            "minecraft:dead_bush" => Some(Item::DeadBush),
            "minecraft:seagrass" => Some(Item::Seagrass),
            "minecraft:sea_pickle" => Some(Item::SeaPickle),
            "minecraft:white_wool" => Some(Item::WhiteWool),
            "minecraft:orange_wool" => Some(Item::OrangeWool),
            "minecraft:magenta_wool" => Some(Item::MagentaWool),
            "minecraft:light_blue_wool" => Some(Item::LightBlueWool),
            "minecraft:yellow_wool" => Some(Item::YellowWool),
            "minecraft:lime_wool" => Some(Item::LimeWool),
            "minecraft:pink_wool" => Some(Item::PinkWool),
            "minecraft:gray_wool" => Some(Item::GrayWool),
            "minecraft:light_gray_wool" => Some(Item::LightGrayWool),
            "minecraft:cyan_wool" => Some(Item::CyanWool),
            "minecraft:purple_wool" => Some(Item::PurpleWool),
            "minecraft:blue_wool" => Some(Item::BlueWool),
            "minecraft:brown_wool" => Some(Item::BrownWool),
            "minecraft:green_wool" => Some(Item::GreenWool),
            "minecraft:red_wool" => Some(Item::RedWool),
            "minecraft:black_wool" => Some(Item::BlackWool),
            "minecraft:dandelion" => Some(Item::Dandelion),
            "minecraft:poppy" => Some(Item::Poppy),
            "minecraft:blue_orchid" => Some(Item::BlueOrchid),
            "minecraft:allium" => Some(Item::Allium),
            "minecraft:azure_bluet" => Some(Item::AzureBluet),
            "minecraft:red_tulip" => Some(Item::RedTulip),
            "minecraft:orange_tulip" => Some(Item::OrangeTulip),
            "minecraft:white_tulip" => Some(Item::WhiteTulip),
            "minecraft:pink_tulip" => Some(Item::PinkTulip),
            "minecraft:oxeye_daisy" => Some(Item::OxeyeDaisy),
            "minecraft:cornflower" => Some(Item::Cornflower),
            "minecraft:lily_of_the_valley" => Some(Item::LilyOfTheValley),
            "minecraft:wither_rose" => Some(Item::WitherRose),
            "minecraft:spore_blossom" => Some(Item::SporeBlossom),
            "minecraft:brown_mushroom" => Some(Item::BrownMushroom),
            "minecraft:red_mushroom" => Some(Item::RedMushroom),
            "minecraft:crimson_fungus" => Some(Item::CrimsonFungus),
            "minecraft:warped_fungus" => Some(Item::WarpedFungus),
            "minecraft:crimson_roots" => Some(Item::CrimsonRoots),
            "minecraft:warped_roots" => Some(Item::WarpedRoots),
            "minecraft:nether_sprouts" => Some(Item::NetherSprouts),
            "minecraft:weeping_vines" => Some(Item::WeepingVines),
            "minecraft:twisting_vines" => Some(Item::TwistingVines),
            "minecraft:sugar_cane" => Some(Item::SugarCane),
            "minecraft:kelp" => Some(Item::Kelp),
            "minecraft:moss_carpet" => Some(Item::MossCarpet),
            "minecraft:moss_block" => Some(Item::MossBlock),
            "minecraft:hanging_roots" => Some(Item::HangingRoots),
            "minecraft:big_dripleaf" => Some(Item::BigDripleaf),
            "minecraft:small_dripleaf" => Some(Item::SmallDripleaf),
            "minecraft:bamboo" => Some(Item::Bamboo),
            "minecraft:oak_slab" => Some(Item::OakSlab),
            "minecraft:spruce_slab" => Some(Item::SpruceSlab),
            "minecraft:birch_slab" => Some(Item::BirchSlab),
            "minecraft:jungle_slab" => Some(Item::JungleSlab),
            "minecraft:acacia_slab" => Some(Item::AcaciaSlab),
            "minecraft:dark_oak_slab" => Some(Item::DarkOakSlab),
            "minecraft:crimson_slab" => Some(Item::CrimsonSlab),
            "minecraft:warped_slab" => Some(Item::WarpedSlab),
            "minecraft:stone_slab" => Some(Item::StoneSlab),
            "minecraft:smooth_stone_slab" => Some(Item::SmoothStoneSlab),
            "minecraft:sandstone_slab" => Some(Item::SandstoneSlab),
            "minecraft:cut_sandstone_slab" => Some(Item::CutSandstoneSlab),
            "minecraft:petrified_oak_slab" => Some(Item::PetrifiedOakSlab),
            "minecraft:cobblestone_slab" => Some(Item::CobblestoneSlab),
            "minecraft:brick_slab" => Some(Item::BrickSlab),
            "minecraft:stone_brick_slab" => Some(Item::StoneBrickSlab),
            "minecraft:nether_brick_slab" => Some(Item::NetherBrickSlab),
            "minecraft:quartz_slab" => Some(Item::QuartzSlab),
            "minecraft:red_sandstone_slab" => Some(Item::RedSandstoneSlab),
            "minecraft:cut_red_sandstone_slab" => Some(Item::CutRedSandstoneSlab),
            "minecraft:purpur_slab" => Some(Item::PurpurSlab),
            "minecraft:prismarine_slab" => Some(Item::PrismarineSlab),
            "minecraft:prismarine_brick_slab" => Some(Item::PrismarineBrickSlab),
            "minecraft:dark_prismarine_slab" => Some(Item::DarkPrismarineSlab),
            "minecraft:smooth_quartz" => Some(Item::SmoothQuartz),
            "minecraft:smooth_red_sandstone" => Some(Item::SmoothRedSandstone),
            "minecraft:smooth_sandstone" => Some(Item::SmoothSandstone),
            "minecraft:smooth_stone" => Some(Item::SmoothStone),
            "minecraft:bricks" => Some(Item::Bricks),
            "minecraft:bookshelf" => Some(Item::Bookshelf),
            "minecraft:mossy_cobblestone" => Some(Item::MossyCobblestone),
            "minecraft:obsidian" => Some(Item::Obsidian),
            "minecraft:torch" => Some(Item::Torch),
            "minecraft:end_rod" => Some(Item::EndRod),
            "minecraft:chorus_plant" => Some(Item::ChorusPlant),
            "minecraft:chorus_flower" => Some(Item::ChorusFlower),
            "minecraft:purpur_block" => Some(Item::PurpurBlock),
            "minecraft:purpur_pillar" => Some(Item::PurpurPillar),
            "minecraft:purpur_stairs" => Some(Item::PurpurStairs),
            "minecraft:spawner" => Some(Item::Spawner),
            "minecraft:oak_stairs" => Some(Item::OakStairs),
            "minecraft:chest" => Some(Item::Chest),
            "minecraft:crafting_table" => Some(Item::CraftingTable),
            "minecraft:farmland" => Some(Item::Farmland),
            "minecraft:furnace" => Some(Item::Furnace),
            "minecraft:ladder" => Some(Item::Ladder),
            "minecraft:cobblestone_stairs" => Some(Item::CobblestoneStairs),
            "minecraft:snow" => Some(Item::Snow),
            "minecraft:ice" => Some(Item::Ice),
            "minecraft:snow_block" => Some(Item::SnowBlock),
            "minecraft:cactus" => Some(Item::Cactus),
            "minecraft:clay" => Some(Item::Clay),
            "minecraft:jukebox" => Some(Item::Jukebox),
            "minecraft:oak_fence" => Some(Item::OakFence),
            "minecraft:spruce_fence" => Some(Item::SpruceFence),
            "minecraft:birch_fence" => Some(Item::BirchFence),
            "minecraft:jungle_fence" => Some(Item::JungleFence),
            "minecraft:acacia_fence" => Some(Item::AcaciaFence),
            "minecraft:dark_oak_fence" => Some(Item::DarkOakFence),
            "minecraft:crimson_fence" => Some(Item::CrimsonFence),
            "minecraft:warped_fence" => Some(Item::WarpedFence),
            "minecraft:pumpkin" => Some(Item::Pumpkin),
            "minecraft:carved_pumpkin" => Some(Item::CarvedPumpkin),
            "minecraft:jack_o_lantern" => Some(Item::JackOLantern),
            "minecraft:netherrack" => Some(Item::Netherrack),
            "minecraft:soul_sand" => Some(Item::SoulSand),
            "minecraft:soul_soil" => Some(Item::SoulSoil),
            "minecraft:basalt" => Some(Item::Basalt),
            "minecraft:polished_basalt" => Some(Item::PolishedBasalt),
            "minecraft:smooth_basalt" => Some(Item::SmoothBasalt),
            "minecraft:soul_torch" => Some(Item::SoulTorch),
            "minecraft:glowstone" => Some(Item::Glowstone),
            "minecraft:infested_stone" => Some(Item::InfestedStone),
            "minecraft:infested_cobblestone" => Some(Item::InfestedCobblestone),
            "minecraft:infested_stone_bricks" => Some(Item::InfestedStoneBricks),
            "minecraft:infested_mossy_stone_bricks" => Some(Item::InfestedMossyStoneBricks),
            "minecraft:infested_cracked_stone_bricks" => Some(Item::InfestedCrackedStoneBricks),
            "minecraft:infested_chiseled_stone_bricks" => Some(Item::InfestedChiseledStoneBricks),
            "minecraft:infested_deepslate" => Some(Item::InfestedDeepslate),
            "minecraft:stone_bricks" => Some(Item::StoneBricks),
            "minecraft:mossy_stone_bricks" => Some(Item::MossyStoneBricks),
            "minecraft:cracked_stone_bricks" => Some(Item::CrackedStoneBricks),
            "minecraft:chiseled_stone_bricks" => Some(Item::ChiseledStoneBricks),
            "minecraft:deepslate_bricks" => Some(Item::DeepslateBricks),
            "minecraft:cracked_deepslate_bricks" => Some(Item::CrackedDeepslateBricks),
            "minecraft:deepslate_tiles" => Some(Item::DeepslateTiles),
            "minecraft:cracked_deepslate_tiles" => Some(Item::CrackedDeepslateTiles),
            "minecraft:chiseled_deepslate" => Some(Item::ChiseledDeepslate),
            "minecraft:brown_mushroom_block" => Some(Item::BrownMushroomBlock),
            "minecraft:red_mushroom_block" => Some(Item::RedMushroomBlock),
            "minecraft:mushroom_stem" => Some(Item::MushroomStem),
            "minecraft:iron_bars" => Some(Item::IronBars),
            "minecraft:chain" => Some(Item::Chain),
            "minecraft:glass_pane" => Some(Item::GlassPane),
            "minecraft:melon" => Some(Item::Melon),
            "minecraft:vine" => Some(Item::Vine),
            "minecraft:glow_lichen" => Some(Item::GlowLichen),
            "minecraft:brick_stairs" => Some(Item::BrickStairs),
            "minecraft:stone_brick_stairs" => Some(Item::StoneBrickStairs),
            "minecraft:mycelium" => Some(Item::Mycelium),
            "minecraft:lily_pad" => Some(Item::LilyPad),
            "minecraft:nether_bricks" => Some(Item::NetherBricks),
            "minecraft:cracked_nether_bricks" => Some(Item::CrackedNetherBricks),
            "minecraft:chiseled_nether_bricks" => Some(Item::ChiseledNetherBricks),
            "minecraft:nether_brick_fence" => Some(Item::NetherBrickFence),
            "minecraft:nether_brick_stairs" => Some(Item::NetherBrickStairs),
            "minecraft:enchanting_table" => Some(Item::EnchantingTable),
            "minecraft:end_portal_frame" => Some(Item::EndPortalFrame),
            "minecraft:end_stone" => Some(Item::EndStone),
            "minecraft:end_stone_bricks" => Some(Item::EndStoneBricks),
            "minecraft:dragon_egg" => Some(Item::DragonEgg),
            "minecraft:sandstone_stairs" => Some(Item::SandstoneStairs),
            "minecraft:ender_chest" => Some(Item::EnderChest),
            "minecraft:emerald_block" => Some(Item::EmeraldBlock),
            "minecraft:spruce_stairs" => Some(Item::SpruceStairs),
            "minecraft:birch_stairs" => Some(Item::BirchStairs),
            "minecraft:jungle_stairs" => Some(Item::JungleStairs),
            "minecraft:crimson_stairs" => Some(Item::CrimsonStairs),
            "minecraft:warped_stairs" => Some(Item::WarpedStairs),
            "minecraft:command_block" => Some(Item::CommandBlock),
            "minecraft:beacon" => Some(Item::Beacon),
            "minecraft:cobblestone_wall" => Some(Item::CobblestoneWall),
            "minecraft:mossy_cobblestone_wall" => Some(Item::MossyCobblestoneWall),
            "minecraft:brick_wall" => Some(Item::BrickWall),
            "minecraft:prismarine_wall" => Some(Item::PrismarineWall),
            "minecraft:red_sandstone_wall" => Some(Item::RedSandstoneWall),
            "minecraft:mossy_stone_brick_wall" => Some(Item::MossyStoneBrickWall),
            "minecraft:granite_wall" => Some(Item::GraniteWall),
            "minecraft:stone_brick_wall" => Some(Item::StoneBrickWall),
            "minecraft:nether_brick_wall" => Some(Item::NetherBrickWall),
            "minecraft:andesite_wall" => Some(Item::AndesiteWall),
            "minecraft:red_nether_brick_wall" => Some(Item::RedNetherBrickWall),
            "minecraft:sandstone_wall" => Some(Item::SandstoneWall),
            "minecraft:end_stone_brick_wall" => Some(Item::EndStoneBrickWall),
            "minecraft:diorite_wall" => Some(Item::DioriteWall),
            "minecraft:blackstone_wall" => Some(Item::BlackstoneWall),
            "minecraft:polished_blackstone_wall" => Some(Item::PolishedBlackstoneWall),
            "minecraft:polished_blackstone_brick_wall" => Some(Item::PolishedBlackstoneBrickWall),
            "minecraft:cobbled_deepslate_wall" => Some(Item::CobbledDeepslateWall),
            "minecraft:polished_deepslate_wall" => Some(Item::PolishedDeepslateWall),
            "minecraft:deepslate_brick_wall" => Some(Item::DeepslateBrickWall),
            "minecraft:deepslate_tile_wall" => Some(Item::DeepslateTileWall),
            "minecraft:anvil" => Some(Item::Anvil),
            "minecraft:chipped_anvil" => Some(Item::ChippedAnvil),
            "minecraft:damaged_anvil" => Some(Item::DamagedAnvil),
            "minecraft:chiseled_quartz_block" => Some(Item::ChiseledQuartzBlock),
            "minecraft:quartz_block" => Some(Item::QuartzBlock),
            "minecraft:quartz_bricks" => Some(Item::QuartzBricks),
            "minecraft:quartz_pillar" => Some(Item::QuartzPillar),
            "minecraft:quartz_stairs" => Some(Item::QuartzStairs),
            "minecraft:white_terracotta" => Some(Item::WhiteTerracotta),
            "minecraft:orange_terracotta" => Some(Item::OrangeTerracotta),
            "minecraft:magenta_terracotta" => Some(Item::MagentaTerracotta),
            "minecraft:light_blue_terracotta" => Some(Item::LightBlueTerracotta),
            "minecraft:yellow_terracotta" => Some(Item::YellowTerracotta),
            "minecraft:lime_terracotta" => Some(Item::LimeTerracotta),
            "minecraft:pink_terracotta" => Some(Item::PinkTerracotta),
            "minecraft:gray_terracotta" => Some(Item::GrayTerracotta),
            "minecraft:light_gray_terracotta" => Some(Item::LightGrayTerracotta),
            "minecraft:cyan_terracotta" => Some(Item::CyanTerracotta),
            "minecraft:purple_terracotta" => Some(Item::PurpleTerracotta),
            "minecraft:blue_terracotta" => Some(Item::BlueTerracotta),
            "minecraft:brown_terracotta" => Some(Item::BrownTerracotta),
            "minecraft:green_terracotta" => Some(Item::GreenTerracotta),
            "minecraft:red_terracotta" => Some(Item::RedTerracotta),
            "minecraft:black_terracotta" => Some(Item::BlackTerracotta),
            "minecraft:barrier" => Some(Item::Barrier),
            "minecraft:light" => Some(Item::Light),
            "minecraft:hay_block" => Some(Item::HayBlock),
            "minecraft:white_carpet" => Some(Item::WhiteCarpet),
            "minecraft:orange_carpet" => Some(Item::OrangeCarpet),
            "minecraft:magenta_carpet" => Some(Item::MagentaCarpet),
            "minecraft:light_blue_carpet" => Some(Item::LightBlueCarpet),
            "minecraft:yellow_carpet" => Some(Item::YellowCarpet),
            "minecraft:lime_carpet" => Some(Item::LimeCarpet),
            "minecraft:pink_carpet" => Some(Item::PinkCarpet),
            "minecraft:gray_carpet" => Some(Item::GrayCarpet),
            "minecraft:light_gray_carpet" => Some(Item::LightGrayCarpet),
            "minecraft:cyan_carpet" => Some(Item::CyanCarpet),
            "minecraft:purple_carpet" => Some(Item::PurpleCarpet),
            "minecraft:blue_carpet" => Some(Item::BlueCarpet),
            "minecraft:brown_carpet" => Some(Item::BrownCarpet),
            "minecraft:green_carpet" => Some(Item::GreenCarpet),
            "minecraft:red_carpet" => Some(Item::RedCarpet),
            "minecraft:black_carpet" => Some(Item::BlackCarpet),
            "minecraft:terracotta" => Some(Item::Terracotta),
            "minecraft:packed_ice" => Some(Item::PackedIce),
            "minecraft:acacia_stairs" => Some(Item::AcaciaStairs),
            "minecraft:dark_oak_stairs" => Some(Item::DarkOakStairs),
            "minecraft:dirt_path" => Some(Item::DirtPath),
            "minecraft:sunflower" => Some(Item::Sunflower),
            "minecraft:lilac" => Some(Item::Lilac),
            "minecraft:rose_bush" => Some(Item::RoseBush),
            "minecraft:peony" => Some(Item::Peony),
            "minecraft:tall_grass" => Some(Item::TallGrass),
            "minecraft:large_fern" => Some(Item::LargeFern),
            "minecraft:white_stained_glass" => Some(Item::WhiteStainedGlass),
            "minecraft:orange_stained_glass" => Some(Item::OrangeStainedGlass),
            "minecraft:magenta_stained_glass" => Some(Item::MagentaStainedGlass),
            "minecraft:light_blue_stained_glass" => Some(Item::LightBlueStainedGlass),
            "minecraft:yellow_stained_glass" => Some(Item::YellowStainedGlass),
            "minecraft:lime_stained_glass" => Some(Item::LimeStainedGlass),
            "minecraft:pink_stained_glass" => Some(Item::PinkStainedGlass),
            "minecraft:gray_stained_glass" => Some(Item::GrayStainedGlass),
            "minecraft:light_gray_stained_glass" => Some(Item::LightGrayStainedGlass),
            "minecraft:cyan_stained_glass" => Some(Item::CyanStainedGlass),
            "minecraft:purple_stained_glass" => Some(Item::PurpleStainedGlass),
            "minecraft:blue_stained_glass" => Some(Item::BlueStainedGlass),
            "minecraft:brown_stained_glass" => Some(Item::BrownStainedGlass),
            "minecraft:green_stained_glass" => Some(Item::GreenStainedGlass),
            "minecraft:red_stained_glass" => Some(Item::RedStainedGlass),
            "minecraft:black_stained_glass" => Some(Item::BlackStainedGlass),
            "minecraft:white_stained_glass_pane" => Some(Item::WhiteStainedGlassPane),
            "minecraft:orange_stained_glass_pane" => Some(Item::OrangeStainedGlassPane),
            "minecraft:magenta_stained_glass_pane" => Some(Item::MagentaStainedGlassPane),
            "minecraft:light_blue_stained_glass_pane" => Some(Item::LightBlueStainedGlassPane),
            "minecraft:yellow_stained_glass_pane" => Some(Item::YellowStainedGlassPane),
            "minecraft:lime_stained_glass_pane" => Some(Item::LimeStainedGlassPane),
            "minecraft:pink_stained_glass_pane" => Some(Item::PinkStainedGlassPane),
            "minecraft:gray_stained_glass_pane" => Some(Item::GrayStainedGlassPane),
            "minecraft:light_gray_stained_glass_pane" => Some(Item::LightGrayStainedGlassPane),
            "minecraft:cyan_stained_glass_pane" => Some(Item::CyanStainedGlassPane),
            "minecraft:purple_stained_glass_pane" => Some(Item::PurpleStainedGlassPane),
            "minecraft:blue_stained_glass_pane" => Some(Item::BlueStainedGlassPane),
            "minecraft:brown_stained_glass_pane" => Some(Item::BrownStainedGlassPane),
            "minecraft:green_stained_glass_pane" => Some(Item::GreenStainedGlassPane),
            "minecraft:red_stained_glass_pane" => Some(Item::RedStainedGlassPane),
            "minecraft:black_stained_glass_pane" => Some(Item::BlackStainedGlassPane),
            "minecraft:prismarine" => Some(Item::Prismarine),
            "minecraft:prismarine_bricks" => Some(Item::PrismarineBricks),
            "minecraft:dark_prismarine" => Some(Item::DarkPrismarine),
            "minecraft:prismarine_stairs" => Some(Item::PrismarineStairs),
            "minecraft:prismarine_brick_stairs" => Some(Item::PrismarineBrickStairs),
            "minecraft:dark_prismarine_stairs" => Some(Item::DarkPrismarineStairs),
            "minecraft:sea_lantern" => Some(Item::SeaLantern),
            "minecraft:red_sandstone" => Some(Item::RedSandstone),
            "minecraft:chiseled_red_sandstone" => Some(Item::ChiseledRedSandstone),
            "minecraft:cut_red_sandstone" => Some(Item::CutRedSandstone),
            "minecraft:red_sandstone_stairs" => Some(Item::RedSandstoneStairs),
            "minecraft:repeating_command_block" => Some(Item::RepeatingCommandBlock),
            "minecraft:chain_command_block" => Some(Item::ChainCommandBlock),
            "minecraft:magma_block" => Some(Item::MagmaBlock),
            "minecraft:nether_wart_block" => Some(Item::NetherWartBlock),
            "minecraft:warped_wart_block" => Some(Item::WarpedWartBlock),
            "minecraft:red_nether_bricks" => Some(Item::RedNetherBricks),
            "minecraft:bone_block" => Some(Item::BoneBlock),
            "minecraft:structure_void" => Some(Item::StructureVoid),
            "minecraft:shulker_box" => Some(Item::ShulkerBox),
            "minecraft:white_shulker_box" => Some(Item::WhiteShulkerBox),
            "minecraft:orange_shulker_box" => Some(Item::OrangeShulkerBox),
            "minecraft:magenta_shulker_box" => Some(Item::MagentaShulkerBox),
            "minecraft:light_blue_shulker_box" => Some(Item::LightBlueShulkerBox),
            "minecraft:yellow_shulker_box" => Some(Item::YellowShulkerBox),
            "minecraft:lime_shulker_box" => Some(Item::LimeShulkerBox),
            "minecraft:pink_shulker_box" => Some(Item::PinkShulkerBox),
            "minecraft:gray_shulker_box" => Some(Item::GrayShulkerBox),
            "minecraft:light_gray_shulker_box" => Some(Item::LightGrayShulkerBox),
            "minecraft:cyan_shulker_box" => Some(Item::CyanShulkerBox),
            "minecraft:purple_shulker_box" => Some(Item::PurpleShulkerBox),
            "minecraft:blue_shulker_box" => Some(Item::BlueShulkerBox),
            "minecraft:brown_shulker_box" => Some(Item::BrownShulkerBox),
            "minecraft:green_shulker_box" => Some(Item::GreenShulkerBox),
            "minecraft:red_shulker_box" => Some(Item::RedShulkerBox),
            "minecraft:black_shulker_box" => Some(Item::BlackShulkerBox),
            "minecraft:white_glazed_terracotta" => Some(Item::WhiteGlazedTerracotta),
            "minecraft:orange_glazed_terracotta" => Some(Item::OrangeGlazedTerracotta),
            "minecraft:magenta_glazed_terracotta" => Some(Item::MagentaGlazedTerracotta),
            "minecraft:light_blue_glazed_terracotta" => Some(Item::LightBlueGlazedTerracotta),
            "minecraft:yellow_glazed_terracotta" => Some(Item::YellowGlazedTerracotta),
            "minecraft:lime_glazed_terracotta" => Some(Item::LimeGlazedTerracotta),
            "minecraft:pink_glazed_terracotta" => Some(Item::PinkGlazedTerracotta),
            "minecraft:gray_glazed_terracotta" => Some(Item::GrayGlazedTerracotta),
            "minecraft:light_gray_glazed_terracotta" => Some(Item::LightGrayGlazedTerracotta),
            "minecraft:cyan_glazed_terracotta" => Some(Item::CyanGlazedTerracotta),
            "minecraft:purple_glazed_terracotta" => Some(Item::PurpleGlazedTerracotta),
            "minecraft:blue_glazed_terracotta" => Some(Item::BlueGlazedTerracotta),
            "minecraft:brown_glazed_terracotta" => Some(Item::BrownGlazedTerracotta),
            "minecraft:green_glazed_terracotta" => Some(Item::GreenGlazedTerracotta),
            "minecraft:red_glazed_terracotta" => Some(Item::RedGlazedTerracotta),
            "minecraft:black_glazed_terracotta" => Some(Item::BlackGlazedTerracotta),
            "minecraft:white_concrete" => Some(Item::WhiteConcrete),
            "minecraft:orange_concrete" => Some(Item::OrangeConcrete),
            "minecraft:magenta_concrete" => Some(Item::MagentaConcrete),
            "minecraft:light_blue_concrete" => Some(Item::LightBlueConcrete),
            "minecraft:yellow_concrete" => Some(Item::YellowConcrete),
            "minecraft:lime_concrete" => Some(Item::LimeConcrete),
            "minecraft:pink_concrete" => Some(Item::PinkConcrete),
            "minecraft:gray_concrete" => Some(Item::GrayConcrete),
            "minecraft:light_gray_concrete" => Some(Item::LightGrayConcrete),
            "minecraft:cyan_concrete" => Some(Item::CyanConcrete),
            "minecraft:purple_concrete" => Some(Item::PurpleConcrete),
            "minecraft:blue_concrete" => Some(Item::BlueConcrete),
            "minecraft:brown_concrete" => Some(Item::BrownConcrete),
            "minecraft:green_concrete" => Some(Item::GreenConcrete),
            "minecraft:red_concrete" => Some(Item::RedConcrete),
            "minecraft:black_concrete" => Some(Item::BlackConcrete),
            "minecraft:white_concrete_powder" => Some(Item::WhiteConcretePowder),
            "minecraft:orange_concrete_powder" => Some(Item::OrangeConcretePowder),
            "minecraft:magenta_concrete_powder" => Some(Item::MagentaConcretePowder),
            "minecraft:light_blue_concrete_powder" => Some(Item::LightBlueConcretePowder),
            "minecraft:yellow_concrete_powder" => Some(Item::YellowConcretePowder),
            "minecraft:lime_concrete_powder" => Some(Item::LimeConcretePowder),
            "minecraft:pink_concrete_powder" => Some(Item::PinkConcretePowder),
            "minecraft:gray_concrete_powder" => Some(Item::GrayConcretePowder),
            "minecraft:light_gray_concrete_powder" => Some(Item::LightGrayConcretePowder),
            "minecraft:cyan_concrete_powder" => Some(Item::CyanConcretePowder),
            "minecraft:purple_concrete_powder" => Some(Item::PurpleConcretePowder),
            "minecraft:blue_concrete_powder" => Some(Item::BlueConcretePowder),
            "minecraft:brown_concrete_powder" => Some(Item::BrownConcretePowder),
            "minecraft:green_concrete_powder" => Some(Item::GreenConcretePowder),
            "minecraft:red_concrete_powder" => Some(Item::RedConcretePowder),
            "minecraft:black_concrete_powder" => Some(Item::BlackConcretePowder),
            "minecraft:turtle_egg" => Some(Item::TurtleEgg),
            "minecraft:dead_tube_coral_block" => Some(Item::DeadTubeCoralBlock),
            "minecraft:dead_brain_coral_block" => Some(Item::DeadBrainCoralBlock),
            "minecraft:dead_bubble_coral_block" => Some(Item::DeadBubbleCoralBlock),
            "minecraft:dead_fire_coral_block" => Some(Item::DeadFireCoralBlock),
            "minecraft:dead_horn_coral_block" => Some(Item::DeadHornCoralBlock),
            "minecraft:tube_coral_block" => Some(Item::TubeCoralBlock),
            "minecraft:brain_coral_block" => Some(Item::BrainCoralBlock),
            "minecraft:bubble_coral_block" => Some(Item::BubbleCoralBlock),
            "minecraft:fire_coral_block" => Some(Item::FireCoralBlock),
            "minecraft:horn_coral_block" => Some(Item::HornCoralBlock),
            "minecraft:tube_coral" => Some(Item::TubeCoral),
            "minecraft:brain_coral" => Some(Item::BrainCoral),
            "minecraft:bubble_coral" => Some(Item::BubbleCoral),
            "minecraft:fire_coral" => Some(Item::FireCoral),
            "minecraft:horn_coral" => Some(Item::HornCoral),
            "minecraft:dead_brain_coral" => Some(Item::DeadBrainCoral),
            "minecraft:dead_bubble_coral" => Some(Item::DeadBubbleCoral),
            "minecraft:dead_fire_coral" => Some(Item::DeadFireCoral),
            "minecraft:dead_horn_coral" => Some(Item::DeadHornCoral),
            "minecraft:dead_tube_coral" => Some(Item::DeadTubeCoral),
            "minecraft:tube_coral_fan" => Some(Item::TubeCoralFan),
            "minecraft:brain_coral_fan" => Some(Item::BrainCoralFan),
            "minecraft:bubble_coral_fan" => Some(Item::BubbleCoralFan),
            "minecraft:fire_coral_fan" => Some(Item::FireCoralFan),
            "minecraft:horn_coral_fan" => Some(Item::HornCoralFan),
            "minecraft:dead_tube_coral_fan" => Some(Item::DeadTubeCoralFan),
            "minecraft:dead_brain_coral_fan" => Some(Item::DeadBrainCoralFan),
            "minecraft:dead_bubble_coral_fan" => Some(Item::DeadBubbleCoralFan),
            "minecraft:dead_fire_coral_fan" => Some(Item::DeadFireCoralFan),
            "minecraft:dead_horn_coral_fan" => Some(Item::DeadHornCoralFan),
            "minecraft:blue_ice" => Some(Item::BlueIce),
            "minecraft:conduit" => Some(Item::Conduit),
            "minecraft:polished_granite_stairs" => Some(Item::PolishedGraniteStairs),
            "minecraft:smooth_red_sandstone_stairs" => Some(Item::SmoothRedSandstoneStairs),
            "minecraft:mossy_stone_brick_stairs" => Some(Item::MossyStoneBrickStairs),
            "minecraft:polished_diorite_stairs" => Some(Item::PolishedDioriteStairs),
            "minecraft:mossy_cobblestone_stairs" => Some(Item::MossyCobblestoneStairs),
            "minecraft:end_stone_brick_stairs" => Some(Item::EndStoneBrickStairs),
            "minecraft:stone_stairs" => Some(Item::StoneStairs),
            "minecraft:smooth_sandstone_stairs" => Some(Item::SmoothSandstoneStairs),
            "minecraft:smooth_quartz_stairs" => Some(Item::SmoothQuartzStairs),
            "minecraft:granite_stairs" => Some(Item::GraniteStairs),
            "minecraft:andesite_stairs" => Some(Item::AndesiteStairs),
            "minecraft:red_nether_brick_stairs" => Some(Item::RedNetherBrickStairs),
            "minecraft:polished_andesite_stairs" => Some(Item::PolishedAndesiteStairs),
            "minecraft:diorite_stairs" => Some(Item::DioriteStairs),
            "minecraft:cobbled_deepslate_stairs" => Some(Item::CobbledDeepslateStairs),
            "minecraft:polished_deepslate_stairs" => Some(Item::PolishedDeepslateStairs),
            "minecraft:deepslate_brick_stairs" => Some(Item::DeepslateBrickStairs),
            "minecraft:deepslate_tile_stairs" => Some(Item::DeepslateTileStairs),
            "minecraft:polished_granite_slab" => Some(Item::PolishedGraniteSlab),
            "minecraft:smooth_red_sandstone_slab" => Some(Item::SmoothRedSandstoneSlab),
            "minecraft:mossy_stone_brick_slab" => Some(Item::MossyStoneBrickSlab),
            "minecraft:polished_diorite_slab" => Some(Item::PolishedDioriteSlab),
            "minecraft:mossy_cobblestone_slab" => Some(Item::MossyCobblestoneSlab),
            "minecraft:end_stone_brick_slab" => Some(Item::EndStoneBrickSlab),
            "minecraft:smooth_sandstone_slab" => Some(Item::SmoothSandstoneSlab),
            "minecraft:smooth_quartz_slab" => Some(Item::SmoothQuartzSlab),
            "minecraft:granite_slab" => Some(Item::GraniteSlab),
            "minecraft:andesite_slab" => Some(Item::AndesiteSlab),
            "minecraft:red_nether_brick_slab" => Some(Item::RedNetherBrickSlab),
            "minecraft:polished_andesite_slab" => Some(Item::PolishedAndesiteSlab),
            "minecraft:diorite_slab" => Some(Item::DioriteSlab),
            "minecraft:cobbled_deepslate_slab" => Some(Item::CobbledDeepslateSlab),
            "minecraft:polished_deepslate_slab" => Some(Item::PolishedDeepslateSlab),
            "minecraft:deepslate_brick_slab" => Some(Item::DeepslateBrickSlab),
            "minecraft:deepslate_tile_slab" => Some(Item::DeepslateTileSlab),
            "minecraft:scaffolding" => Some(Item::Scaffolding),
            "minecraft:redstone" => Some(Item::Redstone),
            "minecraft:redstone_torch" => Some(Item::RedstoneTorch),
            "minecraft:redstone_block" => Some(Item::RedstoneBlock),
            "minecraft:repeater" => Some(Item::Repeater),
            "minecraft:comparator" => Some(Item::Comparator),
            "minecraft:piston" => Some(Item::Piston),
            "minecraft:sticky_piston" => Some(Item::StickyPiston),
            "minecraft:slime_block" => Some(Item::SlimeBlock),
            "minecraft:honey_block" => Some(Item::HoneyBlock),
            "minecraft:observer" => Some(Item::Observer),
            "minecraft:hopper" => Some(Item::Hopper),
            "minecraft:dispenser" => Some(Item::Dispenser),
            "minecraft:dropper" => Some(Item::Dropper),
            "minecraft:lectern" => Some(Item::Lectern),
            "minecraft:target" => Some(Item::Target),
            "minecraft:lever" => Some(Item::Lever),
            "minecraft:lightning_rod" => Some(Item::LightningRod),
            "minecraft:daylight_detector" => Some(Item::DaylightDetector),
            "minecraft:sculk_sensor" => Some(Item::SculkSensor),
            "minecraft:tripwire_hook" => Some(Item::TripwireHook),
            "minecraft:trapped_chest" => Some(Item::TrappedChest),
            "minecraft:tnt" => Some(Item::Tnt),
            "minecraft:redstone_lamp" => Some(Item::RedstoneLamp),
            "minecraft:note_block" => Some(Item::NoteBlock),
            "minecraft:stone_button" => Some(Item::StoneButton),
            "minecraft:polished_blackstone_button" => Some(Item::PolishedBlackstoneButton),
            "minecraft:oak_button" => Some(Item::OakButton),
            "minecraft:spruce_button" => Some(Item::SpruceButton),
            "minecraft:birch_button" => Some(Item::BirchButton),
            "minecraft:jungle_button" => Some(Item::JungleButton),
            "minecraft:acacia_button" => Some(Item::AcaciaButton),
            "minecraft:dark_oak_button" => Some(Item::DarkOakButton),
            "minecraft:crimson_button" => Some(Item::CrimsonButton),
            "minecraft:warped_button" => Some(Item::WarpedButton),
            "minecraft:stone_pressure_plate" => Some(Item::StonePressurePlate),
            "minecraft:polished_blackstone_pressure_plate" => {
                Some(Item::PolishedBlackstonePressurePlate)
            }
            "minecraft:light_weighted_pressure_plate" => Some(Item::LightWeightedPressurePlate),
            "minecraft:heavy_weighted_pressure_plate" => Some(Item::HeavyWeightedPressurePlate),
            "minecraft:oak_pressure_plate" => Some(Item::OakPressurePlate),
            "minecraft:spruce_pressure_plate" => Some(Item::SprucePressurePlate),
            "minecraft:birch_pressure_plate" => Some(Item::BirchPressurePlate),
            "minecraft:jungle_pressure_plate" => Some(Item::JunglePressurePlate),
            "minecraft:acacia_pressure_plate" => Some(Item::AcaciaPressurePlate),
            "minecraft:dark_oak_pressure_plate" => Some(Item::DarkOakPressurePlate),
            "minecraft:crimson_pressure_plate" => Some(Item::CrimsonPressurePlate),
            "minecraft:warped_pressure_plate" => Some(Item::WarpedPressurePlate),
            "minecraft:iron_door" => Some(Item::IronDoor),
            "minecraft:oak_door" => Some(Item::OakDoor),
            "minecraft:spruce_door" => Some(Item::SpruceDoor),
            "minecraft:birch_door" => Some(Item::BirchDoor),
            "minecraft:jungle_door" => Some(Item::JungleDoor),
            "minecraft:acacia_door" => Some(Item::AcaciaDoor),
            "minecraft:dark_oak_door" => Some(Item::DarkOakDoor),
            "minecraft:crimson_door" => Some(Item::CrimsonDoor),
            "minecraft:warped_door" => Some(Item::WarpedDoor),
            "minecraft:iron_trapdoor" => Some(Item::IronTrapdoor),
            "minecraft:oak_trapdoor" => Some(Item::OakTrapdoor),
            "minecraft:spruce_trapdoor" => Some(Item::SpruceTrapdoor),
            "minecraft:birch_trapdoor" => Some(Item::BirchTrapdoor),
            "minecraft:jungle_trapdoor" => Some(Item::JungleTrapdoor),
            "minecraft:acacia_trapdoor" => Some(Item::AcaciaTrapdoor),
            "minecraft:dark_oak_trapdoor" => Some(Item::DarkOakTrapdoor),
            "minecraft:crimson_trapdoor" => Some(Item::CrimsonTrapdoor),
            "minecraft:warped_trapdoor" => Some(Item::WarpedTrapdoor),
            "minecraft:oak_fence_gate" => Some(Item::OakFenceGate),
            "minecraft:spruce_fence_gate" => Some(Item::SpruceFenceGate),
            "minecraft:birch_fence_gate" => Some(Item::BirchFenceGate),
            "minecraft:jungle_fence_gate" => Some(Item::JungleFenceGate),
            "minecraft:acacia_fence_gate" => Some(Item::AcaciaFenceGate),
            "minecraft:dark_oak_fence_gate" => Some(Item::DarkOakFenceGate),
            "minecraft:crimson_fence_gate" => Some(Item::CrimsonFenceGate),
            "minecraft:warped_fence_gate" => Some(Item::WarpedFenceGate),
            "minecraft:powered_rail" => Some(Item::PoweredRail),
            "minecraft:detector_rail" => Some(Item::DetectorRail),
            "minecraft:rail" => Some(Item::Rail),
            "minecraft:activator_rail" => Some(Item::ActivatorRail),
            "minecraft:saddle" => Some(Item::Saddle),
            "minecraft:minecart" => Some(Item::Minecart),
            "minecraft:chest_minecart" => Some(Item::ChestMinecart),
            "minecraft:furnace_minecart" => Some(Item::FurnaceMinecart),
            "minecraft:tnt_minecart" => Some(Item::TntMinecart),
            "minecraft:hopper_minecart" => Some(Item::HopperMinecart),
            "minecraft:carrot_on_a_stick" => Some(Item::CarrotOnAStick),
            "minecraft:warped_fungus_on_a_stick" => Some(Item::WarpedFungusOnAStick),
            "minecraft:elytra" => Some(Item::Elytra),
            "minecraft:oak_boat" => Some(Item::OakBoat),
            "minecraft:spruce_boat" => Some(Item::SpruceBoat),
            "minecraft:birch_boat" => Some(Item::BirchBoat),
            "minecraft:jungle_boat" => Some(Item::JungleBoat),
            "minecraft:acacia_boat" => Some(Item::AcaciaBoat),
            "minecraft:dark_oak_boat" => Some(Item::DarkOakBoat),
            "minecraft:structure_block" => Some(Item::StructureBlock),
            "minecraft:jigsaw" => Some(Item::Jigsaw),
            "minecraft:turtle_helmet" => Some(Item::TurtleHelmet),
            "minecraft:scute" => Some(Item::Scute),
            "minecraft:flint_and_steel" => Some(Item::FlintAndSteel),
            "minecraft:apple" => Some(Item::Apple),
            "minecraft:bow" => Some(Item::Bow),
            "minecraft:arrow" => Some(Item::Arrow),
            "minecraft:coal" => Some(Item::Coal),
            "minecraft:charcoal" => Some(Item::Charcoal),
            "minecraft:diamond" => Some(Item::Diamond),
            "minecraft:emerald" => Some(Item::Emerald),
            "minecraft:lapis_lazuli" => Some(Item::LapisLazuli),
            "minecraft:quartz" => Some(Item::Quartz),
            "minecraft:amethyst_shard" => Some(Item::AmethystShard),
            "minecraft:raw_iron" => Some(Item::RawIron),
            "minecraft:iron_ingot" => Some(Item::IronIngot),
            "minecraft:raw_copper" => Some(Item::RawCopper),
            "minecraft:copper_ingot" => Some(Item::CopperIngot),
            "minecraft:raw_gold" => Some(Item::RawGold),
            "minecraft:gold_ingot" => Some(Item::GoldIngot),
            "minecraft:netherite_ingot" => Some(Item::NetheriteIngot),
            "minecraft:netherite_scrap" => Some(Item::NetheriteScrap),
            "minecraft:wooden_sword" => Some(Item::WoodenSword),
            "minecraft:wooden_shovel" => Some(Item::WoodenShovel),
            "minecraft:wooden_pickaxe" => Some(Item::WoodenPickaxe),
            "minecraft:wooden_axe" => Some(Item::WoodenAxe),
            "minecraft:wooden_hoe" => Some(Item::WoodenHoe),
            "minecraft:stone_sword" => Some(Item::StoneSword),
            "minecraft:stone_shovel" => Some(Item::StoneShovel),
            "minecraft:stone_pickaxe" => Some(Item::StonePickaxe),
            "minecraft:stone_axe" => Some(Item::StoneAxe),
            "minecraft:stone_hoe" => Some(Item::StoneHoe),
            "minecraft:golden_sword" => Some(Item::GoldenSword),
            "minecraft:golden_shovel" => Some(Item::GoldenShovel),
            "minecraft:golden_pickaxe" => Some(Item::GoldenPickaxe),
            "minecraft:golden_axe" => Some(Item::GoldenAxe),
            "minecraft:golden_hoe" => Some(Item::GoldenHoe),
            "minecraft:iron_sword" => Some(Item::IronSword),
            "minecraft:iron_shovel" => Some(Item::IronShovel),
            "minecraft:iron_pickaxe" => Some(Item::IronPickaxe),
            "minecraft:iron_axe" => Some(Item::IronAxe),
            "minecraft:iron_hoe" => Some(Item::IronHoe),
            "minecraft:diamond_sword" => Some(Item::DiamondSword),
            "minecraft:diamond_shovel" => Some(Item::DiamondShovel),
            "minecraft:diamond_pickaxe" => Some(Item::DiamondPickaxe),
            "minecraft:diamond_axe" => Some(Item::DiamondAxe),
            "minecraft:diamond_hoe" => Some(Item::DiamondHoe),
            "minecraft:netherite_sword" => Some(Item::NetheriteSword),
            "minecraft:netherite_shovel" => Some(Item::NetheriteShovel),
            "minecraft:netherite_pickaxe" => Some(Item::NetheritePickaxe),
            "minecraft:netherite_axe" => Some(Item::NetheriteAxe),
            "minecraft:netherite_hoe" => Some(Item::NetheriteHoe),
            "minecraft:stick" => Some(Item::Stick),
            "minecraft:bowl" => Some(Item::Bowl),
            "minecraft:mushroom_stew" => Some(Item::MushroomStew),
            "minecraft:string" => Some(Item::String),
            "minecraft:feather" => Some(Item::Feather),
            "minecraft:gunpowder" => Some(Item::Gunpowder),
            "minecraft:wheat_seeds" => Some(Item::WheatSeeds),
            "minecraft:wheat" => Some(Item::Wheat),
            "minecraft:bread" => Some(Item::Bread),
            "minecraft:leather_helmet" => Some(Item::LeatherHelmet),
            "minecraft:leather_chestplate" => Some(Item::LeatherChestplate),
            "minecraft:leather_leggings" => Some(Item::LeatherLeggings),
            "minecraft:leather_boots" => Some(Item::LeatherBoots),
            "minecraft:chainmail_helmet" => Some(Item::ChainmailHelmet),
            "minecraft:chainmail_chestplate" => Some(Item::ChainmailChestplate),
            "minecraft:chainmail_leggings" => Some(Item::ChainmailLeggings),
            "minecraft:chainmail_boots" => Some(Item::ChainmailBoots),
            "minecraft:iron_helmet" => Some(Item::IronHelmet),
            "minecraft:iron_chestplate" => Some(Item::IronChestplate),
            "minecraft:iron_leggings" => Some(Item::IronLeggings),
            "minecraft:iron_boots" => Some(Item::IronBoots),
            "minecraft:diamond_helmet" => Some(Item::DiamondHelmet),
            "minecraft:diamond_chestplate" => Some(Item::DiamondChestplate),
            "minecraft:diamond_leggings" => Some(Item::DiamondLeggings),
            "minecraft:diamond_boots" => Some(Item::DiamondBoots),
            "minecraft:golden_helmet" => Some(Item::GoldenHelmet),
            "minecraft:golden_chestplate" => Some(Item::GoldenChestplate),
            "minecraft:golden_leggings" => Some(Item::GoldenLeggings),
            "minecraft:golden_boots" => Some(Item::GoldenBoots),
            "minecraft:netherite_helmet" => Some(Item::NetheriteHelmet),
            "minecraft:netherite_chestplate" => Some(Item::NetheriteChestplate),
            "minecraft:netherite_leggings" => Some(Item::NetheriteLeggings),
            "minecraft:netherite_boots" => Some(Item::NetheriteBoots),
            "minecraft:flint" => Some(Item::Flint),
            "minecraft:porkchop" => Some(Item::Porkchop),
            "minecraft:cooked_porkchop" => Some(Item::CookedPorkchop),
            "minecraft:painting" => Some(Item::Painting),
            "minecraft:golden_apple" => Some(Item::GoldenApple),
            "minecraft:enchanted_golden_apple" => Some(Item::EnchantedGoldenApple),
            "minecraft:oak_sign" => Some(Item::OakSign),
            "minecraft:spruce_sign" => Some(Item::SpruceSign),
            "minecraft:birch_sign" => Some(Item::BirchSign),
            "minecraft:jungle_sign" => Some(Item::JungleSign),
            "minecraft:acacia_sign" => Some(Item::AcaciaSign),
            "minecraft:dark_oak_sign" => Some(Item::DarkOakSign),
            "minecraft:crimson_sign" => Some(Item::CrimsonSign),
            "minecraft:warped_sign" => Some(Item::WarpedSign),
            "minecraft:bucket" => Some(Item::Bucket),
            "minecraft:water_bucket" => Some(Item::WaterBucket),
            "minecraft:lava_bucket" => Some(Item::LavaBucket),
            "minecraft:powder_snow_bucket" => Some(Item::PowderSnowBucket),
            "minecraft:snowball" => Some(Item::Snowball),
            "minecraft:leather" => Some(Item::Leather),
            "minecraft:milk_bucket" => Some(Item::MilkBucket),
            "minecraft:pufferfish_bucket" => Some(Item::PufferfishBucket),
            "minecraft:salmon_bucket" => Some(Item::SalmonBucket),
            "minecraft:cod_bucket" => Some(Item::CodBucket),
            "minecraft:tropical_fish_bucket" => Some(Item::TropicalFishBucket),
            "minecraft:axolotl_bucket" => Some(Item::AxolotlBucket),
            "minecraft:brick" => Some(Item::Brick),
            "minecraft:clay_ball" => Some(Item::ClayBall),
            "minecraft:dried_kelp_block" => Some(Item::DriedKelpBlock),
            "minecraft:paper" => Some(Item::Paper),
            "minecraft:book" => Some(Item::Book),
            "minecraft:slime_ball" => Some(Item::SlimeBall),
            "minecraft:egg" => Some(Item::Egg),
            "minecraft:compass" => Some(Item::Compass),
            "minecraft:bundle" => Some(Item::Bundle),
            "minecraft:fishing_rod" => Some(Item::FishingRod),
            "minecraft:clock" => Some(Item::Clock),
            "minecraft:spyglass" => Some(Item::Spyglass),
            "minecraft:glowstone_dust" => Some(Item::GlowstoneDust),
            "minecraft:cod" => Some(Item::Cod),
            "minecraft:salmon" => Some(Item::Salmon),
            "minecraft:tropical_fish" => Some(Item::TropicalFish),
            "minecraft:pufferfish" => Some(Item::Pufferfish),
            "minecraft:cooked_cod" => Some(Item::CookedCod),
            "minecraft:cooked_salmon" => Some(Item::CookedSalmon),
            "minecraft:ink_sac" => Some(Item::InkSac),
            "minecraft:glow_ink_sac" => Some(Item::GlowInkSac),
            "minecraft:cocoa_beans" => Some(Item::CocoaBeans),
            "minecraft:white_dye" => Some(Item::WhiteDye),
            "minecraft:orange_dye" => Some(Item::OrangeDye),
            "minecraft:magenta_dye" => Some(Item::MagentaDye),
            "minecraft:light_blue_dye" => Some(Item::LightBlueDye),
            "minecraft:yellow_dye" => Some(Item::YellowDye),
            "minecraft:lime_dye" => Some(Item::LimeDye),
            "minecraft:pink_dye" => Some(Item::PinkDye),
            "minecraft:gray_dye" => Some(Item::GrayDye),
            "minecraft:light_gray_dye" => Some(Item::LightGrayDye),
            "minecraft:cyan_dye" => Some(Item::CyanDye),
            "minecraft:purple_dye" => Some(Item::PurpleDye),
            "minecraft:blue_dye" => Some(Item::BlueDye),
            "minecraft:brown_dye" => Some(Item::BrownDye),
            "minecraft:green_dye" => Some(Item::GreenDye),
            "minecraft:red_dye" => Some(Item::RedDye),
            "minecraft:black_dye" => Some(Item::BlackDye),
            "minecraft:bone_meal" => Some(Item::BoneMeal),
            "minecraft:bone" => Some(Item::Bone),
            "minecraft:sugar" => Some(Item::Sugar),
            "minecraft:cake" => Some(Item::Cake),
            "minecraft:white_bed" => Some(Item::WhiteBed),
            "minecraft:orange_bed" => Some(Item::OrangeBed),
            "minecraft:magenta_bed" => Some(Item::MagentaBed),
            "minecraft:light_blue_bed" => Some(Item::LightBlueBed),
            "minecraft:yellow_bed" => Some(Item::YellowBed),
            "minecraft:lime_bed" => Some(Item::LimeBed),
            "minecraft:pink_bed" => Some(Item::PinkBed),
            "minecraft:gray_bed" => Some(Item::GrayBed),
            "minecraft:light_gray_bed" => Some(Item::LightGrayBed),
            "minecraft:cyan_bed" => Some(Item::CyanBed),
            "minecraft:purple_bed" => Some(Item::PurpleBed),
            "minecraft:blue_bed" => Some(Item::BlueBed),
            "minecraft:brown_bed" => Some(Item::BrownBed),
            "minecraft:green_bed" => Some(Item::GreenBed),
            "minecraft:red_bed" => Some(Item::RedBed),
            "minecraft:black_bed" => Some(Item::BlackBed),
            "minecraft:cookie" => Some(Item::Cookie),
            "minecraft:filled_map" => Some(Item::FilledMap),
            "minecraft:shears" => Some(Item::Shears),
            "minecraft:melon_slice" => Some(Item::MelonSlice),
            "minecraft:dried_kelp" => Some(Item::DriedKelp),
            "minecraft:pumpkin_seeds" => Some(Item::PumpkinSeeds),
            "minecraft:melon_seeds" => Some(Item::MelonSeeds),
            "minecraft:beef" => Some(Item::Beef),
            "minecraft:cooked_beef" => Some(Item::CookedBeef),
            "minecraft:chicken" => Some(Item::Chicken),
            "minecraft:cooked_chicken" => Some(Item::CookedChicken),
            "minecraft:rotten_flesh" => Some(Item::RottenFlesh),
            "minecraft:ender_pearl" => Some(Item::EnderPearl),
            "minecraft:blaze_rod" => Some(Item::BlazeRod),
            "minecraft:ghast_tear" => Some(Item::GhastTear),
            "minecraft:gold_nugget" => Some(Item::GoldNugget),
            "minecraft:nether_wart" => Some(Item::NetherWart),
            "minecraft:potion" => Some(Item::Potion),
            "minecraft:glass_bottle" => Some(Item::GlassBottle),
            "minecraft:spider_eye" => Some(Item::SpiderEye),
            "minecraft:fermented_spider_eye" => Some(Item::FermentedSpiderEye),
            "minecraft:blaze_powder" => Some(Item::BlazePowder),
            "minecraft:magma_cream" => Some(Item::MagmaCream),
            "minecraft:brewing_stand" => Some(Item::BrewingStand),
            "minecraft:cauldron" => Some(Item::Cauldron),
            "minecraft:ender_eye" => Some(Item::EnderEye),
            "minecraft:glistering_melon_slice" => Some(Item::GlisteringMelonSlice),
            "minecraft:axolotl_spawn_egg" => Some(Item::AxolotlSpawnEgg),
            "minecraft:bat_spawn_egg" => Some(Item::BatSpawnEgg),
            "minecraft:bee_spawn_egg" => Some(Item::BeeSpawnEgg),
            "minecraft:blaze_spawn_egg" => Some(Item::BlazeSpawnEgg),
            "minecraft:cat_spawn_egg" => Some(Item::CatSpawnEgg),
            "minecraft:cave_spider_spawn_egg" => Some(Item::CaveSpiderSpawnEgg),
            "minecraft:chicken_spawn_egg" => Some(Item::ChickenSpawnEgg),
            "minecraft:cod_spawn_egg" => Some(Item::CodSpawnEgg),
            "minecraft:cow_spawn_egg" => Some(Item::CowSpawnEgg),
            "minecraft:creeper_spawn_egg" => Some(Item::CreeperSpawnEgg),
            "minecraft:dolphin_spawn_egg" => Some(Item::DolphinSpawnEgg),
            "minecraft:donkey_spawn_egg" => Some(Item::DonkeySpawnEgg),
            "minecraft:drowned_spawn_egg" => Some(Item::DrownedSpawnEgg),
            "minecraft:elder_guardian_spawn_egg" => Some(Item::ElderGuardianSpawnEgg),
            "minecraft:enderman_spawn_egg" => Some(Item::EndermanSpawnEgg),
            "minecraft:endermite_spawn_egg" => Some(Item::EndermiteSpawnEgg),
            "minecraft:evoker_spawn_egg" => Some(Item::EvokerSpawnEgg),
            "minecraft:fox_spawn_egg" => Some(Item::FoxSpawnEgg),
            "minecraft:ghast_spawn_egg" => Some(Item::GhastSpawnEgg),
            "minecraft:glow_squid_spawn_egg" => Some(Item::GlowSquidSpawnEgg),
            "minecraft:goat_spawn_egg" => Some(Item::GoatSpawnEgg),
            "minecraft:guardian_spawn_egg" => Some(Item::GuardianSpawnEgg),
            "minecraft:hoglin_spawn_egg" => Some(Item::HoglinSpawnEgg),
            "minecraft:horse_spawn_egg" => Some(Item::HorseSpawnEgg),
            "minecraft:husk_spawn_egg" => Some(Item::HuskSpawnEgg),
            "minecraft:llama_spawn_egg" => Some(Item::LlamaSpawnEgg),
            "minecraft:magma_cube_spawn_egg" => Some(Item::MagmaCubeSpawnEgg),
            "minecraft:mooshroom_spawn_egg" => Some(Item::MooshroomSpawnEgg),
            "minecraft:mule_spawn_egg" => Some(Item::MuleSpawnEgg),
            "minecraft:ocelot_spawn_egg" => Some(Item::OcelotSpawnEgg),
            "minecraft:panda_spawn_egg" => Some(Item::PandaSpawnEgg),
            "minecraft:parrot_spawn_egg" => Some(Item::ParrotSpawnEgg),
            "minecraft:phantom_spawn_egg" => Some(Item::PhantomSpawnEgg),
            "minecraft:pig_spawn_egg" => Some(Item::PigSpawnEgg),
            "minecraft:piglin_spawn_egg" => Some(Item::PiglinSpawnEgg),
            "minecraft:piglin_brute_spawn_egg" => Some(Item::PiglinBruteSpawnEgg),
            "minecraft:pillager_spawn_egg" => Some(Item::PillagerSpawnEgg),
            "minecraft:polar_bear_spawn_egg" => Some(Item::PolarBearSpawnEgg),
            "minecraft:pufferfish_spawn_egg" => Some(Item::PufferfishSpawnEgg),
            "minecraft:rabbit_spawn_egg" => Some(Item::RabbitSpawnEgg),
            "minecraft:ravager_spawn_egg" => Some(Item::RavagerSpawnEgg),
            "minecraft:salmon_spawn_egg" => Some(Item::SalmonSpawnEgg),
            "minecraft:sheep_spawn_egg" => Some(Item::SheepSpawnEgg),
            "minecraft:shulker_spawn_egg" => Some(Item::ShulkerSpawnEgg),
            "minecraft:silverfish_spawn_egg" => Some(Item::SilverfishSpawnEgg),
            "minecraft:skeleton_spawn_egg" => Some(Item::SkeletonSpawnEgg),
            "minecraft:skeleton_horse_spawn_egg" => Some(Item::SkeletonHorseSpawnEgg),
            "minecraft:slime_spawn_egg" => Some(Item::SlimeSpawnEgg),
            "minecraft:spider_spawn_egg" => Some(Item::SpiderSpawnEgg),
            "minecraft:squid_spawn_egg" => Some(Item::SquidSpawnEgg),
            "minecraft:stray_spawn_egg" => Some(Item::StraySpawnEgg),
            "minecraft:strider_spawn_egg" => Some(Item::StriderSpawnEgg),
            "minecraft:trader_llama_spawn_egg" => Some(Item::TraderLlamaSpawnEgg),
            "minecraft:tropical_fish_spawn_egg" => Some(Item::TropicalFishSpawnEgg),
            "minecraft:turtle_spawn_egg" => Some(Item::TurtleSpawnEgg),
            "minecraft:vex_spawn_egg" => Some(Item::VexSpawnEgg),
            "minecraft:villager_spawn_egg" => Some(Item::VillagerSpawnEgg),
            "minecraft:vindicator_spawn_egg" => Some(Item::VindicatorSpawnEgg),
            "minecraft:wandering_trader_spawn_egg" => Some(Item::WanderingTraderSpawnEgg),
            "minecraft:witch_spawn_egg" => Some(Item::WitchSpawnEgg),
            "minecraft:wither_skeleton_spawn_egg" => Some(Item::WitherSkeletonSpawnEgg),
            "minecraft:wolf_spawn_egg" => Some(Item::WolfSpawnEgg),
            "minecraft:zoglin_spawn_egg" => Some(Item::ZoglinSpawnEgg),
            "minecraft:zombie_spawn_egg" => Some(Item::ZombieSpawnEgg),
            "minecraft:zombie_horse_spawn_egg" => Some(Item::ZombieHorseSpawnEgg),
            "minecraft:zombie_villager_spawn_egg" => Some(Item::ZombieVillagerSpawnEgg),
            "minecraft:zombified_piglin_spawn_egg" => Some(Item::ZombifiedPiglinSpawnEgg),
            "minecraft:experience_bottle" => Some(Item::ExperienceBottle),
            "minecraft:fire_charge" => Some(Item::FireCharge),
            "minecraft:writable_book" => Some(Item::WritableBook),
            "minecraft:written_book" => Some(Item::WrittenBook),
            "minecraft:item_frame" => Some(Item::ItemFrame),
            "minecraft:glow_item_frame" => Some(Item::GlowItemFrame),
            "minecraft:flower_pot" => Some(Item::FlowerPot),
            "minecraft:carrot" => Some(Item::Carrot),
            "minecraft:potato" => Some(Item::Potato),
            "minecraft:baked_potato" => Some(Item::BakedPotato),
            "minecraft:poisonous_potato" => Some(Item::PoisonousPotato),
            "minecraft:map" => Some(Item::Map),
            "minecraft:golden_carrot" => Some(Item::GoldenCarrot),
            "minecraft:skeleton_skull" => Some(Item::SkeletonSkull),
            "minecraft:wither_skeleton_skull" => Some(Item::WitherSkeletonSkull),
            "minecraft:player_head" => Some(Item::PlayerHead),
            "minecraft:zombie_head" => Some(Item::ZombieHead),
            "minecraft:creeper_head" => Some(Item::CreeperHead),
            "minecraft:dragon_head" => Some(Item::DragonHead),
            "minecraft:nether_star" => Some(Item::NetherStar),
            "minecraft:pumpkin_pie" => Some(Item::PumpkinPie),
            "minecraft:firework_rocket" => Some(Item::FireworkRocket),
            "minecraft:firework_star" => Some(Item::FireworkStar),
            "minecraft:enchanted_book" => Some(Item::EnchantedBook),
            "minecraft:nether_brick" => Some(Item::NetherBrick),
            "minecraft:prismarine_shard" => Some(Item::PrismarineShard),
            "minecraft:prismarine_crystals" => Some(Item::PrismarineCrystals),
            "minecraft:rabbit" => Some(Item::Rabbit),
            "minecraft:cooked_rabbit" => Some(Item::CookedRabbit),
            "minecraft:rabbit_stew" => Some(Item::RabbitStew),
            "minecraft:rabbit_foot" => Some(Item::RabbitFoot),
            "minecraft:rabbit_hide" => Some(Item::RabbitHide),
            "minecraft:armor_stand" => Some(Item::ArmorStand),
            "minecraft:iron_horse_armor" => Some(Item::IronHorseArmor),
            "minecraft:golden_horse_armor" => Some(Item::GoldenHorseArmor),
            "minecraft:diamond_horse_armor" => Some(Item::DiamondHorseArmor),
            "minecraft:leather_horse_armor" => Some(Item::LeatherHorseArmor),
            "minecraft:lead" => Some(Item::Lead),
            "minecraft:name_tag" => Some(Item::NameTag),
            "minecraft:command_block_minecart" => Some(Item::CommandBlockMinecart),
            "minecraft:mutton" => Some(Item::Mutton),
            "minecraft:cooked_mutton" => Some(Item::CookedMutton),
            "minecraft:white_banner" => Some(Item::WhiteBanner),
            "minecraft:orange_banner" => Some(Item::OrangeBanner),
            "minecraft:magenta_banner" => Some(Item::MagentaBanner),
            "minecraft:light_blue_banner" => Some(Item::LightBlueBanner),
            "minecraft:yellow_banner" => Some(Item::YellowBanner),
            "minecraft:lime_banner" => Some(Item::LimeBanner),
            "minecraft:pink_banner" => Some(Item::PinkBanner),
            "minecraft:gray_banner" => Some(Item::GrayBanner),
            "minecraft:light_gray_banner" => Some(Item::LightGrayBanner),
            "minecraft:cyan_banner" => Some(Item::CyanBanner),
            "minecraft:purple_banner" => Some(Item::PurpleBanner),
            "minecraft:blue_banner" => Some(Item::BlueBanner),
            "minecraft:brown_banner" => Some(Item::BrownBanner),
            "minecraft:green_banner" => Some(Item::GreenBanner),
            "minecraft:red_banner" => Some(Item::RedBanner),
            "minecraft:black_banner" => Some(Item::BlackBanner),
            "minecraft:end_crystal" => Some(Item::EndCrystal),
            "minecraft:chorus_fruit" => Some(Item::ChorusFruit),
            "minecraft:popped_chorus_fruit" => Some(Item::PoppedChorusFruit),
            "minecraft:beetroot" => Some(Item::Beetroot),
            "minecraft:beetroot_seeds" => Some(Item::BeetrootSeeds),
            "minecraft:beetroot_soup" => Some(Item::BeetrootSoup),
            "minecraft:dragon_breath" => Some(Item::DragonBreath),
            "minecraft:splash_potion" => Some(Item::SplashPotion),
            "minecraft:spectral_arrow" => Some(Item::SpectralArrow),
            "minecraft:tipped_arrow" => Some(Item::TippedArrow),
            "minecraft:lingering_potion" => Some(Item::LingeringPotion),
            "minecraft:shield" => Some(Item::Shield),
            "minecraft:totem_of_undying" => Some(Item::TotemOfUndying),
            "minecraft:shulker_shell" => Some(Item::ShulkerShell),
            "minecraft:iron_nugget" => Some(Item::IronNugget),
            "minecraft:knowledge_book" => Some(Item::KnowledgeBook),
            "minecraft:debug_stick" => Some(Item::DebugStick),
            "minecraft:music_disc_13" => Some(Item::MusicDisc13),
            "minecraft:music_disc_cat" => Some(Item::MusicDiscCat),
            "minecraft:music_disc_blocks" => Some(Item::MusicDiscBlocks),
            "minecraft:music_disc_chirp" => Some(Item::MusicDiscChirp),
            "minecraft:music_disc_far" => Some(Item::MusicDiscFar),
            "minecraft:music_disc_mall" => Some(Item::MusicDiscMall),
            "minecraft:music_disc_mellohi" => Some(Item::MusicDiscMellohi),
            "minecraft:music_disc_stal" => Some(Item::MusicDiscStal),
            "minecraft:music_disc_strad" => Some(Item::MusicDiscStrad),
            "minecraft:music_disc_ward" => Some(Item::MusicDiscWard),
            "minecraft:music_disc_11" => Some(Item::MusicDisc11),
            "minecraft:music_disc_wait" => Some(Item::MusicDiscWait),
            "minecraft:music_disc_otherside" => Some(Item::MusicDiscOtherside),
            "minecraft:music_disc_pigstep" => Some(Item::MusicDiscPigstep),
            "minecraft:trident" => Some(Item::Trident),
            "minecraft:phantom_membrane" => Some(Item::PhantomMembrane),
            "minecraft:nautilus_shell" => Some(Item::NautilusShell),
            "minecraft:heart_of_the_sea" => Some(Item::HeartOfTheSea),
            "minecraft:crossbow" => Some(Item::Crossbow),
            "minecraft:suspicious_stew" => Some(Item::SuspiciousStew),
            "minecraft:loom" => Some(Item::Loom),
            "minecraft:flower_banner_pattern" => Some(Item::FlowerBannerPattern),
            "minecraft:creeper_banner_pattern" => Some(Item::CreeperBannerPattern),
            "minecraft:skull_banner_pattern" => Some(Item::SkullBannerPattern),
            "minecraft:mojang_banner_pattern" => Some(Item::MojangBannerPattern),
            "minecraft:globe_banner_pattern" => Some(Item::GlobeBannerPattern),
            "minecraft:piglin_banner_pattern" => Some(Item::PiglinBannerPattern),
            "minecraft:composter" => Some(Item::Composter),
            "minecraft:barrel" => Some(Item::Barrel),
            "minecraft:smoker" => Some(Item::Smoker),
            "minecraft:blast_furnace" => Some(Item::BlastFurnace),
            "minecraft:cartography_table" => Some(Item::CartographyTable),
            "minecraft:fletching_table" => Some(Item::FletchingTable),
            "minecraft:grindstone" => Some(Item::Grindstone),
            "minecraft:smithing_table" => Some(Item::SmithingTable),
            "minecraft:stonecutter" => Some(Item::Stonecutter),
            "minecraft:bell" => Some(Item::Bell),
            "minecraft:lantern" => Some(Item::Lantern),
            "minecraft:soul_lantern" => Some(Item::SoulLantern),
            "minecraft:sweet_berries" => Some(Item::SweetBerries),
            "minecraft:glow_berries" => Some(Item::GlowBerries),
            "minecraft:campfire" => Some(Item::Campfire),
            "minecraft:soul_campfire" => Some(Item::SoulCampfire),
            "minecraft:shroomlight" => Some(Item::Shroomlight),
            "minecraft:honeycomb" => Some(Item::Honeycomb),
            "minecraft:bee_nest" => Some(Item::BeeNest),
            "minecraft:beehive" => Some(Item::Beehive),
            "minecraft:honey_bottle" => Some(Item::HoneyBottle),
            "minecraft:honeycomb_block" => Some(Item::HoneycombBlock),
            "minecraft:lodestone" => Some(Item::Lodestone),
            "minecraft:crying_obsidian" => Some(Item::CryingObsidian),
            "minecraft:blackstone" => Some(Item::Blackstone),
            "minecraft:blackstone_slab" => Some(Item::BlackstoneSlab),
            "minecraft:blackstone_stairs" => Some(Item::BlackstoneStairs),
            "minecraft:gilded_blackstone" => Some(Item::GildedBlackstone),
            "minecraft:polished_blackstone" => Some(Item::PolishedBlackstone),
            "minecraft:polished_blackstone_slab" => Some(Item::PolishedBlackstoneSlab),
            "minecraft:polished_blackstone_stairs" => Some(Item::PolishedBlackstoneStairs),
            "minecraft:chiseled_polished_blackstone" => Some(Item::ChiseledPolishedBlackstone),
            "minecraft:polished_blackstone_bricks" => Some(Item::PolishedBlackstoneBricks),
            "minecraft:polished_blackstone_brick_slab" => Some(Item::PolishedBlackstoneBrickSlab),
            "minecraft:polished_blackstone_brick_stairs" => {
                Some(Item::PolishedBlackstoneBrickStairs)
            }
            "minecraft:cracked_polished_blackstone_bricks" => {
                Some(Item::CrackedPolishedBlackstoneBricks)
            }
            "minecraft:respawn_anchor" => Some(Item::RespawnAnchor),
            "minecraft:candle" => Some(Item::Candle),
            "minecraft:white_candle" => Some(Item::WhiteCandle),
            "minecraft:orange_candle" => Some(Item::OrangeCandle),
            "minecraft:magenta_candle" => Some(Item::MagentaCandle),
            "minecraft:light_blue_candle" => Some(Item::LightBlueCandle),
            "minecraft:yellow_candle" => Some(Item::YellowCandle),
            "minecraft:lime_candle" => Some(Item::LimeCandle),
            "minecraft:pink_candle" => Some(Item::PinkCandle),
            "minecraft:gray_candle" => Some(Item::GrayCandle),
            "minecraft:light_gray_candle" => Some(Item::LightGrayCandle),
            "minecraft:cyan_candle" => Some(Item::CyanCandle),
            "minecraft:purple_candle" => Some(Item::PurpleCandle),
            "minecraft:blue_candle" => Some(Item::BlueCandle),
            "minecraft:brown_candle" => Some(Item::BrownCandle),
            "minecraft:green_candle" => Some(Item::GreenCandle),
            "minecraft:red_candle" => Some(Item::RedCandle),
            "minecraft:black_candle" => Some(Item::BlackCandle),
            "minecraft:small_amethyst_bud" => Some(Item::SmallAmethystBud),
            "minecraft:medium_amethyst_bud" => Some(Item::MediumAmethystBud),
            "minecraft:large_amethyst_bud" => Some(Item::LargeAmethystBud),
            "minecraft:amethyst_cluster" => Some(Item::AmethystCluster),
            "minecraft:pointed_dripstone" => Some(Item::PointedDripstone),
            _ => None,
        }
    }
}
impl Item {
    #[doc = "Returns the `stack_size` property of this `Item`."]
    #[inline]
    pub fn stack_size(&self) -> u32 {
        match self {
            Item::Stone => 64u32,
            Item::Granite => 64u32,
            Item::PolishedGranite => 64u32,
            Item::Diorite => 64u32,
            Item::PolishedDiorite => 64u32,
            Item::Andesite => 64u32,
            Item::PolishedAndesite => 64u32,
            Item::Deepslate => 64u32,
            Item::CobbledDeepslate => 64u32,
            Item::PolishedDeepslate => 64u32,
            Item::Calcite => 64u32,
            Item::Tuff => 64u32,
            Item::DripstoneBlock => 64u32,
            Item::GrassBlock => 64u32,
            Item::Dirt => 64u32,
            Item::CoarseDirt => 64u32,
            Item::Podzol => 64u32,
            Item::RootedDirt => 64u32,
            Item::CrimsonNylium => 64u32,
            Item::WarpedNylium => 64u32,
            Item::Cobblestone => 64u32,
            Item::OakPlanks => 64u32,
            Item::SprucePlanks => 64u32,
            Item::BirchPlanks => 64u32,
            Item::JunglePlanks => 64u32,
            Item::AcaciaPlanks => 64u32,
            Item::DarkOakPlanks => 64u32,
            Item::CrimsonPlanks => 64u32,
            Item::WarpedPlanks => 64u32,
            Item::OakSapling => 64u32,
            Item::SpruceSapling => 64u32,
            Item::BirchSapling => 64u32,
            Item::JungleSapling => 64u32,
            Item::AcaciaSapling => 64u32,
            Item::DarkOakSapling => 64u32,
            Item::Bedrock => 64u32,
            Item::Sand => 64u32,
            Item::RedSand => 64u32,
            Item::Gravel => 64u32,
            Item::CoalOre => 64u32,
            Item::DeepslateCoalOre => 64u32,
            Item::IronOre => 64u32,
            Item::DeepslateIronOre => 64u32,
            Item::CopperOre => 64u32,
            Item::DeepslateCopperOre => 64u32,
            Item::GoldOre => 64u32,
            Item::DeepslateGoldOre => 64u32,
            Item::RedstoneOre => 64u32,
            Item::DeepslateRedstoneOre => 64u32,
            Item::EmeraldOre => 64u32,
            Item::DeepslateEmeraldOre => 64u32,
            Item::LapisOre => 64u32,
            Item::DeepslateLapisOre => 64u32,
            Item::DiamondOre => 64u32,
            Item::DeepslateDiamondOre => 64u32,
            Item::NetherGoldOre => 64u32,
            Item::NetherQuartzOre => 64u32,
            Item::AncientDebris => 64u32,
            Item::CoalBlock => 64u32,
            Item::RawIronBlock => 64u32,
            Item::RawCopperBlock => 64u32,
            Item::RawGoldBlock => 64u32,
            Item::AmethystBlock => 64u32,
            Item::BuddingAmethyst => 64u32,
            Item::IronBlock => 64u32,
            Item::CopperBlock => 64u32,
            Item::GoldBlock => 64u32,
            Item::DiamondBlock => 64u32,
            Item::NetheriteBlock => 64u32,
            Item::ExposedCopper => 64u32,
            Item::WeatheredCopper => 64u32,
            Item::OxidizedCopper => 64u32,
            Item::CutCopper => 64u32,
            Item::ExposedCutCopper => 64u32,
            Item::WeatheredCutCopper => 64u32,
            Item::OxidizedCutCopper => 64u32,
            Item::CutCopperStairs => 64u32,
            Item::ExposedCutCopperStairs => 64u32,
            Item::WeatheredCutCopperStairs => 64u32,
            Item::OxidizedCutCopperStairs => 64u32,
            Item::CutCopperSlab => 64u32,
            Item::ExposedCutCopperSlab => 64u32,
            Item::WeatheredCutCopperSlab => 64u32,
            Item::OxidizedCutCopperSlab => 64u32,
            Item::WaxedCopperBlock => 64u32,
            Item::WaxedExposedCopper => 64u32,
            Item::WaxedWeatheredCopper => 64u32,
            Item::WaxedOxidizedCopper => 64u32,
            Item::WaxedCutCopper => 64u32,
            Item::WaxedExposedCutCopper => 64u32,
            Item::WaxedWeatheredCutCopper => 64u32,
            Item::WaxedOxidizedCutCopper => 64u32,
            Item::WaxedCutCopperStairs => 64u32,
            Item::WaxedExposedCutCopperStairs => 64u32,
            Item::WaxedWeatheredCutCopperStairs => 64u32,
            Item::WaxedOxidizedCutCopperStairs => 64u32,
            Item::WaxedCutCopperSlab => 64u32,
            Item::WaxedExposedCutCopperSlab => 64u32,
            Item::WaxedWeatheredCutCopperSlab => 64u32,
            Item::WaxedOxidizedCutCopperSlab => 64u32,
            Item::OakLog => 64u32,
            Item::SpruceLog => 64u32,
            Item::BirchLog => 64u32,
            Item::JungleLog => 64u32,
            Item::AcaciaLog => 64u32,
            Item::DarkOakLog => 64u32,
            Item::CrimsonStem => 64u32,
            Item::WarpedStem => 64u32,
            Item::StrippedOakLog => 64u32,
            Item::StrippedSpruceLog => 64u32,
            Item::StrippedBirchLog => 64u32,
            Item::StrippedJungleLog => 64u32,
            Item::StrippedAcaciaLog => 64u32,
            Item::StrippedDarkOakLog => 64u32,
            Item::StrippedCrimsonStem => 64u32,
            Item::StrippedWarpedStem => 64u32,
            Item::StrippedOakWood => 64u32,
            Item::StrippedSpruceWood => 64u32,
            Item::StrippedBirchWood => 64u32,
            Item::StrippedJungleWood => 64u32,
            Item::StrippedAcaciaWood => 64u32,
            Item::StrippedDarkOakWood => 64u32,
            Item::StrippedCrimsonHyphae => 64u32,
            Item::StrippedWarpedHyphae => 64u32,
            Item::OakWood => 64u32,
            Item::SpruceWood => 64u32,
            Item::BirchWood => 64u32,
            Item::JungleWood => 64u32,
            Item::AcaciaWood => 64u32,
            Item::DarkOakWood => 64u32,
            Item::CrimsonHyphae => 64u32,
            Item::WarpedHyphae => 64u32,
            Item::OakLeaves => 64u32,
            Item::SpruceLeaves => 64u32,
            Item::BirchLeaves => 64u32,
            Item::JungleLeaves => 64u32,
            Item::AcaciaLeaves => 64u32,
            Item::DarkOakLeaves => 64u32,
            Item::AzaleaLeaves => 64u32,
            Item::FloweringAzaleaLeaves => 64u32,
            Item::Sponge => 64u32,
            Item::WetSponge => 64u32,
            Item::Glass => 64u32,
            Item::TintedGlass => 64u32,
            Item::LapisBlock => 64u32,
            Item::Sandstone => 64u32,
            Item::ChiseledSandstone => 64u32,
            Item::CutSandstone => 64u32,
            Item::Cobweb => 64u32,
            Item::Grass => 64u32,
            Item::Fern => 64u32,
            Item::Azalea => 64u32,
            Item::FloweringAzalea => 64u32,
            Item::DeadBush => 64u32,
            Item::Seagrass => 64u32,
            Item::SeaPickle => 64u32,
            Item::WhiteWool => 64u32,
            Item::OrangeWool => 64u32,
            Item::MagentaWool => 64u32,
            Item::LightBlueWool => 64u32,
            Item::YellowWool => 64u32,
            Item::LimeWool => 64u32,
            Item::PinkWool => 64u32,
            Item::GrayWool => 64u32,
            Item::LightGrayWool => 64u32,
            Item::CyanWool => 64u32,
            Item::PurpleWool => 64u32,
            Item::BlueWool => 64u32,
            Item::BrownWool => 64u32,
            Item::GreenWool => 64u32,
            Item::RedWool => 64u32,
            Item::BlackWool => 64u32,
            Item::Dandelion => 64u32,
            Item::Poppy => 64u32,
            Item::BlueOrchid => 64u32,
            Item::Allium => 64u32,
            Item::AzureBluet => 64u32,
            Item::RedTulip => 64u32,
            Item::OrangeTulip => 64u32,
            Item::WhiteTulip => 64u32,
            Item::PinkTulip => 64u32,
            Item::OxeyeDaisy => 64u32,
            Item::Cornflower => 64u32,
            Item::LilyOfTheValley => 64u32,
            Item::WitherRose => 64u32,
            Item::SporeBlossom => 64u32,
            Item::BrownMushroom => 64u32,
            Item::RedMushroom => 64u32,
            Item::CrimsonFungus => 64u32,
            Item::WarpedFungus => 64u32,
            Item::CrimsonRoots => 64u32,
            Item::WarpedRoots => 64u32,
            Item::NetherSprouts => 64u32,
            Item::WeepingVines => 64u32,
            Item::TwistingVines => 64u32,
            Item::SugarCane => 64u32,
            Item::Kelp => 64u32,
            Item::MossCarpet => 64u32,
            Item::MossBlock => 64u32,
            Item::HangingRoots => 64u32,
            Item::BigDripleaf => 64u32,
            Item::SmallDripleaf => 64u32,
            Item::Bamboo => 64u32,
            Item::OakSlab => 64u32,
            Item::SpruceSlab => 64u32,
            Item::BirchSlab => 64u32,
            Item::JungleSlab => 64u32,
            Item::AcaciaSlab => 64u32,
            Item::DarkOakSlab => 64u32,
            Item::CrimsonSlab => 64u32,
            Item::WarpedSlab => 64u32,
            Item::StoneSlab => 64u32,
            Item::SmoothStoneSlab => 64u32,
            Item::SandstoneSlab => 64u32,
            Item::CutSandstoneSlab => 64u32,
            Item::PetrifiedOakSlab => 64u32,
            Item::CobblestoneSlab => 64u32,
            Item::BrickSlab => 64u32,
            Item::StoneBrickSlab => 64u32,
            Item::NetherBrickSlab => 64u32,
            Item::QuartzSlab => 64u32,
            Item::RedSandstoneSlab => 64u32,
            Item::CutRedSandstoneSlab => 64u32,
            Item::PurpurSlab => 64u32,
            Item::PrismarineSlab => 64u32,
            Item::PrismarineBrickSlab => 64u32,
            Item::DarkPrismarineSlab => 64u32,
            Item::SmoothQuartz => 64u32,
            Item::SmoothRedSandstone => 64u32,
            Item::SmoothSandstone => 64u32,
            Item::SmoothStone => 64u32,
            Item::Bricks => 64u32,
            Item::Bookshelf => 64u32,
            Item::MossyCobblestone => 64u32,
            Item::Obsidian => 64u32,
            Item::Torch => 64u32,
            Item::EndRod => 64u32,
            Item::ChorusPlant => 64u32,
            Item::ChorusFlower => 64u32,
            Item::PurpurBlock => 64u32,
            Item::PurpurPillar => 64u32,
            Item::PurpurStairs => 64u32,
            Item::Spawner => 64u32,
            Item::OakStairs => 64u32,
            Item::Chest => 64u32,
            Item::CraftingTable => 64u32,
            Item::Farmland => 64u32,
            Item::Furnace => 64u32,
            Item::Ladder => 64u32,
            Item::CobblestoneStairs => 64u32,
            Item::Snow => 64u32,
            Item::Ice => 64u32,
            Item::SnowBlock => 64u32,
            Item::Cactus => 64u32,
            Item::Clay => 64u32,
            Item::Jukebox => 64u32,
            Item::OakFence => 64u32,
            Item::SpruceFence => 64u32,
            Item::BirchFence => 64u32,
            Item::JungleFence => 64u32,
            Item::AcaciaFence => 64u32,
            Item::DarkOakFence => 64u32,
            Item::CrimsonFence => 64u32,
            Item::WarpedFence => 64u32,
            Item::Pumpkin => 64u32,
            Item::CarvedPumpkin => 64u32,
            Item::JackOLantern => 64u32,
            Item::Netherrack => 64u32,
            Item::SoulSand => 64u32,
            Item::SoulSoil => 64u32,
            Item::Basalt => 64u32,
            Item::PolishedBasalt => 64u32,
            Item::SmoothBasalt => 64u32,
            Item::SoulTorch => 64u32,
            Item::Glowstone => 64u32,
            Item::InfestedStone => 64u32,
            Item::InfestedCobblestone => 64u32,
            Item::InfestedStoneBricks => 64u32,
            Item::InfestedMossyStoneBricks => 64u32,
            Item::InfestedCrackedStoneBricks => 64u32,
            Item::InfestedChiseledStoneBricks => 64u32,
            Item::InfestedDeepslate => 64u32,
            Item::StoneBricks => 64u32,
            Item::MossyStoneBricks => 64u32,
            Item::CrackedStoneBricks => 64u32,
            Item::ChiseledStoneBricks => 64u32,
            Item::DeepslateBricks => 64u32,
            Item::CrackedDeepslateBricks => 64u32,
            Item::DeepslateTiles => 64u32,
            Item::CrackedDeepslateTiles => 64u32,
            Item::ChiseledDeepslate => 64u32,
            Item::BrownMushroomBlock => 64u32,
            Item::RedMushroomBlock => 64u32,
            Item::MushroomStem => 64u32,
            Item::IronBars => 64u32,
            Item::Chain => 64u32,
            Item::GlassPane => 64u32,
            Item::Melon => 64u32,
            Item::Vine => 64u32,
            Item::GlowLichen => 64u32,
            Item::BrickStairs => 64u32,
            Item::StoneBrickStairs => 64u32,
            Item::Mycelium => 64u32,
            Item::LilyPad => 64u32,
            Item::NetherBricks => 64u32,
            Item::CrackedNetherBricks => 64u32,
            Item::ChiseledNetherBricks => 64u32,
            Item::NetherBrickFence => 64u32,
            Item::NetherBrickStairs => 64u32,
            Item::EnchantingTable => 64u32,
            Item::EndPortalFrame => 64u32,
            Item::EndStone => 64u32,
            Item::EndStoneBricks => 64u32,
            Item::DragonEgg => 64u32,
            Item::SandstoneStairs => 64u32,
            Item::EnderChest => 64u32,
            Item::EmeraldBlock => 64u32,
            Item::SpruceStairs => 64u32,
            Item::BirchStairs => 64u32,
            Item::JungleStairs => 64u32,
            Item::CrimsonStairs => 64u32,
            Item::WarpedStairs => 64u32,
            Item::CommandBlock => 64u32,
            Item::Beacon => 64u32,
            Item::CobblestoneWall => 64u32,
            Item::MossyCobblestoneWall => 64u32,
            Item::BrickWall => 64u32,
            Item::PrismarineWall => 64u32,
            Item::RedSandstoneWall => 64u32,
            Item::MossyStoneBrickWall => 64u32,
            Item::GraniteWall => 64u32,
            Item::StoneBrickWall => 64u32,
            Item::NetherBrickWall => 64u32,
            Item::AndesiteWall => 64u32,
            Item::RedNetherBrickWall => 64u32,
            Item::SandstoneWall => 64u32,
            Item::EndStoneBrickWall => 64u32,
            Item::DioriteWall => 64u32,
            Item::BlackstoneWall => 64u32,
            Item::PolishedBlackstoneWall => 64u32,
            Item::PolishedBlackstoneBrickWall => 64u32,
            Item::CobbledDeepslateWall => 64u32,
            Item::PolishedDeepslateWall => 64u32,
            Item::DeepslateBrickWall => 64u32,
            Item::DeepslateTileWall => 64u32,
            Item::Anvil => 64u32,
            Item::ChippedAnvil => 64u32,
            Item::DamagedAnvil => 64u32,
            Item::ChiseledQuartzBlock => 64u32,
            Item::QuartzBlock => 64u32,
            Item::QuartzBricks => 64u32,
            Item::QuartzPillar => 64u32,
            Item::QuartzStairs => 64u32,
            Item::WhiteTerracotta => 64u32,
            Item::OrangeTerracotta => 64u32,
            Item::MagentaTerracotta => 64u32,
            Item::LightBlueTerracotta => 64u32,
            Item::YellowTerracotta => 64u32,
            Item::LimeTerracotta => 64u32,
            Item::PinkTerracotta => 64u32,
            Item::GrayTerracotta => 64u32,
            Item::LightGrayTerracotta => 64u32,
            Item::CyanTerracotta => 64u32,
            Item::PurpleTerracotta => 64u32,
            Item::BlueTerracotta => 64u32,
            Item::BrownTerracotta => 64u32,
            Item::GreenTerracotta => 64u32,
            Item::RedTerracotta => 64u32,
            Item::BlackTerracotta => 64u32,
            Item::Barrier => 64u32,
            Item::Light => 64u32,
            Item::HayBlock => 64u32,
            Item::WhiteCarpet => 64u32,
            Item::OrangeCarpet => 64u32,
            Item::MagentaCarpet => 64u32,
            Item::LightBlueCarpet => 64u32,
            Item::YellowCarpet => 64u32,
            Item::LimeCarpet => 64u32,
            Item::PinkCarpet => 64u32,
            Item::GrayCarpet => 64u32,
            Item::LightGrayCarpet => 64u32,
            Item::CyanCarpet => 64u32,
            Item::PurpleCarpet => 64u32,
            Item::BlueCarpet => 64u32,
            Item::BrownCarpet => 64u32,
            Item::GreenCarpet => 64u32,
            Item::RedCarpet => 64u32,
            Item::BlackCarpet => 64u32,
            Item::Terracotta => 64u32,
            Item::PackedIce => 64u32,
            Item::AcaciaStairs => 64u32,
            Item::DarkOakStairs => 64u32,
            Item::DirtPath => 64u32,
            Item::Sunflower => 64u32,
            Item::Lilac => 64u32,
            Item::RoseBush => 64u32,
            Item::Peony => 64u32,
            Item::TallGrass => 64u32,
            Item::LargeFern => 64u32,
            Item::WhiteStainedGlass => 64u32,
            Item::OrangeStainedGlass => 64u32,
            Item::MagentaStainedGlass => 64u32,
            Item::LightBlueStainedGlass => 64u32,
            Item::YellowStainedGlass => 64u32,
            Item::LimeStainedGlass => 64u32,
            Item::PinkStainedGlass => 64u32,
            Item::GrayStainedGlass => 64u32,
            Item::LightGrayStainedGlass => 64u32,
            Item::CyanStainedGlass => 64u32,
            Item::PurpleStainedGlass => 64u32,
            Item::BlueStainedGlass => 64u32,
            Item::BrownStainedGlass => 64u32,
            Item::GreenStainedGlass => 64u32,
            Item::RedStainedGlass => 64u32,
            Item::BlackStainedGlass => 64u32,
            Item::WhiteStainedGlassPane => 64u32,
            Item::OrangeStainedGlassPane => 64u32,
            Item::MagentaStainedGlassPane => 64u32,
            Item::LightBlueStainedGlassPane => 64u32,
            Item::YellowStainedGlassPane => 64u32,
            Item::LimeStainedGlassPane => 64u32,
            Item::PinkStainedGlassPane => 64u32,
            Item::GrayStainedGlassPane => 64u32,
            Item::LightGrayStainedGlassPane => 64u32,
            Item::CyanStainedGlassPane => 64u32,
            Item::PurpleStainedGlassPane => 64u32,
            Item::BlueStainedGlassPane => 64u32,
            Item::BrownStainedGlassPane => 64u32,
            Item::GreenStainedGlassPane => 64u32,
            Item::RedStainedGlassPane => 64u32,
            Item::BlackStainedGlassPane => 64u32,
            Item::Prismarine => 64u32,
            Item::PrismarineBricks => 64u32,
            Item::DarkPrismarine => 64u32,
            Item::PrismarineStairs => 64u32,
            Item::PrismarineBrickStairs => 64u32,
            Item::DarkPrismarineStairs => 64u32,
            Item::SeaLantern => 64u32,
            Item::RedSandstone => 64u32,
            Item::ChiseledRedSandstone => 64u32,
            Item::CutRedSandstone => 64u32,
            Item::RedSandstoneStairs => 64u32,
            Item::RepeatingCommandBlock => 64u32,
            Item::ChainCommandBlock => 64u32,
            Item::MagmaBlock => 64u32,
            Item::NetherWartBlock => 64u32,
            Item::WarpedWartBlock => 64u32,
            Item::RedNetherBricks => 64u32,
            Item::BoneBlock => 64u32,
            Item::StructureVoid => 64u32,
            Item::ShulkerBox => 1u32,
            Item::WhiteShulkerBox => 1u32,
            Item::OrangeShulkerBox => 1u32,
            Item::MagentaShulkerBox => 1u32,
            Item::LightBlueShulkerBox => 1u32,
            Item::YellowShulkerBox => 1u32,
            Item::LimeShulkerBox => 1u32,
            Item::PinkShulkerBox => 1u32,
            Item::GrayShulkerBox => 1u32,
            Item::LightGrayShulkerBox => 1u32,
            Item::CyanShulkerBox => 1u32,
            Item::PurpleShulkerBox => 1u32,
            Item::BlueShulkerBox => 1u32,
            Item::BrownShulkerBox => 1u32,
            Item::GreenShulkerBox => 1u32,
            Item::RedShulkerBox => 1u32,
            Item::BlackShulkerBox => 1u32,
            Item::WhiteGlazedTerracotta => 64u32,
            Item::OrangeGlazedTerracotta => 64u32,
            Item::MagentaGlazedTerracotta => 64u32,
            Item::LightBlueGlazedTerracotta => 64u32,
            Item::YellowGlazedTerracotta => 64u32,
            Item::LimeGlazedTerracotta => 64u32,
            Item::PinkGlazedTerracotta => 64u32,
            Item::GrayGlazedTerracotta => 64u32,
            Item::LightGrayGlazedTerracotta => 64u32,
            Item::CyanGlazedTerracotta => 64u32,
            Item::PurpleGlazedTerracotta => 64u32,
            Item::BlueGlazedTerracotta => 64u32,
            Item::BrownGlazedTerracotta => 64u32,
            Item::GreenGlazedTerracotta => 64u32,
            Item::RedGlazedTerracotta => 64u32,
            Item::BlackGlazedTerracotta => 64u32,
            Item::WhiteConcrete => 64u32,
            Item::OrangeConcrete => 64u32,
            Item::MagentaConcrete => 64u32,
            Item::LightBlueConcrete => 64u32,
            Item::YellowConcrete => 64u32,
            Item::LimeConcrete => 64u32,
            Item::PinkConcrete => 64u32,
            Item::GrayConcrete => 64u32,
            Item::LightGrayConcrete => 64u32,
            Item::CyanConcrete => 64u32,
            Item::PurpleConcrete => 64u32,
            Item::BlueConcrete => 64u32,
            Item::BrownConcrete => 64u32,
            Item::GreenConcrete => 64u32,
            Item::RedConcrete => 64u32,
            Item::BlackConcrete => 64u32,
            Item::WhiteConcretePowder => 64u32,
            Item::OrangeConcretePowder => 64u32,
            Item::MagentaConcretePowder => 64u32,
            Item::LightBlueConcretePowder => 64u32,
            Item::YellowConcretePowder => 64u32,
            Item::LimeConcretePowder => 64u32,
            Item::PinkConcretePowder => 64u32,
            Item::GrayConcretePowder => 64u32,
            Item::LightGrayConcretePowder => 64u32,
            Item::CyanConcretePowder => 64u32,
            Item::PurpleConcretePowder => 64u32,
            Item::BlueConcretePowder => 64u32,
            Item::BrownConcretePowder => 64u32,
            Item::GreenConcretePowder => 64u32,
            Item::RedConcretePowder => 64u32,
            Item::BlackConcretePowder => 64u32,
            Item::TurtleEgg => 64u32,
            Item::DeadTubeCoralBlock => 64u32,
            Item::DeadBrainCoralBlock => 64u32,
            Item::DeadBubbleCoralBlock => 64u32,
            Item::DeadFireCoralBlock => 64u32,
            Item::DeadHornCoralBlock => 64u32,
            Item::TubeCoralBlock => 64u32,
            Item::BrainCoralBlock => 64u32,
            Item::BubbleCoralBlock => 64u32,
            Item::FireCoralBlock => 64u32,
            Item::HornCoralBlock => 64u32,
            Item::TubeCoral => 64u32,
            Item::BrainCoral => 64u32,
            Item::BubbleCoral => 64u32,
            Item::FireCoral => 64u32,
            Item::HornCoral => 64u32,
            Item::DeadBrainCoral => 64u32,
            Item::DeadBubbleCoral => 64u32,
            Item::DeadFireCoral => 64u32,
            Item::DeadHornCoral => 64u32,
            Item::DeadTubeCoral => 64u32,
            Item::TubeCoralFan => 64u32,
            Item::BrainCoralFan => 64u32,
            Item::BubbleCoralFan => 64u32,
            Item::FireCoralFan => 64u32,
            Item::HornCoralFan => 64u32,
            Item::DeadTubeCoralFan => 64u32,
            Item::DeadBrainCoralFan => 64u32,
            Item::DeadBubbleCoralFan => 64u32,
            Item::DeadFireCoralFan => 64u32,
            Item::DeadHornCoralFan => 64u32,
            Item::BlueIce => 64u32,
            Item::Conduit => 64u32,
            Item::PolishedGraniteStairs => 64u32,
            Item::SmoothRedSandstoneStairs => 64u32,
            Item::MossyStoneBrickStairs => 64u32,
            Item::PolishedDioriteStairs => 64u32,
            Item::MossyCobblestoneStairs => 64u32,
            Item::EndStoneBrickStairs => 64u32,
            Item::StoneStairs => 64u32,
            Item::SmoothSandstoneStairs => 64u32,
            Item::SmoothQuartzStairs => 64u32,
            Item::GraniteStairs => 64u32,
            Item::AndesiteStairs => 64u32,
            Item::RedNetherBrickStairs => 64u32,
            Item::PolishedAndesiteStairs => 64u32,
            Item::DioriteStairs => 64u32,
            Item::CobbledDeepslateStairs => 64u32,
            Item::PolishedDeepslateStairs => 64u32,
            Item::DeepslateBrickStairs => 64u32,
            Item::DeepslateTileStairs => 64u32,
            Item::PolishedGraniteSlab => 64u32,
            Item::SmoothRedSandstoneSlab => 64u32,
            Item::MossyStoneBrickSlab => 64u32,
            Item::PolishedDioriteSlab => 64u32,
            Item::MossyCobblestoneSlab => 64u32,
            Item::EndStoneBrickSlab => 64u32,
            Item::SmoothSandstoneSlab => 64u32,
            Item::SmoothQuartzSlab => 64u32,
            Item::GraniteSlab => 64u32,
            Item::AndesiteSlab => 64u32,
            Item::RedNetherBrickSlab => 64u32,
            Item::PolishedAndesiteSlab => 64u32,
            Item::DioriteSlab => 64u32,
            Item::CobbledDeepslateSlab => 64u32,
            Item::PolishedDeepslateSlab => 64u32,
            Item::DeepslateBrickSlab => 64u32,
            Item::DeepslateTileSlab => 64u32,
            Item::Scaffolding => 64u32,
            Item::Redstone => 64u32,
            Item::RedstoneTorch => 64u32,
            Item::RedstoneBlock => 64u32,
            Item::Repeater => 64u32,
            Item::Comparator => 64u32,
            Item::Piston => 64u32,
            Item::StickyPiston => 64u32,
            Item::SlimeBlock => 64u32,
            Item::HoneyBlock => 64u32,
            Item::Observer => 64u32,
            Item::Hopper => 64u32,
            Item::Dispenser => 64u32,
            Item::Dropper => 64u32,
            Item::Lectern => 64u32,
            Item::Target => 64u32,
            Item::Lever => 64u32,
            Item::LightningRod => 64u32,
            Item::DaylightDetector => 64u32,
            Item::SculkSensor => 64u32,
            Item::TripwireHook => 64u32,
            Item::TrappedChest => 64u32,
            Item::Tnt => 64u32,
            Item::RedstoneLamp => 64u32,
            Item::NoteBlock => 64u32,
            Item::StoneButton => 64u32,
            Item::PolishedBlackstoneButton => 64u32,
            Item::OakButton => 64u32,
            Item::SpruceButton => 64u32,
            Item::BirchButton => 64u32,
            Item::JungleButton => 64u32,
            Item::AcaciaButton => 64u32,
            Item::DarkOakButton => 64u32,
            Item::CrimsonButton => 64u32,
            Item::WarpedButton => 64u32,
            Item::StonePressurePlate => 64u32,
            Item::PolishedBlackstonePressurePlate => 64u32,
            Item::LightWeightedPressurePlate => 64u32,
            Item::HeavyWeightedPressurePlate => 64u32,
            Item::OakPressurePlate => 64u32,
            Item::SprucePressurePlate => 64u32,
            Item::BirchPressurePlate => 64u32,
            Item::JunglePressurePlate => 64u32,
            Item::AcaciaPressurePlate => 64u32,
            Item::DarkOakPressurePlate => 64u32,
            Item::CrimsonPressurePlate => 64u32,
            Item::WarpedPressurePlate => 64u32,
            Item::IronDoor => 64u32,
            Item::OakDoor => 64u32,
            Item::SpruceDoor => 64u32,
            Item::BirchDoor => 64u32,
            Item::JungleDoor => 64u32,
            Item::AcaciaDoor => 64u32,
            Item::DarkOakDoor => 64u32,
            Item::CrimsonDoor => 64u32,
            Item::WarpedDoor => 64u32,
            Item::IronTrapdoor => 64u32,
            Item::OakTrapdoor => 64u32,
            Item::SpruceTrapdoor => 64u32,
            Item::BirchTrapdoor => 64u32,
            Item::JungleTrapdoor => 64u32,
            Item::AcaciaTrapdoor => 64u32,
            Item::DarkOakTrapdoor => 64u32,
            Item::CrimsonTrapdoor => 64u32,
            Item::WarpedTrapdoor => 64u32,
            Item::OakFenceGate => 64u32,
            Item::SpruceFenceGate => 64u32,
            Item::BirchFenceGate => 64u32,
            Item::JungleFenceGate => 64u32,
            Item::AcaciaFenceGate => 64u32,
            Item::DarkOakFenceGate => 64u32,
            Item::CrimsonFenceGate => 64u32,
            Item::WarpedFenceGate => 64u32,
            Item::PoweredRail => 64u32,
            Item::DetectorRail => 64u32,
            Item::Rail => 64u32,
            Item::ActivatorRail => 64u32,
            Item::Saddle => 1u32,
            Item::Minecart => 1u32,
            Item::ChestMinecart => 1u32,
            Item::FurnaceMinecart => 1u32,
            Item::TntMinecart => 1u32,
            Item::HopperMinecart => 1u32,
            Item::CarrotOnAStick => 1u32,
            Item::WarpedFungusOnAStick => 64u32,
            Item::Elytra => 1u32,
            Item::OakBoat => 1u32,
            Item::SpruceBoat => 1u32,
            Item::BirchBoat => 1u32,
            Item::JungleBoat => 1u32,
            Item::AcaciaBoat => 1u32,
            Item::DarkOakBoat => 1u32,
            Item::StructureBlock => 64u32,
            Item::Jigsaw => 64u32,
            Item::TurtleHelmet => 1u32,
            Item::Scute => 64u32,
            Item::FlintAndSteel => 1u32,
            Item::Apple => 64u32,
            Item::Bow => 1u32,
            Item::Arrow => 64u32,
            Item::Coal => 64u32,
            Item::Charcoal => 64u32,
            Item::Diamond => 64u32,
            Item::Emerald => 64u32,
            Item::LapisLazuli => 64u32,
            Item::Quartz => 64u32,
            Item::AmethystShard => 64u32,
            Item::RawIron => 64u32,
            Item::IronIngot => 64u32,
            Item::RawCopper => 64u32,
            Item::CopperIngot => 64u32,
            Item::RawGold => 64u32,
            Item::GoldIngot => 64u32,
            Item::NetheriteIngot => 64u32,
            Item::NetheriteScrap => 64u32,
            Item::WoodenSword => 1u32,
            Item::WoodenShovel => 1u32,
            Item::WoodenPickaxe => 1u32,
            Item::WoodenAxe => 1u32,
            Item::WoodenHoe => 1u32,
            Item::StoneSword => 1u32,
            Item::StoneShovel => 1u32,
            Item::StonePickaxe => 1u32,
            Item::StoneAxe => 1u32,
            Item::StoneHoe => 1u32,
            Item::GoldenSword => 1u32,
            Item::GoldenShovel => 1u32,
            Item::GoldenPickaxe => 1u32,
            Item::GoldenAxe => 1u32,
            Item::GoldenHoe => 1u32,
            Item::IronSword => 1u32,
            Item::IronShovel => 1u32,
            Item::IronPickaxe => 1u32,
            Item::IronAxe => 1u32,
            Item::IronHoe => 1u32,
            Item::DiamondSword => 1u32,
            Item::DiamondShovel => 1u32,
            Item::DiamondPickaxe => 1u32,
            Item::DiamondAxe => 1u32,
            Item::DiamondHoe => 1u32,
            Item::NetheriteSword => 1u32,
            Item::NetheriteShovel => 1u32,
            Item::NetheritePickaxe => 1u32,
            Item::NetheriteAxe => 1u32,
            Item::NetheriteHoe => 1u32,
            Item::Stick => 64u32,
            Item::Bowl => 64u32,
            Item::MushroomStew => 1u32,
            Item::String => 64u32,
            Item::Feather => 64u32,
            Item::Gunpowder => 64u32,
            Item::WheatSeeds => 64u32,
            Item::Wheat => 64u32,
            Item::Bread => 64u32,
            Item::LeatherHelmet => 1u32,
            Item::LeatherChestplate => 1u32,
            Item::LeatherLeggings => 1u32,
            Item::LeatherBoots => 1u32,
            Item::ChainmailHelmet => 1u32,
            Item::ChainmailChestplate => 1u32,
            Item::ChainmailLeggings => 1u32,
            Item::ChainmailBoots => 1u32,
            Item::IronHelmet => 1u32,
            Item::IronChestplate => 1u32,
            Item::IronLeggings => 1u32,
            Item::IronBoots => 1u32,
            Item::DiamondHelmet => 1u32,
            Item::DiamondChestplate => 1u32,
            Item::DiamondLeggings => 1u32,
            Item::DiamondBoots => 1u32,
            Item::GoldenHelmet => 1u32,
            Item::GoldenChestplate => 1u32,
            Item::GoldenLeggings => 1u32,
            Item::GoldenBoots => 1u32,
            Item::NetheriteHelmet => 1u32,
            Item::NetheriteChestplate => 1u32,
            Item::NetheriteLeggings => 1u32,
            Item::NetheriteBoots => 1u32,
            Item::Flint => 64u32,
            Item::Porkchop => 64u32,
            Item::CookedPorkchop => 64u32,
            Item::Painting => 64u32,
            Item::GoldenApple => 64u32,
            Item::EnchantedGoldenApple => 64u32,
            Item::OakSign => 16u32,
            Item::SpruceSign => 16u32,
            Item::BirchSign => 16u32,
            Item::JungleSign => 16u32,
            Item::AcaciaSign => 16u32,
            Item::DarkOakSign => 16u32,
            Item::CrimsonSign => 16u32,
            Item::WarpedSign => 16u32,
            Item::Bucket => 16u32,
            Item::WaterBucket => 1u32,
            Item::LavaBucket => 1u32,
            Item::PowderSnowBucket => 1u32,
            Item::Snowball => 16u32,
            Item::Leather => 64u32,
            Item::MilkBucket => 1u32,
            Item::PufferfishBucket => 1u32,
            Item::SalmonBucket => 1u32,
            Item::CodBucket => 1u32,
            Item::TropicalFishBucket => 1u32,
            Item::AxolotlBucket => 1u32,
            Item::Brick => 64u32,
            Item::ClayBall => 64u32,
            Item::DriedKelpBlock => 64u32,
            Item::Paper => 64u32,
            Item::Book => 64u32,
            Item::SlimeBall => 64u32,
            Item::Egg => 16u32,
            Item::Compass => 64u32,
            Item::Bundle => 1u32,
            Item::FishingRod => 1u32,
            Item::Clock => 64u32,
            Item::Spyglass => 1u32,
            Item::GlowstoneDust => 64u32,
            Item::Cod => 64u32,
            Item::Salmon => 64u32,
            Item::TropicalFish => 64u32,
            Item::Pufferfish => 64u32,
            Item::CookedCod => 64u32,
            Item::CookedSalmon => 64u32,
            Item::InkSac => 64u32,
            Item::GlowInkSac => 64u32,
            Item::CocoaBeans => 64u32,
            Item::WhiteDye => 64u32,
            Item::OrangeDye => 64u32,
            Item::MagentaDye => 64u32,
            Item::LightBlueDye => 64u32,
            Item::YellowDye => 64u32,
            Item::LimeDye => 64u32,
            Item::PinkDye => 64u32,
            Item::GrayDye => 64u32,
            Item::LightGrayDye => 64u32,
            Item::CyanDye => 64u32,
            Item::PurpleDye => 64u32,
            Item::BlueDye => 64u32,
            Item::BrownDye => 64u32,
            Item::GreenDye => 64u32,
            Item::RedDye => 64u32,
            Item::BlackDye => 64u32,
            Item::BoneMeal => 64u32,
            Item::Bone => 64u32,
            Item::Sugar => 64u32,
            Item::Cake => 1u32,
            Item::WhiteBed => 1u32,
            Item::OrangeBed => 1u32,
            Item::MagentaBed => 1u32,
            Item::LightBlueBed => 1u32,
            Item::YellowBed => 1u32,
            Item::LimeBed => 1u32,
            Item::PinkBed => 1u32,
            Item::GrayBed => 1u32,
            Item::LightGrayBed => 1u32,
            Item::CyanBed => 1u32,
            Item::PurpleBed => 1u32,
            Item::BlueBed => 1u32,
            Item::BrownBed => 1u32,
            Item::GreenBed => 1u32,
            Item::RedBed => 1u32,
            Item::BlackBed => 1u32,
            Item::Cookie => 64u32,
            Item::FilledMap => 64u32,
            Item::Shears => 1u32,
            Item::MelonSlice => 64u32,
            Item::DriedKelp => 64u32,
            Item::PumpkinSeeds => 64u32,
            Item::MelonSeeds => 64u32,
            Item::Beef => 64u32,
            Item::CookedBeef => 64u32,
            Item::Chicken => 64u32,
            Item::CookedChicken => 64u32,
            Item::RottenFlesh => 64u32,
            Item::EnderPearl => 16u32,
            Item::BlazeRod => 64u32,
            Item::GhastTear => 64u32,
            Item::GoldNugget => 64u32,
            Item::NetherWart => 64u32,
            Item::Potion => 1u32,
            Item::GlassBottle => 64u32,
            Item::SpiderEye => 64u32,
            Item::FermentedSpiderEye => 64u32,
            Item::BlazePowder => 64u32,
            Item::MagmaCream => 64u32,
            Item::BrewingStand => 64u32,
            Item::Cauldron => 64u32,
            Item::EnderEye => 64u32,
            Item::GlisteringMelonSlice => 64u32,
            Item::AxolotlSpawnEgg => 64u32,
            Item::BatSpawnEgg => 64u32,
            Item::BeeSpawnEgg => 64u32,
            Item::BlazeSpawnEgg => 64u32,
            Item::CatSpawnEgg => 64u32,
            Item::CaveSpiderSpawnEgg => 64u32,
            Item::ChickenSpawnEgg => 64u32,
            Item::CodSpawnEgg => 64u32,
            Item::CowSpawnEgg => 64u32,
            Item::CreeperSpawnEgg => 64u32,
            Item::DolphinSpawnEgg => 64u32,
            Item::DonkeySpawnEgg => 64u32,
            Item::DrownedSpawnEgg => 64u32,
            Item::ElderGuardianSpawnEgg => 64u32,
            Item::EndermanSpawnEgg => 64u32,
            Item::EndermiteSpawnEgg => 64u32,
            Item::EvokerSpawnEgg => 64u32,
            Item::FoxSpawnEgg => 64u32,
            Item::GhastSpawnEgg => 64u32,
            Item::GlowSquidSpawnEgg => 64u32,
            Item::GoatSpawnEgg => 64u32,
            Item::GuardianSpawnEgg => 64u32,
            Item::HoglinSpawnEgg => 64u32,
            Item::HorseSpawnEgg => 64u32,
            Item::HuskSpawnEgg => 64u32,
            Item::LlamaSpawnEgg => 64u32,
            Item::MagmaCubeSpawnEgg => 64u32,
            Item::MooshroomSpawnEgg => 64u32,
            Item::MuleSpawnEgg => 64u32,
            Item::OcelotSpawnEgg => 64u32,
            Item::PandaSpawnEgg => 64u32,
            Item::ParrotSpawnEgg => 64u32,
            Item::PhantomSpawnEgg => 64u32,
            Item::PigSpawnEgg => 64u32,
            Item::PiglinSpawnEgg => 64u32,
            Item::PiglinBruteSpawnEgg => 64u32,
            Item::PillagerSpawnEgg => 64u32,
            Item::PolarBearSpawnEgg => 64u32,
            Item::PufferfishSpawnEgg => 64u32,
            Item::RabbitSpawnEgg => 64u32,
            Item::RavagerSpawnEgg => 64u32,
            Item::SalmonSpawnEgg => 64u32,
            Item::SheepSpawnEgg => 64u32,
            Item::ShulkerSpawnEgg => 64u32,
            Item::SilverfishSpawnEgg => 64u32,
            Item::SkeletonSpawnEgg => 64u32,
            Item::SkeletonHorseSpawnEgg => 64u32,
            Item::SlimeSpawnEgg => 64u32,
            Item::SpiderSpawnEgg => 64u32,
            Item::SquidSpawnEgg => 64u32,
            Item::StraySpawnEgg => 64u32,
            Item::StriderSpawnEgg => 64u32,
            Item::TraderLlamaSpawnEgg => 64u32,
            Item::TropicalFishSpawnEgg => 64u32,
            Item::TurtleSpawnEgg => 64u32,
            Item::VexSpawnEgg => 64u32,
            Item::VillagerSpawnEgg => 64u32,
            Item::VindicatorSpawnEgg => 64u32,
            Item::WanderingTraderSpawnEgg => 64u32,
            Item::WitchSpawnEgg => 64u32,
            Item::WitherSkeletonSpawnEgg => 64u32,
            Item::WolfSpawnEgg => 64u32,
            Item::ZoglinSpawnEgg => 64u32,
            Item::ZombieSpawnEgg => 64u32,
            Item::ZombieHorseSpawnEgg => 64u32,
            Item::ZombieVillagerSpawnEgg => 64u32,
            Item::ZombifiedPiglinSpawnEgg => 64u32,
            Item::ExperienceBottle => 64u32,
            Item::FireCharge => 64u32,
            Item::WritableBook => 1u32,
            Item::WrittenBook => 16u32,
            Item::ItemFrame => 64u32,
            Item::GlowItemFrame => 64u32,
            Item::FlowerPot => 64u32,
            Item::Carrot => 64u32,
            Item::Potato => 64u32,
            Item::BakedPotato => 64u32,
            Item::PoisonousPotato => 64u32,
            Item::Map => 64u32,
            Item::GoldenCarrot => 64u32,
            Item::SkeletonSkull => 64u32,
            Item::WitherSkeletonSkull => 64u32,
            Item::PlayerHead => 64u32,
            Item::ZombieHead => 64u32,
            Item::CreeperHead => 64u32,
            Item::DragonHead => 64u32,
            Item::NetherStar => 64u32,
            Item::PumpkinPie => 64u32,
            Item::FireworkRocket => 64u32,
            Item::FireworkStar => 64u32,
            Item::EnchantedBook => 1u32,
            Item::NetherBrick => 64u32,
            Item::PrismarineShard => 64u32,
            Item::PrismarineCrystals => 64u32,
            Item::Rabbit => 64u32,
            Item::CookedRabbit => 64u32,
            Item::RabbitStew => 1u32,
            Item::RabbitFoot => 64u32,
            Item::RabbitHide => 64u32,
            Item::ArmorStand => 16u32,
            Item::IronHorseArmor => 1u32,
            Item::GoldenHorseArmor => 1u32,
            Item::DiamondHorseArmor => 1u32,
            Item::LeatherHorseArmor => 1u32,
            Item::Lead => 64u32,
            Item::NameTag => 64u32,
            Item::CommandBlockMinecart => 1u32,
            Item::Mutton => 64u32,
            Item::CookedMutton => 64u32,
            Item::WhiteBanner => 16u32,
            Item::OrangeBanner => 16u32,
            Item::MagentaBanner => 16u32,
            Item::LightBlueBanner => 16u32,
            Item::YellowBanner => 16u32,
            Item::LimeBanner => 16u32,
            Item::PinkBanner => 16u32,
            Item::GrayBanner => 16u32,
            Item::LightGrayBanner => 16u32,
            Item::CyanBanner => 16u32,
            Item::PurpleBanner => 16u32,
            Item::BlueBanner => 16u32,
            Item::BrownBanner => 16u32,
            Item::GreenBanner => 16u32,
            Item::RedBanner => 16u32,
            Item::BlackBanner => 16u32,
            Item::EndCrystal => 64u32,
            Item::ChorusFruit => 64u32,
            Item::PoppedChorusFruit => 64u32,
            Item::Beetroot => 64u32,
            Item::BeetrootSeeds => 64u32,
            Item::BeetrootSoup => 1u32,
            Item::DragonBreath => 64u32,
            Item::SplashPotion => 1u32,
            Item::SpectralArrow => 64u32,
            Item::TippedArrow => 64u32,
            Item::LingeringPotion => 1u32,
            Item::Shield => 1u32,
            Item::TotemOfUndying => 1u32,
            Item::ShulkerShell => 64u32,
            Item::IronNugget => 64u32,
            Item::KnowledgeBook => 1u32,
            Item::DebugStick => 1u32,
            Item::MusicDisc13 => 1u32,
            Item::MusicDiscCat => 1u32,
            Item::MusicDiscBlocks => 1u32,
            Item::MusicDiscChirp => 1u32,
            Item::MusicDiscFar => 1u32,
            Item::MusicDiscMall => 1u32,
            Item::MusicDiscMellohi => 1u32,
            Item::MusicDiscStal => 1u32,
            Item::MusicDiscStrad => 1u32,
            Item::MusicDiscWard => 1u32,
            Item::MusicDisc11 => 1u32,
            Item::MusicDiscWait => 1u32,
            Item::MusicDiscOtherside => 1u32,
            Item::MusicDiscPigstep => 1u32,
            Item::Trident => 1u32,
            Item::PhantomMembrane => 64u32,
            Item::NautilusShell => 64u32,
            Item::HeartOfTheSea => 64u32,
            Item::Crossbow => 1u32,
            Item::SuspiciousStew => 1u32,
            Item::Loom => 64u32,
            Item::FlowerBannerPattern => 1u32,
            Item::CreeperBannerPattern => 1u32,
            Item::SkullBannerPattern => 1u32,
            Item::MojangBannerPattern => 1u32,
            Item::GlobeBannerPattern => 1u32,
            Item::PiglinBannerPattern => 1u32,
            Item::Composter => 64u32,
            Item::Barrel => 64u32,
            Item::Smoker => 64u32,
            Item::BlastFurnace => 64u32,
            Item::CartographyTable => 64u32,
            Item::FletchingTable => 64u32,
            Item::Grindstone => 64u32,
            Item::SmithingTable => 64u32,
            Item::Stonecutter => 64u32,
            Item::Bell => 64u32,
            Item::Lantern => 64u32,
            Item::SoulLantern => 64u32,
            Item::SweetBerries => 64u32,
            Item::GlowBerries => 64u32,
            Item::Campfire => 64u32,
            Item::SoulCampfire => 64u32,
            Item::Shroomlight => 64u32,
            Item::Honeycomb => 64u32,
            Item::BeeNest => 64u32,
            Item::Beehive => 64u32,
            Item::HoneyBottle => 16u32,
            Item::HoneycombBlock => 64u32,
            Item::Lodestone => 64u32,
            Item::CryingObsidian => 64u32,
            Item::Blackstone => 64u32,
            Item::BlackstoneSlab => 64u32,
            Item::BlackstoneStairs => 64u32,
            Item::GildedBlackstone => 64u32,
            Item::PolishedBlackstone => 64u32,
            Item::PolishedBlackstoneSlab => 64u32,
            Item::PolishedBlackstoneStairs => 64u32,
            Item::ChiseledPolishedBlackstone => 64u32,
            Item::PolishedBlackstoneBricks => 64u32,
            Item::PolishedBlackstoneBrickSlab => 64u32,
            Item::PolishedBlackstoneBrickStairs => 64u32,
            Item::CrackedPolishedBlackstoneBricks => 64u32,
            Item::RespawnAnchor => 64u32,
            Item::Candle => 64u32,
            Item::WhiteCandle => 64u32,
            Item::OrangeCandle => 64u32,
            Item::MagentaCandle => 64u32,
            Item::LightBlueCandle => 64u32,
            Item::YellowCandle => 64u32,
            Item::LimeCandle => 64u32,
            Item::PinkCandle => 64u32,
            Item::GrayCandle => 64u32,
            Item::LightGrayCandle => 64u32,
            Item::CyanCandle => 64u32,
            Item::PurpleCandle => 64u32,
            Item::BlueCandle => 64u32,
            Item::BrownCandle => 64u32,
            Item::GreenCandle => 64u32,
            Item::RedCandle => 64u32,
            Item::BlackCandle => 64u32,
            Item::SmallAmethystBud => 64u32,
            Item::MediumAmethystBud => 64u32,
            Item::LargeAmethystBud => 64u32,
            Item::AmethystCluster => 64u32,
            Item::PointedDripstone => 64u32,
        }
    }
}
impl Item {
    #[doc = "Returns the `max_durability` property of this `Item`."]
    #[inline]
    pub fn max_durability(&self) -> Option<u32> {
        match self {
            Item::Stone => None,
            Item::Granite => None,
            Item::PolishedGranite => None,
            Item::Diorite => None,
            Item::PolishedDiorite => None,
            Item::Andesite => None,
            Item::PolishedAndesite => None,
            Item::Deepslate => None,
            Item::CobbledDeepslate => None,
            Item::PolishedDeepslate => None,
            Item::Calcite => None,
            Item::Tuff => None,
            Item::DripstoneBlock => None,
            Item::GrassBlock => None,
            Item::Dirt => None,
            Item::CoarseDirt => None,
            Item::Podzol => None,
            Item::RootedDirt => None,
            Item::CrimsonNylium => None,
            Item::WarpedNylium => None,
            Item::Cobblestone => None,
            Item::OakPlanks => None,
            Item::SprucePlanks => None,
            Item::BirchPlanks => None,
            Item::JunglePlanks => None,
            Item::AcaciaPlanks => None,
            Item::DarkOakPlanks => None,
            Item::CrimsonPlanks => None,
            Item::WarpedPlanks => None,
            Item::OakSapling => None,
            Item::SpruceSapling => None,
            Item::BirchSapling => None,
            Item::JungleSapling => None,
            Item::AcaciaSapling => None,
            Item::DarkOakSapling => None,
            Item::Bedrock => None,
            Item::Sand => None,
            Item::RedSand => None,
            Item::Gravel => None,
            Item::CoalOre => None,
            Item::DeepslateCoalOre => None,
            Item::IronOre => None,
            Item::DeepslateIronOre => None,
            Item::CopperOre => None,
            Item::DeepslateCopperOre => None,
            Item::GoldOre => None,
            Item::DeepslateGoldOre => None,
            Item::RedstoneOre => None,
            Item::DeepslateRedstoneOre => None,
            Item::EmeraldOre => None,
            Item::DeepslateEmeraldOre => None,
            Item::LapisOre => None,
            Item::DeepslateLapisOre => None,
            Item::DiamondOre => None,
            Item::DeepslateDiamondOre => None,
            Item::NetherGoldOre => None,
            Item::NetherQuartzOre => None,
            Item::AncientDebris => None,
            Item::CoalBlock => None,
            Item::RawIronBlock => None,
            Item::RawCopperBlock => None,
            Item::RawGoldBlock => None,
            Item::AmethystBlock => None,
            Item::BuddingAmethyst => None,
            Item::IronBlock => None,
            Item::CopperBlock => None,
            Item::GoldBlock => None,
            Item::DiamondBlock => None,
            Item::NetheriteBlock => None,
            Item::ExposedCopper => None,
            Item::WeatheredCopper => None,
            Item::OxidizedCopper => None,
            Item::CutCopper => None,
            Item::ExposedCutCopper => None,
            Item::WeatheredCutCopper => None,
            Item::OxidizedCutCopper => None,
            Item::CutCopperStairs => None,
            Item::ExposedCutCopperStairs => None,
            Item::WeatheredCutCopperStairs => None,
            Item::OxidizedCutCopperStairs => None,
            Item::CutCopperSlab => None,
            Item::ExposedCutCopperSlab => None,
            Item::WeatheredCutCopperSlab => None,
            Item::OxidizedCutCopperSlab => None,
            Item::WaxedCopperBlock => None,
            Item::WaxedExposedCopper => None,
            Item::WaxedWeatheredCopper => None,
            Item::WaxedOxidizedCopper => None,
            Item::WaxedCutCopper => None,
            Item::WaxedExposedCutCopper => None,
            Item::WaxedWeatheredCutCopper => None,
            Item::WaxedOxidizedCutCopper => None,
            Item::WaxedCutCopperStairs => None,
            Item::WaxedExposedCutCopperStairs => None,
            Item::WaxedWeatheredCutCopperStairs => None,
            Item::WaxedOxidizedCutCopperStairs => None,
            Item::WaxedCutCopperSlab => None,
            Item::WaxedExposedCutCopperSlab => None,
            Item::WaxedWeatheredCutCopperSlab => None,
            Item::WaxedOxidizedCutCopperSlab => None,
            Item::OakLog => None,
            Item::SpruceLog => None,
            Item::BirchLog => None,
            Item::JungleLog => None,
            Item::AcaciaLog => None,
            Item::DarkOakLog => None,
            Item::CrimsonStem => None,
            Item::WarpedStem => None,
            Item::StrippedOakLog => None,
            Item::StrippedSpruceLog => None,
            Item::StrippedBirchLog => None,
            Item::StrippedJungleLog => None,
            Item::StrippedAcaciaLog => None,
            Item::StrippedDarkOakLog => None,
            Item::StrippedCrimsonStem => None,
            Item::StrippedWarpedStem => None,
            Item::StrippedOakWood => None,
            Item::StrippedSpruceWood => None,
            Item::StrippedBirchWood => None,
            Item::StrippedJungleWood => None,
            Item::StrippedAcaciaWood => None,
            Item::StrippedDarkOakWood => None,
            Item::StrippedCrimsonHyphae => None,
            Item::StrippedWarpedHyphae => None,
            Item::OakWood => None,
            Item::SpruceWood => None,
            Item::BirchWood => None,
            Item::JungleWood => None,
            Item::AcaciaWood => None,
            Item::DarkOakWood => None,
            Item::CrimsonHyphae => None,
            Item::WarpedHyphae => None,
            Item::OakLeaves => None,
            Item::SpruceLeaves => None,
            Item::BirchLeaves => None,
            Item::JungleLeaves => None,
            Item::AcaciaLeaves => None,
            Item::DarkOakLeaves => None,
            Item::AzaleaLeaves => None,
            Item::FloweringAzaleaLeaves => None,
            Item::Sponge => None,
            Item::WetSponge => None,
            Item::Glass => None,
            Item::TintedGlass => None,
            Item::LapisBlock => None,
            Item::Sandstone => None,
            Item::ChiseledSandstone => None,
            Item::CutSandstone => None,
            Item::Cobweb => None,
            Item::Grass => None,
            Item::Fern => None,
            Item::Azalea => None,
            Item::FloweringAzalea => None,
            Item::DeadBush => None,
            Item::Seagrass => None,
            Item::SeaPickle => None,
            Item::WhiteWool => None,
            Item::OrangeWool => None,
            Item::MagentaWool => None,
            Item::LightBlueWool => None,
            Item::YellowWool => None,
            Item::LimeWool => None,
            Item::PinkWool => None,
            Item::GrayWool => None,
            Item::LightGrayWool => None,
            Item::CyanWool => None,
            Item::PurpleWool => None,
            Item::BlueWool => None,
            Item::BrownWool => None,
            Item::GreenWool => None,
            Item::RedWool => None,
            Item::BlackWool => None,
            Item::Dandelion => None,
            Item::Poppy => None,
            Item::BlueOrchid => None,
            Item::Allium => None,
            Item::AzureBluet => None,
            Item::RedTulip => None,
            Item::OrangeTulip => None,
            Item::WhiteTulip => None,
            Item::PinkTulip => None,
            Item::OxeyeDaisy => None,
            Item::Cornflower => None,
            Item::LilyOfTheValley => None,
            Item::WitherRose => None,
            Item::SporeBlossom => None,
            Item::BrownMushroom => None,
            Item::RedMushroom => None,
            Item::CrimsonFungus => None,
            Item::WarpedFungus => None,
            Item::CrimsonRoots => None,
            Item::WarpedRoots => None,
            Item::NetherSprouts => None,
            Item::WeepingVines => None,
            Item::TwistingVines => None,
            Item::SugarCane => None,
            Item::Kelp => None,
            Item::MossCarpet => None,
            Item::MossBlock => None,
            Item::HangingRoots => None,
            Item::BigDripleaf => None,
            Item::SmallDripleaf => None,
            Item::Bamboo => None,
            Item::OakSlab => None,
            Item::SpruceSlab => None,
            Item::BirchSlab => None,
            Item::JungleSlab => None,
            Item::AcaciaSlab => None,
            Item::DarkOakSlab => None,
            Item::CrimsonSlab => None,
            Item::WarpedSlab => None,
            Item::StoneSlab => None,
            Item::SmoothStoneSlab => None,
            Item::SandstoneSlab => None,
            Item::CutSandstoneSlab => None,
            Item::PetrifiedOakSlab => None,
            Item::CobblestoneSlab => None,
            Item::BrickSlab => None,
            Item::StoneBrickSlab => None,
            Item::NetherBrickSlab => None,
            Item::QuartzSlab => None,
            Item::RedSandstoneSlab => None,
            Item::CutRedSandstoneSlab => None,
            Item::PurpurSlab => None,
            Item::PrismarineSlab => None,
            Item::PrismarineBrickSlab => None,
            Item::DarkPrismarineSlab => None,
            Item::SmoothQuartz => None,
            Item::SmoothRedSandstone => None,
            Item::SmoothSandstone => None,
            Item::SmoothStone => None,
            Item::Bricks => None,
            Item::Bookshelf => None,
            Item::MossyCobblestone => None,
            Item::Obsidian => None,
            Item::Torch => None,
            Item::EndRod => None,
            Item::ChorusPlant => None,
            Item::ChorusFlower => None,
            Item::PurpurBlock => None,
            Item::PurpurPillar => None,
            Item::PurpurStairs => None,
            Item::Spawner => None,
            Item::OakStairs => None,
            Item::Chest => None,
            Item::CraftingTable => None,
            Item::Farmland => None,
            Item::Furnace => None,
            Item::Ladder => None,
            Item::CobblestoneStairs => None,
            Item::Snow => None,
            Item::Ice => None,
            Item::SnowBlock => None,
            Item::Cactus => None,
            Item::Clay => None,
            Item::Jukebox => None,
            Item::OakFence => None,
            Item::SpruceFence => None,
            Item::BirchFence => None,
            Item::JungleFence => None,
            Item::AcaciaFence => None,
            Item::DarkOakFence => None,
            Item::CrimsonFence => None,
            Item::WarpedFence => None,
            Item::Pumpkin => None,
            Item::CarvedPumpkin => None,
            Item::JackOLantern => None,
            Item::Netherrack => None,
            Item::SoulSand => None,
            Item::SoulSoil => None,
            Item::Basalt => None,
            Item::PolishedBasalt => None,
            Item::SmoothBasalt => None,
            Item::SoulTorch => None,
            Item::Glowstone => None,
            Item::InfestedStone => None,
            Item::InfestedCobblestone => None,
            Item::InfestedStoneBricks => None,
            Item::InfestedMossyStoneBricks => None,
            Item::InfestedCrackedStoneBricks => None,
            Item::InfestedChiseledStoneBricks => None,
            Item::InfestedDeepslate => None,
            Item::StoneBricks => None,
            Item::MossyStoneBricks => None,
            Item::CrackedStoneBricks => None,
            Item::ChiseledStoneBricks => None,
            Item::DeepslateBricks => None,
            Item::CrackedDeepslateBricks => None,
            Item::DeepslateTiles => None,
            Item::CrackedDeepslateTiles => None,
            Item::ChiseledDeepslate => None,
            Item::BrownMushroomBlock => None,
            Item::RedMushroomBlock => None,
            Item::MushroomStem => None,
            Item::IronBars => None,
            Item::Chain => None,
            Item::GlassPane => None,
            Item::Melon => None,
            Item::Vine => None,
            Item::GlowLichen => None,
            Item::BrickStairs => None,
            Item::StoneBrickStairs => None,
            Item::Mycelium => None,
            Item::LilyPad => None,
            Item::NetherBricks => None,
            Item::CrackedNetherBricks => None,
            Item::ChiseledNetherBricks => None,
            Item::NetherBrickFence => None,
            Item::NetherBrickStairs => None,
            Item::EnchantingTable => None,
            Item::EndPortalFrame => None,
            Item::EndStone => None,
            Item::EndStoneBricks => None,
            Item::DragonEgg => None,
            Item::SandstoneStairs => None,
            Item::EnderChest => None,
            Item::EmeraldBlock => None,
            Item::SpruceStairs => None,
            Item::BirchStairs => None,
            Item::JungleStairs => None,
            Item::CrimsonStairs => None,
            Item::WarpedStairs => None,
            Item::CommandBlock => None,
            Item::Beacon => None,
            Item::CobblestoneWall => None,
            Item::MossyCobblestoneWall => None,
            Item::BrickWall => None,
            Item::PrismarineWall => None,
            Item::RedSandstoneWall => None,
            Item::MossyStoneBrickWall => None,
            Item::GraniteWall => None,
            Item::StoneBrickWall => None,
            Item::NetherBrickWall => None,
            Item::AndesiteWall => None,
            Item::RedNetherBrickWall => None,
            Item::SandstoneWall => None,
            Item::EndStoneBrickWall => None,
            Item::DioriteWall => None,
            Item::BlackstoneWall => None,
            Item::PolishedBlackstoneWall => None,
            Item::PolishedBlackstoneBrickWall => None,
            Item::CobbledDeepslateWall => None,
            Item::PolishedDeepslateWall => None,
            Item::DeepslateBrickWall => None,
            Item::DeepslateTileWall => None,
            Item::Anvil => None,
            Item::ChippedAnvil => None,
            Item::DamagedAnvil => None,
            Item::ChiseledQuartzBlock => None,
            Item::QuartzBlock => None,
            Item::QuartzBricks => None,
            Item::QuartzPillar => None,
            Item::QuartzStairs => None,
            Item::WhiteTerracotta => None,
            Item::OrangeTerracotta => None,
            Item::MagentaTerracotta => None,
            Item::LightBlueTerracotta => None,
            Item::YellowTerracotta => None,
            Item::LimeTerracotta => None,
            Item::PinkTerracotta => None,
            Item::GrayTerracotta => None,
            Item::LightGrayTerracotta => None,
            Item::CyanTerracotta => None,
            Item::PurpleTerracotta => None,
            Item::BlueTerracotta => None,
            Item::BrownTerracotta => None,
            Item::GreenTerracotta => None,
            Item::RedTerracotta => None,
            Item::BlackTerracotta => None,
            Item::Barrier => None,
            Item::Light => None,
            Item::HayBlock => None,
            Item::WhiteCarpet => None,
            Item::OrangeCarpet => None,
            Item::MagentaCarpet => None,
            Item::LightBlueCarpet => None,
            Item::YellowCarpet => None,
            Item::LimeCarpet => None,
            Item::PinkCarpet => None,
            Item::GrayCarpet => None,
            Item::LightGrayCarpet => None,
            Item::CyanCarpet => None,
            Item::PurpleCarpet => None,
            Item::BlueCarpet => None,
            Item::BrownCarpet => None,
            Item::GreenCarpet => None,
            Item::RedCarpet => None,
            Item::BlackCarpet => None,
            Item::Terracotta => None,
            Item::PackedIce => None,
            Item::AcaciaStairs => None,
            Item::DarkOakStairs => None,
            Item::DirtPath => None,
            Item::Sunflower => None,
            Item::Lilac => None,
            Item::RoseBush => None,
            Item::Peony => None,
            Item::TallGrass => None,
            Item::LargeFern => None,
            Item::WhiteStainedGlass => None,
            Item::OrangeStainedGlass => None,
            Item::MagentaStainedGlass => None,
            Item::LightBlueStainedGlass => None,
            Item::YellowStainedGlass => None,
            Item::LimeStainedGlass => None,
            Item::PinkStainedGlass => None,
            Item::GrayStainedGlass => None,
            Item::LightGrayStainedGlass => None,
            Item::CyanStainedGlass => None,
            Item::PurpleStainedGlass => None,
            Item::BlueStainedGlass => None,
            Item::BrownStainedGlass => None,
            Item::GreenStainedGlass => None,
            Item::RedStainedGlass => None,
            Item::BlackStainedGlass => None,
            Item::WhiteStainedGlassPane => None,
            Item::OrangeStainedGlassPane => None,
            Item::MagentaStainedGlassPane => None,
            Item::LightBlueStainedGlassPane => None,
            Item::YellowStainedGlassPane => None,
            Item::LimeStainedGlassPane => None,
            Item::PinkStainedGlassPane => None,
            Item::GrayStainedGlassPane => None,
            Item::LightGrayStainedGlassPane => None,
            Item::CyanStainedGlassPane => None,
            Item::PurpleStainedGlassPane => None,
            Item::BlueStainedGlassPane => None,
            Item::BrownStainedGlassPane => None,
            Item::GreenStainedGlassPane => None,
            Item::RedStainedGlassPane => None,
            Item::BlackStainedGlassPane => None,
            Item::Prismarine => None,
            Item::PrismarineBricks => None,
            Item::DarkPrismarine => None,
            Item::PrismarineStairs => None,
            Item::PrismarineBrickStairs => None,
            Item::DarkPrismarineStairs => None,
            Item::SeaLantern => None,
            Item::RedSandstone => None,
            Item::ChiseledRedSandstone => None,
            Item::CutRedSandstone => None,
            Item::RedSandstoneStairs => None,
            Item::RepeatingCommandBlock => None,
            Item::ChainCommandBlock => None,
            Item::MagmaBlock => None,
            Item::NetherWartBlock => None,
            Item::WarpedWartBlock => None,
            Item::RedNetherBricks => None,
            Item::BoneBlock => None,
            Item::StructureVoid => None,
            Item::ShulkerBox => None,
            Item::WhiteShulkerBox => None,
            Item::OrangeShulkerBox => None,
            Item::MagentaShulkerBox => None,
            Item::LightBlueShulkerBox => None,
            Item::YellowShulkerBox => None,
            Item::LimeShulkerBox => None,
            Item::PinkShulkerBox => None,
            Item::GrayShulkerBox => None,
            Item::LightGrayShulkerBox => None,
            Item::CyanShulkerBox => None,
            Item::PurpleShulkerBox => None,
            Item::BlueShulkerBox => None,
            Item::BrownShulkerBox => None,
            Item::GreenShulkerBox => None,
            Item::RedShulkerBox => None,
            Item::BlackShulkerBox => None,
            Item::WhiteGlazedTerracotta => None,
            Item::OrangeGlazedTerracotta => None,
            Item::MagentaGlazedTerracotta => None,
            Item::LightBlueGlazedTerracotta => None,
            Item::YellowGlazedTerracotta => None,
            Item::LimeGlazedTerracotta => None,
            Item::PinkGlazedTerracotta => None,
            Item::GrayGlazedTerracotta => None,
            Item::LightGrayGlazedTerracotta => None,
            Item::CyanGlazedTerracotta => None,
            Item::PurpleGlazedTerracotta => None,
            Item::BlueGlazedTerracotta => None,
            Item::BrownGlazedTerracotta => None,
            Item::GreenGlazedTerracotta => None,
            Item::RedGlazedTerracotta => None,
            Item::BlackGlazedTerracotta => None,
            Item::WhiteConcrete => None,
            Item::OrangeConcrete => None,
            Item::MagentaConcrete => None,
            Item::LightBlueConcrete => None,
            Item::YellowConcrete => None,
            Item::LimeConcrete => None,
            Item::PinkConcrete => None,
            Item::GrayConcrete => None,
            Item::LightGrayConcrete => None,
            Item::CyanConcrete => None,
            Item::PurpleConcrete => None,
            Item::BlueConcrete => None,
            Item::BrownConcrete => None,
            Item::GreenConcrete => None,
            Item::RedConcrete => None,
            Item::BlackConcrete => None,
            Item::WhiteConcretePowder => None,
            Item::OrangeConcretePowder => None,
            Item::MagentaConcretePowder => None,
            Item::LightBlueConcretePowder => None,
            Item::YellowConcretePowder => None,
            Item::LimeConcretePowder => None,
            Item::PinkConcretePowder => None,
            Item::GrayConcretePowder => None,
            Item::LightGrayConcretePowder => None,
            Item::CyanConcretePowder => None,
            Item::PurpleConcretePowder => None,
            Item::BlueConcretePowder => None,
            Item::BrownConcretePowder => None,
            Item::GreenConcretePowder => None,
            Item::RedConcretePowder => None,
            Item::BlackConcretePowder => None,
            Item::TurtleEgg => None,
            Item::DeadTubeCoralBlock => None,
            Item::DeadBrainCoralBlock => None,
            Item::DeadBubbleCoralBlock => None,
            Item::DeadFireCoralBlock => None,
            Item::DeadHornCoralBlock => None,
            Item::TubeCoralBlock => None,
            Item::BrainCoralBlock => None,
            Item::BubbleCoralBlock => None,
            Item::FireCoralBlock => None,
            Item::HornCoralBlock => None,
            Item::TubeCoral => None,
            Item::BrainCoral => None,
            Item::BubbleCoral => None,
            Item::FireCoral => None,
            Item::HornCoral => None,
            Item::DeadBrainCoral => None,
            Item::DeadBubbleCoral => None,
            Item::DeadFireCoral => None,
            Item::DeadHornCoral => None,
            Item::DeadTubeCoral => None,
            Item::TubeCoralFan => None,
            Item::BrainCoralFan => None,
            Item::BubbleCoralFan => None,
            Item::FireCoralFan => None,
            Item::HornCoralFan => None,
            Item::DeadTubeCoralFan => None,
            Item::DeadBrainCoralFan => None,
            Item::DeadBubbleCoralFan => None,
            Item::DeadFireCoralFan => None,
            Item::DeadHornCoralFan => None,
            Item::BlueIce => None,
            Item::Conduit => None,
            Item::PolishedGraniteStairs => None,
            Item::SmoothRedSandstoneStairs => None,
            Item::MossyStoneBrickStairs => None,
            Item::PolishedDioriteStairs => None,
            Item::MossyCobblestoneStairs => None,
            Item::EndStoneBrickStairs => None,
            Item::StoneStairs => None,
            Item::SmoothSandstoneStairs => None,
            Item::SmoothQuartzStairs => None,
            Item::GraniteStairs => None,
            Item::AndesiteStairs => None,
            Item::RedNetherBrickStairs => None,
            Item::PolishedAndesiteStairs => None,
            Item::DioriteStairs => None,
            Item::CobbledDeepslateStairs => None,
            Item::PolishedDeepslateStairs => None,
            Item::DeepslateBrickStairs => None,
            Item::DeepslateTileStairs => None,
            Item::PolishedGraniteSlab => None,
            Item::SmoothRedSandstoneSlab => None,
            Item::MossyStoneBrickSlab => None,
            Item::PolishedDioriteSlab => None,
            Item::MossyCobblestoneSlab => None,
            Item::EndStoneBrickSlab => None,
            Item::SmoothSandstoneSlab => None,
            Item::SmoothQuartzSlab => None,
            Item::GraniteSlab => None,
            Item::AndesiteSlab => None,
            Item::RedNetherBrickSlab => None,
            Item::PolishedAndesiteSlab => None,
            Item::DioriteSlab => None,
            Item::CobbledDeepslateSlab => None,
            Item::PolishedDeepslateSlab => None,
            Item::DeepslateBrickSlab => None,
            Item::DeepslateTileSlab => None,
            Item::Scaffolding => None,
            Item::Redstone => None,
            Item::RedstoneTorch => None,
            Item::RedstoneBlock => None,
            Item::Repeater => None,
            Item::Comparator => None,
            Item::Piston => None,
            Item::StickyPiston => None,
            Item::SlimeBlock => None,
            Item::HoneyBlock => None,
            Item::Observer => None,
            Item::Hopper => None,
            Item::Dispenser => None,
            Item::Dropper => None,
            Item::Lectern => None,
            Item::Target => None,
            Item::Lever => None,
            Item::LightningRod => None,
            Item::DaylightDetector => None,
            Item::SculkSensor => None,
            Item::TripwireHook => None,
            Item::TrappedChest => None,
            Item::Tnt => None,
            Item::RedstoneLamp => None,
            Item::NoteBlock => None,
            Item::StoneButton => None,
            Item::PolishedBlackstoneButton => None,
            Item::OakButton => None,
            Item::SpruceButton => None,
            Item::BirchButton => None,
            Item::JungleButton => None,
            Item::AcaciaButton => None,
            Item::DarkOakButton => None,
            Item::CrimsonButton => None,
            Item::WarpedButton => None,
            Item::StonePressurePlate => None,
            Item::PolishedBlackstonePressurePlate => None,
            Item::LightWeightedPressurePlate => None,
            Item::HeavyWeightedPressurePlate => None,
            Item::OakPressurePlate => None,
            Item::SprucePressurePlate => None,
            Item::BirchPressurePlate => None,
            Item::JunglePressurePlate => None,
            Item::AcaciaPressurePlate => None,
            Item::DarkOakPressurePlate => None,
            Item::CrimsonPressurePlate => None,
            Item::WarpedPressurePlate => None,
            Item::IronDoor => None,
            Item::OakDoor => None,
            Item::SpruceDoor => None,
            Item::BirchDoor => None,
            Item::JungleDoor => None,
            Item::AcaciaDoor => None,
            Item::DarkOakDoor => None,
            Item::CrimsonDoor => None,
            Item::WarpedDoor => None,
            Item::IronTrapdoor => None,
            Item::OakTrapdoor => None,
            Item::SpruceTrapdoor => None,
            Item::BirchTrapdoor => None,
            Item::JungleTrapdoor => None,
            Item::AcaciaTrapdoor => None,
            Item::DarkOakTrapdoor => None,
            Item::CrimsonTrapdoor => None,
            Item::WarpedTrapdoor => None,
            Item::OakFenceGate => None,
            Item::SpruceFenceGate => None,
            Item::BirchFenceGate => None,
            Item::JungleFenceGate => None,
            Item::AcaciaFenceGate => None,
            Item::DarkOakFenceGate => None,
            Item::CrimsonFenceGate => None,
            Item::WarpedFenceGate => None,
            Item::PoweredRail => None,
            Item::DetectorRail => None,
            Item::Rail => None,
            Item::ActivatorRail => None,
            Item::Saddle => None,
            Item::Minecart => None,
            Item::ChestMinecart => None,
            Item::FurnaceMinecart => None,
            Item::TntMinecart => None,
            Item::HopperMinecart => None,
            Item::CarrotOnAStick => Some(25u32),
            Item::WarpedFungusOnAStick => Some(100u32),
            Item::Elytra => Some(432u32),
            Item::OakBoat => None,
            Item::SpruceBoat => None,
            Item::BirchBoat => None,
            Item::JungleBoat => None,
            Item::AcaciaBoat => None,
            Item::DarkOakBoat => None,
            Item::StructureBlock => None,
            Item::Jigsaw => None,
            Item::TurtleHelmet => Some(275u32),
            Item::Scute => None,
            Item::FlintAndSteel => Some(64u32),
            Item::Apple => None,
            Item::Bow => Some(384u32),
            Item::Arrow => None,
            Item::Coal => None,
            Item::Charcoal => None,
            Item::Diamond => None,
            Item::Emerald => None,
            Item::LapisLazuli => None,
            Item::Quartz => None,
            Item::AmethystShard => None,
            Item::RawIron => None,
            Item::IronIngot => None,
            Item::RawCopper => None,
            Item::CopperIngot => None,
            Item::RawGold => None,
            Item::GoldIngot => None,
            Item::NetheriteIngot => None,
            Item::NetheriteScrap => None,
            Item::WoodenSword => Some(59u32),
            Item::WoodenShovel => Some(59u32),
            Item::WoodenPickaxe => Some(59u32),
            Item::WoodenAxe => Some(59u32),
            Item::WoodenHoe => Some(59u32),
            Item::StoneSword => Some(131u32),
            Item::StoneShovel => Some(131u32),
            Item::StonePickaxe => Some(131u32),
            Item::StoneAxe => Some(131u32),
            Item::StoneHoe => Some(131u32),
            Item::GoldenSword => Some(32u32),
            Item::GoldenShovel => Some(32u32),
            Item::GoldenPickaxe => Some(32u32),
            Item::GoldenAxe => Some(32u32),
            Item::GoldenHoe => Some(32u32),
            Item::IronSword => Some(250u32),
            Item::IronShovel => Some(250u32),
            Item::IronPickaxe => Some(250u32),
            Item::IronAxe => Some(250u32),
            Item::IronHoe => Some(250u32),
            Item::DiamondSword => Some(1561u32),
            Item::DiamondShovel => Some(1561u32),
            Item::DiamondPickaxe => Some(1561u32),
            Item::DiamondAxe => Some(1561u32),
            Item::DiamondHoe => Some(1561u32),
            Item::NetheriteSword => Some(2031u32),
            Item::NetheriteShovel => Some(2031u32),
            Item::NetheritePickaxe => Some(2031u32),
            Item::NetheriteAxe => Some(2031u32),
            Item::NetheriteHoe => Some(2031u32),
            Item::Stick => None,
            Item::Bowl => None,
            Item::MushroomStew => None,
            Item::String => None,
            Item::Feather => None,
            Item::Gunpowder => None,
            Item::WheatSeeds => None,
            Item::Wheat => None,
            Item::Bread => None,
            Item::LeatherHelmet => Some(55u32),
            Item::LeatherChestplate => Some(80u32),
            Item::LeatherLeggings => Some(75u32),
            Item::LeatherBoots => Some(65u32),
            Item::ChainmailHelmet => Some(165u32),
            Item::ChainmailChestplate => Some(240u32),
            Item::ChainmailLeggings => Some(225u32),
            Item::ChainmailBoots => Some(195u32),
            Item::IronHelmet => Some(165u32),
            Item::IronChestplate => Some(240u32),
            Item::IronLeggings => Some(225u32),
            Item::IronBoots => Some(195u32),
            Item::DiamondHelmet => Some(363u32),
            Item::DiamondChestplate => Some(528u32),
            Item::DiamondLeggings => Some(495u32),
            Item::DiamondBoots => Some(429u32),
            Item::GoldenHelmet => Some(77u32),
            Item::GoldenChestplate => Some(112u32),
            Item::GoldenLeggings => Some(105u32),
            Item::GoldenBoots => Some(91u32),
            Item::NetheriteHelmet => Some(407u32),
            Item::NetheriteChestplate => Some(592u32),
            Item::NetheriteLeggings => Some(555u32),
            Item::NetheriteBoots => Some(481u32),
            Item::Flint => None,
            Item::Porkchop => None,
            Item::CookedPorkchop => None,
            Item::Painting => None,
            Item::GoldenApple => None,
            Item::EnchantedGoldenApple => None,
            Item::OakSign => None,
            Item::SpruceSign => None,
            Item::BirchSign => None,
            Item::JungleSign => None,
            Item::AcaciaSign => None,
            Item::DarkOakSign => None,
            Item::CrimsonSign => None,
            Item::WarpedSign => None,
            Item::Bucket => None,
            Item::WaterBucket => None,
            Item::LavaBucket => None,
            Item::PowderSnowBucket => None,
            Item::Snowball => None,
            Item::Leather => None,
            Item::MilkBucket => None,
            Item::PufferfishBucket => None,
            Item::SalmonBucket => None,
            Item::CodBucket => None,
            Item::TropicalFishBucket => None,
            Item::AxolotlBucket => None,
            Item::Brick => None,
            Item::ClayBall => None,
            Item::DriedKelpBlock => None,
            Item::Paper => None,
            Item::Book => None,
            Item::SlimeBall => None,
            Item::Egg => None,
            Item::Compass => None,
            Item::Bundle => None,
            Item::FishingRod => Some(64u32),
            Item::Clock => None,
            Item::Spyglass => None,
            Item::GlowstoneDust => None,
            Item::Cod => None,
            Item::Salmon => None,
            Item::TropicalFish => None,
            Item::Pufferfish => None,
            Item::CookedCod => None,
            Item::CookedSalmon => None,
            Item::InkSac => None,
            Item::GlowInkSac => None,
            Item::CocoaBeans => None,
            Item::WhiteDye => None,
            Item::OrangeDye => None,
            Item::MagentaDye => None,
            Item::LightBlueDye => None,
            Item::YellowDye => None,
            Item::LimeDye => None,
            Item::PinkDye => None,
            Item::GrayDye => None,
            Item::LightGrayDye => None,
            Item::CyanDye => None,
            Item::PurpleDye => None,
            Item::BlueDye => None,
            Item::BrownDye => None,
            Item::GreenDye => None,
            Item::RedDye => None,
            Item::BlackDye => None,
            Item::BoneMeal => None,
            Item::Bone => None,
            Item::Sugar => None,
            Item::Cake => None,
            Item::WhiteBed => None,
            Item::OrangeBed => None,
            Item::MagentaBed => None,
            Item::LightBlueBed => None,
            Item::YellowBed => None,
            Item::LimeBed => None,
            Item::PinkBed => None,
            Item::GrayBed => None,
            Item::LightGrayBed => None,
            Item::CyanBed => None,
            Item::PurpleBed => None,
            Item::BlueBed => None,
            Item::BrownBed => None,
            Item::GreenBed => None,
            Item::RedBed => None,
            Item::BlackBed => None,
            Item::Cookie => None,
            Item::FilledMap => None,
            Item::Shears => Some(238u32),
            Item::MelonSlice => None,
            Item::DriedKelp => None,
            Item::PumpkinSeeds => None,
            Item::MelonSeeds => None,
            Item::Beef => None,
            Item::CookedBeef => None,
            Item::Chicken => None,
            Item::CookedChicken => None,
            Item::RottenFlesh => None,
            Item::EnderPearl => None,
            Item::BlazeRod => None,
            Item::GhastTear => None,
            Item::GoldNugget => None,
            Item::NetherWart => None,
            Item::Potion => None,
            Item::GlassBottle => None,
            Item::SpiderEye => None,
            Item::FermentedSpiderEye => None,
            Item::BlazePowder => None,
            Item::MagmaCream => None,
            Item::BrewingStand => None,
            Item::Cauldron => None,
            Item::EnderEye => None,
            Item::GlisteringMelonSlice => None,
            Item::AxolotlSpawnEgg => None,
            Item::BatSpawnEgg => None,
            Item::BeeSpawnEgg => None,
            Item::BlazeSpawnEgg => None,
            Item::CatSpawnEgg => None,
            Item::CaveSpiderSpawnEgg => None,
            Item::ChickenSpawnEgg => None,
            Item::CodSpawnEgg => None,
            Item::CowSpawnEgg => None,
            Item::CreeperSpawnEgg => None,
            Item::DolphinSpawnEgg => None,
            Item::DonkeySpawnEgg => None,
            Item::DrownedSpawnEgg => None,
            Item::ElderGuardianSpawnEgg => None,
            Item::EndermanSpawnEgg => None,
            Item::EndermiteSpawnEgg => None,
            Item::EvokerSpawnEgg => None,
            Item::FoxSpawnEgg => None,
            Item::GhastSpawnEgg => None,
            Item::GlowSquidSpawnEgg => None,
            Item::GoatSpawnEgg => None,
            Item::GuardianSpawnEgg => None,
            Item::HoglinSpawnEgg => None,
            Item::HorseSpawnEgg => None,
            Item::HuskSpawnEgg => None,
            Item::LlamaSpawnEgg => None,
            Item::MagmaCubeSpawnEgg => None,
            Item::MooshroomSpawnEgg => None,
            Item::MuleSpawnEgg => None,
            Item::OcelotSpawnEgg => None,
            Item::PandaSpawnEgg => None,
            Item::ParrotSpawnEgg => None,
            Item::PhantomSpawnEgg => None,
            Item::PigSpawnEgg => None,
            Item::PiglinSpawnEgg => None,
            Item::PiglinBruteSpawnEgg => None,
            Item::PillagerSpawnEgg => None,
            Item::PolarBearSpawnEgg => None,
            Item::PufferfishSpawnEgg => None,
            Item::RabbitSpawnEgg => None,
            Item::RavagerSpawnEgg => None,
            Item::SalmonSpawnEgg => None,
            Item::SheepSpawnEgg => None,
            Item::ShulkerSpawnEgg => None,
            Item::SilverfishSpawnEgg => None,
            Item::SkeletonSpawnEgg => None,
            Item::SkeletonHorseSpawnEgg => None,
            Item::SlimeSpawnEgg => None,
            Item::SpiderSpawnEgg => None,
            Item::SquidSpawnEgg => None,
            Item::StraySpawnEgg => None,
            Item::StriderSpawnEgg => None,
            Item::TraderLlamaSpawnEgg => None,
            Item::TropicalFishSpawnEgg => None,
            Item::TurtleSpawnEgg => None,
            Item::VexSpawnEgg => None,
            Item::VillagerSpawnEgg => None,
            Item::VindicatorSpawnEgg => None,
            Item::WanderingTraderSpawnEgg => None,
            Item::WitchSpawnEgg => None,
            Item::WitherSkeletonSpawnEgg => None,
            Item::WolfSpawnEgg => None,
            Item::ZoglinSpawnEgg => None,
            Item::ZombieSpawnEgg => None,
            Item::ZombieHorseSpawnEgg => None,
            Item::ZombieVillagerSpawnEgg => None,
            Item::ZombifiedPiglinSpawnEgg => None,
            Item::ExperienceBottle => None,
            Item::FireCharge => None,
            Item::WritableBook => None,
            Item::WrittenBook => None,
            Item::ItemFrame => None,
            Item::GlowItemFrame => None,
            Item::FlowerPot => None,
            Item::Carrot => None,
            Item::Potato => None,
            Item::BakedPotato => None,
            Item::PoisonousPotato => None,
            Item::Map => None,
            Item::GoldenCarrot => None,
            Item::SkeletonSkull => None,
            Item::WitherSkeletonSkull => None,
            Item::PlayerHead => None,
            Item::ZombieHead => None,
            Item::CreeperHead => None,
            Item::DragonHead => None,
            Item::NetherStar => None,
            Item::PumpkinPie => None,
            Item::FireworkRocket => None,
            Item::FireworkStar => None,
            Item::EnchantedBook => None,
            Item::NetherBrick => None,
            Item::PrismarineShard => None,
            Item::PrismarineCrystals => None,
            Item::Rabbit => None,
            Item::CookedRabbit => None,
            Item::RabbitStew => None,
            Item::RabbitFoot => None,
            Item::RabbitHide => None,
            Item::ArmorStand => None,
            Item::IronHorseArmor => None,
            Item::GoldenHorseArmor => None,
            Item::DiamondHorseArmor => None,
            Item::LeatherHorseArmor => None,
            Item::Lead => None,
            Item::NameTag => None,
            Item::CommandBlockMinecart => None,
            Item::Mutton => None,
            Item::CookedMutton => None,
            Item::WhiteBanner => None,
            Item::OrangeBanner => None,
            Item::MagentaBanner => None,
            Item::LightBlueBanner => None,
            Item::YellowBanner => None,
            Item::LimeBanner => None,
            Item::PinkBanner => None,
            Item::GrayBanner => None,
            Item::LightGrayBanner => None,
            Item::CyanBanner => None,
            Item::PurpleBanner => None,
            Item::BlueBanner => None,
            Item::BrownBanner => None,
            Item::GreenBanner => None,
            Item::RedBanner => None,
            Item::BlackBanner => None,
            Item::EndCrystal => None,
            Item::ChorusFruit => None,
            Item::PoppedChorusFruit => None,
            Item::Beetroot => None,
            Item::BeetrootSeeds => None,
            Item::BeetrootSoup => None,
            Item::DragonBreath => None,
            Item::SplashPotion => None,
            Item::SpectralArrow => None,
            Item::TippedArrow => None,
            Item::LingeringPotion => None,
            Item::Shield => Some(336u32),
            Item::TotemOfUndying => None,
            Item::ShulkerShell => None,
            Item::IronNugget => None,
            Item::KnowledgeBook => None,
            Item::DebugStick => None,
            Item::MusicDisc13 => None,
            Item::MusicDiscCat => None,
            Item::MusicDiscBlocks => None,
            Item::MusicDiscChirp => None,
            Item::MusicDiscFar => None,
            Item::MusicDiscMall => None,
            Item::MusicDiscMellohi => None,
            Item::MusicDiscStal => None,
            Item::MusicDiscStrad => None,
            Item::MusicDiscWard => None,
            Item::MusicDisc11 => None,
            Item::MusicDiscWait => None,
            Item::MusicDiscOtherside => None,
            Item::MusicDiscPigstep => None,
            Item::Trident => Some(250u32),
            Item::PhantomMembrane => None,
            Item::NautilusShell => None,
            Item::HeartOfTheSea => None,
            Item::Crossbow => Some(326u32),
            Item::SuspiciousStew => None,
            Item::Loom => None,
            Item::FlowerBannerPattern => None,
            Item::CreeperBannerPattern => None,
            Item::SkullBannerPattern => None,
            Item::MojangBannerPattern => None,
            Item::GlobeBannerPattern => None,
            Item::PiglinBannerPattern => None,
            Item::Composter => None,
            Item::Barrel => None,
            Item::Smoker => None,
            Item::BlastFurnace => None,
            Item::CartographyTable => None,
            Item::FletchingTable => None,
            Item::Grindstone => None,
            Item::SmithingTable => None,
            Item::Stonecutter => None,
            Item::Bell => None,
            Item::Lantern => None,
            Item::SoulLantern => None,
            Item::SweetBerries => None,
            Item::GlowBerries => None,
            Item::Campfire => None,
            Item::SoulCampfire => None,
            Item::Shroomlight => None,
            Item::Honeycomb => None,
            Item::BeeNest => None,
            Item::Beehive => None,
            Item::HoneyBottle => None,
            Item::HoneycombBlock => None,
            Item::Lodestone => None,
            Item::CryingObsidian => None,
            Item::Blackstone => None,
            Item::BlackstoneSlab => None,
            Item::BlackstoneStairs => None,
            Item::GildedBlackstone => None,
            Item::PolishedBlackstone => None,
            Item::PolishedBlackstoneSlab => None,
            Item::PolishedBlackstoneStairs => None,
            Item::ChiseledPolishedBlackstone => None,
            Item::PolishedBlackstoneBricks => None,
            Item::PolishedBlackstoneBrickSlab => None,
            Item::PolishedBlackstoneBrickStairs => None,
            Item::CrackedPolishedBlackstoneBricks => None,
            Item::RespawnAnchor => None,
            Item::Candle => None,
            Item::WhiteCandle => None,
            Item::OrangeCandle => None,
            Item::MagentaCandle => None,
            Item::LightBlueCandle => None,
            Item::YellowCandle => None,
            Item::LimeCandle => None,
            Item::PinkCandle => None,
            Item::GrayCandle => None,
            Item::LightGrayCandle => None,
            Item::CyanCandle => None,
            Item::PurpleCandle => None,
            Item::BlueCandle => None,
            Item::BrownCandle => None,
            Item::GreenCandle => None,
            Item::RedCandle => None,
            Item::BlackCandle => None,
            Item::SmallAmethystBud => None,
            Item::MediumAmethystBud => None,
            Item::LargeAmethystBud => None,
            Item::AmethystCluster => None,
            Item::PointedDripstone => None,
        }
    }
}
impl Item {
    #[doc = "Returns the `fixed_with` property of this `Item`."]
    #[inline]
    pub fn fixed_with(&self) -> Vec<&'static str> {
        match self {
            Item::Stone => {
                vec![]
            }
            Item::Granite => {
                vec![]
            }
            Item::PolishedGranite => {
                vec![]
            }
            Item::Diorite => {
                vec![]
            }
            Item::PolishedDiorite => {
                vec![]
            }
            Item::Andesite => {
                vec![]
            }
            Item::PolishedAndesite => {
                vec![]
            }
            Item::Deepslate => {
                vec![]
            }
            Item::CobbledDeepslate => {
                vec![]
            }
            Item::PolishedDeepslate => {
                vec![]
            }
            Item::Calcite => {
                vec![]
            }
            Item::Tuff => {
                vec![]
            }
            Item::DripstoneBlock => {
                vec![]
            }
            Item::GrassBlock => {
                vec![]
            }
            Item::Dirt => {
                vec![]
            }
            Item::CoarseDirt => {
                vec![]
            }
            Item::Podzol => {
                vec![]
            }
            Item::RootedDirt => {
                vec![]
            }
            Item::CrimsonNylium => {
                vec![]
            }
            Item::WarpedNylium => {
                vec![]
            }
            Item::Cobblestone => {
                vec![]
            }
            Item::OakPlanks => {
                vec![]
            }
            Item::SprucePlanks => {
                vec![]
            }
            Item::BirchPlanks => {
                vec![]
            }
            Item::JunglePlanks => {
                vec![]
            }
            Item::AcaciaPlanks => {
                vec![]
            }
            Item::DarkOakPlanks => {
                vec![]
            }
            Item::CrimsonPlanks => {
                vec![]
            }
            Item::WarpedPlanks => {
                vec![]
            }
            Item::OakSapling => {
                vec![]
            }
            Item::SpruceSapling => {
                vec![]
            }
            Item::BirchSapling => {
                vec![]
            }
            Item::JungleSapling => {
                vec![]
            }
            Item::AcaciaSapling => {
                vec![]
            }
            Item::DarkOakSapling => {
                vec![]
            }
            Item::Bedrock => {
                vec![]
            }
            Item::Sand => {
                vec![]
            }
            Item::RedSand => {
                vec![]
            }
            Item::Gravel => {
                vec![]
            }
            Item::CoalOre => {
                vec![]
            }
            Item::DeepslateCoalOre => {
                vec![]
            }
            Item::IronOre => {
                vec![]
            }
            Item::DeepslateIronOre => {
                vec![]
            }
            Item::CopperOre => {
                vec![]
            }
            Item::DeepslateCopperOre => {
                vec![]
            }
            Item::GoldOre => {
                vec![]
            }
            Item::DeepslateGoldOre => {
                vec![]
            }
            Item::RedstoneOre => {
                vec![]
            }
            Item::DeepslateRedstoneOre => {
                vec![]
            }
            Item::EmeraldOre => {
                vec![]
            }
            Item::DeepslateEmeraldOre => {
                vec![]
            }
            Item::LapisOre => {
                vec![]
            }
            Item::DeepslateLapisOre => {
                vec![]
            }
            Item::DiamondOre => {
                vec![]
            }
            Item::DeepslateDiamondOre => {
                vec![]
            }
            Item::NetherGoldOre => {
                vec![]
            }
            Item::NetherQuartzOre => {
                vec![]
            }
            Item::AncientDebris => {
                vec![]
            }
            Item::CoalBlock => {
                vec![]
            }
            Item::RawIronBlock => {
                vec![]
            }
            Item::RawCopperBlock => {
                vec![]
            }
            Item::RawGoldBlock => {
                vec![]
            }
            Item::AmethystBlock => {
                vec![]
            }
            Item::BuddingAmethyst => {
                vec![]
            }
            Item::IronBlock => {
                vec![]
            }
            Item::CopperBlock => {
                vec![]
            }
            Item::GoldBlock => {
                vec![]
            }
            Item::DiamondBlock => {
                vec![]
            }
            Item::NetheriteBlock => {
                vec![]
            }
            Item::ExposedCopper => {
                vec![]
            }
            Item::WeatheredCopper => {
                vec![]
            }
            Item::OxidizedCopper => {
                vec![]
            }
            Item::CutCopper => {
                vec![]
            }
            Item::ExposedCutCopper => {
                vec![]
            }
            Item::WeatheredCutCopper => {
                vec![]
            }
            Item::OxidizedCutCopper => {
                vec![]
            }
            Item::CutCopperStairs => {
                vec![]
            }
            Item::ExposedCutCopperStairs => {
                vec![]
            }
            Item::WeatheredCutCopperStairs => {
                vec![]
            }
            Item::OxidizedCutCopperStairs => {
                vec![]
            }
            Item::CutCopperSlab => {
                vec![]
            }
            Item::ExposedCutCopperSlab => {
                vec![]
            }
            Item::WeatheredCutCopperSlab => {
                vec![]
            }
            Item::OxidizedCutCopperSlab => {
                vec![]
            }
            Item::WaxedCopperBlock => {
                vec![]
            }
            Item::WaxedExposedCopper => {
                vec![]
            }
            Item::WaxedWeatheredCopper => {
                vec![]
            }
            Item::WaxedOxidizedCopper => {
                vec![]
            }
            Item::WaxedCutCopper => {
                vec![]
            }
            Item::WaxedExposedCutCopper => {
                vec![]
            }
            Item::WaxedWeatheredCutCopper => {
                vec![]
            }
            Item::WaxedOxidizedCutCopper => {
                vec![]
            }
            Item::WaxedCutCopperStairs => {
                vec![]
            }
            Item::WaxedExposedCutCopperStairs => {
                vec![]
            }
            Item::WaxedWeatheredCutCopperStairs => {
                vec![]
            }
            Item::WaxedOxidizedCutCopperStairs => {
                vec![]
            }
            Item::WaxedCutCopperSlab => {
                vec![]
            }
            Item::WaxedExposedCutCopperSlab => {
                vec![]
            }
            Item::WaxedWeatheredCutCopperSlab => {
                vec![]
            }
            Item::WaxedOxidizedCutCopperSlab => {
                vec![]
            }
            Item::OakLog => {
                vec![]
            }
            Item::SpruceLog => {
                vec![]
            }
            Item::BirchLog => {
                vec![]
            }
            Item::JungleLog => {
                vec![]
            }
            Item::AcaciaLog => {
                vec![]
            }
            Item::DarkOakLog => {
                vec![]
            }
            Item::CrimsonStem => {
                vec![]
            }
            Item::WarpedStem => {
                vec![]
            }
            Item::StrippedOakLog => {
                vec![]
            }
            Item::StrippedSpruceLog => {
                vec![]
            }
            Item::StrippedBirchLog => {
                vec![]
            }
            Item::StrippedJungleLog => {
                vec![]
            }
            Item::StrippedAcaciaLog => {
                vec![]
            }
            Item::StrippedDarkOakLog => {
                vec![]
            }
            Item::StrippedCrimsonStem => {
                vec![]
            }
            Item::StrippedWarpedStem => {
                vec![]
            }
            Item::StrippedOakWood => {
                vec![]
            }
            Item::StrippedSpruceWood => {
                vec![]
            }
            Item::StrippedBirchWood => {
                vec![]
            }
            Item::StrippedJungleWood => {
                vec![]
            }
            Item::StrippedAcaciaWood => {
                vec![]
            }
            Item::StrippedDarkOakWood => {
                vec![]
            }
            Item::StrippedCrimsonHyphae => {
                vec![]
            }
            Item::StrippedWarpedHyphae => {
                vec![]
            }
            Item::OakWood => {
                vec![]
            }
            Item::SpruceWood => {
                vec![]
            }
            Item::BirchWood => {
                vec![]
            }
            Item::JungleWood => {
                vec![]
            }
            Item::AcaciaWood => {
                vec![]
            }
            Item::DarkOakWood => {
                vec![]
            }
            Item::CrimsonHyphae => {
                vec![]
            }
            Item::WarpedHyphae => {
                vec![]
            }
            Item::OakLeaves => {
                vec![]
            }
            Item::SpruceLeaves => {
                vec![]
            }
            Item::BirchLeaves => {
                vec![]
            }
            Item::JungleLeaves => {
                vec![]
            }
            Item::AcaciaLeaves => {
                vec![]
            }
            Item::DarkOakLeaves => {
                vec![]
            }
            Item::AzaleaLeaves => {
                vec![]
            }
            Item::FloweringAzaleaLeaves => {
                vec![]
            }
            Item::Sponge => {
                vec![]
            }
            Item::WetSponge => {
                vec![]
            }
            Item::Glass => {
                vec![]
            }
            Item::TintedGlass => {
                vec![]
            }
            Item::LapisBlock => {
                vec![]
            }
            Item::Sandstone => {
                vec![]
            }
            Item::ChiseledSandstone => {
                vec![]
            }
            Item::CutSandstone => {
                vec![]
            }
            Item::Cobweb => {
                vec![]
            }
            Item::Grass => {
                vec![]
            }
            Item::Fern => {
                vec![]
            }
            Item::Azalea => {
                vec![]
            }
            Item::FloweringAzalea => {
                vec![]
            }
            Item::DeadBush => {
                vec![]
            }
            Item::Seagrass => {
                vec![]
            }
            Item::SeaPickle => {
                vec![]
            }
            Item::WhiteWool => {
                vec![]
            }
            Item::OrangeWool => {
                vec![]
            }
            Item::MagentaWool => {
                vec![]
            }
            Item::LightBlueWool => {
                vec![]
            }
            Item::YellowWool => {
                vec![]
            }
            Item::LimeWool => {
                vec![]
            }
            Item::PinkWool => {
                vec![]
            }
            Item::GrayWool => {
                vec![]
            }
            Item::LightGrayWool => {
                vec![]
            }
            Item::CyanWool => {
                vec![]
            }
            Item::PurpleWool => {
                vec![]
            }
            Item::BlueWool => {
                vec![]
            }
            Item::BrownWool => {
                vec![]
            }
            Item::GreenWool => {
                vec![]
            }
            Item::RedWool => {
                vec![]
            }
            Item::BlackWool => {
                vec![]
            }
            Item::Dandelion => {
                vec![]
            }
            Item::Poppy => {
                vec![]
            }
            Item::BlueOrchid => {
                vec![]
            }
            Item::Allium => {
                vec![]
            }
            Item::AzureBluet => {
                vec![]
            }
            Item::RedTulip => {
                vec![]
            }
            Item::OrangeTulip => {
                vec![]
            }
            Item::WhiteTulip => {
                vec![]
            }
            Item::PinkTulip => {
                vec![]
            }
            Item::OxeyeDaisy => {
                vec![]
            }
            Item::Cornflower => {
                vec![]
            }
            Item::LilyOfTheValley => {
                vec![]
            }
            Item::WitherRose => {
                vec![]
            }
            Item::SporeBlossom => {
                vec![]
            }
            Item::BrownMushroom => {
                vec![]
            }
            Item::RedMushroom => {
                vec![]
            }
            Item::CrimsonFungus => {
                vec![]
            }
            Item::WarpedFungus => {
                vec![]
            }
            Item::CrimsonRoots => {
                vec![]
            }
            Item::WarpedRoots => {
                vec![]
            }
            Item::NetherSprouts => {
                vec![]
            }
            Item::WeepingVines => {
                vec![]
            }
            Item::TwistingVines => {
                vec![]
            }
            Item::SugarCane => {
                vec![]
            }
            Item::Kelp => {
                vec![]
            }
            Item::MossCarpet => {
                vec![]
            }
            Item::MossBlock => {
                vec![]
            }
            Item::HangingRoots => {
                vec![]
            }
            Item::BigDripleaf => {
                vec![]
            }
            Item::SmallDripleaf => {
                vec![]
            }
            Item::Bamboo => {
                vec![]
            }
            Item::OakSlab => {
                vec![]
            }
            Item::SpruceSlab => {
                vec![]
            }
            Item::BirchSlab => {
                vec![]
            }
            Item::JungleSlab => {
                vec![]
            }
            Item::AcaciaSlab => {
                vec![]
            }
            Item::DarkOakSlab => {
                vec![]
            }
            Item::CrimsonSlab => {
                vec![]
            }
            Item::WarpedSlab => {
                vec![]
            }
            Item::StoneSlab => {
                vec![]
            }
            Item::SmoothStoneSlab => {
                vec![]
            }
            Item::SandstoneSlab => {
                vec![]
            }
            Item::CutSandstoneSlab => {
                vec![]
            }
            Item::PetrifiedOakSlab => {
                vec![]
            }
            Item::CobblestoneSlab => {
                vec![]
            }
            Item::BrickSlab => {
                vec![]
            }
            Item::StoneBrickSlab => {
                vec![]
            }
            Item::NetherBrickSlab => {
                vec![]
            }
            Item::QuartzSlab => {
                vec![]
            }
            Item::RedSandstoneSlab => {
                vec![]
            }
            Item::CutRedSandstoneSlab => {
                vec![]
            }
            Item::PurpurSlab => {
                vec![]
            }
            Item::PrismarineSlab => {
                vec![]
            }
            Item::PrismarineBrickSlab => {
                vec![]
            }
            Item::DarkPrismarineSlab => {
                vec![]
            }
            Item::SmoothQuartz => {
                vec![]
            }
            Item::SmoothRedSandstone => {
                vec![]
            }
            Item::SmoothSandstone => {
                vec![]
            }
            Item::SmoothStone => {
                vec![]
            }
            Item::Bricks => {
                vec![]
            }
            Item::Bookshelf => {
                vec![]
            }
            Item::MossyCobblestone => {
                vec![]
            }
            Item::Obsidian => {
                vec![]
            }
            Item::Torch => {
                vec![]
            }
            Item::EndRod => {
                vec![]
            }
            Item::ChorusPlant => {
                vec![]
            }
            Item::ChorusFlower => {
                vec![]
            }
            Item::PurpurBlock => {
                vec![]
            }
            Item::PurpurPillar => {
                vec![]
            }
            Item::PurpurStairs => {
                vec![]
            }
            Item::Spawner => {
                vec![]
            }
            Item::OakStairs => {
                vec![]
            }
            Item::Chest => {
                vec![]
            }
            Item::CraftingTable => {
                vec![]
            }
            Item::Farmland => {
                vec![]
            }
            Item::Furnace => {
                vec![]
            }
            Item::Ladder => {
                vec![]
            }
            Item::CobblestoneStairs => {
                vec![]
            }
            Item::Snow => {
                vec![]
            }
            Item::Ice => {
                vec![]
            }
            Item::SnowBlock => {
                vec![]
            }
            Item::Cactus => {
                vec![]
            }
            Item::Clay => {
                vec![]
            }
            Item::Jukebox => {
                vec![]
            }
            Item::OakFence => {
                vec![]
            }
            Item::SpruceFence => {
                vec![]
            }
            Item::BirchFence => {
                vec![]
            }
            Item::JungleFence => {
                vec![]
            }
            Item::AcaciaFence => {
                vec![]
            }
            Item::DarkOakFence => {
                vec![]
            }
            Item::CrimsonFence => {
                vec![]
            }
            Item::WarpedFence => {
                vec![]
            }
            Item::Pumpkin => {
                vec![]
            }
            Item::CarvedPumpkin => {
                vec![]
            }
            Item::JackOLantern => {
                vec![]
            }
            Item::Netherrack => {
                vec![]
            }
            Item::SoulSand => {
                vec![]
            }
            Item::SoulSoil => {
                vec![]
            }
            Item::Basalt => {
                vec![]
            }
            Item::PolishedBasalt => {
                vec![]
            }
            Item::SmoothBasalt => {
                vec![]
            }
            Item::SoulTorch => {
                vec![]
            }
            Item::Glowstone => {
                vec![]
            }
            Item::InfestedStone => {
                vec![]
            }
            Item::InfestedCobblestone => {
                vec![]
            }
            Item::InfestedStoneBricks => {
                vec![]
            }
            Item::InfestedMossyStoneBricks => {
                vec![]
            }
            Item::InfestedCrackedStoneBricks => {
                vec![]
            }
            Item::InfestedChiseledStoneBricks => {
                vec![]
            }
            Item::InfestedDeepslate => {
                vec![]
            }
            Item::StoneBricks => {
                vec![]
            }
            Item::MossyStoneBricks => {
                vec![]
            }
            Item::CrackedStoneBricks => {
                vec![]
            }
            Item::ChiseledStoneBricks => {
                vec![]
            }
            Item::DeepslateBricks => {
                vec![]
            }
            Item::CrackedDeepslateBricks => {
                vec![]
            }
            Item::DeepslateTiles => {
                vec![]
            }
            Item::CrackedDeepslateTiles => {
                vec![]
            }
            Item::ChiseledDeepslate => {
                vec![]
            }
            Item::BrownMushroomBlock => {
                vec![]
            }
            Item::RedMushroomBlock => {
                vec![]
            }
            Item::MushroomStem => {
                vec![]
            }
            Item::IronBars => {
                vec![]
            }
            Item::Chain => {
                vec![]
            }
            Item::GlassPane => {
                vec![]
            }
            Item::Melon => {
                vec![]
            }
            Item::Vine => {
                vec![]
            }
            Item::GlowLichen => {
                vec![]
            }
            Item::BrickStairs => {
                vec![]
            }
            Item::StoneBrickStairs => {
                vec![]
            }
            Item::Mycelium => {
                vec![]
            }
            Item::LilyPad => {
                vec![]
            }
            Item::NetherBricks => {
                vec![]
            }
            Item::CrackedNetherBricks => {
                vec![]
            }
            Item::ChiseledNetherBricks => {
                vec![]
            }
            Item::NetherBrickFence => {
                vec![]
            }
            Item::NetherBrickStairs => {
                vec![]
            }
            Item::EnchantingTable => {
                vec![]
            }
            Item::EndPortalFrame => {
                vec![]
            }
            Item::EndStone => {
                vec![]
            }
            Item::EndStoneBricks => {
                vec![]
            }
            Item::DragonEgg => {
                vec![]
            }
            Item::SandstoneStairs => {
                vec![]
            }
            Item::EnderChest => {
                vec![]
            }
            Item::EmeraldBlock => {
                vec![]
            }
            Item::SpruceStairs => {
                vec![]
            }
            Item::BirchStairs => {
                vec![]
            }
            Item::JungleStairs => {
                vec![]
            }
            Item::CrimsonStairs => {
                vec![]
            }
            Item::WarpedStairs => {
                vec![]
            }
            Item::CommandBlock => {
                vec![]
            }
            Item::Beacon => {
                vec![]
            }
            Item::CobblestoneWall => {
                vec![]
            }
            Item::MossyCobblestoneWall => {
                vec![]
            }
            Item::BrickWall => {
                vec![]
            }
            Item::PrismarineWall => {
                vec![]
            }
            Item::RedSandstoneWall => {
                vec![]
            }
            Item::MossyStoneBrickWall => {
                vec![]
            }
            Item::GraniteWall => {
                vec![]
            }
            Item::StoneBrickWall => {
                vec![]
            }
            Item::NetherBrickWall => {
                vec![]
            }
            Item::AndesiteWall => {
                vec![]
            }
            Item::RedNetherBrickWall => {
                vec![]
            }
            Item::SandstoneWall => {
                vec![]
            }
            Item::EndStoneBrickWall => {
                vec![]
            }
            Item::DioriteWall => {
                vec![]
            }
            Item::BlackstoneWall => {
                vec![]
            }
            Item::PolishedBlackstoneWall => {
                vec![]
            }
            Item::PolishedBlackstoneBrickWall => {
                vec![]
            }
            Item::CobbledDeepslateWall => {
                vec![]
            }
            Item::PolishedDeepslateWall => {
                vec![]
            }
            Item::DeepslateBrickWall => {
                vec![]
            }
            Item::DeepslateTileWall => {
                vec![]
            }
            Item::Anvil => {
                vec![]
            }
            Item::ChippedAnvil => {
                vec![]
            }
            Item::DamagedAnvil => {
                vec![]
            }
            Item::ChiseledQuartzBlock => {
                vec![]
            }
            Item::QuartzBlock => {
                vec![]
            }
            Item::QuartzBricks => {
                vec![]
            }
            Item::QuartzPillar => {
                vec![]
            }
            Item::QuartzStairs => {
                vec![]
            }
            Item::WhiteTerracotta => {
                vec![]
            }
            Item::OrangeTerracotta => {
                vec![]
            }
            Item::MagentaTerracotta => {
                vec![]
            }
            Item::LightBlueTerracotta => {
                vec![]
            }
            Item::YellowTerracotta => {
                vec![]
            }
            Item::LimeTerracotta => {
                vec![]
            }
            Item::PinkTerracotta => {
                vec![]
            }
            Item::GrayTerracotta => {
                vec![]
            }
            Item::LightGrayTerracotta => {
                vec![]
            }
            Item::CyanTerracotta => {
                vec![]
            }
            Item::PurpleTerracotta => {
                vec![]
            }
            Item::BlueTerracotta => {
                vec![]
            }
            Item::BrownTerracotta => {
                vec![]
            }
            Item::GreenTerracotta => {
                vec![]
            }
            Item::RedTerracotta => {
                vec![]
            }
            Item::BlackTerracotta => {
                vec![]
            }
            Item::Barrier => {
                vec![]
            }
            Item::Light => {
                vec![]
            }
            Item::HayBlock => {
                vec![]
            }
            Item::WhiteCarpet => {
                vec![]
            }
            Item::OrangeCarpet => {
                vec![]
            }
            Item::MagentaCarpet => {
                vec![]
            }
            Item::LightBlueCarpet => {
                vec![]
            }
            Item::YellowCarpet => {
                vec![]
            }
            Item::LimeCarpet => {
                vec![]
            }
            Item::PinkCarpet => {
                vec![]
            }
            Item::GrayCarpet => {
                vec![]
            }
            Item::LightGrayCarpet => {
                vec![]
            }
            Item::CyanCarpet => {
                vec![]
            }
            Item::PurpleCarpet => {
                vec![]
            }
            Item::BlueCarpet => {
                vec![]
            }
            Item::BrownCarpet => {
                vec![]
            }
            Item::GreenCarpet => {
                vec![]
            }
            Item::RedCarpet => {
                vec![]
            }
            Item::BlackCarpet => {
                vec![]
            }
            Item::Terracotta => {
                vec![]
            }
            Item::PackedIce => {
                vec![]
            }
            Item::AcaciaStairs => {
                vec![]
            }
            Item::DarkOakStairs => {
                vec![]
            }
            Item::DirtPath => {
                vec![]
            }
            Item::Sunflower => {
                vec![]
            }
            Item::Lilac => {
                vec![]
            }
            Item::RoseBush => {
                vec![]
            }
            Item::Peony => {
                vec![]
            }
            Item::TallGrass => {
                vec![]
            }
            Item::LargeFern => {
                vec![]
            }
            Item::WhiteStainedGlass => {
                vec![]
            }
            Item::OrangeStainedGlass => {
                vec![]
            }
            Item::MagentaStainedGlass => {
                vec![]
            }
            Item::LightBlueStainedGlass => {
                vec![]
            }
            Item::YellowStainedGlass => {
                vec![]
            }
            Item::LimeStainedGlass => {
                vec![]
            }
            Item::PinkStainedGlass => {
                vec![]
            }
            Item::GrayStainedGlass => {
                vec![]
            }
            Item::LightGrayStainedGlass => {
                vec![]
            }
            Item::CyanStainedGlass => {
                vec![]
            }
            Item::PurpleStainedGlass => {
                vec![]
            }
            Item::BlueStainedGlass => {
                vec![]
            }
            Item::BrownStainedGlass => {
                vec![]
            }
            Item::GreenStainedGlass => {
                vec![]
            }
            Item::RedStainedGlass => {
                vec![]
            }
            Item::BlackStainedGlass => {
                vec![]
            }
            Item::WhiteStainedGlassPane => {
                vec![]
            }
            Item::OrangeStainedGlassPane => {
                vec![]
            }
            Item::MagentaStainedGlassPane => {
                vec![]
            }
            Item::LightBlueStainedGlassPane => {
                vec![]
            }
            Item::YellowStainedGlassPane => {
                vec![]
            }
            Item::LimeStainedGlassPane => {
                vec![]
            }
            Item::PinkStainedGlassPane => {
                vec![]
            }
            Item::GrayStainedGlassPane => {
                vec![]
            }
            Item::LightGrayStainedGlassPane => {
                vec![]
            }
            Item::CyanStainedGlassPane => {
                vec![]
            }
            Item::PurpleStainedGlassPane => {
                vec![]
            }
            Item::BlueStainedGlassPane => {
                vec![]
            }
            Item::BrownStainedGlassPane => {
                vec![]
            }
            Item::GreenStainedGlassPane => {
                vec![]
            }
            Item::RedStainedGlassPane => {
                vec![]
            }
            Item::BlackStainedGlassPane => {
                vec![]
            }
            Item::Prismarine => {
                vec![]
            }
            Item::PrismarineBricks => {
                vec![]
            }
            Item::DarkPrismarine => {
                vec![]
            }
            Item::PrismarineStairs => {
                vec![]
            }
            Item::PrismarineBrickStairs => {
                vec![]
            }
            Item::DarkPrismarineStairs => {
                vec![]
            }
            Item::SeaLantern => {
                vec![]
            }
            Item::RedSandstone => {
                vec![]
            }
            Item::ChiseledRedSandstone => {
                vec![]
            }
            Item::CutRedSandstone => {
                vec![]
            }
            Item::RedSandstoneStairs => {
                vec![]
            }
            Item::RepeatingCommandBlock => {
                vec![]
            }
            Item::ChainCommandBlock => {
                vec![]
            }
            Item::MagmaBlock => {
                vec![]
            }
            Item::NetherWartBlock => {
                vec![]
            }
            Item::WarpedWartBlock => {
                vec![]
            }
            Item::RedNetherBricks => {
                vec![]
            }
            Item::BoneBlock => {
                vec![]
            }
            Item::StructureVoid => {
                vec![]
            }
            Item::ShulkerBox => {
                vec![]
            }
            Item::WhiteShulkerBox => {
                vec![]
            }
            Item::OrangeShulkerBox => {
                vec![]
            }
            Item::MagentaShulkerBox => {
                vec![]
            }
            Item::LightBlueShulkerBox => {
                vec![]
            }
            Item::YellowShulkerBox => {
                vec![]
            }
            Item::LimeShulkerBox => {
                vec![]
            }
            Item::PinkShulkerBox => {
                vec![]
            }
            Item::GrayShulkerBox => {
                vec![]
            }
            Item::LightGrayShulkerBox => {
                vec![]
            }
            Item::CyanShulkerBox => {
                vec![]
            }
            Item::PurpleShulkerBox => {
                vec![]
            }
            Item::BlueShulkerBox => {
                vec![]
            }
            Item::BrownShulkerBox => {
                vec![]
            }
            Item::GreenShulkerBox => {
                vec![]
            }
            Item::RedShulkerBox => {
                vec![]
            }
            Item::BlackShulkerBox => {
                vec![]
            }
            Item::WhiteGlazedTerracotta => {
                vec![]
            }
            Item::OrangeGlazedTerracotta => {
                vec![]
            }
            Item::MagentaGlazedTerracotta => {
                vec![]
            }
            Item::LightBlueGlazedTerracotta => {
                vec![]
            }
            Item::YellowGlazedTerracotta => {
                vec![]
            }
            Item::LimeGlazedTerracotta => {
                vec![]
            }
            Item::PinkGlazedTerracotta => {
                vec![]
            }
            Item::GrayGlazedTerracotta => {
                vec![]
            }
            Item::LightGrayGlazedTerracotta => {
                vec![]
            }
            Item::CyanGlazedTerracotta => {
                vec![]
            }
            Item::PurpleGlazedTerracotta => {
                vec![]
            }
            Item::BlueGlazedTerracotta => {
                vec![]
            }
            Item::BrownGlazedTerracotta => {
                vec![]
            }
            Item::GreenGlazedTerracotta => {
                vec![]
            }
            Item::RedGlazedTerracotta => {
                vec![]
            }
            Item::BlackGlazedTerracotta => {
                vec![]
            }
            Item::WhiteConcrete => {
                vec![]
            }
            Item::OrangeConcrete => {
                vec![]
            }
            Item::MagentaConcrete => {
                vec![]
            }
            Item::LightBlueConcrete => {
                vec![]
            }
            Item::YellowConcrete => {
                vec![]
            }
            Item::LimeConcrete => {
                vec![]
            }
            Item::PinkConcrete => {
                vec![]
            }
            Item::GrayConcrete => {
                vec![]
            }
            Item::LightGrayConcrete => {
                vec![]
            }
            Item::CyanConcrete => {
                vec![]
            }
            Item::PurpleConcrete => {
                vec![]
            }
            Item::BlueConcrete => {
                vec![]
            }
            Item::BrownConcrete => {
                vec![]
            }
            Item::GreenConcrete => {
                vec![]
            }
            Item::RedConcrete => {
                vec![]
            }
            Item::BlackConcrete => {
                vec![]
            }
            Item::WhiteConcretePowder => {
                vec![]
            }
            Item::OrangeConcretePowder => {
                vec![]
            }
            Item::MagentaConcretePowder => {
                vec![]
            }
            Item::LightBlueConcretePowder => {
                vec![]
            }
            Item::YellowConcretePowder => {
                vec![]
            }
            Item::LimeConcretePowder => {
                vec![]
            }
            Item::PinkConcretePowder => {
                vec![]
            }
            Item::GrayConcretePowder => {
                vec![]
            }
            Item::LightGrayConcretePowder => {
                vec![]
            }
            Item::CyanConcretePowder => {
                vec![]
            }
            Item::PurpleConcretePowder => {
                vec![]
            }
            Item::BlueConcretePowder => {
                vec![]
            }
            Item::BrownConcretePowder => {
                vec![]
            }
            Item::GreenConcretePowder => {
                vec![]
            }
            Item::RedConcretePowder => {
                vec![]
            }
            Item::BlackConcretePowder => {
                vec![]
            }
            Item::TurtleEgg => {
                vec![]
            }
            Item::DeadTubeCoralBlock => {
                vec![]
            }
            Item::DeadBrainCoralBlock => {
                vec![]
            }
            Item::DeadBubbleCoralBlock => {
                vec![]
            }
            Item::DeadFireCoralBlock => {
                vec![]
            }
            Item::DeadHornCoralBlock => {
                vec![]
            }
            Item::TubeCoralBlock => {
                vec![]
            }
            Item::BrainCoralBlock => {
                vec![]
            }
            Item::BubbleCoralBlock => {
                vec![]
            }
            Item::FireCoralBlock => {
                vec![]
            }
            Item::HornCoralBlock => {
                vec![]
            }
            Item::TubeCoral => {
                vec![]
            }
            Item::BrainCoral => {
                vec![]
            }
            Item::BubbleCoral => {
                vec![]
            }
            Item::FireCoral => {
                vec![]
            }
            Item::HornCoral => {
                vec![]
            }
            Item::DeadBrainCoral => {
                vec![]
            }
            Item::DeadBubbleCoral => {
                vec![]
            }
            Item::DeadFireCoral => {
                vec![]
            }
            Item::DeadHornCoral => {
                vec![]
            }
            Item::DeadTubeCoral => {
                vec![]
            }
            Item::TubeCoralFan => {
                vec![]
            }
            Item::BrainCoralFan => {
                vec![]
            }
            Item::BubbleCoralFan => {
                vec![]
            }
            Item::FireCoralFan => {
                vec![]
            }
            Item::HornCoralFan => {
                vec![]
            }
            Item::DeadTubeCoralFan => {
                vec![]
            }
            Item::DeadBrainCoralFan => {
                vec![]
            }
            Item::DeadBubbleCoralFan => {
                vec![]
            }
            Item::DeadFireCoralFan => {
                vec![]
            }
            Item::DeadHornCoralFan => {
                vec![]
            }
            Item::BlueIce => {
                vec![]
            }
            Item::Conduit => {
                vec![]
            }
            Item::PolishedGraniteStairs => {
                vec![]
            }
            Item::SmoothRedSandstoneStairs => {
                vec![]
            }
            Item::MossyStoneBrickStairs => {
                vec![]
            }
            Item::PolishedDioriteStairs => {
                vec![]
            }
            Item::MossyCobblestoneStairs => {
                vec![]
            }
            Item::EndStoneBrickStairs => {
                vec![]
            }
            Item::StoneStairs => {
                vec![]
            }
            Item::SmoothSandstoneStairs => {
                vec![]
            }
            Item::SmoothQuartzStairs => {
                vec![]
            }
            Item::GraniteStairs => {
                vec![]
            }
            Item::AndesiteStairs => {
                vec![]
            }
            Item::RedNetherBrickStairs => {
                vec![]
            }
            Item::PolishedAndesiteStairs => {
                vec![]
            }
            Item::DioriteStairs => {
                vec![]
            }
            Item::CobbledDeepslateStairs => {
                vec![]
            }
            Item::PolishedDeepslateStairs => {
                vec![]
            }
            Item::DeepslateBrickStairs => {
                vec![]
            }
            Item::DeepslateTileStairs => {
                vec![]
            }
            Item::PolishedGraniteSlab => {
                vec![]
            }
            Item::SmoothRedSandstoneSlab => {
                vec![]
            }
            Item::MossyStoneBrickSlab => {
                vec![]
            }
            Item::PolishedDioriteSlab => {
                vec![]
            }
            Item::MossyCobblestoneSlab => {
                vec![]
            }
            Item::EndStoneBrickSlab => {
                vec![]
            }
            Item::SmoothSandstoneSlab => {
                vec![]
            }
            Item::SmoothQuartzSlab => {
                vec![]
            }
            Item::GraniteSlab => {
                vec![]
            }
            Item::AndesiteSlab => {
                vec![]
            }
            Item::RedNetherBrickSlab => {
                vec![]
            }
            Item::PolishedAndesiteSlab => {
                vec![]
            }
            Item::DioriteSlab => {
                vec![]
            }
            Item::CobbledDeepslateSlab => {
                vec![]
            }
            Item::PolishedDeepslateSlab => {
                vec![]
            }
            Item::DeepslateBrickSlab => {
                vec![]
            }
            Item::DeepslateTileSlab => {
                vec![]
            }
            Item::Scaffolding => {
                vec![]
            }
            Item::Redstone => {
                vec![]
            }
            Item::RedstoneTorch => {
                vec![]
            }
            Item::RedstoneBlock => {
                vec![]
            }
            Item::Repeater => {
                vec![]
            }
            Item::Comparator => {
                vec![]
            }
            Item::Piston => {
                vec![]
            }
            Item::StickyPiston => {
                vec![]
            }
            Item::SlimeBlock => {
                vec![]
            }
            Item::HoneyBlock => {
                vec![]
            }
            Item::Observer => {
                vec![]
            }
            Item::Hopper => {
                vec![]
            }
            Item::Dispenser => {
                vec![]
            }
            Item::Dropper => {
                vec![]
            }
            Item::Lectern => {
                vec![]
            }
            Item::Target => {
                vec![]
            }
            Item::Lever => {
                vec![]
            }
            Item::LightningRod => {
                vec![]
            }
            Item::DaylightDetector => {
                vec![]
            }
            Item::SculkSensor => {
                vec![]
            }
            Item::TripwireHook => {
                vec![]
            }
            Item::TrappedChest => {
                vec![]
            }
            Item::Tnt => {
                vec![]
            }
            Item::RedstoneLamp => {
                vec![]
            }
            Item::NoteBlock => {
                vec![]
            }
            Item::StoneButton => {
                vec![]
            }
            Item::PolishedBlackstoneButton => {
                vec![]
            }
            Item::OakButton => {
                vec![]
            }
            Item::SpruceButton => {
                vec![]
            }
            Item::BirchButton => {
                vec![]
            }
            Item::JungleButton => {
                vec![]
            }
            Item::AcaciaButton => {
                vec![]
            }
            Item::DarkOakButton => {
                vec![]
            }
            Item::CrimsonButton => {
                vec![]
            }
            Item::WarpedButton => {
                vec![]
            }
            Item::StonePressurePlate => {
                vec![]
            }
            Item::PolishedBlackstonePressurePlate => {
                vec![]
            }
            Item::LightWeightedPressurePlate => {
                vec![]
            }
            Item::HeavyWeightedPressurePlate => {
                vec![]
            }
            Item::OakPressurePlate => {
                vec![]
            }
            Item::SprucePressurePlate => {
                vec![]
            }
            Item::BirchPressurePlate => {
                vec![]
            }
            Item::JunglePressurePlate => {
                vec![]
            }
            Item::AcaciaPressurePlate => {
                vec![]
            }
            Item::DarkOakPressurePlate => {
                vec![]
            }
            Item::CrimsonPressurePlate => {
                vec![]
            }
            Item::WarpedPressurePlate => {
                vec![]
            }
            Item::IronDoor => {
                vec![]
            }
            Item::OakDoor => {
                vec![]
            }
            Item::SpruceDoor => {
                vec![]
            }
            Item::BirchDoor => {
                vec![]
            }
            Item::JungleDoor => {
                vec![]
            }
            Item::AcaciaDoor => {
                vec![]
            }
            Item::DarkOakDoor => {
                vec![]
            }
            Item::CrimsonDoor => {
                vec![]
            }
            Item::WarpedDoor => {
                vec![]
            }
            Item::IronTrapdoor => {
                vec![]
            }
            Item::OakTrapdoor => {
                vec![]
            }
            Item::SpruceTrapdoor => {
                vec![]
            }
            Item::BirchTrapdoor => {
                vec![]
            }
            Item::JungleTrapdoor => {
                vec![]
            }
            Item::AcaciaTrapdoor => {
                vec![]
            }
            Item::DarkOakTrapdoor => {
                vec![]
            }
            Item::CrimsonTrapdoor => {
                vec![]
            }
            Item::WarpedTrapdoor => {
                vec![]
            }
            Item::OakFenceGate => {
                vec![]
            }
            Item::SpruceFenceGate => {
                vec![]
            }
            Item::BirchFenceGate => {
                vec![]
            }
            Item::JungleFenceGate => {
                vec![]
            }
            Item::AcaciaFenceGate => {
                vec![]
            }
            Item::DarkOakFenceGate => {
                vec![]
            }
            Item::CrimsonFenceGate => {
                vec![]
            }
            Item::WarpedFenceGate => {
                vec![]
            }
            Item::PoweredRail => {
                vec![]
            }
            Item::DetectorRail => {
                vec![]
            }
            Item::Rail => {
                vec![]
            }
            Item::ActivatorRail => {
                vec![]
            }
            Item::Saddle => {
                vec![]
            }
            Item::Minecart => {
                vec![]
            }
            Item::ChestMinecart => {
                vec![]
            }
            Item::FurnaceMinecart => {
                vec![]
            }
            Item::TntMinecart => {
                vec![]
            }
            Item::HopperMinecart => {
                vec![]
            }
            Item::CarrotOnAStick => {
                vec![]
            }
            Item::WarpedFungusOnAStick => {
                vec![]
            }
            Item::Elytra => {
                vec![]
            }
            Item::OakBoat => {
                vec![]
            }
            Item::SpruceBoat => {
                vec![]
            }
            Item::BirchBoat => {
                vec![]
            }
            Item::JungleBoat => {
                vec![]
            }
            Item::AcaciaBoat => {
                vec![]
            }
            Item::DarkOakBoat => {
                vec![]
            }
            Item::StructureBlock => {
                vec![]
            }
            Item::Jigsaw => {
                vec![]
            }
            Item::TurtleHelmet => {
                vec![]
            }
            Item::Scute => {
                vec![]
            }
            Item::FlintAndSteel => {
                vec![]
            }
            Item::Apple => {
                vec![]
            }
            Item::Bow => {
                vec![]
            }
            Item::Arrow => {
                vec![]
            }
            Item::Coal => {
                vec![]
            }
            Item::Charcoal => {
                vec![]
            }
            Item::Diamond => {
                vec![]
            }
            Item::Emerald => {
                vec![]
            }
            Item::LapisLazuli => {
                vec![]
            }
            Item::Quartz => {
                vec![]
            }
            Item::AmethystShard => {
                vec![]
            }
            Item::RawIron => {
                vec![]
            }
            Item::IronIngot => {
                vec![]
            }
            Item::RawCopper => {
                vec![]
            }
            Item::CopperIngot => {
                vec![]
            }
            Item::RawGold => {
                vec![]
            }
            Item::GoldIngot => {
                vec![]
            }
            Item::NetheriteIngot => {
                vec![]
            }
            Item::NetheriteScrap => {
                vec![]
            }
            Item::WoodenSword => {
                vec![]
            }
            Item::WoodenShovel => {
                vec![]
            }
            Item::WoodenPickaxe => {
                vec![]
            }
            Item::WoodenAxe => {
                vec![]
            }
            Item::WoodenHoe => {
                vec![]
            }
            Item::StoneSword => {
                vec![]
            }
            Item::StoneShovel => {
                vec![]
            }
            Item::StonePickaxe => {
                vec![]
            }
            Item::StoneAxe => {
                vec![]
            }
            Item::StoneHoe => {
                vec![]
            }
            Item::GoldenSword => {
                vec![]
            }
            Item::GoldenShovel => {
                vec![]
            }
            Item::GoldenPickaxe => {
                vec![]
            }
            Item::GoldenAxe => {
                vec![]
            }
            Item::GoldenHoe => {
                vec![]
            }
            Item::IronSword => {
                vec![]
            }
            Item::IronShovel => {
                vec![]
            }
            Item::IronPickaxe => {
                vec![]
            }
            Item::IronAxe => {
                vec![]
            }
            Item::IronHoe => {
                vec![]
            }
            Item::DiamondSword => {
                vec![]
            }
            Item::DiamondShovel => {
                vec![]
            }
            Item::DiamondPickaxe => {
                vec![]
            }
            Item::DiamondAxe => {
                vec![]
            }
            Item::DiamondHoe => {
                vec![]
            }
            Item::NetheriteSword => {
                vec![]
            }
            Item::NetheriteShovel => {
                vec![]
            }
            Item::NetheritePickaxe => {
                vec![]
            }
            Item::NetheriteAxe => {
                vec![]
            }
            Item::NetheriteHoe => {
                vec![]
            }
            Item::Stick => {
                vec![]
            }
            Item::Bowl => {
                vec![]
            }
            Item::MushroomStew => {
                vec![]
            }
            Item::String => {
                vec![]
            }
            Item::Feather => {
                vec![]
            }
            Item::Gunpowder => {
                vec![]
            }
            Item::WheatSeeds => {
                vec![]
            }
            Item::Wheat => {
                vec![]
            }
            Item::Bread => {
                vec![]
            }
            Item::LeatherHelmet => {
                vec![]
            }
            Item::LeatherChestplate => {
                vec![]
            }
            Item::LeatherLeggings => {
                vec![]
            }
            Item::LeatherBoots => {
                vec![]
            }
            Item::ChainmailHelmet => {
                vec![]
            }
            Item::ChainmailChestplate => {
                vec![]
            }
            Item::ChainmailLeggings => {
                vec![]
            }
            Item::ChainmailBoots => {
                vec![]
            }
            Item::IronHelmet => {
                vec![]
            }
            Item::IronChestplate => {
                vec![]
            }
            Item::IronLeggings => {
                vec![]
            }
            Item::IronBoots => {
                vec![]
            }
            Item::DiamondHelmet => {
                vec![]
            }
            Item::DiamondChestplate => {
                vec![]
            }
            Item::DiamondLeggings => {
                vec![]
            }
            Item::DiamondBoots => {
                vec![]
            }
            Item::GoldenHelmet => {
                vec![]
            }
            Item::GoldenChestplate => {
                vec![]
            }
            Item::GoldenLeggings => {
                vec![]
            }
            Item::GoldenBoots => {
                vec![]
            }
            Item::NetheriteHelmet => {
                vec![]
            }
            Item::NetheriteChestplate => {
                vec![]
            }
            Item::NetheriteLeggings => {
                vec![]
            }
            Item::NetheriteBoots => {
                vec![]
            }
            Item::Flint => {
                vec![]
            }
            Item::Porkchop => {
                vec![]
            }
            Item::CookedPorkchop => {
                vec![]
            }
            Item::Painting => {
                vec![]
            }
            Item::GoldenApple => {
                vec![]
            }
            Item::EnchantedGoldenApple => {
                vec![]
            }
            Item::OakSign => {
                vec![]
            }
            Item::SpruceSign => {
                vec![]
            }
            Item::BirchSign => {
                vec![]
            }
            Item::JungleSign => {
                vec![]
            }
            Item::AcaciaSign => {
                vec![]
            }
            Item::DarkOakSign => {
                vec![]
            }
            Item::CrimsonSign => {
                vec![]
            }
            Item::WarpedSign => {
                vec![]
            }
            Item::Bucket => {
                vec![]
            }
            Item::WaterBucket => {
                vec![]
            }
            Item::LavaBucket => {
                vec![]
            }
            Item::PowderSnowBucket => {
                vec![]
            }
            Item::Snowball => {
                vec![]
            }
            Item::Leather => {
                vec![]
            }
            Item::MilkBucket => {
                vec![]
            }
            Item::PufferfishBucket => {
                vec![]
            }
            Item::SalmonBucket => {
                vec![]
            }
            Item::CodBucket => {
                vec![]
            }
            Item::TropicalFishBucket => {
                vec![]
            }
            Item::AxolotlBucket => {
                vec![]
            }
            Item::Brick => {
                vec![]
            }
            Item::ClayBall => {
                vec![]
            }
            Item::DriedKelpBlock => {
                vec![]
            }
            Item::Paper => {
                vec![]
            }
            Item::Book => {
                vec![]
            }
            Item::SlimeBall => {
                vec![]
            }
            Item::Egg => {
                vec![]
            }
            Item::Compass => {
                vec![]
            }
            Item::Bundle => {
                vec![]
            }
            Item::FishingRod => {
                vec![]
            }
            Item::Clock => {
                vec![]
            }
            Item::Spyglass => {
                vec![]
            }
            Item::GlowstoneDust => {
                vec![]
            }
            Item::Cod => {
                vec![]
            }
            Item::Salmon => {
                vec![]
            }
            Item::TropicalFish => {
                vec![]
            }
            Item::Pufferfish => {
                vec![]
            }
            Item::CookedCod => {
                vec![]
            }
            Item::CookedSalmon => {
                vec![]
            }
            Item::InkSac => {
                vec![]
            }
            Item::GlowInkSac => {
                vec![]
            }
            Item::CocoaBeans => {
                vec![]
            }
            Item::WhiteDye => {
                vec![]
            }
            Item::OrangeDye => {
                vec![]
            }
            Item::MagentaDye => {
                vec![]
            }
            Item::LightBlueDye => {
                vec![]
            }
            Item::YellowDye => {
                vec![]
            }
            Item::LimeDye => {
                vec![]
            }
            Item::PinkDye => {
                vec![]
            }
            Item::GrayDye => {
                vec![]
            }
            Item::LightGrayDye => {
                vec![]
            }
            Item::CyanDye => {
                vec![]
            }
            Item::PurpleDye => {
                vec![]
            }
            Item::BlueDye => {
                vec![]
            }
            Item::BrownDye => {
                vec![]
            }
            Item::GreenDye => {
                vec![]
            }
            Item::RedDye => {
                vec![]
            }
            Item::BlackDye => {
                vec![]
            }
            Item::BoneMeal => {
                vec![]
            }
            Item::Bone => {
                vec![]
            }
            Item::Sugar => {
                vec![]
            }
            Item::Cake => {
                vec![]
            }
            Item::WhiteBed => {
                vec![]
            }
            Item::OrangeBed => {
                vec![]
            }
            Item::MagentaBed => {
                vec![]
            }
            Item::LightBlueBed => {
                vec![]
            }
            Item::YellowBed => {
                vec![]
            }
            Item::LimeBed => {
                vec![]
            }
            Item::PinkBed => {
                vec![]
            }
            Item::GrayBed => {
                vec![]
            }
            Item::LightGrayBed => {
                vec![]
            }
            Item::CyanBed => {
                vec![]
            }
            Item::PurpleBed => {
                vec![]
            }
            Item::BlueBed => {
                vec![]
            }
            Item::BrownBed => {
                vec![]
            }
            Item::GreenBed => {
                vec![]
            }
            Item::RedBed => {
                vec![]
            }
            Item::BlackBed => {
                vec![]
            }
            Item::Cookie => {
                vec![]
            }
            Item::FilledMap => {
                vec![]
            }
            Item::Shears => {
                vec![]
            }
            Item::MelonSlice => {
                vec![]
            }
            Item::DriedKelp => {
                vec![]
            }
            Item::PumpkinSeeds => {
                vec![]
            }
            Item::MelonSeeds => {
                vec![]
            }
            Item::Beef => {
                vec![]
            }
            Item::CookedBeef => {
                vec![]
            }
            Item::Chicken => {
                vec![]
            }
            Item::CookedChicken => {
                vec![]
            }
            Item::RottenFlesh => {
                vec![]
            }
            Item::EnderPearl => {
                vec![]
            }
            Item::BlazeRod => {
                vec![]
            }
            Item::GhastTear => {
                vec![]
            }
            Item::GoldNugget => {
                vec![]
            }
            Item::NetherWart => {
                vec![]
            }
            Item::Potion => {
                vec![]
            }
            Item::GlassBottle => {
                vec![]
            }
            Item::SpiderEye => {
                vec![]
            }
            Item::FermentedSpiderEye => {
                vec![]
            }
            Item::BlazePowder => {
                vec![]
            }
            Item::MagmaCream => {
                vec![]
            }
            Item::BrewingStand => {
                vec![]
            }
            Item::Cauldron => {
                vec![]
            }
            Item::EnderEye => {
                vec![]
            }
            Item::GlisteringMelonSlice => {
                vec![]
            }
            Item::AxolotlSpawnEgg => {
                vec![]
            }
            Item::BatSpawnEgg => {
                vec![]
            }
            Item::BeeSpawnEgg => {
                vec![]
            }
            Item::BlazeSpawnEgg => {
                vec![]
            }
            Item::CatSpawnEgg => {
                vec![]
            }
            Item::CaveSpiderSpawnEgg => {
                vec![]
            }
            Item::ChickenSpawnEgg => {
                vec![]
            }
            Item::CodSpawnEgg => {
                vec![]
            }
            Item::CowSpawnEgg => {
                vec![]
            }
            Item::CreeperSpawnEgg => {
                vec![]
            }
            Item::DolphinSpawnEgg => {
                vec![]
            }
            Item::DonkeySpawnEgg => {
                vec![]
            }
            Item::DrownedSpawnEgg => {
                vec![]
            }
            Item::ElderGuardianSpawnEgg => {
                vec![]
            }
            Item::EndermanSpawnEgg => {
                vec![]
            }
            Item::EndermiteSpawnEgg => {
                vec![]
            }
            Item::EvokerSpawnEgg => {
                vec![]
            }
            Item::FoxSpawnEgg => {
                vec![]
            }
            Item::GhastSpawnEgg => {
                vec![]
            }
            Item::GlowSquidSpawnEgg => {
                vec![]
            }
            Item::GoatSpawnEgg => {
                vec![]
            }
            Item::GuardianSpawnEgg => {
                vec![]
            }
            Item::HoglinSpawnEgg => {
                vec![]
            }
            Item::HorseSpawnEgg => {
                vec![]
            }
            Item::HuskSpawnEgg => {
                vec![]
            }
            Item::LlamaSpawnEgg => {
                vec![]
            }
            Item::MagmaCubeSpawnEgg => {
                vec![]
            }
            Item::MooshroomSpawnEgg => {
                vec![]
            }
            Item::MuleSpawnEgg => {
                vec![]
            }
            Item::OcelotSpawnEgg => {
                vec![]
            }
            Item::PandaSpawnEgg => {
                vec![]
            }
            Item::ParrotSpawnEgg => {
                vec![]
            }
            Item::PhantomSpawnEgg => {
                vec![]
            }
            Item::PigSpawnEgg => {
                vec![]
            }
            Item::PiglinSpawnEgg => {
                vec![]
            }
            Item::PiglinBruteSpawnEgg => {
                vec![]
            }
            Item::PillagerSpawnEgg => {
                vec![]
            }
            Item::PolarBearSpawnEgg => {
                vec![]
            }
            Item::PufferfishSpawnEgg => {
                vec![]
            }
            Item::RabbitSpawnEgg => {
                vec![]
            }
            Item::RavagerSpawnEgg => {
                vec![]
            }
            Item::SalmonSpawnEgg => {
                vec![]
            }
            Item::SheepSpawnEgg => {
                vec![]
            }
            Item::ShulkerSpawnEgg => {
                vec![]
            }
            Item::SilverfishSpawnEgg => {
                vec![]
            }
            Item::SkeletonSpawnEgg => {
                vec![]
            }
            Item::SkeletonHorseSpawnEgg => {
                vec![]
            }
            Item::SlimeSpawnEgg => {
                vec![]
            }
            Item::SpiderSpawnEgg => {
                vec![]
            }
            Item::SquidSpawnEgg => {
                vec![]
            }
            Item::StraySpawnEgg => {
                vec![]
            }
            Item::StriderSpawnEgg => {
                vec![]
            }
            Item::TraderLlamaSpawnEgg => {
                vec![]
            }
            Item::TropicalFishSpawnEgg => {
                vec![]
            }
            Item::TurtleSpawnEgg => {
                vec![]
            }
            Item::VexSpawnEgg => {
                vec![]
            }
            Item::VillagerSpawnEgg => {
                vec![]
            }
            Item::VindicatorSpawnEgg => {
                vec![]
            }
            Item::WanderingTraderSpawnEgg => {
                vec![]
            }
            Item::WitchSpawnEgg => {
                vec![]
            }
            Item::WitherSkeletonSpawnEgg => {
                vec![]
            }
            Item::WolfSpawnEgg => {
                vec![]
            }
            Item::ZoglinSpawnEgg => {
                vec![]
            }
            Item::ZombieSpawnEgg => {
                vec![]
            }
            Item::ZombieHorseSpawnEgg => {
                vec![]
            }
            Item::ZombieVillagerSpawnEgg => {
                vec![]
            }
            Item::ZombifiedPiglinSpawnEgg => {
                vec![]
            }
            Item::ExperienceBottle => {
                vec![]
            }
            Item::FireCharge => {
                vec![]
            }
            Item::WritableBook => {
                vec![]
            }
            Item::WrittenBook => {
                vec![]
            }
            Item::ItemFrame => {
                vec![]
            }
            Item::GlowItemFrame => {
                vec![]
            }
            Item::FlowerPot => {
                vec![]
            }
            Item::Carrot => {
                vec![]
            }
            Item::Potato => {
                vec![]
            }
            Item::BakedPotato => {
                vec![]
            }
            Item::PoisonousPotato => {
                vec![]
            }
            Item::Map => {
                vec![]
            }
            Item::GoldenCarrot => {
                vec![]
            }
            Item::SkeletonSkull => {
                vec![]
            }
            Item::WitherSkeletonSkull => {
                vec![]
            }
            Item::PlayerHead => {
                vec![]
            }
            Item::ZombieHead => {
                vec![]
            }
            Item::CreeperHead => {
                vec![]
            }
            Item::DragonHead => {
                vec![]
            }
            Item::NetherStar => {
                vec![]
            }
            Item::PumpkinPie => {
                vec![]
            }
            Item::FireworkRocket => {
                vec![]
            }
            Item::FireworkStar => {
                vec![]
            }
            Item::EnchantedBook => {
                vec![]
            }
            Item::NetherBrick => {
                vec![]
            }
            Item::PrismarineShard => {
                vec![]
            }
            Item::PrismarineCrystals => {
                vec![]
            }
            Item::Rabbit => {
                vec![]
            }
            Item::CookedRabbit => {
                vec![]
            }
            Item::RabbitStew => {
                vec![]
            }
            Item::RabbitFoot => {
                vec![]
            }
            Item::RabbitHide => {
                vec![]
            }
            Item::ArmorStand => {
                vec![]
            }
            Item::IronHorseArmor => {
                vec![]
            }
            Item::GoldenHorseArmor => {
                vec![]
            }
            Item::DiamondHorseArmor => {
                vec![]
            }
            Item::LeatherHorseArmor => {
                vec![]
            }
            Item::Lead => {
                vec![]
            }
            Item::NameTag => {
                vec![]
            }
            Item::CommandBlockMinecart => {
                vec![]
            }
            Item::Mutton => {
                vec![]
            }
            Item::CookedMutton => {
                vec![]
            }
            Item::WhiteBanner => {
                vec![]
            }
            Item::OrangeBanner => {
                vec![]
            }
            Item::MagentaBanner => {
                vec![]
            }
            Item::LightBlueBanner => {
                vec![]
            }
            Item::YellowBanner => {
                vec![]
            }
            Item::LimeBanner => {
                vec![]
            }
            Item::PinkBanner => {
                vec![]
            }
            Item::GrayBanner => {
                vec![]
            }
            Item::LightGrayBanner => {
                vec![]
            }
            Item::CyanBanner => {
                vec![]
            }
            Item::PurpleBanner => {
                vec![]
            }
            Item::BlueBanner => {
                vec![]
            }
            Item::BrownBanner => {
                vec![]
            }
            Item::GreenBanner => {
                vec![]
            }
            Item::RedBanner => {
                vec![]
            }
            Item::BlackBanner => {
                vec![]
            }
            Item::EndCrystal => {
                vec![]
            }
            Item::ChorusFruit => {
                vec![]
            }
            Item::PoppedChorusFruit => {
                vec![]
            }
            Item::Beetroot => {
                vec![]
            }
            Item::BeetrootSeeds => {
                vec![]
            }
            Item::BeetrootSoup => {
                vec![]
            }
            Item::DragonBreath => {
                vec![]
            }
            Item::SplashPotion => {
                vec![]
            }
            Item::SpectralArrow => {
                vec![]
            }
            Item::TippedArrow => {
                vec![]
            }
            Item::LingeringPotion => {
                vec![]
            }
            Item::Shield => {
                vec![]
            }
            Item::TotemOfUndying => {
                vec![]
            }
            Item::ShulkerShell => {
                vec![]
            }
            Item::IronNugget => {
                vec![]
            }
            Item::KnowledgeBook => {
                vec![]
            }
            Item::DebugStick => {
                vec![]
            }
            Item::MusicDisc13 => {
                vec![]
            }
            Item::MusicDiscCat => {
                vec![]
            }
            Item::MusicDiscBlocks => {
                vec![]
            }
            Item::MusicDiscChirp => {
                vec![]
            }
            Item::MusicDiscFar => {
                vec![]
            }
            Item::MusicDiscMall => {
                vec![]
            }
            Item::MusicDiscMellohi => {
                vec![]
            }
            Item::MusicDiscStal => {
                vec![]
            }
            Item::MusicDiscStrad => {
                vec![]
            }
            Item::MusicDiscWard => {
                vec![]
            }
            Item::MusicDisc11 => {
                vec![]
            }
            Item::MusicDiscWait => {
                vec![]
            }
            Item::MusicDiscOtherside => {
                vec![]
            }
            Item::MusicDiscPigstep => {
                vec![]
            }
            Item::Trident => {
                vec![]
            }
            Item::PhantomMembrane => {
                vec![]
            }
            Item::NautilusShell => {
                vec![]
            }
            Item::HeartOfTheSea => {
                vec![]
            }
            Item::Crossbow => {
                vec![]
            }
            Item::SuspiciousStew => {
                vec![]
            }
            Item::Loom => {
                vec![]
            }
            Item::FlowerBannerPattern => {
                vec![]
            }
            Item::CreeperBannerPattern => {
                vec![]
            }
            Item::SkullBannerPattern => {
                vec![]
            }
            Item::MojangBannerPattern => {
                vec![]
            }
            Item::GlobeBannerPattern => {
                vec![]
            }
            Item::PiglinBannerPattern => {
                vec![]
            }
            Item::Composter => {
                vec![]
            }
            Item::Barrel => {
                vec![]
            }
            Item::Smoker => {
                vec![]
            }
            Item::BlastFurnace => {
                vec![]
            }
            Item::CartographyTable => {
                vec![]
            }
            Item::FletchingTable => {
                vec![]
            }
            Item::Grindstone => {
                vec![]
            }
            Item::SmithingTable => {
                vec![]
            }
            Item::Stonecutter => {
                vec![]
            }
            Item::Bell => {
                vec![]
            }
            Item::Lantern => {
                vec![]
            }
            Item::SoulLantern => {
                vec![]
            }
            Item::SweetBerries => {
                vec![]
            }
            Item::GlowBerries => {
                vec![]
            }
            Item::Campfire => {
                vec![]
            }
            Item::SoulCampfire => {
                vec![]
            }
            Item::Shroomlight => {
                vec![]
            }
            Item::Honeycomb => {
                vec![]
            }
            Item::BeeNest => {
                vec![]
            }
            Item::Beehive => {
                vec![]
            }
            Item::HoneyBottle => {
                vec![]
            }
            Item::HoneycombBlock => {
                vec![]
            }
            Item::Lodestone => {
                vec![]
            }
            Item::CryingObsidian => {
                vec![]
            }
            Item::Blackstone => {
                vec![]
            }
            Item::BlackstoneSlab => {
                vec![]
            }
            Item::BlackstoneStairs => {
                vec![]
            }
            Item::GildedBlackstone => {
                vec![]
            }
            Item::PolishedBlackstone => {
                vec![]
            }
            Item::PolishedBlackstoneSlab => {
                vec![]
            }
            Item::PolishedBlackstoneStairs => {
                vec![]
            }
            Item::ChiseledPolishedBlackstone => {
                vec![]
            }
            Item::PolishedBlackstoneBricks => {
                vec![]
            }
            Item::PolishedBlackstoneBrickSlab => {
                vec![]
            }
            Item::PolishedBlackstoneBrickStairs => {
                vec![]
            }
            Item::CrackedPolishedBlackstoneBricks => {
                vec![]
            }
            Item::RespawnAnchor => {
                vec![]
            }
            Item::Candle => {
                vec![]
            }
            Item::WhiteCandle => {
                vec![]
            }
            Item::OrangeCandle => {
                vec![]
            }
            Item::MagentaCandle => {
                vec![]
            }
            Item::LightBlueCandle => {
                vec![]
            }
            Item::YellowCandle => {
                vec![]
            }
            Item::LimeCandle => {
                vec![]
            }
            Item::PinkCandle => {
                vec![]
            }
            Item::GrayCandle => {
                vec![]
            }
            Item::LightGrayCandle => {
                vec![]
            }
            Item::CyanCandle => {
                vec![]
            }
            Item::PurpleCandle => {
                vec![]
            }
            Item::BlueCandle => {
                vec![]
            }
            Item::BrownCandle => {
                vec![]
            }
            Item::GreenCandle => {
                vec![]
            }
            Item::RedCandle => {
                vec![]
            }
            Item::BlackCandle => {
                vec![]
            }
            Item::SmallAmethystBud => {
                vec![]
            }
            Item::MediumAmethystBud => {
                vec![]
            }
            Item::LargeAmethystBud => {
                vec![]
            }
            Item::AmethystCluster => {
                vec![]
            }
            Item::PointedDripstone => {
                vec![]
            }
        }
    }
}
use std::convert::TryFrom;
use std::str::FromStr;
impl TryFrom<String> for Item {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Some(item) = Item::from_name(value.as_str()) {
            Ok(item)
        } else {
            Err("Unknown item name")
        }
    }
}
impl From<Item> for &'static str {
    fn from(i: Item) -> Self {
        i.name()
    }
}
impl FromStr for Item {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(item) = Item::from_name(s) {
            Ok(item)
        } else {
            Err("Unknown item name")
        }
    }
}
