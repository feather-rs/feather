use serde::{Serialize, Deserialize};
use hashbrown::HashMap;

type Block = String;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Entity {
    This,
    Killer,
    KillerPlayer,
}

#[derive(Serialize, Deserialize)]
pub struct Properties {
    #[serde(default, skip_serializing_if = "is_default")]
    on_fire: bool,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Deserialize)]
#[serde(tag = "condition", rename_all = "snake_case")]
pub enum Condition {
    EntityProperties {
        properties: Properties
    },
    KilledByPlayer {
        #[serde(default, skip_serializing_if = "is_default")] 
        inverse: bool,
    },
    RandomChance {
        chance: f32,
    },
    RandomChanceWithLooting {
        chance: f32,
        looting_multiplier: f32,
    }
}

fn is_default<T: Default + PartialEq>(t: &T) -> bool {
    t == &T::default()
}
