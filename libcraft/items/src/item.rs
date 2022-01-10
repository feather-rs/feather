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
            Item::CraftingTable => 246,
            Item::PinkTerracotta => 360,
            Item::BirchFence => 259,
            Item::OrangeTerracotta => 355,
            Item::Spyglass => 799,
            Item::LingeringPotion => 1008,
            Item::BlackBanner => 997,
            Item::BlueStainedGlass => 411,
            Item::StoneHoe => 708,
            Item::CyanTerracotta => 363,
            Item::WeepingVines => 194,
            Item::LightGrayWool => 165,
            Item::Crossbow => 1033,
            Item::SprucePressurePlate => 624,
            Item::StoneSword => 704,
            Item::JackOLantern => 267,
            Item::CowSpawnEgg => 881,
            Item::HornCoral => 531,
            Item::MagentaCarpet => 375,
            Item::PurpleCandle => 1090,
            Item::SpruceStairs => 318,
            Item::StrippedSpruceLog => 110,
            Item::NetheriteHelmet => 758,
            Item::EnderEye => 871,
            Item::Beetroot => 1001,
            Item::BlueTerracotta => 365,
            Item::WaxedOxidizedCutCopperStairs => 96,
            Item::DarkOakDoor => 637,
            Item::MossyCobblestoneSlab => 571,
            Item::CyanShulkerBox => 461,
            Item::Observer => 594,
            Item::IronBlock => 65,
            Item::CyanDye => 819,
            Item::Grindstone => 1048,
            Item::GildedBlackstone => 1069,
            Item::NetheriteBlock => 69,
            Item::HornCoralBlock => 526,
            Item::Shroomlight => 1058,
            Item::CopperOre => 44,
            Item::CobblestoneWall => 325,
            Item::Campfire => 1056,
            Item::PolishedBlackstoneWall => 340,
            Item::OxeyeDaisy => 182,
            Item::OakFenceGate => 649,
            Item::PufferfishBucket => 783,
            Item::YellowStainedGlassPane => 420,
            Item::SeaLantern => 438,
            Item::WarpedStairs => 322,
            Item::LimeGlazedTerracotta => 473,
            Item::RabbitFoot => 970,
            Item::GoldenAxe => 712,
            Item::Andesite => 6,
            Item::GlowSquidSpawnEgg => 892,
            Item::NoteBlock => 608,
            Item::OrangeShulkerBox => 453,
            Item::DarkOakTrapdoor => 646,
            Item::DriedKelpBlock => 790,
            Item::DeadHornCoralBlock => 521,
            Item::CrimsonSlab => 210,
            Item::GreenStainedGlassPane => 429,
            Item::BrownTerracotta => 366,
            Item::Bricks => 232,
            Item::IronSword => 714,
            Item::FlowerBannerPattern => 1036,
            Item::Bell => 1051,
            Item::AmethystCluster => 1099,
            Item::RedstoneOre => 48,
            Item::PolishedDiorite => 5,
            Item::CrackedStoneBricks => 285,
            Item::MushroomStew => 731,
            Item::LightBlueCandle => 1083,
            Item::OakSlab => 204,
            Item::Clay => 255,
            Item::BirchPlanks => 24,
            Item::BuddingAmethyst => 64,
            Item::StoneButton => 609,
            Item::SlimeBlock => 592,
            Item::DeepslateCoalOre => 41,
            Item::CutSandstoneSlab => 215,
            Item::RedDye => 824,
            Item::OakLeaves => 133,
            Item::WhiteBed => 830,
            Item::MusicDiscPigstep => 1028,
            Item::WitherSkeletonSpawnEgg => 933,
            Item::MagentaDye => 812,
            Item::Vine => 299,
            Item::StrippedCrimsonHyphae => 123,
            Item::SmoothSandstoneStairs => 556,
            Item::SpruceButton => 612,
            Item::ClayBall => 789,
            Item::MushroomStem => 294,
            Item::HangingRoots => 200,
            Item::LimeTerracotta => 359,
            Item::BrewingStand => 869,
            Item::Loom => 1035,
            Item::WaxedExposedCutCopperStairs => 94,
            Item::PolishedBlackstoneBricks => 1074,
            Item::BrownStainedGlassPane => 428,
            Item::AmethystBlock => 63,
            Item::DragonHead => 958,
            Item::OrangeDye => 811,
            Item::SmallDripleaf => 202,
            Item::ChorusPlant => 238,
            Item::PolishedDeepslateSlab => 581,
            Item::Bone => 827,
            Item::CookedMutton => 981,
            Item::WarpedStem => 108,
            Item::BrownMushroom => 187,
            Item::Pumpkin => 265,
            Item::WaxedCopperBlock => 85,
            Item::TurtleEgg => 516,
            Item::Azalea => 152,
            Item::LightBlueBed => 833,
            Item::Honeycomb => 1059,
            Item::CatSpawnEgg => 877,
            Item::MossyCobblestoneWall => 326,
            Item::CobblestoneSlab => 217,
            Item::Obsidian => 235,
            Item::GrayStainedGlass => 407,
            Item::PolishedAndesiteSlab => 578,
            Item::CrimsonTrapdoor => 647,
            Item::GoldenHorseArmor => 974,
            Item::SpruceDoor => 633,
            Item::IronPickaxe => 716,
            Item::CrimsonButton => 617,
            Item::BirchFenceGate => 651,
            Item::Mycelium => 303,
            Item::CreeperHead => 957,
            Item::BlackTerracotta => 369,
            Item::HoglinSpawnEgg => 895,
            Item::InfestedStone => 276,
            Item::FermentedSpiderEye => 866,
            Item::TrappedChest => 605,
            Item::VindicatorSpawnEgg => 930,
            Item::RawCopper => 693,
            Item::NetheriteAxe => 727,
            Item::LimeWool => 162,
            Item::PinkWool => 163,
            Item::OakPlanks => 22,
            Item::WeatheredCutCopperStairs => 79,
            Item::Wheat => 736,
            Item::GreenStainedGlass => 413,
            Item::DamagedAnvil => 348,
            Item::CreeperBannerPattern => 1037,
            Item::PolishedBlackstoneStairs => 1072,
            Item::BlackstoneSlab => 1067,
            Item::Podzol => 17,
            Item::SoulSand => 269,
            Item::GreenShulkerBox => 465,
            Item::PolishedBlackstonePressurePlate => 620,
            Item::SpruceLog => 102,
            Item::WarpedRoots => 192,
            Item::RedBanner => 996,
            Item::Salmon => 802,
            Item::OrangeCandle => 1081,
            Item::IronTrapdoor => 640,
            Item::IronAxe => 717,
            Item::PhantomMembrane => 1030,
            Item::LimeBanner => 987,
            Item::PurpleBed => 840,
            Item::Bowl => 730,
            Item::RedBed => 844,
            Item::PolishedAndesite => 7,
            Item::BirchStairs => 319,
            Item::BrownCarpet => 385,
            Item::CrimsonStairs => 321,
            Item::Chain => 296,
            Item::PrismarineBrickStairs => 436,
            Item::DeepslateCopperOre => 45,
            Item::ShulkerBox => 451,
            Item::BoneBlock => 449,
            Item::Leather => 781,
            Item::WarpedWartBlock => 447,
            Item::MagentaConcrete => 486,
            Item::WritableBook => 942,
            Item::ChiseledQuartzBlock => 349,
            Item::ChiseledSandstone => 147,
            Item::OrangeCarpet => 374,
            Item::DeepslateGoldOre => 47,
            Item::RedstoneLamp => 607,
            Item::WarpedButton => 618,
            Item::DirtPath => 393,
            Item::BirchTrapdoor => 643,
            Item::WarpedFungusOnAStick => 668,
            Item::BrickSlab => 218,
            Item::GlowstoneDust => 800,
            Item::YellowBanner => 986,
            Item::SplashPotion => 1005,
            Item::LapisBlock => 145,
            Item::Lodestone => 1064,
            Item::Cornflower => 183,
            Item::PointedDripstone => 1100,
            Item::WhiteStainedGlassPane => 416,
            Item::HayBlock => 372,
            Item::GoldenChestplate => 755,
            Item::BirchLog => 103,
            Item::Chicken => 855,
            Item::StrippedCrimsonStem => 115,
            Item::OrangeStainedGlassPane => 417,
            Item::GrayWool => 164,
            Item::CyanGlazedTerracotta => 477,
            Item::RepeatingCommandBlock => 443,
            Item::BlueIce => 547,
            Item::FireCharge => 941,
            Item::Candle => 1079,
            Item::ExposedCutCopperSlab => 82,
            Item::MooshroomSpawnEgg => 900,
            Item::BakedPotato => 949,
            Item::RedShulkerBox => 466,
            Item::Hopper => 595,
            Item::OrangeConcrete => 485,
            Item::StoneShovel => 705,
            Item::OakPressurePlate => 623,
            Item::EndStone => 312,
            Item::BrickStairs => 301,
            Item::DeadBrainCoral => 532,
            Item::ElderGuardianSpawnEgg => 886,
            Item::Light => 371,
            Item::WarpedFence => 264,
            Item::QuartzSlab => 221,
            Item::Gunpowder => 734,
            Item::Bow => 682,
            Item::BrownDye => 822,
            Item::Cod => 801,
            Item::RedNetherBrickSlab => 577,
            Item::MagentaBed => 832,
            Item::AcaciaLog => 105,
            Item::GrayBed => 837,
            Item::InfestedStoneBricks => 278,
            Item::LightGrayStainedGlass => 408,
            Item::TubeCoralBlock => 522,
            Item::InkSac => 807,
            Item::MusicDisc11 => 1025,
            Item::SandstoneSlab => 214,
            Item::BeeNest => 1060,
            Item::LightBlueStainedGlass => 403,
            Item::OakSign => 768,
            Item::ChiseledPolishedBlackstone => 1073,
            Item::PrismarineSlab => 225,
            Item::DarkOakLeaves => 138,
            Item::NetherBrickStairs => 309,
            Item::ZombieHead => 956,
            Item::GraniteWall => 331,
            Item::DeadTubeCoral => 536,
            Item::SpruceSlab => 205,
            Item::PolishedDioriteStairs => 552,
            Item::GuardianSpawnEgg => 894,
            Item::WolfSpawnEgg => 934,
            Item::SquidSpawnEgg => 922,
            Item::PurpleTerracotta => 364,
            Item::TropicalFish => 803,
            Item::FireworkRocket => 961,
            Item::BlueShulkerBox => 463,
            Item::SlimeSpawnEgg => 920,
            Item::DarkOakSlab => 209,
            Item::GoldenHelmet => 754,
            Item::YellowDye => 814,
            Item::StoneBricks => 283,
            Item::DeadHornCoralFan => 546,
            Item::FlowerPot => 946,
            Item::GoldNugget => 861,
            Item::MediumAmethystBud => 1097,
            Item::TntMinecart => 665,
            Item::StrippedWarpedStem => 116,
            Item::Chest => 245,
            Item::SculkSensor => 603,
            Item::PolishedBlackstoneBrickSlab => 1075,
            Item::Cobweb => 149,
            Item::DeadFireCoralBlock => 520,
            Item::BlazePowder => 867,
            Item::JungleButton => 614,
            Item::SkeletonSkull => 953,
            Item::LimeDye => 815,
            Item::PigSpawnEgg => 906,
            Item::Diorite => 4,
            Item::PoweredRail => 657,
            Item::YellowWool => 161,
            Item::RabbitSpawnEgg => 912,
            Item::LightBlueStainedGlassPane => 419,
            Item::LavaBucket => 778,
            Item::AcaciaLeaves => 137,
            Item::Beacon => 324,
            Item::DeadBubbleCoral => 533,
            Item::DarkOakPlanks => 27,
            Item::RoseBush => 396,
            Item::Porkchop => 763,
            Item::HopperMinecart => 666,
            Item::JungleDoor => 635,
            Item::WaterBucket => 777,
            Item::Beef => 853,
            Item::SkeletonSpawnEgg => 918,
            Item::OxidizedCopper => 72,
            Item::MusicDisc13 => 1015,
            Item::PrismarineCrystals => 966,
            Item::NetherQuartzOre => 57,
            Item::WhiteStainedGlass => 400,
            Item::AxolotlSpawnEgg => 873,
            Item::Lilac => 395,
            Item::BoneMeal => 826,
            Item::BrownMushroomBlock => 292,
            Item::GoldenCarrot => 952,
            Item::QuartzPillar => 352,
            Item::IronNugget => 1012,
            Item::Potion => 863,
            Item::GraniteStairs => 558,
            Item::PurpleConcrete => 494,
            Item::NameTag => 978,
            Item::DeepslateTiles => 289,
            Item::DarkOakBoat => 675,
            Item::CommandBlock => 323,
            Item::MusicDiscBlocks => 1017,
            Item::StructureBlock => 676,
            Item::Conduit => 548,
            Item::GoldenPickaxe => 711,
            Item::DiamondChestplate => 751,
            Item::JungleSlab => 207,
            Item::YellowConcrete => 488,
            Item::Redstone => 585,
            Item::IronIngot => 692,
            Item::CodBucket => 785,
            Item::FishingRod => 797,
            Item::GreenDye => 823,
            Item::ChainmailBoots => 745,
            Item::Minecart => 662,
            Item::Terracotta => 389,
            Item::NetherGoldOre => 56,
            Item::BirchLeaves => 135,
            Item::LightGrayStainedGlassPane => 424,
            Item::DarkOakButton => 616,
            Item::LightBlueDye => 813,
            Item::MusicDiscMellohi => 1021,
            Item::CobbledDeepslateSlab => 580,
            Item::CrackedPolishedBlackstoneBricks => 1077,
            Item::WitchSpawnEgg => 932,
            Item::DeadTubeCoralFan => 542,
            Item::WhiteCandle => 1080,
            Item::WeatheredCutCopperSlab => 83,
            Item::GlowItemFrame => 945,
            Item::DeepslateBricks => 287,
            Item::RespawnAnchor => 1078,
            Item::StrippedJungleLog => 112,
            Item::SeaPickle => 156,
            Item::DeepslateEmeraldOre => 51,
            Item::RedCarpet => 387,
            Item::PrismarineBricks => 433,
            Item::MossCarpet => 198,
            Item::NetheriteHoe => 728,
            Item::GoldBlock => 67,
            Item::InfestedCrackedStoneBricks => 280,
            Item::RawGoldBlock => 62,
            Item::JungleSign => 771,
            Item::EnchantedBook => 963,
            Item::BlueBed => 841,
            Item::EndCrystal => 998,
            Item::SandstoneWall => 336,
            Item::ChiseledStoneBricks => 286,
            Item::Torch => 236,
            Item::OakBoat => 670,
            Item::SoulCampfire => 1057,
            Item::Poppy => 174,
            Item::BlueGlazedTerracotta => 479,
            Item::BeetrootSeeds => 1002,
            Item::WeatheredCopper => 71,
            Item::WaxedExposedCutCopperSlab => 98,
            Item::NetherBrickSlab => 220,
            Item::NetherSprouts => 193,
            Item::BlackGlazedTerracotta => 483,
            Item::HoneyBlock => 593,
            Item::OrangeBanner => 983,
            Item::HornCoralFan => 541,
            Item::MagentaCandle => 1082,
            Item::AzaleaLeaves => 139,
            Item::FlintAndSteel => 680,
            Item::GrayStainedGlassPane => 423,
            Item::JungleSapling => 33,
            Item::WaxedWeatheredCutCopperStairs => 95,
            Item::EvokerSpawnEgg => 889,
            Item::SkeletonHorseSpawnEgg => 919,
            Item::MagentaWool => 159,
            Item::WarpedDoor => 639,
            Item::FireCoralFan => 540,
            Item::Lectern => 598,
            Item::Shield => 1009,
            Item::Comparator => 589,
            Item::DeepslateTileWall => 345,
            Item::PolishedGraniteStairs => 549,
            Item::DarkOakLog => 106,
            Item::SmoothSandstoneSlab => 573,
            Item::Glowstone => 275,
            Item::RedstoneTorch => 586,
            Item::Scute => 679,
            Item::GraniteSlab => 575,
            Item::Mutton => 980,
            Item::BatSpawnEgg => 874,
            Item::MagentaStainedGlassPane => 418,
            Item::BlackWool => 172,
            Item::Pufferfish => 804,
            Item::GlowInkSac => 808,
            Item::TubeCoralFan => 537,
            Item::DaylightDetector => 602,
            Item::ChestMinecart => 663,
            Item::ParrotSpawnEgg => 904,
            Item::ItemFrame => 944,
            Item::SugarCane => 196,
            Item::RedMushroom => 188,
            Item::LightBlueCarpet => 376,
            Item::PetrifiedOakSlab => 216,
            Item::CopperIngot => 694,
            Item::StonePickaxe => 706,
            Item::ChiseledRedSandstone => 440,
            Item::PrismarineBrickSlab => 226,
            Item::StrippedOakWood => 117,
            Item::PolishedDeepslateStairs => 564,
            Item::AxolotlBucket => 787,
            Item::SmoothSandstone => 230,
            Item::DioriteStairs => 562,
            Item::JungleFence => 260,
            Item::CutCopperStairs => 77,
            Item::StrippedOakLog => 109,
            Item::Charcoal => 685,
            Item::IronHelmet => 746,
            Item::PolishedDioriteSlab => 570,
            Item::MelonSlice => 849,
            Item::WaxedExposedCutCopper => 90,
            Item::CutRedSandstoneSlab => 223,
            Item::Apple => 681,
            Item::WaxedOxidizedCutCopperSlab => 100,
            Item::CoalBlock => 59,
            Item::MossBlock => 199,
            Item::BrainCoralFan => 538,
            Item::ExposedCutCopper => 74,
            Item::AcaciaBoat => 674,
            Item::TippedArrow => 1007,
            Item::BrickWall => 327,
            Item::SandstoneStairs => 315,
            Item::NetheriteScrap => 698,
            Item::SmoothQuartzStairs => 557,
            Item::OakSapling => 30,
            Item::Ice => 252,
            Item::AcaciaButton => 615,
            Item::YellowConcretePowder => 504,
            Item::GrayConcretePowder => 507,
            Item::Basalt => 271,
            Item::AncientDebris => 58,
            Item::EndStoneBrickWall => 337,
            Item::PinkConcrete => 490,
            Item::BlackConcretePowder => 515,
            Item::DeepslateBrickWall => 344,
            Item::LightGrayTerracotta => 362,
            Item::Cake => 829,
            Item::MusicDiscWard => 1024,
            Item::StrippedJungleWood => 120,
            Item::RedTulip => 178,
            Item::Peony => 397,
            Item::Emerald => 687,
            Item::PinkTulip => 181,
            Item::CreeperSpawnEgg => 882,
            Item::Egg => 794,
            Item::StoneBrickSlab => 219,
            Item::Spawner => 243,
            Item::BlueStainedGlassPane => 427,
            Item::MagentaShulkerBox => 454,
            Item::Diamond => 686,
            Item::CrimsonFence => 263,
            Item::JungleLeaves => 136,
            Item::StrippedWarpedHyphae => 124,
            Item::WaxedCutCopperSlab => 97,
            Item::PolishedGranite => 3,
            Item::Snow => 251,
            Item::BlueConcrete => 495,
            Item::OakButton => 611,
            Item::AndesiteStairs => 559,
            Item::OrangeGlazedTerracotta => 469,
            Item::Compass => 795,
            Item::MagmaCream => 868,
            Item::RedSand => 38,
            Item::NetheriteBoots => 761,
            Item::Farmland => 247,
            Item::AcaciaStairs => 391,
            Item::SmoothQuartzSlab => 574,
            Item::WarpedPlanks => 29,
            Item::FloweringAzalea => 153,
            Item::LeatherHorseArmor => 976,
            Item::DarkOakSapling => 35,
            Item::YellowGlazedTerracotta => 472,
            Item::BirchSlab => 206,
            Item::GoldenShovel => 710,
            Item::CarvedPumpkin => 266,
            Item::JungleTrapdoor => 644,
            Item::WoodenHoe => 703,
            Item::CobbledDeepslateWall => 342,
            Item::PumpkinSeeds => 851,
            Item::InfestedMossyStoneBricks => 279,
            Item::PiglinSpawnEgg => 907,
            Item::CyanBanner => 991,
            Item::CookedBeef => 854,
            Item::DiamondBlock => 68,
            Item::StructureVoid => 450,
            Item::WarpedNylium => 20,
            Item::DarkPrismarine => 434,
            Item::WaxedOxidizedCopper => 88,
            Item::JunglePressurePlate => 626,
            Item::NetheriteLeggings => 760,
            Item::StraySpawnEgg => 923,
            Item::DarkOakSign => 773,
            Item::Carrot => 947,
            Item::Lantern => 1052,
            Item::DiamondHelmet => 750,
            Item::Brick => 788,
            Item::PolishedBlackstoneBrickStairs => 1076,
            Item::Granite => 2,
            Item::GrayGlazedTerracotta => 475,
            Item::CyanStainedGlass => 409,
            Item::LightGrayShulkerBox => 460,
            Item::Sand => 37,
            Item::RedConcrete => 498,
            Item::ZombieHorseSpawnEgg => 937,
            Item::StrippedAcaciaWood => 121,
            Item::GrayTerracotta => 361,
            Item::DarkPrismarineStairs => 437,
            Item::SpectralArrow => 1006,
            Item::Seagrass => 155,
            Item::Anvil => 346,
            Item::DeadHornCoral => 535,
            Item::DiamondSword => 719,
            Item::BlueConcretePowder => 511,
            Item::MossyStoneBricks => 284,
            Item::MilkBucket => 782,
            Item::DeadBubbleCoralBlock => 519,
            Item::ExposedCutCopperStairs => 78,
            Item::DeepslateRedstoneOre => 49,
            Item::YellowTerracotta => 358,
            Item::BirchSign => 770,
            Item::Deepslate => 8,
            Item::RawIronBlock => 60,
            Item::CrimsonPressurePlate => 629,
            Item::AcaciaDoor => 636,
            Item::MagentaConcretePowder => 502,
            Item::ZombieSpawnEgg => 936,
            Item::WitherSkeletonSkull => 954,
            Item::BlackShulkerBox => 467,
            Item::RedMushroomBlock => 293,
            Item::SuspiciousStew => 1034,
            Item::YellowShulkerBox => 456,
            Item::LightWeightedPressurePlate => 621,
            Item::ChainmailLeggings => 744,
            Item::CrimsonFenceGate => 655,
            Item::GrayConcrete => 491,
            Item::DeepslateBrickStairs => 565,
            Item::PurpleStainedGlassPane => 426,
            Item::DarkOakWood => 130,
            Item::Painting => 765,
            Item::SmoothRedSandstoneSlab => 568,
            Item::CookedSalmon => 806,
            Item::Map => 951,
            Item::Dandelion => 173,
            Item::ShulkerShell => 1011,
            Item::MusicDiscWait => 1026,
            Item::LightGrayDye => 818,
            Item::GhastTear => 860,
            Item::ChainmailHelmet => 742,
            Item::Bread => 737,
            Item::PoisonousPotato => 950,
            Item::IronShovel => 715,
            Item::EndermiteSpawnEgg => 888,
            Item::LightGrayGlazedTerracotta => 476,
            Item::NetherBrickWall => 333,
            Item::GrayDye => 817,
            Item::Elytra => 669,
            Item::Snowball => 780,
            Item::Melon => 298,
            Item::ZoglinSpawnEgg => 935,
            Item::BrownWool => 169,
            Item::WitherRose => 185,
            Item::TallGrass => 398,
            Item::Rabbit => 967,
            Item::PolishedBlackstoneButton => 610,
            Item::WeatheredCutCopper => 75,
            Item::EndRod => 237,
            Item::Calcite => 11,
            Item::BirchButton => 613,
            Item::ShulkerSpawnEgg => 916,
            Item::DarkPrismarineSlab => 227,
            Item::TropicalFishSpawnEgg => 926,
            Item::DiamondShovel => 720,
            Item::HorseSpawnEgg => 896,
            Item::GreenWool => 170,
            Item::OrangeTulip => 179,
            Item::CobblestoneStairs => 250,
            Item::LimeStainedGlass => 405,
            Item::BrownConcrete => 496,
            Item::BrownGlazedTerracotta => 480,
            Item::TraderLlamaSpawnEgg => 925,
            Item::PurpleGlazedTerracotta => 478,
            Item::RedNetherBrickStairs => 560,
            Item::BirchPressurePlate => 625,
            Item::WarpedFenceGate => 656,
            Item::DiamondPickaxe => 721,
            Item::MossyStoneBrickWall => 330,
            Item::CookedChicken => 856,
            Item::MagmaCubeSpawnEgg => 899,
            Item::NetherBrick => 964,
            Item::LightningRod => 601,
            Item::BlastFurnace => 1045,
            Item::PurpleDye => 820,
            Item::ChorusFruit => 999,
            Item::DioriteWall => 338,
            Item::TotemOfUndying => 1010,
            Item::PurpurSlab => 224,
            Item::BrownConcretePowder => 512,
            Item::BlueCandle => 1091,
            Item::WarpedSlab => 211,
            Item::PinkConcretePowder => 506,
            Item::GoldenHoe => 713,
            Item::SlimeBall => 793,
            Item::PurpleConcretePowder => 510,
            Item::Jigsaw => 677,
            Item::CookedCod => 805,
            Item::AndesiteWall => 334,
            Item::PolishedDeepslateWall => 343,
            Item::Grass => 150,
            Item::AcaciaWood => 129,
            Item::RedWool => 171,
            Item::SalmonBucket => 784,
            Item::NetherStar => 959,
            Item::Sandstone => 146,
            Item::GhastSpawnEgg => 891,
            Item::StoneSlab => 212,
            Item::QuartzBlock => 350,
            Item::Furnace => 248,
            Item::Clock => 798,
            Item::DragonEgg => 314,
            Item::Scaffolding => 584,
            Item::PiglinBannerPattern => 1041,
            Item::HoneyBottle => 1062,
            Item::PurpleShulkerBox => 462,
            Item::StonePressurePlate => 619,
            Item::OakStairs => 244,
            Item::SpruceTrapdoor => 642,
            Item::PolishedBlackstoneSlab => 1071,
            Item::OrangeWool => 158,
            Item::SoulSoil => 270,
            Item::DeepslateIronOre => 43,
            Item::Barrier => 370,
            Item::LlamaSpawnEgg => 898,
            Item::HuskSpawnEgg => 897,
            Item::CommandBlockMinecart => 979,
            Item::LightBlueConcrete => 487,
            Item::CocoaBeans => 809,
            Item::WrittenBook => 943,
            Item::HoneycombBlock => 1063,
            Item::BubbleCoralBlock => 524,
            Item::DeadFireCoral => 534,
            Item::LimeConcretePowder => 505,
            Item::WaxedCutCopperStairs => 93,
            Item::RedstoneBlock => 587,
            Item::WarpedPressurePlate => 630,
            Item::RedSandstoneStairs => 442,
            Item::StoneBrickWall => 332,
            Item::LightGrayBed => 838,
            Item::MossyCobblestone => 234,
            Item::FoxSpawnEgg => 890,
            Item::AcaciaFenceGate => 653,
            Item::WarpedSign => 775,
            Item::Bundle => 796,
            Item::SmoothRedSandstone => 229,
            Item::ChiseledNetherBricks => 307,
            Item::OxidizedCutCopper => 76,
            Item::Bookshelf => 233,
            Item::NetheriteSword => 724,
            Item::AcaciaSign => 772,
            Item::Arrow => 683,
            Item::OrangeBed => 831,
            Item::SilverfishSpawnEgg => 917,
            Item::SoulLantern => 1053,
            Item::PinkCandle => 1086,
            Item::DriedKelp => 850,
            Item::SpruceSign => 769,
            Item::PurpleCarpet => 383,
            Item::EndStoneBrickSlab => 572,
            Item::BirchSapling => 32,
            Item::Sunflower => 394,
            Item::Book => 792,
            Item::CrimsonFungus => 189,
            Item::RabbitHide => 971,
            Item::EndStoneBrickStairs => 554,
            Item::StoneStairs => 555,
            Item::Piston => 590,
            Item::WhiteWool => 157,
            Item::SnowBlock => 253,
            Item::AzureBluet => 177,
            Item::ExposedCopper => 70,
            Item::CutCopper => 73,
            Item::GreenCandle => 1093,
            Item::ChainCommandBlock => 444,
            Item::YellowStainedGlass => 404,
            Item::GreenTerracotta => 367,
            Item::LapisLazuli => 688,
            Item::NetheriteIngot => 697,
            Item::PufferfishSpawnEgg => 911,
            Item::RedTerracotta => 368,
            Item::InfestedDeepslate => 282,
            Item::PackedIce => 390,
            Item::InfestedCobblestone => 277,
            Item::AcaciaPlanks => 26,
            Item::WoodenPickaxe => 701,
            Item::StrippedAcaciaLog => 113,
            Item::HeavyWeightedPressurePlate => 622,
            Item::MossyCobblestoneStairs => 553,
            Item::Bamboo => 203,
            Item::FloweringAzaleaLeaves => 140,
            Item::CrimsonSign => 774,
            Item::MusicDiscChirp => 1018,
            Item::IronLeggings => 748,
            Item::LightBlueGlazedTerracotta => 471,
            Item::BlackStainedGlassPane => 431,
            Item::PinkBed => 836,
            Item::NautilusShell => 1031,
            Item::EnchantingTable => 310,
            Item::PurpurBlock => 240,
            Item::PlayerHead => 955,
            Item::WhiteTulip => 180,
            Item::IronBars => 295,
            Item::StoneAxe => 707,
            Item::EnchantedGoldenApple => 767,
            Item::DeadBubbleCoralFan => 544,
            Item::BlackCarpet => 388,
            Item::WaxedWeatheredCutCopperSlab => 99,
            Item::BlueDye => 821,
            Item::SmoothBasalt => 273,
            Item::CoarseDirt => 16,
            Item::Repeater => 588,
            Item::TwistingVines => 195,
            Item::PinkCarpet => 379,
            Item::RedSandstone => 439,
            Item::OakWood => 125,
            Item::SpruceSapling => 31,
            Item::CrimsonNylium => 19,
            Item::JungleBoat => 673,
            Item::WheatSeeds => 735,
            Item::CartographyTable => 1046,
            Item::DetectorRail => 658,
            Item::AmethystShard => 690,
            Item::IronHoe => 718,
            Item::JunglePlanks => 25,
            Item::RedSandstoneSlab => 222,
            Item::YellowCarpet => 377,
            Item::NetherWartBlock => 446,
            Item::Jukebox => 256,
            Item::WoodenShovel => 700,
            Item::CookedPorkchop => 764,
            Item::JungleWood => 128,
            Item::NetherBricks => 305,
            Item::SpiderEye => 865,
            Item::SmallAmethystBud => 1096,
            Item::Potato => 948,
            Item::Barrel => 1043,
            Item::Cactus => 254,
            Item::WarpedHyphae => 132,
            Item::ZombieVillagerSpawnEgg => 938,
            Item::Flint => 762,
            Item::WaxedWeatheredCopper => 87,
            Item::IronChestplate => 747,
            Item::LeatherHelmet => 738,
            Item::LightBlueBanner => 985,
            Item::SpruceFenceGate => 650,
            Item::DeepslateTileStairs => 566,
            Item::FireworkStar => 962,
            Item::TubeCoral => 527,
            Item::ChippedAnvil => 347,
            Item::StrippedDarkOakWood => 122,
            Item::PurpurStairs => 242,
            Item::BlueCarpet => 384,
            Item::CyanStainedGlassPane => 425,
            Item::PinkShulkerBox => 458,
            Item::EnderPearl => 858,
            Item::BlazeRod => 859,
            Item::DeadTubeCoralBlock => 517,
            Item::Tuff => 12,
            Item::IronHorseArmor => 973,
            Item::PinkGlazedTerracotta => 474,
            Item::ActivatorRail => 660,
            Item::WhiteGlazedTerracotta => 468,
            Item::BigDripleaf => 201,
            Item::MusicDiscStal => 1022,
            Item::RedSandstoneWall => 329,
            Item::EmeraldOre => 50,
            Item::GoldenSword => 709,
            Item::LeatherBoots => 741,
            Item::AcaciaSapling => 34,
            Item::EmeraldBlock => 317,
            Item::FireCoral => 530,
            Item::PinkDye => 816,
            Item::BrownBed => 842,
            Item::PrismarineShard => 965,
            Item::DebugStick => 1014,
            Item::DiamondHoe => 723,
            Item::CyanConcrete => 493,
            Item::WhiteTerracotta => 354,
            Item::Glass => 143,
            Item::PinkBanner => 988,
            Item::Blackstone => 1066,
            Item::MusicDiscOtherside => 1027,
            Item::CobbledDeepslate => 9,
            Item::RabbitStew => 969,
            Item::LightGrayBanner => 990,
            Item::StrippedDarkOakLog => 114,
            Item::PrismarineWall => 328,
            Item::SoulTorch => 274,
            Item::Lever => 600,
            Item::BlueBanner => 993,
            Item::PurpurPillar => 241,
            Item::PiglinBruteSpawnEgg => 908,
            Item::CrimsonHyphae => 131,
            Item::DrownedSpawnEgg => 885,
            Item::CyanCarpet => 382,
            Item::NetherWart => 862,
            Item::WhiteShulkerBox => 452,
            Item::Cookie => 846,
            Item::BlazeSpawnEgg => 876,
            Item::GreenCarpet => 386,
            Item::OakTrapdoor => 641,
            Item::GlobeBannerPattern => 1040,
            Item::DeadBush => 154,
            Item::PinkStainedGlass => 406,
            Item::RawIron => 691,
            Item::BlueOrchid => 175,
            Item::Saddle => 661,
            Item::WoodenSword => 699,
            Item::MusicDiscFar => 1019,
            Item::WanderingTraderSpawnEgg => 931,
            Item::PurpleWool => 167,
            Item::LimeCandle => 1085,
            Item::DeepslateLapisOre => 53,
            Item::AcaciaPressurePlate => 627,
            Item::CyanBed => 839,
            Item::BrainCoral => 528,
            Item::QuartzBricks => 351,
            Item::MagentaTerracotta => 356,
            Item::PowderSnowBucket => 779,
            Item::GlisteringMelonSlice => 872,
            Item::PurpleBanner => 992,
            Item::GrayShulkerBox => 459,
            Item::Allium => 176,
            Item::BlackstoneWall => 339,
            Item::IronBoots => 749,
            Item::PoppedChorusFruit => 1000,
            Item::MagentaStainedGlass => 402,
            Item::Smoker => 1044,
            Item::PandaSpawnEgg => 903,
            Item::InfestedChiseledStoneBricks => 281,
            Item::CobbledDeepslateStairs => 563,
            Item::PolishedBlackstoneBrickWall => 341,
            Item::NetheritePickaxe => 726,
            Item::LightGrayConcretePowder => 508,
            Item::CaveSpiderSpawnEgg => 878,
            Item::PolishedBlackstone => 1070,
            Item::LimeConcrete => 489,
            Item::Cobblestone => 21,
            Item::CoalOre => 40,
            Item::Stone => 1,
            Item::BirchWood => 127,
            Item::PinkStainedGlassPane => 422,
            Item::RedConcretePowder => 514,
            Item::Stick => 729,
            Item::BlackConcrete => 499,
            Item::RootedDirt => 18,
            Item::CyanConcretePowder => 509,
            Item::CodSpawnEgg => 880,
            Item::SalmonSpawnEgg => 914,
            Item::EndPortalFrame => 311,
            Item::SmithingTable => 1049,
            Item::SpruceFence => 258,
            Item::GrayCarpet => 380,
            Item::CarrotOnAStick => 667,
            Item::BubbleCoralFan => 539,
            Item::PolishedAndesiteStairs => 561,
            Item::Quartz => 689,
            Item::StoneBrickStairs => 302,
            Item::AndesiteSlab => 576,
            Item::WhiteDye => 810,
            Item::YellowBed => 834,
            Item::CryingObsidian => 1065,
            Item::MelonSeeds => 852,
            Item::Target => 599,
            Item::RawCopperBlock => 61,
            Item::GoldenApple => 766,
            Item::SmoothStone => 231,
            Item::OakFence => 257,
            Item::OakLog => 101,
            Item::VillagerSpawnEgg => 929,
            Item::MusicDiscStrad => 1023,
            Item::SpruceLeaves => 134,
            Item::WhiteConcretePowder => 500,
            Item::BrownShulkerBox => 464,
            Item::JungleFenceGate => 652,
            Item::GreenBed => 843,
            Item::RavagerSpawnEgg => 913,
            Item::Fern => 151,
            Item::WarpedFungus => 190,
            Item::GlowLichen => 300,
            Item::GreenGlazedTerracotta => 481,
            Item::OrangeConcretePowder => 501,
            Item::SpruceBoat => 671,
            Item::Trident => 1029,
            Item::AcaciaFence => 261,
            Item::CrackedNetherBricks => 306,
            Item::FilledMap => 847,
            Item::PolishedGraniteSlab => 567,
            Item::Lead => 977,
            Item::EndStoneBricks => 313,
            Item::CrackedDeepslateBricks => 288,
            Item::BlackBed => 845,
            Item::BubbleCoral => 529,
            Item::FletchingTable => 1047,
            Item::DeadBrainCoralBlock => 518,
            Item::SkullBannerPattern => 1038,
            Item::Paper => 791,
            Item::GlowBerries => 1055,
            Item::WaxedOxidizedCutCopper => 92,
            Item::LightBlueConcretePowder => 503,
            Item::BeetrootSoup => 1003,
            Item::QuartzStairs => 353,
            Item::ChainmailChestplate => 743,
            Item::PolishedDeepslate => 10,
            Item::OxidizedCutCopperSlab => 84,
            Item::LimeStainedGlassPane => 421,
            Item::MusicDiscCat => 1016,
            Item::DarkOakFenceGate => 654,
            Item::DeepslateTileSlab => 583,
            Item::WoodenAxe => 702,
            Item::DiamondBoots => 753,
            Item::LightGrayCandle => 1088,
            Item::WetSponge => 142,
            Item::GrayCandle => 1087,
            Item::LargeAmethystBud => 1098,
            Item::AcaciaSlab => 208,
            Item::Composter => 1042,
            Item::LightGrayConcrete => 492,
            Item::MuleSpawnEgg => 901,
            Item::DiamondAxe => 722,
            Item::LeatherChestplate => 739,
            Item::YellowCandle => 1084,
            Item::WhiteCarpet => 373,
            Item::NetheriteShovel => 725,
            Item::OakDoor => 632,
            Item::LimeBed => 835,
            Item::LilyOfTheValley => 184,
            Item::TintedGlass => 144,
            Item::Rail => 659,
            Item::LimeShulkerBox => 457,
            Item::LilyPad => 304,
            Item::MusicDiscMall => 1020,
            Item::RedGlazedTerracotta => 482,
            Item::CrimsonPlanks => 28,
            Item::Dropper => 597,
            Item::Cauldron => 870,
            Item::MojangBannerPattern => 1039,
            Item::CyanWool => 166,
            Item::Coal => 684,
            Item::IronOre => 42,
            Item::WaxedCutCopper => 89,
            Item::LargeFern => 399,
            Item::RedNetherBrickWall => 335,
            Item::WarpedTrapdoor => 648,
            Item::SmoothQuartz => 228,
            Item::RedNetherBricks => 448,
            Item::Dirt => 15,
            Item::DeepslateDiamondOre => 55,
            Item::DeadFireCoralFan => 545,
            Item::WaxedExposedCopper => 86,
            Item::DarkOakPressurePlate => 628,
            Item::BlueWool => 168,
            Item::BeeSpawnEgg => 875,
            Item::SpiderSpawnEgg => 921,
            Item::RawGold => 695,
            Item::CrimsonStem => 107,
            Item::TripwireHook => 604,
            Item::ArmorStand => 972,
            Item::RedStainedGlassPane => 430,
            Item::GreenBanner => 995,
            Item::RedStainedGlass => 414,
            Item::TurtleHelmet => 678,
            Item::SpruceWood => 126,
            Item::CutRedSandstone => 441,
            Item::DiamondOre => 54,
            Item::LightBlueWool => 160,
            Item::BrownStainedGlass => 412,
            Item::GrayBanner => 989,
            Item::HeartOfTheSea => 1032,
            Item::ChorusFlower => 239,
            Item::Sugar => 828,
            Item::GrassBlock => 14,
            Item::BirchBoat => 672,
            Item::CookedRabbit => 968,
            Item::StrippedSpruceWood => 118,
            Item::DeepslateBrickSlab => 582,
            Item::EndermanSpawnEgg => 887,
            Item::MagentaGlazedTerracotta => 470,
            Item::FurnaceMinecart => 664,
            Item::ZombifiedPiglinSpawnEgg => 939,
            Item::SweetBerries => 1054,
            Item::String => 732,
            Item::GoldenLeggings => 756,
            Item::MagentaBanner => 984,
            Item::OxidizedCutCopperStairs => 80,
            Item::GoldIngot => 696,
            Item::RottenFlesh => 857,
            Item::Bedrock => 36,
            Item::AcaciaTrapdoor => 645,
            Item::EnderChest => 316,
            Item::PurpleStainedGlass => 410,
            Item::DonkeySpawnEgg => 884,
            Item::Stonecutter => 1050,
            Item::CutSandstone => 148,
            Item::GlassBottle => 864,
            Item::SprucePlanks => 23,
            Item::GreenConcretePowder => 513,
            Item::DiamondLeggings => 752,
            Item::BlackCandle => 1095,
            Item::OrangeStainedGlass => 401,
            Item::DragonBreath => 1004,
            Item::Shears => 848,
            Item::StrippedBirchLog => 111,
            Item::Prismarine => 432,
            Item::BlackStainedGlass => 415,
            Item::WaxedWeatheredCutCopper => 91,
            Item::CrimsonDoor => 638,
            Item::BlackstoneStairs => 1068,
            Item::PumpkinPie => 960,
            Item::IronDoor => 631,
            Item::PhantomSpawnEgg => 905,
            Item::DiamondHorseArmor => 975,
            Item::Kelp => 197,
            Item::LeatherLeggings => 740,
            Item::VexSpawnEgg => 928,
            Item::LightBlueShulkerBox => 455,
            Item::StickyPiston => 591,
            Item::StrippedBirchWood => 119,
            Item::WhiteConcrete => 484,
            Item::MossyStoneBrickStairs => 551,
            Item::CopperBlock => 66,
            Item::Ladder => 249,
            Item::CrimsonRoots => 191,
            Item::FireCoralBlock => 525,
            Item::Feather => 733,
            Item::NetheriteChestplate => 759,
            Item::OcelotSpawnEgg => 902,
            Item::NetherBrickFence => 308,
            Item::LightBlueTerracotta => 357,
            Item::SporeBlossom => 186,
            Item::SmoothStoneSlab => 213,
            Item::KnowledgeBook => 1013,
            Item::Beehive => 1061,
            Item::LapisOre => 52,
            Item::JungleLog => 104,
            Item::GoatSpawnEgg => 893,
            Item::RedCandle => 1094,
            Item::ChiseledDeepslate => 291,
            Item::LightGrayCarpet => 381,
            Item::DolphinSpawnEgg => 883,
            Item::DioriteSlab => 579,
            Item::Sponge => 141,
            Item::ExperienceBottle => 940,
            Item::Tnt => 606,
            Item::Dispenser => 596,
            Item::Netherrack => 268,
            Item::PolishedBasalt => 272,
            Item::GoldOre => 46,
            Item::CyanCandle => 1089,
            Item::LimeCarpet => 378,
            Item::GlassPane => 297,
            Item::PillagerSpawnEgg => 909,
            Item::Bucket => 776,
            Item::BlackDye => 825,
            Item::JungleStairs => 320,
            Item::WhiteBanner => 982,
            Item::BrownBanner => 994,
            Item::PolarBearSpawnEgg => 910,
            Item::ChickenSpawnEgg => 879,
            Item::BirchDoor => 634,
            Item::DeadBrainCoralFan => 543,
            Item::StriderSpawnEgg => 924,
            Item::MagmaBlock => 445,
            Item::DarkOakFence => 262,
            Item::DarkOakStairs => 392,
            Item::MossyStoneBrickSlab => 569,
            Item::CutCopperSlab => 81,
            Item::BrainCoralBlock => 523,
            Item::TropicalFishBucket => 786,
            Item::PrismarineStairs => 435,
            Item::CrackedDeepslateTiles => 290,
            Item::DripstoneBlock => 13,
            Item::GoldenBoots => 757,
            Item::BrownCandle => 1092,
            Item::TurtleSpawnEgg => 927,
            Item::SmoothRedSandstoneStairs => 550,
            Item::SheepSpawnEgg => 915,
            Item::GreenConcrete => 497,
            Item::Gravel => 39,
        }
    }
    #[doc = "Gets a `Item` by its `id`."]
    #[inline]
    pub fn from_id(id: u32) -> Option<Self> {
        match id {
            246 => Some(Item::CraftingTable),
            360 => Some(Item::PinkTerracotta),
            259 => Some(Item::BirchFence),
            355 => Some(Item::OrangeTerracotta),
            799 => Some(Item::Spyglass),
            1008 => Some(Item::LingeringPotion),
            997 => Some(Item::BlackBanner),
            411 => Some(Item::BlueStainedGlass),
            708 => Some(Item::StoneHoe),
            363 => Some(Item::CyanTerracotta),
            194 => Some(Item::WeepingVines),
            165 => Some(Item::LightGrayWool),
            1033 => Some(Item::Crossbow),
            624 => Some(Item::SprucePressurePlate),
            704 => Some(Item::StoneSword),
            267 => Some(Item::JackOLantern),
            881 => Some(Item::CowSpawnEgg),
            531 => Some(Item::HornCoral),
            375 => Some(Item::MagentaCarpet),
            1090 => Some(Item::PurpleCandle),
            318 => Some(Item::SpruceStairs),
            110 => Some(Item::StrippedSpruceLog),
            758 => Some(Item::NetheriteHelmet),
            871 => Some(Item::EnderEye),
            1001 => Some(Item::Beetroot),
            365 => Some(Item::BlueTerracotta),
            96 => Some(Item::WaxedOxidizedCutCopperStairs),
            637 => Some(Item::DarkOakDoor),
            571 => Some(Item::MossyCobblestoneSlab),
            461 => Some(Item::CyanShulkerBox),
            594 => Some(Item::Observer),
            65 => Some(Item::IronBlock),
            819 => Some(Item::CyanDye),
            1048 => Some(Item::Grindstone),
            1069 => Some(Item::GildedBlackstone),
            69 => Some(Item::NetheriteBlock),
            526 => Some(Item::HornCoralBlock),
            1058 => Some(Item::Shroomlight),
            44 => Some(Item::CopperOre),
            325 => Some(Item::CobblestoneWall),
            1056 => Some(Item::Campfire),
            340 => Some(Item::PolishedBlackstoneWall),
            182 => Some(Item::OxeyeDaisy),
            649 => Some(Item::OakFenceGate),
            783 => Some(Item::PufferfishBucket),
            420 => Some(Item::YellowStainedGlassPane),
            438 => Some(Item::SeaLantern),
            322 => Some(Item::WarpedStairs),
            473 => Some(Item::LimeGlazedTerracotta),
            970 => Some(Item::RabbitFoot),
            712 => Some(Item::GoldenAxe),
            6 => Some(Item::Andesite),
            892 => Some(Item::GlowSquidSpawnEgg),
            608 => Some(Item::NoteBlock),
            453 => Some(Item::OrangeShulkerBox),
            646 => Some(Item::DarkOakTrapdoor),
            790 => Some(Item::DriedKelpBlock),
            521 => Some(Item::DeadHornCoralBlock),
            210 => Some(Item::CrimsonSlab),
            429 => Some(Item::GreenStainedGlassPane),
            366 => Some(Item::BrownTerracotta),
            232 => Some(Item::Bricks),
            714 => Some(Item::IronSword),
            1036 => Some(Item::FlowerBannerPattern),
            1051 => Some(Item::Bell),
            1099 => Some(Item::AmethystCluster),
            48 => Some(Item::RedstoneOre),
            5 => Some(Item::PolishedDiorite),
            285 => Some(Item::CrackedStoneBricks),
            731 => Some(Item::MushroomStew),
            1083 => Some(Item::LightBlueCandle),
            204 => Some(Item::OakSlab),
            255 => Some(Item::Clay),
            24 => Some(Item::BirchPlanks),
            64 => Some(Item::BuddingAmethyst),
            609 => Some(Item::StoneButton),
            592 => Some(Item::SlimeBlock),
            41 => Some(Item::DeepslateCoalOre),
            215 => Some(Item::CutSandstoneSlab),
            824 => Some(Item::RedDye),
            133 => Some(Item::OakLeaves),
            830 => Some(Item::WhiteBed),
            1028 => Some(Item::MusicDiscPigstep),
            933 => Some(Item::WitherSkeletonSpawnEgg),
            812 => Some(Item::MagentaDye),
            299 => Some(Item::Vine),
            123 => Some(Item::StrippedCrimsonHyphae),
            556 => Some(Item::SmoothSandstoneStairs),
            612 => Some(Item::SpruceButton),
            789 => Some(Item::ClayBall),
            294 => Some(Item::MushroomStem),
            200 => Some(Item::HangingRoots),
            359 => Some(Item::LimeTerracotta),
            869 => Some(Item::BrewingStand),
            1035 => Some(Item::Loom),
            94 => Some(Item::WaxedExposedCutCopperStairs),
            1074 => Some(Item::PolishedBlackstoneBricks),
            428 => Some(Item::BrownStainedGlassPane),
            63 => Some(Item::AmethystBlock),
            958 => Some(Item::DragonHead),
            811 => Some(Item::OrangeDye),
            202 => Some(Item::SmallDripleaf),
            238 => Some(Item::ChorusPlant),
            581 => Some(Item::PolishedDeepslateSlab),
            827 => Some(Item::Bone),
            981 => Some(Item::CookedMutton),
            108 => Some(Item::WarpedStem),
            187 => Some(Item::BrownMushroom),
            265 => Some(Item::Pumpkin),
            85 => Some(Item::WaxedCopperBlock),
            516 => Some(Item::TurtleEgg),
            152 => Some(Item::Azalea),
            833 => Some(Item::LightBlueBed),
            1059 => Some(Item::Honeycomb),
            877 => Some(Item::CatSpawnEgg),
            326 => Some(Item::MossyCobblestoneWall),
            217 => Some(Item::CobblestoneSlab),
            235 => Some(Item::Obsidian),
            407 => Some(Item::GrayStainedGlass),
            578 => Some(Item::PolishedAndesiteSlab),
            647 => Some(Item::CrimsonTrapdoor),
            974 => Some(Item::GoldenHorseArmor),
            633 => Some(Item::SpruceDoor),
            716 => Some(Item::IronPickaxe),
            617 => Some(Item::CrimsonButton),
            651 => Some(Item::BirchFenceGate),
            303 => Some(Item::Mycelium),
            957 => Some(Item::CreeperHead),
            369 => Some(Item::BlackTerracotta),
            895 => Some(Item::HoglinSpawnEgg),
            276 => Some(Item::InfestedStone),
            866 => Some(Item::FermentedSpiderEye),
            605 => Some(Item::TrappedChest),
            930 => Some(Item::VindicatorSpawnEgg),
            693 => Some(Item::RawCopper),
            727 => Some(Item::NetheriteAxe),
            162 => Some(Item::LimeWool),
            163 => Some(Item::PinkWool),
            22 => Some(Item::OakPlanks),
            79 => Some(Item::WeatheredCutCopperStairs),
            736 => Some(Item::Wheat),
            413 => Some(Item::GreenStainedGlass),
            348 => Some(Item::DamagedAnvil),
            1037 => Some(Item::CreeperBannerPattern),
            1072 => Some(Item::PolishedBlackstoneStairs),
            1067 => Some(Item::BlackstoneSlab),
            17 => Some(Item::Podzol),
            269 => Some(Item::SoulSand),
            465 => Some(Item::GreenShulkerBox),
            620 => Some(Item::PolishedBlackstonePressurePlate),
            102 => Some(Item::SpruceLog),
            192 => Some(Item::WarpedRoots),
            996 => Some(Item::RedBanner),
            802 => Some(Item::Salmon),
            1081 => Some(Item::OrangeCandle),
            640 => Some(Item::IronTrapdoor),
            717 => Some(Item::IronAxe),
            1030 => Some(Item::PhantomMembrane),
            987 => Some(Item::LimeBanner),
            840 => Some(Item::PurpleBed),
            730 => Some(Item::Bowl),
            844 => Some(Item::RedBed),
            7 => Some(Item::PolishedAndesite),
            319 => Some(Item::BirchStairs),
            385 => Some(Item::BrownCarpet),
            321 => Some(Item::CrimsonStairs),
            296 => Some(Item::Chain),
            436 => Some(Item::PrismarineBrickStairs),
            45 => Some(Item::DeepslateCopperOre),
            451 => Some(Item::ShulkerBox),
            449 => Some(Item::BoneBlock),
            781 => Some(Item::Leather),
            447 => Some(Item::WarpedWartBlock),
            486 => Some(Item::MagentaConcrete),
            942 => Some(Item::WritableBook),
            349 => Some(Item::ChiseledQuartzBlock),
            147 => Some(Item::ChiseledSandstone),
            374 => Some(Item::OrangeCarpet),
            47 => Some(Item::DeepslateGoldOre),
            607 => Some(Item::RedstoneLamp),
            618 => Some(Item::WarpedButton),
            393 => Some(Item::DirtPath),
            643 => Some(Item::BirchTrapdoor),
            668 => Some(Item::WarpedFungusOnAStick),
            218 => Some(Item::BrickSlab),
            800 => Some(Item::GlowstoneDust),
            986 => Some(Item::YellowBanner),
            1005 => Some(Item::SplashPotion),
            145 => Some(Item::LapisBlock),
            1064 => Some(Item::Lodestone),
            183 => Some(Item::Cornflower),
            1100 => Some(Item::PointedDripstone),
            416 => Some(Item::WhiteStainedGlassPane),
            372 => Some(Item::HayBlock),
            755 => Some(Item::GoldenChestplate),
            103 => Some(Item::BirchLog),
            855 => Some(Item::Chicken),
            115 => Some(Item::StrippedCrimsonStem),
            417 => Some(Item::OrangeStainedGlassPane),
            164 => Some(Item::GrayWool),
            477 => Some(Item::CyanGlazedTerracotta),
            443 => Some(Item::RepeatingCommandBlock),
            547 => Some(Item::BlueIce),
            941 => Some(Item::FireCharge),
            1079 => Some(Item::Candle),
            82 => Some(Item::ExposedCutCopperSlab),
            900 => Some(Item::MooshroomSpawnEgg),
            949 => Some(Item::BakedPotato),
            466 => Some(Item::RedShulkerBox),
            595 => Some(Item::Hopper),
            485 => Some(Item::OrangeConcrete),
            705 => Some(Item::StoneShovel),
            623 => Some(Item::OakPressurePlate),
            312 => Some(Item::EndStone),
            301 => Some(Item::BrickStairs),
            532 => Some(Item::DeadBrainCoral),
            886 => Some(Item::ElderGuardianSpawnEgg),
            371 => Some(Item::Light),
            264 => Some(Item::WarpedFence),
            221 => Some(Item::QuartzSlab),
            734 => Some(Item::Gunpowder),
            682 => Some(Item::Bow),
            822 => Some(Item::BrownDye),
            801 => Some(Item::Cod),
            577 => Some(Item::RedNetherBrickSlab),
            832 => Some(Item::MagentaBed),
            105 => Some(Item::AcaciaLog),
            837 => Some(Item::GrayBed),
            278 => Some(Item::InfestedStoneBricks),
            408 => Some(Item::LightGrayStainedGlass),
            522 => Some(Item::TubeCoralBlock),
            807 => Some(Item::InkSac),
            1025 => Some(Item::MusicDisc11),
            214 => Some(Item::SandstoneSlab),
            1060 => Some(Item::BeeNest),
            403 => Some(Item::LightBlueStainedGlass),
            768 => Some(Item::OakSign),
            1073 => Some(Item::ChiseledPolishedBlackstone),
            225 => Some(Item::PrismarineSlab),
            138 => Some(Item::DarkOakLeaves),
            309 => Some(Item::NetherBrickStairs),
            956 => Some(Item::ZombieHead),
            331 => Some(Item::GraniteWall),
            536 => Some(Item::DeadTubeCoral),
            205 => Some(Item::SpruceSlab),
            552 => Some(Item::PolishedDioriteStairs),
            894 => Some(Item::GuardianSpawnEgg),
            934 => Some(Item::WolfSpawnEgg),
            922 => Some(Item::SquidSpawnEgg),
            364 => Some(Item::PurpleTerracotta),
            803 => Some(Item::TropicalFish),
            961 => Some(Item::FireworkRocket),
            463 => Some(Item::BlueShulkerBox),
            920 => Some(Item::SlimeSpawnEgg),
            209 => Some(Item::DarkOakSlab),
            754 => Some(Item::GoldenHelmet),
            814 => Some(Item::YellowDye),
            283 => Some(Item::StoneBricks),
            546 => Some(Item::DeadHornCoralFan),
            946 => Some(Item::FlowerPot),
            861 => Some(Item::GoldNugget),
            1097 => Some(Item::MediumAmethystBud),
            665 => Some(Item::TntMinecart),
            116 => Some(Item::StrippedWarpedStem),
            245 => Some(Item::Chest),
            603 => Some(Item::SculkSensor),
            1075 => Some(Item::PolishedBlackstoneBrickSlab),
            149 => Some(Item::Cobweb),
            520 => Some(Item::DeadFireCoralBlock),
            867 => Some(Item::BlazePowder),
            614 => Some(Item::JungleButton),
            953 => Some(Item::SkeletonSkull),
            815 => Some(Item::LimeDye),
            906 => Some(Item::PigSpawnEgg),
            4 => Some(Item::Diorite),
            657 => Some(Item::PoweredRail),
            161 => Some(Item::YellowWool),
            912 => Some(Item::RabbitSpawnEgg),
            419 => Some(Item::LightBlueStainedGlassPane),
            778 => Some(Item::LavaBucket),
            137 => Some(Item::AcaciaLeaves),
            324 => Some(Item::Beacon),
            533 => Some(Item::DeadBubbleCoral),
            27 => Some(Item::DarkOakPlanks),
            396 => Some(Item::RoseBush),
            763 => Some(Item::Porkchop),
            666 => Some(Item::HopperMinecart),
            635 => Some(Item::JungleDoor),
            777 => Some(Item::WaterBucket),
            853 => Some(Item::Beef),
            918 => Some(Item::SkeletonSpawnEgg),
            72 => Some(Item::OxidizedCopper),
            1015 => Some(Item::MusicDisc13),
            966 => Some(Item::PrismarineCrystals),
            57 => Some(Item::NetherQuartzOre),
            400 => Some(Item::WhiteStainedGlass),
            873 => Some(Item::AxolotlSpawnEgg),
            395 => Some(Item::Lilac),
            826 => Some(Item::BoneMeal),
            292 => Some(Item::BrownMushroomBlock),
            952 => Some(Item::GoldenCarrot),
            352 => Some(Item::QuartzPillar),
            1012 => Some(Item::IronNugget),
            863 => Some(Item::Potion),
            558 => Some(Item::GraniteStairs),
            494 => Some(Item::PurpleConcrete),
            978 => Some(Item::NameTag),
            289 => Some(Item::DeepslateTiles),
            675 => Some(Item::DarkOakBoat),
            323 => Some(Item::CommandBlock),
            1017 => Some(Item::MusicDiscBlocks),
            676 => Some(Item::StructureBlock),
            548 => Some(Item::Conduit),
            711 => Some(Item::GoldenPickaxe),
            751 => Some(Item::DiamondChestplate),
            207 => Some(Item::JungleSlab),
            488 => Some(Item::YellowConcrete),
            585 => Some(Item::Redstone),
            692 => Some(Item::IronIngot),
            785 => Some(Item::CodBucket),
            797 => Some(Item::FishingRod),
            823 => Some(Item::GreenDye),
            745 => Some(Item::ChainmailBoots),
            662 => Some(Item::Minecart),
            389 => Some(Item::Terracotta),
            56 => Some(Item::NetherGoldOre),
            135 => Some(Item::BirchLeaves),
            424 => Some(Item::LightGrayStainedGlassPane),
            616 => Some(Item::DarkOakButton),
            813 => Some(Item::LightBlueDye),
            1021 => Some(Item::MusicDiscMellohi),
            580 => Some(Item::CobbledDeepslateSlab),
            1077 => Some(Item::CrackedPolishedBlackstoneBricks),
            932 => Some(Item::WitchSpawnEgg),
            542 => Some(Item::DeadTubeCoralFan),
            1080 => Some(Item::WhiteCandle),
            83 => Some(Item::WeatheredCutCopperSlab),
            945 => Some(Item::GlowItemFrame),
            287 => Some(Item::DeepslateBricks),
            1078 => Some(Item::RespawnAnchor),
            112 => Some(Item::StrippedJungleLog),
            156 => Some(Item::SeaPickle),
            51 => Some(Item::DeepslateEmeraldOre),
            387 => Some(Item::RedCarpet),
            433 => Some(Item::PrismarineBricks),
            198 => Some(Item::MossCarpet),
            728 => Some(Item::NetheriteHoe),
            67 => Some(Item::GoldBlock),
            280 => Some(Item::InfestedCrackedStoneBricks),
            62 => Some(Item::RawGoldBlock),
            771 => Some(Item::JungleSign),
            963 => Some(Item::EnchantedBook),
            841 => Some(Item::BlueBed),
            998 => Some(Item::EndCrystal),
            336 => Some(Item::SandstoneWall),
            286 => Some(Item::ChiseledStoneBricks),
            236 => Some(Item::Torch),
            670 => Some(Item::OakBoat),
            1057 => Some(Item::SoulCampfire),
            174 => Some(Item::Poppy),
            479 => Some(Item::BlueGlazedTerracotta),
            1002 => Some(Item::BeetrootSeeds),
            71 => Some(Item::WeatheredCopper),
            98 => Some(Item::WaxedExposedCutCopperSlab),
            220 => Some(Item::NetherBrickSlab),
            193 => Some(Item::NetherSprouts),
            483 => Some(Item::BlackGlazedTerracotta),
            593 => Some(Item::HoneyBlock),
            983 => Some(Item::OrangeBanner),
            541 => Some(Item::HornCoralFan),
            1082 => Some(Item::MagentaCandle),
            139 => Some(Item::AzaleaLeaves),
            680 => Some(Item::FlintAndSteel),
            423 => Some(Item::GrayStainedGlassPane),
            33 => Some(Item::JungleSapling),
            95 => Some(Item::WaxedWeatheredCutCopperStairs),
            889 => Some(Item::EvokerSpawnEgg),
            919 => Some(Item::SkeletonHorseSpawnEgg),
            159 => Some(Item::MagentaWool),
            639 => Some(Item::WarpedDoor),
            540 => Some(Item::FireCoralFan),
            598 => Some(Item::Lectern),
            1009 => Some(Item::Shield),
            589 => Some(Item::Comparator),
            345 => Some(Item::DeepslateTileWall),
            549 => Some(Item::PolishedGraniteStairs),
            106 => Some(Item::DarkOakLog),
            573 => Some(Item::SmoothSandstoneSlab),
            275 => Some(Item::Glowstone),
            586 => Some(Item::RedstoneTorch),
            679 => Some(Item::Scute),
            575 => Some(Item::GraniteSlab),
            980 => Some(Item::Mutton),
            874 => Some(Item::BatSpawnEgg),
            418 => Some(Item::MagentaStainedGlassPane),
            172 => Some(Item::BlackWool),
            804 => Some(Item::Pufferfish),
            808 => Some(Item::GlowInkSac),
            537 => Some(Item::TubeCoralFan),
            602 => Some(Item::DaylightDetector),
            663 => Some(Item::ChestMinecart),
            904 => Some(Item::ParrotSpawnEgg),
            944 => Some(Item::ItemFrame),
            196 => Some(Item::SugarCane),
            188 => Some(Item::RedMushroom),
            376 => Some(Item::LightBlueCarpet),
            216 => Some(Item::PetrifiedOakSlab),
            694 => Some(Item::CopperIngot),
            706 => Some(Item::StonePickaxe),
            440 => Some(Item::ChiseledRedSandstone),
            226 => Some(Item::PrismarineBrickSlab),
            117 => Some(Item::StrippedOakWood),
            564 => Some(Item::PolishedDeepslateStairs),
            787 => Some(Item::AxolotlBucket),
            230 => Some(Item::SmoothSandstone),
            562 => Some(Item::DioriteStairs),
            260 => Some(Item::JungleFence),
            77 => Some(Item::CutCopperStairs),
            109 => Some(Item::StrippedOakLog),
            685 => Some(Item::Charcoal),
            746 => Some(Item::IronHelmet),
            570 => Some(Item::PolishedDioriteSlab),
            849 => Some(Item::MelonSlice),
            90 => Some(Item::WaxedExposedCutCopper),
            223 => Some(Item::CutRedSandstoneSlab),
            681 => Some(Item::Apple),
            100 => Some(Item::WaxedOxidizedCutCopperSlab),
            59 => Some(Item::CoalBlock),
            199 => Some(Item::MossBlock),
            538 => Some(Item::BrainCoralFan),
            74 => Some(Item::ExposedCutCopper),
            674 => Some(Item::AcaciaBoat),
            1007 => Some(Item::TippedArrow),
            327 => Some(Item::BrickWall),
            315 => Some(Item::SandstoneStairs),
            698 => Some(Item::NetheriteScrap),
            557 => Some(Item::SmoothQuartzStairs),
            30 => Some(Item::OakSapling),
            252 => Some(Item::Ice),
            615 => Some(Item::AcaciaButton),
            504 => Some(Item::YellowConcretePowder),
            507 => Some(Item::GrayConcretePowder),
            271 => Some(Item::Basalt),
            58 => Some(Item::AncientDebris),
            337 => Some(Item::EndStoneBrickWall),
            490 => Some(Item::PinkConcrete),
            515 => Some(Item::BlackConcretePowder),
            344 => Some(Item::DeepslateBrickWall),
            362 => Some(Item::LightGrayTerracotta),
            829 => Some(Item::Cake),
            1024 => Some(Item::MusicDiscWard),
            120 => Some(Item::StrippedJungleWood),
            178 => Some(Item::RedTulip),
            397 => Some(Item::Peony),
            687 => Some(Item::Emerald),
            181 => Some(Item::PinkTulip),
            882 => Some(Item::CreeperSpawnEgg),
            794 => Some(Item::Egg),
            219 => Some(Item::StoneBrickSlab),
            243 => Some(Item::Spawner),
            427 => Some(Item::BlueStainedGlassPane),
            454 => Some(Item::MagentaShulkerBox),
            686 => Some(Item::Diamond),
            263 => Some(Item::CrimsonFence),
            136 => Some(Item::JungleLeaves),
            124 => Some(Item::StrippedWarpedHyphae),
            97 => Some(Item::WaxedCutCopperSlab),
            3 => Some(Item::PolishedGranite),
            251 => Some(Item::Snow),
            495 => Some(Item::BlueConcrete),
            611 => Some(Item::OakButton),
            559 => Some(Item::AndesiteStairs),
            469 => Some(Item::OrangeGlazedTerracotta),
            795 => Some(Item::Compass),
            868 => Some(Item::MagmaCream),
            38 => Some(Item::RedSand),
            761 => Some(Item::NetheriteBoots),
            247 => Some(Item::Farmland),
            391 => Some(Item::AcaciaStairs),
            574 => Some(Item::SmoothQuartzSlab),
            29 => Some(Item::WarpedPlanks),
            153 => Some(Item::FloweringAzalea),
            976 => Some(Item::LeatherHorseArmor),
            35 => Some(Item::DarkOakSapling),
            472 => Some(Item::YellowGlazedTerracotta),
            206 => Some(Item::BirchSlab),
            710 => Some(Item::GoldenShovel),
            266 => Some(Item::CarvedPumpkin),
            644 => Some(Item::JungleTrapdoor),
            703 => Some(Item::WoodenHoe),
            342 => Some(Item::CobbledDeepslateWall),
            851 => Some(Item::PumpkinSeeds),
            279 => Some(Item::InfestedMossyStoneBricks),
            907 => Some(Item::PiglinSpawnEgg),
            991 => Some(Item::CyanBanner),
            854 => Some(Item::CookedBeef),
            68 => Some(Item::DiamondBlock),
            450 => Some(Item::StructureVoid),
            20 => Some(Item::WarpedNylium),
            434 => Some(Item::DarkPrismarine),
            88 => Some(Item::WaxedOxidizedCopper),
            626 => Some(Item::JunglePressurePlate),
            760 => Some(Item::NetheriteLeggings),
            923 => Some(Item::StraySpawnEgg),
            773 => Some(Item::DarkOakSign),
            947 => Some(Item::Carrot),
            1052 => Some(Item::Lantern),
            750 => Some(Item::DiamondHelmet),
            788 => Some(Item::Brick),
            1076 => Some(Item::PolishedBlackstoneBrickStairs),
            2 => Some(Item::Granite),
            475 => Some(Item::GrayGlazedTerracotta),
            409 => Some(Item::CyanStainedGlass),
            460 => Some(Item::LightGrayShulkerBox),
            37 => Some(Item::Sand),
            498 => Some(Item::RedConcrete),
            937 => Some(Item::ZombieHorseSpawnEgg),
            121 => Some(Item::StrippedAcaciaWood),
            361 => Some(Item::GrayTerracotta),
            437 => Some(Item::DarkPrismarineStairs),
            1006 => Some(Item::SpectralArrow),
            155 => Some(Item::Seagrass),
            346 => Some(Item::Anvil),
            535 => Some(Item::DeadHornCoral),
            719 => Some(Item::DiamondSword),
            511 => Some(Item::BlueConcretePowder),
            284 => Some(Item::MossyStoneBricks),
            782 => Some(Item::MilkBucket),
            519 => Some(Item::DeadBubbleCoralBlock),
            78 => Some(Item::ExposedCutCopperStairs),
            49 => Some(Item::DeepslateRedstoneOre),
            358 => Some(Item::YellowTerracotta),
            770 => Some(Item::BirchSign),
            8 => Some(Item::Deepslate),
            60 => Some(Item::RawIronBlock),
            629 => Some(Item::CrimsonPressurePlate),
            636 => Some(Item::AcaciaDoor),
            502 => Some(Item::MagentaConcretePowder),
            936 => Some(Item::ZombieSpawnEgg),
            954 => Some(Item::WitherSkeletonSkull),
            467 => Some(Item::BlackShulkerBox),
            293 => Some(Item::RedMushroomBlock),
            1034 => Some(Item::SuspiciousStew),
            456 => Some(Item::YellowShulkerBox),
            621 => Some(Item::LightWeightedPressurePlate),
            744 => Some(Item::ChainmailLeggings),
            655 => Some(Item::CrimsonFenceGate),
            491 => Some(Item::GrayConcrete),
            565 => Some(Item::DeepslateBrickStairs),
            426 => Some(Item::PurpleStainedGlassPane),
            130 => Some(Item::DarkOakWood),
            765 => Some(Item::Painting),
            568 => Some(Item::SmoothRedSandstoneSlab),
            806 => Some(Item::CookedSalmon),
            951 => Some(Item::Map),
            173 => Some(Item::Dandelion),
            1011 => Some(Item::ShulkerShell),
            1026 => Some(Item::MusicDiscWait),
            818 => Some(Item::LightGrayDye),
            860 => Some(Item::GhastTear),
            742 => Some(Item::ChainmailHelmet),
            737 => Some(Item::Bread),
            950 => Some(Item::PoisonousPotato),
            715 => Some(Item::IronShovel),
            888 => Some(Item::EndermiteSpawnEgg),
            476 => Some(Item::LightGrayGlazedTerracotta),
            333 => Some(Item::NetherBrickWall),
            817 => Some(Item::GrayDye),
            669 => Some(Item::Elytra),
            780 => Some(Item::Snowball),
            298 => Some(Item::Melon),
            935 => Some(Item::ZoglinSpawnEgg),
            169 => Some(Item::BrownWool),
            185 => Some(Item::WitherRose),
            398 => Some(Item::TallGrass),
            967 => Some(Item::Rabbit),
            610 => Some(Item::PolishedBlackstoneButton),
            75 => Some(Item::WeatheredCutCopper),
            237 => Some(Item::EndRod),
            11 => Some(Item::Calcite),
            613 => Some(Item::BirchButton),
            916 => Some(Item::ShulkerSpawnEgg),
            227 => Some(Item::DarkPrismarineSlab),
            926 => Some(Item::TropicalFishSpawnEgg),
            720 => Some(Item::DiamondShovel),
            896 => Some(Item::HorseSpawnEgg),
            170 => Some(Item::GreenWool),
            179 => Some(Item::OrangeTulip),
            250 => Some(Item::CobblestoneStairs),
            405 => Some(Item::LimeStainedGlass),
            496 => Some(Item::BrownConcrete),
            480 => Some(Item::BrownGlazedTerracotta),
            925 => Some(Item::TraderLlamaSpawnEgg),
            478 => Some(Item::PurpleGlazedTerracotta),
            560 => Some(Item::RedNetherBrickStairs),
            625 => Some(Item::BirchPressurePlate),
            656 => Some(Item::WarpedFenceGate),
            721 => Some(Item::DiamondPickaxe),
            330 => Some(Item::MossyStoneBrickWall),
            856 => Some(Item::CookedChicken),
            899 => Some(Item::MagmaCubeSpawnEgg),
            964 => Some(Item::NetherBrick),
            601 => Some(Item::LightningRod),
            1045 => Some(Item::BlastFurnace),
            820 => Some(Item::PurpleDye),
            999 => Some(Item::ChorusFruit),
            338 => Some(Item::DioriteWall),
            1010 => Some(Item::TotemOfUndying),
            224 => Some(Item::PurpurSlab),
            512 => Some(Item::BrownConcretePowder),
            1091 => Some(Item::BlueCandle),
            211 => Some(Item::WarpedSlab),
            506 => Some(Item::PinkConcretePowder),
            713 => Some(Item::GoldenHoe),
            793 => Some(Item::SlimeBall),
            510 => Some(Item::PurpleConcretePowder),
            677 => Some(Item::Jigsaw),
            805 => Some(Item::CookedCod),
            334 => Some(Item::AndesiteWall),
            343 => Some(Item::PolishedDeepslateWall),
            150 => Some(Item::Grass),
            129 => Some(Item::AcaciaWood),
            171 => Some(Item::RedWool),
            784 => Some(Item::SalmonBucket),
            959 => Some(Item::NetherStar),
            146 => Some(Item::Sandstone),
            891 => Some(Item::GhastSpawnEgg),
            212 => Some(Item::StoneSlab),
            350 => Some(Item::QuartzBlock),
            248 => Some(Item::Furnace),
            798 => Some(Item::Clock),
            314 => Some(Item::DragonEgg),
            584 => Some(Item::Scaffolding),
            1041 => Some(Item::PiglinBannerPattern),
            1062 => Some(Item::HoneyBottle),
            462 => Some(Item::PurpleShulkerBox),
            619 => Some(Item::StonePressurePlate),
            244 => Some(Item::OakStairs),
            642 => Some(Item::SpruceTrapdoor),
            1071 => Some(Item::PolishedBlackstoneSlab),
            158 => Some(Item::OrangeWool),
            270 => Some(Item::SoulSoil),
            43 => Some(Item::DeepslateIronOre),
            370 => Some(Item::Barrier),
            898 => Some(Item::LlamaSpawnEgg),
            897 => Some(Item::HuskSpawnEgg),
            979 => Some(Item::CommandBlockMinecart),
            487 => Some(Item::LightBlueConcrete),
            809 => Some(Item::CocoaBeans),
            943 => Some(Item::WrittenBook),
            1063 => Some(Item::HoneycombBlock),
            524 => Some(Item::BubbleCoralBlock),
            534 => Some(Item::DeadFireCoral),
            505 => Some(Item::LimeConcretePowder),
            93 => Some(Item::WaxedCutCopperStairs),
            587 => Some(Item::RedstoneBlock),
            630 => Some(Item::WarpedPressurePlate),
            442 => Some(Item::RedSandstoneStairs),
            332 => Some(Item::StoneBrickWall),
            838 => Some(Item::LightGrayBed),
            234 => Some(Item::MossyCobblestone),
            890 => Some(Item::FoxSpawnEgg),
            653 => Some(Item::AcaciaFenceGate),
            775 => Some(Item::WarpedSign),
            796 => Some(Item::Bundle),
            229 => Some(Item::SmoothRedSandstone),
            307 => Some(Item::ChiseledNetherBricks),
            76 => Some(Item::OxidizedCutCopper),
            233 => Some(Item::Bookshelf),
            724 => Some(Item::NetheriteSword),
            772 => Some(Item::AcaciaSign),
            683 => Some(Item::Arrow),
            831 => Some(Item::OrangeBed),
            917 => Some(Item::SilverfishSpawnEgg),
            1053 => Some(Item::SoulLantern),
            1086 => Some(Item::PinkCandle),
            850 => Some(Item::DriedKelp),
            769 => Some(Item::SpruceSign),
            383 => Some(Item::PurpleCarpet),
            572 => Some(Item::EndStoneBrickSlab),
            32 => Some(Item::BirchSapling),
            394 => Some(Item::Sunflower),
            792 => Some(Item::Book),
            189 => Some(Item::CrimsonFungus),
            971 => Some(Item::RabbitHide),
            554 => Some(Item::EndStoneBrickStairs),
            555 => Some(Item::StoneStairs),
            590 => Some(Item::Piston),
            157 => Some(Item::WhiteWool),
            253 => Some(Item::SnowBlock),
            177 => Some(Item::AzureBluet),
            70 => Some(Item::ExposedCopper),
            73 => Some(Item::CutCopper),
            1093 => Some(Item::GreenCandle),
            444 => Some(Item::ChainCommandBlock),
            404 => Some(Item::YellowStainedGlass),
            367 => Some(Item::GreenTerracotta),
            688 => Some(Item::LapisLazuli),
            697 => Some(Item::NetheriteIngot),
            911 => Some(Item::PufferfishSpawnEgg),
            368 => Some(Item::RedTerracotta),
            282 => Some(Item::InfestedDeepslate),
            390 => Some(Item::PackedIce),
            277 => Some(Item::InfestedCobblestone),
            26 => Some(Item::AcaciaPlanks),
            701 => Some(Item::WoodenPickaxe),
            113 => Some(Item::StrippedAcaciaLog),
            622 => Some(Item::HeavyWeightedPressurePlate),
            553 => Some(Item::MossyCobblestoneStairs),
            203 => Some(Item::Bamboo),
            140 => Some(Item::FloweringAzaleaLeaves),
            774 => Some(Item::CrimsonSign),
            1018 => Some(Item::MusicDiscChirp),
            748 => Some(Item::IronLeggings),
            471 => Some(Item::LightBlueGlazedTerracotta),
            431 => Some(Item::BlackStainedGlassPane),
            836 => Some(Item::PinkBed),
            1031 => Some(Item::NautilusShell),
            310 => Some(Item::EnchantingTable),
            240 => Some(Item::PurpurBlock),
            955 => Some(Item::PlayerHead),
            180 => Some(Item::WhiteTulip),
            295 => Some(Item::IronBars),
            707 => Some(Item::StoneAxe),
            767 => Some(Item::EnchantedGoldenApple),
            544 => Some(Item::DeadBubbleCoralFan),
            388 => Some(Item::BlackCarpet),
            99 => Some(Item::WaxedWeatheredCutCopperSlab),
            821 => Some(Item::BlueDye),
            273 => Some(Item::SmoothBasalt),
            16 => Some(Item::CoarseDirt),
            588 => Some(Item::Repeater),
            195 => Some(Item::TwistingVines),
            379 => Some(Item::PinkCarpet),
            439 => Some(Item::RedSandstone),
            125 => Some(Item::OakWood),
            31 => Some(Item::SpruceSapling),
            19 => Some(Item::CrimsonNylium),
            673 => Some(Item::JungleBoat),
            735 => Some(Item::WheatSeeds),
            1046 => Some(Item::CartographyTable),
            658 => Some(Item::DetectorRail),
            690 => Some(Item::AmethystShard),
            718 => Some(Item::IronHoe),
            25 => Some(Item::JunglePlanks),
            222 => Some(Item::RedSandstoneSlab),
            377 => Some(Item::YellowCarpet),
            446 => Some(Item::NetherWartBlock),
            256 => Some(Item::Jukebox),
            700 => Some(Item::WoodenShovel),
            764 => Some(Item::CookedPorkchop),
            128 => Some(Item::JungleWood),
            305 => Some(Item::NetherBricks),
            865 => Some(Item::SpiderEye),
            1096 => Some(Item::SmallAmethystBud),
            948 => Some(Item::Potato),
            1043 => Some(Item::Barrel),
            254 => Some(Item::Cactus),
            132 => Some(Item::WarpedHyphae),
            938 => Some(Item::ZombieVillagerSpawnEgg),
            762 => Some(Item::Flint),
            87 => Some(Item::WaxedWeatheredCopper),
            747 => Some(Item::IronChestplate),
            738 => Some(Item::LeatherHelmet),
            985 => Some(Item::LightBlueBanner),
            650 => Some(Item::SpruceFenceGate),
            566 => Some(Item::DeepslateTileStairs),
            962 => Some(Item::FireworkStar),
            527 => Some(Item::TubeCoral),
            347 => Some(Item::ChippedAnvil),
            122 => Some(Item::StrippedDarkOakWood),
            242 => Some(Item::PurpurStairs),
            384 => Some(Item::BlueCarpet),
            425 => Some(Item::CyanStainedGlassPane),
            458 => Some(Item::PinkShulkerBox),
            858 => Some(Item::EnderPearl),
            859 => Some(Item::BlazeRod),
            517 => Some(Item::DeadTubeCoralBlock),
            12 => Some(Item::Tuff),
            973 => Some(Item::IronHorseArmor),
            474 => Some(Item::PinkGlazedTerracotta),
            660 => Some(Item::ActivatorRail),
            468 => Some(Item::WhiteGlazedTerracotta),
            201 => Some(Item::BigDripleaf),
            1022 => Some(Item::MusicDiscStal),
            329 => Some(Item::RedSandstoneWall),
            50 => Some(Item::EmeraldOre),
            709 => Some(Item::GoldenSword),
            741 => Some(Item::LeatherBoots),
            34 => Some(Item::AcaciaSapling),
            317 => Some(Item::EmeraldBlock),
            530 => Some(Item::FireCoral),
            816 => Some(Item::PinkDye),
            842 => Some(Item::BrownBed),
            965 => Some(Item::PrismarineShard),
            1014 => Some(Item::DebugStick),
            723 => Some(Item::DiamondHoe),
            493 => Some(Item::CyanConcrete),
            354 => Some(Item::WhiteTerracotta),
            143 => Some(Item::Glass),
            988 => Some(Item::PinkBanner),
            1066 => Some(Item::Blackstone),
            1027 => Some(Item::MusicDiscOtherside),
            9 => Some(Item::CobbledDeepslate),
            969 => Some(Item::RabbitStew),
            990 => Some(Item::LightGrayBanner),
            114 => Some(Item::StrippedDarkOakLog),
            328 => Some(Item::PrismarineWall),
            274 => Some(Item::SoulTorch),
            600 => Some(Item::Lever),
            993 => Some(Item::BlueBanner),
            241 => Some(Item::PurpurPillar),
            908 => Some(Item::PiglinBruteSpawnEgg),
            131 => Some(Item::CrimsonHyphae),
            885 => Some(Item::DrownedSpawnEgg),
            382 => Some(Item::CyanCarpet),
            862 => Some(Item::NetherWart),
            452 => Some(Item::WhiteShulkerBox),
            846 => Some(Item::Cookie),
            876 => Some(Item::BlazeSpawnEgg),
            386 => Some(Item::GreenCarpet),
            641 => Some(Item::OakTrapdoor),
            1040 => Some(Item::GlobeBannerPattern),
            154 => Some(Item::DeadBush),
            406 => Some(Item::PinkStainedGlass),
            691 => Some(Item::RawIron),
            175 => Some(Item::BlueOrchid),
            661 => Some(Item::Saddle),
            699 => Some(Item::WoodenSword),
            1019 => Some(Item::MusicDiscFar),
            931 => Some(Item::WanderingTraderSpawnEgg),
            167 => Some(Item::PurpleWool),
            1085 => Some(Item::LimeCandle),
            53 => Some(Item::DeepslateLapisOre),
            627 => Some(Item::AcaciaPressurePlate),
            839 => Some(Item::CyanBed),
            528 => Some(Item::BrainCoral),
            351 => Some(Item::QuartzBricks),
            356 => Some(Item::MagentaTerracotta),
            779 => Some(Item::PowderSnowBucket),
            872 => Some(Item::GlisteringMelonSlice),
            992 => Some(Item::PurpleBanner),
            459 => Some(Item::GrayShulkerBox),
            176 => Some(Item::Allium),
            339 => Some(Item::BlackstoneWall),
            749 => Some(Item::IronBoots),
            1000 => Some(Item::PoppedChorusFruit),
            402 => Some(Item::MagentaStainedGlass),
            1044 => Some(Item::Smoker),
            903 => Some(Item::PandaSpawnEgg),
            281 => Some(Item::InfestedChiseledStoneBricks),
            563 => Some(Item::CobbledDeepslateStairs),
            341 => Some(Item::PolishedBlackstoneBrickWall),
            726 => Some(Item::NetheritePickaxe),
            508 => Some(Item::LightGrayConcretePowder),
            878 => Some(Item::CaveSpiderSpawnEgg),
            1070 => Some(Item::PolishedBlackstone),
            489 => Some(Item::LimeConcrete),
            21 => Some(Item::Cobblestone),
            40 => Some(Item::CoalOre),
            1 => Some(Item::Stone),
            127 => Some(Item::BirchWood),
            422 => Some(Item::PinkStainedGlassPane),
            514 => Some(Item::RedConcretePowder),
            729 => Some(Item::Stick),
            499 => Some(Item::BlackConcrete),
            18 => Some(Item::RootedDirt),
            509 => Some(Item::CyanConcretePowder),
            880 => Some(Item::CodSpawnEgg),
            914 => Some(Item::SalmonSpawnEgg),
            311 => Some(Item::EndPortalFrame),
            1049 => Some(Item::SmithingTable),
            258 => Some(Item::SpruceFence),
            380 => Some(Item::GrayCarpet),
            667 => Some(Item::CarrotOnAStick),
            539 => Some(Item::BubbleCoralFan),
            561 => Some(Item::PolishedAndesiteStairs),
            689 => Some(Item::Quartz),
            302 => Some(Item::StoneBrickStairs),
            576 => Some(Item::AndesiteSlab),
            810 => Some(Item::WhiteDye),
            834 => Some(Item::YellowBed),
            1065 => Some(Item::CryingObsidian),
            852 => Some(Item::MelonSeeds),
            599 => Some(Item::Target),
            61 => Some(Item::RawCopperBlock),
            766 => Some(Item::GoldenApple),
            231 => Some(Item::SmoothStone),
            257 => Some(Item::OakFence),
            101 => Some(Item::OakLog),
            929 => Some(Item::VillagerSpawnEgg),
            1023 => Some(Item::MusicDiscStrad),
            134 => Some(Item::SpruceLeaves),
            500 => Some(Item::WhiteConcretePowder),
            464 => Some(Item::BrownShulkerBox),
            652 => Some(Item::JungleFenceGate),
            843 => Some(Item::GreenBed),
            913 => Some(Item::RavagerSpawnEgg),
            151 => Some(Item::Fern),
            190 => Some(Item::WarpedFungus),
            300 => Some(Item::GlowLichen),
            481 => Some(Item::GreenGlazedTerracotta),
            501 => Some(Item::OrangeConcretePowder),
            671 => Some(Item::SpruceBoat),
            1029 => Some(Item::Trident),
            261 => Some(Item::AcaciaFence),
            306 => Some(Item::CrackedNetherBricks),
            847 => Some(Item::FilledMap),
            567 => Some(Item::PolishedGraniteSlab),
            977 => Some(Item::Lead),
            313 => Some(Item::EndStoneBricks),
            288 => Some(Item::CrackedDeepslateBricks),
            845 => Some(Item::BlackBed),
            529 => Some(Item::BubbleCoral),
            1047 => Some(Item::FletchingTable),
            518 => Some(Item::DeadBrainCoralBlock),
            1038 => Some(Item::SkullBannerPattern),
            791 => Some(Item::Paper),
            1055 => Some(Item::GlowBerries),
            92 => Some(Item::WaxedOxidizedCutCopper),
            503 => Some(Item::LightBlueConcretePowder),
            1003 => Some(Item::BeetrootSoup),
            353 => Some(Item::QuartzStairs),
            743 => Some(Item::ChainmailChestplate),
            10 => Some(Item::PolishedDeepslate),
            84 => Some(Item::OxidizedCutCopperSlab),
            421 => Some(Item::LimeStainedGlassPane),
            1016 => Some(Item::MusicDiscCat),
            654 => Some(Item::DarkOakFenceGate),
            583 => Some(Item::DeepslateTileSlab),
            702 => Some(Item::WoodenAxe),
            753 => Some(Item::DiamondBoots),
            1088 => Some(Item::LightGrayCandle),
            142 => Some(Item::WetSponge),
            1087 => Some(Item::GrayCandle),
            1098 => Some(Item::LargeAmethystBud),
            208 => Some(Item::AcaciaSlab),
            1042 => Some(Item::Composter),
            492 => Some(Item::LightGrayConcrete),
            901 => Some(Item::MuleSpawnEgg),
            722 => Some(Item::DiamondAxe),
            739 => Some(Item::LeatherChestplate),
            1084 => Some(Item::YellowCandle),
            373 => Some(Item::WhiteCarpet),
            725 => Some(Item::NetheriteShovel),
            632 => Some(Item::OakDoor),
            835 => Some(Item::LimeBed),
            184 => Some(Item::LilyOfTheValley),
            144 => Some(Item::TintedGlass),
            659 => Some(Item::Rail),
            457 => Some(Item::LimeShulkerBox),
            304 => Some(Item::LilyPad),
            1020 => Some(Item::MusicDiscMall),
            482 => Some(Item::RedGlazedTerracotta),
            28 => Some(Item::CrimsonPlanks),
            597 => Some(Item::Dropper),
            870 => Some(Item::Cauldron),
            1039 => Some(Item::MojangBannerPattern),
            166 => Some(Item::CyanWool),
            684 => Some(Item::Coal),
            42 => Some(Item::IronOre),
            89 => Some(Item::WaxedCutCopper),
            399 => Some(Item::LargeFern),
            335 => Some(Item::RedNetherBrickWall),
            648 => Some(Item::WarpedTrapdoor),
            228 => Some(Item::SmoothQuartz),
            448 => Some(Item::RedNetherBricks),
            15 => Some(Item::Dirt),
            55 => Some(Item::DeepslateDiamondOre),
            545 => Some(Item::DeadFireCoralFan),
            86 => Some(Item::WaxedExposedCopper),
            628 => Some(Item::DarkOakPressurePlate),
            168 => Some(Item::BlueWool),
            875 => Some(Item::BeeSpawnEgg),
            921 => Some(Item::SpiderSpawnEgg),
            695 => Some(Item::RawGold),
            107 => Some(Item::CrimsonStem),
            604 => Some(Item::TripwireHook),
            972 => Some(Item::ArmorStand),
            430 => Some(Item::RedStainedGlassPane),
            995 => Some(Item::GreenBanner),
            414 => Some(Item::RedStainedGlass),
            678 => Some(Item::TurtleHelmet),
            126 => Some(Item::SpruceWood),
            441 => Some(Item::CutRedSandstone),
            54 => Some(Item::DiamondOre),
            160 => Some(Item::LightBlueWool),
            412 => Some(Item::BrownStainedGlass),
            989 => Some(Item::GrayBanner),
            1032 => Some(Item::HeartOfTheSea),
            239 => Some(Item::ChorusFlower),
            828 => Some(Item::Sugar),
            14 => Some(Item::GrassBlock),
            672 => Some(Item::BirchBoat),
            968 => Some(Item::CookedRabbit),
            118 => Some(Item::StrippedSpruceWood),
            582 => Some(Item::DeepslateBrickSlab),
            887 => Some(Item::EndermanSpawnEgg),
            470 => Some(Item::MagentaGlazedTerracotta),
            664 => Some(Item::FurnaceMinecart),
            939 => Some(Item::ZombifiedPiglinSpawnEgg),
            1054 => Some(Item::SweetBerries),
            732 => Some(Item::String),
            756 => Some(Item::GoldenLeggings),
            984 => Some(Item::MagentaBanner),
            80 => Some(Item::OxidizedCutCopperStairs),
            696 => Some(Item::GoldIngot),
            857 => Some(Item::RottenFlesh),
            36 => Some(Item::Bedrock),
            645 => Some(Item::AcaciaTrapdoor),
            316 => Some(Item::EnderChest),
            410 => Some(Item::PurpleStainedGlass),
            884 => Some(Item::DonkeySpawnEgg),
            1050 => Some(Item::Stonecutter),
            148 => Some(Item::CutSandstone),
            864 => Some(Item::GlassBottle),
            23 => Some(Item::SprucePlanks),
            513 => Some(Item::GreenConcretePowder),
            752 => Some(Item::DiamondLeggings),
            1095 => Some(Item::BlackCandle),
            401 => Some(Item::OrangeStainedGlass),
            1004 => Some(Item::DragonBreath),
            848 => Some(Item::Shears),
            111 => Some(Item::StrippedBirchLog),
            432 => Some(Item::Prismarine),
            415 => Some(Item::BlackStainedGlass),
            91 => Some(Item::WaxedWeatheredCutCopper),
            638 => Some(Item::CrimsonDoor),
            1068 => Some(Item::BlackstoneStairs),
            960 => Some(Item::PumpkinPie),
            631 => Some(Item::IronDoor),
            905 => Some(Item::PhantomSpawnEgg),
            975 => Some(Item::DiamondHorseArmor),
            197 => Some(Item::Kelp),
            740 => Some(Item::LeatherLeggings),
            928 => Some(Item::VexSpawnEgg),
            455 => Some(Item::LightBlueShulkerBox),
            591 => Some(Item::StickyPiston),
            119 => Some(Item::StrippedBirchWood),
            484 => Some(Item::WhiteConcrete),
            551 => Some(Item::MossyStoneBrickStairs),
            66 => Some(Item::CopperBlock),
            249 => Some(Item::Ladder),
            191 => Some(Item::CrimsonRoots),
            525 => Some(Item::FireCoralBlock),
            733 => Some(Item::Feather),
            759 => Some(Item::NetheriteChestplate),
            902 => Some(Item::OcelotSpawnEgg),
            308 => Some(Item::NetherBrickFence),
            357 => Some(Item::LightBlueTerracotta),
            186 => Some(Item::SporeBlossom),
            213 => Some(Item::SmoothStoneSlab),
            1013 => Some(Item::KnowledgeBook),
            1061 => Some(Item::Beehive),
            52 => Some(Item::LapisOre),
            104 => Some(Item::JungleLog),
            893 => Some(Item::GoatSpawnEgg),
            1094 => Some(Item::RedCandle),
            291 => Some(Item::ChiseledDeepslate),
            381 => Some(Item::LightGrayCarpet),
            883 => Some(Item::DolphinSpawnEgg),
            579 => Some(Item::DioriteSlab),
            141 => Some(Item::Sponge),
            940 => Some(Item::ExperienceBottle),
            606 => Some(Item::Tnt),
            596 => Some(Item::Dispenser),
            268 => Some(Item::Netherrack),
            272 => Some(Item::PolishedBasalt),
            46 => Some(Item::GoldOre),
            1089 => Some(Item::CyanCandle),
            378 => Some(Item::LimeCarpet),
            297 => Some(Item::GlassPane),
            909 => Some(Item::PillagerSpawnEgg),
            776 => Some(Item::Bucket),
            825 => Some(Item::BlackDye),
            320 => Some(Item::JungleStairs),
            982 => Some(Item::WhiteBanner),
            994 => Some(Item::BrownBanner),
            910 => Some(Item::PolarBearSpawnEgg),
            879 => Some(Item::ChickenSpawnEgg),
            634 => Some(Item::BirchDoor),
            543 => Some(Item::DeadBrainCoralFan),
            924 => Some(Item::StriderSpawnEgg),
            445 => Some(Item::MagmaBlock),
            262 => Some(Item::DarkOakFence),
            392 => Some(Item::DarkOakStairs),
            569 => Some(Item::MossyStoneBrickSlab),
            81 => Some(Item::CutCopperSlab),
            523 => Some(Item::BrainCoralBlock),
            786 => Some(Item::TropicalFishBucket),
            435 => Some(Item::PrismarineStairs),
            290 => Some(Item::CrackedDeepslateTiles),
            13 => Some(Item::DripstoneBlock),
            757 => Some(Item::GoldenBoots),
            1092 => Some(Item::BrownCandle),
            927 => Some(Item::TurtleSpawnEgg),
            550 => Some(Item::SmoothRedSandstoneStairs),
            915 => Some(Item::SheepSpawnEgg),
            497 => Some(Item::GreenConcrete),
            39 => Some(Item::Gravel),
            _ => None,
        }
    }
}
impl Item {
    #[doc = "Returns the `name` property of this `Item`."]
    #[inline]
    pub fn name(&self) -> &'static str {
        match self {
            Item::AncientDebris => "ancient_debris",
            Item::ShulkerShell => "shulker_shell",
            Item::QuartzBlock => "quartz_block",
            Item::CrackedNetherBricks => "cracked_nether_bricks",
            Item::Saddle => "saddle",
            Item::GoldenAxe => "golden_axe",
            Item::ExperienceBottle => "experience_bottle",
            Item::PolishedBasalt => "polished_basalt",
            Item::String => "string",
            Item::OrangeStainedGlass => "orange_stained_glass",
            Item::PhantomSpawnEgg => "phantom_spawn_egg",
            Item::RabbitFoot => "rabbit_foot",
            Item::LightBlueStainedGlassPane => "light_blue_stained_glass_pane",
            Item::DarkOakTrapdoor => "dark_oak_trapdoor",
            Item::NetherBrickSlab => "nether_brick_slab",
            Item::StrippedBirchWood => "stripped_birch_wood",
            Item::RawIronBlock => "raw_iron_block",
            Item::PolishedBlackstoneWall => "polished_blackstone_wall",
            Item::WarpedSign => "warped_sign",
            Item::DiamondOre => "diamond_ore",
            Item::WarpedFence => "warped_fence",
            Item::PolishedAndesiteStairs => "polished_andesite_stairs",
            Item::NetheriteSword => "netherite_sword",
            Item::PurpleConcrete => "purple_concrete",
            Item::StructureVoid => "structure_void",
            Item::OakButton => "oak_button",
            Item::Scute => "scute",
            Item::CobbledDeepslateStairs => "cobbled_deepslate_stairs",
            Item::Bedrock => "bedrock",
            Item::MediumAmethystBud => "medium_amethyst_bud",
            Item::NetherBrickStairs => "nether_brick_stairs",
            Item::GreenBed => "green_bed",
            Item::SmoothQuartzStairs => "smooth_quartz_stairs",
            Item::LimeGlazedTerracotta => "lime_glazed_terracotta",
            Item::SalmonBucket => "salmon_bucket",
            Item::BlazePowder => "blaze_powder",
            Item::PiglinBannerPattern => "piglin_banner_pattern",
            Item::SmoothQuartz => "smooth_quartz",
            Item::DriedKelp => "dried_kelp",
            Item::CoalOre => "coal_ore",
            Item::DarkOakPlanks => "dark_oak_planks",
            Item::OrangeGlazedTerracotta => "orange_glazed_terracotta",
            Item::PurpleStainedGlassPane => "purple_stained_glass_pane",
            Item::DirtPath => "dirt_path",
            Item::DeadFireCoralBlock => "dead_fire_coral_block",
            Item::GoldenHorseArmor => "golden_horse_armor",
            Item::Painting => "painting",
            Item::BlackConcrete => "black_concrete",
            Item::DaylightDetector => "daylight_detector",
            Item::NetheritePickaxe => "netherite_pickaxe",
            Item::MooshroomSpawnEgg => "mooshroom_spawn_egg",
            Item::WaxedOxidizedCopper => "waxed_oxidized_copper",
            Item::GlowItemFrame => "glow_item_frame",
            Item::OakLog => "oak_log",
            Item::Anvil => "anvil",
            Item::NetheriteAxe => "netherite_axe",
            Item::GoldenHelmet => "golden_helmet",
            Item::Stonecutter => "stonecutter",
            Item::ChiseledStoneBricks => "chiseled_stone_bricks",
            Item::MossyCobblestone => "mossy_cobblestone",
            Item::GrayConcretePowder => "gray_concrete_powder",
            Item::DeepslateBrickSlab => "deepslate_brick_slab",
            Item::Mycelium => "mycelium",
            Item::MushroomStem => "mushroom_stem",
            Item::GraniteWall => "granite_wall",
            Item::PinkBanner => "pink_banner",
            Item::ZombifiedPiglinSpawnEgg => "zombified_piglin_spawn_egg",
            Item::MusicDiscWait => "music_disc_wait",
            Item::BirchPlanks => "birch_planks",
            Item::AcaciaWood => "acacia_wood",
            Item::DeepslateTileSlab => "deepslate_tile_slab",
            Item::YellowShulkerBox => "yellow_shulker_box",
            Item::LeatherLeggings => "leather_leggings",
            Item::DragonHead => "dragon_head",
            Item::DarkPrismarine => "dark_prismarine",
            Item::Azalea => "azalea",
            Item::HoneycombBlock => "honeycomb_block",
            Item::GreenTerracotta => "green_terracotta",
            Item::Dispenser => "dispenser",
            Item::AndesiteWall => "andesite_wall",
            Item::BlackStainedGlassPane => "black_stained_glass_pane",
            Item::PinkShulkerBox => "pink_shulker_box",
            Item::BubbleCoralBlock => "bubble_coral_block",
            Item::CrimsonNylium => "crimson_nylium",
            Item::CyanStainedGlass => "cyan_stained_glass",
            Item::PrismarineBrickSlab => "prismarine_brick_slab",
            Item::LargeAmethystBud => "large_amethyst_bud",
            Item::FermentedSpiderEye => "fermented_spider_eye",
            Item::BubbleCoral => "bubble_coral",
            Item::PillagerSpawnEgg => "pillager_spawn_egg",
            Item::GoldenBoots => "golden_boots",
            Item::BlueBed => "blue_bed",
            Item::PetrifiedOakSlab => "petrified_oak_slab",
            Item::MagentaCandle => "magenta_candle",
            Item::SlimeBall => "slime_ball",
            Item::TropicalFishBucket => "tropical_fish_bucket",
            Item::GraniteStairs => "granite_stairs",
            Item::PurpleCandle => "purple_candle",
            Item::WhiteWool => "white_wool",
            Item::Blackstone => "blackstone",
            Item::Poppy => "poppy",
            Item::BirchWood => "birch_wood",
            Item::Netherrack => "netherrack",
            Item::Glass => "glass",
            Item::MagentaConcretePowder => "magenta_concrete_powder",
            Item::YellowBanner => "yellow_banner",
            Item::BrownBanner => "brown_banner",
            Item::StraySpawnEgg => "stray_spawn_egg",
            Item::PolishedBlackstoneBricks => "polished_blackstone_bricks",
            Item::OxidizedCutCopper => "oxidized_cut_copper",
            Item::HornCoral => "horn_coral",
            Item::WeatheredCutCopperSlab => "weathered_cut_copper_slab",
            Item::SmallAmethystBud => "small_amethyst_bud",
            Item::CopperIngot => "copper_ingot",
            Item::Target => "target",
            Item::NetherSprouts => "nether_sprouts",
            Item::PandaSpawnEgg => "panda_spawn_egg",
            Item::BlazeRod => "blaze_rod",
            Item::BirchLog => "birch_log",
            Item::PurpleTerracotta => "purple_terracotta",
            Item::CyanGlazedTerracotta => "cyan_glazed_terracotta",
            Item::StrippedCrimsonHyphae => "stripped_crimson_hyphae",
            Item::AcaciaButton => "acacia_button",
            Item::AcaciaLog => "acacia_log",
            Item::WaxedExposedCutCopperStairs => "waxed_exposed_cut_copper_stairs",
            Item::MusicDiscFar => "music_disc_far",
            Item::LavaBucket => "lava_bucket",
            Item::WaxedWeatheredCutCopperSlab => "waxed_weathered_cut_copper_slab",
            Item::ChiseledSandstone => "chiseled_sandstone",
            Item::InfestedStoneBricks => "infested_stone_bricks",
            Item::OxeyeDaisy => "oxeye_daisy",
            Item::Cactus => "cactus",
            Item::JungleSign => "jungle_sign",
            Item::WaxedOxidizedCutCopper => "waxed_oxidized_cut_copper",
            Item::FloweringAzaleaLeaves => "flowering_azalea_leaves",
            Item::SmoothStoneSlab => "smooth_stone_slab",
            Item::StrippedDarkOakWood => "stripped_dark_oak_wood",
            Item::SoulCampfire => "soul_campfire",
            Item::MagentaConcrete => "magenta_concrete",
            Item::PolishedDioriteStairs => "polished_diorite_stairs",
            Item::OakPressurePlate => "oak_pressure_plate",
            Item::PrismarineShard => "prismarine_shard",
            Item::MojangBannerPattern => "mojang_banner_pattern",
            Item::OrangeCarpet => "orange_carpet",
            Item::StrippedDarkOakLog => "stripped_dark_oak_log",
            Item::SlimeBlock => "slime_block",
            Item::StonePickaxe => "stone_pickaxe",
            Item::HeartOfTheSea => "heart_of_the_sea",
            Item::Quartz => "quartz",
            Item::SoulSoil => "soul_soil",
            Item::Cauldron => "cauldron",
            Item::RepeatingCommandBlock => "repeating_command_block",
            Item::WoodenPickaxe => "wooden_pickaxe",
            Item::GlowLichen => "glow_lichen",
            Item::SpruceDoor => "spruce_door",
            Item::MusicDiscWard => "music_disc_ward",
            Item::SoulTorch => "soul_torch",
            Item::GoldenPickaxe => "golden_pickaxe",
            Item::LimeCarpet => "lime_carpet",
            Item::AndesiteSlab => "andesite_slab",
            Item::Composter => "composter",
            Item::WeatheredCutCopper => "weathered_cut_copper",
            Item::LimeShulkerBox => "lime_shulker_box",
            Item::StrippedAcaciaWood => "stripped_acacia_wood",
            Item::Sponge => "sponge",
            Item::StrippedSpruceWood => "stripped_spruce_wood",
            Item::Snow => "snow",
            Item::YellowTerracotta => "yellow_terracotta",
            Item::AcaciaBoat => "acacia_boat",
            Item::StriderSpawnEgg => "strider_spawn_egg",
            Item::LightGrayCandle => "light_gray_candle",
            Item::IronBoots => "iron_boots",
            Item::TurtleEgg => "turtle_egg",
            Item::MossyStoneBrickStairs => "mossy_stone_brick_stairs",
            Item::WaxedCutCopperSlab => "waxed_cut_copper_slab",
            Item::PolishedGraniteStairs => "polished_granite_stairs",
            Item::InfestedChiseledStoneBricks => "infested_chiseled_stone_bricks",
            Item::OxidizedCopper => "oxidized_copper",
            Item::CrackedStoneBricks => "cracked_stone_bricks",
            Item::Scaffolding => "scaffolding",
            Item::CodBucket => "cod_bucket",
            Item::HuskSpawnEgg => "husk_spawn_egg",
            Item::StrippedBirchLog => "stripped_birch_log",
            Item::LimeConcretePowder => "lime_concrete_powder",
            Item::CreeperSpawnEgg => "creeper_spawn_egg",
            Item::GoatSpawnEgg => "goat_spawn_egg",
            Item::SpruceLeaves => "spruce_leaves",
            Item::Sugar => "sugar",
            Item::CodSpawnEgg => "cod_spawn_egg",
            Item::RabbitHide => "rabbit_hide",
            Item::MusicDiscOtherside => "music_disc_otherside",
            Item::CyanCandle => "cyan_candle",
            Item::DarkPrismarineSlab => "dark_prismarine_slab",
            Item::SkullBannerPattern => "skull_banner_pattern",
            Item::Bricks => "bricks",
            Item::Shield => "shield",
            Item::LlamaSpawnEgg => "llama_spawn_egg",
            Item::GrassBlock => "grass_block",
            Item::Book => "book",
            Item::Bell => "bell",
            Item::RedNetherBrickWall => "red_nether_brick_wall",
            Item::BlackBed => "black_bed",
            Item::JungleLeaves => "jungle_leaves",
            Item::DeadFireCoralFan => "dead_fire_coral_fan",
            Item::TropicalFish => "tropical_fish",
            Item::BoneMeal => "bone_meal",
            Item::OrangeBed => "orange_bed",
            Item::Chest => "chest",
            Item::RedCarpet => "red_carpet",
            Item::LightBlueConcrete => "light_blue_concrete",
            Item::WeepingVines => "weeping_vines",
            Item::Crossbow => "crossbow",
            Item::TintedGlass => "tinted_glass",
            Item::LimeBanner => "lime_banner",
            Item::CrimsonButton => "crimson_button",
            Item::PrismarineBricks => "prismarine_bricks",
            Item::StoneHoe => "stone_hoe",
            Item::BlackConcretePowder => "black_concrete_powder",
            Item::Porkchop => "porkchop",
            Item::WeatheredCutCopperStairs => "weathered_cut_copper_stairs",
            Item::PurpleCarpet => "purple_carpet",
            Item::FireCharge => "fire_charge",
            Item::MossyStoneBrickWall => "mossy_stone_brick_wall",
            Item::MagentaStainedGlassPane => "magenta_stained_glass_pane",
            Item::RedConcretePowder => "red_concrete_powder",
            Item::AcaciaSign => "acacia_sign",
            Item::LightGrayWool => "light_gray_wool",
            Item::PurpurBlock => "purpur_block",
            Item::GoldenSword => "golden_sword",
            Item::PurpleDye => "purple_dye",
            Item::NetherBricks => "nether_bricks",
            Item::LightBlueBed => "light_blue_bed",
            Item::DarkOakLeaves => "dark_oak_leaves",
            Item::ZombieSpawnEgg => "zombie_spawn_egg",
            Item::MusicDiscMellohi => "music_disc_mellohi",
            Item::DetectorRail => "detector_rail",
            Item::WhiteBed => "white_bed",
            Item::Minecart => "minecart",
            Item::CartographyTable => "cartography_table",
            Item::DeepslateIronOre => "deepslate_iron_ore",
            Item::DarkOakPressurePlate => "dark_oak_pressure_plate",
            Item::CrackedDeepslateBricks => "cracked_deepslate_bricks",
            Item::BlackStainedGlass => "black_stained_glass",
            Item::IronChestplate => "iron_chestplate",
            Item::Chicken => "chicken",
            Item::SmoothRedSandstoneStairs => "smooth_red_sandstone_stairs",
            Item::WarpedFungus => "warped_fungus",
            Item::MusicDisc13 => "music_disc_13",
            Item::DeepslateBrickStairs => "deepslate_brick_stairs",
            Item::TallGrass => "tall_grass",
            Item::MagentaDye => "magenta_dye",
            Item::Dandelion => "dandelion",
            Item::WrittenBook => "written_book",
            Item::JungleFence => "jungle_fence",
            Item::OakPlanks => "oak_planks",
            Item::GlassBottle => "glass_bottle",
            Item::LightBlueGlazedTerracotta => "light_blue_glazed_terracotta",
            Item::RedstoneTorch => "redstone_torch",
            Item::RedMushroomBlock => "red_mushroom_block",
            Item::DeadBubbleCoralBlock => "dead_bubble_coral_block",
            Item::Bookshelf => "bookshelf",
            Item::DiamondHoe => "diamond_hoe",
            Item::DeepslateBrickWall => "deepslate_brick_wall",
            Item::Light => "light",
            Item::WaterBucket => "water_bucket",
            Item::DeepslateEmeraldOre => "deepslate_emerald_ore",
            Item::DeadBubbleCoralFan => "dead_bubble_coral_fan",
            Item::GuardianSpawnEgg => "guardian_spawn_egg",
            Item::MuleSpawnEgg => "mule_spawn_egg",
            Item::RawCopperBlock => "raw_copper_block",
            Item::JungleWood => "jungle_wood",
            Item::RedStainedGlass => "red_stained_glass",
            Item::DeadBubbleCoral => "dead_bubble_coral",
            Item::Potato => "potato",
            Item::Compass => "compass",
            Item::PinkDye => "pink_dye",
            Item::Observer => "observer",
            Item::MagentaGlazedTerracotta => "magenta_glazed_terracotta",
            Item::DeadBush => "dead_bush",
            Item::PolishedBlackstoneSlab => "polished_blackstone_slab",
            Item::VindicatorSpawnEgg => "vindicator_spawn_egg",
            Item::ChainmailLeggings => "chainmail_leggings",
            Item::DarkOakDoor => "dark_oak_door",
            Item::BlueOrchid => "blue_orchid",
            Item::BrownMushroomBlock => "brown_mushroom_block",
            Item::RedSandstoneWall => "red_sandstone_wall",
            Item::WhiteStainedGlass => "white_stained_glass",
            Item::Beetroot => "beetroot",
            Item::Rail => "rail",
            Item::PhantomMembrane => "phantom_membrane",
            Item::WhiteShulkerBox => "white_shulker_box",
            Item::BlueBanner => "blue_banner",
            Item::Farmland => "farmland",
            Item::RedNetherBrickSlab => "red_nether_brick_slab",
            Item::PolishedAndesiteSlab => "polished_andesite_slab",
            Item::BlueStainedGlass => "blue_stained_glass",
            Item::DragonBreath => "dragon_breath",
            Item::WritableBook => "writable_book",
            Item::GreenGlazedTerracotta => "green_glazed_terracotta",
            Item::DeepslateRedstoneOre => "deepslate_redstone_ore",
            Item::Prismarine => "prismarine",
            Item::PurpleBed => "purple_bed",
            Item::BlastFurnace => "blast_furnace",
            Item::BrickSlab => "brick_slab",
            Item::GreenStainedGlassPane => "green_stained_glass_pane",
            Item::ChiseledRedSandstone => "chiseled_red_sandstone",
            Item::MossyCobblestoneSlab => "mossy_cobblestone_slab",
            Item::BlackDye => "black_dye",
            Item::DeepslateBricks => "deepslate_bricks",
            Item::HeavyWeightedPressurePlate => "heavy_weighted_pressure_plate",
            Item::BeetrootSeeds => "beetroot_seeds",
            Item::PolishedGraniteSlab => "polished_granite_slab",
            Item::Bow => "bow",
            Item::LightGrayGlazedTerracotta => "light_gray_glazed_terracotta",
            Item::TripwireHook => "tripwire_hook",
            Item::IronNugget => "iron_nugget",
            Item::Granite => "granite",
            Item::IronBlock => "iron_block",
            Item::WarpedSlab => "warped_slab",
            Item::SmallDripleaf => "small_dripleaf",
            Item::PolishedDeepslate => "polished_deepslate",
            Item::RedWool => "red_wool",
            Item::OakBoat => "oak_boat",
            Item::WhiteCarpet => "white_carpet",
            Item::OrangeDye => "orange_dye",
            Item::SmoothQuartzSlab => "smooth_quartz_slab",
            Item::CowSpawnEgg => "cow_spawn_egg",
            Item::SeaLantern => "sea_lantern",
            Item::LightWeightedPressurePlate => "light_weighted_pressure_plate",
            Item::RoseBush => "rose_bush",
            Item::PinkBed => "pink_bed",
            Item::CookedMutton => "cooked_mutton",
            Item::DeepslateLapisOre => "deepslate_lapis_ore",
            Item::Seagrass => "seagrass",
            Item::PolishedBlackstoneButton => "polished_blackstone_button",
            Item::CocoaBeans => "cocoa_beans",
            Item::ExposedCopper => "exposed_copper",
            Item::PolishedDeepslateStairs => "polished_deepslate_stairs",
            Item::GoldenApple => "golden_apple",
            Item::Clock => "clock",
            Item::PurpleStainedGlass => "purple_stained_glass",
            Item::KnowledgeBook => "knowledge_book",
            Item::Coal => "coal",
            Item::EmeraldBlock => "emerald_block",
            Item::PowderSnowBucket => "powder_snow_bucket",
            Item::LightBlueBanner => "light_blue_banner",
            Item::CutCopper => "cut_copper",
            Item::CrimsonPlanks => "crimson_planks",
            Item::JungleFenceGate => "jungle_fence_gate",
            Item::Candle => "candle",
            Item::PolishedBlackstoneStairs => "polished_blackstone_stairs",
            Item::HornCoralFan => "horn_coral_fan",
            Item::WeatheredCopper => "weathered_copper",
            Item::WarpedPlanks => "warped_planks",
            Item::EndStoneBrickWall => "end_stone_brick_wall",
            Item::JungleButton => "jungle_button",
            Item::DeepslateTiles => "deepslate_tiles",
            Item::Ice => "ice",
            Item::CutRedSandstoneSlab => "cut_red_sandstone_slab",
            Item::LimeCandle => "lime_candle",
            Item::SoulLantern => "soul_lantern",
            Item::Tnt => "tnt",
            Item::AcaciaPlanks => "acacia_planks",
            Item::WheatSeeds => "wheat_seeds",
            Item::StoneStairs => "stone_stairs",
            Item::MagmaCubeSpawnEgg => "magma_cube_spawn_egg",
            Item::LimeTerracotta => "lime_terracotta",
            Item::LightGrayBed => "light_gray_bed",
            Item::MushroomStew => "mushroom_stew",
            Item::RedSandstone => "red_sandstone",
            Item::SpectralArrow => "spectral_arrow",
            Item::MusicDiscStrad => "music_disc_strad",
            Item::Emerald => "emerald",
            Item::GlassPane => "glass_pane",
            Item::PolishedBlackstonePressurePlate => "polished_blackstone_pressure_plate",
            Item::Stick => "stick",
            Item::LightGrayShulkerBox => "light_gray_shulker_box",
            Item::StoneBricks => "stone_bricks",
            Item::Vine => "vine",
            Item::Comparator => "comparator",
            Item::EndCrystal => "end_crystal",
            Item::DeepslateCoalOre => "deepslate_coal_ore",
            Item::Smoker => "smoker",
            Item::DeepslateDiamondOre => "deepslate_diamond_ore",
            Item::LimeStainedGlass => "lime_stained_glass",
            Item::Melon => "melon",
            Item::IronAxe => "iron_axe",
            Item::WaxedOxidizedCutCopperStairs => "waxed_oxidized_cut_copper_stairs",
            Item::RedNetherBrickStairs => "red_nether_brick_stairs",
            Item::LapisLazuli => "lapis_lazuli",
            Item::BlackstoneStairs => "blackstone_stairs",
            Item::OcelotSpawnEgg => "ocelot_spawn_egg",
            Item::RedConcrete => "red_concrete",
            Item::DripstoneBlock => "dripstone_block",
            Item::InfestedDeepslate => "infested_deepslate",
            Item::ShulkerSpawnEgg => "shulker_spawn_egg",
            Item::JungleSapling => "jungle_sapling",
            Item::SpiderEye => "spider_eye",
            Item::YellowCandle => "yellow_candle",
            Item::Deepslate => "deepslate",
            Item::Fern => "fern",
            Item::BrownConcretePowder => "brown_concrete_powder",
            Item::DeepslateTileStairs => "deepslate_tile_stairs",
            Item::Cookie => "cookie",
            Item::EnchantedBook => "enchanted_book",
            Item::CutSandstone => "cut_sandstone",
            Item::LilyPad => "lily_pad",
            Item::Cornflower => "cornflower",
            Item::NetherBrickWall => "nether_brick_wall",
            Item::Loom => "loom",
            Item::JungleBoat => "jungle_boat",
            Item::YellowDye => "yellow_dye",
            Item::OakFence => "oak_fence",
            Item::FlintAndSteel => "flint_and_steel",
            Item::GreenStainedGlass => "green_stained_glass",
            Item::Lodestone => "lodestone",
            Item::SporeBlossom => "spore_blossom",
            Item::YellowGlazedTerracotta => "yellow_glazed_terracotta",
            Item::PolishedBlackstoneBrickWall => "polished_blackstone_brick_wall",
            Item::DiamondBoots => "diamond_boots",
            Item::HangingRoots => "hanging_roots",
            Item::BlueIce => "blue_ice",
            Item::MilkBucket => "milk_bucket",
            Item::DiamondBlock => "diamond_block",
            Item::ZombieVillagerSpawnEgg => "zombie_villager_spawn_egg",
            Item::GoldenCarrot => "golden_carrot",
            Item::Pufferfish => "pufferfish",
            Item::LightGrayConcretePowder => "light_gray_concrete_powder",
            Item::OrangeTerracotta => "orange_terracotta",
            Item::DeepslateGoldOre => "deepslate_gold_ore",
            Item::PrismarineBrickStairs => "prismarine_brick_stairs",
            Item::BirchSign => "birch_sign",
            Item::MelonSlice => "melon_slice",
            Item::CookedChicken => "cooked_chicken",
            Item::PolarBearSpawnEgg => "polar_bear_spawn_egg",
            Item::BrownCandle => "brown_candle",
            Item::BirchFence => "birch_fence",
            Item::EndStone => "end_stone",
            Item::CyanBed => "cyan_bed",
            Item::OrangeStainedGlassPane => "orange_stained_glass_pane",
            Item::DeadTubeCoral => "dead_tube_coral",
            Item::WarpedDoor => "warped_door",
            Item::BirchStairs => "birch_stairs",
            Item::EndStoneBrickStairs => "end_stone_brick_stairs",
            Item::Gunpowder => "gunpowder",
            Item::BeetrootSoup => "beetroot_soup",
            Item::GreenDye => "green_dye",
            Item::LightBlueCandle => "light_blue_candle",
            Item::Apple => "apple",
            Item::SkeletonSkull => "skeleton_skull",
            Item::BrainCoralFan => "brain_coral_fan",
            Item::LightGrayCarpet => "light_gray_carpet",
            Item::SprucePlanks => "spruce_planks",
            Item::Snowball => "snowball",
            Item::Piston => "piston",
            Item::SpruceSign => "spruce_sign",
            Item::LightGrayConcrete => "light_gray_concrete",
            Item::StoneAxe => "stone_axe",
            Item::MusicDiscChirp => "music_disc_chirp",
            Item::EnderPearl => "ender_pearl",
            Item::GoldNugget => "gold_nugget",
            Item::WaxedExposedCopper => "waxed_exposed_copper",
            Item::GoldenShovel => "golden_shovel",
            Item::TwistingVines => "twisting_vines",
            Item::PiglinBruteSpawnEgg => "piglin_brute_spawn_egg",
            Item::Lever => "lever",
            Item::PumpkinSeeds => "pumpkin_seeds",
            Item::BlueDye => "blue_dye",
            Item::Peony => "peony",
            Item::Cake => "cake",
            Item::OrangeBanner => "orange_banner",
            Item::MusicDiscCat => "music_disc_cat",
            Item::SmoothStone => "smooth_stone",
            Item::BrainCoral => "brain_coral",
            Item::OakDoor => "oak_door",
            Item::TubeCoral => "tube_coral",
            Item::FurnaceMinecart => "furnace_minecart",
            Item::Leather => "leather",
            Item::NautilusShell => "nautilus_shell",
            Item::Kelp => "kelp",
            Item::SandstoneWall => "sandstone_wall",
            Item::RawGold => "raw_gold",
            Item::PumpkinPie => "pumpkin_pie",
            Item::AcaciaSapling => "acacia_sapling",
            Item::GrayCarpet => "gray_carpet",
            Item::RedstoneBlock => "redstone_block",
            Item::MossyStoneBrickSlab => "mossy_stone_brick_slab",
            Item::PoweredRail => "powered_rail",
            Item::LightGrayStainedGlass => "light_gray_stained_glass",
            Item::NetheriteChestplate => "netherite_chestplate",
            Item::LightBlueTerracotta => "light_blue_terracotta",
            Item::BakedPotato => "baked_potato",
            Item::PurpleConcretePowder => "purple_concrete_powder",
            Item::Stone => "stone",
            Item::SmithingTable => "smithing_table",
            Item::TrappedChest => "trapped_chest",
            Item::GrayTerracotta => "gray_terracotta",
            Item::WaxedOxidizedCutCopperSlab => "waxed_oxidized_cut_copper_slab",
            Item::GildedBlackstone => "gilded_blackstone",
            Item::TropicalFishSpawnEgg => "tropical_fish_spawn_egg",
            Item::AcaciaLeaves => "acacia_leaves",
            Item::BlackCarpet => "black_carpet",
            Item::GoldIngot => "gold_ingot",
            Item::CommandBlockMinecart => "command_block_minecart",
            Item::BubbleCoralFan => "bubble_coral_fan",
            Item::MusicDisc11 => "music_disc_11",
            Item::PurpleBanner => "purple_banner",
            Item::PolishedBlackstone => "polished_blackstone",
            Item::RedGlazedTerracotta => "red_glazed_terracotta",
            Item::BlueStainedGlassPane => "blue_stained_glass_pane",
            Item::MagmaBlock => "magma_block",
            Item::Podzol => "podzol",
            Item::StickyPiston => "sticky_piston",
            Item::GhastTear => "ghast_tear",
            Item::BigDripleaf => "big_dripleaf",
            Item::BlueCarpet => "blue_carpet",
            Item::CyanStainedGlassPane => "cyan_stained_glass_pane",
            Item::YellowConcretePowder => "yellow_concrete_powder",
            Item::DiamondChestplate => "diamond_chestplate",
            Item::ZombieHorseSpawnEgg => "zombie_horse_spawn_egg",
            Item::WarpedFenceGate => "warped_fence_gate",
            Item::SweetBerries => "sweet_berries",
            Item::StrippedSpruceLog => "stripped_spruce_log",
            Item::CoalBlock => "coal_block",
            Item::RedSandstoneSlab => "red_sandstone_slab",
            Item::DarkOakBoat => "dark_oak_boat",
            Item::Bundle => "bundle",
            Item::NetheriteIngot => "netherite_ingot",
            Item::CrimsonStairs => "crimson_stairs",
            Item::WhiteConcretePowder => "white_concrete_powder",
            Item::PufferfishBucket => "pufferfish_bucket",
            Item::CobblestoneStairs => "cobblestone_stairs",
            Item::WarpedWartBlock => "warped_wart_block",
            Item::ChickenSpawnEgg => "chicken_spawn_egg",
            Item::OrangeTulip => "orange_tulip",
            Item::Glowstone => "glowstone",
            Item::Chain => "chain",
            Item::BirchTrapdoor => "birch_trapdoor",
            Item::MagmaCream => "magma_cream",
            Item::AxolotlSpawnEgg => "axolotl_spawn_egg",
            Item::StoneButton => "stone_button",
            Item::CopperBlock => "copper_block",
            Item::SculkSensor => "sculk_sensor",
            Item::OakStairs => "oak_stairs",
            Item::BirchDoor => "birch_door",
            Item::PinkCarpet => "pink_carpet",
            Item::StrippedWarpedHyphae => "stripped_warped_hyphae",
            Item::BirchSapling => "birch_sapling",
            Item::PinkConcretePowder => "pink_concrete_powder",
            Item::JungleStairs => "jungle_stairs",
            Item::BlueConcretePowder => "blue_concrete_powder",
            Item::PolishedBlackstoneBrickSlab => "polished_blackstone_brick_slab",
            Item::FlowerPot => "flower_pot",
            Item::Jigsaw => "jigsaw",
            Item::LightGrayBanner => "light_gray_banner",
            Item::ZoglinSpawnEgg => "zoglin_spawn_egg",
            Item::CreeperBannerPattern => "creeper_banner_pattern",
            Item::LapisBlock => "lapis_block",
            Item::AmethystShard => "amethyst_shard",
            Item::PufferfishSpawnEgg => "pufferfish_spawn_egg",
            Item::PurpurPillar => "purpur_pillar",
            Item::Furnace => "furnace",
            Item::OakWood => "oak_wood",
            Item::CutSandstoneSlab => "cut_sandstone_slab",
            Item::BlueShulkerBox => "blue_shulker_box",
            Item::RottenFlesh => "rotten_flesh",
            Item::WaxedWeatheredCutCopperStairs => "waxed_weathered_cut_copper_stairs",
            Item::SpruceLog => "spruce_log",
            Item::NetheriteLeggings => "netherite_leggings",
            Item::ShulkerBox => "shulker_box",
            Item::JunglePressurePlate => "jungle_pressure_plate",
            Item::DiamondSword => "diamond_sword",
            Item::RedMushroom => "red_mushroom",
            Item::HopperMinecart => "hopper_minecart",
            Item::WanderingTraderSpawnEgg => "wandering_trader_spawn_egg",
            Item::Map => "map",
            Item::EndermanSpawnEgg => "enderman_spawn_egg",
            Item::Shears => "shears",
            Item::RedSand => "red_sand",
            Item::WhiteTulip => "white_tulip",
            Item::BrownStainedGlass => "brown_stained_glass",
            Item::LeatherChestplate => "leather_chestplate",
            Item::CraftingTable => "crafting_table",
            Item::DarkOakSign => "dark_oak_sign",
            Item::Campfire => "campfire",
            Item::DonkeySpawnEgg => "donkey_spawn_egg",
            Item::Trident => "trident",
            Item::FireCoralFan => "fire_coral_fan",
            Item::GrayCandle => "gray_candle",
            Item::BrownConcrete => "brown_concrete",
            Item::NetheriteShovel => "netherite_shovel",
            Item::PrismarineStairs => "prismarine_stairs",
            Item::AmethystCluster => "amethyst_cluster",
            Item::InfestedCrackedStoneBricks => "infested_cracked_stone_bricks",
            Item::MossyStoneBricks => "mossy_stone_bricks",
            Item::BrewingStand => "brewing_stand",
            Item::SmoothSandstone => "smooth_sandstone",
            Item::RawGoldBlock => "raw_gold_block",
            Item::BrownStainedGlassPane => "brown_stained_glass_pane",
            Item::GreenCarpet => "green_carpet",
            Item::StoneShovel => "stone_shovel",
            Item::CrackedPolishedBlackstoneBricks => "cracked_polished_blackstone_bricks",
            Item::YellowConcrete => "yellow_concrete",
            Item::MagentaWool => "magenta_wool",
            Item::WhiteStainedGlassPane => "white_stained_glass_pane",
            Item::Sandstone => "sandstone",
            Item::GreenWool => "green_wool",
            Item::SlimeSpawnEgg => "slime_spawn_egg",
            Item::SmoothSandstoneSlab => "smooth_sandstone_slab",
            Item::EnchantingTable => "enchanting_table",
            Item::FireworkRocket => "firework_rocket",
            Item::SmoothRedSandstoneSlab => "smooth_red_sandstone_slab",
            Item::Bucket => "bucket",
            Item::WaxedCutCopperStairs => "waxed_cut_copper_stairs",
            Item::ChainmailChestplate => "chainmail_chestplate",
            Item::EndPortalFrame => "end_portal_frame",
            Item::WhiteConcrete => "white_concrete",
            Item::CrimsonTrapdoor => "crimson_trapdoor",
            Item::IronOre => "iron_ore",
            Item::RedstoneLamp => "redstone_lamp",
            Item::RedNetherBricks => "red_nether_bricks",
            Item::VexSpawnEgg => "vex_spawn_egg",
            Item::PrismarineSlab => "prismarine_slab",
            Item::EndStoneBricks => "end_stone_bricks",
            Item::FoxSpawnEgg => "fox_spawn_egg",
            Item::Basalt => "basalt",
            Item::OrangeShulkerBox => "orange_shulker_box",
            Item::Charcoal => "charcoal",
            Item::BlackShulkerBox => "black_shulker_box",
            Item::GrayBed => "gray_bed",
            Item::Calcite => "calcite",
            Item::PrismarineCrystals => "prismarine_crystals",
            Item::WarpedHyphae => "warped_hyphae",
            Item::CrimsonFence => "crimson_fence",
            Item::LightBlueCarpet => "light_blue_carpet",
            Item::LightBlueDye => "light_blue_dye",
            Item::BlueWool => "blue_wool",
            Item::NetherWartBlock => "nether_wart_block",
            Item::RedCandle => "red_candle",
            Item::SpruceFenceGate => "spruce_fence_gate",
            Item::LeatherHorseArmor => "leather_horse_armor",
            Item::HoneyBottle => "honey_bottle",
            Item::PinkStainedGlassPane => "pink_stained_glass_pane",
            Item::InfestedStone => "infested_stone",
            Item::DolphinSpawnEgg => "dolphin_spawn_egg",
            Item::DarkOakStairs => "dark_oak_stairs",
            Item::DamagedAnvil => "damaged_anvil",
            Item::Sunflower => "sunflower",
            Item::GlowInkSac => "glow_ink_sac",
            Item::AcaciaSlab => "acacia_slab",
            Item::WhiteCandle => "white_candle",
            Item::OrangeCandle => "orange_candle",
            Item::WarpedTrapdoor => "warped_trapdoor",
            Item::StrippedJungleWood => "stripped_jungle_wood",
            Item::DeadHornCoralBlock => "dead_horn_coral_block",
            Item::SpruceSapling => "spruce_sapling",
            Item::BeeNest => "bee_nest",
            Item::ChippedAnvil => "chipped_anvil",
            Item::DeadBrainCoral => "dead_brain_coral",
            Item::SpruceBoat => "spruce_boat",
            Item::IronSword => "iron_sword",
            Item::IronHelmet => "iron_helmet",
            Item::DeadTubeCoralFan => "dead_tube_coral_fan",
            Item::FletchingTable => "fletching_table",
            Item::CookedSalmon => "cooked_salmon",
            Item::DarkPrismarineStairs => "dark_prismarine_stairs",
            Item::Brick => "brick",
            Item::AxolotlBucket => "axolotl_bucket",
            Item::BrownMushroom => "brown_mushroom",
            Item::FishingRod => "fishing_rod",
            Item::DioriteWall => "diorite_wall",
            Item::Honeycomb => "honeycomb",
            Item::RedStainedGlassPane => "red_stained_glass_pane",
            Item::RabbitSpawnEgg => "rabbit_spawn_egg",
            Item::WitherSkeletonSkull => "wither_skeleton_skull",
            Item::WarpedFungusOnAStick => "warped_fungus_on_a_stick",
            Item::IronBars => "iron_bars",
            Item::HornCoralBlock => "horn_coral_block",
            Item::IronShovel => "iron_shovel",
            Item::AcaciaStairs => "acacia_stairs",
            Item::SpruceStairs => "spruce_stairs",
            Item::TubeCoralBlock => "tube_coral_block",
            Item::Dropper => "dropper",
            Item::CyanCarpet => "cyan_carpet",
            Item::OakSign => "oak_sign",
            Item::Jukebox => "jukebox",
            Item::FireworkStar => "firework_star",
            Item::Rabbit => "rabbit",
            Item::MusicDiscStal => "music_disc_stal",
            Item::PolishedBlackstoneBrickStairs => "polished_blackstone_brick_stairs",
            Item::SpruceFence => "spruce_fence",
            Item::StoneSword => "stone_sword",
            Item::MagentaTerracotta => "magenta_terracotta",
            Item::JackOLantern => "jack_o_lantern",
            Item::WoodenHoe => "wooden_hoe",
            Item::DiamondAxe => "diamond_axe",
            Item::RedBed => "red_bed",
            Item::Dirt => "dirt",
            Item::PurpurSlab => "purpur_slab",
            Item::PolishedDeepslateSlab => "polished_deepslate_slab",
            Item::JungleDoor => "jungle_door",
            Item::PoisonousPotato => "poisonous_potato",
            Item::DeadHornCoralFan => "dead_horn_coral_fan",
            Item::HorseSpawnEgg => "horse_spawn_egg",
            Item::TraderLlamaSpawnEgg => "trader_llama_spawn_egg",
            Item::CobblestoneSlab => "cobblestone_slab",
            Item::WitherRose => "wither_rose",
            Item::StrippedOakWood => "stripped_oak_wood",
            Item::GlobeBannerPattern => "globe_banner_pattern",
            Item::GrayDye => "gray_dye",
            Item::ChiseledQuartzBlock => "chiseled_quartz_block",
            Item::DeadBrainCoralBlock => "dead_brain_coral_block",
            Item::AzureBluet => "azure_bluet",
            Item::EmeraldOre => "emerald_ore",
            Item::InfestedCobblestone => "infested_cobblestone",
            Item::WhiteDye => "white_dye",
            Item::CookedCod => "cooked_cod",
            Item::DarkOakSapling => "dark_oak_sapling",
            Item::LightGrayStainedGlassPane => "light_gray_stained_glass_pane",
            Item::PinkGlazedTerracotta => "pink_glazed_terracotta",
            Item::DioriteSlab => "diorite_slab",
            Item::OrangeConcrete => "orange_concrete",
            Item::EnchantedGoldenApple => "enchanted_golden_apple",
            Item::BoneBlock => "bone_block",
            Item::Hopper => "hopper",
            Item::SpruceWood => "spruce_wood",
            Item::SeaPickle => "sea_pickle",
            Item::EnderChest => "ender_chest",
            Item::DiamondLeggings => "diamond_leggings",
            Item::CobbledDeepslateWall => "cobbled_deepslate_wall",
            Item::CyanConcrete => "cyan_concrete",
            Item::BrickWall => "brick_wall",
            Item::LimeBed => "lime_bed",
            Item::SkeletonHorseSpawnEgg => "skeleton_horse_spawn_egg",
            Item::CobbledDeepslateSlab => "cobbled_deepslate_slab",
            Item::WhiteGlazedTerracotta => "white_glazed_terracotta",
            Item::SplashPotion => "splash_potion",
            Item::Bone => "bone",
            Item::AcaciaPressurePlate => "acacia_pressure_plate",
            Item::GrayShulkerBox => "gray_shulker_box",
            Item::LightBlueConcretePowder => "light_blue_concrete_powder",
            Item::WarpedPressurePlate => "warped_pressure_plate",
            Item::Andesite => "andesite",
            Item::TntMinecart => "tnt_minecart",
            Item::BlueTerracotta => "blue_terracotta",
            Item::WoodenAxe => "wooden_axe",
            Item::ExposedCutCopperStairs => "exposed_cut_copper_stairs",
            Item::SpruceTrapdoor => "spruce_trapdoor",
            Item::Potion => "potion",
            Item::CoarseDirt => "coarse_dirt",
            Item::DeepslateTileWall => "deepslate_tile_wall",
            Item::BrownWool => "brown_wool",
            Item::SmoothSandstoneStairs => "smooth_sandstone_stairs",
            Item::Lantern => "lantern",
            Item::BrownTerracotta => "brown_terracotta",
            Item::CatSpawnEgg => "cat_spawn_egg",
            Item::WaxedExposedCutCopperSlab => "waxed_exposed_cut_copper_slab",
            Item::PinkCandle => "pink_candle",
            Item::ExposedCutCopper => "exposed_cut_copper",
            Item::GlowBerries => "glow_berries",
            Item::SnowBlock => "snow_block",
            Item::StoneSlab => "stone_slab",
            Item::DriedKelpBlock => "dried_kelp_block",
            Item::CrimsonFungus => "crimson_fungus",
            Item::OxidizedCutCopperSlab => "oxidized_cut_copper_slab",
            Item::WaxedExposedCutCopper => "waxed_exposed_cut_copper",
            Item::LargeFern => "large_fern",
            Item::GoldBlock => "gold_block",
            Item::Repeater => "repeater",
            Item::GoldenHoe => "golden_hoe",
            Item::BlackWool => "black_wool",
            Item::NetherQuartzOre => "nether_quartz_ore",
            Item::WarpedButton => "warped_button",
            Item::BirchFenceGate => "birch_fence_gate",
            Item::Elytra => "elytra",
            Item::LingeringPotion => "lingering_potion",
            Item::LeatherHelmet => "leather_helmet",
            Item::Barrier => "barrier",
            Item::BrownDye => "brown_dye",
            Item::PolishedDioriteSlab => "polished_diorite_slab",
            Item::ActivatorRail => "activator_rail",
            Item::DrownedSpawnEgg => "drowned_spawn_egg",
            Item::DarkOakLog => "dark_oak_log",
            Item::GlowSquidSpawnEgg => "glow_squid_spawn_egg",
            Item::GreenCandle => "green_candle",
            Item::Tuff => "tuff",
            Item::ChainCommandBlock => "chain_command_block",
            Item::BlazeSpawnEgg => "blaze_spawn_egg",
            Item::DioriteStairs => "diorite_stairs",
            Item::Pumpkin => "pumpkin",
            Item::BlackstoneWall => "blackstone_wall",
            Item::HoglinSpawnEgg => "hoglin_spawn_egg",
            Item::ChainmailHelmet => "chainmail_helmet",
            Item::DarkOakButton => "dark_oak_button",
            Item::YellowWool => "yellow_wool",
            Item::GoldenChestplate => "golden_chestplate",
            Item::QuartzSlab => "quartz_slab",
            Item::Cod => "cod",
            Item::InkSac => "ink_sac",
            Item::CrimsonDoor => "crimson_door",
            Item::Arrow => "arrow",
            Item::PigSpawnEgg => "pig_spawn_egg",
            Item::CyanTerracotta => "cyan_terracotta",
            Item::NetherStar => "nether_star",
            Item::OakSapling => "oak_sapling",
            Item::HoneyBlock => "honey_block",
            Item::CutRedSandstone => "cut_red_sandstone",
            Item::Paper => "paper",
            Item::ExposedCutCopperSlab => "exposed_cut_copper_slab",
            Item::SuspiciousStew => "suspicious_stew",
            Item::FlowerBannerPattern => "flower_banner_pattern",
            Item::RawIron => "raw_iron",
            Item::BlackGlazedTerracotta => "black_glazed_terracotta",
            Item::SheepSpawnEgg => "sheep_spawn_egg",
            Item::BlackBanner => "black_banner",
            Item::Spyglass => "spyglass",
            Item::PinkTulip => "pink_tulip",
            Item::PinkWool => "pink_wool",
            Item::NetheriteScrap => "netherite_scrap",
            Item::DiamondHorseArmor => "diamond_horse_armor",
            Item::ChiseledPolishedBlackstone => "chiseled_polished_blackstone",
            Item::LimeConcrete => "lime_concrete",
            Item::ChiseledNetherBricks => "chiseled_nether_bricks",
            Item::PolishedDeepslateWall => "polished_deepslate_wall",
            Item::GraniteSlab => "granite_slab",
            Item::LightningRod => "lightning_rod",
            Item::MagentaBed => "magenta_bed",
            Item::QuartzBricks => "quartz_bricks",
            Item::SoulSand => "soul_sand",
            Item::RedSandstoneStairs => "red_sandstone_stairs",
            Item::LimeStainedGlassPane => "lime_stained_glass_pane",
            Item::BlackCandle => "black_candle",
            Item::WaxedWeatheredCutCopper => "waxed_weathered_cut_copper",
            Item::RawCopper => "raw_copper",
            Item::DeepslateCopperOre => "deepslate_copper_ore",
            Item::GrayGlazedTerracotta => "gray_glazed_terracotta",
            Item::Cobblestone => "cobblestone",
            Item::GreenBanner => "green_banner",
            Item::LilyOfTheValley => "lily_of_the_valley",
            Item::FloweringAzalea => "flowering_azalea",
            Item::PolishedAndesite => "polished_andesite",
            Item::WetSponge => "wet_sponge",
            Item::Grindstone => "grindstone",
            Item::Wheat => "wheat",
            Item::Beef => "beef",
            Item::WoodenSword => "wooden_sword",
            Item::LightGrayTerracotta => "light_gray_terracotta",
            Item::Diorite => "diorite",
            Item::NetherBrick => "nether_brick",
            Item::BirchButton => "birch_button",
            Item::AcaciaFenceGate => "acacia_fence_gate",
            Item::WolfSpawnEgg => "wolf_spawn_egg",
            Item::CrimsonPressurePlate => "crimson_pressure_plate",
            Item::CobblestoneWall => "cobblestone_wall",
            Item::LightBlueStainedGlass => "light_blue_stained_glass",
            Item::BrownShulkerBox => "brown_shulker_box",
            Item::WaxedWeatheredCopper => "waxed_weathered_copper",
            Item::Conduit => "conduit",
            Item::DiamondPickaxe => "diamond_pickaxe",
            Item::EndStoneBrickSlab => "end_stone_brick_slab",
            Item::IronLeggings => "iron_leggings",
            Item::Flint => "flint",
            Item::WarpedNylium => "warped_nylium",
            Item::BrownCarpet => "brown_carpet",
            Item::StrippedWarpedStem => "stripped_warped_stem",
            Item::WhiteBanner => "white_banner",
            Item::WhiteTerracotta => "white_terracotta",
            Item::CreeperHead => "creeper_head",
            Item::RedShulkerBox => "red_shulker_box",
            Item::WoodenShovel => "wooden_shovel",
            Item::AndesiteStairs => "andesite_stairs",
            Item::JungleSlab => "jungle_slab",
            Item::SpiderSpawnEgg => "spider_spawn_egg",
            Item::MossyCobblestoneStairs => "mossy_cobblestone_stairs",
            Item::QuartzPillar => "quartz_pillar",
            Item::AcaciaFence => "acacia_fence",
            Item::BrainCoralBlock => "brain_coral_block",
            Item::CyanBanner => "cyan_banner",
            Item::BirchSlab => "birch_slab",
            Item::CrimsonSlab => "crimson_slab",
            Item::AmethystBlock => "amethyst_block",
            Item::StoneBrickStairs => "stone_brick_stairs",
            Item::CrackedDeepslateTiles => "cracked_deepslate_tiles",
            Item::RedTulip => "red_tulip",
            Item::Allium => "allium",
            Item::GreenConcretePowder => "green_concrete_powder",
            Item::MusicDiscBlocks => "music_disc_blocks",
            Item::Salmon => "salmon",
            Item::RedTerracotta => "red_terracotta",
            Item::SandstoneStairs => "sandstone_stairs",
            Item::GhastSpawnEgg => "ghast_spawn_egg",
            Item::GrayStainedGlassPane => "gray_stained_glass_pane",
            Item::EnderEye => "ender_eye",
            Item::CookedRabbit => "cooked_rabbit",
            Item::Beehive => "beehive",
            Item::NetheriteHelmet => "netherite_helmet",
            Item::TurtleSpawnEgg => "turtle_spawn_egg",
            Item::LeatherBoots => "leather_boots",
            Item::CrimsonStem => "crimson_stem",
            Item::TubeCoralFan => "tube_coral_fan",
            Item::EndermiteSpawnEgg => "endermite_spawn_egg",
            Item::WitherSkeletonSpawnEgg => "wither_skeleton_spawn_egg",
            Item::PurpleShulkerBox => "purple_shulker_box",
            Item::NetheriteBlock => "netherite_block",
            Item::SprucePressurePlate => "spruce_pressure_plate",
            Item::CyanConcretePowder => "cyan_concrete_powder",
            Item::Egg => "egg",
            Item::PinkStainedGlass => "pink_stained_glass",
            Item::RedBanner => "red_banner",
            Item::JunglePlanks => "jungle_planks",
            Item::RespawnAnchor => "respawn_anchor",
            Item::BatSpawnEgg => "bat_spawn_egg",
            Item::PointedDripstone => "pointed_dripstone",
            Item::Gravel => "gravel",
            Item::Bowl => "bowl",
            Item::CaveSpiderSpawnEgg => "cave_spider_spawn_egg",
            Item::QuartzStairs => "quartz_stairs",
            Item::SpruceButton => "spruce_button",
            Item::CarvedPumpkin => "carved_pumpkin",
            Item::LightBlueShulkerBox => "light_blue_shulker_box",
            Item::BeeSpawnEgg => "bee_spawn_egg",
            Item::OakFenceGate => "oak_fence_gate",
            Item::NetherBrickFence => "nether_brick_fence",
            Item::IronHorseArmor => "iron_horse_armor",
            Item::DeadHornCoral => "dead_horn_coral",
            Item::PolishedDiorite => "polished_diorite",
            Item::RavagerSpawnEgg => "ravager_spawn_egg",
            Item::EndRod => "end_rod",
            Item::MagentaStainedGlass => "magenta_stained_glass",
            Item::ChorusPlant => "chorus_plant",
            Item::OrangeConcretePowder => "orange_concrete_powder",
            Item::DragonEgg => "dragon_egg",
            Item::GreenConcrete => "green_concrete",
            Item::StrippedAcaciaLog => "stripped_acacia_log",
            Item::Diamond => "diamond",
            Item::Ladder => "ladder",
            Item::PrismarineWall => "prismarine_wall",
            Item::MossBlock => "moss_block",
            Item::OakLeaves => "oak_leaves",
            Item::CookedPorkchop => "cooked_porkchop",
            Item::YellowStainedGlassPane => "yellow_stained_glass_pane",
            Item::SalmonSpawnEgg => "salmon_spawn_egg",
            Item::ChainmailBoots => "chainmail_boots",
            Item::WitchSpawnEgg => "witch_spawn_egg",
            Item::OxidizedCutCopperStairs => "oxidized_cut_copper_stairs",
            Item::LapisOre => "lapis_ore",
            Item::SmoothRedSandstone => "smooth_red_sandstone",
            Item::JungleTrapdoor => "jungle_trapdoor",
            Item::Shroomlight => "shroomlight",
            Item::BlackstoneSlab => "blackstone_slab",
            Item::Mutton => "mutton",
            Item::StrippedOakLog => "stripped_oak_log",
            Item::YellowStainedGlass => "yellow_stained_glass",
            Item::CryingObsidian => "crying_obsidian",
            Item::SpruceSlab => "spruce_slab",
            Item::GrayConcrete => "gray_concrete",
            Item::ChorusFruit => "chorus_fruit",
            Item::SilverfishSpawnEgg => "silverfish_spawn_egg",
            Item::CyanShulkerBox => "cyan_shulker_box",
            Item::BlackTerracotta => "black_terracotta",
            Item::RedstoneOre => "redstone_ore",
            Item::RedDye => "red_dye",
            Item::SmoothBasalt => "smooth_basalt",
            Item::PurpleGlazedTerracotta => "purple_glazed_terracotta",
            Item::PoppedChorusFruit => "popped_chorus_fruit",
            Item::DebugStick => "debug_stick",
            Item::WaxedCopperBlock => "waxed_copper_block",
            Item::WarpedRoots => "warped_roots",
            Item::DiamondHelmet => "diamond_helmet",
            Item::BirchPressurePlate => "birch_pressure_plate",
            Item::PinkConcrete => "pink_concrete",
            Item::ChestMinecart => "chest_minecart",
            Item::Torch => "torch",
            Item::AcaciaDoor => "acacia_door",
            Item::DiamondShovel => "diamond_shovel",
            Item::Bamboo => "bamboo",
            Item::IronTrapdoor => "iron_trapdoor",
            Item::DarkOakSlab => "dark_oak_slab",
            Item::BrownBed => "brown_bed",
            Item::RootedDirt => "rooted_dirt",
            Item::AcaciaTrapdoor => "acacia_trapdoor",
            Item::Carrot => "carrot",
            Item::CobbledDeepslate => "cobbled_deepslate",
            Item::CrimsonRoots => "crimson_roots",
            Item::GoldenLeggings => "golden_leggings",
            Item::Grass => "grass",
            Item::MusicDiscPigstep => "music_disc_pigstep",
            Item::CommandBlock => "command_block",
            Item::CutCopperSlab => "cut_copper_slab",
            Item::PinkTerracotta => "pink_terracotta",
            Item::LimeWool => "lime_wool",
            Item::FireCoral => "fire_coral",
            Item::BirchBoat => "birch_boat",
            Item::LightGrayDye => "light_gray_dye",
            Item::Obsidian => "obsidian",
            Item::Spawner => "spawner",
            Item::GreenShulkerBox => "green_shulker_box",
            Item::FilledMap => "filled_map",
            Item::BlueCandle => "blue_candle",
            Item::JungleLog => "jungle_log",
            Item::SkeletonSpawnEgg => "skeleton_spawn_egg",
            Item::SquidSpawnEgg => "squid_spawn_egg",
            Item::BlueGlazedTerracotta => "blue_glazed_terracotta",
            Item::MagentaShulkerBox => "magenta_shulker_box",
            Item::NetherGoldOre => "nether_gold_ore",
            Item::StonePressurePlate => "stone_pressure_plate",
            Item::ItemFrame => "item_frame",
            Item::Lectern => "lectern",
            Item::NetheriteHoe => "netherite_hoe",
            Item::DarkOakFence => "dark_oak_fence",
            Item::WarpedStairs => "warped_stairs",
            Item::TippedArrow => "tipped_arrow",
            Item::CrimsonHyphae => "crimson_hyphae",
            Item::ChorusFlower => "chorus_flower",
            Item::SandstoneSlab => "sandstone_slab",
            Item::BrownGlazedTerracotta => "brown_glazed_terracotta",
            Item::DeadFireCoral => "dead_fire_coral",
            Item::GoldOre => "gold_ore",
            Item::StrippedCrimsonStem => "stripped_crimson_stem",
            Item::StoneBrickWall => "stone_brick_wall",
            Item::DeadTubeCoralBlock => "dead_tube_coral_block",
            Item::NetheriteBoots => "netherite_boots",
            Item::CopperOre => "copper_ore",
            Item::DeadBrainCoralFan => "dead_brain_coral_fan",
            Item::YellowCarpet => "yellow_carpet",
            Item::VillagerSpawnEgg => "villager_spawn_egg",
            Item::ArmorStand => "armor_stand",
            Item::GrayBanner => "gray_banner",
            Item::NoteBlock => "note_block",
            Item::MagentaBanner => "magenta_banner",
            Item::GlisteringMelonSlice => "glistering_melon_slice",
            Item::EvokerSpawnEgg => "evoker_spawn_egg",
            Item::HayBlock => "hay_block",
            Item::BlueConcrete => "blue_concrete",
            Item::CookedBeef => "cooked_beef",
            Item::RabbitStew => "rabbit_stew",
            Item::PackedIce => "packed_ice",
            Item::BirchLeaves => "birch_leaves",
            Item::MossCarpet => "moss_carpet",
            Item::GrayWool => "gray_wool",
            Item::MossyCobblestoneWall => "mossy_cobblestone_wall",
            Item::Lilac => "lilac",
            Item::StoneBrickSlab => "stone_brick_slab",
            Item::CarrotOnAStick => "carrot_on_a_stick",
            Item::CrimsonFenceGate => "crimson_fence_gate",
            Item::InfestedMossyStoneBricks => "infested_mossy_stone_bricks",
            Item::Cobweb => "cobweb",
            Item::TurtleHelmet => "turtle_helmet",
            Item::OakSlab => "oak_slab",
            Item::PolishedGranite => "polished_granite",
            Item::Clay => "clay",
            Item::OrangeWool => "orange_wool",
            Item::Sand => "sand",
            Item::FireCoralBlock => "fire_coral_block",
            Item::Barrel => "barrel",
            Item::Lead => "lead",
            Item::CutCopperStairs => "cut_copper_stairs",
            Item::WaxedCutCopper => "waxed_cut_copper",
            Item::WarpedStem => "warped_stem",
            Item::LightBlueWool => "light_blue_wool",
            Item::DarkOakWood => "dark_oak_wood",
            Item::SugarCane => "sugar_cane",
            Item::CyanDye => "cyan_dye",
            Item::ZombieHead => "zombie_head",
            Item::NetherWart => "nether_wart",
            Item::Feather => "feather",
            Item::IronPickaxe => "iron_pickaxe",
            Item::TotemOfUndying => "totem_of_undying",
            Item::NameTag => "name_tag",
            Item::GlowstoneDust => "glowstone_dust",
            Item::MelonSeeds => "melon_seeds",
            Item::StrippedJungleLog => "stripped_jungle_log",
            Item::BrickStairs => "brick_stairs",
            Item::PurpleWool => "purple_wool",
            Item::MagentaCarpet => "magenta_carpet",
            Item::IronDoor => "iron_door",
            Item::CyanWool => "cyan_wool",
            Item::Terracotta => "terracotta",
            Item::Bread => "bread",
            Item::ClayBall => "clay_ball",
            Item::MusicDiscMall => "music_disc_mall",
            Item::BuddingAmethyst => "budding_amethyst",
            Item::CrimsonSign => "crimson_sign",
            Item::LimeDye => "lime_dye",
            Item::PurpurStairs => "purpur_stairs",
            Item::StructureBlock => "structure_block",
            Item::PlayerHead => "player_head",
            Item::OakTrapdoor => "oak_trapdoor",
            Item::ParrotSpawnEgg => "parrot_spawn_egg",
            Item::AzaleaLeaves => "azalea_leaves",
            Item::YellowBed => "yellow_bed",
            Item::IronIngot => "iron_ingot",
            Item::Redstone => "redstone",
            Item::Beacon => "beacon",
            Item::ChiseledDeepslate => "chiseled_deepslate",
            Item::GrayStainedGlass => "gray_stained_glass",
            Item::IronHoe => "iron_hoe",
            Item::PiglinSpawnEgg => "piglin_spawn_egg",
            Item::ElderGuardianSpawnEgg => "elder_guardian_spawn_egg",
            Item::DarkOakFenceGate => "dark_oak_fence_gate",
        }
    }
    #[doc = "Gets a `Item` by its `name`."]
    #[inline]
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "ancient_debris" => Some(Item::AncientDebris),
            "shulker_shell" => Some(Item::ShulkerShell),
            "quartz_block" => Some(Item::QuartzBlock),
            "cracked_nether_bricks" => Some(Item::CrackedNetherBricks),
            "saddle" => Some(Item::Saddle),
            "golden_axe" => Some(Item::GoldenAxe),
            "experience_bottle" => Some(Item::ExperienceBottle),
            "polished_basalt" => Some(Item::PolishedBasalt),
            "string" => Some(Item::String),
            "orange_stained_glass" => Some(Item::OrangeStainedGlass),
            "phantom_spawn_egg" => Some(Item::PhantomSpawnEgg),
            "rabbit_foot" => Some(Item::RabbitFoot),
            "light_blue_stained_glass_pane" => Some(Item::LightBlueStainedGlassPane),
            "dark_oak_trapdoor" => Some(Item::DarkOakTrapdoor),
            "nether_brick_slab" => Some(Item::NetherBrickSlab),
            "stripped_birch_wood" => Some(Item::StrippedBirchWood),
            "raw_iron_block" => Some(Item::RawIronBlock),
            "polished_blackstone_wall" => Some(Item::PolishedBlackstoneWall),
            "warped_sign" => Some(Item::WarpedSign),
            "diamond_ore" => Some(Item::DiamondOre),
            "warped_fence" => Some(Item::WarpedFence),
            "polished_andesite_stairs" => Some(Item::PolishedAndesiteStairs),
            "netherite_sword" => Some(Item::NetheriteSword),
            "purple_concrete" => Some(Item::PurpleConcrete),
            "structure_void" => Some(Item::StructureVoid),
            "oak_button" => Some(Item::OakButton),
            "scute" => Some(Item::Scute),
            "cobbled_deepslate_stairs" => Some(Item::CobbledDeepslateStairs),
            "bedrock" => Some(Item::Bedrock),
            "medium_amethyst_bud" => Some(Item::MediumAmethystBud),
            "nether_brick_stairs" => Some(Item::NetherBrickStairs),
            "green_bed" => Some(Item::GreenBed),
            "smooth_quartz_stairs" => Some(Item::SmoothQuartzStairs),
            "lime_glazed_terracotta" => Some(Item::LimeGlazedTerracotta),
            "salmon_bucket" => Some(Item::SalmonBucket),
            "blaze_powder" => Some(Item::BlazePowder),
            "piglin_banner_pattern" => Some(Item::PiglinBannerPattern),
            "smooth_quartz" => Some(Item::SmoothQuartz),
            "dried_kelp" => Some(Item::DriedKelp),
            "coal_ore" => Some(Item::CoalOre),
            "dark_oak_planks" => Some(Item::DarkOakPlanks),
            "orange_glazed_terracotta" => Some(Item::OrangeGlazedTerracotta),
            "purple_stained_glass_pane" => Some(Item::PurpleStainedGlassPane),
            "dirt_path" => Some(Item::DirtPath),
            "dead_fire_coral_block" => Some(Item::DeadFireCoralBlock),
            "golden_horse_armor" => Some(Item::GoldenHorseArmor),
            "painting" => Some(Item::Painting),
            "black_concrete" => Some(Item::BlackConcrete),
            "daylight_detector" => Some(Item::DaylightDetector),
            "netherite_pickaxe" => Some(Item::NetheritePickaxe),
            "mooshroom_spawn_egg" => Some(Item::MooshroomSpawnEgg),
            "waxed_oxidized_copper" => Some(Item::WaxedOxidizedCopper),
            "glow_item_frame" => Some(Item::GlowItemFrame),
            "oak_log" => Some(Item::OakLog),
            "anvil" => Some(Item::Anvil),
            "netherite_axe" => Some(Item::NetheriteAxe),
            "golden_helmet" => Some(Item::GoldenHelmet),
            "stonecutter" => Some(Item::Stonecutter),
            "chiseled_stone_bricks" => Some(Item::ChiseledStoneBricks),
            "mossy_cobblestone" => Some(Item::MossyCobblestone),
            "gray_concrete_powder" => Some(Item::GrayConcretePowder),
            "deepslate_brick_slab" => Some(Item::DeepslateBrickSlab),
            "mycelium" => Some(Item::Mycelium),
            "mushroom_stem" => Some(Item::MushroomStem),
            "granite_wall" => Some(Item::GraniteWall),
            "pink_banner" => Some(Item::PinkBanner),
            "zombified_piglin_spawn_egg" => Some(Item::ZombifiedPiglinSpawnEgg),
            "music_disc_wait" => Some(Item::MusicDiscWait),
            "birch_planks" => Some(Item::BirchPlanks),
            "acacia_wood" => Some(Item::AcaciaWood),
            "deepslate_tile_slab" => Some(Item::DeepslateTileSlab),
            "yellow_shulker_box" => Some(Item::YellowShulkerBox),
            "leather_leggings" => Some(Item::LeatherLeggings),
            "dragon_head" => Some(Item::DragonHead),
            "dark_prismarine" => Some(Item::DarkPrismarine),
            "azalea" => Some(Item::Azalea),
            "honeycomb_block" => Some(Item::HoneycombBlock),
            "green_terracotta" => Some(Item::GreenTerracotta),
            "dispenser" => Some(Item::Dispenser),
            "andesite_wall" => Some(Item::AndesiteWall),
            "black_stained_glass_pane" => Some(Item::BlackStainedGlassPane),
            "pink_shulker_box" => Some(Item::PinkShulkerBox),
            "bubble_coral_block" => Some(Item::BubbleCoralBlock),
            "crimson_nylium" => Some(Item::CrimsonNylium),
            "cyan_stained_glass" => Some(Item::CyanStainedGlass),
            "prismarine_brick_slab" => Some(Item::PrismarineBrickSlab),
            "large_amethyst_bud" => Some(Item::LargeAmethystBud),
            "fermented_spider_eye" => Some(Item::FermentedSpiderEye),
            "bubble_coral" => Some(Item::BubbleCoral),
            "pillager_spawn_egg" => Some(Item::PillagerSpawnEgg),
            "golden_boots" => Some(Item::GoldenBoots),
            "blue_bed" => Some(Item::BlueBed),
            "petrified_oak_slab" => Some(Item::PetrifiedOakSlab),
            "magenta_candle" => Some(Item::MagentaCandle),
            "slime_ball" => Some(Item::SlimeBall),
            "tropical_fish_bucket" => Some(Item::TropicalFishBucket),
            "granite_stairs" => Some(Item::GraniteStairs),
            "purple_candle" => Some(Item::PurpleCandle),
            "white_wool" => Some(Item::WhiteWool),
            "blackstone" => Some(Item::Blackstone),
            "poppy" => Some(Item::Poppy),
            "birch_wood" => Some(Item::BirchWood),
            "netherrack" => Some(Item::Netherrack),
            "glass" => Some(Item::Glass),
            "magenta_concrete_powder" => Some(Item::MagentaConcretePowder),
            "yellow_banner" => Some(Item::YellowBanner),
            "brown_banner" => Some(Item::BrownBanner),
            "stray_spawn_egg" => Some(Item::StraySpawnEgg),
            "polished_blackstone_bricks" => Some(Item::PolishedBlackstoneBricks),
            "oxidized_cut_copper" => Some(Item::OxidizedCutCopper),
            "horn_coral" => Some(Item::HornCoral),
            "weathered_cut_copper_slab" => Some(Item::WeatheredCutCopperSlab),
            "small_amethyst_bud" => Some(Item::SmallAmethystBud),
            "copper_ingot" => Some(Item::CopperIngot),
            "target" => Some(Item::Target),
            "nether_sprouts" => Some(Item::NetherSprouts),
            "panda_spawn_egg" => Some(Item::PandaSpawnEgg),
            "blaze_rod" => Some(Item::BlazeRod),
            "birch_log" => Some(Item::BirchLog),
            "purple_terracotta" => Some(Item::PurpleTerracotta),
            "cyan_glazed_terracotta" => Some(Item::CyanGlazedTerracotta),
            "stripped_crimson_hyphae" => Some(Item::StrippedCrimsonHyphae),
            "acacia_button" => Some(Item::AcaciaButton),
            "acacia_log" => Some(Item::AcaciaLog),
            "waxed_exposed_cut_copper_stairs" => Some(Item::WaxedExposedCutCopperStairs),
            "music_disc_far" => Some(Item::MusicDiscFar),
            "lava_bucket" => Some(Item::LavaBucket),
            "waxed_weathered_cut_copper_slab" => Some(Item::WaxedWeatheredCutCopperSlab),
            "chiseled_sandstone" => Some(Item::ChiseledSandstone),
            "infested_stone_bricks" => Some(Item::InfestedStoneBricks),
            "oxeye_daisy" => Some(Item::OxeyeDaisy),
            "cactus" => Some(Item::Cactus),
            "jungle_sign" => Some(Item::JungleSign),
            "waxed_oxidized_cut_copper" => Some(Item::WaxedOxidizedCutCopper),
            "flowering_azalea_leaves" => Some(Item::FloweringAzaleaLeaves),
            "smooth_stone_slab" => Some(Item::SmoothStoneSlab),
            "stripped_dark_oak_wood" => Some(Item::StrippedDarkOakWood),
            "soul_campfire" => Some(Item::SoulCampfire),
            "magenta_concrete" => Some(Item::MagentaConcrete),
            "polished_diorite_stairs" => Some(Item::PolishedDioriteStairs),
            "oak_pressure_plate" => Some(Item::OakPressurePlate),
            "prismarine_shard" => Some(Item::PrismarineShard),
            "mojang_banner_pattern" => Some(Item::MojangBannerPattern),
            "orange_carpet" => Some(Item::OrangeCarpet),
            "stripped_dark_oak_log" => Some(Item::StrippedDarkOakLog),
            "slime_block" => Some(Item::SlimeBlock),
            "stone_pickaxe" => Some(Item::StonePickaxe),
            "heart_of_the_sea" => Some(Item::HeartOfTheSea),
            "quartz" => Some(Item::Quartz),
            "soul_soil" => Some(Item::SoulSoil),
            "cauldron" => Some(Item::Cauldron),
            "repeating_command_block" => Some(Item::RepeatingCommandBlock),
            "wooden_pickaxe" => Some(Item::WoodenPickaxe),
            "glow_lichen" => Some(Item::GlowLichen),
            "spruce_door" => Some(Item::SpruceDoor),
            "music_disc_ward" => Some(Item::MusicDiscWard),
            "soul_torch" => Some(Item::SoulTorch),
            "golden_pickaxe" => Some(Item::GoldenPickaxe),
            "lime_carpet" => Some(Item::LimeCarpet),
            "andesite_slab" => Some(Item::AndesiteSlab),
            "composter" => Some(Item::Composter),
            "weathered_cut_copper" => Some(Item::WeatheredCutCopper),
            "lime_shulker_box" => Some(Item::LimeShulkerBox),
            "stripped_acacia_wood" => Some(Item::StrippedAcaciaWood),
            "sponge" => Some(Item::Sponge),
            "stripped_spruce_wood" => Some(Item::StrippedSpruceWood),
            "snow" => Some(Item::Snow),
            "yellow_terracotta" => Some(Item::YellowTerracotta),
            "acacia_boat" => Some(Item::AcaciaBoat),
            "strider_spawn_egg" => Some(Item::StriderSpawnEgg),
            "light_gray_candle" => Some(Item::LightGrayCandle),
            "iron_boots" => Some(Item::IronBoots),
            "turtle_egg" => Some(Item::TurtleEgg),
            "mossy_stone_brick_stairs" => Some(Item::MossyStoneBrickStairs),
            "waxed_cut_copper_slab" => Some(Item::WaxedCutCopperSlab),
            "polished_granite_stairs" => Some(Item::PolishedGraniteStairs),
            "infested_chiseled_stone_bricks" => Some(Item::InfestedChiseledStoneBricks),
            "oxidized_copper" => Some(Item::OxidizedCopper),
            "cracked_stone_bricks" => Some(Item::CrackedStoneBricks),
            "scaffolding" => Some(Item::Scaffolding),
            "cod_bucket" => Some(Item::CodBucket),
            "husk_spawn_egg" => Some(Item::HuskSpawnEgg),
            "stripped_birch_log" => Some(Item::StrippedBirchLog),
            "lime_concrete_powder" => Some(Item::LimeConcretePowder),
            "creeper_spawn_egg" => Some(Item::CreeperSpawnEgg),
            "goat_spawn_egg" => Some(Item::GoatSpawnEgg),
            "spruce_leaves" => Some(Item::SpruceLeaves),
            "sugar" => Some(Item::Sugar),
            "cod_spawn_egg" => Some(Item::CodSpawnEgg),
            "rabbit_hide" => Some(Item::RabbitHide),
            "music_disc_otherside" => Some(Item::MusicDiscOtherside),
            "cyan_candle" => Some(Item::CyanCandle),
            "dark_prismarine_slab" => Some(Item::DarkPrismarineSlab),
            "skull_banner_pattern" => Some(Item::SkullBannerPattern),
            "bricks" => Some(Item::Bricks),
            "shield" => Some(Item::Shield),
            "llama_spawn_egg" => Some(Item::LlamaSpawnEgg),
            "grass_block" => Some(Item::GrassBlock),
            "book" => Some(Item::Book),
            "bell" => Some(Item::Bell),
            "red_nether_brick_wall" => Some(Item::RedNetherBrickWall),
            "black_bed" => Some(Item::BlackBed),
            "jungle_leaves" => Some(Item::JungleLeaves),
            "dead_fire_coral_fan" => Some(Item::DeadFireCoralFan),
            "tropical_fish" => Some(Item::TropicalFish),
            "bone_meal" => Some(Item::BoneMeal),
            "orange_bed" => Some(Item::OrangeBed),
            "chest" => Some(Item::Chest),
            "red_carpet" => Some(Item::RedCarpet),
            "light_blue_concrete" => Some(Item::LightBlueConcrete),
            "weeping_vines" => Some(Item::WeepingVines),
            "crossbow" => Some(Item::Crossbow),
            "tinted_glass" => Some(Item::TintedGlass),
            "lime_banner" => Some(Item::LimeBanner),
            "crimson_button" => Some(Item::CrimsonButton),
            "prismarine_bricks" => Some(Item::PrismarineBricks),
            "stone_hoe" => Some(Item::StoneHoe),
            "black_concrete_powder" => Some(Item::BlackConcretePowder),
            "porkchop" => Some(Item::Porkchop),
            "weathered_cut_copper_stairs" => Some(Item::WeatheredCutCopperStairs),
            "purple_carpet" => Some(Item::PurpleCarpet),
            "fire_charge" => Some(Item::FireCharge),
            "mossy_stone_brick_wall" => Some(Item::MossyStoneBrickWall),
            "magenta_stained_glass_pane" => Some(Item::MagentaStainedGlassPane),
            "red_concrete_powder" => Some(Item::RedConcretePowder),
            "acacia_sign" => Some(Item::AcaciaSign),
            "light_gray_wool" => Some(Item::LightGrayWool),
            "purpur_block" => Some(Item::PurpurBlock),
            "golden_sword" => Some(Item::GoldenSword),
            "purple_dye" => Some(Item::PurpleDye),
            "nether_bricks" => Some(Item::NetherBricks),
            "light_blue_bed" => Some(Item::LightBlueBed),
            "dark_oak_leaves" => Some(Item::DarkOakLeaves),
            "zombie_spawn_egg" => Some(Item::ZombieSpawnEgg),
            "music_disc_mellohi" => Some(Item::MusicDiscMellohi),
            "detector_rail" => Some(Item::DetectorRail),
            "white_bed" => Some(Item::WhiteBed),
            "minecart" => Some(Item::Minecart),
            "cartography_table" => Some(Item::CartographyTable),
            "deepslate_iron_ore" => Some(Item::DeepslateIronOre),
            "dark_oak_pressure_plate" => Some(Item::DarkOakPressurePlate),
            "cracked_deepslate_bricks" => Some(Item::CrackedDeepslateBricks),
            "black_stained_glass" => Some(Item::BlackStainedGlass),
            "iron_chestplate" => Some(Item::IronChestplate),
            "chicken" => Some(Item::Chicken),
            "smooth_red_sandstone_stairs" => Some(Item::SmoothRedSandstoneStairs),
            "warped_fungus" => Some(Item::WarpedFungus),
            "music_disc_13" => Some(Item::MusicDisc13),
            "deepslate_brick_stairs" => Some(Item::DeepslateBrickStairs),
            "tall_grass" => Some(Item::TallGrass),
            "magenta_dye" => Some(Item::MagentaDye),
            "dandelion" => Some(Item::Dandelion),
            "written_book" => Some(Item::WrittenBook),
            "jungle_fence" => Some(Item::JungleFence),
            "oak_planks" => Some(Item::OakPlanks),
            "glass_bottle" => Some(Item::GlassBottle),
            "light_blue_glazed_terracotta" => Some(Item::LightBlueGlazedTerracotta),
            "redstone_torch" => Some(Item::RedstoneTorch),
            "red_mushroom_block" => Some(Item::RedMushroomBlock),
            "dead_bubble_coral_block" => Some(Item::DeadBubbleCoralBlock),
            "bookshelf" => Some(Item::Bookshelf),
            "diamond_hoe" => Some(Item::DiamondHoe),
            "deepslate_brick_wall" => Some(Item::DeepslateBrickWall),
            "light" => Some(Item::Light),
            "water_bucket" => Some(Item::WaterBucket),
            "deepslate_emerald_ore" => Some(Item::DeepslateEmeraldOre),
            "dead_bubble_coral_fan" => Some(Item::DeadBubbleCoralFan),
            "guardian_spawn_egg" => Some(Item::GuardianSpawnEgg),
            "mule_spawn_egg" => Some(Item::MuleSpawnEgg),
            "raw_copper_block" => Some(Item::RawCopperBlock),
            "jungle_wood" => Some(Item::JungleWood),
            "red_stained_glass" => Some(Item::RedStainedGlass),
            "dead_bubble_coral" => Some(Item::DeadBubbleCoral),
            "potato" => Some(Item::Potato),
            "compass" => Some(Item::Compass),
            "pink_dye" => Some(Item::PinkDye),
            "observer" => Some(Item::Observer),
            "magenta_glazed_terracotta" => Some(Item::MagentaGlazedTerracotta),
            "dead_bush" => Some(Item::DeadBush),
            "polished_blackstone_slab" => Some(Item::PolishedBlackstoneSlab),
            "vindicator_spawn_egg" => Some(Item::VindicatorSpawnEgg),
            "chainmail_leggings" => Some(Item::ChainmailLeggings),
            "dark_oak_door" => Some(Item::DarkOakDoor),
            "blue_orchid" => Some(Item::BlueOrchid),
            "brown_mushroom_block" => Some(Item::BrownMushroomBlock),
            "red_sandstone_wall" => Some(Item::RedSandstoneWall),
            "white_stained_glass" => Some(Item::WhiteStainedGlass),
            "beetroot" => Some(Item::Beetroot),
            "rail" => Some(Item::Rail),
            "phantom_membrane" => Some(Item::PhantomMembrane),
            "white_shulker_box" => Some(Item::WhiteShulkerBox),
            "blue_banner" => Some(Item::BlueBanner),
            "farmland" => Some(Item::Farmland),
            "red_nether_brick_slab" => Some(Item::RedNetherBrickSlab),
            "polished_andesite_slab" => Some(Item::PolishedAndesiteSlab),
            "blue_stained_glass" => Some(Item::BlueStainedGlass),
            "dragon_breath" => Some(Item::DragonBreath),
            "writable_book" => Some(Item::WritableBook),
            "green_glazed_terracotta" => Some(Item::GreenGlazedTerracotta),
            "deepslate_redstone_ore" => Some(Item::DeepslateRedstoneOre),
            "prismarine" => Some(Item::Prismarine),
            "purple_bed" => Some(Item::PurpleBed),
            "blast_furnace" => Some(Item::BlastFurnace),
            "brick_slab" => Some(Item::BrickSlab),
            "green_stained_glass_pane" => Some(Item::GreenStainedGlassPane),
            "chiseled_red_sandstone" => Some(Item::ChiseledRedSandstone),
            "mossy_cobblestone_slab" => Some(Item::MossyCobblestoneSlab),
            "black_dye" => Some(Item::BlackDye),
            "deepslate_bricks" => Some(Item::DeepslateBricks),
            "heavy_weighted_pressure_plate" => Some(Item::HeavyWeightedPressurePlate),
            "beetroot_seeds" => Some(Item::BeetrootSeeds),
            "polished_granite_slab" => Some(Item::PolishedGraniteSlab),
            "bow" => Some(Item::Bow),
            "light_gray_glazed_terracotta" => Some(Item::LightGrayGlazedTerracotta),
            "tripwire_hook" => Some(Item::TripwireHook),
            "iron_nugget" => Some(Item::IronNugget),
            "granite" => Some(Item::Granite),
            "iron_block" => Some(Item::IronBlock),
            "warped_slab" => Some(Item::WarpedSlab),
            "small_dripleaf" => Some(Item::SmallDripleaf),
            "polished_deepslate" => Some(Item::PolishedDeepslate),
            "red_wool" => Some(Item::RedWool),
            "oak_boat" => Some(Item::OakBoat),
            "white_carpet" => Some(Item::WhiteCarpet),
            "orange_dye" => Some(Item::OrangeDye),
            "smooth_quartz_slab" => Some(Item::SmoothQuartzSlab),
            "cow_spawn_egg" => Some(Item::CowSpawnEgg),
            "sea_lantern" => Some(Item::SeaLantern),
            "light_weighted_pressure_plate" => Some(Item::LightWeightedPressurePlate),
            "rose_bush" => Some(Item::RoseBush),
            "pink_bed" => Some(Item::PinkBed),
            "cooked_mutton" => Some(Item::CookedMutton),
            "deepslate_lapis_ore" => Some(Item::DeepslateLapisOre),
            "seagrass" => Some(Item::Seagrass),
            "polished_blackstone_button" => Some(Item::PolishedBlackstoneButton),
            "cocoa_beans" => Some(Item::CocoaBeans),
            "exposed_copper" => Some(Item::ExposedCopper),
            "polished_deepslate_stairs" => Some(Item::PolishedDeepslateStairs),
            "golden_apple" => Some(Item::GoldenApple),
            "clock" => Some(Item::Clock),
            "purple_stained_glass" => Some(Item::PurpleStainedGlass),
            "knowledge_book" => Some(Item::KnowledgeBook),
            "coal" => Some(Item::Coal),
            "emerald_block" => Some(Item::EmeraldBlock),
            "powder_snow_bucket" => Some(Item::PowderSnowBucket),
            "light_blue_banner" => Some(Item::LightBlueBanner),
            "cut_copper" => Some(Item::CutCopper),
            "crimson_planks" => Some(Item::CrimsonPlanks),
            "jungle_fence_gate" => Some(Item::JungleFenceGate),
            "candle" => Some(Item::Candle),
            "polished_blackstone_stairs" => Some(Item::PolishedBlackstoneStairs),
            "horn_coral_fan" => Some(Item::HornCoralFan),
            "weathered_copper" => Some(Item::WeatheredCopper),
            "warped_planks" => Some(Item::WarpedPlanks),
            "end_stone_brick_wall" => Some(Item::EndStoneBrickWall),
            "jungle_button" => Some(Item::JungleButton),
            "deepslate_tiles" => Some(Item::DeepslateTiles),
            "ice" => Some(Item::Ice),
            "cut_red_sandstone_slab" => Some(Item::CutRedSandstoneSlab),
            "lime_candle" => Some(Item::LimeCandle),
            "soul_lantern" => Some(Item::SoulLantern),
            "tnt" => Some(Item::Tnt),
            "acacia_planks" => Some(Item::AcaciaPlanks),
            "wheat_seeds" => Some(Item::WheatSeeds),
            "stone_stairs" => Some(Item::StoneStairs),
            "magma_cube_spawn_egg" => Some(Item::MagmaCubeSpawnEgg),
            "lime_terracotta" => Some(Item::LimeTerracotta),
            "light_gray_bed" => Some(Item::LightGrayBed),
            "mushroom_stew" => Some(Item::MushroomStew),
            "red_sandstone" => Some(Item::RedSandstone),
            "spectral_arrow" => Some(Item::SpectralArrow),
            "music_disc_strad" => Some(Item::MusicDiscStrad),
            "emerald" => Some(Item::Emerald),
            "glass_pane" => Some(Item::GlassPane),
            "polished_blackstone_pressure_plate" => Some(Item::PolishedBlackstonePressurePlate),
            "stick" => Some(Item::Stick),
            "light_gray_shulker_box" => Some(Item::LightGrayShulkerBox),
            "stone_bricks" => Some(Item::StoneBricks),
            "vine" => Some(Item::Vine),
            "comparator" => Some(Item::Comparator),
            "end_crystal" => Some(Item::EndCrystal),
            "deepslate_coal_ore" => Some(Item::DeepslateCoalOre),
            "smoker" => Some(Item::Smoker),
            "deepslate_diamond_ore" => Some(Item::DeepslateDiamondOre),
            "lime_stained_glass" => Some(Item::LimeStainedGlass),
            "melon" => Some(Item::Melon),
            "iron_axe" => Some(Item::IronAxe),
            "waxed_oxidized_cut_copper_stairs" => Some(Item::WaxedOxidizedCutCopperStairs),
            "red_nether_brick_stairs" => Some(Item::RedNetherBrickStairs),
            "lapis_lazuli" => Some(Item::LapisLazuli),
            "blackstone_stairs" => Some(Item::BlackstoneStairs),
            "ocelot_spawn_egg" => Some(Item::OcelotSpawnEgg),
            "red_concrete" => Some(Item::RedConcrete),
            "dripstone_block" => Some(Item::DripstoneBlock),
            "infested_deepslate" => Some(Item::InfestedDeepslate),
            "shulker_spawn_egg" => Some(Item::ShulkerSpawnEgg),
            "jungle_sapling" => Some(Item::JungleSapling),
            "spider_eye" => Some(Item::SpiderEye),
            "yellow_candle" => Some(Item::YellowCandle),
            "deepslate" => Some(Item::Deepslate),
            "fern" => Some(Item::Fern),
            "brown_concrete_powder" => Some(Item::BrownConcretePowder),
            "deepslate_tile_stairs" => Some(Item::DeepslateTileStairs),
            "cookie" => Some(Item::Cookie),
            "enchanted_book" => Some(Item::EnchantedBook),
            "cut_sandstone" => Some(Item::CutSandstone),
            "lily_pad" => Some(Item::LilyPad),
            "cornflower" => Some(Item::Cornflower),
            "nether_brick_wall" => Some(Item::NetherBrickWall),
            "loom" => Some(Item::Loom),
            "jungle_boat" => Some(Item::JungleBoat),
            "yellow_dye" => Some(Item::YellowDye),
            "oak_fence" => Some(Item::OakFence),
            "flint_and_steel" => Some(Item::FlintAndSteel),
            "green_stained_glass" => Some(Item::GreenStainedGlass),
            "lodestone" => Some(Item::Lodestone),
            "spore_blossom" => Some(Item::SporeBlossom),
            "yellow_glazed_terracotta" => Some(Item::YellowGlazedTerracotta),
            "polished_blackstone_brick_wall" => Some(Item::PolishedBlackstoneBrickWall),
            "diamond_boots" => Some(Item::DiamondBoots),
            "hanging_roots" => Some(Item::HangingRoots),
            "blue_ice" => Some(Item::BlueIce),
            "milk_bucket" => Some(Item::MilkBucket),
            "diamond_block" => Some(Item::DiamondBlock),
            "zombie_villager_spawn_egg" => Some(Item::ZombieVillagerSpawnEgg),
            "golden_carrot" => Some(Item::GoldenCarrot),
            "pufferfish" => Some(Item::Pufferfish),
            "light_gray_concrete_powder" => Some(Item::LightGrayConcretePowder),
            "orange_terracotta" => Some(Item::OrangeTerracotta),
            "deepslate_gold_ore" => Some(Item::DeepslateGoldOre),
            "prismarine_brick_stairs" => Some(Item::PrismarineBrickStairs),
            "birch_sign" => Some(Item::BirchSign),
            "melon_slice" => Some(Item::MelonSlice),
            "cooked_chicken" => Some(Item::CookedChicken),
            "polar_bear_spawn_egg" => Some(Item::PolarBearSpawnEgg),
            "brown_candle" => Some(Item::BrownCandle),
            "birch_fence" => Some(Item::BirchFence),
            "end_stone" => Some(Item::EndStone),
            "cyan_bed" => Some(Item::CyanBed),
            "orange_stained_glass_pane" => Some(Item::OrangeStainedGlassPane),
            "dead_tube_coral" => Some(Item::DeadTubeCoral),
            "warped_door" => Some(Item::WarpedDoor),
            "birch_stairs" => Some(Item::BirchStairs),
            "end_stone_brick_stairs" => Some(Item::EndStoneBrickStairs),
            "gunpowder" => Some(Item::Gunpowder),
            "beetroot_soup" => Some(Item::BeetrootSoup),
            "green_dye" => Some(Item::GreenDye),
            "light_blue_candle" => Some(Item::LightBlueCandle),
            "apple" => Some(Item::Apple),
            "skeleton_skull" => Some(Item::SkeletonSkull),
            "brain_coral_fan" => Some(Item::BrainCoralFan),
            "light_gray_carpet" => Some(Item::LightGrayCarpet),
            "spruce_planks" => Some(Item::SprucePlanks),
            "snowball" => Some(Item::Snowball),
            "piston" => Some(Item::Piston),
            "spruce_sign" => Some(Item::SpruceSign),
            "light_gray_concrete" => Some(Item::LightGrayConcrete),
            "stone_axe" => Some(Item::StoneAxe),
            "music_disc_chirp" => Some(Item::MusicDiscChirp),
            "ender_pearl" => Some(Item::EnderPearl),
            "gold_nugget" => Some(Item::GoldNugget),
            "waxed_exposed_copper" => Some(Item::WaxedExposedCopper),
            "golden_shovel" => Some(Item::GoldenShovel),
            "twisting_vines" => Some(Item::TwistingVines),
            "piglin_brute_spawn_egg" => Some(Item::PiglinBruteSpawnEgg),
            "lever" => Some(Item::Lever),
            "pumpkin_seeds" => Some(Item::PumpkinSeeds),
            "blue_dye" => Some(Item::BlueDye),
            "peony" => Some(Item::Peony),
            "cake" => Some(Item::Cake),
            "orange_banner" => Some(Item::OrangeBanner),
            "music_disc_cat" => Some(Item::MusicDiscCat),
            "smooth_stone" => Some(Item::SmoothStone),
            "brain_coral" => Some(Item::BrainCoral),
            "oak_door" => Some(Item::OakDoor),
            "tube_coral" => Some(Item::TubeCoral),
            "furnace_minecart" => Some(Item::FurnaceMinecart),
            "leather" => Some(Item::Leather),
            "nautilus_shell" => Some(Item::NautilusShell),
            "kelp" => Some(Item::Kelp),
            "sandstone_wall" => Some(Item::SandstoneWall),
            "raw_gold" => Some(Item::RawGold),
            "pumpkin_pie" => Some(Item::PumpkinPie),
            "acacia_sapling" => Some(Item::AcaciaSapling),
            "gray_carpet" => Some(Item::GrayCarpet),
            "redstone_block" => Some(Item::RedstoneBlock),
            "mossy_stone_brick_slab" => Some(Item::MossyStoneBrickSlab),
            "powered_rail" => Some(Item::PoweredRail),
            "light_gray_stained_glass" => Some(Item::LightGrayStainedGlass),
            "netherite_chestplate" => Some(Item::NetheriteChestplate),
            "light_blue_terracotta" => Some(Item::LightBlueTerracotta),
            "baked_potato" => Some(Item::BakedPotato),
            "purple_concrete_powder" => Some(Item::PurpleConcretePowder),
            "stone" => Some(Item::Stone),
            "smithing_table" => Some(Item::SmithingTable),
            "trapped_chest" => Some(Item::TrappedChest),
            "gray_terracotta" => Some(Item::GrayTerracotta),
            "waxed_oxidized_cut_copper_slab" => Some(Item::WaxedOxidizedCutCopperSlab),
            "gilded_blackstone" => Some(Item::GildedBlackstone),
            "tropical_fish_spawn_egg" => Some(Item::TropicalFishSpawnEgg),
            "acacia_leaves" => Some(Item::AcaciaLeaves),
            "black_carpet" => Some(Item::BlackCarpet),
            "gold_ingot" => Some(Item::GoldIngot),
            "command_block_minecart" => Some(Item::CommandBlockMinecart),
            "bubble_coral_fan" => Some(Item::BubbleCoralFan),
            "music_disc_11" => Some(Item::MusicDisc11),
            "purple_banner" => Some(Item::PurpleBanner),
            "polished_blackstone" => Some(Item::PolishedBlackstone),
            "red_glazed_terracotta" => Some(Item::RedGlazedTerracotta),
            "blue_stained_glass_pane" => Some(Item::BlueStainedGlassPane),
            "magma_block" => Some(Item::MagmaBlock),
            "podzol" => Some(Item::Podzol),
            "sticky_piston" => Some(Item::StickyPiston),
            "ghast_tear" => Some(Item::GhastTear),
            "big_dripleaf" => Some(Item::BigDripleaf),
            "blue_carpet" => Some(Item::BlueCarpet),
            "cyan_stained_glass_pane" => Some(Item::CyanStainedGlassPane),
            "yellow_concrete_powder" => Some(Item::YellowConcretePowder),
            "diamond_chestplate" => Some(Item::DiamondChestplate),
            "zombie_horse_spawn_egg" => Some(Item::ZombieHorseSpawnEgg),
            "warped_fence_gate" => Some(Item::WarpedFenceGate),
            "sweet_berries" => Some(Item::SweetBerries),
            "stripped_spruce_log" => Some(Item::StrippedSpruceLog),
            "coal_block" => Some(Item::CoalBlock),
            "red_sandstone_slab" => Some(Item::RedSandstoneSlab),
            "dark_oak_boat" => Some(Item::DarkOakBoat),
            "bundle" => Some(Item::Bundle),
            "netherite_ingot" => Some(Item::NetheriteIngot),
            "crimson_stairs" => Some(Item::CrimsonStairs),
            "white_concrete_powder" => Some(Item::WhiteConcretePowder),
            "pufferfish_bucket" => Some(Item::PufferfishBucket),
            "cobblestone_stairs" => Some(Item::CobblestoneStairs),
            "warped_wart_block" => Some(Item::WarpedWartBlock),
            "chicken_spawn_egg" => Some(Item::ChickenSpawnEgg),
            "orange_tulip" => Some(Item::OrangeTulip),
            "glowstone" => Some(Item::Glowstone),
            "chain" => Some(Item::Chain),
            "birch_trapdoor" => Some(Item::BirchTrapdoor),
            "magma_cream" => Some(Item::MagmaCream),
            "axolotl_spawn_egg" => Some(Item::AxolotlSpawnEgg),
            "stone_button" => Some(Item::StoneButton),
            "copper_block" => Some(Item::CopperBlock),
            "sculk_sensor" => Some(Item::SculkSensor),
            "oak_stairs" => Some(Item::OakStairs),
            "birch_door" => Some(Item::BirchDoor),
            "pink_carpet" => Some(Item::PinkCarpet),
            "stripped_warped_hyphae" => Some(Item::StrippedWarpedHyphae),
            "birch_sapling" => Some(Item::BirchSapling),
            "pink_concrete_powder" => Some(Item::PinkConcretePowder),
            "jungle_stairs" => Some(Item::JungleStairs),
            "blue_concrete_powder" => Some(Item::BlueConcretePowder),
            "polished_blackstone_brick_slab" => Some(Item::PolishedBlackstoneBrickSlab),
            "flower_pot" => Some(Item::FlowerPot),
            "jigsaw" => Some(Item::Jigsaw),
            "light_gray_banner" => Some(Item::LightGrayBanner),
            "zoglin_spawn_egg" => Some(Item::ZoglinSpawnEgg),
            "creeper_banner_pattern" => Some(Item::CreeperBannerPattern),
            "lapis_block" => Some(Item::LapisBlock),
            "amethyst_shard" => Some(Item::AmethystShard),
            "pufferfish_spawn_egg" => Some(Item::PufferfishSpawnEgg),
            "purpur_pillar" => Some(Item::PurpurPillar),
            "furnace" => Some(Item::Furnace),
            "oak_wood" => Some(Item::OakWood),
            "cut_sandstone_slab" => Some(Item::CutSandstoneSlab),
            "blue_shulker_box" => Some(Item::BlueShulkerBox),
            "rotten_flesh" => Some(Item::RottenFlesh),
            "waxed_weathered_cut_copper_stairs" => Some(Item::WaxedWeatheredCutCopperStairs),
            "spruce_log" => Some(Item::SpruceLog),
            "netherite_leggings" => Some(Item::NetheriteLeggings),
            "shulker_box" => Some(Item::ShulkerBox),
            "jungle_pressure_plate" => Some(Item::JunglePressurePlate),
            "diamond_sword" => Some(Item::DiamondSword),
            "red_mushroom" => Some(Item::RedMushroom),
            "hopper_minecart" => Some(Item::HopperMinecart),
            "wandering_trader_spawn_egg" => Some(Item::WanderingTraderSpawnEgg),
            "map" => Some(Item::Map),
            "enderman_spawn_egg" => Some(Item::EndermanSpawnEgg),
            "shears" => Some(Item::Shears),
            "red_sand" => Some(Item::RedSand),
            "white_tulip" => Some(Item::WhiteTulip),
            "brown_stained_glass" => Some(Item::BrownStainedGlass),
            "leather_chestplate" => Some(Item::LeatherChestplate),
            "crafting_table" => Some(Item::CraftingTable),
            "dark_oak_sign" => Some(Item::DarkOakSign),
            "campfire" => Some(Item::Campfire),
            "donkey_spawn_egg" => Some(Item::DonkeySpawnEgg),
            "trident" => Some(Item::Trident),
            "fire_coral_fan" => Some(Item::FireCoralFan),
            "gray_candle" => Some(Item::GrayCandle),
            "brown_concrete" => Some(Item::BrownConcrete),
            "netherite_shovel" => Some(Item::NetheriteShovel),
            "prismarine_stairs" => Some(Item::PrismarineStairs),
            "amethyst_cluster" => Some(Item::AmethystCluster),
            "infested_cracked_stone_bricks" => Some(Item::InfestedCrackedStoneBricks),
            "mossy_stone_bricks" => Some(Item::MossyStoneBricks),
            "brewing_stand" => Some(Item::BrewingStand),
            "smooth_sandstone" => Some(Item::SmoothSandstone),
            "raw_gold_block" => Some(Item::RawGoldBlock),
            "brown_stained_glass_pane" => Some(Item::BrownStainedGlassPane),
            "green_carpet" => Some(Item::GreenCarpet),
            "stone_shovel" => Some(Item::StoneShovel),
            "cracked_polished_blackstone_bricks" => Some(Item::CrackedPolishedBlackstoneBricks),
            "yellow_concrete" => Some(Item::YellowConcrete),
            "magenta_wool" => Some(Item::MagentaWool),
            "white_stained_glass_pane" => Some(Item::WhiteStainedGlassPane),
            "sandstone" => Some(Item::Sandstone),
            "green_wool" => Some(Item::GreenWool),
            "slime_spawn_egg" => Some(Item::SlimeSpawnEgg),
            "smooth_sandstone_slab" => Some(Item::SmoothSandstoneSlab),
            "enchanting_table" => Some(Item::EnchantingTable),
            "firework_rocket" => Some(Item::FireworkRocket),
            "smooth_red_sandstone_slab" => Some(Item::SmoothRedSandstoneSlab),
            "bucket" => Some(Item::Bucket),
            "waxed_cut_copper_stairs" => Some(Item::WaxedCutCopperStairs),
            "chainmail_chestplate" => Some(Item::ChainmailChestplate),
            "end_portal_frame" => Some(Item::EndPortalFrame),
            "white_concrete" => Some(Item::WhiteConcrete),
            "crimson_trapdoor" => Some(Item::CrimsonTrapdoor),
            "iron_ore" => Some(Item::IronOre),
            "redstone_lamp" => Some(Item::RedstoneLamp),
            "red_nether_bricks" => Some(Item::RedNetherBricks),
            "vex_spawn_egg" => Some(Item::VexSpawnEgg),
            "prismarine_slab" => Some(Item::PrismarineSlab),
            "end_stone_bricks" => Some(Item::EndStoneBricks),
            "fox_spawn_egg" => Some(Item::FoxSpawnEgg),
            "basalt" => Some(Item::Basalt),
            "orange_shulker_box" => Some(Item::OrangeShulkerBox),
            "charcoal" => Some(Item::Charcoal),
            "black_shulker_box" => Some(Item::BlackShulkerBox),
            "gray_bed" => Some(Item::GrayBed),
            "calcite" => Some(Item::Calcite),
            "prismarine_crystals" => Some(Item::PrismarineCrystals),
            "warped_hyphae" => Some(Item::WarpedHyphae),
            "crimson_fence" => Some(Item::CrimsonFence),
            "light_blue_carpet" => Some(Item::LightBlueCarpet),
            "light_blue_dye" => Some(Item::LightBlueDye),
            "blue_wool" => Some(Item::BlueWool),
            "nether_wart_block" => Some(Item::NetherWartBlock),
            "red_candle" => Some(Item::RedCandle),
            "spruce_fence_gate" => Some(Item::SpruceFenceGate),
            "leather_horse_armor" => Some(Item::LeatherHorseArmor),
            "honey_bottle" => Some(Item::HoneyBottle),
            "pink_stained_glass_pane" => Some(Item::PinkStainedGlassPane),
            "infested_stone" => Some(Item::InfestedStone),
            "dolphin_spawn_egg" => Some(Item::DolphinSpawnEgg),
            "dark_oak_stairs" => Some(Item::DarkOakStairs),
            "damaged_anvil" => Some(Item::DamagedAnvil),
            "sunflower" => Some(Item::Sunflower),
            "glow_ink_sac" => Some(Item::GlowInkSac),
            "acacia_slab" => Some(Item::AcaciaSlab),
            "white_candle" => Some(Item::WhiteCandle),
            "orange_candle" => Some(Item::OrangeCandle),
            "warped_trapdoor" => Some(Item::WarpedTrapdoor),
            "stripped_jungle_wood" => Some(Item::StrippedJungleWood),
            "dead_horn_coral_block" => Some(Item::DeadHornCoralBlock),
            "spruce_sapling" => Some(Item::SpruceSapling),
            "bee_nest" => Some(Item::BeeNest),
            "chipped_anvil" => Some(Item::ChippedAnvil),
            "dead_brain_coral" => Some(Item::DeadBrainCoral),
            "spruce_boat" => Some(Item::SpruceBoat),
            "iron_sword" => Some(Item::IronSword),
            "iron_helmet" => Some(Item::IronHelmet),
            "dead_tube_coral_fan" => Some(Item::DeadTubeCoralFan),
            "fletching_table" => Some(Item::FletchingTable),
            "cooked_salmon" => Some(Item::CookedSalmon),
            "dark_prismarine_stairs" => Some(Item::DarkPrismarineStairs),
            "brick" => Some(Item::Brick),
            "axolotl_bucket" => Some(Item::AxolotlBucket),
            "brown_mushroom" => Some(Item::BrownMushroom),
            "fishing_rod" => Some(Item::FishingRod),
            "diorite_wall" => Some(Item::DioriteWall),
            "honeycomb" => Some(Item::Honeycomb),
            "red_stained_glass_pane" => Some(Item::RedStainedGlassPane),
            "rabbit_spawn_egg" => Some(Item::RabbitSpawnEgg),
            "wither_skeleton_skull" => Some(Item::WitherSkeletonSkull),
            "warped_fungus_on_a_stick" => Some(Item::WarpedFungusOnAStick),
            "iron_bars" => Some(Item::IronBars),
            "horn_coral_block" => Some(Item::HornCoralBlock),
            "iron_shovel" => Some(Item::IronShovel),
            "acacia_stairs" => Some(Item::AcaciaStairs),
            "spruce_stairs" => Some(Item::SpruceStairs),
            "tube_coral_block" => Some(Item::TubeCoralBlock),
            "dropper" => Some(Item::Dropper),
            "cyan_carpet" => Some(Item::CyanCarpet),
            "oak_sign" => Some(Item::OakSign),
            "jukebox" => Some(Item::Jukebox),
            "firework_star" => Some(Item::FireworkStar),
            "rabbit" => Some(Item::Rabbit),
            "music_disc_stal" => Some(Item::MusicDiscStal),
            "polished_blackstone_brick_stairs" => Some(Item::PolishedBlackstoneBrickStairs),
            "spruce_fence" => Some(Item::SpruceFence),
            "stone_sword" => Some(Item::StoneSword),
            "magenta_terracotta" => Some(Item::MagentaTerracotta),
            "jack_o_lantern" => Some(Item::JackOLantern),
            "wooden_hoe" => Some(Item::WoodenHoe),
            "diamond_axe" => Some(Item::DiamondAxe),
            "red_bed" => Some(Item::RedBed),
            "dirt" => Some(Item::Dirt),
            "purpur_slab" => Some(Item::PurpurSlab),
            "polished_deepslate_slab" => Some(Item::PolishedDeepslateSlab),
            "jungle_door" => Some(Item::JungleDoor),
            "poisonous_potato" => Some(Item::PoisonousPotato),
            "dead_horn_coral_fan" => Some(Item::DeadHornCoralFan),
            "horse_spawn_egg" => Some(Item::HorseSpawnEgg),
            "trader_llama_spawn_egg" => Some(Item::TraderLlamaSpawnEgg),
            "cobblestone_slab" => Some(Item::CobblestoneSlab),
            "wither_rose" => Some(Item::WitherRose),
            "stripped_oak_wood" => Some(Item::StrippedOakWood),
            "globe_banner_pattern" => Some(Item::GlobeBannerPattern),
            "gray_dye" => Some(Item::GrayDye),
            "chiseled_quartz_block" => Some(Item::ChiseledQuartzBlock),
            "dead_brain_coral_block" => Some(Item::DeadBrainCoralBlock),
            "azure_bluet" => Some(Item::AzureBluet),
            "emerald_ore" => Some(Item::EmeraldOre),
            "infested_cobblestone" => Some(Item::InfestedCobblestone),
            "white_dye" => Some(Item::WhiteDye),
            "cooked_cod" => Some(Item::CookedCod),
            "dark_oak_sapling" => Some(Item::DarkOakSapling),
            "light_gray_stained_glass_pane" => Some(Item::LightGrayStainedGlassPane),
            "pink_glazed_terracotta" => Some(Item::PinkGlazedTerracotta),
            "diorite_slab" => Some(Item::DioriteSlab),
            "orange_concrete" => Some(Item::OrangeConcrete),
            "enchanted_golden_apple" => Some(Item::EnchantedGoldenApple),
            "bone_block" => Some(Item::BoneBlock),
            "hopper" => Some(Item::Hopper),
            "spruce_wood" => Some(Item::SpruceWood),
            "sea_pickle" => Some(Item::SeaPickle),
            "ender_chest" => Some(Item::EnderChest),
            "diamond_leggings" => Some(Item::DiamondLeggings),
            "cobbled_deepslate_wall" => Some(Item::CobbledDeepslateWall),
            "cyan_concrete" => Some(Item::CyanConcrete),
            "brick_wall" => Some(Item::BrickWall),
            "lime_bed" => Some(Item::LimeBed),
            "skeleton_horse_spawn_egg" => Some(Item::SkeletonHorseSpawnEgg),
            "cobbled_deepslate_slab" => Some(Item::CobbledDeepslateSlab),
            "white_glazed_terracotta" => Some(Item::WhiteGlazedTerracotta),
            "splash_potion" => Some(Item::SplashPotion),
            "bone" => Some(Item::Bone),
            "acacia_pressure_plate" => Some(Item::AcaciaPressurePlate),
            "gray_shulker_box" => Some(Item::GrayShulkerBox),
            "light_blue_concrete_powder" => Some(Item::LightBlueConcretePowder),
            "warped_pressure_plate" => Some(Item::WarpedPressurePlate),
            "andesite" => Some(Item::Andesite),
            "tnt_minecart" => Some(Item::TntMinecart),
            "blue_terracotta" => Some(Item::BlueTerracotta),
            "wooden_axe" => Some(Item::WoodenAxe),
            "exposed_cut_copper_stairs" => Some(Item::ExposedCutCopperStairs),
            "spruce_trapdoor" => Some(Item::SpruceTrapdoor),
            "potion" => Some(Item::Potion),
            "coarse_dirt" => Some(Item::CoarseDirt),
            "deepslate_tile_wall" => Some(Item::DeepslateTileWall),
            "brown_wool" => Some(Item::BrownWool),
            "smooth_sandstone_stairs" => Some(Item::SmoothSandstoneStairs),
            "lantern" => Some(Item::Lantern),
            "brown_terracotta" => Some(Item::BrownTerracotta),
            "cat_spawn_egg" => Some(Item::CatSpawnEgg),
            "waxed_exposed_cut_copper_slab" => Some(Item::WaxedExposedCutCopperSlab),
            "pink_candle" => Some(Item::PinkCandle),
            "exposed_cut_copper" => Some(Item::ExposedCutCopper),
            "glow_berries" => Some(Item::GlowBerries),
            "snow_block" => Some(Item::SnowBlock),
            "stone_slab" => Some(Item::StoneSlab),
            "dried_kelp_block" => Some(Item::DriedKelpBlock),
            "crimson_fungus" => Some(Item::CrimsonFungus),
            "oxidized_cut_copper_slab" => Some(Item::OxidizedCutCopperSlab),
            "waxed_exposed_cut_copper" => Some(Item::WaxedExposedCutCopper),
            "large_fern" => Some(Item::LargeFern),
            "gold_block" => Some(Item::GoldBlock),
            "repeater" => Some(Item::Repeater),
            "golden_hoe" => Some(Item::GoldenHoe),
            "black_wool" => Some(Item::BlackWool),
            "nether_quartz_ore" => Some(Item::NetherQuartzOre),
            "warped_button" => Some(Item::WarpedButton),
            "birch_fence_gate" => Some(Item::BirchFenceGate),
            "elytra" => Some(Item::Elytra),
            "lingering_potion" => Some(Item::LingeringPotion),
            "leather_helmet" => Some(Item::LeatherHelmet),
            "barrier" => Some(Item::Barrier),
            "brown_dye" => Some(Item::BrownDye),
            "polished_diorite_slab" => Some(Item::PolishedDioriteSlab),
            "activator_rail" => Some(Item::ActivatorRail),
            "drowned_spawn_egg" => Some(Item::DrownedSpawnEgg),
            "dark_oak_log" => Some(Item::DarkOakLog),
            "glow_squid_spawn_egg" => Some(Item::GlowSquidSpawnEgg),
            "green_candle" => Some(Item::GreenCandle),
            "tuff" => Some(Item::Tuff),
            "chain_command_block" => Some(Item::ChainCommandBlock),
            "blaze_spawn_egg" => Some(Item::BlazeSpawnEgg),
            "diorite_stairs" => Some(Item::DioriteStairs),
            "pumpkin" => Some(Item::Pumpkin),
            "blackstone_wall" => Some(Item::BlackstoneWall),
            "hoglin_spawn_egg" => Some(Item::HoglinSpawnEgg),
            "chainmail_helmet" => Some(Item::ChainmailHelmet),
            "dark_oak_button" => Some(Item::DarkOakButton),
            "yellow_wool" => Some(Item::YellowWool),
            "golden_chestplate" => Some(Item::GoldenChestplate),
            "quartz_slab" => Some(Item::QuartzSlab),
            "cod" => Some(Item::Cod),
            "ink_sac" => Some(Item::InkSac),
            "crimson_door" => Some(Item::CrimsonDoor),
            "arrow" => Some(Item::Arrow),
            "pig_spawn_egg" => Some(Item::PigSpawnEgg),
            "cyan_terracotta" => Some(Item::CyanTerracotta),
            "nether_star" => Some(Item::NetherStar),
            "oak_sapling" => Some(Item::OakSapling),
            "honey_block" => Some(Item::HoneyBlock),
            "cut_red_sandstone" => Some(Item::CutRedSandstone),
            "paper" => Some(Item::Paper),
            "exposed_cut_copper_slab" => Some(Item::ExposedCutCopperSlab),
            "suspicious_stew" => Some(Item::SuspiciousStew),
            "flower_banner_pattern" => Some(Item::FlowerBannerPattern),
            "raw_iron" => Some(Item::RawIron),
            "black_glazed_terracotta" => Some(Item::BlackGlazedTerracotta),
            "sheep_spawn_egg" => Some(Item::SheepSpawnEgg),
            "black_banner" => Some(Item::BlackBanner),
            "spyglass" => Some(Item::Spyglass),
            "pink_tulip" => Some(Item::PinkTulip),
            "pink_wool" => Some(Item::PinkWool),
            "netherite_scrap" => Some(Item::NetheriteScrap),
            "diamond_horse_armor" => Some(Item::DiamondHorseArmor),
            "chiseled_polished_blackstone" => Some(Item::ChiseledPolishedBlackstone),
            "lime_concrete" => Some(Item::LimeConcrete),
            "chiseled_nether_bricks" => Some(Item::ChiseledNetherBricks),
            "polished_deepslate_wall" => Some(Item::PolishedDeepslateWall),
            "granite_slab" => Some(Item::GraniteSlab),
            "lightning_rod" => Some(Item::LightningRod),
            "magenta_bed" => Some(Item::MagentaBed),
            "quartz_bricks" => Some(Item::QuartzBricks),
            "soul_sand" => Some(Item::SoulSand),
            "red_sandstone_stairs" => Some(Item::RedSandstoneStairs),
            "lime_stained_glass_pane" => Some(Item::LimeStainedGlassPane),
            "black_candle" => Some(Item::BlackCandle),
            "waxed_weathered_cut_copper" => Some(Item::WaxedWeatheredCutCopper),
            "raw_copper" => Some(Item::RawCopper),
            "deepslate_copper_ore" => Some(Item::DeepslateCopperOre),
            "gray_glazed_terracotta" => Some(Item::GrayGlazedTerracotta),
            "cobblestone" => Some(Item::Cobblestone),
            "green_banner" => Some(Item::GreenBanner),
            "lily_of_the_valley" => Some(Item::LilyOfTheValley),
            "flowering_azalea" => Some(Item::FloweringAzalea),
            "polished_andesite" => Some(Item::PolishedAndesite),
            "wet_sponge" => Some(Item::WetSponge),
            "grindstone" => Some(Item::Grindstone),
            "wheat" => Some(Item::Wheat),
            "beef" => Some(Item::Beef),
            "wooden_sword" => Some(Item::WoodenSword),
            "light_gray_terracotta" => Some(Item::LightGrayTerracotta),
            "diorite" => Some(Item::Diorite),
            "nether_brick" => Some(Item::NetherBrick),
            "birch_button" => Some(Item::BirchButton),
            "acacia_fence_gate" => Some(Item::AcaciaFenceGate),
            "wolf_spawn_egg" => Some(Item::WolfSpawnEgg),
            "crimson_pressure_plate" => Some(Item::CrimsonPressurePlate),
            "cobblestone_wall" => Some(Item::CobblestoneWall),
            "light_blue_stained_glass" => Some(Item::LightBlueStainedGlass),
            "brown_shulker_box" => Some(Item::BrownShulkerBox),
            "waxed_weathered_copper" => Some(Item::WaxedWeatheredCopper),
            "conduit" => Some(Item::Conduit),
            "diamond_pickaxe" => Some(Item::DiamondPickaxe),
            "end_stone_brick_slab" => Some(Item::EndStoneBrickSlab),
            "iron_leggings" => Some(Item::IronLeggings),
            "flint" => Some(Item::Flint),
            "warped_nylium" => Some(Item::WarpedNylium),
            "brown_carpet" => Some(Item::BrownCarpet),
            "stripped_warped_stem" => Some(Item::StrippedWarpedStem),
            "white_banner" => Some(Item::WhiteBanner),
            "white_terracotta" => Some(Item::WhiteTerracotta),
            "creeper_head" => Some(Item::CreeperHead),
            "red_shulker_box" => Some(Item::RedShulkerBox),
            "wooden_shovel" => Some(Item::WoodenShovel),
            "andesite_stairs" => Some(Item::AndesiteStairs),
            "jungle_slab" => Some(Item::JungleSlab),
            "spider_spawn_egg" => Some(Item::SpiderSpawnEgg),
            "mossy_cobblestone_stairs" => Some(Item::MossyCobblestoneStairs),
            "quartz_pillar" => Some(Item::QuartzPillar),
            "acacia_fence" => Some(Item::AcaciaFence),
            "brain_coral_block" => Some(Item::BrainCoralBlock),
            "cyan_banner" => Some(Item::CyanBanner),
            "birch_slab" => Some(Item::BirchSlab),
            "crimson_slab" => Some(Item::CrimsonSlab),
            "amethyst_block" => Some(Item::AmethystBlock),
            "stone_brick_stairs" => Some(Item::StoneBrickStairs),
            "cracked_deepslate_tiles" => Some(Item::CrackedDeepslateTiles),
            "red_tulip" => Some(Item::RedTulip),
            "allium" => Some(Item::Allium),
            "green_concrete_powder" => Some(Item::GreenConcretePowder),
            "music_disc_blocks" => Some(Item::MusicDiscBlocks),
            "salmon" => Some(Item::Salmon),
            "red_terracotta" => Some(Item::RedTerracotta),
            "sandstone_stairs" => Some(Item::SandstoneStairs),
            "ghast_spawn_egg" => Some(Item::GhastSpawnEgg),
            "gray_stained_glass_pane" => Some(Item::GrayStainedGlassPane),
            "ender_eye" => Some(Item::EnderEye),
            "cooked_rabbit" => Some(Item::CookedRabbit),
            "beehive" => Some(Item::Beehive),
            "netherite_helmet" => Some(Item::NetheriteHelmet),
            "turtle_spawn_egg" => Some(Item::TurtleSpawnEgg),
            "leather_boots" => Some(Item::LeatherBoots),
            "crimson_stem" => Some(Item::CrimsonStem),
            "tube_coral_fan" => Some(Item::TubeCoralFan),
            "endermite_spawn_egg" => Some(Item::EndermiteSpawnEgg),
            "wither_skeleton_spawn_egg" => Some(Item::WitherSkeletonSpawnEgg),
            "purple_shulker_box" => Some(Item::PurpleShulkerBox),
            "netherite_block" => Some(Item::NetheriteBlock),
            "spruce_pressure_plate" => Some(Item::SprucePressurePlate),
            "cyan_concrete_powder" => Some(Item::CyanConcretePowder),
            "egg" => Some(Item::Egg),
            "pink_stained_glass" => Some(Item::PinkStainedGlass),
            "red_banner" => Some(Item::RedBanner),
            "jungle_planks" => Some(Item::JunglePlanks),
            "respawn_anchor" => Some(Item::RespawnAnchor),
            "bat_spawn_egg" => Some(Item::BatSpawnEgg),
            "pointed_dripstone" => Some(Item::PointedDripstone),
            "gravel" => Some(Item::Gravel),
            "bowl" => Some(Item::Bowl),
            "cave_spider_spawn_egg" => Some(Item::CaveSpiderSpawnEgg),
            "quartz_stairs" => Some(Item::QuartzStairs),
            "spruce_button" => Some(Item::SpruceButton),
            "carved_pumpkin" => Some(Item::CarvedPumpkin),
            "light_blue_shulker_box" => Some(Item::LightBlueShulkerBox),
            "bee_spawn_egg" => Some(Item::BeeSpawnEgg),
            "oak_fence_gate" => Some(Item::OakFenceGate),
            "nether_brick_fence" => Some(Item::NetherBrickFence),
            "iron_horse_armor" => Some(Item::IronHorseArmor),
            "dead_horn_coral" => Some(Item::DeadHornCoral),
            "polished_diorite" => Some(Item::PolishedDiorite),
            "ravager_spawn_egg" => Some(Item::RavagerSpawnEgg),
            "end_rod" => Some(Item::EndRod),
            "magenta_stained_glass" => Some(Item::MagentaStainedGlass),
            "chorus_plant" => Some(Item::ChorusPlant),
            "orange_concrete_powder" => Some(Item::OrangeConcretePowder),
            "dragon_egg" => Some(Item::DragonEgg),
            "green_concrete" => Some(Item::GreenConcrete),
            "stripped_acacia_log" => Some(Item::StrippedAcaciaLog),
            "diamond" => Some(Item::Diamond),
            "ladder" => Some(Item::Ladder),
            "prismarine_wall" => Some(Item::PrismarineWall),
            "moss_block" => Some(Item::MossBlock),
            "oak_leaves" => Some(Item::OakLeaves),
            "cooked_porkchop" => Some(Item::CookedPorkchop),
            "yellow_stained_glass_pane" => Some(Item::YellowStainedGlassPane),
            "salmon_spawn_egg" => Some(Item::SalmonSpawnEgg),
            "chainmail_boots" => Some(Item::ChainmailBoots),
            "witch_spawn_egg" => Some(Item::WitchSpawnEgg),
            "oxidized_cut_copper_stairs" => Some(Item::OxidizedCutCopperStairs),
            "lapis_ore" => Some(Item::LapisOre),
            "smooth_red_sandstone" => Some(Item::SmoothRedSandstone),
            "jungle_trapdoor" => Some(Item::JungleTrapdoor),
            "shroomlight" => Some(Item::Shroomlight),
            "blackstone_slab" => Some(Item::BlackstoneSlab),
            "mutton" => Some(Item::Mutton),
            "stripped_oak_log" => Some(Item::StrippedOakLog),
            "yellow_stained_glass" => Some(Item::YellowStainedGlass),
            "crying_obsidian" => Some(Item::CryingObsidian),
            "spruce_slab" => Some(Item::SpruceSlab),
            "gray_concrete" => Some(Item::GrayConcrete),
            "chorus_fruit" => Some(Item::ChorusFruit),
            "silverfish_spawn_egg" => Some(Item::SilverfishSpawnEgg),
            "cyan_shulker_box" => Some(Item::CyanShulkerBox),
            "black_terracotta" => Some(Item::BlackTerracotta),
            "redstone_ore" => Some(Item::RedstoneOre),
            "red_dye" => Some(Item::RedDye),
            "smooth_basalt" => Some(Item::SmoothBasalt),
            "purple_glazed_terracotta" => Some(Item::PurpleGlazedTerracotta),
            "popped_chorus_fruit" => Some(Item::PoppedChorusFruit),
            "debug_stick" => Some(Item::DebugStick),
            "waxed_copper_block" => Some(Item::WaxedCopperBlock),
            "warped_roots" => Some(Item::WarpedRoots),
            "diamond_helmet" => Some(Item::DiamondHelmet),
            "birch_pressure_plate" => Some(Item::BirchPressurePlate),
            "pink_concrete" => Some(Item::PinkConcrete),
            "chest_minecart" => Some(Item::ChestMinecart),
            "torch" => Some(Item::Torch),
            "acacia_door" => Some(Item::AcaciaDoor),
            "diamond_shovel" => Some(Item::DiamondShovel),
            "bamboo" => Some(Item::Bamboo),
            "iron_trapdoor" => Some(Item::IronTrapdoor),
            "dark_oak_slab" => Some(Item::DarkOakSlab),
            "brown_bed" => Some(Item::BrownBed),
            "rooted_dirt" => Some(Item::RootedDirt),
            "acacia_trapdoor" => Some(Item::AcaciaTrapdoor),
            "carrot" => Some(Item::Carrot),
            "cobbled_deepslate" => Some(Item::CobbledDeepslate),
            "crimson_roots" => Some(Item::CrimsonRoots),
            "golden_leggings" => Some(Item::GoldenLeggings),
            "grass" => Some(Item::Grass),
            "music_disc_pigstep" => Some(Item::MusicDiscPigstep),
            "command_block" => Some(Item::CommandBlock),
            "cut_copper_slab" => Some(Item::CutCopperSlab),
            "pink_terracotta" => Some(Item::PinkTerracotta),
            "lime_wool" => Some(Item::LimeWool),
            "fire_coral" => Some(Item::FireCoral),
            "birch_boat" => Some(Item::BirchBoat),
            "light_gray_dye" => Some(Item::LightGrayDye),
            "obsidian" => Some(Item::Obsidian),
            "spawner" => Some(Item::Spawner),
            "green_shulker_box" => Some(Item::GreenShulkerBox),
            "filled_map" => Some(Item::FilledMap),
            "blue_candle" => Some(Item::BlueCandle),
            "jungle_log" => Some(Item::JungleLog),
            "skeleton_spawn_egg" => Some(Item::SkeletonSpawnEgg),
            "squid_spawn_egg" => Some(Item::SquidSpawnEgg),
            "blue_glazed_terracotta" => Some(Item::BlueGlazedTerracotta),
            "magenta_shulker_box" => Some(Item::MagentaShulkerBox),
            "nether_gold_ore" => Some(Item::NetherGoldOre),
            "stone_pressure_plate" => Some(Item::StonePressurePlate),
            "item_frame" => Some(Item::ItemFrame),
            "lectern" => Some(Item::Lectern),
            "netherite_hoe" => Some(Item::NetheriteHoe),
            "dark_oak_fence" => Some(Item::DarkOakFence),
            "warped_stairs" => Some(Item::WarpedStairs),
            "tipped_arrow" => Some(Item::TippedArrow),
            "crimson_hyphae" => Some(Item::CrimsonHyphae),
            "chorus_flower" => Some(Item::ChorusFlower),
            "sandstone_slab" => Some(Item::SandstoneSlab),
            "brown_glazed_terracotta" => Some(Item::BrownGlazedTerracotta),
            "dead_fire_coral" => Some(Item::DeadFireCoral),
            "gold_ore" => Some(Item::GoldOre),
            "stripped_crimson_stem" => Some(Item::StrippedCrimsonStem),
            "stone_brick_wall" => Some(Item::StoneBrickWall),
            "dead_tube_coral_block" => Some(Item::DeadTubeCoralBlock),
            "netherite_boots" => Some(Item::NetheriteBoots),
            "copper_ore" => Some(Item::CopperOre),
            "dead_brain_coral_fan" => Some(Item::DeadBrainCoralFan),
            "yellow_carpet" => Some(Item::YellowCarpet),
            "villager_spawn_egg" => Some(Item::VillagerSpawnEgg),
            "armor_stand" => Some(Item::ArmorStand),
            "gray_banner" => Some(Item::GrayBanner),
            "note_block" => Some(Item::NoteBlock),
            "magenta_banner" => Some(Item::MagentaBanner),
            "glistering_melon_slice" => Some(Item::GlisteringMelonSlice),
            "evoker_spawn_egg" => Some(Item::EvokerSpawnEgg),
            "hay_block" => Some(Item::HayBlock),
            "blue_concrete" => Some(Item::BlueConcrete),
            "cooked_beef" => Some(Item::CookedBeef),
            "rabbit_stew" => Some(Item::RabbitStew),
            "packed_ice" => Some(Item::PackedIce),
            "birch_leaves" => Some(Item::BirchLeaves),
            "moss_carpet" => Some(Item::MossCarpet),
            "gray_wool" => Some(Item::GrayWool),
            "mossy_cobblestone_wall" => Some(Item::MossyCobblestoneWall),
            "lilac" => Some(Item::Lilac),
            "stone_brick_slab" => Some(Item::StoneBrickSlab),
            "carrot_on_a_stick" => Some(Item::CarrotOnAStick),
            "crimson_fence_gate" => Some(Item::CrimsonFenceGate),
            "infested_mossy_stone_bricks" => Some(Item::InfestedMossyStoneBricks),
            "cobweb" => Some(Item::Cobweb),
            "turtle_helmet" => Some(Item::TurtleHelmet),
            "oak_slab" => Some(Item::OakSlab),
            "polished_granite" => Some(Item::PolishedGranite),
            "clay" => Some(Item::Clay),
            "orange_wool" => Some(Item::OrangeWool),
            "sand" => Some(Item::Sand),
            "fire_coral_block" => Some(Item::FireCoralBlock),
            "barrel" => Some(Item::Barrel),
            "lead" => Some(Item::Lead),
            "cut_copper_stairs" => Some(Item::CutCopperStairs),
            "waxed_cut_copper" => Some(Item::WaxedCutCopper),
            "warped_stem" => Some(Item::WarpedStem),
            "light_blue_wool" => Some(Item::LightBlueWool),
            "dark_oak_wood" => Some(Item::DarkOakWood),
            "sugar_cane" => Some(Item::SugarCane),
            "cyan_dye" => Some(Item::CyanDye),
            "zombie_head" => Some(Item::ZombieHead),
            "nether_wart" => Some(Item::NetherWart),
            "feather" => Some(Item::Feather),
            "iron_pickaxe" => Some(Item::IronPickaxe),
            "totem_of_undying" => Some(Item::TotemOfUndying),
            "name_tag" => Some(Item::NameTag),
            "glowstone_dust" => Some(Item::GlowstoneDust),
            "melon_seeds" => Some(Item::MelonSeeds),
            "stripped_jungle_log" => Some(Item::StrippedJungleLog),
            "brick_stairs" => Some(Item::BrickStairs),
            "purple_wool" => Some(Item::PurpleWool),
            "magenta_carpet" => Some(Item::MagentaCarpet),
            "iron_door" => Some(Item::IronDoor),
            "cyan_wool" => Some(Item::CyanWool),
            "terracotta" => Some(Item::Terracotta),
            "bread" => Some(Item::Bread),
            "clay_ball" => Some(Item::ClayBall),
            "music_disc_mall" => Some(Item::MusicDiscMall),
            "budding_amethyst" => Some(Item::BuddingAmethyst),
            "crimson_sign" => Some(Item::CrimsonSign),
            "lime_dye" => Some(Item::LimeDye),
            "purpur_stairs" => Some(Item::PurpurStairs),
            "structure_block" => Some(Item::StructureBlock),
            "player_head" => Some(Item::PlayerHead),
            "oak_trapdoor" => Some(Item::OakTrapdoor),
            "parrot_spawn_egg" => Some(Item::ParrotSpawnEgg),
            "azalea_leaves" => Some(Item::AzaleaLeaves),
            "yellow_bed" => Some(Item::YellowBed),
            "iron_ingot" => Some(Item::IronIngot),
            "redstone" => Some(Item::Redstone),
            "beacon" => Some(Item::Beacon),
            "chiseled_deepslate" => Some(Item::ChiseledDeepslate),
            "gray_stained_glass" => Some(Item::GrayStainedGlass),
            "iron_hoe" => Some(Item::IronHoe),
            "piglin_spawn_egg" => Some(Item::PiglinSpawnEgg),
            "elder_guardian_spawn_egg" => Some(Item::ElderGuardianSpawnEgg),
            "dark_oak_fence_gate" => Some(Item::DarkOakFenceGate),
            _ => None,
        }
    }
}
impl Item {
    #[doc = "Returns the `namespaced_id` property of this `Item`."]
    #[inline]
    pub fn namespaced_id(&self) -> &'static str {
        match self {
            Item::DeepslateIronOre => "minecraft:deepslate_iron_ore",
            Item::IronHoe => "minecraft:iron_hoe",
            Item::GoldenBoots => "minecraft:golden_boots",
            Item::BoneBlock => "minecraft:bone_block",
            Item::BirchPressurePlate => "minecraft:birch_pressure_plate",
            Item::CraftingTable => "minecraft:crafting_table",
            Item::IronHorseArmor => "minecraft:iron_horse_armor",
            Item::NetherStar => "minecraft:nether_star",
            Item::NetherQuartzOre => "minecraft:nether_quartz_ore",
            Item::WoodenSword => "minecraft:wooden_sword",
            Item::ZombieHead => "minecraft:zombie_head",
            Item::RavagerSpawnEgg => "minecraft:ravager_spawn_egg",
            Item::ChorusFlower => "minecraft:chorus_flower",
            Item::PiglinBannerPattern => "minecraft:piglin_banner_pattern",
            Item::BrownCandle => "minecraft:brown_candle",
            Item::Map => "minecraft:map",
            Item::BirchTrapdoor => "minecraft:birch_trapdoor",
            Item::VexSpawnEgg => "minecraft:vex_spawn_egg",
            Item::ActivatorRail => "minecraft:activator_rail",
            Item::JunglePlanks => "minecraft:jungle_planks",
            Item::NetherSprouts => "minecraft:nether_sprouts",
            Item::CrackedDeepslateTiles => "minecraft:cracked_deepslate_tiles",
            Item::MossyStoneBrickStairs => "minecraft:mossy_stone_brick_stairs",
            Item::LeatherChestplate => "minecraft:leather_chestplate",
            Item::YellowStainedGlassPane => "minecraft:yellow_stained_glass_pane",
            Item::GrayConcrete => "minecraft:gray_concrete",
            Item::CyanCandle => "minecraft:cyan_candle",
            Item::WaxedCutCopperSlab => "minecraft:waxed_cut_copper_slab",
            Item::BlackConcrete => "minecraft:black_concrete",
            Item::DeadBrainCoral => "minecraft:dead_brain_coral",
            Item::Honeycomb => "minecraft:honeycomb",
            Item::Jukebox => "minecraft:jukebox",
            Item::Farmland => "minecraft:farmland",
            Item::WhiteTerracotta => "minecraft:white_terracotta",
            Item::GoldenChestplate => "minecraft:golden_chestplate",
            Item::CrackedPolishedBlackstoneBricks => "minecraft:cracked_polished_blackstone_bricks",
            Item::Lectern => "minecraft:lectern",
            Item::RedCarpet => "minecraft:red_carpet",
            Item::PrismarineBrickSlab => "minecraft:prismarine_brick_slab",
            Item::EndStoneBrickWall => "minecraft:end_stone_brick_wall",
            Item::SoulSoil => "minecraft:soul_soil",
            Item::PinkCarpet => "minecraft:pink_carpet",
            Item::Charcoal => "minecraft:charcoal",
            Item::Bone => "minecraft:bone",
            Item::WaxedExposedCutCopperStairs => "minecraft:waxed_exposed_cut_copper_stairs",
            Item::MossyCobblestoneSlab => "minecraft:mossy_cobblestone_slab",
            Item::BrewingStand => "minecraft:brewing_stand",
            Item::RedTerracotta => "minecraft:red_terracotta",
            Item::Elytra => "minecraft:elytra",
            Item::TropicalFish => "minecraft:tropical_fish",
            Item::ChainmailHelmet => "minecraft:chainmail_helmet",
            Item::Jigsaw => "minecraft:jigsaw",
            Item::OrangeShulkerBox => "minecraft:orange_shulker_box",
            Item::DarkOakPressurePlate => "minecraft:dark_oak_pressure_plate",
            Item::DeepslateLapisOre => "minecraft:deepslate_lapis_ore",
            Item::OrangeStainedGlassPane => "minecraft:orange_stained_glass_pane",
            Item::CarvedPumpkin => "minecraft:carved_pumpkin",
            Item::WhiteShulkerBox => "minecraft:white_shulker_box",
            Item::MossyCobblestoneStairs => "minecraft:mossy_cobblestone_stairs",
            Item::DeepslateTiles => "minecraft:deepslate_tiles",
            Item::MelonSeeds => "minecraft:melon_seeds",
            Item::LimeCandle => "minecraft:lime_candle",
            Item::EnderPearl => "minecraft:ender_pearl",
            Item::Shroomlight => "minecraft:shroomlight",
            Item::WhiteWool => "minecraft:white_wool",
            Item::Sandstone => "minecraft:sandstone",
            Item::WhiteCandle => "minecraft:white_candle",
            Item::SalmonBucket => "minecraft:salmon_bucket",
            Item::AcaciaLeaves => "minecraft:acacia_leaves",
            Item::OakLog => "minecraft:oak_log",
            Item::Beef => "minecraft:beef",
            Item::MusicDiscBlocks => "minecraft:music_disc_blocks",
            Item::AndesiteStairs => "minecraft:andesite_stairs",
            Item::EndRod => "minecraft:end_rod",
            Item::EndStoneBricks => "minecraft:end_stone_bricks",
            Item::CrimsonDoor => "minecraft:crimson_door",
            Item::Stonecutter => "minecraft:stonecutter",
            Item::WaterBucket => "minecraft:water_bucket",
            Item::Poppy => "minecraft:poppy",
            Item::StoneBrickWall => "minecraft:stone_brick_wall",
            Item::BlackBed => "minecraft:black_bed",
            Item::Prismarine => "minecraft:prismarine",
            Item::ExposedCutCopper => "minecraft:exposed_cut_copper",
            Item::PolishedDeepslateSlab => "minecraft:polished_deepslate_slab",
            Item::Arrow => "minecraft:arrow",
            Item::GreenStainedGlassPane => "minecraft:green_stained_glass_pane",
            Item::BlueBed => "minecraft:blue_bed",
            Item::StrippedWarpedStem => "minecraft:stripped_warped_stem",
            Item::PolishedBlackstone => "minecraft:polished_blackstone",
            Item::BirchSlab => "minecraft:birch_slab",
            Item::WhiteConcretePowder => "minecraft:white_concrete_powder",
            Item::Loom => "minecraft:loom",
            Item::LilyOfTheValley => "minecraft:lily_of_the_valley",
            Item::DioriteWall => "minecraft:diorite_wall",
            Item::BlackGlazedTerracotta => "minecraft:black_glazed_terracotta",
            Item::RoseBush => "minecraft:rose_bush",
            Item::GreenShulkerBox => "minecraft:green_shulker_box",
            Item::BlazeRod => "minecraft:blaze_rod",
            Item::Clock => "minecraft:clock",
            Item::SpruceLeaves => "minecraft:spruce_leaves",
            Item::LightGrayDye => "minecraft:light_gray_dye",
            Item::Mutton => "minecraft:mutton",
            Item::MusicDisc11 => "minecraft:music_disc_11",
            Item::RedMushroomBlock => "minecraft:red_mushroom_block",
            Item::BrownWool => "minecraft:brown_wool",
            Item::CutCopper => "minecraft:cut_copper",
            Item::CartographyTable => "minecraft:cartography_table",
            Item::LilyPad => "minecraft:lily_pad",
            Item::GreenCarpet => "minecraft:green_carpet",
            Item::LightWeightedPressurePlate => "minecraft:light_weighted_pressure_plate",
            Item::DarkOakSapling => "minecraft:dark_oak_sapling",
            Item::MagentaShulkerBox => "minecraft:magenta_shulker_box",
            Item::LeatherBoots => "minecraft:leather_boots",
            Item::DeepslateBricks => "minecraft:deepslate_bricks",
            Item::AmethystShard => "minecraft:amethyst_shard",
            Item::JungleSign => "minecraft:jungle_sign",
            Item::Cake => "minecraft:cake",
            Item::NetheriteBlock => "minecraft:netherite_block",
            Item::LightGrayConcrete => "minecraft:light_gray_concrete",
            Item::DarkOakWood => "minecraft:dark_oak_wood",
            Item::IronTrapdoor => "minecraft:iron_trapdoor",
            Item::DeadTubeCoralBlock => "minecraft:dead_tube_coral_block",
            Item::PinkCandle => "minecraft:pink_candle",
            Item::ExposedCopper => "minecraft:exposed_copper",
            Item::DarkOakPlanks => "minecraft:dark_oak_planks",
            Item::CrackedNetherBricks => "minecraft:cracked_nether_bricks",
            Item::DebugStick => "minecraft:debug_stick",
            Item::PolishedDeepslateWall => "minecraft:polished_deepslate_wall",
            Item::PowderSnowBucket => "minecraft:powder_snow_bucket",
            Item::CyanBed => "minecraft:cyan_bed",
            Item::CatSpawnEgg => "minecraft:cat_spawn_egg",
            Item::FloweringAzalea => "minecraft:flowering_azalea",
            Item::Cod => "minecraft:cod",
            Item::MusicDiscChirp => "minecraft:music_disc_chirp",
            Item::LightBlueTerracotta => "minecraft:light_blue_terracotta",
            Item::PinkBanner => "minecraft:pink_banner",
            Item::LightBlueDye => "minecraft:light_blue_dye",
            Item::PurpleDye => "minecraft:purple_dye",
            Item::CommandBlock => "minecraft:command_block",
            Item::DirtPath => "minecraft:dirt_path",
            Item::StoneButton => "minecraft:stone_button",
            Item::StoneBricks => "minecraft:stone_bricks",
            Item::MushroomStem => "minecraft:mushroom_stem",
            Item::SmoothSandstone => "minecraft:smooth_sandstone",
            Item::StriderSpawnEgg => "minecraft:strider_spawn_egg",
            Item::GlassBottle => "minecraft:glass_bottle",
            Item::SkeletonSkull => "minecraft:skeleton_skull",
            Item::RottenFlesh => "minecraft:rotten_flesh",
            Item::BubbleCoralBlock => "minecraft:bubble_coral_block",
            Item::SoulCampfire => "minecraft:soul_campfire",
            Item::RedstoneOre => "minecraft:redstone_ore",
            Item::YellowGlazedTerracotta => "minecraft:yellow_glazed_terracotta",
            Item::FireCoralBlock => "minecraft:fire_coral_block",
            Item::ChorusPlant => "minecraft:chorus_plant",
            Item::BlueConcrete => "minecraft:blue_concrete",
            Item::RawCopper => "minecraft:raw_copper",
            Item::Blackstone => "minecraft:blackstone",
            Item::PurpleCandle => "minecraft:purple_candle",
            Item::Rabbit => "minecraft:rabbit",
            Item::WaxedExposedCopper => "minecraft:waxed_exposed_copper",
            Item::IronPickaxe => "minecraft:iron_pickaxe",
            Item::GraniteWall => "minecraft:granite_wall",
            Item::ZombieVillagerSpawnEgg => "minecraft:zombie_villager_spawn_egg",
            Item::DetectorRail => "minecraft:detector_rail",
            Item::MusicDiscOtherside => "minecraft:music_disc_otherside",
            Item::SpruceStairs => "minecraft:spruce_stairs",
            Item::NetherBrickStairs => "minecraft:nether_brick_stairs",
            Item::PurpleConcrete => "minecraft:purple_concrete",
            Item::ChiseledSandstone => "minecraft:chiseled_sandstone",
            Item::PurpleWool => "minecraft:purple_wool",
            Item::Paper => "minecraft:paper",
            Item::RedNetherBrickWall => "minecraft:red_nether_brick_wall",
            Item::JungleButton => "minecraft:jungle_button",
            Item::ChiseledNetherBricks => "minecraft:chiseled_nether_bricks",
            Item::GlowInkSac => "minecraft:glow_ink_sac",
            Item::MagentaStainedGlassPane => "minecraft:magenta_stained_glass_pane",
            Item::Furnace => "minecraft:furnace",
            Item::BrownMushroom => "minecraft:brown_mushroom",
            Item::Sponge => "minecraft:sponge",
            Item::StonePressurePlate => "minecraft:stone_pressure_plate",
            Item::NetheriteSword => "minecraft:netherite_sword",
            Item::Pumpkin => "minecraft:pumpkin",
            Item::ArmorStand => "minecraft:armor_stand",
            Item::BatSpawnEgg => "minecraft:bat_spawn_egg",
            Item::DarkOakSlab => "minecraft:dark_oak_slab",
            Item::LeatherLeggings => "minecraft:leather_leggings",
            Item::HorseSpawnEgg => "minecraft:horse_spawn_egg",
            Item::Bread => "minecraft:bread",
            Item::WaxedCopperBlock => "minecraft:waxed_copper_block",
            Item::WaxedCutCopper => "minecraft:waxed_cut_copper",
            Item::WaxedWeatheredCutCopperSlab => "minecraft:waxed_weathered_cut_copper_slab",
            Item::WeepingVines => "minecraft:weeping_vines",
            Item::EnchantingTable => "minecraft:enchanting_table",
            Item::CyanConcrete => "minecraft:cyan_concrete",
            Item::OcelotSpawnEgg => "minecraft:ocelot_spawn_egg",
            Item::Beetroot => "minecraft:beetroot",
            Item::PinkConcretePowder => "minecraft:pink_concrete_powder",
            Item::NetheriteBoots => "minecraft:netherite_boots",
            Item::GoldenShovel => "minecraft:golden_shovel",
            Item::AzureBluet => "minecraft:azure_bluet",
            Item::PolishedBlackstoneBrickWall => "minecraft:polished_blackstone_brick_wall",
            Item::WarpedTrapdoor => "minecraft:warped_trapdoor",
            Item::BlueCandle => "minecraft:blue_candle",
            Item::Potion => "minecraft:potion",
            Item::MediumAmethystBud => "minecraft:medium_amethyst_bud",
            Item::BlueGlazedTerracotta => "minecraft:blue_glazed_terracotta",
            Item::Feather => "minecraft:feather",
            Item::Gunpowder => "minecraft:gunpowder",
            Item::AcaciaSign => "minecraft:acacia_sign",
            Item::GhastSpawnEgg => "minecraft:ghast_spawn_egg",
            Item::TropicalFishSpawnEgg => "minecraft:tropical_fish_spawn_egg",
            Item::PolishedBlackstoneBrickSlab => "minecraft:polished_blackstone_brick_slab",
            Item::OxidizedCutCopperSlab => "minecraft:oxidized_cut_copper_slab",
            Item::DarkOakFenceGate => "minecraft:dark_oak_fence_gate",
            Item::LightBlueGlazedTerracotta => "minecraft:light_blue_glazed_terracotta",
            Item::BlackShulkerBox => "minecraft:black_shulker_box",
            Item::GoatSpawnEgg => "minecraft:goat_spawn_egg",
            Item::BrickWall => "minecraft:brick_wall",
            Item::Leather => "minecraft:leather",
            Item::CopperOre => "minecraft:copper_ore",
            Item::PurpleBed => "minecraft:purple_bed",
            Item::LlamaSpawnEgg => "minecraft:llama_spawn_egg",
            Item::LingeringPotion => "minecraft:lingering_potion",
            Item::SuspiciousStew => "minecraft:suspicious_stew",
            Item::YellowTerracotta => "minecraft:yellow_terracotta",
            Item::ChickenSpawnEgg => "minecraft:chicken_spawn_egg",
            Item::StrippedCrimsonStem => "minecraft:stripped_crimson_stem",
            Item::WhiteStainedGlassPane => "minecraft:white_stained_glass_pane",
            Item::AndesiteSlab => "minecraft:andesite_slab",
            Item::DiamondLeggings => "minecraft:diamond_leggings",
            Item::Carrot => "minecraft:carrot",
            Item::WarpedStem => "minecraft:warped_stem",
            Item::WarpedStairs => "minecraft:warped_stairs",
            Item::LightBlueStainedGlass => "minecraft:light_blue_stained_glass",
            Item::OakSign => "minecraft:oak_sign",
            Item::FletchingTable => "minecraft:fletching_table",
            Item::GreenGlazedTerracotta => "minecraft:green_glazed_terracotta",
            Item::GrayCandle => "minecraft:gray_candle",
            Item::MojangBannerPattern => "minecraft:mojang_banner_pattern",
            Item::DripstoneBlock => "minecraft:dripstone_block",
            Item::WaxedWeatheredCopper => "minecraft:waxed_weathered_copper",
            Item::WaxedOxidizedCutCopper => "minecraft:waxed_oxidized_cut_copper",
            Item::CrimsonNylium => "minecraft:crimson_nylium",
            Item::RepeatingCommandBlock => "minecraft:repeating_command_block",
            Item::OakButton => "minecraft:oak_button",
            Item::OrangeTulip => "minecraft:orange_tulip",
            Item::LightBlueConcretePowder => "minecraft:light_blue_concrete_powder",
            Item::Sand => "minecraft:sand",
            Item::CopperBlock => "minecraft:copper_block",
            Item::GoldIngot => "minecraft:gold_ingot",
            Item::Apple => "minecraft:apple",
            Item::BlackCandle => "minecraft:black_candle",
            Item::CyanShulkerBox => "minecraft:cyan_shulker_box",
            Item::BirchDoor => "minecraft:birch_door",
            Item::StoneSword => "minecraft:stone_sword",
            Item::JungleSlab => "minecraft:jungle_slab",
            Item::PurpleConcretePowder => "minecraft:purple_concrete_powder",
            Item::LightGrayConcretePowder => "minecraft:light_gray_concrete_powder",
            Item::SpiderSpawnEgg => "minecraft:spider_spawn_egg",
            Item::LargeAmethystBud => "minecraft:large_amethyst_bud",
            Item::PinkBed => "minecraft:pink_bed",
            Item::JungleTrapdoor => "minecraft:jungle_trapdoor",
            Item::CaveSpiderSpawnEgg => "minecraft:cave_spider_spawn_egg",
            Item::NetherBrickFence => "minecraft:nether_brick_fence",
            Item::PigSpawnEgg => "minecraft:pig_spawn_egg",
            Item::BirchWood => "minecraft:birch_wood",
            Item::JungleStairs => "minecraft:jungle_stairs",
            Item::Beehive => "minecraft:beehive",
            Item::RedSandstoneStairs => "minecraft:red_sandstone_stairs",
            Item::VillagerSpawnEgg => "minecraft:villager_spawn_egg",
            Item::CutSandstoneSlab => "minecraft:cut_sandstone_slab",
            Item::WarpedFungus => "minecraft:warped_fungus",
            Item::SlimeSpawnEgg => "minecraft:slime_spawn_egg",
            Item::SpruceWood => "minecraft:spruce_wood",
            Item::Deepslate => "minecraft:deepslate",
            Item::YellowShulkerBox => "minecraft:yellow_shulker_box",
            Item::DarkPrismarineStairs => "minecraft:dark_prismarine_stairs",
            Item::HopperMinecart => "minecraft:hopper_minecart",
            Item::DeadBubbleCoralBlock => "minecraft:dead_bubble_coral_block",
            Item::RawCopperBlock => "minecraft:raw_copper_block",
            Item::Flint => "minecraft:flint",
            Item::SmoothQuartzSlab => "minecraft:smooth_quartz_slab",
            Item::StructureBlock => "minecraft:structure_block",
            Item::IronSword => "minecraft:iron_sword",
            Item::Compass => "minecraft:compass",
            Item::GoldenHoe => "minecraft:golden_hoe",
            Item::Peony => "minecraft:peony",
            Item::NetheriteChestplate => "minecraft:netherite_chestplate",
            Item::IronBlock => "minecraft:iron_block",
            Item::CrimsonTrapdoor => "minecraft:crimson_trapdoor",
            Item::GlobeBannerPattern => "minecraft:globe_banner_pattern",
            Item::LightGrayTerracotta => "minecraft:light_gray_terracotta",
            Item::MossyStoneBricks => "minecraft:mossy_stone_bricks",
            Item::LightGrayBed => "minecraft:light_gray_bed",
            Item::StickyPiston => "minecraft:sticky_piston",
            Item::CyanStainedGlassPane => "minecraft:cyan_stained_glass_pane",
            Item::PolishedDeepslateStairs => "minecraft:polished_deepslate_stairs",
            Item::SprucePressurePlate => "minecraft:spruce_pressure_plate",
            Item::CookedSalmon => "minecraft:cooked_salmon",
            Item::Granite => "minecraft:granite",
            Item::RedMushroom => "minecraft:red_mushroom",
            Item::EndStone => "minecraft:end_stone",
            Item::NetherBrick => "minecraft:nether_brick",
            Item::CrimsonButton => "minecraft:crimson_button",
            Item::DriedKelp => "minecraft:dried_kelp",
            Item::SugarCane => "minecraft:sugar_cane",
            Item::FireCoralFan => "minecraft:fire_coral_fan",
            Item::BlueShulkerBox => "minecraft:blue_shulker_box",
            Item::CreeperHead => "minecraft:creeper_head",
            Item::SmithingTable => "minecraft:smithing_table",
            Item::Azalea => "minecraft:azalea",
            Item::FermentedSpiderEye => "minecraft:fermented_spider_eye",
            Item::BirchBoat => "minecraft:birch_boat",
            Item::DeepslateTileWall => "minecraft:deepslate_tile_wall",
            Item::StoneHoe => "minecraft:stone_hoe",
            Item::GrayStainedGlassPane => "minecraft:gray_stained_glass_pane",
            Item::DiamondHelmet => "minecraft:diamond_helmet",
            Item::DeadBrainCoralBlock => "minecraft:dead_brain_coral_block",
            Item::CutSandstone => "minecraft:cut_sandstone",
            Item::SlimeBall => "minecraft:slime_ball",
            Item::GildedBlackstone => "minecraft:gilded_blackstone",
            Item::WitchSpawnEgg => "minecraft:witch_spawn_egg",
            Item::BlackstoneWall => "minecraft:blackstone_wall",
            Item::FireCoral => "minecraft:fire_coral",
            Item::Redstone => "minecraft:redstone",
            Item::WrittenBook => "minecraft:written_book",
            Item::WaxedCutCopperStairs => "minecraft:waxed_cut_copper_stairs",
            Item::GraniteSlab => "minecraft:granite_slab",
            Item::Scaffolding => "minecraft:scaffolding",
            Item::LimeDye => "minecraft:lime_dye",
            Item::PolishedBlackstoneButton => "minecraft:polished_blackstone_button",
            Item::DeadBubbleCoralFan => "minecraft:dead_bubble_coral_fan",
            Item::Glowstone => "minecraft:glowstone",
            Item::HornCoral => "minecraft:horn_coral",
            Item::SquidSpawnEgg => "minecraft:squid_spawn_egg",
            Item::YellowCarpet => "minecraft:yellow_carpet",
            Item::LapisLazuli => "minecraft:lapis_lazuli",
            Item::WarpedWartBlock => "minecraft:warped_wart_block",
            Item::SmoothQuartz => "minecraft:smooth_quartz",
            Item::CobbledDeepslate => "minecraft:cobbled_deepslate",
            Item::TripwireHook => "minecraft:tripwire_hook",
            Item::Dropper => "minecraft:dropper",
            Item::MagentaGlazedTerracotta => "minecraft:magenta_glazed_terracotta",
            Item::SoulSand => "minecraft:soul_sand",
            Item::Potato => "minecraft:potato",
            Item::MagmaCream => "minecraft:magma_cream",
            Item::CowSpawnEgg => "minecraft:cow_spawn_egg",
            Item::TrappedChest => "minecraft:trapped_chest",
            Item::JungleFenceGate => "minecraft:jungle_fence_gate",
            Item::GoldenApple => "minecraft:golden_apple",
            Item::DarkOakFence => "minecraft:dark_oak_fence",
            Item::DeadHornCoral => "minecraft:dead_horn_coral",
            Item::DarkOakLeaves => "minecraft:dark_oak_leaves",
            Item::DeadFireCoral => "minecraft:dead_fire_coral",
            Item::IronShovel => "minecraft:iron_shovel",
            Item::EnchantedBook => "minecraft:enchanted_book",
            Item::WhiteTulip => "minecraft:white_tulip",
            Item::AcaciaButton => "minecraft:acacia_button",
            Item::Emerald => "minecraft:emerald",
            Item::SandstoneSlab => "minecraft:sandstone_slab",
            Item::IronDoor => "minecraft:iron_door",
            Item::RawIron => "minecraft:raw_iron",
            Item::LapisBlock => "minecraft:lapis_block",
            Item::BigDripleaf => "minecraft:big_dripleaf",
            Item::BlueConcretePowder => "minecraft:blue_concrete_powder",
            Item::Barrel => "minecraft:barrel",
            Item::MagentaStainedGlass => "minecraft:magenta_stained_glass",
            Item::BrownConcrete => "minecraft:brown_concrete",
            Item::PinkTulip => "minecraft:pink_tulip",
            Item::GrayBanner => "minecraft:gray_banner",
            Item::Bedrock => "minecraft:bedrock",
            Item::DeadFireCoralBlock => "minecraft:dead_fire_coral_block",
            Item::Anvil => "minecraft:anvil",
            Item::StonePickaxe => "minecraft:stone_pickaxe",
            Item::LeatherHelmet => "minecraft:leather_helmet",
            Item::Porkchop => "minecraft:porkchop",
            Item::StoneBrickSlab => "minecraft:stone_brick_slab",
            Item::OrangeCandle => "minecraft:orange_candle",
            Item::BlueWool => "minecraft:blue_wool",
            Item::MossCarpet => "minecraft:moss_carpet",
            Item::SilverfishSpawnEgg => "minecraft:silverfish_spawn_egg",
            Item::SpectralArrow => "minecraft:spectral_arrow",
            Item::CookedRabbit => "minecraft:cooked_rabbit",
            Item::IronChestplate => "minecraft:iron_chestplate",
            Item::GrayConcretePowder => "minecraft:gray_concrete_powder",
            Item::WarpedPlanks => "minecraft:warped_planks",
            Item::DeadTubeCoral => "minecraft:dead_tube_coral",
            Item::DrownedSpawnEgg => "minecraft:drowned_spawn_egg",
            Item::SplashPotion => "minecraft:splash_potion",
            Item::WoodenShovel => "minecraft:wooden_shovel",
            Item::WarpedRoots => "minecraft:warped_roots",
            Item::QuartzPillar => "minecraft:quartz_pillar",
            Item::GlisteringMelonSlice => "minecraft:glistering_melon_slice",
            Item::CrimsonStairs => "minecraft:crimson_stairs",
            Item::Obsidian => "minecraft:obsidian",
            Item::RedCandle => "minecraft:red_candle",
            Item::SoulTorch => "minecraft:soul_torch",
            Item::GhastTear => "minecraft:ghast_tear",
            Item::Diamond => "minecraft:diamond",
            Item::EndStoneBrickSlab => "minecraft:end_stone_brick_slab",
            Item::GrayGlazedTerracotta => "minecraft:gray_glazed_terracotta",
            Item::OakTrapdoor => "minecraft:oak_trapdoor",
            Item::LimeBanner => "minecraft:lime_banner",
            Item::DarkOakTrapdoor => "minecraft:dark_oak_trapdoor",
            Item::OakSapling => "minecraft:oak_sapling",
            Item::PufferfishBucket => "minecraft:pufferfish_bucket",
            Item::Clay => "minecraft:clay",
            Item::CyanConcretePowder => "minecraft:cyan_concrete_powder",
            Item::Gravel => "minecraft:gravel",
            Item::YellowStainedGlass => "minecraft:yellow_stained_glass",
            Item::GrayWool => "minecraft:gray_wool",
            Item::BlueIce => "minecraft:blue_ice",
            Item::StrippedDarkOakLog => "minecraft:stripped_dark_oak_log",
            Item::Fern => "minecraft:fern",
            Item::MilkBucket => "minecraft:milk_bucket",
            Item::BrainCoralBlock => "minecraft:brain_coral_block",
            Item::OrangeBanner => "minecraft:orange_banner",
            Item::FilledMap => "minecraft:filled_map",
            Item::RedNetherBrickSlab => "minecraft:red_nether_brick_slab",
            Item::PolishedGraniteStairs => "minecraft:polished_granite_stairs",
            Item::ChainmailBoots => "minecraft:chainmail_boots",
            Item::OakDoor => "minecraft:oak_door",
            Item::SheepSpawnEgg => "minecraft:sheep_spawn_egg",
            Item::OakWood => "minecraft:oak_wood",
            Item::Shield => "minecraft:shield",
            Item::BlastFurnace => "minecraft:blast_furnace",
            Item::RedStainedGlassPane => "minecraft:red_stained_glass_pane",
            Item::Spyglass => "minecraft:spyglass",
            Item::WanderingTraderSpawnEgg => "minecraft:wandering_trader_spawn_egg",
            Item::Repeater => "minecraft:repeater",
            Item::Ladder => "minecraft:ladder",
            Item::HornCoralFan => "minecraft:horn_coral_fan",
            Item::WhiteConcrete => "minecraft:white_concrete",
            Item::SprucePlanks => "minecraft:spruce_planks",
            Item::PurpurPillar => "minecraft:purpur_pillar",
            Item::BlackstoneSlab => "minecraft:blackstone_slab",
            Item::DarkOakDoor => "minecraft:dark_oak_door",
            Item::Seagrass => "minecraft:seagrass",
            Item::DeepslateBrickSlab => "minecraft:deepslate_brick_slab",
            Item::CutCopperStairs => "minecraft:cut_copper_stairs",
            Item::StoneStairs => "minecraft:stone_stairs",
            Item::DioriteSlab => "minecraft:diorite_slab",
            Item::PolishedBlackstoneBrickStairs => "minecraft:polished_blackstone_brick_stairs",
            Item::AndesiteWall => "minecraft:andesite_wall",
            Item::PinkDye => "minecraft:pink_dye",
            Item::PhantomSpawnEgg => "minecraft:phantom_spawn_egg",
            Item::LimeStainedGlass => "minecraft:lime_stained_glass",
            Item::SkeletonHorseSpawnEgg => "minecraft:skeleton_horse_spawn_egg",
            Item::SpruceFenceGate => "minecraft:spruce_fence_gate",
            Item::Bamboo => "minecraft:bamboo",
            Item::DarkOakStairs => "minecraft:dark_oak_stairs",
            Item::LightBlueBanner => "minecraft:light_blue_banner",
            Item::Conduit => "minecraft:conduit",
            Item::PurpleCarpet => "minecraft:purple_carpet",
            Item::Cornflower => "minecraft:cornflower",
            Item::WheatSeeds => "minecraft:wheat_seeds",
            Item::MusicDiscMellohi => "minecraft:music_disc_mellohi",
            Item::Smoker => "minecraft:smoker",
            Item::MossyCobblestone => "minecraft:mossy_cobblestone",
            Item::WaxedWeatheredCutCopper => "minecraft:waxed_weathered_cut_copper",
            Item::Netherrack => "minecraft:netherrack",
            Item::BrownTerracotta => "minecraft:brown_terracotta",
            Item::Piston => "minecraft:piston",
            Item::ChorusFruit => "minecraft:chorus_fruit",
            Item::RedstoneLamp => "minecraft:redstone_lamp",
            Item::AcaciaPressurePlate => "minecraft:acacia_pressure_plate",
            Item::PurpleBanner => "minecraft:purple_banner",
            Item::LightBlueCarpet => "minecraft:light_blue_carpet",
            Item::CyanWool => "minecraft:cyan_wool",
            Item::CocoaBeans => "minecraft:cocoa_beans",
            Item::BrownStainedGlassPane => "minecraft:brown_stained_glass_pane",
            Item::CodSpawnEgg => "minecraft:cod_spawn_egg",
            Item::CrimsonFence => "minecraft:crimson_fence",
            Item::LeatherHorseArmor => "minecraft:leather_horse_armor",
            Item::Andesite => "minecraft:andesite",
            Item::OakSlab => "minecraft:oak_slab",
            Item::WitherSkeletonSkull => "minecraft:wither_skeleton_skull",
            Item::HayBlock => "minecraft:hay_block",
            Item::BrownGlazedTerracotta => "minecraft:brown_glazed_terracotta",
            Item::PinkWool => "minecraft:pink_wool",
            Item::InkSac => "minecraft:ink_sac",
            Item::AcaciaPlanks => "minecraft:acacia_planks",
            Item::EndPortalFrame => "minecraft:end_portal_frame",
            Item::Cookie => "minecraft:cookie",
            Item::CrackedDeepslateBricks => "minecraft:cracked_deepslate_bricks",
            Item::FireworkStar => "minecraft:firework_star",
            Item::LightBlueCandle => "minecraft:light_blue_candle",
            Item::NetherGoldOre => "minecraft:nether_gold_ore",
            Item::CutRedSandstone => "minecraft:cut_red_sandstone",
            Item::CobbledDeepslateSlab => "minecraft:cobbled_deepslate_slab",
            Item::CrimsonFenceGate => "minecraft:crimson_fence_gate",
            Item::DeepslateBrickStairs => "minecraft:deepslate_brick_stairs",
            Item::JungleBoat => "minecraft:jungle_boat",
            Item::StructureVoid => "minecraft:structure_void",
            Item::PinkGlazedTerracotta => "minecraft:pink_glazed_terracotta",
            Item::DioriteStairs => "minecraft:diorite_stairs",
            Item::WeatheredCutCopperSlab => "minecraft:weathered_cut_copper_slab",
            Item::MagentaCarpet => "minecraft:magenta_carpet",
            Item::WaxedOxidizedCutCopperSlab => "minecraft:waxed_oxidized_cut_copper_slab",
            Item::PurpleStainedGlass => "minecraft:purple_stained_glass",
            Item::LimeShulkerBox => "minecraft:lime_shulker_box",
            Item::BlackConcretePowder => "minecraft:black_concrete_powder",
            Item::Rail => "minecraft:rail",
            Item::DriedKelpBlock => "minecraft:dried_kelp_block",
            Item::AcaciaLog => "minecraft:acacia_log",
            Item::ChiseledQuartzBlock => "minecraft:chiseled_quartz_block",
            Item::PiglinBruteSpawnEgg => "minecraft:piglin_brute_spawn_egg",
            Item::DeepslateCoalOre => "minecraft:deepslate_coal_ore",
            Item::PlayerHead => "minecraft:player_head",
            Item::ChiseledStoneBricks => "minecraft:chiseled_stone_bricks",
            Item::AcaciaSapling => "minecraft:acacia_sapling",
            Item::BrownCarpet => "minecraft:brown_carpet",
            Item::CobblestoneStairs => "minecraft:cobblestone_stairs",
            Item::Beacon => "minecraft:beacon",
            Item::CobbledDeepslateWall => "minecraft:cobbled_deepslate_wall",
            Item::InfestedCobblestone => "minecraft:infested_cobblestone",
            Item::WarpedPressurePlate => "minecraft:warped_pressure_plate",
            Item::NetheritePickaxe => "minecraft:netherite_pickaxe",
            Item::EndermiteSpawnEgg => "minecraft:endermite_spawn_egg",
            Item::OakFence => "minecraft:oak_fence",
            Item::BrownBed => "minecraft:brown_bed",
            Item::StrippedWarpedHyphae => "minecraft:stripped_warped_hyphae",
            Item::WaxedExposedCutCopper => "minecraft:waxed_exposed_cut_copper",
            Item::RedNetherBricks => "minecraft:red_nether_bricks",
            Item::RedWool => "minecraft:red_wool",
            Item::PolishedAndesiteSlab => "minecraft:polished_andesite_slab",
            Item::NetherWart => "minecraft:nether_wart",
            Item::YellowWool => "minecraft:yellow_wool",
            Item::BlackCarpet => "minecraft:black_carpet",
            Item::EmeraldBlock => "minecraft:emerald_block",
            Item::LightGrayWool => "minecraft:light_gray_wool",
            Item::InfestedDeepslate => "minecraft:infested_deepslate",
            Item::SmoothSandstoneSlab => "minecraft:smooth_sandstone_slab",
            Item::AcaciaTrapdoor => "minecraft:acacia_trapdoor",
            Item::BrownDye => "minecraft:brown_dye",
            Item::EmeraldOre => "minecraft:emerald_ore",
            Item::NautilusShell => "minecraft:nautilus_shell",
            Item::Tuff => "minecraft:tuff",
            Item::Chest => "minecraft:chest",
            Item::Snow => "minecraft:snow",
            Item::TwistingVines => "minecraft:twisting_vines",
            Item::GrayDye => "minecraft:gray_dye",
            Item::SmoothRedSandstone => "minecraft:smooth_red_sandstone",
            Item::RootedDirt => "minecraft:rooted_dirt",
            Item::RawIronBlock => "minecraft:raw_iron_block",
            Item::Painting => "minecraft:painting",
            Item::ShulkerSpawnEgg => "minecraft:shulker_spawn_egg",
            Item::FlintAndSteel => "minecraft:flint_and_steel",
            Item::OrangeStainedGlass => "minecraft:orange_stained_glass",
            Item::AmethystBlock => "minecraft:amethyst_block",
            Item::ChiseledRedSandstone => "minecraft:chiseled_red_sandstone",
            Item::LimeConcrete => "minecraft:lime_concrete",
            Item::SpruceTrapdoor => "minecraft:spruce_trapdoor",
            Item::AncientDebris => "minecraft:ancient_debris",
            Item::LightGrayStainedGlassPane => "minecraft:light_gray_stained_glass_pane",
            Item::RedstoneBlock => "minecraft:redstone_block",
            Item::PiglinSpawnEgg => "minecraft:piglin_spawn_egg",
            Item::NameTag => "minecraft:name_tag",
            Item::SmoothRedSandstoneSlab => "minecraft:smooth_red_sandstone_slab",
            Item::WarpedSign => "minecraft:warped_sign",
            Item::Crossbow => "minecraft:crossbow",
            Item::AxolotlSpawnEgg => "minecraft:axolotl_spawn_egg",
            Item::NetheriteHelmet => "minecraft:netherite_helmet",
            Item::GoldenAxe => "minecraft:golden_axe",
            Item::MusicDiscCat => "minecraft:music_disc_cat",
            Item::Diorite => "minecraft:diorite",
            Item::WarpedSlab => "minecraft:warped_slab",
            Item::NetherBrickSlab => "minecraft:nether_brick_slab",
            Item::SmoothSandstoneStairs => "minecraft:smooth_sandstone_stairs",
            Item::NetheriteIngot => "minecraft:netherite_ingot",
            Item::HangingRoots => "minecraft:hanging_roots",
            Item::DeadHornCoralBlock => "minecraft:dead_horn_coral_block",
            Item::ZoglinSpawnEgg => "minecraft:zoglin_spawn_egg",
            Item::DiamondBlock => "minecraft:diamond_block",
            Item::StrippedBirchWood => "minecraft:stripped_birch_wood",
            Item::OrangeGlazedTerracotta => "minecraft:orange_glazed_terracotta",
            Item::NetheriteLeggings => "minecraft:netherite_leggings",
            Item::DiamondShovel => "minecraft:diamond_shovel",
            Item::Trident => "minecraft:trident",
            Item::BirchFence => "minecraft:birch_fence",
            Item::WetSponge => "minecraft:wet_sponge",
            Item::RabbitHide => "minecraft:rabbit_hide",
            Item::InfestedStoneBricks => "minecraft:infested_stone_bricks",
            Item::MagentaTerracotta => "minecraft:magenta_terracotta",
            Item::WeatheredCopper => "minecraft:weathered_copper",
            Item::GrayCarpet => "minecraft:gray_carpet",
            Item::JungleLeaves => "minecraft:jungle_leaves",
            Item::BlueOrchid => "minecraft:blue_orchid",
            Item::Spawner => "minecraft:spawner",
            Item::CyanGlazedTerracotta => "minecraft:cyan_glazed_terracotta",
            Item::CookedPorkchop => "minecraft:cooked_porkchop",
            Item::HuskSpawnEgg => "minecraft:husk_spawn_egg",
            Item::SkeletonSpawnEgg => "minecraft:skeleton_spawn_egg",
            Item::GoldenHorseArmor => "minecraft:golden_horse_armor",
            Item::OrangeWool => "minecraft:orange_wool",
            Item::HeartOfTheSea => "minecraft:heart_of_the_sea",
            Item::SpruceLog => "minecraft:spruce_log",
            Item::CobblestoneSlab => "minecraft:cobblestone_slab",
            Item::BoneMeal => "minecraft:bone_meal",
            Item::NetheriteScrap => "minecraft:netherite_scrap",
            Item::MusicDiscWait => "minecraft:music_disc_wait",
            Item::LimeWool => "minecraft:lime_wool",
            Item::RawGoldBlock => "minecraft:raw_gold_block",
            Item::LightBlueStainedGlassPane => "minecraft:light_blue_stained_glass_pane",
            Item::OakPressurePlate => "minecraft:oak_pressure_plate",
            Item::WoodenPickaxe => "minecraft:wooden_pickaxe",
            Item::EnchantedGoldenApple => "minecraft:enchanted_golden_apple",
            Item::SweetBerries => "minecraft:sweet_berries",
            Item::RedDye => "minecraft:red_dye",
            Item::PrismarineBrickStairs => "minecraft:prismarine_brick_stairs",
            Item::CutRedSandstoneSlab => "minecraft:cut_red_sandstone_slab",
            Item::GoldBlock => "minecraft:gold_block",
            Item::IronHelmet => "minecraft:iron_helmet",
            Item::GlowLichen => "minecraft:glow_lichen",
            Item::MossyStoneBrickWall => "minecraft:mossy_stone_brick_wall",
            Item::StrippedBirchLog => "minecraft:stripped_birch_log",
            Item::ChainmailChestplate => "minecraft:chainmail_chestplate",
            Item::PumpkinSeeds => "minecraft:pumpkin_seeds",
            Item::IronAxe => "minecraft:iron_axe",
            Item::CutCopperSlab => "minecraft:cut_copper_slab",
            Item::BakedPotato => "minecraft:baked_potato",
            Item::DiamondSword => "minecraft:diamond_sword",
            Item::BubbleCoral => "minecraft:bubble_coral",
            Item::RedNetherBrickStairs => "minecraft:red_nether_brick_stairs",
            Item::WoodenHoe => "minecraft:wooden_hoe",
            Item::Lantern => "minecraft:lantern",
            Item::DragonEgg => "minecraft:dragon_egg",
            Item::ExposedCutCopperSlab => "minecraft:exposed_cut_copper_slab",
            Item::MagentaConcrete => "minecraft:magenta_concrete",
            Item::AxolotlBucket => "minecraft:axolotl_bucket",
            Item::MuleSpawnEgg => "minecraft:mule_spawn_egg",
            Item::PumpkinPie => "minecraft:pumpkin_pie",
            Item::InfestedStone => "minecraft:infested_stone",
            Item::MagmaCubeSpawnEgg => "minecraft:magma_cube_spawn_egg",
            Item::TubeCoralBlock => "minecraft:tube_coral_block",
            Item::BlueBanner => "minecraft:blue_banner",
            Item::CoalBlock => "minecraft:coal_block",
            Item::StrippedAcaciaWood => "minecraft:stripped_acacia_wood",
            Item::BeetrootSoup => "minecraft:beetroot_soup",
            Item::Barrier => "minecraft:barrier",
            Item::EndCrystal => "minecraft:end_crystal",
            Item::Mycelium => "minecraft:mycelium",
            Item::SporeBlossom => "minecraft:spore_blossom",
            Item::Wheat => "minecraft:wheat",
            Item::PrismarineStairs => "minecraft:prismarine_stairs",
            Item::PurpleTerracotta => "minecraft:purple_terracotta",
            Item::TropicalFishBucket => "minecraft:tropical_fish_bucket",
            Item::Lever => "minecraft:lever",
            Item::OrangeDye => "minecraft:orange_dye",
            Item::GreenBed => "minecraft:green_bed",
            Item::PolishedBlackstoneBricks => "minecraft:polished_blackstone_bricks",
            Item::RedConcretePowder => "minecraft:red_concrete_powder",
            Item::MusicDiscStal => "minecraft:music_disc_stal",
            Item::CrimsonHyphae => "minecraft:crimson_hyphae",
            Item::FloweringAzaleaLeaves => "minecraft:flowering_azalea_leaves",
            Item::PolishedBlackstoneWall => "minecraft:polished_blackstone_wall",
            Item::PolishedDioriteStairs => "minecraft:polished_diorite_stairs",
            Item::BlackStainedGlass => "minecraft:black_stained_glass",
            Item::GreenWool => "minecraft:green_wool",
            Item::AcaciaStairs => "minecraft:acacia_stairs",
            Item::Chain => "minecraft:chain",
            Item::LightBlueWool => "minecraft:light_blue_wool",
            Item::BlackStainedGlassPane => "minecraft:black_stained_glass_pane",
            Item::Candle => "minecraft:candle",
            Item::ChippedAnvil => "minecraft:chipped_anvil",
            Item::LavaBucket => "minecraft:lava_bucket",
            Item::SalmonSpawnEgg => "minecraft:salmon_spawn_egg",
            Item::Stick => "minecraft:stick",
            Item::Dirt => "minecraft:dirt",
            Item::CyanStainedGlass => "minecraft:cyan_stained_glass",
            Item::StrippedJungleWood => "minecraft:stripped_jungle_wood",
            Item::CrimsonRoots => "minecraft:crimson_roots",
            Item::SeaLantern => "minecraft:sea_lantern",
            Item::HeavyWeightedPressurePlate => "minecraft:heavy_weighted_pressure_plate",
            Item::WhiteDye => "minecraft:white_dye",
            Item::GreenDye => "minecraft:green_dye",
            Item::JungleWood => "minecraft:jungle_wood",
            Item::BeeNest => "minecraft:bee_nest",
            Item::DarkPrismarine => "minecraft:dark_prismarine",
            Item::DarkPrismarineSlab => "minecraft:dark_prismarine_slab",
            Item::ChainmailLeggings => "minecraft:chainmail_leggings",
            Item::TntMinecart => "minecraft:tnt_minecart",
            Item::DarkOakBoat => "minecraft:dark_oak_boat",
            Item::IronBars => "minecraft:iron_bars",
            Item::Lilac => "minecraft:lilac",
            Item::CyanBanner => "minecraft:cyan_banner",
            Item::YellowConcrete => "minecraft:yellow_concrete",
            Item::LightGrayStainedGlass => "minecraft:light_gray_stained_glass",
            Item::Cobblestone => "minecraft:cobblestone",
            Item::GrassBlock => "minecraft:grass_block",
            Item::WaxedOxidizedCutCopperStairs => "minecraft:waxed_oxidized_cut_copper_stairs",
            Item::GrayStainedGlass => "minecraft:gray_stained_glass",
            Item::BlackWool => "minecraft:black_wool",
            Item::BlazeSpawnEgg => "minecraft:blaze_spawn_egg",
            Item::BirchSign => "minecraft:birch_sign",
            Item::InfestedCrackedStoneBricks => "minecraft:infested_cracked_stone_bricks",
            Item::MusicDiscStrad => "minecraft:music_disc_strad",
            Item::BrownMushroomBlock => "minecraft:brown_mushroom_block",
            Item::BrownStainedGlass => "minecraft:brown_stained_glass",
            Item::BeetrootSeeds => "minecraft:beetroot_seeds",
            Item::StrippedOakLog => "minecraft:stripped_oak_log",
            Item::PurpurStairs => "minecraft:purpur_stairs",
            Item::GlowstoneDust => "minecraft:glowstone_dust",
            Item::SmoothQuartzStairs => "minecraft:smooth_quartz_stairs",
            Item::Stone => "minecraft:stone",
            Item::SandstoneWall => "minecraft:sandstone_wall",
            Item::TurtleSpawnEgg => "minecraft:turtle_spawn_egg",
            Item::DarkOakLog => "minecraft:dark_oak_log",
            Item::ClayBall => "minecraft:clay_ball",
            Item::CrimsonSlab => "minecraft:crimson_slab",
            Item::WarpedDoor => "minecraft:warped_door",
            Item::StoneShovel => "minecraft:stone_shovel",
            Item::ShulkerBox => "minecraft:shulker_box",
            Item::Egg => "minecraft:egg",
            Item::MossBlock => "minecraft:moss_block",
            Item::WhiteGlazedTerracotta => "minecraft:white_glazed_terracotta",
            Item::EndStoneBrickStairs => "minecraft:end_stone_brick_stairs",
            Item::PoisonousPotato => "minecraft:poisonous_potato",
            Item::RedTulip => "minecraft:red_tulip",
            Item::PolishedBasalt => "minecraft:polished_basalt",
            Item::BlackstoneStairs => "minecraft:blackstone_stairs",
            Item::SnowBlock => "minecraft:snow_block",
            Item::Ice => "minecraft:ice",
            Item::WeatheredCutCopperStairs => "minecraft:weathered_cut_copper_stairs",
            Item::DiamondHorseArmor => "minecraft:diamond_horse_armor",
            Item::BirchLog => "minecraft:birch_log",
            Item::InfestedMossyStoneBricks => "minecraft:infested_mossy_stone_bricks",
            Item::OrangeTerracotta => "minecraft:orange_terracotta",
            Item::PinkTerracotta => "minecraft:pink_terracotta",
            Item::Composter => "minecraft:composter",
            Item::IronLeggings => "minecraft:iron_leggings",
            Item::MagentaWool => "minecraft:magenta_wool",
            Item::GoldenHelmet => "minecraft:golden_helmet",
            Item::LimeBed => "minecraft:lime_bed",
            Item::ChainCommandBlock => "minecraft:chain_command_block",
            Item::Podzol => "minecraft:podzol",
            Item::Hopper => "minecraft:hopper",
            Item::CopperIngot => "minecraft:copper_ingot",
            Item::SlimeBlock => "minecraft:slime_block",
            Item::TraderLlamaSpawnEgg => "minecraft:trader_llama_spawn_egg",
            Item::LightBlueConcrete => "minecraft:light_blue_concrete",
            Item::GlowSquidSpawnEgg => "minecraft:glow_squid_spawn_egg",
            Item::PolishedDiorite => "minecraft:polished_diorite",
            Item::Melon => "minecraft:melon",
            Item::WhiteBed => "minecraft:white_bed",
            Item::PetrifiedOakSlab => "minecraft:petrified_oak_slab",
            Item::CrimsonSign => "minecraft:crimson_sign",
            Item::PrismarineSlab => "minecraft:prismarine_slab",
            Item::LightGrayBanner => "minecraft:light_gray_banner",
            Item::CrimsonFungus => "minecraft:crimson_fungus",
            Item::RabbitSpawnEgg => "minecraft:rabbit_spawn_egg",
            Item::RedBanner => "minecraft:red_banner",
            Item::PoppedChorusFruit => "minecraft:popped_chorus_fruit",
            Item::LimeCarpet => "minecraft:lime_carpet",
            Item::IronNugget => "minecraft:iron_nugget",
            Item::StrippedCrimsonHyphae => "minecraft:stripped_crimson_hyphae",
            Item::Dandelion => "minecraft:dandelion",
            Item::CreeperBannerPattern => "minecraft:creeper_banner_pattern",
            Item::IronBoots => "minecraft:iron_boots",
            Item::OxidizedCutCopper => "minecraft:oxidized_cut_copper",
            Item::GoldenSword => "minecraft:golden_sword",
            Item::DiamondPickaxe => "minecraft:diamond_pickaxe",
            Item::Target => "minecraft:target",
            Item::Terracotta => "minecraft:terracotta",
            Item::GreenBanner => "minecraft:green_banner",
            Item::DolphinSpawnEgg => "minecraft:dolphin_spawn_egg",
            Item::RedSand => "minecraft:red_sand",
            Item::BrickStairs => "minecraft:brick_stairs",
            Item::FireworkRocket => "minecraft:firework_rocket",
            Item::RedShulkerBox => "minecraft:red_shulker_box",
            Item::PointedDripstone => "minecraft:pointed_dripstone",
            Item::LightGrayCarpet => "minecraft:light_gray_carpet",
            Item::JungleFence => "minecraft:jungle_fence",
            Item::CoalOre => "minecraft:coal_ore",
            Item::JungleDoor => "minecraft:jungle_door",
            Item::LightningRod => "minecraft:lightning_rod",
            Item::GoldenCarrot => "minecraft:golden_carrot",
            Item::Grindstone => "minecraft:grindstone",
            Item::RedSandstoneSlab => "minecraft:red_sandstone_slab",
            Item::DeadBrainCoralFan => "minecraft:dead_brain_coral_fan",
            Item::OrangeConcretePowder => "minecraft:orange_concrete_powder",
            Item::WarpedButton => "minecraft:warped_button",
            Item::DeadTubeCoralFan => "minecraft:dead_tube_coral_fan",
            Item::PolishedBlackstonePressurePlate => "minecraft:polished_blackstone_pressure_plate",
            Item::GreenTerracotta => "minecraft:green_terracotta",
            Item::TubeCoral => "minecraft:tube_coral",
            Item::JungleLog => "minecraft:jungle_log",
            Item::WarpedNylium => "minecraft:warped_nylium",
            Item::GuardianSpawnEgg => "minecraft:guardian_spawn_egg",
            Item::BrickSlab => "minecraft:brick_slab",
            Item::StrippedOakWood => "minecraft:stripped_oak_wood",
            Item::SculkSensor => "minecraft:sculk_sensor",
            Item::BirchStairs => "minecraft:birch_stairs",
            Item::MooshroomSpawnEgg => "minecraft:mooshroom_spawn_egg",
            Item::FlowerBannerPattern => "minecraft:flower_banner_pattern",
            Item::CryingObsidian => "minecraft:crying_obsidian",
            Item::QuartzBlock => "minecraft:quartz_block",
            Item::CyanTerracotta => "minecraft:cyan_terracotta",
            Item::Observer => "minecraft:observer",
            Item::InfestedChiseledStoneBricks => "minecraft:infested_chiseled_stone_bricks",
            Item::WoodenAxe => "minecraft:wooden_axe",
            Item::YellowCandle => "minecraft:yellow_candle",
            Item::WritableBook => "minecraft:writable_book",
            Item::CreeperSpawnEgg => "minecraft:creeper_spawn_egg",
            Item::NetherBricks => "minecraft:nether_bricks",
            Item::BlackTerracotta => "minecraft:black_terracotta",
            Item::DeadBubbleCoral => "minecraft:dead_bubble_coral",
            Item::CrimsonPressurePlate => "minecraft:crimson_pressure_plate",
            Item::GoldOre => "minecraft:gold_ore",
            Item::DeadHornCoralFan => "minecraft:dead_horn_coral_fan",
            Item::FurnaceMinecart => "minecraft:furnace_minecart",
            Item::Shears => "minecraft:shears",
            Item::SmoothStoneSlab => "minecraft:smooth_stone_slab",
            Item::WitherRose => "minecraft:wither_rose",
            Item::DeepslateBrickWall => "minecraft:deepslate_brick_wall",
            Item::PurpleGlazedTerracotta => "minecraft:purple_glazed_terracotta",
            Item::JackOLantern => "minecraft:jack_o_lantern",
            Item::HornCoralBlock => "minecraft:horn_coral_block",
            Item::SpruceFence => "minecraft:spruce_fence",
            Item::LightGrayGlazedTerracotta => "minecraft:light_gray_glazed_terracotta",
            Item::GlowBerries => "minecraft:glow_berries",
            Item::CodBucket => "minecraft:cod_bucket",
            Item::Sunflower => "minecraft:sunflower",
            Item::OrangeCarpet => "minecraft:orange_carpet",
            Item::BlueCarpet => "minecraft:blue_carpet",
            Item::SeaPickle => "minecraft:sea_pickle",
            Item::PinkShulkerBox => "minecraft:pink_shulker_box",
            Item::RedGlazedTerracotta => "minecraft:red_glazed_terracotta",
            Item::BrainCoral => "minecraft:brain_coral",
            Item::DragonHead => "minecraft:dragon_head",
            Item::MagentaCandle => "minecraft:magenta_candle",
            Item::DeepslateGoldOre => "minecraft:deepslate_gold_ore",
            Item::StrippedSpruceLog => "minecraft:stripped_spruce_log",
            Item::DonkeySpawnEgg => "minecraft:donkey_spawn_egg",
            Item::LimeConcretePowder => "minecraft:lime_concrete_powder",
            Item::PrismarineBricks => "minecraft:prismarine_bricks",
            Item::BrownBanner => "minecraft:brown_banner",
            Item::WhiteStainedGlass => "minecraft:white_stained_glass",
            Item::BlueStainedGlass => "minecraft:blue_stained_glass",
            Item::DiamondHoe => "minecraft:diamond_hoe",
            Item::SpruceSign => "minecraft:spruce_sign",
            Item::Lodestone => "minecraft:lodestone",
            Item::Sugar => "minecraft:sugar",
            Item::SmoothRedSandstoneStairs => "minecraft:smooth_red_sandstone_stairs",
            Item::LightBlueShulkerBox => "minecraft:light_blue_shulker_box",
            Item::CyanDye => "minecraft:cyan_dye",
            Item::WeatheredCutCopper => "minecraft:weathered_cut_copper",
            Item::SpruceBoat => "minecraft:spruce_boat",
            Item::SandstoneStairs => "minecraft:sandstone_stairs",
            Item::MagmaBlock => "minecraft:magma_block",
            Item::LightGrayShulkerBox => "minecraft:light_gray_shulker_box",
            Item::PrismarineWall => "minecraft:prismarine_wall",
            Item::TubeCoralFan => "minecraft:tube_coral_fan",
            Item::Coal => "minecraft:coal",
            Item::ItemFrame => "minecraft:item_frame",
            Item::YellowBanner => "minecraft:yellow_banner",
            Item::SmallAmethystBud => "minecraft:small_amethyst_bud",
            Item::BrownShulkerBox => "minecraft:brown_shulker_box",
            Item::IronOre => "minecraft:iron_ore",
            Item::SoulLantern => "minecraft:soul_lantern",
            Item::GoldenPickaxe => "minecraft:golden_pickaxe",
            Item::QuartzSlab => "minecraft:quartz_slab",
            Item::LimeTerracotta => "minecraft:lime_terracotta",
            Item::BrainCoralFan => "minecraft:brain_coral_fan",
            Item::StrippedJungleLog => "minecraft:stripped_jungle_log",
            Item::TurtleHelmet => "minecraft:turtle_helmet",
            Item::PurpurSlab => "minecraft:purpur_slab",
            Item::MagentaBed => "minecraft:magenta_bed",
            Item::WarpedHyphae => "minecraft:warped_hyphae",
            Item::HoneycombBlock => "minecraft:honeycomb_block",
            Item::PurpleStainedGlassPane => "minecraft:purple_stained_glass_pane",
            Item::DiamondOre => "minecraft:diamond_ore",
            Item::Calcite => "minecraft:calcite",
            Item::OakBoat => "minecraft:oak_boat",
            Item::HoglinSpawnEgg => "minecraft:hoglin_spawn_egg",
            Item::RespawnAnchor => "minecraft:respawn_anchor",
            Item::MagentaBanner => "minecraft:magenta_banner",
            Item::ChiseledPolishedBlackstone => "minecraft:chiseled_polished_blackstone",
            Item::BuddingAmethyst => "minecraft:budding_amethyst",
            Item::Glass => "minecraft:glass",
            Item::IronIngot => "minecraft:iron_ingot",
            Item::DeepslateDiamondOre => "minecraft:deepslate_diamond_ore",
            Item::AcaciaDoor => "minecraft:acacia_door",
            Item::GrayBed => "minecraft:gray_bed",
            Item::LargeFern => "minecraft:large_fern",
            Item::RedStainedGlass => "minecraft:red_stained_glass",
            Item::Brick => "minecraft:brick",
            Item::OakLeaves => "minecraft:oak_leaves",
            Item::ElderGuardianSpawnEgg => "minecraft:elder_guardian_spawn_egg",
            Item::GoldenLeggings => "minecraft:golden_leggings",
            Item::PinkStainedGlass => "minecraft:pink_stained_glass",
            Item::RedSandstoneWall => "minecraft:red_sandstone_wall",
            Item::GlowItemFrame => "minecraft:glow_item_frame",
            Item::Bell => "minecraft:bell",
            Item::CobbledDeepslateStairs => "minecraft:cobbled_deepslate_stairs",
            Item::WolfSpawnEgg => "minecraft:wolf_spawn_egg",
            Item::KnowledgeBook => "minecraft:knowledge_book",
            Item::CookedMutton => "minecraft:cooked_mutton",
            Item::SmoothStone => "minecraft:smooth_stone",
            Item::LimeGlazedTerracotta => "minecraft:lime_glazed_terracotta",
            Item::NetheriteHoe => "minecraft:netherite_hoe",
            Item::OakPlanks => "minecraft:oak_planks",
            Item::MagentaDye => "minecraft:magenta_dye",
            Item::CrackedStoneBricks => "minecraft:cracked_stone_bricks",
            Item::CookedBeef => "minecraft:cooked_beef",
            Item::CoarseDirt => "minecraft:coarse_dirt",
            Item::EnderChest => "minecraft:ender_chest",
            Item::ZombieSpawnEgg => "minecraft:zombie_spawn_egg",
            Item::WarpedFenceGate => "minecraft:warped_fence_gate",
            Item::Bucket => "minecraft:bucket",
            Item::BlueDye => "minecraft:blue_dye",
            Item::Cactus => "minecraft:cactus",
            Item::Lead => "minecraft:lead",
            Item::MushroomStew => "minecraft:mushroom_stew",
            Item::LightBlueBed => "minecraft:light_blue_bed",
            Item::BeeSpawnEgg => "minecraft:bee_spawn_egg",
            Item::OxidizedCutCopperStairs => "minecraft:oxidized_cut_copper_stairs",
            Item::Kelp => "minecraft:kelp",
            Item::GreenConcretePowder => "minecraft:green_concrete_powder",
            Item::RedstoneTorch => "minecraft:redstone_torch",
            Item::Snowball => "minecraft:snowball",
            Item::SpruceSapling => "minecraft:spruce_sapling",
            Item::GreenCandle => "minecraft:green_candle",
            Item::OxidizedCopper => "minecraft:oxidized_copper",
            Item::StrippedAcaciaLog => "minecraft:stripped_acacia_log",
            Item::ZombieHorseSpawnEgg => "minecraft:zombie_horse_spawn_egg",
            Item::AcaciaSlab => "minecraft:acacia_slab",
            Item::SpruceDoor => "minecraft:spruce_door",
            Item::DarkOakSign => "minecraft:dark_oak_sign",
            Item::GreenConcrete => "minecraft:green_concrete",
            Item::Salmon => "minecraft:salmon",
            Item::DeadFireCoralFan => "minecraft:dead_fire_coral_fan",
            Item::BlueStainedGlassPane => "minecraft:blue_stained_glass_pane",
            Item::OakFenceGate => "minecraft:oak_fence_gate",
            Item::Minecart => "minecraft:minecart",
            Item::FishingRod => "minecraft:fishing_rod",
            Item::VindicatorSpawnEgg => "minecraft:vindicator_spawn_egg",
            Item::Cauldron => "minecraft:cauldron",
            Item::SpruceButton => "minecraft:spruce_button",
            Item::Dispenser => "minecraft:dispenser",
            Item::StrippedSpruceWood => "minecraft:stripped_spruce_wood",
            Item::YellowBed => "minecraft:yellow_bed",
            Item::NetherBrickWall => "minecraft:nether_brick_wall",
            Item::JungleSapling => "minecraft:jungle_sapling",
            Item::DamagedAnvil => "minecraft:damaged_anvil",
            Item::Quartz => "minecraft:quartz",
            Item::BrownConcretePowder => "minecraft:brown_concrete_powder",
            Item::TallGrass => "minecraft:tall_grass",
            Item::PolishedAndesiteStairs => "minecraft:polished_andesite_stairs",
            Item::RawGold => "minecraft:raw_gold",
            Item::Cobweb => "minecraft:cobweb",
            Item::BirchLeaves => "minecraft:birch_leaves",
            Item::NetherWartBlock => "minecraft:nether_wart_block",
            Item::GreenStainedGlass => "minecraft:green_stained_glass",
            Item::Bow => "minecraft:bow",
            Item::CrimsonPlanks => "minecraft:crimson_planks",
            Item::DeepslateCopperOre => "minecraft:deepslate_copper_ore",
            Item::FlowerPot => "minecraft:flower_pot",
            Item::WhiteCarpet => "minecraft:white_carpet",
            Item::Allium => "minecraft:allium",
            Item::StrippedDarkOakWood => "minecraft:stripped_dark_oak_wood",
            Item::Basalt => "minecraft:basalt",
            Item::PolishedAndesite => "minecraft:polished_andesite",
            Item::Saddle => "minecraft:saddle",
            Item::Bowl => "minecraft:bowl",
            Item::EnderEye => "minecraft:ender_eye",
            Item::DaylightDetector => "minecraft:daylight_detector",
            Item::Grass => "minecraft:grass",
            Item::MossyCobblestoneWall => "minecraft:mossy_cobblestone_wall",
            Item::YellowDye => "minecraft:yellow_dye",
            Item::EndermanSpawnEgg => "minecraft:enderman_spawn_egg",
            Item::ParrotSpawnEgg => "minecraft:parrot_spawn_egg",
            Item::JunglePressurePlate => "minecraft:jungle_pressure_plate",
            Item::WaxedExposedCutCopperSlab => "minecraft:waxed_exposed_cut_copper_slab",
            Item::Vine => "minecraft:vine",
            Item::PurpurBlock => "minecraft:purpur_block",
            Item::RabbitFoot => "minecraft:rabbit_foot",
            Item::DiamondChestplate => "minecraft:diamond_chestplate",
            Item::Bricks => "minecraft:bricks",
            Item::MusicDiscFar => "minecraft:music_disc_far",
            Item::OakStairs => "minecraft:oak_stairs",
            Item::HoneyBlock => "minecraft:honey_block",
            Item::CarrotOnAStick => "minecraft:carrot_on_a_stick",
            Item::QuartzStairs => "minecraft:quartz_stairs",
            Item::AcaciaFenceGate => "minecraft:acacia_fence_gate",
            Item::DeadBush => "minecraft:dead_bush",
            Item::CookedChicken => "minecraft:cooked_chicken",
            Item::PurpleShulkerBox => "minecraft:purple_shulker_box",
            Item::PackedIce => "minecraft:packed_ice",
            Item::ChiseledDeepslate => "minecraft:chiseled_deepslate",
            Item::PhantomMembrane => "minecraft:phantom_membrane",
            Item::SkullBannerPattern => "minecraft:skull_banner_pattern",
            Item::WaxedWeatheredCutCopperStairs => "minecraft:waxed_weathered_cut_copper_stairs",
            Item::ExposedCutCopperStairs => "minecraft:exposed_cut_copper_stairs",
            Item::CyanCarpet => "minecraft:cyan_carpet",
            Item::SmoothBasalt => "minecraft:smooth_basalt",
            Item::OrangeBed => "minecraft:orange_bed",
            Item::PandaSpawnEgg => "minecraft:panda_spawn_egg",
            Item::AcaciaBoat => "minecraft:acacia_boat",
            Item::AcaciaFence => "minecraft:acacia_fence",
            Item::WitherSkeletonSpawnEgg => "minecraft:wither_skeleton_spawn_egg",
            Item::Torch => "minecraft:torch",
            Item::RabbitStew => "minecraft:rabbit_stew",
            Item::NetheriteShovel => "minecraft:netherite_shovel",
            Item::Campfire => "minecraft:campfire",
            Item::BirchSapling => "minecraft:birch_sapling",
            Item::WhiteBanner => "minecraft:white_banner",
            Item::WaxedOxidizedCopper => "minecraft:waxed_oxidized_copper",
            Item::AzaleaLeaves => "minecraft:azalea_leaves",
            Item::GraniteStairs => "minecraft:granite_stairs",
            Item::GrayShulkerBox => "minecraft:gray_shulker_box",
            Item::OxeyeDaisy => "minecraft:oxeye_daisy",
            Item::DeepslateTileStairs => "minecraft:deepslate_tile_stairs",
            Item::GrayTerracotta => "minecraft:gray_terracotta",
            Item::PinkStainedGlassPane => "minecraft:pink_stained_glass_pane",
            Item::Bookshelf => "minecraft:bookshelf",
            Item::PolishedGraniteSlab => "minecraft:polished_granite_slab",
            Item::DeepslateTileSlab => "minecraft:deepslate_tile_slab",
            Item::Pufferfish => "minecraft:pufferfish",
            Item::DiamondAxe => "minecraft:diamond_axe",
            Item::BlueTerracotta => "minecraft:blue_terracotta",
            Item::RedConcrete => "minecraft:red_concrete",
            Item::CookedCod => "minecraft:cooked_cod",
            Item::PillagerSpawnEgg => "minecraft:pillager_spawn_egg",
            Item::GoldNugget => "minecraft:gold_nugget",
            Item::WarpedFence => "minecraft:warped_fence",
            Item::SpiderEye => "minecraft:spider_eye",
            Item::PrismarineCrystals => "minecraft:prismarine_crystals",
            Item::PolishedGranite => "minecraft:polished_granite",
            Item::LimeStainedGlassPane => "minecraft:lime_stained_glass_pane",
            Item::BirchFenceGate => "minecraft:birch_fence_gate",
            Item::NoteBlock => "minecraft:note_block",
            Item::ShulkerShell => "minecraft:shulker_shell",
            Item::BlazePowder => "minecraft:blaze_powder",
            Item::RedBed => "minecraft:red_bed",
            Item::PolishedDeepslate => "minecraft:polished_deepslate",
            Item::BirchButton => "minecraft:birch_button",
            Item::MusicDiscMall => "minecraft:music_disc_mall",
            Item::FoxSpawnEgg => "minecraft:fox_spawn_egg",
            Item::TippedArrow => "minecraft:tipped_arrow",
            Item::PolishedBlackstoneStairs => "minecraft:polished_blackstone_stairs",
            Item::ZombifiedPiglinSpawnEgg => "minecraft:zombified_piglin_spawn_egg",
            Item::SmallDripleaf => "minecraft:small_dripleaf",
            Item::PinkConcrete => "minecraft:pink_concrete",
            Item::LapisOre => "minecraft:lapis_ore",
            Item::BirchPlanks => "minecraft:birch_planks",
            Item::MelonSlice => "minecraft:melon_slice",
            Item::PolarBearSpawnEgg => "minecraft:polar_bear_spawn_egg",
            Item::CommandBlockMinecart => "minecraft:command_block_minecart",
            Item::MusicDiscPigstep => "minecraft:music_disc_pigstep",
            Item::TintedGlass => "minecraft:tinted_glass",
            Item::WarpedFungusOnAStick => "minecraft:warped_fungus_on_a_stick",
            Item::DeepslateRedstoneOre => "minecraft:deepslate_redstone_ore",
            Item::Bundle => "minecraft:bundle",
            Item::MusicDiscWard => "minecraft:music_disc_ward",
            Item::AmethystCluster => "minecraft:amethyst_cluster",
            Item::StoneAxe => "minecraft:stone_axe",
            Item::StraySpawnEgg => "minecraft:stray_spawn_egg",
            Item::PolishedDioriteSlab => "minecraft:polished_diorite_slab",
            Item::MossyStoneBrickSlab => "minecraft:mossy_stone_brick_slab",
            Item::BlackBanner => "minecraft:black_banner",
            Item::EvokerSpawnEgg => "minecraft:evoker_spawn_egg",
            Item::LightGrayCandle => "minecraft:light_gray_candle",
            Item::PufferfishSpawnEgg => "minecraft:pufferfish_spawn_egg",
            Item::ExperienceBottle => "minecraft:experience_bottle",
            Item::BlackDye => "minecraft:black_dye",
            Item::PrismarineShard => "minecraft:prismarine_shard",
            Item::TurtleEgg => "minecraft:turtle_egg",
            Item::SpruceSlab => "minecraft:spruce_slab",
            Item::Chicken => "minecraft:chicken",
            Item::AcaciaWood => "minecraft:acacia_wood",
            Item::FireCharge => "minecraft:fire_charge",
            Item::Comparator => "minecraft:comparator",
            Item::PoweredRail => "minecraft:powered_rail",
            Item::StoneBrickStairs => "minecraft:stone_brick_stairs",
            Item::BubbleCoralFan => "minecraft:bubble_coral_fan",
            Item::MusicDisc13 => "minecraft:music_disc_13",
            Item::QuartzBricks => "minecraft:quartz_bricks",
            Item::HoneyBottle => "minecraft:honey_bottle",
            Item::YellowConcretePowder => "minecraft:yellow_concrete_powder",
            Item::DeepslateEmeraldOre => "minecraft:deepslate_emerald_ore",
            Item::Book => "minecraft:book",
            Item::Tnt => "minecraft:tnt",
            Item::Scute => "minecraft:scute",
            Item::ChestMinecart => "minecraft:chest_minecart",
            Item::NetheriteAxe => "minecraft:netherite_axe",
            Item::DarkOakButton => "minecraft:dark_oak_button",
            Item::DiamondBoots => "minecraft:diamond_boots",
            Item::TotemOfUndying => "minecraft:totem_of_undying",
            Item::GlassPane => "minecraft:glass_pane",
            Item::PolishedBlackstoneSlab => "minecraft:polished_blackstone_slab",
            Item::CobblestoneWall => "minecraft:cobblestone_wall",
            Item::MagentaConcretePowder => "minecraft:magenta_concrete_powder",
            Item::RedSandstone => "minecraft:red_sandstone",
            Item::OrangeConcrete => "minecraft:orange_concrete",
            Item::Light => "minecraft:light",
            Item::StoneSlab => "minecraft:stone_slab",
            Item::String => "minecraft:string",
            Item::DragonBreath => "minecraft:dragon_breath",
            Item::CrimsonStem => "minecraft:crimson_stem",
        }
    }
    #[doc = "Gets a `Item` by its `namespaced_id`."]
    #[inline]
    pub fn from_namespaced_id(namespaced_id: &str) -> Option<Self> {
        match namespaced_id {
            "minecraft:deepslate_iron_ore" => Some(Item::DeepslateIronOre),
            "minecraft:iron_hoe" => Some(Item::IronHoe),
            "minecraft:golden_boots" => Some(Item::GoldenBoots),
            "minecraft:bone_block" => Some(Item::BoneBlock),
            "minecraft:birch_pressure_plate" => Some(Item::BirchPressurePlate),
            "minecraft:crafting_table" => Some(Item::CraftingTable),
            "minecraft:iron_horse_armor" => Some(Item::IronHorseArmor),
            "minecraft:nether_star" => Some(Item::NetherStar),
            "minecraft:nether_quartz_ore" => Some(Item::NetherQuartzOre),
            "minecraft:wooden_sword" => Some(Item::WoodenSword),
            "minecraft:zombie_head" => Some(Item::ZombieHead),
            "minecraft:ravager_spawn_egg" => Some(Item::RavagerSpawnEgg),
            "minecraft:chorus_flower" => Some(Item::ChorusFlower),
            "minecraft:piglin_banner_pattern" => Some(Item::PiglinBannerPattern),
            "minecraft:brown_candle" => Some(Item::BrownCandle),
            "minecraft:map" => Some(Item::Map),
            "minecraft:birch_trapdoor" => Some(Item::BirchTrapdoor),
            "minecraft:vex_spawn_egg" => Some(Item::VexSpawnEgg),
            "minecraft:activator_rail" => Some(Item::ActivatorRail),
            "minecraft:jungle_planks" => Some(Item::JunglePlanks),
            "minecraft:nether_sprouts" => Some(Item::NetherSprouts),
            "minecraft:cracked_deepslate_tiles" => Some(Item::CrackedDeepslateTiles),
            "minecraft:mossy_stone_brick_stairs" => Some(Item::MossyStoneBrickStairs),
            "minecraft:leather_chestplate" => Some(Item::LeatherChestplate),
            "minecraft:yellow_stained_glass_pane" => Some(Item::YellowStainedGlassPane),
            "minecraft:gray_concrete" => Some(Item::GrayConcrete),
            "minecraft:cyan_candle" => Some(Item::CyanCandle),
            "minecraft:waxed_cut_copper_slab" => Some(Item::WaxedCutCopperSlab),
            "minecraft:black_concrete" => Some(Item::BlackConcrete),
            "minecraft:dead_brain_coral" => Some(Item::DeadBrainCoral),
            "minecraft:honeycomb" => Some(Item::Honeycomb),
            "minecraft:jukebox" => Some(Item::Jukebox),
            "minecraft:farmland" => Some(Item::Farmland),
            "minecraft:white_terracotta" => Some(Item::WhiteTerracotta),
            "minecraft:golden_chestplate" => Some(Item::GoldenChestplate),
            "minecraft:cracked_polished_blackstone_bricks" => {
                Some(Item::CrackedPolishedBlackstoneBricks)
            }
            "minecraft:lectern" => Some(Item::Lectern),
            "minecraft:red_carpet" => Some(Item::RedCarpet),
            "minecraft:prismarine_brick_slab" => Some(Item::PrismarineBrickSlab),
            "minecraft:end_stone_brick_wall" => Some(Item::EndStoneBrickWall),
            "minecraft:soul_soil" => Some(Item::SoulSoil),
            "minecraft:pink_carpet" => Some(Item::PinkCarpet),
            "minecraft:charcoal" => Some(Item::Charcoal),
            "minecraft:bone" => Some(Item::Bone),
            "minecraft:waxed_exposed_cut_copper_stairs" => Some(Item::WaxedExposedCutCopperStairs),
            "minecraft:mossy_cobblestone_slab" => Some(Item::MossyCobblestoneSlab),
            "minecraft:brewing_stand" => Some(Item::BrewingStand),
            "minecraft:red_terracotta" => Some(Item::RedTerracotta),
            "minecraft:elytra" => Some(Item::Elytra),
            "minecraft:tropical_fish" => Some(Item::TropicalFish),
            "minecraft:chainmail_helmet" => Some(Item::ChainmailHelmet),
            "minecraft:jigsaw" => Some(Item::Jigsaw),
            "minecraft:orange_shulker_box" => Some(Item::OrangeShulkerBox),
            "minecraft:dark_oak_pressure_plate" => Some(Item::DarkOakPressurePlate),
            "minecraft:deepslate_lapis_ore" => Some(Item::DeepslateLapisOre),
            "minecraft:orange_stained_glass_pane" => Some(Item::OrangeStainedGlassPane),
            "minecraft:carved_pumpkin" => Some(Item::CarvedPumpkin),
            "minecraft:white_shulker_box" => Some(Item::WhiteShulkerBox),
            "minecraft:mossy_cobblestone_stairs" => Some(Item::MossyCobblestoneStairs),
            "minecraft:deepslate_tiles" => Some(Item::DeepslateTiles),
            "minecraft:melon_seeds" => Some(Item::MelonSeeds),
            "minecraft:lime_candle" => Some(Item::LimeCandle),
            "minecraft:ender_pearl" => Some(Item::EnderPearl),
            "minecraft:shroomlight" => Some(Item::Shroomlight),
            "minecraft:white_wool" => Some(Item::WhiteWool),
            "minecraft:sandstone" => Some(Item::Sandstone),
            "minecraft:white_candle" => Some(Item::WhiteCandle),
            "minecraft:salmon_bucket" => Some(Item::SalmonBucket),
            "minecraft:acacia_leaves" => Some(Item::AcaciaLeaves),
            "minecraft:oak_log" => Some(Item::OakLog),
            "minecraft:beef" => Some(Item::Beef),
            "minecraft:music_disc_blocks" => Some(Item::MusicDiscBlocks),
            "minecraft:andesite_stairs" => Some(Item::AndesiteStairs),
            "minecraft:end_rod" => Some(Item::EndRod),
            "minecraft:end_stone_bricks" => Some(Item::EndStoneBricks),
            "minecraft:crimson_door" => Some(Item::CrimsonDoor),
            "minecraft:stonecutter" => Some(Item::Stonecutter),
            "minecraft:water_bucket" => Some(Item::WaterBucket),
            "minecraft:poppy" => Some(Item::Poppy),
            "minecraft:stone_brick_wall" => Some(Item::StoneBrickWall),
            "minecraft:black_bed" => Some(Item::BlackBed),
            "minecraft:prismarine" => Some(Item::Prismarine),
            "minecraft:exposed_cut_copper" => Some(Item::ExposedCutCopper),
            "minecraft:polished_deepslate_slab" => Some(Item::PolishedDeepslateSlab),
            "minecraft:arrow" => Some(Item::Arrow),
            "minecraft:green_stained_glass_pane" => Some(Item::GreenStainedGlassPane),
            "minecraft:blue_bed" => Some(Item::BlueBed),
            "minecraft:stripped_warped_stem" => Some(Item::StrippedWarpedStem),
            "minecraft:polished_blackstone" => Some(Item::PolishedBlackstone),
            "minecraft:birch_slab" => Some(Item::BirchSlab),
            "minecraft:white_concrete_powder" => Some(Item::WhiteConcretePowder),
            "minecraft:loom" => Some(Item::Loom),
            "minecraft:lily_of_the_valley" => Some(Item::LilyOfTheValley),
            "minecraft:diorite_wall" => Some(Item::DioriteWall),
            "minecraft:black_glazed_terracotta" => Some(Item::BlackGlazedTerracotta),
            "minecraft:rose_bush" => Some(Item::RoseBush),
            "minecraft:green_shulker_box" => Some(Item::GreenShulkerBox),
            "minecraft:blaze_rod" => Some(Item::BlazeRod),
            "minecraft:clock" => Some(Item::Clock),
            "minecraft:spruce_leaves" => Some(Item::SpruceLeaves),
            "minecraft:light_gray_dye" => Some(Item::LightGrayDye),
            "minecraft:mutton" => Some(Item::Mutton),
            "minecraft:music_disc_11" => Some(Item::MusicDisc11),
            "minecraft:red_mushroom_block" => Some(Item::RedMushroomBlock),
            "minecraft:brown_wool" => Some(Item::BrownWool),
            "minecraft:cut_copper" => Some(Item::CutCopper),
            "minecraft:cartography_table" => Some(Item::CartographyTable),
            "minecraft:lily_pad" => Some(Item::LilyPad),
            "minecraft:green_carpet" => Some(Item::GreenCarpet),
            "minecraft:light_weighted_pressure_plate" => Some(Item::LightWeightedPressurePlate),
            "minecraft:dark_oak_sapling" => Some(Item::DarkOakSapling),
            "minecraft:magenta_shulker_box" => Some(Item::MagentaShulkerBox),
            "minecraft:leather_boots" => Some(Item::LeatherBoots),
            "minecraft:deepslate_bricks" => Some(Item::DeepslateBricks),
            "minecraft:amethyst_shard" => Some(Item::AmethystShard),
            "minecraft:jungle_sign" => Some(Item::JungleSign),
            "minecraft:cake" => Some(Item::Cake),
            "minecraft:netherite_block" => Some(Item::NetheriteBlock),
            "minecraft:light_gray_concrete" => Some(Item::LightGrayConcrete),
            "minecraft:dark_oak_wood" => Some(Item::DarkOakWood),
            "minecraft:iron_trapdoor" => Some(Item::IronTrapdoor),
            "minecraft:dead_tube_coral_block" => Some(Item::DeadTubeCoralBlock),
            "minecraft:pink_candle" => Some(Item::PinkCandle),
            "minecraft:exposed_copper" => Some(Item::ExposedCopper),
            "minecraft:dark_oak_planks" => Some(Item::DarkOakPlanks),
            "minecraft:cracked_nether_bricks" => Some(Item::CrackedNetherBricks),
            "minecraft:debug_stick" => Some(Item::DebugStick),
            "minecraft:polished_deepslate_wall" => Some(Item::PolishedDeepslateWall),
            "minecraft:powder_snow_bucket" => Some(Item::PowderSnowBucket),
            "minecraft:cyan_bed" => Some(Item::CyanBed),
            "minecraft:cat_spawn_egg" => Some(Item::CatSpawnEgg),
            "minecraft:flowering_azalea" => Some(Item::FloweringAzalea),
            "minecraft:cod" => Some(Item::Cod),
            "minecraft:music_disc_chirp" => Some(Item::MusicDiscChirp),
            "minecraft:light_blue_terracotta" => Some(Item::LightBlueTerracotta),
            "minecraft:pink_banner" => Some(Item::PinkBanner),
            "minecraft:light_blue_dye" => Some(Item::LightBlueDye),
            "minecraft:purple_dye" => Some(Item::PurpleDye),
            "minecraft:command_block" => Some(Item::CommandBlock),
            "minecraft:dirt_path" => Some(Item::DirtPath),
            "minecraft:stone_button" => Some(Item::StoneButton),
            "minecraft:stone_bricks" => Some(Item::StoneBricks),
            "minecraft:mushroom_stem" => Some(Item::MushroomStem),
            "minecraft:smooth_sandstone" => Some(Item::SmoothSandstone),
            "minecraft:strider_spawn_egg" => Some(Item::StriderSpawnEgg),
            "minecraft:glass_bottle" => Some(Item::GlassBottle),
            "minecraft:skeleton_skull" => Some(Item::SkeletonSkull),
            "minecraft:rotten_flesh" => Some(Item::RottenFlesh),
            "minecraft:bubble_coral_block" => Some(Item::BubbleCoralBlock),
            "minecraft:soul_campfire" => Some(Item::SoulCampfire),
            "minecraft:redstone_ore" => Some(Item::RedstoneOre),
            "minecraft:yellow_glazed_terracotta" => Some(Item::YellowGlazedTerracotta),
            "minecraft:fire_coral_block" => Some(Item::FireCoralBlock),
            "minecraft:chorus_plant" => Some(Item::ChorusPlant),
            "minecraft:blue_concrete" => Some(Item::BlueConcrete),
            "minecraft:raw_copper" => Some(Item::RawCopper),
            "minecraft:blackstone" => Some(Item::Blackstone),
            "minecraft:purple_candle" => Some(Item::PurpleCandle),
            "minecraft:rabbit" => Some(Item::Rabbit),
            "minecraft:waxed_exposed_copper" => Some(Item::WaxedExposedCopper),
            "minecraft:iron_pickaxe" => Some(Item::IronPickaxe),
            "minecraft:granite_wall" => Some(Item::GraniteWall),
            "minecraft:zombie_villager_spawn_egg" => Some(Item::ZombieVillagerSpawnEgg),
            "minecraft:detector_rail" => Some(Item::DetectorRail),
            "minecraft:music_disc_otherside" => Some(Item::MusicDiscOtherside),
            "minecraft:spruce_stairs" => Some(Item::SpruceStairs),
            "minecraft:nether_brick_stairs" => Some(Item::NetherBrickStairs),
            "minecraft:purple_concrete" => Some(Item::PurpleConcrete),
            "minecraft:chiseled_sandstone" => Some(Item::ChiseledSandstone),
            "minecraft:purple_wool" => Some(Item::PurpleWool),
            "minecraft:paper" => Some(Item::Paper),
            "minecraft:red_nether_brick_wall" => Some(Item::RedNetherBrickWall),
            "minecraft:jungle_button" => Some(Item::JungleButton),
            "minecraft:chiseled_nether_bricks" => Some(Item::ChiseledNetherBricks),
            "minecraft:glow_ink_sac" => Some(Item::GlowInkSac),
            "minecraft:magenta_stained_glass_pane" => Some(Item::MagentaStainedGlassPane),
            "minecraft:furnace" => Some(Item::Furnace),
            "minecraft:brown_mushroom" => Some(Item::BrownMushroom),
            "minecraft:sponge" => Some(Item::Sponge),
            "minecraft:stone_pressure_plate" => Some(Item::StonePressurePlate),
            "minecraft:netherite_sword" => Some(Item::NetheriteSword),
            "minecraft:pumpkin" => Some(Item::Pumpkin),
            "minecraft:armor_stand" => Some(Item::ArmorStand),
            "minecraft:bat_spawn_egg" => Some(Item::BatSpawnEgg),
            "minecraft:dark_oak_slab" => Some(Item::DarkOakSlab),
            "minecraft:leather_leggings" => Some(Item::LeatherLeggings),
            "minecraft:horse_spawn_egg" => Some(Item::HorseSpawnEgg),
            "minecraft:bread" => Some(Item::Bread),
            "minecraft:waxed_copper_block" => Some(Item::WaxedCopperBlock),
            "minecraft:waxed_cut_copper" => Some(Item::WaxedCutCopper),
            "minecraft:waxed_weathered_cut_copper_slab" => Some(Item::WaxedWeatheredCutCopperSlab),
            "minecraft:weeping_vines" => Some(Item::WeepingVines),
            "minecraft:enchanting_table" => Some(Item::EnchantingTable),
            "minecraft:cyan_concrete" => Some(Item::CyanConcrete),
            "minecraft:ocelot_spawn_egg" => Some(Item::OcelotSpawnEgg),
            "minecraft:beetroot" => Some(Item::Beetroot),
            "minecraft:pink_concrete_powder" => Some(Item::PinkConcretePowder),
            "minecraft:netherite_boots" => Some(Item::NetheriteBoots),
            "minecraft:golden_shovel" => Some(Item::GoldenShovel),
            "minecraft:azure_bluet" => Some(Item::AzureBluet),
            "minecraft:polished_blackstone_brick_wall" => Some(Item::PolishedBlackstoneBrickWall),
            "minecraft:warped_trapdoor" => Some(Item::WarpedTrapdoor),
            "minecraft:blue_candle" => Some(Item::BlueCandle),
            "minecraft:potion" => Some(Item::Potion),
            "minecraft:medium_amethyst_bud" => Some(Item::MediumAmethystBud),
            "minecraft:blue_glazed_terracotta" => Some(Item::BlueGlazedTerracotta),
            "minecraft:feather" => Some(Item::Feather),
            "minecraft:gunpowder" => Some(Item::Gunpowder),
            "minecraft:acacia_sign" => Some(Item::AcaciaSign),
            "minecraft:ghast_spawn_egg" => Some(Item::GhastSpawnEgg),
            "minecraft:tropical_fish_spawn_egg" => Some(Item::TropicalFishSpawnEgg),
            "minecraft:polished_blackstone_brick_slab" => Some(Item::PolishedBlackstoneBrickSlab),
            "minecraft:oxidized_cut_copper_slab" => Some(Item::OxidizedCutCopperSlab),
            "minecraft:dark_oak_fence_gate" => Some(Item::DarkOakFenceGate),
            "minecraft:light_blue_glazed_terracotta" => Some(Item::LightBlueGlazedTerracotta),
            "minecraft:black_shulker_box" => Some(Item::BlackShulkerBox),
            "minecraft:goat_spawn_egg" => Some(Item::GoatSpawnEgg),
            "minecraft:brick_wall" => Some(Item::BrickWall),
            "minecraft:leather" => Some(Item::Leather),
            "minecraft:copper_ore" => Some(Item::CopperOre),
            "minecraft:purple_bed" => Some(Item::PurpleBed),
            "minecraft:llama_spawn_egg" => Some(Item::LlamaSpawnEgg),
            "minecraft:lingering_potion" => Some(Item::LingeringPotion),
            "minecraft:suspicious_stew" => Some(Item::SuspiciousStew),
            "minecraft:yellow_terracotta" => Some(Item::YellowTerracotta),
            "minecraft:chicken_spawn_egg" => Some(Item::ChickenSpawnEgg),
            "minecraft:stripped_crimson_stem" => Some(Item::StrippedCrimsonStem),
            "minecraft:white_stained_glass_pane" => Some(Item::WhiteStainedGlassPane),
            "minecraft:andesite_slab" => Some(Item::AndesiteSlab),
            "minecraft:diamond_leggings" => Some(Item::DiamondLeggings),
            "minecraft:carrot" => Some(Item::Carrot),
            "minecraft:warped_stem" => Some(Item::WarpedStem),
            "minecraft:warped_stairs" => Some(Item::WarpedStairs),
            "minecraft:light_blue_stained_glass" => Some(Item::LightBlueStainedGlass),
            "minecraft:oak_sign" => Some(Item::OakSign),
            "minecraft:fletching_table" => Some(Item::FletchingTable),
            "minecraft:green_glazed_terracotta" => Some(Item::GreenGlazedTerracotta),
            "minecraft:gray_candle" => Some(Item::GrayCandle),
            "minecraft:mojang_banner_pattern" => Some(Item::MojangBannerPattern),
            "minecraft:dripstone_block" => Some(Item::DripstoneBlock),
            "minecraft:waxed_weathered_copper" => Some(Item::WaxedWeatheredCopper),
            "minecraft:waxed_oxidized_cut_copper" => Some(Item::WaxedOxidizedCutCopper),
            "minecraft:crimson_nylium" => Some(Item::CrimsonNylium),
            "minecraft:repeating_command_block" => Some(Item::RepeatingCommandBlock),
            "minecraft:oak_button" => Some(Item::OakButton),
            "minecraft:orange_tulip" => Some(Item::OrangeTulip),
            "minecraft:light_blue_concrete_powder" => Some(Item::LightBlueConcretePowder),
            "minecraft:sand" => Some(Item::Sand),
            "minecraft:copper_block" => Some(Item::CopperBlock),
            "minecraft:gold_ingot" => Some(Item::GoldIngot),
            "minecraft:apple" => Some(Item::Apple),
            "minecraft:black_candle" => Some(Item::BlackCandle),
            "minecraft:cyan_shulker_box" => Some(Item::CyanShulkerBox),
            "minecraft:birch_door" => Some(Item::BirchDoor),
            "minecraft:stone_sword" => Some(Item::StoneSword),
            "minecraft:jungle_slab" => Some(Item::JungleSlab),
            "minecraft:purple_concrete_powder" => Some(Item::PurpleConcretePowder),
            "minecraft:light_gray_concrete_powder" => Some(Item::LightGrayConcretePowder),
            "minecraft:spider_spawn_egg" => Some(Item::SpiderSpawnEgg),
            "minecraft:large_amethyst_bud" => Some(Item::LargeAmethystBud),
            "minecraft:pink_bed" => Some(Item::PinkBed),
            "minecraft:jungle_trapdoor" => Some(Item::JungleTrapdoor),
            "minecraft:cave_spider_spawn_egg" => Some(Item::CaveSpiderSpawnEgg),
            "minecraft:nether_brick_fence" => Some(Item::NetherBrickFence),
            "minecraft:pig_spawn_egg" => Some(Item::PigSpawnEgg),
            "minecraft:birch_wood" => Some(Item::BirchWood),
            "minecraft:jungle_stairs" => Some(Item::JungleStairs),
            "minecraft:beehive" => Some(Item::Beehive),
            "minecraft:red_sandstone_stairs" => Some(Item::RedSandstoneStairs),
            "minecraft:villager_spawn_egg" => Some(Item::VillagerSpawnEgg),
            "minecraft:cut_sandstone_slab" => Some(Item::CutSandstoneSlab),
            "minecraft:warped_fungus" => Some(Item::WarpedFungus),
            "minecraft:slime_spawn_egg" => Some(Item::SlimeSpawnEgg),
            "minecraft:spruce_wood" => Some(Item::SpruceWood),
            "minecraft:deepslate" => Some(Item::Deepslate),
            "minecraft:yellow_shulker_box" => Some(Item::YellowShulkerBox),
            "minecraft:dark_prismarine_stairs" => Some(Item::DarkPrismarineStairs),
            "minecraft:hopper_minecart" => Some(Item::HopperMinecart),
            "minecraft:dead_bubble_coral_block" => Some(Item::DeadBubbleCoralBlock),
            "minecraft:raw_copper_block" => Some(Item::RawCopperBlock),
            "minecraft:flint" => Some(Item::Flint),
            "minecraft:smooth_quartz_slab" => Some(Item::SmoothQuartzSlab),
            "minecraft:structure_block" => Some(Item::StructureBlock),
            "minecraft:iron_sword" => Some(Item::IronSword),
            "minecraft:compass" => Some(Item::Compass),
            "minecraft:golden_hoe" => Some(Item::GoldenHoe),
            "minecraft:peony" => Some(Item::Peony),
            "minecraft:netherite_chestplate" => Some(Item::NetheriteChestplate),
            "minecraft:iron_block" => Some(Item::IronBlock),
            "minecraft:crimson_trapdoor" => Some(Item::CrimsonTrapdoor),
            "minecraft:globe_banner_pattern" => Some(Item::GlobeBannerPattern),
            "minecraft:light_gray_terracotta" => Some(Item::LightGrayTerracotta),
            "minecraft:mossy_stone_bricks" => Some(Item::MossyStoneBricks),
            "minecraft:light_gray_bed" => Some(Item::LightGrayBed),
            "minecraft:sticky_piston" => Some(Item::StickyPiston),
            "minecraft:cyan_stained_glass_pane" => Some(Item::CyanStainedGlassPane),
            "minecraft:polished_deepslate_stairs" => Some(Item::PolishedDeepslateStairs),
            "minecraft:spruce_pressure_plate" => Some(Item::SprucePressurePlate),
            "minecraft:cooked_salmon" => Some(Item::CookedSalmon),
            "minecraft:granite" => Some(Item::Granite),
            "minecraft:red_mushroom" => Some(Item::RedMushroom),
            "minecraft:end_stone" => Some(Item::EndStone),
            "minecraft:nether_brick" => Some(Item::NetherBrick),
            "minecraft:crimson_button" => Some(Item::CrimsonButton),
            "minecraft:dried_kelp" => Some(Item::DriedKelp),
            "minecraft:sugar_cane" => Some(Item::SugarCane),
            "minecraft:fire_coral_fan" => Some(Item::FireCoralFan),
            "minecraft:blue_shulker_box" => Some(Item::BlueShulkerBox),
            "minecraft:creeper_head" => Some(Item::CreeperHead),
            "minecraft:smithing_table" => Some(Item::SmithingTable),
            "minecraft:azalea" => Some(Item::Azalea),
            "minecraft:fermented_spider_eye" => Some(Item::FermentedSpiderEye),
            "minecraft:birch_boat" => Some(Item::BirchBoat),
            "minecraft:deepslate_tile_wall" => Some(Item::DeepslateTileWall),
            "minecraft:stone_hoe" => Some(Item::StoneHoe),
            "minecraft:gray_stained_glass_pane" => Some(Item::GrayStainedGlassPane),
            "minecraft:diamond_helmet" => Some(Item::DiamondHelmet),
            "minecraft:dead_brain_coral_block" => Some(Item::DeadBrainCoralBlock),
            "minecraft:cut_sandstone" => Some(Item::CutSandstone),
            "minecraft:slime_ball" => Some(Item::SlimeBall),
            "minecraft:gilded_blackstone" => Some(Item::GildedBlackstone),
            "minecraft:witch_spawn_egg" => Some(Item::WitchSpawnEgg),
            "minecraft:blackstone_wall" => Some(Item::BlackstoneWall),
            "minecraft:fire_coral" => Some(Item::FireCoral),
            "minecraft:redstone" => Some(Item::Redstone),
            "minecraft:written_book" => Some(Item::WrittenBook),
            "minecraft:waxed_cut_copper_stairs" => Some(Item::WaxedCutCopperStairs),
            "minecraft:granite_slab" => Some(Item::GraniteSlab),
            "minecraft:scaffolding" => Some(Item::Scaffolding),
            "minecraft:lime_dye" => Some(Item::LimeDye),
            "minecraft:polished_blackstone_button" => Some(Item::PolishedBlackstoneButton),
            "minecraft:dead_bubble_coral_fan" => Some(Item::DeadBubbleCoralFan),
            "minecraft:glowstone" => Some(Item::Glowstone),
            "minecraft:horn_coral" => Some(Item::HornCoral),
            "minecraft:squid_spawn_egg" => Some(Item::SquidSpawnEgg),
            "minecraft:yellow_carpet" => Some(Item::YellowCarpet),
            "minecraft:lapis_lazuli" => Some(Item::LapisLazuli),
            "minecraft:warped_wart_block" => Some(Item::WarpedWartBlock),
            "minecraft:smooth_quartz" => Some(Item::SmoothQuartz),
            "minecraft:cobbled_deepslate" => Some(Item::CobbledDeepslate),
            "minecraft:tripwire_hook" => Some(Item::TripwireHook),
            "minecraft:dropper" => Some(Item::Dropper),
            "minecraft:magenta_glazed_terracotta" => Some(Item::MagentaGlazedTerracotta),
            "minecraft:soul_sand" => Some(Item::SoulSand),
            "minecraft:potato" => Some(Item::Potato),
            "minecraft:magma_cream" => Some(Item::MagmaCream),
            "minecraft:cow_spawn_egg" => Some(Item::CowSpawnEgg),
            "minecraft:trapped_chest" => Some(Item::TrappedChest),
            "minecraft:jungle_fence_gate" => Some(Item::JungleFenceGate),
            "minecraft:golden_apple" => Some(Item::GoldenApple),
            "minecraft:dark_oak_fence" => Some(Item::DarkOakFence),
            "minecraft:dead_horn_coral" => Some(Item::DeadHornCoral),
            "minecraft:dark_oak_leaves" => Some(Item::DarkOakLeaves),
            "minecraft:dead_fire_coral" => Some(Item::DeadFireCoral),
            "minecraft:iron_shovel" => Some(Item::IronShovel),
            "minecraft:enchanted_book" => Some(Item::EnchantedBook),
            "minecraft:white_tulip" => Some(Item::WhiteTulip),
            "minecraft:acacia_button" => Some(Item::AcaciaButton),
            "minecraft:emerald" => Some(Item::Emerald),
            "minecraft:sandstone_slab" => Some(Item::SandstoneSlab),
            "minecraft:iron_door" => Some(Item::IronDoor),
            "minecraft:raw_iron" => Some(Item::RawIron),
            "minecraft:lapis_block" => Some(Item::LapisBlock),
            "minecraft:big_dripleaf" => Some(Item::BigDripleaf),
            "minecraft:blue_concrete_powder" => Some(Item::BlueConcretePowder),
            "minecraft:barrel" => Some(Item::Barrel),
            "minecraft:magenta_stained_glass" => Some(Item::MagentaStainedGlass),
            "minecraft:brown_concrete" => Some(Item::BrownConcrete),
            "minecraft:pink_tulip" => Some(Item::PinkTulip),
            "minecraft:gray_banner" => Some(Item::GrayBanner),
            "minecraft:bedrock" => Some(Item::Bedrock),
            "minecraft:dead_fire_coral_block" => Some(Item::DeadFireCoralBlock),
            "minecraft:anvil" => Some(Item::Anvil),
            "minecraft:stone_pickaxe" => Some(Item::StonePickaxe),
            "minecraft:leather_helmet" => Some(Item::LeatherHelmet),
            "minecraft:porkchop" => Some(Item::Porkchop),
            "minecraft:stone_brick_slab" => Some(Item::StoneBrickSlab),
            "minecraft:orange_candle" => Some(Item::OrangeCandle),
            "minecraft:blue_wool" => Some(Item::BlueWool),
            "minecraft:moss_carpet" => Some(Item::MossCarpet),
            "minecraft:silverfish_spawn_egg" => Some(Item::SilverfishSpawnEgg),
            "minecraft:spectral_arrow" => Some(Item::SpectralArrow),
            "minecraft:cooked_rabbit" => Some(Item::CookedRabbit),
            "minecraft:iron_chestplate" => Some(Item::IronChestplate),
            "minecraft:gray_concrete_powder" => Some(Item::GrayConcretePowder),
            "minecraft:warped_planks" => Some(Item::WarpedPlanks),
            "minecraft:dead_tube_coral" => Some(Item::DeadTubeCoral),
            "minecraft:drowned_spawn_egg" => Some(Item::DrownedSpawnEgg),
            "minecraft:splash_potion" => Some(Item::SplashPotion),
            "minecraft:wooden_shovel" => Some(Item::WoodenShovel),
            "minecraft:warped_roots" => Some(Item::WarpedRoots),
            "minecraft:quartz_pillar" => Some(Item::QuartzPillar),
            "minecraft:glistering_melon_slice" => Some(Item::GlisteringMelonSlice),
            "minecraft:crimson_stairs" => Some(Item::CrimsonStairs),
            "minecraft:obsidian" => Some(Item::Obsidian),
            "minecraft:red_candle" => Some(Item::RedCandle),
            "minecraft:soul_torch" => Some(Item::SoulTorch),
            "minecraft:ghast_tear" => Some(Item::GhastTear),
            "minecraft:diamond" => Some(Item::Diamond),
            "minecraft:end_stone_brick_slab" => Some(Item::EndStoneBrickSlab),
            "minecraft:gray_glazed_terracotta" => Some(Item::GrayGlazedTerracotta),
            "minecraft:oak_trapdoor" => Some(Item::OakTrapdoor),
            "minecraft:lime_banner" => Some(Item::LimeBanner),
            "minecraft:dark_oak_trapdoor" => Some(Item::DarkOakTrapdoor),
            "minecraft:oak_sapling" => Some(Item::OakSapling),
            "minecraft:pufferfish_bucket" => Some(Item::PufferfishBucket),
            "minecraft:clay" => Some(Item::Clay),
            "minecraft:cyan_concrete_powder" => Some(Item::CyanConcretePowder),
            "minecraft:gravel" => Some(Item::Gravel),
            "minecraft:yellow_stained_glass" => Some(Item::YellowStainedGlass),
            "minecraft:gray_wool" => Some(Item::GrayWool),
            "minecraft:blue_ice" => Some(Item::BlueIce),
            "minecraft:stripped_dark_oak_log" => Some(Item::StrippedDarkOakLog),
            "minecraft:fern" => Some(Item::Fern),
            "minecraft:milk_bucket" => Some(Item::MilkBucket),
            "minecraft:brain_coral_block" => Some(Item::BrainCoralBlock),
            "minecraft:orange_banner" => Some(Item::OrangeBanner),
            "minecraft:filled_map" => Some(Item::FilledMap),
            "minecraft:red_nether_brick_slab" => Some(Item::RedNetherBrickSlab),
            "minecraft:polished_granite_stairs" => Some(Item::PolishedGraniteStairs),
            "minecraft:chainmail_boots" => Some(Item::ChainmailBoots),
            "minecraft:oak_door" => Some(Item::OakDoor),
            "minecraft:sheep_spawn_egg" => Some(Item::SheepSpawnEgg),
            "minecraft:oak_wood" => Some(Item::OakWood),
            "minecraft:shield" => Some(Item::Shield),
            "minecraft:blast_furnace" => Some(Item::BlastFurnace),
            "minecraft:red_stained_glass_pane" => Some(Item::RedStainedGlassPane),
            "minecraft:spyglass" => Some(Item::Spyglass),
            "minecraft:wandering_trader_spawn_egg" => Some(Item::WanderingTraderSpawnEgg),
            "minecraft:repeater" => Some(Item::Repeater),
            "minecraft:ladder" => Some(Item::Ladder),
            "minecraft:horn_coral_fan" => Some(Item::HornCoralFan),
            "minecraft:white_concrete" => Some(Item::WhiteConcrete),
            "minecraft:spruce_planks" => Some(Item::SprucePlanks),
            "minecraft:purpur_pillar" => Some(Item::PurpurPillar),
            "minecraft:blackstone_slab" => Some(Item::BlackstoneSlab),
            "minecraft:dark_oak_door" => Some(Item::DarkOakDoor),
            "minecraft:seagrass" => Some(Item::Seagrass),
            "minecraft:deepslate_brick_slab" => Some(Item::DeepslateBrickSlab),
            "minecraft:cut_copper_stairs" => Some(Item::CutCopperStairs),
            "minecraft:stone_stairs" => Some(Item::StoneStairs),
            "minecraft:diorite_slab" => Some(Item::DioriteSlab),
            "minecraft:polished_blackstone_brick_stairs" => {
                Some(Item::PolishedBlackstoneBrickStairs)
            }
            "minecraft:andesite_wall" => Some(Item::AndesiteWall),
            "minecraft:pink_dye" => Some(Item::PinkDye),
            "minecraft:phantom_spawn_egg" => Some(Item::PhantomSpawnEgg),
            "minecraft:lime_stained_glass" => Some(Item::LimeStainedGlass),
            "minecraft:skeleton_horse_spawn_egg" => Some(Item::SkeletonHorseSpawnEgg),
            "minecraft:spruce_fence_gate" => Some(Item::SpruceFenceGate),
            "minecraft:bamboo" => Some(Item::Bamboo),
            "minecraft:dark_oak_stairs" => Some(Item::DarkOakStairs),
            "minecraft:light_blue_banner" => Some(Item::LightBlueBanner),
            "minecraft:conduit" => Some(Item::Conduit),
            "minecraft:purple_carpet" => Some(Item::PurpleCarpet),
            "minecraft:cornflower" => Some(Item::Cornflower),
            "minecraft:wheat_seeds" => Some(Item::WheatSeeds),
            "minecraft:music_disc_mellohi" => Some(Item::MusicDiscMellohi),
            "minecraft:smoker" => Some(Item::Smoker),
            "minecraft:mossy_cobblestone" => Some(Item::MossyCobblestone),
            "minecraft:waxed_weathered_cut_copper" => Some(Item::WaxedWeatheredCutCopper),
            "minecraft:netherrack" => Some(Item::Netherrack),
            "minecraft:brown_terracotta" => Some(Item::BrownTerracotta),
            "minecraft:piston" => Some(Item::Piston),
            "minecraft:chorus_fruit" => Some(Item::ChorusFruit),
            "minecraft:redstone_lamp" => Some(Item::RedstoneLamp),
            "minecraft:acacia_pressure_plate" => Some(Item::AcaciaPressurePlate),
            "minecraft:purple_banner" => Some(Item::PurpleBanner),
            "minecraft:light_blue_carpet" => Some(Item::LightBlueCarpet),
            "minecraft:cyan_wool" => Some(Item::CyanWool),
            "minecraft:cocoa_beans" => Some(Item::CocoaBeans),
            "minecraft:brown_stained_glass_pane" => Some(Item::BrownStainedGlassPane),
            "minecraft:cod_spawn_egg" => Some(Item::CodSpawnEgg),
            "minecraft:crimson_fence" => Some(Item::CrimsonFence),
            "minecraft:leather_horse_armor" => Some(Item::LeatherHorseArmor),
            "minecraft:andesite" => Some(Item::Andesite),
            "minecraft:oak_slab" => Some(Item::OakSlab),
            "minecraft:wither_skeleton_skull" => Some(Item::WitherSkeletonSkull),
            "minecraft:hay_block" => Some(Item::HayBlock),
            "minecraft:brown_glazed_terracotta" => Some(Item::BrownGlazedTerracotta),
            "minecraft:pink_wool" => Some(Item::PinkWool),
            "minecraft:ink_sac" => Some(Item::InkSac),
            "minecraft:acacia_planks" => Some(Item::AcaciaPlanks),
            "minecraft:end_portal_frame" => Some(Item::EndPortalFrame),
            "minecraft:cookie" => Some(Item::Cookie),
            "minecraft:cracked_deepslate_bricks" => Some(Item::CrackedDeepslateBricks),
            "minecraft:firework_star" => Some(Item::FireworkStar),
            "minecraft:light_blue_candle" => Some(Item::LightBlueCandle),
            "minecraft:nether_gold_ore" => Some(Item::NetherGoldOre),
            "minecraft:cut_red_sandstone" => Some(Item::CutRedSandstone),
            "minecraft:cobbled_deepslate_slab" => Some(Item::CobbledDeepslateSlab),
            "minecraft:crimson_fence_gate" => Some(Item::CrimsonFenceGate),
            "minecraft:deepslate_brick_stairs" => Some(Item::DeepslateBrickStairs),
            "minecraft:jungle_boat" => Some(Item::JungleBoat),
            "minecraft:structure_void" => Some(Item::StructureVoid),
            "minecraft:pink_glazed_terracotta" => Some(Item::PinkGlazedTerracotta),
            "minecraft:diorite_stairs" => Some(Item::DioriteStairs),
            "minecraft:weathered_cut_copper_slab" => Some(Item::WeatheredCutCopperSlab),
            "minecraft:magenta_carpet" => Some(Item::MagentaCarpet),
            "minecraft:waxed_oxidized_cut_copper_slab" => Some(Item::WaxedOxidizedCutCopperSlab),
            "minecraft:purple_stained_glass" => Some(Item::PurpleStainedGlass),
            "minecraft:lime_shulker_box" => Some(Item::LimeShulkerBox),
            "minecraft:black_concrete_powder" => Some(Item::BlackConcretePowder),
            "minecraft:rail" => Some(Item::Rail),
            "minecraft:dried_kelp_block" => Some(Item::DriedKelpBlock),
            "minecraft:acacia_log" => Some(Item::AcaciaLog),
            "minecraft:chiseled_quartz_block" => Some(Item::ChiseledQuartzBlock),
            "minecraft:piglin_brute_spawn_egg" => Some(Item::PiglinBruteSpawnEgg),
            "minecraft:deepslate_coal_ore" => Some(Item::DeepslateCoalOre),
            "minecraft:player_head" => Some(Item::PlayerHead),
            "minecraft:chiseled_stone_bricks" => Some(Item::ChiseledStoneBricks),
            "minecraft:acacia_sapling" => Some(Item::AcaciaSapling),
            "minecraft:brown_carpet" => Some(Item::BrownCarpet),
            "minecraft:cobblestone_stairs" => Some(Item::CobblestoneStairs),
            "minecraft:beacon" => Some(Item::Beacon),
            "minecraft:cobbled_deepslate_wall" => Some(Item::CobbledDeepslateWall),
            "minecraft:infested_cobblestone" => Some(Item::InfestedCobblestone),
            "minecraft:warped_pressure_plate" => Some(Item::WarpedPressurePlate),
            "minecraft:netherite_pickaxe" => Some(Item::NetheritePickaxe),
            "minecraft:endermite_spawn_egg" => Some(Item::EndermiteSpawnEgg),
            "minecraft:oak_fence" => Some(Item::OakFence),
            "minecraft:brown_bed" => Some(Item::BrownBed),
            "minecraft:stripped_warped_hyphae" => Some(Item::StrippedWarpedHyphae),
            "minecraft:waxed_exposed_cut_copper" => Some(Item::WaxedExposedCutCopper),
            "minecraft:red_nether_bricks" => Some(Item::RedNetherBricks),
            "minecraft:red_wool" => Some(Item::RedWool),
            "minecraft:polished_andesite_slab" => Some(Item::PolishedAndesiteSlab),
            "minecraft:nether_wart" => Some(Item::NetherWart),
            "minecraft:yellow_wool" => Some(Item::YellowWool),
            "minecraft:black_carpet" => Some(Item::BlackCarpet),
            "minecraft:emerald_block" => Some(Item::EmeraldBlock),
            "minecraft:light_gray_wool" => Some(Item::LightGrayWool),
            "minecraft:infested_deepslate" => Some(Item::InfestedDeepslate),
            "minecraft:smooth_sandstone_slab" => Some(Item::SmoothSandstoneSlab),
            "minecraft:acacia_trapdoor" => Some(Item::AcaciaTrapdoor),
            "minecraft:brown_dye" => Some(Item::BrownDye),
            "minecraft:emerald_ore" => Some(Item::EmeraldOre),
            "minecraft:nautilus_shell" => Some(Item::NautilusShell),
            "minecraft:tuff" => Some(Item::Tuff),
            "minecraft:chest" => Some(Item::Chest),
            "minecraft:snow" => Some(Item::Snow),
            "minecraft:twisting_vines" => Some(Item::TwistingVines),
            "minecraft:gray_dye" => Some(Item::GrayDye),
            "minecraft:smooth_red_sandstone" => Some(Item::SmoothRedSandstone),
            "minecraft:rooted_dirt" => Some(Item::RootedDirt),
            "minecraft:raw_iron_block" => Some(Item::RawIronBlock),
            "minecraft:painting" => Some(Item::Painting),
            "minecraft:shulker_spawn_egg" => Some(Item::ShulkerSpawnEgg),
            "minecraft:flint_and_steel" => Some(Item::FlintAndSteel),
            "minecraft:orange_stained_glass" => Some(Item::OrangeStainedGlass),
            "minecraft:amethyst_block" => Some(Item::AmethystBlock),
            "minecraft:chiseled_red_sandstone" => Some(Item::ChiseledRedSandstone),
            "minecraft:lime_concrete" => Some(Item::LimeConcrete),
            "minecraft:spruce_trapdoor" => Some(Item::SpruceTrapdoor),
            "minecraft:ancient_debris" => Some(Item::AncientDebris),
            "minecraft:light_gray_stained_glass_pane" => Some(Item::LightGrayStainedGlassPane),
            "minecraft:redstone_block" => Some(Item::RedstoneBlock),
            "minecraft:piglin_spawn_egg" => Some(Item::PiglinSpawnEgg),
            "minecraft:name_tag" => Some(Item::NameTag),
            "minecraft:smooth_red_sandstone_slab" => Some(Item::SmoothRedSandstoneSlab),
            "minecraft:warped_sign" => Some(Item::WarpedSign),
            "minecraft:crossbow" => Some(Item::Crossbow),
            "minecraft:axolotl_spawn_egg" => Some(Item::AxolotlSpawnEgg),
            "minecraft:netherite_helmet" => Some(Item::NetheriteHelmet),
            "minecraft:golden_axe" => Some(Item::GoldenAxe),
            "minecraft:music_disc_cat" => Some(Item::MusicDiscCat),
            "minecraft:diorite" => Some(Item::Diorite),
            "minecraft:warped_slab" => Some(Item::WarpedSlab),
            "minecraft:nether_brick_slab" => Some(Item::NetherBrickSlab),
            "minecraft:smooth_sandstone_stairs" => Some(Item::SmoothSandstoneStairs),
            "minecraft:netherite_ingot" => Some(Item::NetheriteIngot),
            "minecraft:hanging_roots" => Some(Item::HangingRoots),
            "minecraft:dead_horn_coral_block" => Some(Item::DeadHornCoralBlock),
            "minecraft:zoglin_spawn_egg" => Some(Item::ZoglinSpawnEgg),
            "minecraft:diamond_block" => Some(Item::DiamondBlock),
            "minecraft:stripped_birch_wood" => Some(Item::StrippedBirchWood),
            "minecraft:orange_glazed_terracotta" => Some(Item::OrangeGlazedTerracotta),
            "minecraft:netherite_leggings" => Some(Item::NetheriteLeggings),
            "minecraft:diamond_shovel" => Some(Item::DiamondShovel),
            "minecraft:trident" => Some(Item::Trident),
            "minecraft:birch_fence" => Some(Item::BirchFence),
            "minecraft:wet_sponge" => Some(Item::WetSponge),
            "minecraft:rabbit_hide" => Some(Item::RabbitHide),
            "minecraft:infested_stone_bricks" => Some(Item::InfestedStoneBricks),
            "minecraft:magenta_terracotta" => Some(Item::MagentaTerracotta),
            "minecraft:weathered_copper" => Some(Item::WeatheredCopper),
            "minecraft:gray_carpet" => Some(Item::GrayCarpet),
            "minecraft:jungle_leaves" => Some(Item::JungleLeaves),
            "minecraft:blue_orchid" => Some(Item::BlueOrchid),
            "minecraft:spawner" => Some(Item::Spawner),
            "minecraft:cyan_glazed_terracotta" => Some(Item::CyanGlazedTerracotta),
            "minecraft:cooked_porkchop" => Some(Item::CookedPorkchop),
            "minecraft:husk_spawn_egg" => Some(Item::HuskSpawnEgg),
            "minecraft:skeleton_spawn_egg" => Some(Item::SkeletonSpawnEgg),
            "minecraft:golden_horse_armor" => Some(Item::GoldenHorseArmor),
            "minecraft:orange_wool" => Some(Item::OrangeWool),
            "minecraft:heart_of_the_sea" => Some(Item::HeartOfTheSea),
            "minecraft:spruce_log" => Some(Item::SpruceLog),
            "minecraft:cobblestone_slab" => Some(Item::CobblestoneSlab),
            "minecraft:bone_meal" => Some(Item::BoneMeal),
            "minecraft:netherite_scrap" => Some(Item::NetheriteScrap),
            "minecraft:music_disc_wait" => Some(Item::MusicDiscWait),
            "minecraft:lime_wool" => Some(Item::LimeWool),
            "minecraft:raw_gold_block" => Some(Item::RawGoldBlock),
            "minecraft:light_blue_stained_glass_pane" => Some(Item::LightBlueStainedGlassPane),
            "minecraft:oak_pressure_plate" => Some(Item::OakPressurePlate),
            "minecraft:wooden_pickaxe" => Some(Item::WoodenPickaxe),
            "minecraft:enchanted_golden_apple" => Some(Item::EnchantedGoldenApple),
            "minecraft:sweet_berries" => Some(Item::SweetBerries),
            "minecraft:red_dye" => Some(Item::RedDye),
            "minecraft:prismarine_brick_stairs" => Some(Item::PrismarineBrickStairs),
            "minecraft:cut_red_sandstone_slab" => Some(Item::CutRedSandstoneSlab),
            "minecraft:gold_block" => Some(Item::GoldBlock),
            "minecraft:iron_helmet" => Some(Item::IronHelmet),
            "minecraft:glow_lichen" => Some(Item::GlowLichen),
            "minecraft:mossy_stone_brick_wall" => Some(Item::MossyStoneBrickWall),
            "minecraft:stripped_birch_log" => Some(Item::StrippedBirchLog),
            "minecraft:chainmail_chestplate" => Some(Item::ChainmailChestplate),
            "minecraft:pumpkin_seeds" => Some(Item::PumpkinSeeds),
            "minecraft:iron_axe" => Some(Item::IronAxe),
            "minecraft:cut_copper_slab" => Some(Item::CutCopperSlab),
            "minecraft:baked_potato" => Some(Item::BakedPotato),
            "minecraft:diamond_sword" => Some(Item::DiamondSword),
            "minecraft:bubble_coral" => Some(Item::BubbleCoral),
            "minecraft:red_nether_brick_stairs" => Some(Item::RedNetherBrickStairs),
            "minecraft:wooden_hoe" => Some(Item::WoodenHoe),
            "minecraft:lantern" => Some(Item::Lantern),
            "minecraft:dragon_egg" => Some(Item::DragonEgg),
            "minecraft:exposed_cut_copper_slab" => Some(Item::ExposedCutCopperSlab),
            "minecraft:magenta_concrete" => Some(Item::MagentaConcrete),
            "minecraft:axolotl_bucket" => Some(Item::AxolotlBucket),
            "minecraft:mule_spawn_egg" => Some(Item::MuleSpawnEgg),
            "minecraft:pumpkin_pie" => Some(Item::PumpkinPie),
            "minecraft:infested_stone" => Some(Item::InfestedStone),
            "minecraft:magma_cube_spawn_egg" => Some(Item::MagmaCubeSpawnEgg),
            "minecraft:tube_coral_block" => Some(Item::TubeCoralBlock),
            "minecraft:blue_banner" => Some(Item::BlueBanner),
            "minecraft:coal_block" => Some(Item::CoalBlock),
            "minecraft:stripped_acacia_wood" => Some(Item::StrippedAcaciaWood),
            "minecraft:beetroot_soup" => Some(Item::BeetrootSoup),
            "minecraft:barrier" => Some(Item::Barrier),
            "minecraft:end_crystal" => Some(Item::EndCrystal),
            "minecraft:mycelium" => Some(Item::Mycelium),
            "minecraft:spore_blossom" => Some(Item::SporeBlossom),
            "minecraft:wheat" => Some(Item::Wheat),
            "minecraft:prismarine_stairs" => Some(Item::PrismarineStairs),
            "minecraft:purple_terracotta" => Some(Item::PurpleTerracotta),
            "minecraft:tropical_fish_bucket" => Some(Item::TropicalFishBucket),
            "minecraft:lever" => Some(Item::Lever),
            "minecraft:orange_dye" => Some(Item::OrangeDye),
            "minecraft:green_bed" => Some(Item::GreenBed),
            "minecraft:polished_blackstone_bricks" => Some(Item::PolishedBlackstoneBricks),
            "minecraft:red_concrete_powder" => Some(Item::RedConcretePowder),
            "minecraft:music_disc_stal" => Some(Item::MusicDiscStal),
            "minecraft:crimson_hyphae" => Some(Item::CrimsonHyphae),
            "minecraft:flowering_azalea_leaves" => Some(Item::FloweringAzaleaLeaves),
            "minecraft:polished_blackstone_wall" => Some(Item::PolishedBlackstoneWall),
            "minecraft:polished_diorite_stairs" => Some(Item::PolishedDioriteStairs),
            "minecraft:black_stained_glass" => Some(Item::BlackStainedGlass),
            "minecraft:green_wool" => Some(Item::GreenWool),
            "minecraft:acacia_stairs" => Some(Item::AcaciaStairs),
            "minecraft:chain" => Some(Item::Chain),
            "minecraft:light_blue_wool" => Some(Item::LightBlueWool),
            "minecraft:black_stained_glass_pane" => Some(Item::BlackStainedGlassPane),
            "minecraft:candle" => Some(Item::Candle),
            "minecraft:chipped_anvil" => Some(Item::ChippedAnvil),
            "minecraft:lava_bucket" => Some(Item::LavaBucket),
            "minecraft:salmon_spawn_egg" => Some(Item::SalmonSpawnEgg),
            "minecraft:stick" => Some(Item::Stick),
            "minecraft:dirt" => Some(Item::Dirt),
            "minecraft:cyan_stained_glass" => Some(Item::CyanStainedGlass),
            "minecraft:stripped_jungle_wood" => Some(Item::StrippedJungleWood),
            "minecraft:crimson_roots" => Some(Item::CrimsonRoots),
            "minecraft:sea_lantern" => Some(Item::SeaLantern),
            "minecraft:heavy_weighted_pressure_plate" => Some(Item::HeavyWeightedPressurePlate),
            "minecraft:white_dye" => Some(Item::WhiteDye),
            "minecraft:green_dye" => Some(Item::GreenDye),
            "minecraft:jungle_wood" => Some(Item::JungleWood),
            "minecraft:bee_nest" => Some(Item::BeeNest),
            "minecraft:dark_prismarine" => Some(Item::DarkPrismarine),
            "minecraft:dark_prismarine_slab" => Some(Item::DarkPrismarineSlab),
            "minecraft:chainmail_leggings" => Some(Item::ChainmailLeggings),
            "minecraft:tnt_minecart" => Some(Item::TntMinecart),
            "minecraft:dark_oak_boat" => Some(Item::DarkOakBoat),
            "minecraft:iron_bars" => Some(Item::IronBars),
            "minecraft:lilac" => Some(Item::Lilac),
            "minecraft:cyan_banner" => Some(Item::CyanBanner),
            "minecraft:yellow_concrete" => Some(Item::YellowConcrete),
            "minecraft:light_gray_stained_glass" => Some(Item::LightGrayStainedGlass),
            "minecraft:cobblestone" => Some(Item::Cobblestone),
            "minecraft:grass_block" => Some(Item::GrassBlock),
            "minecraft:waxed_oxidized_cut_copper_stairs" => {
                Some(Item::WaxedOxidizedCutCopperStairs)
            }
            "minecraft:gray_stained_glass" => Some(Item::GrayStainedGlass),
            "minecraft:black_wool" => Some(Item::BlackWool),
            "minecraft:blaze_spawn_egg" => Some(Item::BlazeSpawnEgg),
            "minecraft:birch_sign" => Some(Item::BirchSign),
            "minecraft:infested_cracked_stone_bricks" => Some(Item::InfestedCrackedStoneBricks),
            "minecraft:music_disc_strad" => Some(Item::MusicDiscStrad),
            "minecraft:brown_mushroom_block" => Some(Item::BrownMushroomBlock),
            "minecraft:brown_stained_glass" => Some(Item::BrownStainedGlass),
            "minecraft:beetroot_seeds" => Some(Item::BeetrootSeeds),
            "minecraft:stripped_oak_log" => Some(Item::StrippedOakLog),
            "minecraft:purpur_stairs" => Some(Item::PurpurStairs),
            "minecraft:glowstone_dust" => Some(Item::GlowstoneDust),
            "minecraft:smooth_quartz_stairs" => Some(Item::SmoothQuartzStairs),
            "minecraft:stone" => Some(Item::Stone),
            "minecraft:sandstone_wall" => Some(Item::SandstoneWall),
            "minecraft:turtle_spawn_egg" => Some(Item::TurtleSpawnEgg),
            "minecraft:dark_oak_log" => Some(Item::DarkOakLog),
            "minecraft:clay_ball" => Some(Item::ClayBall),
            "minecraft:crimson_slab" => Some(Item::CrimsonSlab),
            "minecraft:warped_door" => Some(Item::WarpedDoor),
            "minecraft:stone_shovel" => Some(Item::StoneShovel),
            "minecraft:shulker_box" => Some(Item::ShulkerBox),
            "minecraft:egg" => Some(Item::Egg),
            "minecraft:moss_block" => Some(Item::MossBlock),
            "minecraft:white_glazed_terracotta" => Some(Item::WhiteGlazedTerracotta),
            "minecraft:end_stone_brick_stairs" => Some(Item::EndStoneBrickStairs),
            "minecraft:poisonous_potato" => Some(Item::PoisonousPotato),
            "minecraft:red_tulip" => Some(Item::RedTulip),
            "minecraft:polished_basalt" => Some(Item::PolishedBasalt),
            "minecraft:blackstone_stairs" => Some(Item::BlackstoneStairs),
            "minecraft:snow_block" => Some(Item::SnowBlock),
            "minecraft:ice" => Some(Item::Ice),
            "minecraft:weathered_cut_copper_stairs" => Some(Item::WeatheredCutCopperStairs),
            "minecraft:diamond_horse_armor" => Some(Item::DiamondHorseArmor),
            "minecraft:birch_log" => Some(Item::BirchLog),
            "minecraft:infested_mossy_stone_bricks" => Some(Item::InfestedMossyStoneBricks),
            "minecraft:orange_terracotta" => Some(Item::OrangeTerracotta),
            "minecraft:pink_terracotta" => Some(Item::PinkTerracotta),
            "minecraft:composter" => Some(Item::Composter),
            "minecraft:iron_leggings" => Some(Item::IronLeggings),
            "minecraft:magenta_wool" => Some(Item::MagentaWool),
            "minecraft:golden_helmet" => Some(Item::GoldenHelmet),
            "minecraft:lime_bed" => Some(Item::LimeBed),
            "minecraft:chain_command_block" => Some(Item::ChainCommandBlock),
            "minecraft:podzol" => Some(Item::Podzol),
            "minecraft:hopper" => Some(Item::Hopper),
            "minecraft:copper_ingot" => Some(Item::CopperIngot),
            "minecraft:slime_block" => Some(Item::SlimeBlock),
            "minecraft:trader_llama_spawn_egg" => Some(Item::TraderLlamaSpawnEgg),
            "minecraft:light_blue_concrete" => Some(Item::LightBlueConcrete),
            "minecraft:glow_squid_spawn_egg" => Some(Item::GlowSquidSpawnEgg),
            "minecraft:polished_diorite" => Some(Item::PolishedDiorite),
            "minecraft:melon" => Some(Item::Melon),
            "minecraft:white_bed" => Some(Item::WhiteBed),
            "minecraft:petrified_oak_slab" => Some(Item::PetrifiedOakSlab),
            "minecraft:crimson_sign" => Some(Item::CrimsonSign),
            "minecraft:prismarine_slab" => Some(Item::PrismarineSlab),
            "minecraft:light_gray_banner" => Some(Item::LightGrayBanner),
            "minecraft:crimson_fungus" => Some(Item::CrimsonFungus),
            "minecraft:rabbit_spawn_egg" => Some(Item::RabbitSpawnEgg),
            "minecraft:red_banner" => Some(Item::RedBanner),
            "minecraft:popped_chorus_fruit" => Some(Item::PoppedChorusFruit),
            "minecraft:lime_carpet" => Some(Item::LimeCarpet),
            "minecraft:iron_nugget" => Some(Item::IronNugget),
            "minecraft:stripped_crimson_hyphae" => Some(Item::StrippedCrimsonHyphae),
            "minecraft:dandelion" => Some(Item::Dandelion),
            "minecraft:creeper_banner_pattern" => Some(Item::CreeperBannerPattern),
            "minecraft:iron_boots" => Some(Item::IronBoots),
            "minecraft:oxidized_cut_copper" => Some(Item::OxidizedCutCopper),
            "minecraft:golden_sword" => Some(Item::GoldenSword),
            "minecraft:diamond_pickaxe" => Some(Item::DiamondPickaxe),
            "minecraft:target" => Some(Item::Target),
            "minecraft:terracotta" => Some(Item::Terracotta),
            "minecraft:green_banner" => Some(Item::GreenBanner),
            "minecraft:dolphin_spawn_egg" => Some(Item::DolphinSpawnEgg),
            "minecraft:red_sand" => Some(Item::RedSand),
            "minecraft:brick_stairs" => Some(Item::BrickStairs),
            "minecraft:firework_rocket" => Some(Item::FireworkRocket),
            "minecraft:red_shulker_box" => Some(Item::RedShulkerBox),
            "minecraft:pointed_dripstone" => Some(Item::PointedDripstone),
            "minecraft:light_gray_carpet" => Some(Item::LightGrayCarpet),
            "minecraft:jungle_fence" => Some(Item::JungleFence),
            "minecraft:coal_ore" => Some(Item::CoalOre),
            "minecraft:jungle_door" => Some(Item::JungleDoor),
            "minecraft:lightning_rod" => Some(Item::LightningRod),
            "minecraft:golden_carrot" => Some(Item::GoldenCarrot),
            "minecraft:grindstone" => Some(Item::Grindstone),
            "minecraft:red_sandstone_slab" => Some(Item::RedSandstoneSlab),
            "minecraft:dead_brain_coral_fan" => Some(Item::DeadBrainCoralFan),
            "minecraft:orange_concrete_powder" => Some(Item::OrangeConcretePowder),
            "minecraft:warped_button" => Some(Item::WarpedButton),
            "minecraft:dead_tube_coral_fan" => Some(Item::DeadTubeCoralFan),
            "minecraft:polished_blackstone_pressure_plate" => {
                Some(Item::PolishedBlackstonePressurePlate)
            }
            "minecraft:green_terracotta" => Some(Item::GreenTerracotta),
            "minecraft:tube_coral" => Some(Item::TubeCoral),
            "minecraft:jungle_log" => Some(Item::JungleLog),
            "minecraft:warped_nylium" => Some(Item::WarpedNylium),
            "minecraft:guardian_spawn_egg" => Some(Item::GuardianSpawnEgg),
            "minecraft:brick_slab" => Some(Item::BrickSlab),
            "minecraft:stripped_oak_wood" => Some(Item::StrippedOakWood),
            "minecraft:sculk_sensor" => Some(Item::SculkSensor),
            "minecraft:birch_stairs" => Some(Item::BirchStairs),
            "minecraft:mooshroom_spawn_egg" => Some(Item::MooshroomSpawnEgg),
            "minecraft:flower_banner_pattern" => Some(Item::FlowerBannerPattern),
            "minecraft:crying_obsidian" => Some(Item::CryingObsidian),
            "minecraft:quartz_block" => Some(Item::QuartzBlock),
            "minecraft:cyan_terracotta" => Some(Item::CyanTerracotta),
            "minecraft:observer" => Some(Item::Observer),
            "minecraft:infested_chiseled_stone_bricks" => Some(Item::InfestedChiseledStoneBricks),
            "minecraft:wooden_axe" => Some(Item::WoodenAxe),
            "minecraft:yellow_candle" => Some(Item::YellowCandle),
            "minecraft:writable_book" => Some(Item::WritableBook),
            "minecraft:creeper_spawn_egg" => Some(Item::CreeperSpawnEgg),
            "minecraft:nether_bricks" => Some(Item::NetherBricks),
            "minecraft:black_terracotta" => Some(Item::BlackTerracotta),
            "minecraft:dead_bubble_coral" => Some(Item::DeadBubbleCoral),
            "minecraft:crimson_pressure_plate" => Some(Item::CrimsonPressurePlate),
            "minecraft:gold_ore" => Some(Item::GoldOre),
            "minecraft:dead_horn_coral_fan" => Some(Item::DeadHornCoralFan),
            "minecraft:furnace_minecart" => Some(Item::FurnaceMinecart),
            "minecraft:shears" => Some(Item::Shears),
            "minecraft:smooth_stone_slab" => Some(Item::SmoothStoneSlab),
            "minecraft:wither_rose" => Some(Item::WitherRose),
            "minecraft:deepslate_brick_wall" => Some(Item::DeepslateBrickWall),
            "minecraft:purple_glazed_terracotta" => Some(Item::PurpleGlazedTerracotta),
            "minecraft:jack_o_lantern" => Some(Item::JackOLantern),
            "minecraft:horn_coral_block" => Some(Item::HornCoralBlock),
            "minecraft:spruce_fence" => Some(Item::SpruceFence),
            "minecraft:light_gray_glazed_terracotta" => Some(Item::LightGrayGlazedTerracotta),
            "minecraft:glow_berries" => Some(Item::GlowBerries),
            "minecraft:cod_bucket" => Some(Item::CodBucket),
            "minecraft:sunflower" => Some(Item::Sunflower),
            "minecraft:orange_carpet" => Some(Item::OrangeCarpet),
            "minecraft:blue_carpet" => Some(Item::BlueCarpet),
            "minecraft:sea_pickle" => Some(Item::SeaPickle),
            "minecraft:pink_shulker_box" => Some(Item::PinkShulkerBox),
            "minecraft:red_glazed_terracotta" => Some(Item::RedGlazedTerracotta),
            "minecraft:brain_coral" => Some(Item::BrainCoral),
            "minecraft:dragon_head" => Some(Item::DragonHead),
            "minecraft:magenta_candle" => Some(Item::MagentaCandle),
            "minecraft:deepslate_gold_ore" => Some(Item::DeepslateGoldOre),
            "minecraft:stripped_spruce_log" => Some(Item::StrippedSpruceLog),
            "minecraft:donkey_spawn_egg" => Some(Item::DonkeySpawnEgg),
            "minecraft:lime_concrete_powder" => Some(Item::LimeConcretePowder),
            "minecraft:prismarine_bricks" => Some(Item::PrismarineBricks),
            "minecraft:brown_banner" => Some(Item::BrownBanner),
            "minecraft:white_stained_glass" => Some(Item::WhiteStainedGlass),
            "minecraft:blue_stained_glass" => Some(Item::BlueStainedGlass),
            "minecraft:diamond_hoe" => Some(Item::DiamondHoe),
            "minecraft:spruce_sign" => Some(Item::SpruceSign),
            "minecraft:lodestone" => Some(Item::Lodestone),
            "minecraft:sugar" => Some(Item::Sugar),
            "minecraft:smooth_red_sandstone_stairs" => Some(Item::SmoothRedSandstoneStairs),
            "minecraft:light_blue_shulker_box" => Some(Item::LightBlueShulkerBox),
            "minecraft:cyan_dye" => Some(Item::CyanDye),
            "minecraft:weathered_cut_copper" => Some(Item::WeatheredCutCopper),
            "minecraft:spruce_boat" => Some(Item::SpruceBoat),
            "minecraft:sandstone_stairs" => Some(Item::SandstoneStairs),
            "minecraft:magma_block" => Some(Item::MagmaBlock),
            "minecraft:light_gray_shulker_box" => Some(Item::LightGrayShulkerBox),
            "minecraft:prismarine_wall" => Some(Item::PrismarineWall),
            "minecraft:tube_coral_fan" => Some(Item::TubeCoralFan),
            "minecraft:coal" => Some(Item::Coal),
            "minecraft:item_frame" => Some(Item::ItemFrame),
            "minecraft:yellow_banner" => Some(Item::YellowBanner),
            "minecraft:small_amethyst_bud" => Some(Item::SmallAmethystBud),
            "minecraft:brown_shulker_box" => Some(Item::BrownShulkerBox),
            "minecraft:iron_ore" => Some(Item::IronOre),
            "minecraft:soul_lantern" => Some(Item::SoulLantern),
            "minecraft:golden_pickaxe" => Some(Item::GoldenPickaxe),
            "minecraft:quartz_slab" => Some(Item::QuartzSlab),
            "minecraft:lime_terracotta" => Some(Item::LimeTerracotta),
            "minecraft:brain_coral_fan" => Some(Item::BrainCoralFan),
            "minecraft:stripped_jungle_log" => Some(Item::StrippedJungleLog),
            "minecraft:turtle_helmet" => Some(Item::TurtleHelmet),
            "minecraft:purpur_slab" => Some(Item::PurpurSlab),
            "minecraft:magenta_bed" => Some(Item::MagentaBed),
            "minecraft:warped_hyphae" => Some(Item::WarpedHyphae),
            "minecraft:honeycomb_block" => Some(Item::HoneycombBlock),
            "minecraft:purple_stained_glass_pane" => Some(Item::PurpleStainedGlassPane),
            "minecraft:diamond_ore" => Some(Item::DiamondOre),
            "minecraft:calcite" => Some(Item::Calcite),
            "minecraft:oak_boat" => Some(Item::OakBoat),
            "minecraft:hoglin_spawn_egg" => Some(Item::HoglinSpawnEgg),
            "minecraft:respawn_anchor" => Some(Item::RespawnAnchor),
            "minecraft:magenta_banner" => Some(Item::MagentaBanner),
            "minecraft:chiseled_polished_blackstone" => Some(Item::ChiseledPolishedBlackstone),
            "minecraft:budding_amethyst" => Some(Item::BuddingAmethyst),
            "minecraft:glass" => Some(Item::Glass),
            "minecraft:iron_ingot" => Some(Item::IronIngot),
            "minecraft:deepslate_diamond_ore" => Some(Item::DeepslateDiamondOre),
            "minecraft:acacia_door" => Some(Item::AcaciaDoor),
            "minecraft:gray_bed" => Some(Item::GrayBed),
            "minecraft:large_fern" => Some(Item::LargeFern),
            "minecraft:red_stained_glass" => Some(Item::RedStainedGlass),
            "minecraft:brick" => Some(Item::Brick),
            "minecraft:oak_leaves" => Some(Item::OakLeaves),
            "minecraft:elder_guardian_spawn_egg" => Some(Item::ElderGuardianSpawnEgg),
            "minecraft:golden_leggings" => Some(Item::GoldenLeggings),
            "minecraft:pink_stained_glass" => Some(Item::PinkStainedGlass),
            "minecraft:red_sandstone_wall" => Some(Item::RedSandstoneWall),
            "minecraft:glow_item_frame" => Some(Item::GlowItemFrame),
            "minecraft:bell" => Some(Item::Bell),
            "minecraft:cobbled_deepslate_stairs" => Some(Item::CobbledDeepslateStairs),
            "minecraft:wolf_spawn_egg" => Some(Item::WolfSpawnEgg),
            "minecraft:knowledge_book" => Some(Item::KnowledgeBook),
            "minecraft:cooked_mutton" => Some(Item::CookedMutton),
            "minecraft:smooth_stone" => Some(Item::SmoothStone),
            "minecraft:lime_glazed_terracotta" => Some(Item::LimeGlazedTerracotta),
            "minecraft:netherite_hoe" => Some(Item::NetheriteHoe),
            "minecraft:oak_planks" => Some(Item::OakPlanks),
            "minecraft:magenta_dye" => Some(Item::MagentaDye),
            "minecraft:cracked_stone_bricks" => Some(Item::CrackedStoneBricks),
            "minecraft:cooked_beef" => Some(Item::CookedBeef),
            "minecraft:coarse_dirt" => Some(Item::CoarseDirt),
            "minecraft:ender_chest" => Some(Item::EnderChest),
            "minecraft:zombie_spawn_egg" => Some(Item::ZombieSpawnEgg),
            "minecraft:warped_fence_gate" => Some(Item::WarpedFenceGate),
            "minecraft:bucket" => Some(Item::Bucket),
            "minecraft:blue_dye" => Some(Item::BlueDye),
            "minecraft:cactus" => Some(Item::Cactus),
            "minecraft:lead" => Some(Item::Lead),
            "minecraft:mushroom_stew" => Some(Item::MushroomStew),
            "minecraft:light_blue_bed" => Some(Item::LightBlueBed),
            "minecraft:bee_spawn_egg" => Some(Item::BeeSpawnEgg),
            "minecraft:oxidized_cut_copper_stairs" => Some(Item::OxidizedCutCopperStairs),
            "minecraft:kelp" => Some(Item::Kelp),
            "minecraft:green_concrete_powder" => Some(Item::GreenConcretePowder),
            "minecraft:redstone_torch" => Some(Item::RedstoneTorch),
            "minecraft:snowball" => Some(Item::Snowball),
            "minecraft:spruce_sapling" => Some(Item::SpruceSapling),
            "minecraft:green_candle" => Some(Item::GreenCandle),
            "minecraft:oxidized_copper" => Some(Item::OxidizedCopper),
            "minecraft:stripped_acacia_log" => Some(Item::StrippedAcaciaLog),
            "minecraft:zombie_horse_spawn_egg" => Some(Item::ZombieHorseSpawnEgg),
            "minecraft:acacia_slab" => Some(Item::AcaciaSlab),
            "minecraft:spruce_door" => Some(Item::SpruceDoor),
            "minecraft:dark_oak_sign" => Some(Item::DarkOakSign),
            "minecraft:green_concrete" => Some(Item::GreenConcrete),
            "minecraft:salmon" => Some(Item::Salmon),
            "minecraft:dead_fire_coral_fan" => Some(Item::DeadFireCoralFan),
            "minecraft:blue_stained_glass_pane" => Some(Item::BlueStainedGlassPane),
            "minecraft:oak_fence_gate" => Some(Item::OakFenceGate),
            "minecraft:minecart" => Some(Item::Minecart),
            "minecraft:fishing_rod" => Some(Item::FishingRod),
            "minecraft:vindicator_spawn_egg" => Some(Item::VindicatorSpawnEgg),
            "minecraft:cauldron" => Some(Item::Cauldron),
            "minecraft:spruce_button" => Some(Item::SpruceButton),
            "minecraft:dispenser" => Some(Item::Dispenser),
            "minecraft:stripped_spruce_wood" => Some(Item::StrippedSpruceWood),
            "minecraft:yellow_bed" => Some(Item::YellowBed),
            "minecraft:nether_brick_wall" => Some(Item::NetherBrickWall),
            "minecraft:jungle_sapling" => Some(Item::JungleSapling),
            "minecraft:damaged_anvil" => Some(Item::DamagedAnvil),
            "minecraft:quartz" => Some(Item::Quartz),
            "minecraft:brown_concrete_powder" => Some(Item::BrownConcretePowder),
            "minecraft:tall_grass" => Some(Item::TallGrass),
            "minecraft:polished_andesite_stairs" => Some(Item::PolishedAndesiteStairs),
            "minecraft:raw_gold" => Some(Item::RawGold),
            "minecraft:cobweb" => Some(Item::Cobweb),
            "minecraft:birch_leaves" => Some(Item::BirchLeaves),
            "minecraft:nether_wart_block" => Some(Item::NetherWartBlock),
            "minecraft:green_stained_glass" => Some(Item::GreenStainedGlass),
            "minecraft:bow" => Some(Item::Bow),
            "minecraft:crimson_planks" => Some(Item::CrimsonPlanks),
            "minecraft:deepslate_copper_ore" => Some(Item::DeepslateCopperOre),
            "minecraft:flower_pot" => Some(Item::FlowerPot),
            "minecraft:white_carpet" => Some(Item::WhiteCarpet),
            "minecraft:allium" => Some(Item::Allium),
            "minecraft:stripped_dark_oak_wood" => Some(Item::StrippedDarkOakWood),
            "minecraft:basalt" => Some(Item::Basalt),
            "minecraft:polished_andesite" => Some(Item::PolishedAndesite),
            "minecraft:saddle" => Some(Item::Saddle),
            "minecraft:bowl" => Some(Item::Bowl),
            "minecraft:ender_eye" => Some(Item::EnderEye),
            "minecraft:daylight_detector" => Some(Item::DaylightDetector),
            "minecraft:grass" => Some(Item::Grass),
            "minecraft:mossy_cobblestone_wall" => Some(Item::MossyCobblestoneWall),
            "minecraft:yellow_dye" => Some(Item::YellowDye),
            "minecraft:enderman_spawn_egg" => Some(Item::EndermanSpawnEgg),
            "minecraft:parrot_spawn_egg" => Some(Item::ParrotSpawnEgg),
            "minecraft:jungle_pressure_plate" => Some(Item::JunglePressurePlate),
            "minecraft:waxed_exposed_cut_copper_slab" => Some(Item::WaxedExposedCutCopperSlab),
            "minecraft:vine" => Some(Item::Vine),
            "minecraft:purpur_block" => Some(Item::PurpurBlock),
            "minecraft:rabbit_foot" => Some(Item::RabbitFoot),
            "minecraft:diamond_chestplate" => Some(Item::DiamondChestplate),
            "minecraft:bricks" => Some(Item::Bricks),
            "minecraft:music_disc_far" => Some(Item::MusicDiscFar),
            "minecraft:oak_stairs" => Some(Item::OakStairs),
            "minecraft:honey_block" => Some(Item::HoneyBlock),
            "minecraft:carrot_on_a_stick" => Some(Item::CarrotOnAStick),
            "minecraft:quartz_stairs" => Some(Item::QuartzStairs),
            "minecraft:acacia_fence_gate" => Some(Item::AcaciaFenceGate),
            "minecraft:dead_bush" => Some(Item::DeadBush),
            "minecraft:cooked_chicken" => Some(Item::CookedChicken),
            "minecraft:purple_shulker_box" => Some(Item::PurpleShulkerBox),
            "minecraft:packed_ice" => Some(Item::PackedIce),
            "minecraft:chiseled_deepslate" => Some(Item::ChiseledDeepslate),
            "minecraft:phantom_membrane" => Some(Item::PhantomMembrane),
            "minecraft:skull_banner_pattern" => Some(Item::SkullBannerPattern),
            "minecraft:waxed_weathered_cut_copper_stairs" => {
                Some(Item::WaxedWeatheredCutCopperStairs)
            }
            "minecraft:exposed_cut_copper_stairs" => Some(Item::ExposedCutCopperStairs),
            "minecraft:cyan_carpet" => Some(Item::CyanCarpet),
            "minecraft:smooth_basalt" => Some(Item::SmoothBasalt),
            "minecraft:orange_bed" => Some(Item::OrangeBed),
            "minecraft:panda_spawn_egg" => Some(Item::PandaSpawnEgg),
            "minecraft:acacia_boat" => Some(Item::AcaciaBoat),
            "minecraft:acacia_fence" => Some(Item::AcaciaFence),
            "minecraft:wither_skeleton_spawn_egg" => Some(Item::WitherSkeletonSpawnEgg),
            "minecraft:torch" => Some(Item::Torch),
            "minecraft:rabbit_stew" => Some(Item::RabbitStew),
            "minecraft:netherite_shovel" => Some(Item::NetheriteShovel),
            "minecraft:campfire" => Some(Item::Campfire),
            "minecraft:birch_sapling" => Some(Item::BirchSapling),
            "minecraft:white_banner" => Some(Item::WhiteBanner),
            "minecraft:waxed_oxidized_copper" => Some(Item::WaxedOxidizedCopper),
            "minecraft:azalea_leaves" => Some(Item::AzaleaLeaves),
            "minecraft:granite_stairs" => Some(Item::GraniteStairs),
            "minecraft:gray_shulker_box" => Some(Item::GrayShulkerBox),
            "minecraft:oxeye_daisy" => Some(Item::OxeyeDaisy),
            "minecraft:deepslate_tile_stairs" => Some(Item::DeepslateTileStairs),
            "minecraft:gray_terracotta" => Some(Item::GrayTerracotta),
            "minecraft:pink_stained_glass_pane" => Some(Item::PinkStainedGlassPane),
            "minecraft:bookshelf" => Some(Item::Bookshelf),
            "minecraft:polished_granite_slab" => Some(Item::PolishedGraniteSlab),
            "minecraft:deepslate_tile_slab" => Some(Item::DeepslateTileSlab),
            "minecraft:pufferfish" => Some(Item::Pufferfish),
            "minecraft:diamond_axe" => Some(Item::DiamondAxe),
            "minecraft:blue_terracotta" => Some(Item::BlueTerracotta),
            "minecraft:red_concrete" => Some(Item::RedConcrete),
            "minecraft:cooked_cod" => Some(Item::CookedCod),
            "minecraft:pillager_spawn_egg" => Some(Item::PillagerSpawnEgg),
            "minecraft:gold_nugget" => Some(Item::GoldNugget),
            "minecraft:warped_fence" => Some(Item::WarpedFence),
            "minecraft:spider_eye" => Some(Item::SpiderEye),
            "minecraft:prismarine_crystals" => Some(Item::PrismarineCrystals),
            "minecraft:polished_granite" => Some(Item::PolishedGranite),
            "minecraft:lime_stained_glass_pane" => Some(Item::LimeStainedGlassPane),
            "minecraft:birch_fence_gate" => Some(Item::BirchFenceGate),
            "minecraft:note_block" => Some(Item::NoteBlock),
            "minecraft:shulker_shell" => Some(Item::ShulkerShell),
            "minecraft:blaze_powder" => Some(Item::BlazePowder),
            "minecraft:red_bed" => Some(Item::RedBed),
            "minecraft:polished_deepslate" => Some(Item::PolishedDeepslate),
            "minecraft:birch_button" => Some(Item::BirchButton),
            "minecraft:music_disc_mall" => Some(Item::MusicDiscMall),
            "minecraft:fox_spawn_egg" => Some(Item::FoxSpawnEgg),
            "minecraft:tipped_arrow" => Some(Item::TippedArrow),
            "minecraft:polished_blackstone_stairs" => Some(Item::PolishedBlackstoneStairs),
            "minecraft:zombified_piglin_spawn_egg" => Some(Item::ZombifiedPiglinSpawnEgg),
            "minecraft:small_dripleaf" => Some(Item::SmallDripleaf),
            "minecraft:pink_concrete" => Some(Item::PinkConcrete),
            "minecraft:lapis_ore" => Some(Item::LapisOre),
            "minecraft:birch_planks" => Some(Item::BirchPlanks),
            "minecraft:melon_slice" => Some(Item::MelonSlice),
            "minecraft:polar_bear_spawn_egg" => Some(Item::PolarBearSpawnEgg),
            "minecraft:command_block_minecart" => Some(Item::CommandBlockMinecart),
            "minecraft:music_disc_pigstep" => Some(Item::MusicDiscPigstep),
            "minecraft:tinted_glass" => Some(Item::TintedGlass),
            "minecraft:warped_fungus_on_a_stick" => Some(Item::WarpedFungusOnAStick),
            "minecraft:deepslate_redstone_ore" => Some(Item::DeepslateRedstoneOre),
            "minecraft:bundle" => Some(Item::Bundle),
            "minecraft:music_disc_ward" => Some(Item::MusicDiscWard),
            "minecraft:amethyst_cluster" => Some(Item::AmethystCluster),
            "minecraft:stone_axe" => Some(Item::StoneAxe),
            "minecraft:stray_spawn_egg" => Some(Item::StraySpawnEgg),
            "minecraft:polished_diorite_slab" => Some(Item::PolishedDioriteSlab),
            "minecraft:mossy_stone_brick_slab" => Some(Item::MossyStoneBrickSlab),
            "minecraft:black_banner" => Some(Item::BlackBanner),
            "minecraft:evoker_spawn_egg" => Some(Item::EvokerSpawnEgg),
            "minecraft:light_gray_candle" => Some(Item::LightGrayCandle),
            "minecraft:pufferfish_spawn_egg" => Some(Item::PufferfishSpawnEgg),
            "minecraft:experience_bottle" => Some(Item::ExperienceBottle),
            "minecraft:black_dye" => Some(Item::BlackDye),
            "minecraft:prismarine_shard" => Some(Item::PrismarineShard),
            "minecraft:turtle_egg" => Some(Item::TurtleEgg),
            "minecraft:spruce_slab" => Some(Item::SpruceSlab),
            "minecraft:chicken" => Some(Item::Chicken),
            "minecraft:acacia_wood" => Some(Item::AcaciaWood),
            "minecraft:fire_charge" => Some(Item::FireCharge),
            "minecraft:comparator" => Some(Item::Comparator),
            "minecraft:powered_rail" => Some(Item::PoweredRail),
            "minecraft:stone_brick_stairs" => Some(Item::StoneBrickStairs),
            "minecraft:bubble_coral_fan" => Some(Item::BubbleCoralFan),
            "minecraft:music_disc_13" => Some(Item::MusicDisc13),
            "minecraft:quartz_bricks" => Some(Item::QuartzBricks),
            "minecraft:honey_bottle" => Some(Item::HoneyBottle),
            "minecraft:yellow_concrete_powder" => Some(Item::YellowConcretePowder),
            "minecraft:deepslate_emerald_ore" => Some(Item::DeepslateEmeraldOre),
            "minecraft:book" => Some(Item::Book),
            "minecraft:tnt" => Some(Item::Tnt),
            "minecraft:scute" => Some(Item::Scute),
            "minecraft:chest_minecart" => Some(Item::ChestMinecart),
            "minecraft:netherite_axe" => Some(Item::NetheriteAxe),
            "minecraft:dark_oak_button" => Some(Item::DarkOakButton),
            "minecraft:diamond_boots" => Some(Item::DiamondBoots),
            "minecraft:totem_of_undying" => Some(Item::TotemOfUndying),
            "minecraft:glass_pane" => Some(Item::GlassPane),
            "minecraft:polished_blackstone_slab" => Some(Item::PolishedBlackstoneSlab),
            "minecraft:cobblestone_wall" => Some(Item::CobblestoneWall),
            "minecraft:magenta_concrete_powder" => Some(Item::MagentaConcretePowder),
            "minecraft:red_sandstone" => Some(Item::RedSandstone),
            "minecraft:orange_concrete" => Some(Item::OrangeConcrete),
            "minecraft:light" => Some(Item::Light),
            "minecraft:stone_slab" => Some(Item::StoneSlab),
            "minecraft:string" => Some(Item::String),
            "minecraft:dragon_breath" => Some(Item::DragonBreath),
            "minecraft:crimson_stem" => Some(Item::CrimsonStem),
            _ => None,
        }
    }
}
impl Item {
    #[doc = "Returns the `stack_size` property of this `Item`."]
    #[inline]
    pub fn stack_size(&self) -> u32 {
        match self {
            Item::WhiteStainedGlassPane => 64,
            Item::LightGrayShulkerBox => 1,
            Item::NetherWart => 64,
            Item::Chest => 64,
            Item::Cactus => 64,
            Item::MelonSeeds => 64,
            Item::SpectralArrow => 64,
            Item::Kelp => 64,
            Item::OrangeConcretePowder => 64,
            Item::PurpleConcrete => 64,
            Item::DiamondHelmet => 1,
            Item::Chicken => 64,
            Item::PinkBanner => 16,
            Item::DebugStick => 1,
            Item::SpruceButton => 64,
            Item::PinkTulip => 64,
            Item::EndStone => 64,
            Item::LightBlueConcrete => 64,
            Item::Clock => 64,
            Item::NetherBrickFence => 64,
            Item::DeadFireCoralFan => 64,
            Item::ChainmailLeggings => 1,
            Item::AcaciaSapling => 64,
            Item::MossyCobblestoneSlab => 64,
            Item::ExposedCutCopperSlab => 64,
            Item::OrangeWool => 64,
            Item::NetherSprouts => 64,
            Item::MossyStoneBrickStairs => 64,
            Item::IronHoe => 1,
            Item::TurtleSpawnEgg => 64,
            Item::IronSword => 1,
            Item::OrangeCandle => 64,
            Item::Jigsaw => 64,
            Item::MagentaBed => 1,
            Item::StrippedWarpedStem => 64,
            Item::GrayWool => 64,
            Item::WhiteShulkerBox => 1,
            Item::CreeperSpawnEgg => 64,
            Item::Cake => 1,
            Item::PolishedGranite => 64,
            Item::PurpleCarpet => 64,
            Item::Minecart => 1,
            Item::DeepslateCoalOre => 64,
            Item::OakStairs => 64,
            Item::ZombifiedPiglinSpawnEgg => 64,
            Item::DarkOakBoat => 1,
            Item::RepeatingCommandBlock => 64,
            Item::Sunflower => 64,
            Item::MagmaBlock => 64,
            Item::PowderSnowBucket => 1,
            Item::FishingRod => 1,
            Item::GlowstoneDust => 64,
            Item::ZoglinSpawnEgg => 64,
            Item::Lantern => 64,
            Item::DeepslateTileWall => 64,
            Item::PhantomMembrane => 64,
            Item::QuartzPillar => 64,
            Item::GrayTerracotta => 64,
            Item::DiamondBlock => 64,
            Item::CarrotOnAStick => 1,
            Item::MagentaConcrete => 64,
            Item::Potato => 64,
            Item::Bow => 1,
            Item::StoneShovel => 1,
            Item::PolishedBlackstoneBrickStairs => 64,
            Item::CutSandstoneSlab => 64,
            Item::BrownBed => 1,
            Item::GreenConcretePowder => 64,
            Item::GreenTerracotta => 64,
            Item::LightGrayCandle => 64,
            Item::LingeringPotion => 1,
            Item::SpruceSlab => 64,
            Item::LeatherHorseArmor => 1,
            Item::RedSand => 64,
            Item::CrimsonFenceGate => 64,
            Item::CyanDye => 64,
            Item::BlazeRod => 64,
            Item::WeatheredCutCopper => 64,
            Item::Seagrass => 64,
            Item::SmoothQuartzStairs => 64,
            Item::RawGold => 64,
            Item::CookedPorkchop => 64,
            Item::ChorusFlower => 64,
            Item::SkeletonSpawnEgg => 64,
            Item::KnowledgeBook => 1,
            Item::StoneBricks => 64,
            Item::RawIronBlock => 64,
            Item::GoldenShovel => 1,
            Item::RedSandstoneSlab => 64,
            Item::GreenCarpet => 64,
            Item::DeepslateCopperOre => 64,
            Item::JungleBoat => 1,
            Item::TropicalFish => 64,
            Item::EnderPearl => 16,
            Item::SmoothSandstoneStairs => 64,
            Item::WaxedExposedCopper => 64,
            Item::DeadBubbleCoralFan => 64,
            Item::LightBlueCandle => 64,
            Item::Beacon => 64,
            Item::PinkTerracotta => 64,
            Item::DarkOakLeaves => 64,
            Item::LightBlueShulkerBox => 1,
            Item::RawGoldBlock => 64,
            Item::PolishedBlackstoneStairs => 64,
            Item::LightBlueStainedGlassPane => 64,
            Item::DeadFireCoral => 64,
            Item::DiamondChestplate => 1,
            Item::AcaciaBoat => 1,
            Item::Emerald => 64,
            Item::RespawnAnchor => 64,
            Item::IronDoor => 64,
            Item::ZombieSpawnEgg => 64,
            Item::MusicDiscBlocks => 1,
            Item::Andesite => 64,
            Item::Campfire => 64,
            Item::GrayStainedGlass => 64,
            Item::Saddle => 1,
            Item::VindicatorSpawnEgg => 64,
            Item::LapisBlock => 64,
            Item::SlimeBlock => 64,
            Item::DarkOakSign => 16,
            Item::PolishedDioriteStairs => 64,
            Item::BirchSign => 16,
            Item::Stonecutter => 64,
            Item::VexSpawnEgg => 64,
            Item::CutCopper => 64,
            Item::InfestedMossyStoneBricks => 64,
            Item::LightBlueStainedGlass => 64,
            Item::SpruceDoor => 64,
            Item::WarpedFenceGate => 64,
            Item::LeatherHelmet => 1,
            Item::ElderGuardianSpawnEgg => 64,
            Item::PolishedGraniteSlab => 64,
            Item::MossCarpet => 64,
            Item::CrackedPolishedBlackstoneBricks => 64,
            Item::LimeWool => 64,
            Item::RedMushroomBlock => 64,
            Item::EndermanSpawnEgg => 64,
            Item::Candle => 64,
            Item::OxeyeDaisy => 64,
            Item::JungleLeaves => 64,
            Item::Bedrock => 64,
            Item::MagentaDye => 64,
            Item::BuddingAmethyst => 64,
            Item::OxidizedCopper => 64,
            Item::PinkCarpet => 64,
            Item::CreeperHead => 64,
            Item::Cauldron => 64,
            Item::Sandstone => 64,
            Item::SmoothBasalt => 64,
            Item::Calcite => 64,
            Item::BlackWool => 64,
            Item::IronPickaxe => 1,
            Item::PoweredRail => 64,
            Item::Egg => 16,
            Item::BirchTrapdoor => 64,
            Item::StrippedAcaciaWood => 64,
            Item::LightBlueGlazedTerracotta => 64,
            Item::LightBlueCarpet => 64,
            Item::Coal => 64,
            Item::Mycelium => 64,
            Item::MusicDisc11 => 1,
            Item::HopperMinecart => 1,
            Item::DolphinSpawnEgg => 64,
            Item::SoulSoil => 64,
            Item::NetherBrickSlab => 64,
            Item::PolishedDiorite => 64,
            Item::SoulTorch => 64,
            Item::RedGlazedTerracotta => 64,
            Item::Glowstone => 64,
            Item::RawIron => 64,
            Item::JungleSlab => 64,
            Item::IronNugget => 64,
            Item::CrimsonFence => 64,
            Item::CyanBed => 1,
            Item::BirchLog => 64,
            Item::LightGrayGlazedTerracotta => 64,
            Item::CobbledDeepslateWall => 64,
            Item::PinkStainedGlassPane => 64,
            Item::SmallAmethystBud => 64,
            Item::AmethystCluster => 64,
            Item::DarkOakButton => 64,
            Item::SkeletonSkull => 64,
            Item::RedSandstone => 64,
            Item::SplashPotion => 1,
            Item::PurpurPillar => 64,
            Item::ChiseledDeepslate => 64,
            Item::Porkchop => 64,
            Item::GlowInkSac => 64,
            Item::Clay => 64,
            Item::Carrot => 64,
            Item::BrownBanner => 16,
            Item::Loom => 64,
            Item::LapisOre => 64,
            Item::SprucePlanks => 64,
            Item::LightGrayConcrete => 64,
            Item::Salmon => 64,
            Item::MooshroomSpawnEgg => 64,
            Item::WhiteDye => 64,
            Item::ShulkerShell => 64,
            Item::BrownCandle => 64,
            Item::PolishedBlackstone => 64,
            Item::GlobeBannerPattern => 1,
            Item::PolishedGraniteStairs => 64,
            Item::YellowGlazedTerracotta => 64,
            Item::DiamondOre => 64,
            Item::DarkPrismarineStairs => 64,
            Item::DragonEgg => 64,
            Item::AcaciaTrapdoor => 64,
            Item::NetheritePickaxe => 1,
            Item::JungleStairs => 64,
            Item::Honeycomb => 64,
            Item::ChippedAnvil => 64,
            Item::WaxedOxidizedCutCopper => 64,
            Item::FermentedSpiderEye => 64,
            Item::BlazeSpawnEgg => 64,
            Item::OrangeBed => 1,
            Item::Shroomlight => 64,
            Item::OakSapling => 64,
            Item::CutCopperSlab => 64,
            Item::HangingRoots => 64,
            Item::DeadBrainCoralBlock => 64,
            Item::DeadBubbleCoralBlock => 64,
            Item::OakButton => 64,
            Item::OakTrapdoor => 64,
            Item::RedSandstoneWall => 64,
            Item::DonkeySpawnEgg => 64,
            Item::OrangeBanner => 16,
            Item::PurpurBlock => 64,
            Item::RootedDirt => 64,
            Item::DeadTubeCoral => 64,
            Item::RedstoneBlock => 64,
            Item::OrangeShulkerBox => 1,
            Item::BlueWool => 64,
            Item::String => 64,
            Item::JunglePlanks => 64,
            Item::FloweringAzalea => 64,
            Item::OrangeTerracotta => 64,
            Item::MusicDiscWait => 1,
            Item::CrimsonPlanks => 64,
            Item::BrownStainedGlass => 64,
            Item::Cod => 64,
            Item::Repeater => 64,
            Item::CowSpawnEgg => 64,
            Item::GreenShulkerBox => 1,
            Item::WaxedWeatheredCutCopperSlab => 64,
            Item::BlazePowder => 64,
            Item::CrackedDeepslateTiles => 64,
            Item::PrismarineWall => 64,
            Item::BlackGlazedTerracotta => 64,
            Item::CyanCandle => 64,
            Item::HuskSpawnEgg => 64,
            Item::YellowStainedGlass => 64,
            Item::Beetroot => 64,
            Item::Apple => 64,
            Item::CommandBlockMinecart => 1,
            Item::SeaPickle => 64,
            Item::Shield => 1,
            Item::CrimsonFungus => 64,
            Item::InfestedStone => 64,
            Item::LimeCarpet => 64,
            Item::CrimsonPressurePlate => 64,
            Item::BrownMushroom => 64,
            Item::Grindstone => 64,
            Item::Conduit => 64,
            Item::BrownConcrete => 64,
            Item::StrippedAcaciaLog => 64,
            Item::LilyPad => 64,
            Item::MagentaStainedGlassPane => 64,
            Item::PolishedBasalt => 64,
            Item::MojangBannerPattern => 1,
            Item::HornCoralBlock => 64,
            Item::Netherrack => 64,
            Item::WanderingTraderSpawnEgg => 64,
            Item::LightBlueTerracotta => 64,
            Item::Barrel => 64,
            Item::DeepslateBricks => 64,
            Item::CyanStainedGlass => 64,
            Item::YellowStainedGlassPane => 64,
            Item::StructureVoid => 64,
            Item::SkeletonHorseSpawnEgg => 64,
            Item::EndStoneBricks => 64,
            Item::CobblestoneStairs => 64,
            Item::DeepslateEmeraldOre => 64,
            Item::BlueTerracotta => 64,
            Item::RedConcretePowder => 64,
            Item::SmoothQuartz => 64,
            Item::NetherBrickStairs => 64,
            Item::Dandelion => 64,
            Item::Flint => 64,
            Item::InfestedCrackedStoneBricks => 64,
            Item::DeadHornCoralFan => 64,
            Item::Pufferfish => 64,
            Item::MushroomStem => 64,
            Item::BrainCoralFan => 64,
            Item::SpiderEye => 64,
            Item::WarpedDoor => 64,
            Item::DeepslateBrickWall => 64,
            Item::DarkOakStairs => 64,
            Item::ChorusFruit => 64,
            Item::BlueConcretePowder => 64,
            Item::Obsidian => 64,
            Item::SmoothStoneSlab => 64,
            Item::Bell => 64,
            Item::RedNetherBrickSlab => 64,
            Item::OakBoat => 1,
            Item::EndStoneBrickSlab => 64,
            Item::MagentaShulkerBox => 1,
            Item::ShulkerSpawnEgg => 64,
            Item::DiamondAxe => 1,
            Item::DarkOakFence => 64,
            Item::CrackedNetherBricks => 64,
            Item::BlastFurnace => 64,
            Item::Furnace => 64,
            Item::CrackedStoneBricks => 64,
            Item::GraniteWall => 64,
            Item::ParrotSpawnEgg => 64,
            Item::SmoothRedSandstoneStairs => 64,
            Item::WaxedExposedCutCopper => 64,
            Item::NetherGoldOre => 64,
            Item::CrimsonTrapdoor => 64,
            Item::BirchBoat => 1,
            Item::HorseSpawnEgg => 64,
            Item::PandaSpawnEgg => 64,
            Item::OxidizedCutCopperSlab => 64,
            Item::HoneycombBlock => 64,
            Item::BoneBlock => 64,
            Item::CookedBeef => 64,
            Item::YellowTerracotta => 64,
            Item::AncientDebris => 64,
            Item::SmoothRedSandstoneSlab => 64,
            Item::StoneAxe => 1,
            Item::DeepslateTiles => 64,
            Item::BlackstoneWall => 64,
            Item::StonePickaxe => 1,
            Item::EnchantingTable => 64,
            Item::SmoothSandstone => 64,
            Item::Cobblestone => 64,
            Item::ShulkerBox => 1,
            Item::WoodenHoe => 1,
            Item::Terracotta => 64,
            Item::OakLeaves => 64,
            Item::AmethystBlock => 64,
            Item::Dirt => 64,
            Item::InfestedStoneBricks => 64,
            Item::AndesiteStairs => 64,
            Item::Light => 64,
            Item::Lead => 64,
            Item::HoneyBottle => 16,
            Item::EmeraldBlock => 64,
            Item::Diamond => 64,
            Item::AzaleaLeaves => 64,
            Item::LimeCandle => 64,
            Item::MagentaStainedGlass => 64,
            Item::PolishedBlackstoneBrickSlab => 64,
            Item::BirchWood => 64,
            Item::Comparator => 64,
            Item::DeepslateDiamondOre => 64,
            Item::MossyCobblestoneWall => 64,
            Item::Chain => 64,
            Item::RavagerSpawnEgg => 64,
            Item::CryingObsidian => 64,
            Item::YellowCandle => 64,
            Item::MushroomStew => 1,
            Item::SandstoneStairs => 64,
            Item::Lectern => 64,
            Item::GoldenSword => 1,
            Item::SpruceStairs => 64,
            Item::SquidSpawnEgg => 64,
            Item::BlackShulkerBox => 1,
            Item::WitherSkeletonSkull => 64,
            Item::EndRod => 64,
            Item::LimeShulkerBox => 1,
            Item::DeepslateTileSlab => 64,
            Item::MusicDiscPigstep => 1,
            Item::WaxedWeatheredCopper => 64,
            Item::BlackConcretePowder => 64,
            Item::GoldBlock => 64,
            Item::AcaciaPlanks => 64,
            Item::MusicDiscChirp => 1,
            Item::AcaciaSign => 16,
            Item::StrippedCrimsonHyphae => 64,
            Item::Composter => 64,
            Item::PetrifiedOakSlab => 64,
            Item::Rail => 64,
            Item::WitchSpawnEgg => 64,
            Item::CopperBlock => 64,
            Item::IronHelmet => 1,
            Item::NetheriteLeggings => 1,
            Item::Cobweb => 64,
            Item::BirchFenceGate => 64,
            Item::BubbleCoral => 64,
            Item::WhiteTerracotta => 64,
            Item::ZombieHead => 64,
            Item::NetheriteBoots => 1,
            Item::PolishedBlackstoneBricks => 64,
            Item::StrippedBirchWood => 64,
            Item::HornCoralFan => 64,
            Item::RedWool => 64,
            Item::Gravel => 64,
            Item::RedDye => 64,
            Item::WarpedRoots => 64,
            Item::StrippedOakWood => 64,
            Item::WarpedButton => 64,
            Item::NautilusShell => 64,
            Item::StructureBlock => 64,
            Item::HornCoral => 64,
            Item::Sand => 64,
            Item::ActivatorRail => 64,
            Item::BigDripleaf => 64,
            Item::AndesiteSlab => 64,
            Item::WhiteConcrete => 64,
            Item::QuartzSlab => 64,
            Item::ChestMinecart => 1,
            Item::RedStainedGlassPane => 64,
            Item::WaterBucket => 1,
            Item::BoneMeal => 64,
            Item::RoseBush => 64,
            Item::PiglinBannerPattern => 1,
            Item::WaxedCutCopper => 64,
            Item::WhiteTulip => 64,
            Item::WaxedCutCopperStairs => 64,
            Item::SugarCane => 64,
            Item::WarpedSign => 16,
            Item::ChiseledPolishedBlackstone => 64,
            Item::GhastSpawnEgg => 64,
            Item::Cornflower => 64,
            Item::MossyCobblestoneStairs => 64,
            Item::LimeConcrete => 64,
            Item::Lever => 64,
            Item::Feather => 64,
            Item::QuartzBlock => 64,
            Item::AzureBluet => 64,
            Item::WaxedOxidizedCopper => 64,
            Item::YellowShulkerBox => 1,
            Item::DeepslateTileStairs => 64,
            Item::WetSponge => 64,
            Item::SculkSensor => 64,
            Item::RawCopper => 64,
            Item::BlackStainedGlassPane => 64,
            Item::TropicalFishBucket => 1,
            Item::Beehive => 64,
            Item::WarpedFungusOnAStick => 64,
            Item::GraniteSlab => 64,
            Item::CutRedSandstone => 64,
            Item::GreenCandle => 64,
            Item::CookedCod => 64,
            Item::CobblestoneSlab => 64,
            Item::SlimeBall => 64,
            Item::WeepingVines => 64,
            Item::StrippedSpruceLog => 64,
            Item::PurpleCandle => 64,
            Item::PolishedDeepslateWall => 64,
            Item::Bricks => 64,
            Item::BrownTerracotta => 64,
            Item::TripwireHook => 64,
            Item::MediumAmethystBud => 64,
            Item::NetheriteBlock => 64,
            Item::PolishedBlackstoneWall => 64,
            Item::WaxedOxidizedCutCopperStairs => 64,
            Item::BubbleCoralFan => 64,
            Item::PurpleDye => 64,
            Item::StoneBrickSlab => 64,
            Item::Brick => 64,
            Item::SkullBannerPattern => 1,
            Item::SoulCampfire => 64,
            Item::WhiteWool => 64,
            Item::FireCoralBlock => 64,
            Item::SalmonBucket => 1,
            Item::CookedRabbit => 64,
            Item::RedNetherBrickWall => 64,
            Item::IronBars => 64,
            Item::BrownShulkerBox => 1,
            Item::DeadBrainCoral => 64,
            Item::Barrier => 64,
            Item::DeepslateBrickStairs => 64,
            Item::NetheriteSword => 1,
            Item::CyanStainedGlassPane => 64,
            Item::LightGrayBanner => 16,
            Item::WitherRose => 64,
            Item::NetheriteHoe => 1,
            Item::DeepslateGoldOre => 64,
            Item::BirchPressurePlate => 64,
            Item::IronIngot => 64,
            Item::OrangeConcrete => 64,
            Item::ChiseledStoneBricks => 64,
            Item::GrayConcrete => 64,
            Item::SlimeSpawnEgg => 64,
            Item::TrappedChest => 64,
            Item::SandstoneWall => 64,
            Item::BlackBed => 1,
            Item::NetherStar => 64,
            Item::BrainCoralBlock => 64,
            Item::DetectorRail => 64,
            Item::YellowBanner => 16,
            Item::SmoothSandstoneSlab => 64,
            Item::DarkOakSapling => 64,
            Item::YellowConcretePowder => 64,
            Item::PolishedAndesiteSlab => 64,
            Item::NetheriteIngot => 64,
            Item::EmeraldOre => 64,
            Item::HeavyWeightedPressurePlate => 64,
            Item::DarkOakSlab => 64,
            Item::PinkConcretePowder => 64,
            Item::CookedChicken => 64,
            Item::SilverfishSpawnEgg => 64,
            Item::PoppedChorusFruit => 64,
            Item::WarpedStem => 64,
            Item::NoteBlock => 64,
            Item::NetherQuartzOre => 64,
            Item::JackOLantern => 64,
            Item::BlueStainedGlass => 64,
            Item::HayBlock => 64,
            Item::ChiseledQuartzBlock => 64,
            Item::MagentaGlazedTerracotta => 64,
            Item::RedNetherBrickStairs => 64,
            Item::StoneSword => 1,
            Item::NetherBrick => 64,
            Item::Snowball => 16,
            Item::TurtleEgg => 64,
            Item::PolishedDioriteSlab => 64,
            Item::BrownWool => 64,
            Item::LimeBanner => 16,
            Item::BrickSlab => 64,
            Item::SmoothQuartzSlab => 64,
            Item::FireworkRocket => 64,
            Item::MossyStoneBrickSlab => 64,
            Item::Granite => 64,
            Item::ExperienceBottle => 64,
            Item::BlackCandle => 64,
            Item::GoldenCarrot => 64,
            Item::NetheriteShovel => 1,
            Item::Scute => 64,
            Item::CyanTerracotta => 64,
            Item::CobbledDeepslate => 64,
            Item::EndPortalFrame => 64,
            Item::PinkStainedGlass => 64,
            Item::GrayStainedGlassPane => 64,
            Item::GoldIngot => 64,
            Item::IronOre => 64,
            Item::QuartzBricks => 64,
            Item::StraySpawnEgg => 64,
            Item::JungleDoor => 64,
            Item::IronLeggings => 1,
            Item::RabbitSpawnEgg => 64,
            Item::GlowLichen => 64,
            Item::DarkOakDoor => 64,
            Item::EndStoneBrickStairs => 64,
            Item::PurpleBanner => 16,
            Item::Fern => 64,
            Item::Compass => 64,
            Item::OrangeStainedGlass => 64,
            Item::OrangeStainedGlassPane => 64,
            Item::IronHorseArmor => 1,
            Item::MossyStoneBricks => 64,
            Item::CyanConcrete => 64,
            Item::NameTag => 64,
            Item::InfestedCobblestone => 64,
            Item::JungleWood => 64,
            Item::Peony => 64,
            Item::SeaLantern => 64,
            Item::MusicDisc13 => 1,
            Item::ChickenSpawnEgg => 64,
            Item::GoldenHoe => 1,
            Item::Diorite => 64,
            Item::BrownCarpet => 64,
            Item::CrimsonHyphae => 64,
            Item::NetheriteScrap => 64,
            Item::DeadHornCoralBlock => 64,
            Item::OrangeGlazedTerracotta => 64,
            Item::PhantomSpawnEgg => 64,
            Item::AcaciaWood => 64,
            Item::OakSlab => 64,
            Item::AndesiteWall => 64,
            Item::LightningRod => 64,
            Item::MusicDiscFar => 1,
            Item::WolfSpawnEgg => 64,
            Item::Piston => 64,
            Item::ChiseledNetherBricks => 64,
            Item::BlueConcrete => 64,
            Item::CookedMutton => 64,
            Item::WoodenShovel => 1,
            Item::JungleSign => 16,
            Item::SweetBerries => 64,
            Item::EndermiteSpawnEgg => 64,
            Item::TropicalFishSpawnEgg => 64,
            Item::SpruceBoat => 1,
            Item::MossyCobblestone => 64,
            Item::Elytra => 1,
            Item::SmithingTable => 64,
            Item::DaylightDetector => 64,
            Item::RabbitHide => 64,
            Item::CrimsonDoor => 64,
            Item::BrewingStand => 64,
            Item::GoatSpawnEgg => 64,
            Item::Vine => 64,
            Item::PolishedDeepslateSlab => 64,
            Item::DarkPrismarine => 64,
            Item::LlamaSpawnEgg => 64,
            Item::DeepslateRedstoneOre => 64,
            Item::GlassPane => 64,
            Item::OrangeCarpet => 64,
            Item::PolishedDeepslate => 64,
            Item::OrangeTulip => 64,
            Item::CrimsonButton => 64,
            Item::Arrow => 64,
            Item::LimeTerracotta => 64,
            Item::MagentaTerracotta => 64,
            Item::Spyglass => 1,
            Item::Potion => 1,
            Item::CutRedSandstoneSlab => 64,
            Item::PolishedBlackstoneBrickWall => 64,
            Item::DarkOakLog => 64,
            Item::LapisLazuli => 64,
            Item::BeeSpawnEgg => 64,
            Item::RedBanner => 16,
            Item::EnderEye => 64,
            Item::ExposedCopper => 64,
            Item::GuardianSpawnEgg => 64,
            Item::PinkCandle => 64,
            Item::CutCopperStairs => 64,
            Item::CodBucket => 1,
            Item::MagmaCubeSpawnEgg => 64,
            Item::DragonHead => 64,
            Item::IronBlock => 64,
            Item::PackedIce => 64,
            Item::JungleFence => 64,
            Item::CraftingTable => 64,
            Item::WhiteGlazedTerracotta => 64,
            Item::LightWeightedPressurePlate => 64,
            Item::IronShovel => 1,
            Item::PrismarineSlab => 64,
            Item::DeadBush => 64,
            Item::RabbitStew => 1,
            Item::Allium => 64,
            Item::TwistingVines => 64,
            Item::StoneBrickStairs => 64,
            Item::PufferfishSpawnEgg => 64,
            Item::CatSpawnEgg => 64,
            Item::DriedKelp => 64,
            Item::TintedGlass => 64,
            Item::OxidizedCutCopperStairs => 64,
            Item::DeadHornCoral => 64,
            Item::FireworkStar => 64,
            Item::RedNetherBricks => 64,
            Item::SalmonSpawnEgg => 64,
            Item::LightBlueDye => 64,
            Item::PrismarineShard => 64,
            Item::RottenFlesh => 64,
            Item::LimeStainedGlass => 64,
            Item::CoalOre => 64,
            Item::LightGrayWool => 64,
            Item::DarkOakPressurePlate => 64,
            Item::Bowl => 64,
            Item::Sugar => 64,
            Item::BlueDye => 64,
            Item::Blackstone => 64,
            Item::Snow => 64,
            Item::CobblestoneWall => 64,
            Item::AcaciaFence => 64,
            Item::WhiteConcretePowder => 64,
            Item::PolishedBlackstoneButton => 64,
            Item::Rabbit => 64,
            Item::MusicDiscCat => 1,
            Item::WhiteCandle => 64,
            Item::PiglinBruteSpawnEgg => 64,
            Item::WaxedWeatheredCutCopper => 64,
            Item::CoarseDirt => 64,
            Item::DripstoneBlock => 64,
            Item::YellowCarpet => 64,
            Item::TubeCoralBlock => 64,
            Item::WarpedPlanks => 64,
            Item::CyanCarpet => 64,
            Item::DeadBubbleCoral => 64,
            Item::FlintAndSteel => 1,
            Item::WheatSeeds => 64,
            Item::PurpleBed => 1,
            Item::Crossbow => 1,
            Item::Redstone => 64,
            Item::JunglePressurePlate => 64,
            Item::EnderChest => 64,
            Item::Leather => 64,
            Item::RedstoneLamp => 64,
            Item::CookedSalmon => 64,
            Item::CocoaBeans => 64,
            Item::MusicDiscWard => 1,
            Item::WaxedExposedCutCopperSlab => 64,
            Item::PurpleStainedGlass => 64,
            Item::SprucePressurePlate => 64,
            Item::WaxedCutCopperSlab => 64,
            Item::NetherBrickWall => 64,
            Item::InkSac => 64,
            Item::TraderLlamaSpawnEgg => 64,
            Item::BeetrootSeeds => 64,
            Item::LightBlueBed => 1,
            Item::IronBoots => 1,
            Item::NetherWartBlock => 64,
            Item::PolishedAndesiteStairs => 64,
            Item::GoldenApple => 64,
            Item::GrayDye => 64,
            Item::PurpleShulkerBox => 1,
            Item::ZombieVillagerSpawnEgg => 64,
            Item::BirchSapling => 64,
            Item::LightGrayCarpet => 64,
            Item::GreenStainedGlass => 64,
            Item::LeatherBoots => 1,
            Item::ChainmailHelmet => 1,
            Item::ClayBall => 64,
            Item::OakFenceGate => 64,
            Item::FlowerBannerPattern => 1,
            Item::LargeAmethystBud => 64,
            Item::DeepslateBrickSlab => 64,
            Item::DeepslateIronOre => 64,
            Item::Jukebox => 64,
            Item::GoldenPickaxe => 1,
            Item::VillagerSpawnEgg => 64,
            Item::BlackDye => 64,
            Item::Deepslate => 64,
            Item::LightGrayBed => 1,
            Item::BlueOrchid => 64,
            Item::DeadFireCoralBlock => 64,
            Item::WoodenPickaxe => 1,
            Item::CrackedDeepslateBricks => 64,
            Item::PrismarineStairs => 64,
            Item::AcaciaPressurePlate => 64,
            Item::LilyOfTheValley => 64,
            Item::BlueStainedGlassPane => 64,
            Item::SpruceFenceGate => 64,
            Item::Painting => 64,
            Item::Bundle => 1,
            Item::CaveSpiderSpawnEgg => 64,
            Item::DragonBreath => 64,
            Item::CyanBanner => 16,
            Item::PrismarineBrickStairs => 64,
            Item::MuleSpawnEgg => 64,
            Item::DioriteStairs => 64,
            Item::PiglinSpawnEgg => 64,
            Item::LightBlueWool => 64,
            Item::WoodenSword => 1,
            Item::MusicDiscMall => 1,
            Item::GrayConcretePowder => 64,
            Item::MossBlock => 64,
            Item::PrismarineBrickSlab => 64,
            Item::MagentaCarpet => 64,
            Item::WaxedOxidizedCutCopperSlab => 64,
            Item::WrittenBook => 16,
            Item::PrismarineCrystals => 64,
            Item::StoneButton => 64,
            Item::BlueShulkerBox => 1,
            Item::MusicDiscOtherside => 1,
            Item::SporeBlossom => 64,
            Item::DriedKelpBlock => 64,
            Item::GlowBerries => 64,
            Item::StoneStairs => 64,
            Item::Quartz => 64,
            Item::StrippedJungleLog => 64,
            Item::WaxedWeatheredCutCopperStairs => 64,
            Item::PinkGlazedTerracotta => 64,
            Item::TippedArrow => 64,
            Item::BlueIce => 64,
            Item::StickyPiston => 64,
            Item::JungleFenceGate => 64,
            Item::CarvedPumpkin => 64,
            Item::RedConcrete => 64,
            Item::RedSandstoneStairs => 64,
            Item::DiamondBoots => 1,
            Item::OakFence => 64,
            Item::Smoker => 64,
            Item::CommandBlock => 64,
            Item::LightBlueBanner => 16,
            Item::DirtPath => 64,
            Item::BrownConcretePowder => 64,
            Item::TubeCoral => 64,
            Item::GlowItemFrame => 64,
            Item::PlayerHead => 64,
            Item::BrownDye => 64,
            Item::HoglinSpawnEgg => 64,
            Item::Ice => 64,
            Item::StoneHoe => 1,
            Item::RawCopperBlock => 64,
            Item::SmoothRedSandstone => 64,
            Item::PolishedDeepslateStairs => 64,
            Item::GreenBanner => 16,
            Item::LargeFern => 64,
            Item::PurpurStairs => 64,
            Item::CyanShulkerBox => 1,
            Item::PolarBearSpawnEgg => 64,
            Item::DeadBrainCoralFan => 64,
            Item::BirchPlanks => 64,
            Item::YellowConcrete => 64,
            Item::IronAxe => 1,
            Item::RedTerracotta => 64,
            Item::BlueBanner => 16,
            Item::CyanGlazedTerracotta => 64,
            Item::GrayBed => 1,
            Item::Dispenser => 64,
            Item::DiamondLeggings => 1,
            Item::BirchFence => 64,
            Item::Bucket => 16,
            Item::PolishedAndesite => 64,
            Item::RedstoneOre => 64,
            Item::WarpedPressurePlate => 64,
            Item::DiamondHoe => 1,
            Item::BeeNest => 64,
            Item::Anvil => 64,
            Item::ZombieHorseSpawnEgg => 64,
            Item::BlueCandle => 64,
            Item::BrownStainedGlassPane => 64,
            Item::RedStainedGlass => 64,
            Item::DrownedSpawnEgg => 64,
            Item::BakedPotato => 64,
            Item::FletchingTable => 64,
            Item::ChainmailBoots => 1,
            Item::SpruceLog => 64,
            Item::BlackstoneStairs => 64,
            Item::AcaciaSlab => 64,
            Item::StrippedOakLog => 64,
            Item::WaxedCopperBlock => 64,
            Item::Bookshelf => 64,
            Item::StrippedBirchLog => 64,
            Item::PinkShulkerBox => 1,
            Item::GlisteringMelonSlice => 64,
            Item::Basalt => 64,
            Item::GoldenLeggings => 1,
            Item::AcaciaLog => 64,
            Item::PinkBed => 1,
            Item::Glass => 64,
            Item::CopperOre => 64,
            Item::LimeConcretePowder => 64,
            Item::Podzol => 64,
            Item::GreenBed => 1,
            Item::BlackConcrete => 64,
            Item::BirchLeaves => 64,
            Item::ChainCommandBlock => 64,
            Item::Scaffolding => 64,
            Item::DeadTubeCoralBlock => 64,
            Item::Bread => 64,
            Item::PufferfishBucket => 1,
            Item::BirchSlab => 64,
            Item::SmoothStone => 64,
            Item::LightGrayDye => 64,
            Item::PinkConcrete => 64,
            Item::QuartzStairs => 64,
            Item::MagmaCream => 64,
            Item::WhiteBanner => 16,
            Item::SuspiciousStew => 1,
            Item::EndStoneBrickWall => 64,
            Item::Wheat => 64,
            Item::MelonSlice => 64,
            Item::SpruceFence => 64,
            Item::WeatheredCutCopperSlab => 64,
            Item::OakDoor => 64,
            Item::GrayBanner => 16,
            Item::BlackstoneSlab => 64,
            Item::DarkOakWood => 64,
            Item::PigSpawnEgg => 64,
            Item::StriderSpawnEgg => 64,
            Item::DarkOakFenceGate => 64,
            Item::CrimsonStairs => 64,
            Item::NetheriteChestplate => 1,
            Item::ChorusPlant => 64,
            Item::ExposedCutCopperStairs => 64,
            Item::MossyStoneBrickWall => 64,
            Item::OakWood => 64,
            Item::LimeBed => 1,
            Item::Charcoal => 64,
            Item::Hopper => 64,
            Item::NetheriteHelmet => 1,
            Item::Cookie => 64,
            Item::EnchantedBook => 1,
            Item::WhiteCarpet => 64,
            Item::Ladder => 64,
            Item::LightGrayConcretePowder => 64,
            Item::GreenWool => 64,
            Item::Shears => 1,
            Item::JungleButton => 64,
            Item::Gunpowder => 64,
            Item::CutSandstone => 64,
            Item::CrimsonNylium => 64,
            Item::DiamondHorseArmor => 1,
            Item::MusicDiscMellohi => 1,
            Item::GoldenBoots => 1,
            Item::AcaciaFenceGate => 64,
            Item::MilkBucket => 1,
            Item::BrownGlazedTerracotta => 64,
            Item::Stick => 64,
            Item::RedCandle => 64,
            Item::Grass => 64,
            Item::MagentaBanner => 16,
            Item::PumpkinSeeds => 64,
            Item::GlowSquidSpawnEgg => 64,
            Item::CobbledDeepslateStairs => 64,
            Item::JungleTrapdoor => 64,
            Item::MusicDiscStrad => 1,
            Item::OcelotSpawnEgg => 64,
            Item::Map => 64,
            Item::GoldenHelmet => 1,
            Item::StoneSlab => 64,
            Item::Sponge => 64,
            Item::PurpleStainedGlassPane => 64,
            Item::PurpleConcretePowder => 64,
            Item::SpruceSapling => 64,
            Item::MusicDiscStal => 1,
            Item::BubbleCoralBlock => 64,
            Item::GhastTear => 64,
            Item::SpruceTrapdoor => 64,
            Item::EvokerSpawnEgg => 64,
            Item::DiamondPickaxe => 1,
            Item::OxidizedCutCopper => 64,
            Item::BrickStairs => 64,
            Item::AcaciaButton => 64,
            Item::AmethystShard => 64,
            Item::Paper => 64,
            Item::GreenGlazedTerracotta => 64,
            Item::GraniteStairs => 64,
            Item::Melon => 64,
            Item::EnchantedGoldenApple => 64,
            Item::NetheriteAxe => 1,
            Item::DiamondSword => 1,
            Item::GrayCarpet => 64,
            Item::SpruceSign => 16,
            Item::CrimsonSign => 16,
            Item::Beef => 64,
            Item::JungleLog => 64,
            Item::StrippedDarkOakLog => 64,
            Item::EndCrystal => 64,
            Item::FoxSpawnEgg => 64,
            Item::CreeperBannerPattern => 1,
            Item::WeatheredCopper => 64,
            Item::WeatheredCutCopperStairs => 64,
            Item::WarpedFungus => 64,
            Item::HeartOfTheSea => 64,
            Item::WhiteBed => 1,
            Item::BlueGlazedTerracotta => 64,
            Item::WitherSkeletonSpawnEgg => 64,
            Item::DamagedAnvil => 64,
            Item::FurnaceMinecart => 1,
            Item::OakSign => 16,
            Item::StrippedDarkOakWood => 64,
            Item::MagentaConcretePowder => 64,
            Item::SpiderSpawnEgg => 64,
            Item::FireCoralFan => 64,
            Item::PurpleGlazedTerracotta => 64,
            Item::ExposedCutCopper => 64,
            Item::PolishedBlackstonePressurePlate => 64,
            Item::Bone => 64,
            Item::OakLog => 64,
            Item::BrainCoral => 64,
            Item::Trident => 1,
            Item::FireCharge => 64,
            Item::SoulSand => 64,
            Item::GrayShulkerBox => 1,
            Item::LightGrayTerracotta => 64,
            Item::DarkPrismarineSlab => 64,
            Item::GreenStainedGlassPane => 64,
            Item::Prismarine => 64,
            Item::PrismarineBricks => 64,
            Item::GrayGlazedTerracotta => 64,
            Item::RedCarpet => 64,
            Item::StrippedSpruceWood => 64,
            Item::ChiseledRedSandstone => 64,
            Item::ChainmailChestplate => 1,
            Item::GoldenChestplate => 1,
            Item::LavaBucket => 1,
            Item::RabbitFoot => 64,
            Item::CartographyTable => 64,
            Item::BirchDoor => 64,
            Item::SmallDripleaf => 64,
            Item::TntMinecart => 1,
            Item::WoodenAxe => 1,
            Item::CyanConcretePowder => 64,
            Item::SoulLantern => 64,
            Item::PinkWool => 64,
            Item::BrownMushroomBlock => 64,
            Item::DeadTubeCoralFan => 64,
            Item::OakPressurePlate => 64,
            Item::BeetrootSoup => 1,
            Item::OakPlanks => 64,
            Item::LightGrayStainedGlass => 64,
            Item::Observer => 64,
            Item::FlowerPot => 64,
            Item::RedBed => 1,
            Item::BlueBed => 1,
            Item::DarkOakPlanks => 64,
            Item::DioriteWall => 64,
            Item::Poppy => 64,
            Item::Spawner => 64,
            Item::WarpedTrapdoor => 64,
            Item::PinkDye => 64,
            Item::BlackTerracotta => 64,
            Item::RedShulkerBox => 1,
            Item::CrimsonRoots => 64,
            Item::Bamboo => 64,
            Item::AcaciaDoor => 64,
            Item::Tuff => 64,
            Item::CrimsonStem => 64,
            Item::SnowBlock => 64,
            Item::AcaciaStairs => 64,
            Item::YellowBed => 1,
            Item::TallGrass => 64,
            Item::OrangeDye => 64,
            Item::StoneBrickWall => 64,
            Item::GrassBlock => 64,
            Item::PurpleTerracotta => 64,
            Item::GreenConcrete => 64,
            Item::Target => 64,
            Item::RedstoneTorch => 64,
            Item::PurpurSlab => 64,
            Item::GreenDye => 64,
            Item::HoneyBlock => 64,
            Item::PoisonousPotato => 64,
            Item::FilledMap => 64,
            Item::WarpedStairs => 64,
            Item::StrippedWarpedHyphae => 64,
            Item::SpruceLeaves => 64,
            Item::BlackCarpet => 64,
            Item::PolishedBlackstoneSlab => 64,
            Item::SheepSpawnEgg => 64,
            Item::JungleSapling => 64,
            Item::MagentaWool => 64,
            Item::WarpedFence => 64,
            Item::ArmorStand => 16,
            Item::InfestedDeepslate => 64,
            Item::GlassBottle => 64,
            Item::WhiteStainedGlass => 64,
            Item::MagentaCandle => 64,
            Item::RedMushroom => 64,
            Item::CopperIngot => 64,
            Item::DioriteSlab => 64,
            Item::TurtleHelmet => 1,
            Item::GoldNugget => 64,
            Item::StonePressurePlate => 64,
            Item::NetherBricks => 64,
            Item::Dropper => 64,
            Item::Stone => 64,
            Item::PointedDripstone => 64,
            Item::PurpleWool => 64,
            Item::LimeDye => 64,
            Item::DeepslateLapisOre => 64,
            Item::CrimsonSlab => 64,
            Item::GoldOre => 64,
            Item::BlackStainedGlass => 64,
            Item::Tnt => 64,
            Item::WarpedNylium => 64,
            Item::IronChestplate => 1,
            Item::SandstoneSlab => 64,
            Item::IronTrapdoor => 64,
            Item::GoldenAxe => 1,
            Item::FloweringAzaleaLeaves => 64,
            Item::AxolotlSpawnEgg => 64,
            Item::AxolotlBucket => 1,
            Item::GoldenHorseArmor => 1,
            Item::StrippedJungleWood => 64,
            Item::LeatherChestplate => 1,
            Item::AcaciaLeaves => 64,
            Item::InfestedChiseledStoneBricks => 64,
            Item::GrayCandle => 64,
            Item::Lilac => 64,
            Item::LimeGlazedTerracotta => 64,
            Item::TubeCoralFan => 64,
            Item::CodSpawnEgg => 64,
            Item::LimeStainedGlassPane => 64,
            Item::RedTulip => 64,
            Item::LightGrayStainedGlassPane => 64,
            Item::DiamondShovel => 1,
            Item::WarpedWartBlock => 64,
            Item::YellowDye => 64,
            Item::SpruceWood => 64,
            Item::CoalBlock => 64,
            Item::ChiseledSandstone => 64,
            Item::WaxedExposedCutCopperStairs => 64,
            Item::GildedBlackstone => 64,
            Item::Torch => 64,
            Item::WritableBook => 1,
            Item::TotemOfUndying => 1,
            Item::WarpedHyphae => 64,
            Item::PillagerSpawnEgg => 64,
            Item::BirchButton => 64,
            Item::CyanWool => 64,
            Item::Mutton => 64,
            Item::FireCoral => 64,
            Item::Lodestone => 64,
            Item::WarpedSlab => 64,
            Item::BlueCarpet => 64,
            Item::Azalea => 64,
            Item::BirchStairs => 64,
            Item::Pumpkin => 64,
            Item::StrippedCrimsonStem => 64,
            Item::YellowWool => 64,
            Item::CobbledDeepslateSlab => 64,
            Item::DarkOakTrapdoor => 64,
            Item::BrickWall => 64,
            Item::LightBlueConcretePowder => 64,
            Item::Farmland => 64,
            Item::Book => 64,
            Item::BatSpawnEgg => 64,
            Item::ItemFrame => 64,
            Item::BlackBanner => 16,
            Item::LeatherLeggings => 1,
            Item::PumpkinPie => 64,
        }
    }
}
impl Item {
    #[doc = "Returns the `max_durability` property of this `Item`."]
    #[inline]
    pub fn max_durability(&self) -> Option<u32> {
        match self {
            Item::IronBars => None,
            Item::PinkGlazedTerracotta => None,
            Item::WarpedPlanks => None,
            Item::IronNugget => None,
            Item::EndStoneBrickSlab => None,
            Item::CobbledDeepslateStairs => None,
            Item::StrippedOakLog => None,
            Item::PolishedBlackstone => None,
            Item::DeepslateBrickStairs => None,
            Item::StoneShovel => Some(131),
            Item::OakSign => None,
            Item::SporeBlossom => None,
            Item::SlimeBall => None,
            Item::WeepingVines => None,
            Item::Compass => None,
            Item::CodSpawnEgg => None,
            Item::QuartzBricks => None,
            Item::Map => None,
            Item::PiglinBannerPattern => None,
            Item::NetheriteShovel => Some(2031),
            Item::CyanWool => None,
            Item::RedSandstoneSlab => None,
            Item::PinkDye => None,
            Item::SplashPotion => None,
            Item::PolishedBlackstoneSlab => None,
            Item::WaxedWeatheredCopper => None,
            Item::BrownTerracotta => None,
            Item::PoweredRail => None,
            Item::JungleSign => None,
            Item::Tnt => None,
            Item::BlackTerracotta => None,
            Item::AcaciaDoor => None,
            Item::LightGrayBanner => None,
            Item::CrimsonStairs => None,
            Item::MossyStoneBricks => None,
            Item::SandstoneSlab => None,
            Item::InfestedCobblestone => None,
            Item::BlackShulkerBox => None,
            Item::MagentaBed => None,
            Item::Tuff => None,
            Item::BlueCarpet => None,
            Item::BirchFenceGate => None,
            Item::GreenDye => None,
            Item::MusicDiscFar => None,
            Item::SpruceFenceGate => None,
            Item::PinkStainedGlass => None,
            Item::JackOLantern => None,
            Item::Pumpkin => None,
            Item::AndesiteWall => None,
            Item::DripstoneBlock => None,
            Item::CutCopperSlab => None,
            Item::RedCandle => None,
            Item::PinkWool => None,
            Item::DeadFireCoral => None,
            Item::BlackstoneWall => None,
            Item::EndPortalFrame => None,
            Item::RedstoneTorch => None,
            Item::BatSpawnEgg => None,
            Item::BrownBed => None,
            Item::SmoothSandstone => None,
            Item::ExposedCutCopperSlab => None,
            Item::DarkOakWood => None,
            Item::DiamondAxe => Some(1561),
            Item::ZombieVillagerSpawnEgg => None,
            Item::Comparator => None,
            Item::JungleSapling => None,
            Item::EndStoneBricks => None,
            Item::BlueStainedGlassPane => None,
            Item::Cod => None,
            Item::BrownDye => None,
            Item::DeadFireCoralBlock => None,
            Item::Quartz => None,
            Item::AcaciaSapling => None,
            Item::OakSapling => None,
            Item::DeadHornCoralBlock => None,
            Item::DiamondPickaxe => Some(1561),
            Item::ChainmailLeggings => Some(225),
            Item::PurpurStairs => None,
            Item::BlackStainedGlass => None,
            Item::DiamondHorseArmor => None,
            Item::Dispenser => None,
            Item::CookedPorkchop => None,
            Item::WaxedWeatheredCutCopperSlab => None,
            Item::AcaciaSign => None,
            Item::Chicken => None,
            Item::YellowBanner => None,
            Item::LargeAmethystBud => None,
            Item::Beehive => None,
            Item::SpruceSign => None,
            Item::PurpleStainedGlass => None,
            Item::WarpedTrapdoor => None,
            Item::PolishedGranite => None,
            Item::PolishedBlackstoneWall => None,
            Item::WitherSkeletonSpawnEgg => None,
            Item::Bricks => None,
            Item::BrownShulkerBox => None,
            Item::PolishedBlackstoneButton => None,
            Item::Snow => None,
            Item::TurtleHelmet => Some(275),
            Item::FilledMap => None,
            Item::StoneButton => None,
            Item::DeepslateRedstoneOre => None,
            Item::AcaciaLeaves => None,
            Item::EndRod => None,
            Item::BrownGlazedTerracotta => None,
            Item::HornCoral => None,
            Item::RawCopper => None,
            Item::OrangeStainedGlass => None,
            Item::RawGold => None,
            Item::BlackBed => None,
            Item::SalmonSpawnEgg => None,
            Item::EnchantingTable => None,
            Item::AmethystCluster => None,
            Item::SmoothStoneSlab => None,
            Item::StoneBrickWall => None,
            Item::GreenStainedGlassPane => None,
            Item::PolishedAndesiteStairs => None,
            Item::CobblestoneStairs => None,
            Item::Coal => None,
            Item::PhantomMembrane => None,
            Item::Lantern => None,
            Item::MagmaCubeSpawnEgg => None,
            Item::RawIronBlock => None,
            Item::GoatSpawnEgg => None,
            Item::PrismarineBricks => None,
            Item::MossyStoneBrickStairs => None,
            Item::CutSandstone => None,
            Item::CarrotOnAStick => Some(25),
            Item::GoldenShovel => Some(32),
            Item::GoldNugget => None,
            Item::CreeperBannerPattern => None,
            Item::DeepslateGoldOre => None,
            Item::OrangeStainedGlassPane => None,
            Item::DeadFireCoralFan => None,
            Item::PrismarineWall => None,
            Item::SmoothRedSandstone => None,
            Item::Charcoal => None,
            Item::MagmaBlock => None,
            Item::DarkOakSign => None,
            Item::IronLeggings => Some(225),
            Item::SkeletonHorseSpawnEgg => None,
            Item::BlueStainedGlass => None,
            Item::StrippedSpruceWood => None,
            Item::RedSandstone => None,
            Item::NetheriteScrap => None,
            Item::PolarBearSpawnEgg => None,
            Item::FireCoralFan => None,
            Item::GoldenCarrot => None,
            Item::LeatherChestplate => Some(80),
            Item::WaxedCopperBlock => None,
            Item::OrangeBed => None,
            Item::BlueConcrete => None,
            Item::PrismarineBrickSlab => None,
            Item::WeatheredCutCopperSlab => None,
            Item::DirtPath => None,
            Item::BirchBoat => None,
            Item::DeepslateBrickWall => None,
            Item::SpectralArrow => None,
            Item::BeetrootSeeds => None,
            Item::YellowTerracotta => None,
            Item::WhiteConcrete => None,
            Item::OrangeTulip => None,
            Item::Scaffolding => None,
            Item::BirchDoor => None,
            Item::DriedKelpBlock => None,
            Item::WhiteCarpet => None,
            Item::StoneHoe => Some(131),
            Item::WetSponge => None,
            Item::LimeWool => None,
            Item::SmoothStone => None,
            Item::BirchStairs => None,
            Item::LightGrayStainedGlassPane => None,
            Item::RedMushroom => None,
            Item::PufferfishSpawnEgg => None,
            Item::DiamondChestplate => Some(528),
            Item::VindicatorSpawnEgg => None,
            Item::CutSandstoneSlab => None,
            Item::NetherQuartzOre => None,
            Item::SoulSand => None,
            Item::MushroomStem => None,
            Item::ChippedAnvil => None,
            Item::Terracotta => None,
            Item::WeatheredCopper => None,
            Item::GlowItemFrame => None,
            Item::KnowledgeBook => None,
            Item::Jukebox => None,
            Item::MusicDiscStrad => None,
            Item::NetherWart => None,
            Item::RootedDirt => None,
            Item::EmeraldBlock => None,
            Item::HayBlock => None,
            Item::LightBlueConcretePowder => None,
            Item::ParrotSpawnEgg => None,
            Item::DeepslateDiamondOre => None,
            Item::PinkStainedGlassPane => None,
            Item::PurpleCarpet => None,
            Item::ChiseledSandstone => None,
            Item::MagentaCandle => None,
            Item::NetheriteChestplate => Some(592),
            Item::WhiteTerracotta => None,
            Item::YellowConcrete => None,
            Item::CrimsonStem => None,
            Item::DarkPrismarine => None,
            Item::GoldIngot => None,
            Item::EnchantedBook => None,
            Item::MagmaCream => None,
            Item::SeaPickle => None,
            Item::CookedMutton => None,
            Item::GoldenLeggings => Some(105),
            Item::ChiseledStoneBricks => None,
            Item::RabbitSpawnEgg => None,
            Item::LimeStainedGlassPane => None,
            Item::LeatherBoots => Some(65),
            Item::CrimsonNylium => None,
            Item::TotemOfUndying => None,
            Item::JungleFence => None,
            Item::AxolotlBucket => None,
            Item::WaxedOxidizedCopper => None,
            Item::RedCarpet => None,
            Item::EvokerSpawnEgg => None,
            Item::GrayConcretePowder => None,
            Item::JungleButton => None,
            Item::AncientDebris => None,
            Item::InfestedCrackedStoneBricks => None,
            Item::DolphinSpawnEgg => None,
            Item::MooshroomSpawnEgg => None,
            Item::FireCoralBlock => None,
            Item::IronChestplate => Some(240),
            Item::AmethystShard => None,
            Item::NetheritePickaxe => Some(2031),
            Item::HopperMinecart => None,
            Item::ShulkerShell => None,
            Item::PhantomSpawnEgg => None,
            Item::StoneBrickStairs => None,
            Item::LimeShulkerBox => None,
            Item::PurpleBanner => None,
            Item::BrainCoralFan => None,
            Item::RedConcretePowder => None,
            Item::EndCrystal => None,
            Item::CutCopperStairs => None,
            Item::RedStainedGlassPane => None,
            Item::AcaciaFence => None,
            Item::CyanDye => None,
            Item::BeeNest => None,
            Item::JunglePlanks => None,
            Item::Mycelium => None,
            Item::GoldenPickaxe => Some(32),
            Item::RepeatingCommandBlock => None,
            Item::StrippedBirchLog => None,
            Item::DeadTubeCoralBlock => None,
            Item::LargeFern => None,
            Item::LightWeightedPressurePlate => None,
            Item::CrimsonTrapdoor => None,
            Item::DarkOakSlab => None,
            Item::CrackedDeepslateBricks => None,
            Item::CobbledDeepslateSlab => None,
            Item::StrippedOakWood => None,
            Item::NetherWartBlock => None,
            Item::Shroomlight => None,
            Item::HoglinSpawnEgg => None,
            Item::SpiderSpawnEgg => None,
            Item::GlassBottle => None,
            Item::BubbleCoral => None,
            Item::BlackCandle => None,
            Item::DeepslateTileSlab => None,
            Item::GreenConcretePowder => None,
            Item::PackedIce => None,
            Item::DarkOakLog => None,
            Item::NetheriteLeggings => Some(555),
            Item::Salmon => None,
            Item::WrittenBook => None,
            Item::RabbitFoot => None,
            Item::SlimeBlock => None,
            Item::IronHelmet => Some(165),
            Item::JungleFenceGate => None,
            Item::SweetBerries => None,
            Item::LightBlueTerracotta => None,
            Item::BrickWall => None,
            Item::Grass => None,
            Item::LightGrayCarpet => None,
            Item::RoseBush => None,
            Item::ExposedCutCopperStairs => None,
            Item::StonePressurePlate => None,
            Item::SkullBannerPattern => None,
            Item::HoneyBottle => None,
            Item::AcaciaStairs => None,
            Item::GrayShulkerBox => None,
            Item::BlackstoneStairs => None,
            Item::BirchSapling => None,
            Item::YellowStainedGlass => None,
            Item::FishingRod => Some(64),
            Item::LlamaSpawnEgg => None,
            Item::PolishedDeepslateStairs => None,
            Item::Peony => None,
            Item::Calcite => None,
            Item::RedTerracotta => None,
            Item::PinkTulip => None,
            Item::BrickSlab => None,
            Item::WarpedDoor => None,
            Item::WaxedOxidizedCutCopper => None,
            Item::DebugStick => None,
            Item::StoneSword => Some(131),
            Item::Emerald => None,
            Item::BrownCarpet => None,
            Item::LightBlueWool => None,
            Item::Barrier => None,
            Item::RedstoneLamp => None,
            Item::YellowConcretePowder => None,
            Item::BubbleCoralBlock => None,
            Item::WhiteTulip => None,
            Item::BrownMushroom => None,
            Item::DarkOakFenceGate => None,
            Item::StoneStairs => None,
            Item::WaterBucket => None,
            Item::SpruceBoat => None,
            Item::CatSpawnEgg => None,
            Item::BlueBanner => None,
            Item::CrimsonPressurePlate => None,
            Item::MusicDisc11 => None,
            Item::ChainmailBoots => Some(195),
            Item::CoalOre => None,
            Item::LimeStainedGlass => None,
            Item::BirchLeaves => None,
            Item::ChainmailHelmet => Some(165),
            Item::InfestedStone => None,
            Item::HoneyBlock => None,
            Item::PinkBanner => None,
            Item::TallGrass => None,
            Item::RedConcrete => None,
            Item::LightGrayConcretePowder => None,
            Item::StoneAxe => Some(131),
            Item::ExposedCopper => None,
            Item::SmoothRedSandstoneSlab => None,
            Item::SandstoneWall => None,
            Item::EndStoneBrickWall => None,
            Item::DiamondShovel => Some(1561),
            Item::CyanStainedGlassPane => None,
            Item::Scute => None,
            Item::SprucePlanks => None,
            Item::PurpurBlock => None,
            Item::GreenStainedGlass => None,
            Item::OakDoor => None,
            Item::GrassBlock => None,
            Item::BlueShulkerBox => None,
            Item::WarpedNylium => None,
            Item::Cookie => None,
            Item::CopperBlock => None,
            Item::DragonHead => None,
            Item::OrangeConcretePowder => None,
            Item::DeepslateCopperOre => None,
            Item::TropicalFishBucket => None,
            Item::Cactus => None,
            Item::BrownStainedGlass => None,
            Item::GrayBed => None,
            Item::CreeperHead => None,
            Item::DioriteStairs => None,
            Item::DeepslateBricks => None,
            Item::AcaciaWood => None,
            Item::BrownStainedGlassPane => None,
            Item::PinkCarpet => None,
            Item::CodBucket => None,
            Item::ChiseledPolishedBlackstone => None,
            Item::DioriteSlab => None,
            Item::GoldenHelmet => Some(77),
            Item::CutRedSandstone => None,
            Item::FireCoral => None,
            Item::DiamondSword => Some(1561),
            Item::DeadBubbleCoral => None,
            Item::Bell => None,
            Item::BirchButton => None,
            Item::Candle => None,
            Item::ChorusFlower => None,
            Item::Sugar => None,
            Item::MelonSeeds => None,
            Item::OakPlanks => None,
            Item::NetheriteBlock => None,
            Item::LightBlueConcrete => None,
            Item::DeadBrainCoralFan => None,
            Item::Jigsaw => None,
            Item::Mutton => None,
            Item::WaxedOxidizedCutCopperSlab => None,
            Item::Loom => None,
            Item::FloweringAzaleaLeaves => None,
            Item::WarpedFungus => None,
            Item::Barrel => None,
            Item::OakWood => None,
            Item::BrainCoralBlock => None,
            Item::Saddle => None,
            Item::LilyOfTheValley => None,
            Item::LightGrayDye => None,
            Item::SpiderEye => None,
            Item::BlueDye => None,
            Item::DeepslateTiles => None,
            Item::LeatherHorseArmor => None,
            Item::CopperOre => None,
            Item::IronBlock => None,
            Item::GoldenSword => Some(32),
            Item::GraniteSlab => None,
            Item::WheatSeeds => None,
            Item::Bowl => None,
            Item::RedNetherBricks => None,
            Item::StrippedSpruceLog => None,
            Item::InfestedStoneBricks => None,
            Item::DeepslateEmeraldOre => None,
            Item::GreenShulkerBox => None,
            Item::YellowGlazedTerracotta => None,
            Item::GlowBerries => None,
            Item::WaxedOxidizedCutCopperStairs => None,
            Item::OakStairs => None,
            Item::MagentaStainedGlassPane => None,
            Item::ChainmailChestplate => Some(240),
            Item::RedStainedGlass => None,
            Item::DiamondHoe => Some(1561),
            Item::WaxedWeatheredCutCopper => None,
            Item::Paper => None,
            Item::GoldenChestplate => Some(112),
            Item::BlueTerracotta => None,
            Item::OcelotSpawnEgg => None,
            Item::LightBlueShulkerBox => None,
            Item::DiamondOre => None,
            Item::FloweringAzalea => None,
            Item::PrismarineBrickStairs => None,
            Item::CrimsonSign => None,
            Item::LimeGlazedTerracotta => None,
            Item::BlackGlazedTerracotta => None,
            Item::BoneMeal => None,
            Item::BirchFence => None,
            Item::CobblestoneWall => None,
            Item::GraniteWall => None,
            Item::Cake => None,
            Item::SmoothBasalt => None,
            Item::NetheriteIngot => None,
            Item::PumpkinSeeds => None,
            Item::Redstone => None,
            Item::WaxedCutCopperSlab => None,
            Item::DarkOakButton => None,
            Item::MossCarpet => None,
            Item::GreenGlazedTerracotta => None,
            Item::BakedPotato => None,
            Item::OxidizedCutCopper => None,
            Item::CrimsonButton => None,
            Item::Painting => None,
            Item::SpruceLeaves => None,
            Item::SuspiciousStew => None,
            Item::GrayWool => None,
            Item::CoalBlock => None,
            Item::CartographyTable => None,
            Item::WarpedFungusOnAStick => Some(100),
            Item::StrippedCrimsonHyphae => None,
            Item::IronSword => Some(250),
            Item::SoulCampfire => None,
            Item::AcaciaBoat => None,
            Item::SmithingTable => None,
            Item::SpruceStairs => None,
            Item::PinkConcretePowder => None,
            Item::PurpurSlab => None,
            Item::Spyglass => None,
            Item::Stonecutter => None,
            Item::RedSandstoneWall => None,
            Item::Bamboo => None,
            Item::WhiteBed => None,
            Item::NetherBrickSlab => None,
            Item::BlackConcretePowder => None,
            Item::MusicDisc13 => None,
            Item::OrangeWool => None,
            Item::GoldenHorseArmor => None,
            Item::BlueGlazedTerracotta => None,
            Item::RedNetherBrickSlab => None,
            Item::Lodestone => None,
            Item::GrayCarpet => None,
            Item::CyanShulkerBox => None,
            Item::TripwireHook => None,
            Item::Seagrass => None,
            Item::BirchLog => None,
            Item::IronHoe => Some(250),
            Item::YellowCarpet => None,
            Item::PolishedGraniteSlab => None,
            Item::Bow => Some(384),
            Item::JungleDoor => None,
            Item::PinkCandle => None,
            Item::CutRedSandstoneSlab => None,
            Item::CrimsonPlanks => None,
            Item::Azalea => None,
            Item::StructureBlock => None,
            Item::RedDye => None,
            Item::SculkSensor => None,
            Item::Allium => None,
            Item::DrownedSpawnEgg => None,
            Item::SquidSpawnEgg => None,
            Item::GoldenHoe => Some(32),
            Item::TraderLlamaSpawnEgg => None,
            Item::BlackBanner => None,
            Item::LilyPad => None,
            Item::Smoker => None,
            Item::YellowCandle => None,
            Item::DeepslateTileWall => None,
            Item::DeadTubeCoral => None,
            Item::StoneBricks => None,
            Item::GrayTerracotta => None,
            Item::Clay => None,
            Item::SugarCane => None,
            Item::YellowStainedGlassPane => None,
            Item::Honeycomb => None,
            Item::ZombieHead => None,
            Item::AzaleaLeaves => None,
            Item::Kelp => None,
            Item::MagentaGlazedTerracotta => None,
            Item::GuardianSpawnEgg => None,
            Item::LightBlueBed => None,
            Item::CyanConcretePowder => None,
            Item::BigDripleaf => None,
            Item::TrappedChest => None,
            Item::Carrot => None,
            Item::CookedChicken => None,
            Item::Dandelion => None,
            Item::NetheriteSword => Some(2031),
            Item::Repeater => None,
            Item::JunglePressurePlate => None,
            Item::HornCoralFan => None,
            Item::SheepSpawnEgg => None,
            Item::GlassPane => None,
            Item::LimeBed => None,
            Item::ShulkerSpawnEgg => None,
            Item::MagentaBanner => None,
            Item::PrismarineCrystals => None,
            Item::DioriteWall => None,
            Item::WoodenShovel => Some(59),
            Item::WoodenAxe => Some(59),
            Item::DiamondBlock => None,
            Item::LapisLazuli => None,
            Item::Hopper => None,
            Item::BoneBlock => None,
            Item::NetherBrick => None,
            Item::FlowerPot => None,
            Item::GrayCandle => None,
            Item::Light => None,
            Item::Poppy => None,
            Item::ZombieSpawnEgg => None,
            Item::TubeCoralFan => None,
            Item::BirchPlanks => None,
            Item::HangingRoots => None,
            Item::OxidizedCutCopperSlab => None,
            Item::LeatherHelmet => Some(55),
            Item::GhastTear => None,
            Item::MagentaStainedGlass => None,
            Item::Bread => None,
            Item::PurpurPillar => None,
            Item::DeadTubeCoralFan => None,
            Item::DaylightDetector => None,
            Item::PoppedChorusFruit => None,
            Item::NoteBlock => None,
            Item::NetheriteHelmet => Some(407),
            Item::DarkOakPlanks => None,
            Item::MagentaWool => None,
            Item::WhiteGlazedTerracotta => None,
            Item::DiamondHelmet => Some(363),
            Item::BirchSign => None,
            Item::Bucket => None,
            Item::PurpleBed => None,
            Item::Piston => None,
            Item::BirchPressurePlate => None,
            Item::FletchingTable => None,
            Item::PufferfishBucket => None,
            Item::ChainCommandBlock => None,
            Item::PigSpawnEgg => None,
            Item::GreenCarpet => None,
            Item::StrippedCrimsonStem => None,
            Item::BlueConcretePowder => None,
            Item::MusicDiscChirp => None,
            Item::MusicDiscMellohi => None,
            Item::PurpleDye => None,
            Item::PolishedDeepslateWall => None,
            Item::MusicDiscBlocks => None,
            Item::BirchWood => None,
            Item::OakBoat => None,
            Item::MusicDiscPigstep => None,
            Item::TntMinecart => None,
            Item::Melon => None,
            Item::AcaciaButton => None,
            Item::MagentaConcretePowder => None,
            Item::MagentaConcrete => None,
            Item::OrangeGlazedTerracotta => None,
            Item::YellowWool => None,
            Item::BlazeSpawnEgg => None,
            Item::PiglinBruteSpawnEgg => None,
            Item::PolishedBlackstonePressurePlate => None,
            Item::BlackDye => None,
            Item::JungleWood => None,
            Item::AndesiteStairs => None,
            Item::Bookshelf => None,
            Item::RedTulip => None,
            Item::Elytra => Some(432),
            Item::MusicDiscOtherside => None,
            Item::Potato => None,
            Item::IronDoor => None,
            Item::WaxedExposedCutCopper => None,
            Item::StrippedAcaciaWood => None,
            Item::YellowDye => None,
            Item::ClayBall => None,
            Item::Sandstone => None,
            Item::CobblestoneSlab => None,
            Item::CobbledDeepslateWall => None,
            Item::CrimsonFenceGate => None,
            Item::CreeperSpawnEgg => None,
            Item::GhastSpawnEgg => None,
            Item::ItemFrame => None,
            Item::Composter => None,
            Item::Minecart => None,
            Item::CookedRabbit => None,
            Item::IronHorseArmor => None,
            Item::SandstoneStairs => None,
            Item::Diorite => None,
            Item::WarpedFence => None,
            Item::DarkOakFence => None,
            Item::Porkchop => None,
            Item::LightBlueCarpet => None,
            Item::LightGrayShulkerBox => None,
            Item::PurpleConcretePowder => None,
            Item::PolishedBlackstoneBrickWall => None,
            Item::ExperienceBottle => None,
            Item::PoisonousPotato => None,
            Item::WoodenPickaxe => Some(59),
            Item::StonePickaxe => Some(131),
            Item::Spawner => None,
            Item::MusicDiscWait => None,
            Item::DragonEgg => None,
            Item::QuartzStairs => None,
            Item::GoldOre => None,
            Item::RedNetherBrickWall => None,
            Item::GrayStainedGlass => None,
            Item::MossyCobblestoneStairs => None,
            Item::FoxSpawnEgg => None,
            Item::PolishedDeepslate => None,
            Item::AmethystBlock => None,
            Item::ElderGuardianSpawnEgg => None,
            Item::PlayerHead => None,
            Item::QuartzSlab => None,
            Item::Diamond => None,
            Item::GrayDye => None,
            Item::CaveSpiderSpawnEgg => None,
            Item::NetherBrickStairs => None,
            Item::BlueIce => None,
            Item::WaxedCutCopper => None,
            Item::GlowLichen => None,
            Item::Fern => None,
            Item::BrownMushroomBlock => None,
            Item::IronIngot => None,
            Item::LightBlueBanner => None,
            Item::StrippedDarkOakWood => None,
            Item::PolishedAndesiteSlab => None,
            Item::GlisteringMelonSlice => None,
            Item::LapisOre => None,
            Item::ChickenSpawnEgg => None,
            Item::GlowInkSac => None,
            Item::Ice => None,
            Item::MossyCobblestoneSlab => None,
            Item::SalmonBucket => None,
            Item::GoldenApple => None,
            Item::StraySpawnEgg => None,
            Item::BrownCandle => None,
            Item::PiglinSpawnEgg => None,
            Item::ChestMinecart => None,
            Item::PolishedBlackstoneBricks => None,
            Item::SnowBlock => None,
            Item::GlowSquidSpawnEgg => None,
            Item::JungleStairs => None,
            Item::PandaSpawnEgg => None,
            Item::HeavyWeightedPressurePlate => None,
            Item::AcaciaLog => None,
            Item::BlueOrchid => None,
            Item::CookedCod => None,
            Item::NetherSprouts => None,
            Item::RespawnAnchor => None,
            Item::DeadHornCoral => None,
            Item::WarpedRoots => None,
            Item::DiamondLeggings => Some(495),
            Item::FlintAndSteel => Some(64),
            Item::LavaBucket => None,
            Item::WeatheredCutCopper => None,
            Item::LightBlueCandle => None,
            Item::TintedGlass => None,
            Item::PolishedAndesite => None,
            Item::Arrow => None,
            Item::WhiteDye => None,
            Item::NautilusShell => None,
            Item::SmoothSandstoneSlab => None,
            Item::PolishedBlackstoneStairs => None,
            Item::PolishedGraniteStairs => None,
            Item::ArmorStand => None,
            Item::OakFenceGate => None,
            Item::GlobeBannerPattern => None,
            Item::PolishedDiorite => None,
            Item::AcaciaTrapdoor => None,
            Item::InfestedChiseledStoneBricks => None,
            Item::JungleBoat => None,
            Item::SoulTorch => None,
            Item::WanderingTraderSpawnEgg => None,
            Item::LightBlueGlazedTerracotta => None,
            Item::MagentaDye => None,
            Item::PumpkinPie => None,
            Item::CrackedDeepslateTiles => None,
            Item::RedGlazedTerracotta => None,
            Item::RawCopperBlock => None,
            Item::OxidizedCopper => None,
            Item::WoodenSword => Some(59),
            Item::WritableBook => None,
            Item::Deepslate => None,
            Item::PrismarineSlab => None,
            Item::HoneycombBlock => None,
            Item::FireCharge => None,
            Item::CarvedPumpkin => None,
            Item::DeadBush => None,
            Item::LightGrayBed => None,
            Item::LightGrayTerracotta => None,
            Item::WhiteShulkerBox => None,
            Item::DarkPrismarineStairs => None,
            Item::Vine => None,
            Item::OakFence => None,
            Item::WhiteBanner => None,
            Item::WarpedSlab => None,
            Item::CrackedStoneBricks => None,
            Item::WeatheredCutCopperStairs => None,
            Item::Prismarine => None,
            Item::Observer => None,
            Item::MusicDiscStal => None,
            Item::PolishedDioriteStairs => None,
            Item::YellowBed => None,
            Item::Stone => None,
            Item::Netherrack => None,
            Item::DetectorRail => None,
            Item::WarpedHyphae => None,
            Item::SkeletonSkull => None,
            Item::PolishedDeepslateSlab => None,
            Item::Chain => None,
            Item::GoldenAxe => Some(32),
            Item::WarpedPressurePlate => None,
            Item::PointedDripstone => None,
            Item::RedNetherBrickStairs => None,
            Item::TropicalFish => None,
            Item::CyanBanner => None,
            Item::Crossbow => Some(326),
            Item::Sunflower => None,
            Item::CommandBlockMinecart => None,
            Item::IronShovel => Some(250),
            Item::DiamondBoots => Some(429),
            Item::PurpleShulkerBox => None,
            Item::BlackstoneSlab => None,
            Item::PinkShulkerBox => None,
            Item::PurpleConcrete => None,
            Item::Lilac => None,
            Item::JungleLog => None,
            Item::SmoothQuartz => None,
            Item::Lead => None,
            Item::MagentaTerracotta => None,
            Item::PolishedBlackstoneBrickStairs => None,
            Item::WitherSkeletonSkull => None,
            Item::DeepslateTileStairs => None,
            Item::WhiteStainedGlass => None,
            Item::DeepslateBrickSlab => None,
            Item::JungleTrapdoor => None,
            Item::EnderEye => None,
            Item::SoulLantern => None,
            Item::EmeraldOre => None,
            Item::CookedSalmon => None,
            Item::DarkPrismarineSlab => None,
            Item::AxolotlSpawnEgg => None,
            Item::CrimsonRoots => None,
            Item::Glowstone => None,
            Item::WhiteWool => None,
            Item::GildedBlackstone => None,
            Item::CyanStainedGlass => None,
            Item::StriderSpawnEgg => None,
            Item::StoneSlab => None,
            Item::BeeSpawnEgg => None,
            Item::DeadBrainCoralBlock => None,
            Item::IronOre => None,
            Item::WhiteCandle => None,
            Item::BuddingAmethyst => None,
            Item::LimeDye => None,
            Item::NetheriteBoots => Some(481),
            Item::DragonBreath => None,
            Item::ExposedCutCopper => None,
            Item::SoulSoil => None,
            Item::AcaciaSlab => None,
            Item::DeadBrainCoral => None,
            Item::PolishedBasalt => None,
            Item::DeadBubbleCoralFan => None,
            Item::IronTrapdoor => None,
            Item::LimeCandle => None,
            Item::WaxedWeatheredCutCopperStairs => None,
            Item::BrewingStand => None,
            Item::SilverfishSpawnEgg => None,
            Item::BlackCarpet => None,
            Item::BlackConcrete => None,
            Item::FireworkRocket => None,
            Item::Anvil => None,
            Item::NetheriteAxe => Some(2031),
            Item::Rail => None,
            Item::StrippedAcaciaLog => None,
            Item::RedstoneBlock => None,
            Item::LightningRod => None,
            Item::GreenConcrete => None,
            Item::Potion => None,
            Item::DonkeySpawnEgg => None,
            Item::Rabbit => None,
            Item::OrangeTerracotta => None,
            Item::AcaciaPressurePlate => None,
            Item::AcaciaPlanks => None,
            Item::TropicalFishSpawnEgg => None,
            Item::Bundle => None,
            Item::EnchantedGoldenApple => None,
            Item::TippedArrow => None,
            Item::CoarseDirt => None,
            Item::DarkOakSapling => None,
            Item::YellowShulkerBox => None,
            Item::DeepslateCoalOre => None,
            Item::WhiteConcretePowder => None,
            Item::BubbleCoralFan => None,
            Item::GoldenBoots => Some(91),
            Item::SkeletonSpawnEgg => None,
            Item::EndermiteSpawnEgg => None,
            Item::DeepslateLapisOre => None,
            Item::Cobblestone => None,
            Item::ZoglinSpawnEgg => None,
            Item::MusicDiscMall => None,
            Item::BlazeRod => None,
            Item::SmoothRedSandstoneStairs => None,
            Item::OrangeCandle => None,
            Item::MossyStoneBrickWall => None,
            Item::Granite => None,
            Item::PowderSnowBucket => None,
            Item::HeartOfTheSea => None,
            Item::LightGrayStainedGlass => None,
            Item::StrippedWarpedHyphae => None,
            Item::LightBlueStainedGlass => None,
            Item::SmoothQuartzStairs => None,
            Item::Furnace => None,
            Item::SpruceSapling => None,
            Item::WarpedFenceGate => None,
            Item::StrippedBirchWood => None,
            Item::Flint => None,
            Item::MagentaCarpet => None,
            Item::SprucePressurePlate => None,
            Item::Farmland => None,
            Item::PurpleStainedGlassPane => None,
            Item::SpruceSlab => None,
            Item::TwistingVines => None,
            Item::SpruceTrapdoor => None,
            Item::CrimsonSlab => None,
            Item::RawIron => None,
            Item::EnderPearl => None,
            Item::VillagerSpawnEgg => None,
            Item::MelonSlice => None,
            Item::GrayStainedGlassPane => None,
            Item::NetherStar => None,
            Item::EndermanSpawnEgg => None,
            Item::FermentedSpiderEye => None,
            Item::OakTrapdoor => None,
            Item::LimeConcretePowder => None,
            Item::PurpleCandle => None,
            Item::CutCopper => None,
            Item::StoneBrickSlab => None,
            Item::ChiseledRedSandstone => None,
            Item::InfestedDeepslate => None,
            Item::MuleSpawnEgg => None,
            Item::InkSac => None,
            Item::CommandBlock => None,
            Item::Shield => Some(336),
            Item::HornCoralBlock => None,
            Item::OxeyeDaisy => None,
            Item::Stick => None,
            Item::Cobweb => None,
            Item::BlazePowder => None,
            Item::DeadHornCoralFan => None,
            Item::Wheat => None,
            Item::OrangeDye => None,
            Item::CrimsonFungus => None,
            Item::ShulkerBox => None,
            Item::OakLeaves => None,
            Item::NetheriteHoe => Some(2031),
            Item::BrickStairs => None,
            Item::TurtleSpawnEgg => None,
            Item::WaxedExposedCutCopperSlab => None,
            Item::EndStone => None,
            Item::DamagedAnvil => None,
            Item::GoldBlock => None,
            Item::FlowerBannerPattern => None,
            Item::Lectern => None,
            Item::WarpedStairs => None,
            Item::StrippedWarpedStem => None,
            Item::LimeBanner => None,
            Item::DarkOakPressurePlate => None,
            Item::CobbledDeepslate => None,
            Item::Obsidian => None,
            Item::WolfSpawnEgg => None,
            Item::RedBanner => None,
            Item::CookedBeef => None,
            Item::StickyPiston => None,
            Item::DeepslateIronOre => None,
            Item::HuskSpawnEgg => None,
            Item::Beetroot => None,
            Item::GreenBanner => None,
            Item::RabbitHide => None,
            Item::OakButton => None,
            Item::BrownBanner => None,
            Item::BlueWool => None,
            Item::LightBlueStainedGlassPane => None,
            Item::EndStoneBrickStairs => None,
            Item::LingeringPotion => None,
            Item::SmoothQuartzSlab => None,
            Item::IronAxe => Some(250),
            Item::RedBed => None,
            Item::PolishedBlackstoneBrickSlab => None,
            Item::CopperIngot => None,
            Item::LeatherLeggings => Some(75),
            Item::ChiseledNetherBricks => None,
            Item::CryingObsidian => None,
            Item::WaxedCutCopperStairs => None,
            Item::OxidizedCutCopperStairs => None,
            Item::RedSand => None,
            Item::OrangeBanner => None,
            Item::Gravel => None,
            Item::StrippedJungleWood => None,
            Item::StructureVoid => None,
            Item::RabbitStew => None,
            Item::GrayBanner => None,
            Item::CrimsonHyphae => None,
            Item::GrayGlazedTerracotta => None,
            Item::BeetrootSoup => None,
            Item::TurtleEgg => None,
            Item::Bone => None,
            Item::SmallDripleaf => None,
            Item::ZombieHorseSpawnEgg => None,
            Item::GreenCandle => None,
            Item::RedstoneOre => None,
            Item::Chest => None,
            Item::DarkOakStairs => None,
            Item::Bedrock => None,
            Item::Gunpowder => None,
            Item::Brick => None,
            Item::CraftingTable => None,
            Item::QuartzPillar => None,
            Item::TubeCoral => None,
            Item::LightGrayWool => None,
            Item::SpruceWood => None,
            Item::SlimeSpawnEgg => None,
            Item::Clock => None,
            Item::CrackedNetherBricks => None,
            Item::RedMushroomBlock => None,
            Item::QuartzBlock => None,
            Item::LightGrayCandle => None,
            Item::BlackWool => None,
            Item::PrismarineShard => None,
            Item::CyanConcrete => None,
            Item::Torch => None,
            Item::Podzol => None,
            Item::RedShulkerBox => None,
            Item::WitchSpawnEgg => None,
            Item::LimeCarpet => None,
            Item::GreenBed => None,
            Item::Leather => None,
            Item::CyanCandle => None,
            Item::NetherGoldOre => None,
            Item::WarpedSign => None,
            Item::MossyCobblestoneWall => None,
            Item::SmoothSandstoneStairs => None,
            Item::SpruceDoor => None,
            Item::NetherBrickFence => None,
            Item::LapisBlock => None,
            Item::IronPickaxe => Some(250),
            Item::PinkTerracotta => None,
            Item::MusicDiscWard => None,
            Item::CyanBed => None,
            Item::Blackstone => None,
            Item::RottenFlesh => None,
            Item::OrangeCarpet => None,
            Item::ChorusFruit => None,
            Item::Egg => None,
            Item::MusicDiscCat => None,
            Item::BrownConcrete => None,
            Item::PurpleTerracotta => None,
            Item::Andesite => None,
            Item::Sand => None,
            Item::Beacon => None,
            Item::PrismarineStairs => None,
            Item::SeaLantern => None,
            Item::OrangeShulkerBox => None,
            Item::WoodenHoe => Some(59),
            Item::LightBlueDye => None,
            Item::Conduit => None,
            Item::MilkBucket => None,
            Item::CowSpawnEgg => None,
            Item::NetherBricks => None,
            Item::ActivatorRail => None,
            Item::DarkOakDoor => None,
            Item::BrownConcretePowder => None,
            Item::MagentaShulkerBox => None,
            Item::DeadBubbleCoralBlock => None,
            Item::GraniteStairs => None,
            Item::MossyStoneBrickSlab => None,
            Item::PillagerSpawnEgg => None,
            Item::DarkOakBoat => None,
            Item::AndesiteSlab => None,
            Item::Cornflower => None,
            Item::LightGrayConcrete => None,
            Item::FireworkStar => None,
            Item::Grindstone => None,
            Item::Target => None,
            Item::Glass => None,
            Item::ChorusPlant => None,
            Item::BrainCoral => None,
            Item::WaxedExposedCopper => None,
            Item::Lever => None,
            Item::Apple => None,
            Item::Trident => Some(250),
            Item::PinkBed => None,
            Item::BlastFurnace => None,
            Item::WaxedExposedCutCopperStairs => None,
            Item::ChiseledQuartzBlock => None,
            Item::BlueBed => None,
            Item::PurpleWool => None,
            Item::Basalt => None,
            Item::WarpedButton => None,
            Item::PurpleGlazedTerracotta => None,
            Item::CocoaBeans => None,
            Item::Shears => Some(238),
            Item::PolishedDioriteSlab => None,
            Item::Pufferfish => None,
            Item::MossyCobblestone => None,
            Item::SmallAmethystBud => None,
            Item::NetherBrickWall => None,
            Item::CrimsonFence => None,
            Item::NameTag => None,
            Item::CyanGlazedTerracotta => None,
            Item::JungleLeaves => None,
            Item::GrayConcrete => None,
            Item::OakSlab => None,
            Item::TubeCoralBlock => None,
            Item::Feather => None,
            Item::WarpedWartBlock => None,
            Item::EnderChest => None,
            Item::PetrifiedOakSlab => None,
            Item::String => None,
            Item::SpruceButton => None,
            Item::SpruceLog => None,
            Item::InfestedMossyStoneBricks => None,
            Item::LimeConcrete => None,
            Item::GlowstoneDust => None,
            Item::LightGrayGlazedTerracotta => None,
            Item::DriedKelp => None,
            Item::CrackedPolishedBlackstoneBricks => None,
            Item::GreenTerracotta => None,
            Item::WitherRose => None,
            Item::WhiteStainedGlassPane => None,
            Item::Book => None,
            Item::MossBlock => None,
            Item::LimeTerracotta => None,
            Item::CyanCarpet => None,
            Item::OrangeConcrete => None,
            Item::StrippedJungleLog => None,
            Item::BirchSlab => None,
            Item::FurnaceMinecart => None,
            Item::PinkConcrete => None,
            Item::StrippedDarkOakLog => None,
            Item::CrimsonDoor => None,
            Item::OakLog => None,
            Item::AzureBluet => None,
            Item::JungleSlab => None,
            Item::BrownWool => None,
            Item::ZombifiedPiglinSpawnEgg => None,
            Item::Dropper => None,
            Item::OakPressurePlate => None,
            Item::MushroomStew => None,
            Item::Sponge => None,
            Item::ChiseledDeepslate => None,
            Item::RavagerSpawnEgg => None,
            Item::VexSpawnEgg => None,
            Item::BirchTrapdoor => None,
            Item::CyanTerracotta => None,
            Item::Campfire => None,
            Item::RawGoldBlock => None,
            Item::BlackStainedGlassPane => None,
            Item::Beef => None,
            Item::MojangBannerPattern => None,
            Item::SpruceFence => None,
            Item::AcaciaFenceGate => None,
            Item::HorseSpawnEgg => None,
            Item::Dirt => None,
            Item::WarpedStem => None,
            Item::DarkOakLeaves => None,
            Item::Snowball => None,
            Item::Cauldron => None,
            Item::RedWool => None,
            Item::RedSandstoneStairs => None,
            Item::IronBoots => Some(195),
            Item::MediumAmethystBud => None,
            Item::Ladder => None,
            Item::DarkOakTrapdoor => None,
            Item::GreenWool => None,
            Item::BlueCandle => None,
        }
    }
}
impl Item {
    #[doc = "Returns the `fixed_with` property of this `Item`."]
    #[inline]
    pub fn fixed_with(&self) -> Vec<&str> {
        match self {
            Item::GuardianSpawnEgg => {
                vec![]
            }
            Item::GreenBed => {
                vec![]
            }
            Item::IronBars => {
                vec![]
            }
            Item::LilyOfTheValley => {
                vec![]
            }
            Item::GlassPane => {
                vec![]
            }
            Item::DarkOakButton => {
                vec![]
            }
            Item::ZoglinSpawnEgg => {
                vec![]
            }
            Item::GoatSpawnEgg => {
                vec![]
            }
            Item::RedstoneOre => {
                vec![]
            }
            Item::OxidizedCutCopper => {
                vec![]
            }
            Item::Kelp => {
                vec![]
            }
            Item::GrayTerracotta => {
                vec![]
            }
            Item::DeadFireCoral => {
                vec![]
            }
            Item::MushroomStew => {
                vec![]
            }
            Item::CyanConcretePowder => {
                vec![]
            }
            Item::TrappedChest => {
                vec![]
            }
            Item::TintedGlass => {
                vec![]
            }
            Item::Gunpowder => {
                vec![]
            }
            Item::WaxedCutCopper => {
                vec![]
            }
            Item::Allium => {
                vec![]
            }
            Item::DeepslateBrickSlab => {
                vec![]
            }
            Item::Scute => {
                vec![]
            }
            Item::Chain => {
                vec![]
            }
            Item::CodBucket => {
                vec![]
            }
            Item::EndRod => {
                vec![]
            }
            Item::IronDoor => {
                vec![]
            }
            Item::DetectorRail => {
                vec![]
            }
            Item::BlazeSpawnEgg => {
                vec![]
            }
            Item::GoldenSword => {
                vec![]
            }
            Item::Saddle => {
                vec![]
            }
            Item::LeatherHelmet => {
                vec![]
            }
            Item::Lead => {
                vec![]
            }
            Item::DeadBrainCoralBlock => {
                vec![]
            }
            Item::Bell => {
                vec![]
            }
            Item::Map => {
                vec![]
            }
            Item::MusicDiscWard => {
                vec![]
            }
            Item::Bamboo => {
                vec![]
            }
            Item::CobblestoneWall => {
                vec![]
            }
            Item::Painting => {
                vec![]
            }
            Item::RedSandstone => {
                vec![]
            }
            Item::CyanBed => {
                vec![]
            }
            Item::DeadHornCoralFan => {
                vec![]
            }
            Item::BrownConcretePowder => {
                vec![]
            }
            Item::NetheriteLeggings => {
                vec![]
            }
            Item::BeetrootSoup => {
                vec![]
            }
            Item::InfestedStoneBricks => {
                vec![]
            }
            Item::GoldenShovel => {
                vec![]
            }
            Item::BuddingAmethyst => {
                vec![]
            }
            Item::LightBlueStainedGlass => {
                vec![]
            }
            Item::EmeraldBlock => {
                vec![]
            }
            Item::CrimsonButton => {
                vec![]
            }
            Item::MusicDisc13 => {
                vec![]
            }
            Item::Barrel => {
                vec![]
            }
            Item::PolishedGraniteStairs => {
                vec![]
            }
            Item::OakStairs => {
                vec![]
            }
            Item::Shield => {
                vec![]
            }
            Item::GrayShulkerBox => {
                vec![]
            }
            Item::PolishedAndesiteStairs => {
                vec![]
            }
            Item::LightGrayWool => {
                vec![]
            }
            Item::Vine => {
                vec![]
            }
            Item::InfestedDeepslate => {
                vec![]
            }
            Item::TropicalFishBucket => {
                vec![]
            }
            Item::SmoothRedSandstoneStairs => {
                vec![]
            }
            Item::StoneHoe => {
                vec![]
            }
            Item::YellowCarpet => {
                vec![]
            }
            Item::BrownMushroom => {
                vec![]
            }
            Item::IronHelmet => {
                vec![]
            }
            Item::LightGrayCarpet => {
                vec![]
            }
            Item::Apple => {
                vec![]
            }
            Item::Wheat => {
                vec![]
            }
            Item::SalmonSpawnEgg => {
                vec![]
            }
            Item::PurpleCandle => {
                vec![]
            }
            Item::StoneBrickStairs => {
                vec![]
            }
            Item::AcaciaFence => {
                vec![]
            }
            Item::Snow => {
                vec![]
            }
            Item::CookedSalmon => {
                vec![]
            }
            Item::AcaciaPlanks => {
                vec![]
            }
            Item::CoalOre => {
                vec![]
            }
            Item::StrippedCrimsonHyphae => {
                vec![]
            }
            Item::CutCopperSlab => {
                vec![]
            }
            Item::LightBlueGlazedTerracotta => {
                vec![]
            }
            Item::GrayCandle => {
                vec![]
            }
            Item::SoulLantern => {
                vec![]
            }
            Item::CrimsonFungus => {
                vec![]
            }
            Item::BlueConcrete => {
                vec![]
            }
            Item::DarkPrismarineSlab => {
                vec![]
            }
            Item::CarrotOnAStick => {
                vec![]
            }
            Item::IronAxe => {
                vec![]
            }
            Item::CobbledDeepslateWall => {
                vec![]
            }
            Item::BlackConcretePowder => {
                vec![]
            }
            Item::LightBlueDye => {
                vec![]
            }
            Item::HopperMinecart => {
                vec![]
            }
            Item::LightGrayStainedGlass => {
                vec![]
            }
            Item::BrownStainedGlass => {
                vec![]
            }
            Item::ChickenSpawnEgg => {
                vec![]
            }
            Item::CyanDye => {
                vec![]
            }
            Item::PinkWool => {
                vec![]
            }
            Item::PinkBed => {
                vec![]
            }
            Item::OrangeShulkerBox => {
                vec![]
            }
            Item::Bucket => {
                vec![]
            }
            Item::Redstone => {
                vec![]
            }
            Item::WhiteCarpet => {
                vec![]
            }
            Item::BrownTerracotta => {
                vec![]
            }
            Item::BirchSign => {
                vec![]
            }
            Item::DarkOakStairs => {
                vec![]
            }
            Item::BirchPlanks => {
                vec![]
            }
            Item::ChainmailBoots => {
                vec![]
            }
            Item::RedstoneTorch => {
                vec![]
            }
            Item::Torch => {
                vec![]
            }
            Item::BlackstoneSlab => {
                vec![]
            }
            Item::WitchSpawnEgg => {
                vec![]
            }
            Item::MusicDiscMall => {
                vec![]
            }
            Item::EndermiteSpawnEgg => {
                vec![]
            }
            Item::Glowstone => {
                vec![]
            }
            Item::WanderingTraderSpawnEgg => {
                vec![]
            }
            Item::Deepslate => {
                vec![]
            }
            Item::WaxedWeatheredCutCopper => {
                vec![]
            }
            Item::AmethystShard => {
                vec![]
            }
            Item::LilyPad => {
                vec![]
            }
            Item::AcaciaDoor => {
                vec![]
            }
            Item::Observer => {
                vec![]
            }
            Item::DarkOakTrapdoor => {
                vec![]
            }
            Item::WrittenBook => {
                vec![]
            }
            Item::RawCopper => {
                vec![]
            }
            Item::Obsidian => {
                vec![]
            }
            Item::CobbledDeepslate => {
                vec![]
            }
            Item::WoodenAxe => {
                vec![]
            }
            Item::Compass => {
                vec![]
            }
            Item::SpruceSapling => {
                vec![]
            }
            Item::MusicDiscWait => {
                vec![]
            }
            Item::PetrifiedOakSlab => {
                vec![]
            }
            Item::PolishedBasalt => {
                vec![]
            }
            Item::CyanGlazedTerracotta => {
                vec![]
            }
            Item::LightBlueConcrete => {
                vec![]
            }
            Item::GoldNugget => {
                vec![]
            }
            Item::WarpedFungus => {
                vec![]
            }
            Item::CrackedStoneBricks => {
                vec![]
            }
            Item::PolishedBlackstoneSlab => {
                vec![]
            }
            Item::FireCoralBlock => {
                vec![]
            }
            Item::LightGrayConcretePowder => {
                vec![]
            }
            Item::QuartzPillar => {
                vec![]
            }
            Item::Beef => {
                vec![]
            }
            Item::ZombieHorseSpawnEgg => {
                vec![]
            }
            Item::IronPickaxe => {
                vec![]
            }
            Item::StrippedBirchLog => {
                vec![]
            }
            Item::BatSpawnEgg => {
                vec![]
            }
            Item::StoneShovel => {
                vec![]
            }
            Item::ChiseledQuartzBlock => {
                vec![]
            }
            Item::SugarCane => {
                vec![]
            }
            Item::DiamondLeggings => {
                vec![]
            }
            Item::Melon => {
                vec![]
            }
            Item::MagentaBed => {
                vec![]
            }
            Item::Stick => {
                vec![]
            }
            Item::VexSpawnEgg => {
                vec![]
            }
            Item::NetherStar => {
                vec![]
            }
            Item::Mycelium => {
                vec![]
            }
            Item::IronNugget => {
                vec![]
            }
            Item::PinkCandle => {
                vec![]
            }
            Item::MagentaBanner => {
                vec![]
            }
            Item::MossyCobblestoneStairs => {
                vec![]
            }
            Item::EnchantedGoldenApple => {
                vec![]
            }
            Item::SnowBlock => {
                vec![]
            }
            Item::FishingRod => {
                vec![]
            }
            Item::RedStainedGlass => {
                vec![]
            }
            Item::LimeStainedGlass => {
                vec![]
            }
            Item::BeeNest => {
                vec![]
            }
            Item::SlimeSpawnEgg => {
                vec![]
            }
            Item::DriedKelp => {
                vec![]
            }
            Item::CryingObsidian => {
                vec![]
            }
            Item::TubeCoral => {
                vec![]
            }
            Item::MelonSeeds => {
                vec![]
            }
            Item::TurtleEgg => {
                vec![]
            }
            Item::RawCopperBlock => {
                vec![]
            }
            Item::Netherrack => {
                vec![]
            }
            Item::PrismarineShard => {
                vec![]
            }
            Item::EndermanSpawnEgg => {
                vec![]
            }
            Item::CutCopper => {
                vec![]
            }
            Item::Charcoal => {
                vec![]
            }
            Item::Cactus => {
                vec![]
            }
            Item::DeadHornCoral => {
                vec![]
            }
            Item::OakSlab => {
                vec![]
            }
            Item::DonkeySpawnEgg => {
                vec![]
            }
            Item::LightWeightedPressurePlate => {
                vec![]
            }
            Item::FilledMap => {
                vec![]
            }
            Item::NautilusShell => {
                vec![]
            }
            Item::WarpedSlab => {
                vec![]
            }
            Item::ParrotSpawnEgg => {
                vec![]
            }
            Item::SquidSpawnEgg => {
                vec![]
            }
            Item::DeepslateRedstoneOre => {
                vec![]
            }
            Item::Bricks => {
                vec![]
            }
            Item::DarkOakSign => {
                vec![]
            }
            Item::CookedChicken => {
                vec![]
            }
            Item::StoneSword => {
                vec![]
            }
            Item::BlueWool => {
                vec![]
            }
            Item::DeadTubeCoralBlock => {
                vec![]
            }
            Item::RedBed => {
                vec![]
            }
            Item::RawIronBlock => {
                vec![]
            }
            Item::SmallAmethystBud => {
                vec![]
            }
            Item::CrimsonRoots => {
                vec![]
            }
            Item::PurpurBlock => {
                vec![]
            }
            Item::Pumpkin => {
                vec![]
            }
            Item::DeadFireCoralFan => {
                vec![]
            }
            Item::MusicDiscPigstep => {
                vec![]
            }
            Item::MusicDiscCat => {
                vec![]
            }
            Item::DeadFireCoralBlock => {
                vec![]
            }
            Item::CatSpawnEgg => {
                vec![]
            }
            Item::HoneyBottle => {
                vec![]
            }
            Item::DripstoneBlock => {
                vec![]
            }
            Item::DeadBubbleCoralFan => {
                vec![]
            }
            Item::CyanConcrete => {
                vec![]
            }
            Item::BirchStairs => {
                vec![]
            }
            Item::MusicDiscMellohi => {
                vec![]
            }
            Item::DarkOakFenceGate => {
                vec![]
            }
            Item::HorseSpawnEgg => {
                vec![]
            }
            Item::PolishedGranite => {
                vec![]
            }
            Item::FloweringAzaleaLeaves => {
                vec![]
            }
            Item::ExposedCutCopperStairs => {
                vec![]
            }
            Item::OrangeTulip => {
                vec![]
            }
            Item::BlueGlazedTerracotta => {
                vec![]
            }
            Item::JungleBoat => {
                vec![]
            }
            Item::StructureVoid => {
                vec![]
            }
            Item::BlueStainedGlass => {
                vec![]
            }
            Item::WitherRose => {
                vec![]
            }
            Item::BeetrootSeeds => {
                vec![]
            }
            Item::PigSpawnEgg => {
                vec![]
            }
            Item::OxeyeDaisy => {
                vec![]
            }
            Item::JungleFenceGate => {
                vec![]
            }
            Item::CookedPorkchop => {
                vec![]
            }
            Item::GoldenApple => {
                vec![]
            }
            Item::AndesiteSlab => {
                vec![]
            }
            Item::GreenConcrete => {
                vec![]
            }
            Item::StrippedJungleLog => {
                vec![]
            }
            Item::StrippedAcaciaWood => {
                vec![]
            }
            Item::OakLog => {
                vec![]
            }
            Item::WeepingVines => {
                vec![]
            }
            Item::LapisOre => {
                vec![]
            }
            Item::PoppedChorusFruit => {
                vec![]
            }
            Item::MossyCobblestoneWall => {
                vec![]
            }
            Item::GreenCandle => {
                vec![]
            }
            Item::Spyglass => {
                vec![]
            }
            Item::BrownGlazedTerracotta => {
                vec![]
            }
            Item::OakButton => {
                vec![]
            }
            Item::LightBlueConcretePowder => {
                vec![]
            }
            Item::Poppy => {
                vec![]
            }
            Item::BirchTrapdoor => {
                vec![]
            }
            Item::WaxedWeatheredCopper => {
                vec![]
            }
            Item::ZombieVillagerSpawnEgg => {
                vec![]
            }
            Item::WritableBook => {
                vec![]
            }
            Item::Cobblestone => {
                vec![]
            }
            Item::Anvil => {
                vec![]
            }
            Item::LimeStainedGlassPane => {
                vec![]
            }
            Item::EndStoneBrickSlab => {
                vec![]
            }
            Item::LightningRod => {
                vec![]
            }
            Item::SpruceLog => {
                vec![]
            }
            Item::JungleDoor => {
                vec![]
            }
            Item::BlackstoneWall => {
                vec![]
            }
            Item::FermentedSpiderEye => {
                vec![]
            }
            Item::WitherSkeletonSpawnEgg => {
                vec![]
            }
            Item::Rabbit => {
                vec![]
            }
            Item::PinkShulkerBox => {
                vec![]
            }
            Item::SmoothQuartzSlab => {
                vec![]
            }
            Item::PrismarineBricks => {
                vec![]
            }
            Item::Andesite => {
                vec![]
            }
            Item::DeepslateTileWall => {
                vec![]
            }
            Item::DiamondPickaxe => {
                vec![]
            }
            Item::GlowBerries => {
                vec![]
            }
            Item::ChiseledSandstone => {
                vec![]
            }
            Item::DeadHornCoralBlock => {
                vec![]
            }
            Item::GreenDye => {
                vec![]
            }
            Item::WaxedCutCopperSlab => {
                vec![]
            }
            Item::Coal => {
                vec![]
            }
            Item::LapisLazuli => {
                vec![]
            }
            Item::CutRedSandstone => {
                vec![]
            }
            Item::ChainCommandBlock => {
                vec![]
            }
            Item::QuartzBlock => {
                vec![]
            }
            Item::RedstoneBlock => {
                vec![]
            }
            Item::WolfSpawnEgg => {
                vec![]
            }
            Item::GrayDye => {
                vec![]
            }
            Item::LimeShulkerBox => {
                vec![]
            }
            Item::LimeBanner => {
                vec![]
            }
            Item::DarkOakFence => {
                vec![]
            }
            Item::Carrot => {
                vec![]
            }
            Item::RedConcrete => {
                vec![]
            }
            Item::FurnaceMinecart => {
                vec![]
            }
            Item::WaxedExposedCutCopper => {
                vec![]
            }
            Item::GraniteSlab => {
                vec![]
            }
            Item::Rail => {
                vec![]
            }
            Item::PhantomSpawnEgg => {
                vec![]
            }
            Item::HeavyWeightedPressurePlate => {
                vec![]
            }
            Item::Bowl => {
                vec![]
            }
            Item::BlueShulkerBox => {
                vec![]
            }
            Item::PrismarineCrystals => {
                vec![]
            }
            Item::EndCrystal => {
                vec![]
            }
            Item::BlackstoneStairs => {
                vec![]
            }
            Item::Sunflower => {
                vec![]
            }
            Item::StrippedOakWood => {
                vec![]
            }
            Item::SmallDripleaf => {
                vec![]
            }
            Item::ChiseledNetherBricks => {
                vec![]
            }
            Item::LightGrayGlazedTerracotta => {
                vec![]
            }
            Item::Dispenser => {
                vec![]
            }
            Item::CyanStainedGlassPane => {
                vec![]
            }
            Item::RedSandstoneStairs => {
                vec![]
            }
            Item::GrayBanner => {
                vec![]
            }
            Item::HoneycombBlock => {
                vec![]
            }
            Item::CrackedPolishedBlackstoneBricks => {
                vec![]
            }
            Item::GoldBlock => {
                vec![]
            }
            Item::PinkStainedGlassPane => {
                vec![]
            }
            Item::ChiseledRedSandstone => {
                vec![]
            }
            Item::CrimsonSlab => {
                vec![]
            }
            Item::SilverfishSpawnEgg => {
                vec![]
            }
            Item::GlisteringMelonSlice => {
                vec![]
            }
            Item::MusicDiscOtherside => {
                vec![]
            }
            Item::DeepslateIronOre => {
                vec![]
            }
            Item::WarpedFence => {
                vec![]
            }
            Item::LightGrayBed => {
                vec![]
            }
            Item::VillagerSpawnEgg => {
                vec![]
            }
            Item::FlintAndSteel => {
                vec![]
            }
            Item::NetherGoldOre => {
                vec![]
            }
            Item::SmoothRedSandstone => {
                vec![]
            }
            Item::WarpedNylium => {
                vec![]
            }
            Item::RabbitSpawnEgg => {
                vec![]
            }
            Item::BubbleCoralBlock => {
                vec![]
            }
            Item::EnchantedBook => {
                vec![]
            }
            Item::CopperIngot => {
                vec![]
            }
            Item::BlueBanner => {
                vec![]
            }
            Item::NetheriteIngot => {
                vec![]
            }
            Item::WarpedButton => {
                vec![]
            }
            Item::PurpleBanner => {
                vec![]
            }
            Item::CyanShulkerBox => {
                vec![]
            }
            Item::GrayWool => {
                vec![]
            }
            Item::SkeletonSkull => {
                vec![]
            }
            Item::ChorusPlant => {
                vec![]
            }
            Item::Honeycomb => {
                vec![]
            }
            Item::SpruceWood => {
                vec![]
            }
            Item::NetherBrickWall => {
                vec![]
            }
            Item::MusicDiscFar => {
                vec![]
            }
            Item::SlimeBall => {
                vec![]
            }
            Item::NetheriteAxe => {
                vec![]
            }
            Item::WheatSeeds => {
                vec![]
            }
            Item::SmoothStoneSlab => {
                vec![]
            }
            Item::CrimsonStem => {
                vec![]
            }
            Item::RedstoneLamp => {
                vec![]
            }
            Item::EvokerSpawnEgg => {
                vec![]
            }
            Item::GoldenHorseArmor => {
                vec![]
            }
            Item::OxidizedCutCopperSlab => {
                vec![]
            }
            Item::BlackWool => {
                vec![]
            }
            Item::PinkBanner => {
                vec![]
            }
            Item::DeadBrainCoral => {
                vec![]
            }
            Item::OakSign => {
                vec![]
            }
            Item::FlowerBannerPattern => {
                vec![]
            }
            Item::NetheriteScrap => {
                vec![]
            }
            Item::GlowLichen => {
                vec![]
            }
            Item::Cod => {
                vec![]
            }
            Item::Ice => {
                vec![]
            }
            Item::OrangeBanner => {
                vec![]
            }
            Item::PinkConcrete => {
                vec![]
            }
            Item::AcaciaSign => {
                vec![]
            }
            Item::DiamondShovel => {
                vec![]
            }
            Item::LightBlueCarpet => {
                vec![]
            }
            Item::PolishedBlackstoneStairs => {
                vec![]
            }
            Item::IronTrapdoor => {
                vec![]
            }
            Item::Prismarine => {
                vec![]
            }
            Item::DeepslateCopperOre => {
                vec![]
            }
            Item::GhastTear => {
                vec![]
            }
            Item::JungleWood => {
                vec![]
            }
            Item::WaxedWeatheredCutCopperSlab => {
                vec![]
            }
            Item::PolishedBlackstoneBrickWall => {
                vec![]
            }
            Item::ShulkerSpawnEgg => {
                vec![]
            }
            Item::String => {
                vec![]
            }
            Item::CartographyTable => {
                vec![]
            }
            Item::MediumAmethystBud => {
                vec![]
            }
            Item::IronShovel => {
                vec![]
            }
            Item::Egg => {
                vec![]
            }
            Item::RepeatingCommandBlock => {
                vec![]
            }
            Item::BrainCoralFan => {
                vec![]
            }
            Item::JungleButton => {
                vec![]
            }
            Item::Book => {
                vec![]
            }
            Item::BlueOrchid => {
                vec![]
            }
            Item::GoldenHelmet => {
                vec![]
            }
            Item::FoxSpawnEgg => {
                vec![]
            }
            Item::MelonSlice => {
                vec![]
            }
            Item::EndStone => {
                vec![]
            }
            Item::WeatheredCutCopper => {
                vec![]
            }
            Item::OrangeConcretePowder => {
                vec![]
            }
            Item::SprucePressurePlate => {
                vec![]
            }
            Item::LimeTerracotta => {
                vec![]
            }
            Item::YellowWool => {
                vec![]
            }
            Item::GreenStainedGlassPane => {
                vec![]
            }
            Item::OrangeTerracotta => {
                vec![]
            }
            Item::RedBanner => {
                vec![]
            }
            Item::SkullBannerPattern => {
                vec![]
            }
            Item::SoulSand => {
                vec![]
            }
            Item::SprucePlanks => {
                vec![]
            }
            Item::OxidizedCopper => {
                vec![]
            }
            Item::HangingRoots => {
                vec![]
            }
            Item::WhiteBed => {
                vec![]
            }
            Item::TraderLlamaSpawnEgg => {
                vec![]
            }
            Item::OakSapling => {
                vec![]
            }
            Item::CyanWool => {
                vec![]
            }
            Item::LingeringPotion => {
                vec![]
            }
            Item::BrickSlab => {
                vec![]
            }
            Item::MooshroomSpawnEgg => {
                vec![]
            }
            Item::GreenConcretePowder => {
                vec![]
            }
            Item::WoodenSword => {
                vec![]
            }
            Item::Lectern => {
                vec![]
            }
            Item::BoneBlock => {
                vec![]
            }
            Item::CobbledDeepslateSlab => {
                vec![]
            }
            Item::CocoaBeans => {
                vec![]
            }
            Item::BlackGlazedTerracotta => {
                vec![]
            }
            Item::CommandBlock => {
                vec![]
            }
            Item::ChainmailChestplate => {
                vec![]
            }
            Item::Diorite => {
                vec![]
            }
            Item::Smoker => {
                vec![]
            }
            Item::GreenStainedGlass => {
                vec![]
            }
            Item::ZombieHead => {
                vec![]
            }
            Item::CookedMutton => {
                vec![]
            }
            Item::GlowstoneDust => {
                vec![]
            }
            Item::ChiseledPolishedBlackstone => {
                vec![]
            }
            Item::SmithingTable => {
                vec![]
            }
            Item::DolphinSpawnEgg => {
                vec![]
            }
            Item::Dirt => {
                vec![]
            }
            Item::NetherBrickSlab => {
                vec![]
            }
            Item::NetheriteHelmet => {
                vec![]
            }
            Item::BirchFenceGate => {
                vec![]
            }
            Item::Loom => {
                vec![]
            }
            Item::RedShulkerBox => {
                vec![]
            }
            Item::DeepslateDiamondOre => {
                vec![]
            }
            Item::JungleLog => {
                vec![]
            }
            Item::OakLeaves => {
                vec![]
            }
            Item::SpruceLeaves => {
                vec![]
            }
            Item::WarpedRoots => {
                vec![]
            }
            Item::CraftingTable => {
                vec![]
            }
            Item::LightBlueStainedGlassPane => {
                vec![]
            }
            Item::LightBlueBed => {
                vec![]
            }
            Item::WaxedExposedCutCopperStairs => {
                vec![]
            }
            Item::WaxedExposedCutCopperSlab => {
                vec![]
            }
            Item::Podzol => {
                vec![]
            }
            Item::AcaciaLog => {
                vec![]
            }
            Item::QuartzStairs => {
                vec![]
            }
            Item::TubeCoralBlock => {
                vec![]
            }
            Item::DaylightDetector => {
                vec![]
            }
            Item::PowderSnowBucket => {
                vec![]
            }
            Item::AmethystCluster => {
                vec![]
            }
            Item::RawGoldBlock => {
                vec![]
            }
            Item::LightBlueWool => {
                vec![]
            }
            Item::CrimsonPressurePlate => {
                vec![]
            }
            Item::MagmaBlock => {
                vec![]
            }
            Item::TotemOfUndying => {
                vec![]
            }
            Item::CommandBlockMinecart => {
                vec![]
            }
            Item::DebugStick => {
                vec![]
            }
            Item::CaveSpiderSpawnEgg => {
                vec![]
            }
            Item::WarpedDoor => {
                vec![]
            }
            Item::NetherBrickFence => {
                vec![]
            }
            Item::DeadBubbleCoralBlock => {
                vec![]
            }
            Item::BlueIce => {
                vec![]
            }
            Item::DeadBrainCoralFan => {
                vec![]
            }
            Item::GoldenHoe => {
                vec![]
            }
            Item::DeadTubeCoral => {
                vec![]
            }
            Item::BeeSpawnEgg => {
                vec![]
            }
            Item::PurpleTerracotta => {
                vec![]
            }
            Item::BrownCarpet => {
                vec![]
            }
            Item::BrickStairs => {
                vec![]
            }
            Item::Barrier => {
                vec![]
            }
            Item::TippedArrow => {
                vec![]
            }
            Item::Bow => {
                vec![]
            }
            Item::GoldenPickaxe => {
                vec![]
            }
            Item::WarpedSign => {
                vec![]
            }
            Item::StrippedBirchWood => {
                vec![]
            }
            Item::DioriteStairs => {
                vec![]
            }
            Item::StrippedAcaciaLog => {
                vec![]
            }
            Item::BirchFence => {
                vec![]
            }
            Item::WarpedFenceGate => {
                vec![]
            }
            Item::InfestedMossyStoneBricks => {
                vec![]
            }
            Item::TntMinecart => {
                vec![]
            }
            Item::Elytra => {
                vec![]
            }
            Item::GoldenBoots => {
                vec![]
            }
            Item::RedNetherBrickSlab => {
                vec![]
            }
            Item::CyanBanner => {
                vec![]
            }
            Item::SpiderSpawnEgg => {
                vec![]
            }
            Item::WaxedWeatheredCutCopperStairs => {
                vec![]
            }
            Item::BubbleCoralFan => {
                vec![]
            }
            Item::Ladder => {
                vec![]
            }
            Item::CarvedPumpkin => {
                vec![]
            }
            Item::IronBoots => {
                vec![]
            }
            Item::ChiseledDeepslate => {
                vec![]
            }
            Item::BirchLeaves => {
                vec![]
            }
            Item::Porkchop => {
                vec![]
            }
            Item::RabbitHide => {
                vec![]
            }
            Item::IronHoe => {
                vec![]
            }
            Item::FireworkRocket => {
                vec![]
            }
            Item::LimeGlazedTerracotta => {
                vec![]
            }
            Item::BoneMeal => {
                vec![]
            }
            Item::DeepslateLapisOre => {
                vec![]
            }
            Item::LimeWool => {
                vec![]
            }
            Item::Bundle => {
                vec![]
            }
            Item::CutRedSandstoneSlab => {
                vec![]
            }
            Item::WeatheredCutCopperStairs => {
                vec![]
            }
            Item::MagentaConcretePowder => {
                vec![]
            }
            Item::BirchPressurePlate => {
                vec![]
            }
            Item::SkeletonSpawnEgg => {
                vec![]
            }
            Item::LargeAmethystBud => {
                vec![]
            }
            Item::GoldIngot => {
                vec![]
            }
            Item::WaxedOxidizedCutCopperSlab => {
                vec![]
            }
            Item::BirchLog => {
                vec![]
            }
            Item::ShulkerBox => {
                vec![]
            }
            Item::BrownStainedGlassPane => {
                vec![]
            }
            Item::BlueTerracotta => {
                vec![]
            }
            Item::GlowInkSac => {
                vec![]
            }
            Item::SmoothBasalt => {
                vec![]
            }
            Item::Repeater => {
                vec![]
            }
            Item::Leather => {
                vec![]
            }
            Item::SmoothQuartzStairs => {
                vec![]
            }
            Item::VindicatorSpawnEgg => {
                vec![]
            }
            Item::PoisonousPotato => {
                vec![]
            }
            Item::StrippedSpruceLog => {
                vec![]
            }
            Item::PrismarineBrickSlab => {
                vec![]
            }
            Item::DarkOakSapling => {
                vec![]
            }
            Item::DioriteWall => {
                vec![]
            }
            Item::MuleSpawnEgg => {
                vec![]
            }
            Item::RabbitFoot => {
                vec![]
            }
            Item::LightGrayBanner => {
                vec![]
            }
            Item::InfestedStone => {
                vec![]
            }
            Item::CutSandstone => {
                vec![]
            }
            Item::MagentaConcrete => {
                vec![]
            }
            Item::Tnt => {
                vec![]
            }
            Item::SandstoneWall => {
                vec![]
            }
            Item::PufferfishBucket => {
                vec![]
            }
            Item::WaxedCopperBlock => {
                vec![]
            }
            Item::Sponge => {
                vec![]
            }
            Item::EnchantingTable => {
                vec![]
            }
            Item::PinkTulip => {
                vec![]
            }
            Item::CreeperSpawnEgg => {
                vec![]
            }
            Item::SheepSpawnEgg => {
                vec![]
            }
            Item::JungleFence => {
                vec![]
            }
            Item::Calcite => {
                vec![]
            }
            Item::DragonEgg => {
                vec![]
            }
            Item::WaxedCutCopperStairs => {
                vec![]
            }
            Item::SoulCampfire => {
                vec![]
            }
            Item::LeatherHorseArmor => {
                vec![]
            }
            Item::YellowTerracotta => {
                vec![]
            }
            Item::TripwireHook => {
                vec![]
            }
            Item::Bedrock => {
                vec![]
            }
            Item::OrangeConcrete => {
                vec![]
            }
            Item::Lantern => {
                vec![]
            }
            Item::StoneBricks => {
                vec![]
            }
            Item::SmoothSandstone => {
                vec![]
            }
            Item::RoseBush => {
                vec![]
            }
            Item::HornCoral => {
                vec![]
            }
            Item::PurpleCarpet => {
                vec![]
            }
            Item::FlowerPot => {
                vec![]
            }
            Item::PolishedBlackstoneWall => {
                vec![]
            }
            Item::JackOLantern => {
                vec![]
            }
            Item::PointedDripstone => {
                vec![]
            }
            Item::StoneStairs => {
                vec![]
            }
            Item::LightGrayTerracotta => {
                vec![]
            }
            Item::StoneBrickWall => {
                vec![]
            }
            Item::GrayCarpet => {
                vec![]
            }
            Item::RootedDirt => {
                vec![]
            }
            Item::YellowConcretePowder => {
                vec![]
            }
            Item::Salmon => {
                vec![]
            }
            Item::StrippedDarkOakWood => {
                vec![]
            }
            Item::MagentaWool => {
                vec![]
            }
            Item::OakDoor => {
                vec![]
            }
            Item::Beetroot => {
                vec![]
            }
            Item::Scaffolding => {
                vec![]
            }
            Item::CrimsonPlanks => {
                vec![]
            }
            Item::AzureBluet => {
                vec![]
            }
            Item::RedTerracotta => {
                vec![]
            }
            Item::StraySpawnEgg => {
                vec![]
            }
            Item::Cookie => {
                vec![]
            }
            Item::DarkOakLeaves => {
                vec![]
            }
            Item::Lilac => {
                vec![]
            }
            Item::CyanCandle => {
                vec![]
            }
            Item::RedCandle => {
                vec![]
            }
            Item::MagentaCarpet => {
                vec![]
            }
            Item::PurpleConcretePowder => {
                vec![]
            }
            Item::StrippedJungleWood => {
                vec![]
            }
            Item::Fern => {
                vec![]
            }
            Item::FloweringAzalea => {
                vec![]
            }
            Item::CrimsonFenceGate => {
                vec![]
            }
            Item::DiamondChestplate => {
                vec![]
            }
            Item::MossyStoneBrickWall => {
                vec![]
            }
            Item::CyanCarpet => {
                vec![]
            }
            Item::WhiteTerracotta => {
                vec![]
            }
            Item::BlueDye => {
                vec![]
            }
            Item::Glass => {
                vec![]
            }
            Item::DamagedAnvil => {
                vec![]
            }
            Item::YellowBanner => {
                vec![]
            }
            Item::LeatherLeggings => {
                vec![]
            }
            Item::OakPlanks => {
                vec![]
            }
            Item::Gravel => {
                vec![]
            }
            Item::Clay => {
                vec![]
            }
            Item::AcaciaLeaves => {
                vec![]
            }
            Item::InfestedChiseledStoneBricks => {
                vec![]
            }
            Item::WhiteStainedGlass => {
                vec![]
            }
            Item::ExposedCutCopperSlab => {
                vec![]
            }
            Item::PrismarineSlab => {
                vec![]
            }
            Item::AndesiteStairs => {
                vec![]
            }
            Item::PinkDye => {
                vec![]
            }
            Item::LargeFern => {
                vec![]
            }
            Item::FireCharge => {
                vec![]
            }
            Item::DeepslateTileSlab => {
                vec![]
            }
            Item::PlayerHead => {
                vec![]
            }
            Item::DiamondHelmet => {
                vec![]
            }
            Item::CreeperBannerPattern => {
                vec![]
            }
            Item::BlastFurnace => {
                vec![]
            }
            Item::GreenGlazedTerracotta => {
                vec![]
            }
            Item::QuartzSlab => {
                vec![]
            }
            Item::LavaBucket => {
                vec![]
            }
            Item::WhiteCandle => {
                vec![]
            }
            Item::JunglePressurePlate => {
                vec![]
            }
            Item::Diamond => {
                vec![]
            }
            Item::MossBlock => {
                vec![]
            }
            Item::DarkOakBoat => {
                vec![]
            }
            Item::GlassBottle => {
                vec![]
            }
            Item::JungleSlab => {
                vec![]
            }
            Item::HornCoralBlock => {
                vec![]
            }
            Item::BlackConcrete => {
                vec![]
            }
            Item::LightBlueCandle => {
                vec![]
            }
            Item::CowSpawnEgg => {
                vec![]
            }
            Item::StonePickaxe => {
                vec![]
            }
            Item::StoneAxe => {
                vec![]
            }
            Item::GoldenAxe => {
                vec![]
            }
            Item::WhiteDye => {
                vec![]
            }
            Item::PillagerSpawnEgg => {
                vec![]
            }
            Item::PrismarineBrickStairs => {
                vec![]
            }
            Item::BrownMushroomBlock => {
                vec![]
            }
            Item::YellowStainedGlassPane => {
                vec![]
            }
            Item::SmoothRedSandstoneSlab => {
                vec![]
            }
            Item::EnderPearl => {
                vec![]
            }
            Item::StoneSlab => {
                vec![]
            }
            Item::FireworkStar => {
                vec![]
            }
            Item::AmethystBlock => {
                vec![]
            }
            Item::NetheriteBlock => {
                vec![]
            }
            Item::CrackedNetherBricks => {
                vec![]
            }
            Item::AcaciaTrapdoor => {
                vec![]
            }
            Item::NetherQuartzOre => {
                vec![]
            }
            Item::BlackBed => {
                vec![]
            }
            Item::CoarseDirt => {
                vec![]
            }
            Item::Dandelion => {
                vec![]
            }
            Item::DiamondSword => {
                vec![]
            }
            Item::PolarBearSpawnEgg => {
                vec![]
            }
            Item::KnowledgeBook => {
                vec![]
            }
            Item::MagmaCream => {
                vec![]
            }
            Item::RedCarpet => {
                vec![]
            }
            Item::ChainmailLeggings => {
                vec![]
            }
            Item::PurpleShulkerBox => {
                vec![]
            }
            Item::GoldenCarrot => {
                vec![]
            }
            Item::Trident => {
                vec![]
            }
            Item::StonePressurePlate => {
                vec![]
            }
            Item::WarpedStem => {
                vec![]
            }
            Item::SporeBlossom => {
                vec![]
            }
            Item::Jukebox => {
                vec![]
            }
            Item::WhiteBanner => {
                vec![]
            }
            Item::BakedPotato => {
                vec![]
            }
            Item::YellowCandle => {
                vec![]
            }
            Item::BubbleCoral => {
                vec![]
            }
            Item::BirchWood => {
                vec![]
            }
            Item::MusicDisc11 => {
                vec![]
            }
            Item::CookedCod => {
                vec![]
            }
            Item::DeepslateCoalOre => {
                vec![]
            }
            Item::MagentaShulkerBox => {
                vec![]
            }
            Item::Paper => {
                vec![]
            }
            Item::CoalBlock => {
                vec![]
            }
            Item::Seagrass => {
                vec![]
            }
            Item::Lever => {
                vec![]
            }
            Item::WoodenPickaxe => {
                vec![]
            }
            Item::Azalea => {
                vec![]
            }
            Item::PolishedAndesiteSlab => {
                vec![]
            }
            Item::StickyPiston => {
                vec![]
            }
            Item::NetheriteShovel => {
                vec![]
            }
            Item::WhiteWool => {
                vec![]
            }
            Item::LeatherBoots => {
                vec![]
            }
            Item::BlueStainedGlassPane => {
                vec![]
            }
            Item::PolishedBlackstone => {
                vec![]
            }
            Item::BlackDye => {
                vec![]
            }
            Item::BlueCandle => {
                vec![]
            }
            Item::MossyStoneBrickSlab => {
                vec![]
            }
            Item::AcaciaFenceGate => {
                vec![]
            }
            Item::Crossbow => {
                vec![]
            }
            Item::AndesiteWall => {
                vec![]
            }
            Item::PinkTerracotta => {
                vec![]
            }
            Item::GreenTerracotta => {
                vec![]
            }
            Item::OakFence => {
                vec![]
            }
            Item::MushroomStem => {
                vec![]
            }
            Item::OrangeGlazedTerracotta => {
                vec![]
            }
            Item::NetheriteSword => {
                vec![]
            }
            Item::JunglePlanks => {
                vec![]
            }
            Item::NetherBrick => {
                vec![]
            }
            Item::FireCoralFan => {
                vec![]
            }
            Item::BirchSlab => {
                vec![]
            }
            Item::GraniteStairs => {
                vec![]
            }
            Item::SpruceTrapdoor => {
                vec![]
            }
            Item::StrippedWarpedStem => {
                vec![]
            }
            Item::SlimeBlock => {
                vec![]
            }
            Item::Piston => {
                vec![]
            }
            Item::BrownConcrete => {
                vec![]
            }
            Item::Target => {
                vec![]
            }
            Item::MagentaGlazedTerracotta => {
                vec![]
            }
            Item::GraniteWall => {
                vec![]
            }
            Item::DiamondOre => {
                vec![]
            }
            Item::Cauldron => {
                vec![]
            }
            Item::DragonBreath => {
                vec![]
            }
            Item::SoulSoil => {
                vec![]
            }
            Item::InfestedCobblestone => {
                vec![]
            }
            Item::MusicDiscStrad => {
                vec![]
            }
            Item::JungleSapling => {
                vec![]
            }
            Item::BirchDoor => {
                vec![]
            }
            Item::InfestedCrackedStoneBricks => {
                vec![]
            }
            Item::BlackStainedGlass => {
                vec![]
            }
            Item::DirtPath => {
                vec![]
            }
            Item::CrimsonSign => {
                vec![]
            }
            Item::Brick => {
                vec![]
            }
            Item::IronLeggings => {
                vec![]
            }
            Item::GhastSpawnEgg => {
                vec![]
            }
            Item::PolishedDiorite => {
                vec![]
            }
            Item::TurtleSpawnEgg => {
                vec![]
            }
            Item::StrippedSpruceWood => {
                vec![]
            }
            Item::ExperienceBottle => {
                vec![]
            }
            Item::OakWood => {
                vec![]
            }
            Item::SkeletonHorseSpawnEgg => {
                vec![]
            }
            Item::WhiteStainedGlassPane => {
                vec![]
            }
            Item::JungleTrapdoor => {
                vec![]
            }
            Item::SpruceFenceGate => {
                vec![]
            }
            Item::Flint => {
                vec![]
            }
            Item::WaxedOxidizedCutCopper => {
                vec![]
            }
            Item::LightBlueShulkerBox => {
                vec![]
            }
            Item::PolishedBlackstoneButton => {
                vec![]
            }
            Item::RedSand => {
                vec![]
            }
            Item::CrimsonNylium => {
                vec![]
            }
            Item::Sandstone => {
                vec![]
            }
            Item::RedMushroomBlock => {
                vec![]
            }
            Item::IronIngot => {
                vec![]
            }
            Item::BigDripleaf => {
                vec![]
            }
            Item::HayBlock => {
                vec![]
            }
            Item::AcaciaPressurePlate => {
                vec![]
            }
            Item::RedTulip => {
                vec![]
            }
            Item::PolishedDeepslateSlab => {
                vec![]
            }
            Item::InkSac => {
                vec![]
            }
            Item::PurpleConcrete => {
                vec![]
            }
            Item::HeartOfTheSea => {
                vec![]
            }
            Item::GlobeBannerPattern => {
                vec![]
            }
            Item::LimeCandle => {
                vec![]
            }
            Item::SmoothQuartz => {
                vec![]
            }
            Item::RedNetherBrickWall => {
                vec![]
            }
            Item::ClayBall => {
                vec![]
            }
            Item::DrownedSpawnEgg => {
                vec![]
            }
            Item::CutCopperStairs => {
                vec![]
            }
            Item::QuartzBricks => {
                vec![]
            }
            Item::CrackedDeepslateBricks => {
                vec![]
            }
            Item::Comparator => {
                vec![]
            }
            Item::Stonecutter => {
                vec![]
            }
            Item::GreenShulkerBox => {
                vec![]
            }
            Item::WarpedWartBlock => {
                vec![]
            }
            Item::WarpedPressurePlate => {
                vec![]
            }
            Item::AzaleaLeaves => {
                vec![]
            }
            Item::LimeCarpet => {
                vec![]
            }
            Item::WarpedTrapdoor => {
                vec![]
            }
            Item::CyanStainedGlass => {
                vec![]
            }
            Item::ElderGuardianSpawnEgg => {
                vec![]
            }
            Item::DarkOakPlanks => {
                vec![]
            }
            Item::RedNetherBrickStairs => {
                vec![]
            }
            Item::GoldOre => {
                vec![]
            }
            Item::BrickWall => {
                vec![]
            }
            Item::IronHorseArmor => {
                vec![]
            }
            Item::ChestMinecart => {
                vec![]
            }
            Item::NetheriteHoe => {
                vec![]
            }
            Item::BlazePowder => {
                vec![]
            }
            Item::Cornflower => {
                vec![]
            }
            Item::DeepslateEmeraldOre => {
                vec![]
            }
            Item::DarkOakLog => {
                vec![]
            }
            Item::YellowGlazedTerracotta => {
                vec![]
            }
            Item::OakBoat => {
                vec![]
            }
            Item::SculkSensor => {
                vec![]
            }
            Item::TurtleHelmet => {
                vec![]
            }
            Item::CobblestoneStairs => {
                vec![]
            }
            Item::DragonHead => {
                vec![]
            }
            Item::OrangeStainedGlassPane => {
                vec![]
            }
            Item::DeadBubbleCoral => {
                vec![]
            }
            Item::ItemFrame => {
                vec![]
            }
            Item::LlamaSpawnEgg => {
                vec![]
            }
            Item::LimeBed => {
                vec![]
            }
            Item::IronBlock => {
                vec![]
            }
            Item::NetheritePickaxe => {
                vec![]
            }
            Item::OrangeDye => {
                vec![]
            }
            Item::PurpleBed => {
                vec![]
            }
            Item::BrewingStand => {
                vec![]
            }
            Item::PinkCarpet => {
                vec![]
            }
            Item::CrimsonHyphae => {
                vec![]
            }
            Item::Hopper => {
                vec![]
            }
            Item::OrangeCandle => {
                vec![]
            }
            Item::ChiseledStoneBricks => {
                vec![]
            }
            Item::DeepslateBrickWall => {
                vec![]
            }
            Item::RedStainedGlassPane => {
                vec![]
            }
            Item::RedDye => {
                vec![]
            }
            Item::MagentaStainedGlassPane => {
                vec![]
            }
            Item::AcaciaBoat => {
                vec![]
            }
            Item::PurpleWool => {
                vec![]
            }
            Item::LightBlueTerracotta => {
                vec![]
            }
            Item::PolishedDeepslateStairs => {
                vec![]
            }
            Item::CopperBlock => {
                vec![]
            }
            Item::AcaciaStairs => {
                vec![]
            }
            Item::PolishedDioriteSlab => {
                vec![]
            }
            Item::AxolotlSpawnEgg => {
                vec![]
            }
            Item::Candle => {
                vec![]
            }
            Item::CrimsonTrapdoor => {
                vec![]
            }
            Item::Sand => {
                vec![]
            }
            Item::LimeConcrete => {
                vec![]
            }
            Item::Bone => {
                vec![]
            }
            Item::TropicalFishSpawnEgg => {
                vec![]
            }
            Item::SoulTorch => {
                vec![]
            }
            Item::BrownCandle => {
                vec![]
            }
            Item::Minecart => {
                vec![]
            }
            Item::YellowShulkerBox => {
                vec![]
            }
            Item::RabbitStew => {
                vec![]
            }
            Item::OakTrapdoor => {
                vec![]
            }
            Item::WaxedOxidizedCutCopperStairs => {
                vec![]
            }
            Item::AcaciaWood => {
                vec![]
            }
            Item::NetheriteChestplate => {
                vec![]
            }
            Item::BlackCandle => {
                vec![]
            }
            Item::CobbledDeepslateStairs => {
                vec![]
            }
            Item::SpiderEye => {
                vec![]
            }
            Item::GrayConcrete => {
                vec![]
            }
            Item::GrayStainedGlass => {
                vec![]
            }
            Item::LightGrayStainedGlassPane => {
                vec![]
            }
            Item::PolishedBlackstonePressurePlate => {
                vec![]
            }
            Item::JungleLeaves => {
                vec![]
            }
            Item::ArmorStand => {
                vec![]
            }
            Item::CrimsonFence => {
                vec![]
            }
            Item::SandstoneSlab => {
                vec![]
            }
            Item::OrangeCarpet => {
                vec![]
            }
            Item::AncientDebris => {
                vec![]
            }
            Item::SmoothSandstoneSlab => {
                vec![]
            }
            Item::WhiteShulkerBox => {
                vec![]
            }
            Item::CrackedDeepslateTiles => {
                vec![]
            }
            Item::PurpleStainedGlassPane => {
                vec![]
            }
            Item::PolishedBlackstoneBricks => {
                vec![]
            }
            Item::TwistingVines => {
                vec![]
            }
            Item::DriedKelpBlock => {
                vec![]
            }
            Item::StrippedOakLog => {
                vec![]
            }
            Item::LightGrayConcrete => {
                vec![]
            }
            Item::Peony => {
                vec![]
            }
            Item::DioriteSlab => {
                vec![]
            }
            Item::SweetBerries => {
                vec![]
            }
            Item::Spawner => {
                vec![]
            }
            Item::ExposedCopper => {
                vec![]
            }
            Item::WhiteConcretePowder => {
                vec![]
            }
            Item::RedConcretePowder => {
                vec![]
            }
            Item::PiglinSpawnEgg => {
                vec![]
            }
            Item::Potato => {
                vec![]
            }
            Item::LapisBlock => {
                vec![]
            }
            Item::YellowBed => {
                vec![]
            }
            Item::Clock => {
                vec![]
            }
            Item::DiamondAxe => {
                vec![]
            }
            Item::PurpurStairs => {
                vec![]
            }
            Item::Light => {
                vec![]
            }
            Item::TropicalFish => {
                vec![]
            }
            Item::PumpkinSeeds => {
                vec![]
            }
            Item::PolishedGraniteSlab => {
                vec![]
            }
            Item::CrimsonDoor => {
                vec![]
            }
            Item::StriderSpawnEgg => {
                vec![]
            }
            Item::OrangeWool => {
                vec![]
            }
            Item::BrainCoral => {
                vec![]
            }
            Item::IronChestplate => {
                vec![]
            }
            Item::LightGrayCandle => {
                vec![]
            }
            Item::NameTag => {
                vec![]
            }
            Item::WarpedStairs => {
                vec![]
            }
            Item::Shroomlight => {
                vec![]
            }
            Item::MossyStoneBrickStairs => {
                vec![]
            }
            Item::DeadTubeCoralFan => {
                vec![]
            }
            Item::MossCarpet => {
                vec![]
            }
            Item::WaterBucket => {
                vec![]
            }
            Item::BlackShulkerBox => {
                vec![]
            }
            Item::MossyCobblestoneSlab => {
                vec![]
            }
            Item::GreenWool => {
                vec![]
            }
            Item::MossyStoneBricks => {
                vec![]
            }
            Item::NoteBlock => {
                vec![]
            }
            Item::MilkBucket => {
                vec![]
            }
            Item::MossyCobblestone => {
                vec![]
            }
            Item::PurpurPillar => {
                vec![]
            }
            Item::Beacon => {
                vec![]
            }
            Item::PolishedAndesite => {
                vec![]
            }
            Item::DeepslateTileStairs => {
                vec![]
            }
            Item::PolishedBlackstoneBrickStairs => {
                vec![]
            }
            Item::MagentaStainedGlass => {
                vec![]
            }
            Item::OrangeBed => {
                vec![]
            }
            Item::Grass => {
                vec![]
            }
            Item::GrayConcretePowder => {
                vec![]
            }
            Item::CookedBeef => {
                vec![]
            }
            Item::WhiteTulip => {
                vec![]
            }
            Item::Cake => {
                vec![]
            }
            Item::Bread => {
                vec![]
            }
            Item::ChorusFruit => {
                vec![]
            }
            Item::RottenFlesh => {
                vec![]
            }
            Item::DarkOakPressurePlate => {
                vec![]
            }
            Item::MagentaCandle => {
                vec![]
            }
            Item::SeaPickle => {
                vec![]
            }
            Item::CrimsonStairs => {
                vec![]
            }
            Item::MagentaDye => {
                vec![]
            }
            Item::GlowItemFrame => {
                vec![]
            }
            Item::BirchSapling => {
                vec![]
            }
            Item::MagentaTerracotta => {
                vec![]
            }
            Item::CyanTerracotta => {
                vec![]
            }
            Item::RedMushroom => {
                vec![]
            }
            Item::PiglinBruteSpawnEgg => {
                vec![]
            }
            Item::RespawnAnchor => {
                vec![]
            }
            Item::ActivatorRail => {
                vec![]
            }
            Item::LimeDye => {
                vec![]
            }
            Item::CodSpawnEgg => {
                vec![]
            }
            Item::WaxedExposedCopper => {
                vec![]
            }
            Item::CobblestoneSlab => {
                vec![]
            }
            Item::PurpleStainedGlass => {
                vec![]
            }
            Item::PackedIce => {
                vec![]
            }
            Item::Arrow => {
                vec![]
            }
            Item::SpruceBoat => {
                vec![]
            }
            Item::EmeraldOre => {
                vec![]
            }
            Item::ExposedCutCopper => {
                vec![]
            }
            Item::Pufferfish => {
                vec![]
            }
            Item::MusicDiscBlocks => {
                vec![]
            }
            Item::BlackCarpet => {
                vec![]
            }
            Item::SandstoneStairs => {
                vec![]
            }
            Item::TubeCoralFan => {
                vec![]
            }
            Item::Jigsaw => {
                vec![]
            }
            Item::StrippedWarpedHyphae => {
                vec![]
            }
            Item::WoodenShovel => {
                vec![]
            }
            Item::CutSandstoneSlab => {
                vec![]
            }
            Item::PufferfishSpawnEgg => {
                vec![]
            }
            Item::SalmonBucket => {
                vec![]
            }
            Item::GrayStainedGlassPane => {
                vec![]
            }
            Item::BirchButton => {
                vec![]
            }
            Item::Grindstone => {
                vec![]
            }
            Item::OakFenceGate => {
                vec![]
            }
            Item::Snowball => {
                vec![]
            }
            Item::StoneBrickSlab => {
                vec![]
            }
            Item::PolishedBlackstoneBrickSlab => {
                vec![]
            }
            Item::BlackTerracotta => {
                vec![]
            }
            Item::Potion => {
                vec![]
            }
            Item::DiamondHoe => {
                vec![]
            }
            Item::IronOre => {
                vec![]
            }
            Item::Mutton => {
                vec![]
            }
            Item::SpectralArrow => {
                vec![]
            }
            Item::GoldenChestplate => {
                vec![]
            }
            Item::PolishedDeepslateWall => {
                vec![]
            }
            Item::RedWool => {
                vec![]
            }
            Item::GreenCarpet => {
                vec![]
            }
            Item::RedSandstoneSlab => {
                vec![]
            }
            Item::WarpedPlanks => {
                vec![]
            }
            Item::AcaciaSapling => {
                vec![]
            }
            Item::Stone => {
                vec![]
            }
            Item::PurpleGlazedTerracotta => {
                vec![]
            }
            Item::Chest => {
                vec![]
            }
            Item::WitherSkeletonSkull => {
                vec![]
            }
            Item::ShulkerShell => {
                vec![]
            }
            Item::Blackstone => {
                vec![]
            }
            Item::BrownShulkerBox => {
                vec![]
            }
            Item::CookedRabbit => {
                vec![]
            }
            Item::CopperOre => {
                vec![]
            }
            Item::Chicken => {
                vec![]
            }
            Item::Tuff => {
                vec![]
            }
            Item::WarpedHyphae => {
                vec![]
            }
            Item::GrayGlazedTerracotta => {
                vec![]
            }
            Item::BrownDye => {
                vec![]
            }
            Item::GlowSquidSpawnEgg => {
                vec![]
            }
            Item::Bookshelf => {
                vec![]
            }
            Item::HoneyBlock => {
                vec![]
            }
            Item::DiamondBoots => {
                vec![]
            }
            Item::PhantomMembrane => {
                vec![]
            }
            Item::Farmland => {
                vec![]
            }
            Item::HornCoralFan => {
                vec![]
            }
            Item::PurpurSlab => {
                vec![]
            }
            Item::BrownBanner => {
                vec![]
            }
            Item::ChippedAnvil => {
                vec![]
            }
            Item::WaxedOxidizedCopper => {
                vec![]
            }
            Item::HuskSpawnEgg => {
                vec![]
            }
            Item::NetherWart => {
                vec![]
            }
            Item::WeatheredCutCopperSlab => {
                vec![]
            }
            Item::PurpleDye => {
                vec![]
            }
            Item::DeepslateBrickStairs => {
                vec![]
            }
            Item::EnderEye => {
                vec![]
            }
            Item::WarpedFungusOnAStick => {
                vec![]
            }
            Item::TallGrass => {
                vec![]
            }
            Item::RavagerSpawnEgg => {
                vec![]
            }
            Item::WetSponge => {
                vec![]
            }
            Item::DeepslateGoldOre => {
                vec![]
            }
            Item::Dropper => {
                vec![]
            }
            Item::NetheriteBoots => {
                vec![]
            }
            Item::Emerald => {
                vec![]
            }
            Item::SpruceSign => {
                vec![]
            }
            Item::Furnace => {
                vec![]
            }
            Item::AcaciaSlab => {
                vec![]
            }
            Item::RedNetherBricks => {
                vec![]
            }
            Item::BrownWool => {
                vec![]
            }
            Item::MojangBannerPattern => {
                vec![]
            }
            Item::SuspiciousStew => {
                vec![]
            }
            Item::OcelotSpawnEgg => {
                vec![]
            }
            Item::BrainCoralBlock => {
                vec![]
            }
            Item::DiamondHorseArmor => {
                vec![]
            }
            Item::Composter => {
                vec![]
            }
            Item::StrippedDarkOakLog => {
                vec![]
            }
            Item::FireCoral => {
                vec![]
            }
            Item::SeaLantern => {
                vec![]
            }
            Item::PolishedDioriteStairs => {
                vec![]
            }
            Item::Campfire => {
                vec![]
            }
            Item::SmoothStone => {
                vec![]
            }
            Item::IronSword => {
                vec![]
            }
            Item::Terracotta => {
                vec![]
            }
            Item::SmoothSandstoneStairs => {
                vec![]
            }
            Item::PrismarineWall => {
                vec![]
            }
            Item::RawIron => {
                vec![]
            }
            Item::YellowDye => {
                vec![]
            }
            Item::BlackBanner => {
                vec![]
            }
            Item::MusicDiscStal => {
                vec![]
            }
            Item::Basalt => {
                vec![]
            }
            Item::BlazeRod => {
                vec![]
            }
            Item::PoweredRail => {
                vec![]
            }
            Item::Lodestone => {
                vec![]
            }
            Item::PolishedDeepslate => {
                vec![]
            }
            Item::JungleSign => {
                vec![]
            }
            Item::StoneButton => {
                vec![]
            }
            Item::NetherBricks => {
                vec![]
            }
            Item::HoglinSpawnEgg => {
                vec![]
            }
            Item::Beehive => {
                vec![]
            }
            Item::JungleStairs => {
                vec![]
            }
            Item::PrismarineStairs => {
                vec![]
            }
            Item::EndStoneBrickStairs => {
                vec![]
            }
            Item::StrippedCrimsonStem => {
                vec![]
            }
            Item::Shears => {
                vec![]
            }
            Item::PumpkinPie => {
                vec![]
            }
            Item::SplashPotion => {
                vec![]
            }
            Item::WeatheredCopper => {
                vec![]
            }
            Item::WhiteConcrete => {
                vec![]
            }
            Item::EndStoneBrickWall => {
                vec![]
            }
            Item::LightGrayDye => {
                vec![]
            }
            Item::BrownBed => {
                vec![]
            }
            Item::DiamondBlock => {
                vec![]
            }
            Item::DarkOakSlab => {
                vec![]
            }
            Item::DeadBush => {
                vec![]
            }
            Item::GrayBed => {
                vec![]
            }
            Item::CreeperHead => {
                vec![]
            }
            Item::BlueCarpet => {
                vec![]
            }
            Item::Cobweb => {
                vec![]
            }
            Item::OxidizedCutCopperStairs => {
                vec![]
            }
            Item::SpruceStairs => {
                vec![]
            }
            Item::EndPortalFrame => {
                vec![]
            }
            Item::DarkOakDoor => {
                vec![]
            }
            Item::LightBlueBanner => {
                vec![]
            }
            Item::NetherBrickStairs => {
                vec![]
            }
            Item::DarkPrismarine => {
                vec![]
            }
            Item::Sugar => {
                vec![]
            }
            Item::YellowConcrete => {
                vec![]
            }
            Item::RawGold => {
                vec![]
            }
            Item::LightGrayShulkerBox => {
                vec![]
            }
            Item::PinkStainedGlass => {
                vec![]
            }
            Item::PinkConcretePowder => {
                vec![]
            }
            Item::AxolotlBucket => {
                vec![]
            }
            Item::MusicDiscChirp => {
                vec![]
            }
            Item::BlackStainedGlassPane => {
                vec![]
            }
            Item::Feather => {
                vec![]
            }
            Item::EnderChest => {
                vec![]
            }
            Item::BlueBed => {
                vec![]
            }
            Item::PandaSpawnEgg => {
                vec![]
            }
            Item::YellowStainedGlass => {
                vec![]
            }
            Item::GoldenLeggings => {
                vec![]
            }
            Item::RedSandstoneWall => {
                vec![]
            }
            Item::NetherWartBlock => {
                vec![]
            }
            Item::NetherSprouts => {
                vec![]
            }
            Item::ChainmailHelmet => {
                vec![]
            }
            Item::ChorusFlower => {
                vec![]
            }
            Item::FletchingTable => {
                vec![]
            }
            Item::ZombifiedPiglinSpawnEgg => {
                vec![]
            }
            Item::ZombieSpawnEgg => {
                vec![]
            }
            Item::SpruceSlab => {
                vec![]
            }
            Item::PiglinBannerPattern => {
                vec![]
            }
            Item::DarkOakWood => {
                vec![]
            }
            Item::GildedBlackstone => {
                vec![]
            }
            Item::EndStoneBricks => {
                vec![]
            }
            Item::BirchBoat => {
                vec![]
            }
            Item::WoodenHoe => {
                vec![]
            }
            Item::SpruceDoor => {
                vec![]
            }
            Item::RedGlazedTerracotta => {
                vec![]
            }
            Item::AcaciaButton => {
                vec![]
            }
            Item::BlueConcretePowder => {
                vec![]
            }
            Item::DeepslateBricks => {
                vec![]
            }
            Item::OrangeStainedGlass => {
                vec![]
            }
            Item::SpruceButton => {
                vec![]
            }
            Item::Conduit => {
                vec![]
            }
            Item::Quartz => {
                vec![]
            }
            Item::LeatherChestplate => {
                vec![]
            }
            Item::LimeConcretePowder => {
                vec![]
            }
            Item::WhiteGlazedTerracotta => {
                vec![]
            }
            Item::SpruceFence => {
                vec![]
            }
            Item::DarkPrismarineStairs => {
                vec![]
            }
            Item::Granite => {
                vec![]
            }
            Item::GrassBlock => {
                vec![]
            }
            Item::OakPressurePlate => {
                vec![]
            }
            Item::StructureBlock => {
                vec![]
            }
            Item::GreenBanner => {
                vec![]
            }
            Item::DeepslateTiles => {
                vec![]
            }
            Item::MagmaCubeSpawnEgg => {
                vec![]
            }
            Item::PinkGlazedTerracotta => {
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
