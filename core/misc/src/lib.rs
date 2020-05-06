use feather_blocks::BlockId;
use feather_items::ItemStack;
use ordinalizer::Ordinal;

/// This is an enum over the kinds of particles
/// listed on [the Particle data type](https://wiki.vg/index.php?title=Protocol&diff=14889&oldid=14881#Particle).
#[derive(Copy, Clone, Debug, PartialEq, Ordinal)]
pub enum ParticleData {
    AmbientEntityEffect,
    AngryVillager,
    Barrier,
    /// Block break particles
    Block(BlockId),
    Bubble,
    Cloud,
    Crit,
    DamageIndicator,
    DragonBreath,
    DrippingLava,
    FallingLava,
    LandingLava,
    DrippingWater,
    FallingWater,
    Dust {
        red: f32,
        green: f32,
        blue: f32,
        /// Clamped between 0.01 and 4.0.
        scale: f32,
    },
    Effect,
    ElderGuardian,
    EnchantedHit,
    Enchant,
    EndRod,
    EntityEffect,
    ExplosionEmitter,
    Explosion,
    FallingDust(BlockId),
    Firework,
    Fishing,
    Flame,
    Flash,
    HappyVillager,
    Composter,
    Heart,
    InstantEffect,
    Item(Option<ItemStack>),
    ItemSlime,
    ItemSnowball,
    LargeSmoke,
    Lava,
    Mycelium,
    Note,
    Poof,
    Portal,
    Rain,
    Smoke,
    Sneeze,
    Spit,
    SquidInk,
    SweepAttack,
    TotemOfUndying,
    Underwater,
    Splash,
    Witch,
    BublePop,
    CurrentDown,
    BubbleColumnUp,
    Nautilus,
    Dolphin,
    CampfireCosySmoke,
    CampfireSignalSmoke,
    DrippingHoney,
    FallingHoney,
    LandingHoney,
    FallingNectar,
}

impl Default for ParticleData {
    fn default() -> Self {
        ParticleData::Dust {
            red: 1.0,
            blue: 0.0,
            green: 0.0,
            scale: 1.0,
        }
    }
}
