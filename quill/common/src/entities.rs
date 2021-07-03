use crate::HostComponent;

/// A struct with an entity ID.
///
/// Entity wrappers need to implement this trait.
pub trait HasEntityId {
    fn entity_id(&self) -> EntityId;
}

/// An entity wrapper struct.
///
/// See the `quill` API documentation for more information.
pub trait EntityWrapper: HasEntityId {
    /// Wraps the entity with the given ID.
    fn from_entity_id(entity_id: EntityId) -> Self;

    /// Gets the marker component that indicates whether
    /// an entity can be wrapped by this struct.
    fn marker_component() -> HostComponent;
}

/// Implemenents `EntityWrapper` for an entity wrapper struct.
macro_rules! entity_wrapper_impl {
    ($wrapper_struct:ident, $marker:ident) => {
        impl crate::entities::EntityWrapper for $wrapper_struct {
            fn from_entity_id(entity_id: EntityId) -> Self {
                Self { id: entity_id }
            }

            fn marker_component() -> crate::HostComponent {
                crate::HostComponent::$marker
            }
        }
    };
}

/// Implements `FromQuery` for an entity wrapper struct.
macro_rules! wrapper_from_query_impl {
    ($wrapper_struct:ident, $marker:ident) => {
        impl crate::entity::FromQuery for $wrapper_struct {
            type Item = Self;

            fn add_component_types(components: &mut impl Extend<crate::HostComponent>) {
                components.extend(std::iter::once(crate::HostComponent::$marker));
            }

            unsafe fn get_unchecked(
                entity_id: crate::entity::EntityId,
                _: &crate::entity::QueryData,
                _: &mut usize,
                _: &mut [usize],
            ) -> Self {
                Self { id: entity_id }
            }
        }
    };
}

pub mod area_effect_cloud;
pub use area_effect_cloud::{AreaEffectCloud, AreaEffectCloudMarker};
pub mod armor_stand;
pub use armor_stand::{ArmorStand, ArmorStandMarker};
pub mod arrow;
pub use arrow::{Arrow, ArrowMarker};
pub mod bat;
pub use bat::{Bat, BatMarker};
pub mod bee;
pub use bee::{Bee, BeeMarker};
pub mod blaze;
pub use blaze::{Blaze, BlazeMarker};
pub mod boat;
pub use boat::{Boat, BoatMarker};
pub mod cat;
pub use cat::{Cat, CatMarker};
pub mod cave_spider;
pub use cave_spider::{CaveSpider, CaveSpiderMarker};
pub mod chicken;
pub use chicken::{Chicken, ChickenMarker};
pub mod cod;
pub use cod::{Cod, CodMarker};
pub mod cow;
pub use cow::{Cow, CowMarker};
pub mod creeper;
pub use creeper::{Creeper, CreeperMarker};
pub mod dolphin;
pub use dolphin::{Dolphin, DolphinMarker};
pub mod donkey;
pub use donkey::{Donkey, DonkeyMarker};
pub mod dragon_fireball;
pub use dragon_fireball::{DragonFireball, DragonFireballMarker};
pub mod drowned;
pub use drowned::{Drowned, DrownedMarker};
pub mod elder_guardian;
pub use elder_guardian::{ElderGuardian, ElderGuardianMarker};
pub mod piglin_brute;
pub use piglin_brute::{PiglinBrute, PiglinBruteMarker};
pub mod end_crystal;
pub use end_crystal::{EndCrystal, EndCrystalMarker};
pub mod ender_dragon;
pub use ender_dragon::{EnderDragon, EnderDragonMarker};
pub mod enderman;
pub use enderman::{Enderman, EndermanMarker};
pub mod endermite;
pub use endermite::{Endermite, EndermiteMarker};
pub mod evoker;
pub use evoker::{Evoker, EvokerMarker};
pub mod evoker_fangs;
pub use evoker_fangs::{EvokerFangs, EvokerFangsMarker};
pub mod experience_orb;
pub use experience_orb::{ExperienceOrb, ExperienceOrbMarker};
pub mod eye_of_ender;
pub use eye_of_ender::{EyeOfEnder, EyeOfEnderMarker};
pub mod falling_block;
pub use falling_block::{FallingBlock, FallingBlockMarker};
pub mod firework_rocket;
pub use firework_rocket::{FireworkRocket, FireworkRocketMarker};
pub mod fox;
pub use fox::{Fox, FoxMarker};
pub mod ghast;
pub use ghast::{Ghast, GhastMarker};
pub mod giant;
pub use giant::{Giant, GiantMarker};
pub mod guardian;
pub use guardian::{Guardian, GuardianMarker};
pub mod hoglin;
pub use hoglin::{Hoglin, HoglinMarker};
pub mod horse;
pub use horse::{Horse, HorseMarker};
pub mod husk;
pub use husk::{Husk, HuskMarker};
pub mod illusioner;
pub use illusioner::{Illusioner, IllusionerMarker};
pub mod iron_golem;
pub use iron_golem::{IronGolem, IronGolemMarker};
pub mod item;
pub use item::{Item, ItemMarker};
pub mod item_frame;
pub use item_frame::{ItemFrame, ItemFrameMarker};
pub mod fireball;
pub use fireball::{Fireball, FireballMarker};
pub mod leash_knot;
pub use leash_knot::{LeashKnot, LeashKnotMarker};
pub mod lightning_bolt;
pub use lightning_bolt::{LightningBolt, LightningBoltMarker};
pub mod llama;
pub use llama::{Llama, LlamaMarker};
pub mod llama_spit;
pub use llama_spit::{LlamaSpit, LlamaSpitMarker};
pub mod magma_cube;
pub use magma_cube::{MagmaCube, MagmaCubeMarker};
pub mod minecart;
pub use minecart::{Minecart, MinecartMarker};
pub mod chest_minecart;
pub use chest_minecart::{ChestMinecart, ChestMinecartMarker};
pub mod command_block_minecart;
pub use command_block_minecart::{CommandBlockMinecart, CommandBlockMinecartMarker};
pub mod furnace_minecart;
pub use furnace_minecart::{FurnaceMinecart, FurnaceMinecartMarker};
pub mod hopper_minecart;
pub use hopper_minecart::{HopperMinecart, HopperMinecartMarker};
pub mod spawner_minecart;
pub use spawner_minecart::{SpawnerMinecart, SpawnerMinecartMarker};
pub mod tnt_minecart;
pub use tnt_minecart::{TntMinecart, TntMinecartMarker};
pub mod mule;
pub use mule::{Mule, MuleMarker};
pub mod mooshroom;
pub use mooshroom::{Mooshroom, MooshroomMarker};
pub mod ocelot;
pub use ocelot::{Ocelot, OcelotMarker};
pub mod painting;
pub use painting::{Painting, PaintingMarker};
pub mod panda;
pub use panda::{Panda, PandaMarker};
pub mod parrot;
pub use parrot::{Parrot, ParrotMarker};
pub mod phantom;
pub use phantom::{Phantom, PhantomMarker};
pub mod pig;
pub use pig::{Pig, PigMarker};
pub mod piglin;
pub use piglin::{Piglin, PiglinMarker};
pub mod pillager;
pub use pillager::{Pillager, PillagerMarker};
pub mod polar_bear;
pub use polar_bear::{PolarBear, PolarBearMarker};
pub mod tnt;
pub use tnt::{Tnt, TntMarker};
pub mod pufferfish;
pub use pufferfish::{Pufferfish, PufferfishMarker};
pub mod rabbit;
pub use rabbit::{Rabbit, RabbitMarker};
pub mod ravager;
pub use ravager::{Ravager, RavagerMarker};
pub mod salmon;
pub use salmon::{Salmon, SalmonMarker};
pub mod sheep;
pub use sheep::{Sheep, SheepMarker};
pub mod shulker;
pub use shulker::{Shulker, ShulkerMarker};
pub mod shulker_bullet;
pub use shulker_bullet::{ShulkerBullet, ShulkerBulletMarker};
pub mod silverfish;
pub use silverfish::{Silverfish, SilverfishMarker};
pub mod skeleton;
pub use skeleton::{Skeleton, SkeletonMarker};
pub mod skeleton_horse;
pub use skeleton_horse::{SkeletonHorse, SkeletonHorseMarker};
pub mod slime;
pub use slime::{Slime, SlimeMarker};
pub mod small_fireball;
pub use small_fireball::{SmallFireball, SmallFireballMarker};
pub mod snow_golem;
pub use snow_golem::{SnowGolem, SnowGolemMarker};
pub mod snowball;
pub use snowball::{Snowball, SnowballMarker};
pub mod spectral_arrow;
pub use spectral_arrow::{SpectralArrow, SpectralArrowMarker};
pub mod spider;
pub use spider::{Spider, SpiderMarker};
pub mod squid;
pub use squid::{Squid, SquidMarker};
pub mod stray;
pub use stray::{Stray, StrayMarker};
pub mod strider;
pub use strider::{Strider, StriderMarker};
pub mod egg;
pub use egg::{Egg, EggMarker};
pub mod ender_pearl;
pub use ender_pearl::{EnderPearl, EnderPearlMarker};
pub mod experience_bottle;
pub use experience_bottle::{ExperienceBottle, ExperienceBottleMarker};
pub mod potion;
pub use potion::{Potion, PotionMarker};
pub mod trident;
pub use trident::{Trident, TridentMarker};
pub mod trader_llama;
pub use trader_llama::{TraderLlama, TraderLlamaMarker};
pub mod tropical_fish;
pub use tropical_fish::{TropicalFish, TropicalFishMarker};
pub mod turtle;
pub use turtle::{Turtle, TurtleMarker};
pub mod vex;
pub use vex::{Vex, VexMarker};
pub mod villager;
pub use villager::{Villager, VillagerMarker};
pub mod vindicator;
pub use vindicator::{Vindicator, VindicatorMarker};
pub mod wandering_trader;
pub use wandering_trader::{WanderingTrader, WanderingTraderMarker};
pub mod witch;
pub use witch::{Witch, WitchMarker};
pub mod wither;
pub use wither::{Wither, WitherMarker};
pub mod wither_skeleton;
pub use wither_skeleton::{WitherSkeleton, WitherSkeletonMarker};
pub mod wither_skull;
pub use wither_skull::{WitherSkull, WitherSkullMarker};
pub mod wolf;
pub use wolf::{Wolf, WolfMarker};
pub mod zoglin;
pub use zoglin::{Zoglin, ZoglinMarker};
pub mod zombie;
pub use zombie::{Zombie, ZombieMarker};
pub mod zombie_horse;
pub use zombie_horse::{ZombieHorse, ZombieHorseMarker};
pub mod zombie_villager;
pub use zombie_villager::{ZombieVillager, ZombieVillagerMarker};
pub mod zombified_piglin;
pub use zombified_piglin::{ZombifiedPiglin, ZombifiedPiglinMarker};
pub mod player;
pub use player::{Player, PlayerMarker};
pub mod fishing_bobber;
pub use fishing_bobber::{FishingBobber, FishingBobberMarker};

use crate::EntityId;
