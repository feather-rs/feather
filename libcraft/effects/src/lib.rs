use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub enum PotionEffect {
    None = 0,
    Speed = 1,
    Slowness = 2,
    Haste = 3,
    MiningFatigue = 4,
    Strength = 5,
    InstantHealth = 6,
    InstantDamage = 7,
    JumpBoost = 8,
    Nausea = 9,
    Regeneration = 10,
    Resistance = 11,
    FireResistance = 12,
    WaterBreathing = 13,
    Invisibility = 14,
    Blindness = 15,
    NightVision = 16,
    Hunger = 17,
    Weakness = 18,
    Poison = 19,
    Wither = 20,
    HealthBoost = 21,
    Absorption = 22,
    Saturation = 23,
    Glowing = 24,
    Levitation = 25,
    Luck = 26,
    BadLuck = 27,
    SlowFalling = 28,
    ConduitPower = 29,
    DolphinsGrace = 30,
    BadOmen = 31,
    HeroOfTheVillage = 32,
}

#[derive(Debug, Copy, Clone)]
pub struct PotionEffectConvertError;

impl TryFrom<&str> for PotionEffect {
    type Error = PotionEffectConvertError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "speed" => Ok(Self::Speed),
            "slowness" => Ok(Self::Slowness),
            "haste" => Ok(Self::Haste),
            "mining_fatigue" => Ok(Self::MiningFatigue),
            "strength" => Ok(Self::Strength),
            "instant_health" => Ok(Self::InstantHealth),
            "instant_damage" => Ok(Self::InstantDamage),
            "jump_boost" => Ok(Self::JumpBoost),
            "nausea" => Ok(Self::Nausea),
            "regeneration" => Ok(Self::Regeneration),
            "resistance" => Ok(Self::Resistance),
            "fire_resistance" => Ok(Self::FireResistance),
            "water_breathing" => Ok(Self::WaterBreathing),
            "invisibility" => Ok(Self::Invisibility),
            "blindness" => Ok(Self::Blindness),
            "night_vision" => Ok(Self::NightVision),
            "hunger" => Ok(Self::Hunger),
            "weakness" => Ok(Self::Weakness),
            "poison" => Ok(Self::Poison),
            "wither" => Ok(Self::Wither),
            "health_boost" => Ok(Self::HealthBoost),
            "absorption" => Ok(Self::Absorption),
            "saturation" => Ok(Self::Saturation),
            "glowing" => Ok(Self::Glowing),
            "levitation" => Ok(Self::Levitation),
            "luck" => Ok(Self::Luck),
            "unluck" => Ok(Self::BadLuck),
            "slow_falling" => Ok(Self::SlowFalling),
            "conduit_power" => Ok(Self::ConduitPower),
            "dolphins_grace" => Ok(Self::DolphinsGrace),
            "bad_omen" => Ok(Self::BadOmen),
            "hero_of_the_village" => Ok(Self::HeroOfTheVillage),
            _ => Err(PotionEffectConvertError),
        }
    }
}
