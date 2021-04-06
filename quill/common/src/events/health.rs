//! Taken from Spigot's documentation, EntityRegainHealthEvent<https://hub.spigotmc.org/javadocs/bukkit/org/bukkit/event/entity/EntityRegainHealthEvent.html> and EntityDamageEvent<https://hub.spigotmc.org/javadocs/bukkit/org/bukkit/event/entity/EntityDamageEvent.html>

use serde::{Deserialize, Serialize};

/// Raw events for mutating health.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EntityHealthEvent {
    Damage(u32),
    Regen(u32),
}

/// Damage Events
pub mod entity_damage_event_type {
    use super::{Deserialize, Serialize};

    /// Damage caused by being in the area when a block explodes.
    ///
    /// Damage: Variable
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct BlockExplosion(u32);

    /// Damage caused when an entity contacts a block such as a Cactus.
    ///
    /// Damage: 1
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Contact;

    /// Damage caused when an entity is colliding with too many entities due to the `maxEntityCramming` game rule.
    ///
    /// Damage: 6
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Cramming;

    /// Damage caused by a dragon breathing fire.
    ///
    /// Damage: Variable
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct DragonBreath(u32);

    /// Damage caused by running out of air while in water.
    ///
    /// Damage: 2
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Drowning;

    /// Damage caused when an entity that should be in water is not.
    ///
    /// Damage: 1
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct DryOut;

    /// Damage caused when an entity attacks another entity.
    ///
    /// Damage: Variable
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct EntityAttack(u32);

    /// Damage caused by being in the area when an entity, such as a Creeper, explodes.
    ///
    /// Damage: Variable
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct EntityExplosion(u32);

    /// Damage caused when an entity attacks another entity in a sweep attack.
    ///
    /// Damage: Variable
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct EntitySweepAttack(u32);

    /// Damage caused when an entity falls a distance greater than 3 blocks.
    ///
    /// Damage: fall_height - 3
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Fall(u32);

    /// Damage caused by being hit by a falling block which deals damage.
    ///
    /// Damage: Variable
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct FallingBlock(u32);

    /// Damage caused by direct exposure to fire.
    ///
    /// Damage: 1
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Fire;

    /// Damage caused due to burns caused by fire.
    ///
    /// Damage: 1
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct FireTick;

    /// Damage caused when an entity runs into a wall.
    ///
    /// Damage: Variable
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct FlyIntoWall(u32);

    /// Damage caused when an entity steps on a magma block.
    ///
    /// Damage: 1
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct HotFloor;

    /// Damage caused by direct exposure to lava.
    ///
    /// Damage: 4
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Lava;

    /// Damage caused by being struck by lightning.
    ///
    /// Damage: 5
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Lightning;

    /// Damage caused by being hit by a damage potion or spell.
    ///
    /// Damage: Variable
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Magic(u32);

    /// Damage caused due to a snowman melting.
    ///
    /// Damage: 1
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Melting;

    /// Damage caused due to an ongoing poison effect.
    ///
    /// Damage: 1
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Poison;

    /// Damage caused when attacked by a projectile.
    ///
    /// Damage: Variable
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Projectile(u32);

    /// Damage caused by starving due to having an empty hunger bar.
    ///
    /// Damage: 1
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Starvation;

    /// Damage caused by being put in a block.
    ///
    /// Damage: 1
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Suffocation;

    /// Damage caused in retaliation to another attack by the Thorns enchantment.
    ///
    /// Damage: 1-4
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Thorns;

    /// Damage caused by falling into the void.
    ///
    /// Damage: 4 for players
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Void;

    /// Damage caused by Wither potion effect.
    ///
    /// Damage: Variable
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Wither(u32);
}

/// Regeneration Events
pub mod entity_regen_event_type {
    use super::{Deserialize, Serialize};

    /// Health regeneration from eating consumables.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Eating;

    /// Ender Dragon health regeneration from ender crystals.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct EnderCrystal;

    /// Potions or spells.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Magic;

    /// Healed over time by potions or spells.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct MagicRegen;

    /// Health regeneration due to Peaceful mode (difficulty=0).
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Regen;

    /// Health regeneration due to their hunger being satisfied.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Satiated;

    /// Wither passive health regeneration.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct Wither;

    /// Wither health regeneration when spawning.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct WitherSpawn;
}

pub mod entity_special_event_type {
    use super::{Deserialize, Serialize};

    /// Special event for respawning.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct EntityResurrectionEvent;

    /// Caused by committing suicide using the command, `/kill`.
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct EntitySuicideEvent;
}
