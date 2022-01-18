/*
All events in this file are triggered when there is a change in a certain value.
*/

use derive_more::Deref;
use libcraft_core::Gamemode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreativeFlyingEvent {
    pub is_flying: bool,
}

impl CreativeFlyingEvent {
    pub fn new(changed_to: bool) -> Self {
        Self {
            is_flying: changed_to,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SneakEvent {
    pub is_sneaking: bool,
}

impl SneakEvent {
    pub fn new(changed_to: bool) -> Self {
        Self {
            is_sneaking: changed_to,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SprintEvent {
    pub is_sprinting: bool,
}

impl SprintEvent {
    pub fn new(changed_to: bool) -> Self {
        Self {
            is_sprinting: changed_to,
        }
    }
}

/// This event is called when a player's gamemode is changed and every time the player joins.
#[derive(Debug, Serialize, Deserialize, Clone, Deref)]
pub struct GamemodeEvent(pub Gamemode);

/// This event is called when player's ability to instantly break blocks changes.
#[derive(Debug, Serialize, Deserialize, Clone, Deref)]
pub struct InstabreakEvent(pub bool);

/// This event is called when player's ability to fly changes.
#[derive(Debug, Serialize, Deserialize, Clone, Deref)]
pub struct FlyingAbilityEvent(pub bool);

/// This event is called when player's ability to place or break blocks changes.
#[derive(Debug, Serialize, Deserialize, Clone, Deref)]
pub struct BuildingAbilityEvent(pub bool);

/// This event is called when player's invulnerability property changes.
#[derive(Debug, Serialize, Deserialize, Clone, Deref)]
pub struct InvulnerabilityEvent(pub bool);
