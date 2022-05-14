//! Data sourced from: <https://minecraft.gamepedia.com/Enchanting#Enchantments>

use serde::{Deserialize, Serialize};

/// An enchantment attached to an item.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Enchantment {
    /// The type of the enchantment.
    #[serde(rename = "id")]
    kind: EnchantmentKind,
    /// Enchantment level, represented by an `i8` for vanilla compatibility
    #[serde(rename = "lvl")]
    level: i8,
}

impl Enchantment {
    /// Creates an enchantment given the type of
    /// enchantment and the level.
    ///
    /// Will allow any level of enchantment, i.e,
    /// level is not capped by the maximum level
    /// of the enchantment that can be acquired in the game.
    ///
    /// The level is capped at `i8::MAX` for compatability
    /// with Vanilla.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn new(kind: EnchantmentKind, level: u32) -> Self {
        Self {
            kind,
            level: level.min(i8::MAX as u32) as i8,
        }
    }

    /// Gets the kind of this enchantment.
    #[must_use]
    pub const fn kind(&self) -> EnchantmentKind {
        self.kind
    }

    /// Gets the level of this enchantment.
    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    pub fn level(&self) -> u32 {
        self.level.max(0) as u32
    }

    /// Sets the kind of this enchantment.
    ///
    /// The level is not affected.
    pub fn set_kind(&mut self, kind: EnchantmentKind) {
        self.kind = kind;
    }

    /// Sets the level of this enchantment.
    ///
    /// The level is capped to  `i8::MAX`.
    #[allow(clippy::cast_possible_truncation)]
    pub fn set_level(&mut self, level: u32) {
        self.level = level.min(i8::MAX as u32) as i8;
    }
}

/// Kind of an enchantment.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnchantmentKind {
    AquaAffinity,
    BaneOfArthropods,
    BlastProtection,
    Channeling,
    Cleaving,
    CurseOfBinding,
    CurseOfVanishing,
    DepthStrider,
    Efficiency,
    FeatherFalling,
    FireAspect,
    FireProtection,
    Flame,
    Fortune,
    FrostWalker,
    Impaling,
    Infinity,
    Knockback,
    Looting,
    Loyalty,
    LuckOfTheSea,
    Lure,
    Mending,
    Multishot,
    Piercing,
    Power,
    ProjectileProtection,
    Protection,
    Punch,
    QuickCharge,
    Respiration,
    Riptide,
    Sharpness,
    SilkTouch,
    Smite,
    SoulSpeed,
    SweepingEdge,
    Thorns,
    Unbreaking,
}
