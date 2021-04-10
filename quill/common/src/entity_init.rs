use serde::{Deserialize, Serialize};

/// Initial state of an entity passed
/// to `Game::create_entity_builder`.
#[derive(Debug, Serialize, Deserialize)]
pub enum EntityInit {
    /// Spawn an area effect cloud.
    AreaEffectCloud,

    /// Spawn an armor stand.
    ArmorStand,

    /// Spawn an arrow.
    Arrow,

    /// Spawn a bat.
    Bat,

    /// Spawn a bee.
    Bee,

    /// Spawn a blaze.
    Blaze,

    /// Spawn a boat.
    Boat,

    /// Spawn a cat.
    Cat,

    /// Spawn a cave spider.
    CaveSpider,

    /// Spawn a chicken.
    Chicken,

    /// Spawn a cod.
    Cod,

    /// Spawn a cow.
    Cow,

    /// Spawn a creeper.
    Creeper,

    /// Spawn a dolphin.
    Dolphin,

    /// Spawn a donkey.
    Donkey,

    /// Spawn a dragon fireball.
    DragonFireball,

    /// Spawn a drowned.
    Drowned,

    /// Spawn an elder guardian.
    ElderGuardian,

    /// Spawn an end crystal.
    EndCrystal,

    /// Spawn an ender dragon.
    EnderDragon,

    /// Spawn an enderman.
    Enderman,

    /// Spawn an endermite.
    Endermite,

    /// Spawn an evoker.
    Evoker,

    /// Spawn an evoker fangs.
    EvokerFangs,

    /// Spawn an experience orb.
    ExperienceOrb,

    /// Spawn an eye of ender.
    EyeOfEnder,

    /// Spawn a falling block.
    FallingBlock,

    /// Spawn a firework rocket.
    FireworkRocket,

    /// Spawn a fox.
    Fox,

    /// Spawn a ghast.
    Ghast,

    /// Spawn a giant.
    Giant,

    /// Spawn a guardian.
    Guardian,

    /// Spawn a hoglin.
    Hoglin,

    /// Spawn a horse.
    Horse,

    /// Spawn a husk.
    Husk,

    /// Spawn an illusioner.
    Illusioner,

    /// Spawn an iron golem.
    IronGolem,

    /// Spawn an item.
    Item,

    /// Spawn an item frame.
    ItemFrame,

    /// Spawn a fireball.
    Fireball,

    /// Spawn a leash knot.
    LeashKnot,

    /// Spawn a lightning bolt.
    LightningBolt,

    /// Spawn a llama.
    Llama,

    /// Spawn a llama spit.
    LlamaSpit,

    /// Spawn a magma cube.
    MagmaCube,

    /// Spawn a minecart.
    Minecart,

    /// Spawn a chest minecart.
    ChestMinecart,

    /// Spawn a command block minecart.
    CommandBlockMinecart,

    /// Spawn a furnace minecart.
    FurnaceMinecart,

    /// Spawn a hopper minecart.
    HopperMinecart,

    /// Spawn a spawner minecart.
    SpawnerMinecart,

    /// Spawn a tnt minecart.
    TntMinecart,

    /// Spawn a mule.
    Mule,

    /// Spawn a mooshroom.
    Mooshroom,

    /// Spawn an ocelot.
    Ocelot,

    /// Spawn a painting.
    Painting,

    /// Spawn a panda.
    Panda,

    /// Spawn a parrot.
    Parrot,

    /// Spawn a phantom.
    Phantom,

    /// Spawn a pig.
    Pig,

    /// Spawn a piglin.
    Piglin,

    /// Spawn a piglin brute.
    PiglinBrute,

    /// Spawn a pillager.
    Pillager,

    /// Spawn a polar bear.
    PolarBear,

    /// Spawn a tnt.
    Tnt,

    /// Spawn a pufferfish.
    Pufferfish,

    /// Spawn a rabbit.
    Rabbit,

    /// Spawn a ravager.
    Ravager,

    /// Spawn a salmon.
    Salmon,

    /// Spawn a sheep.
    Sheep,

    /// Spawn a shulker.
    Shulker,

    /// Spawn a shulker bullet.
    ShulkerBullet,

    /// Spawn a silverfish.
    Silverfish,

    /// Spawn a skeleton.
    Skeleton,

    /// Spawn a skeleton horse.
    SkeletonHorse,

    /// Spawn a slime.
    Slime,

    /// Spawn a small fireball.
    SmallFireball,

    /// Spawn a snow golem.
    SnowGolem,

    /// Spawn a snowball.
    Snowball,

    /// Spawn a spectral arrow.
    SpectralArrow,

    /// Spawn a spider.
    Spider,

    /// Spawn a squid.
    Squid,

    /// Spawn a stray.
    Stray,

    /// Spawn a strider.
    Strider,

    /// Spawn an egg.
    Egg,

    /// Spawn an ender pearl.
    EnderPearl,

    /// Spawn an experience bottle.
    ExperienceBottle,

    /// Spawn a potion.
    Potion,

    /// Spawn a trident.
    Trident,

    /// Spawn a trader llama.
    TraderLlama,

    /// Spawn a tropical fish.
    TropicalFish,

    /// Spawn a turtle.
    Turtle,

    /// Spawn a vex.
    Vex,

    /// Spawn a villager.
    Villager,

    /// Spawn a vindicator.
    Vindicator,

    /// Spawn a wandering trader.
    WanderingTrader,

    /// Spawn a witch.
    Witch,

    /// Spawn a wither.
    Wither,

    /// Spawn a wither skeleton.
    WitherSkeleton,

    /// Spawn a wither skull.
    WitherSkull,

    /// Spawn a wolf.
    Wolf,

    /// Spawn a zoglin.
    Zoglin,

    /// Spawn a zombie.
    Zombie,

    /// Spawn a zombie horse.
    ZombieHorse,

    /// Spawn a zombie villager.
    ZombieVillager,

    /// Spawn a zombified piglin.
    ZombifiedPiglin,

    /// Spawn a player.
    Player,

    /// Spawn a fishing bobber.
    FishingBobber,
}
