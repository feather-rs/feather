//! Taken from Spigot's documentation, EntityRegainHealthEvent<https://hub.spigotmc.org/javadocs/bukkit/org/bukkit/event/entity/EntityRegainHealthEvent.html> and EntityDamageEvent<https://hub.spigotmc.org/javadocs/bukkit/org/bukkit/event/entity/EntityDamageEvent.html>

use serde::{Deserialize, Serialize};

/// Raw events for mutating health.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EntityHealthEvent {
    Damage(u32),
    Regen(u32),
}

/// Damage Events
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EntityDamageEventType {
    /// Damage caused by being in the area when a block explodes.
    ///
    /// Damage: Variable
    BlockExplosion(u32),

    /// Damage caused when an entity contacts a block such as a Cactus.
    ///
    /// Damage: 1
    Contact,

    /// Damage caused when an entity is colliding with too many entities due to the `maxEntityCramming` game rule.
    ///
    /// Damage: 6
    Cramming,

    /// Damage caused by a dragon breathing fire.
    ///
    /// Damage: Variable
    DragonBreath(u32),

    /// Damage caused by running out of air while in water.
    ///
    /// Damage: 2
    Drowning,

    /// Damage caused when an entity that should be in water is not.
    ///
    /// Damage: 1
    DryOut,

    /// Damage caused when an entity attacks another entity.
    ///
    /// Damage: Variable
    EntityAttack(u32),

    /// Damage caused by being in the area when an entity, such as a Creeper, explodes.
    ///
    /// Damage: Variable
    EntityExplosion(u32),

    /// Damage caused when an entity attacks another entity in a sweep attack.
    ///
    /// Damage: Variable
    EntitySweepAttack(u32),

    /// Damage caused when an entity falls a distance greater than 3 blocks.
    ///
    /// Damage: fall_height - 3
    Fall(u32),

    /// Damage caused by being hit by a falling block which deals damage.
    ///
    /// Damage: Variable
    FallingBlock(u32),

    /// Damage caused by direct exposure to fire.
    ///
    /// Damage: 1
    Fire,

    /// Damage caused due to burns caused by fire.
    ///
    /// Damage: 1
    FireTick,

    /// Damage caused when an entity runs into a wall.
    ///
    /// Damage: Variable
    FlyIntoWall(u32),

    /// Damage caused when an entity steps on a magma block.
    ///
    /// Damage: 1
    HotFloor,

    /// Damage caused by direct exposure to lava.
    ///
    /// Damage: 4
    Lava,

    /// Damage caused by being struck by lightning.
    ///
    /// Damage: 5
    Lightning,

    /// Damage caused by being hit by a damage potion or spell.
    ///
    /// Damage: Variable
    Magic(u32),

    /// Damage caused due to a snowman melting.
    ///
    /// Damage: 1
    Melting,

    /// Damage caused due to an ongoing poison effect.
    ///
    /// Damage: 1
    Poison,

    /// Damage caused when attacked by a projectile.
    ///
    /// Damage: Variable
    Projectile(u32),

    /// Damage caused by starving due to having an empty hunger bar.
    ///
    /// Damage: 1
    Starvation,

    /// Damage caused by being put in a block.
    ///
    /// Damage: 1
    Suffocation,

    /// Damage caused in retaliation to another attack by the Thorns enchantment.
    ///
    /// Damage: 1-4
    Thorns,

    /// Damage caused by falling into the void.
    ///
    /// Damage: 4 for players
    Void,

    /// Damage caused by Wither potion effect.
    ///
    /// Damage: Variable
    Wither(u32),
}

/// Regeneration Events
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EntityRegenEventType {
    /// Health regeneration from eating consumables.
    Eating,

    /// Ender Dragon health regeneration from ender crystals.
    EnderCrystal,

    /// Potions or spells.
    Magic,

    /// Healed over time by potions or spells.
    MagicRegen,

    /// Health regeneration due to Peaceful mode (difficulty=0).
    Regen,

    /// Health regeneration due to their hunger being satisfied.
    Satiated,

    /// Wither passive health regeneration.
    Wither,

    /// Wither health regeneration when spawning.
    WitherSpawn,
}

/// Special event for respawning.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityResurrectionEvent;

/// Caused by committing suicide using the command, `/kill`.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntitySuicideEvent;
