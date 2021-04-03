use serde::{Deserialize, Serialize};

/// Raw events for mutating health.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EntityHealthEvent {
    Damage(u32),
    Regen(u32),
}

// Damage Events
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EntityDamageEventType {
    BlockExplosion(u32),
    Contact,
    Cramming,
    DragonBreath(u32),
    Drowning,
    DryOut,
    EntityAttack(u32),
    EntityExplosion(u32),
    EntitySweepAttack(u32),
    Fall(u32),
    FallingBlock(u32),
    Fire,
    FireTick,
    FlyIntoWall(u32),
    HotFloor,
    Lava,
    Lightning,
    Magic(u32),
    Melting,
    Poison,
    Projectile(u32),
    Starvation,
    Suffocation,
    Suicide,
    Thorns,
    Void,
    Wither,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EntityRegenEventType {
    Eating,
    EnderCrystal,
    Magic,
    MagicRegen,
    Regen,
    Satiated,
    Wither,
    WitherSpawn,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityResurrectionEvent;
