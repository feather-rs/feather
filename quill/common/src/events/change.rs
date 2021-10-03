/*
All events in this file are triggered when there is a change in a certain value.
*/

use crate::components::PreviousGamemode;
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GamemodeUpdateEvent {
    pub old: PreviousGamemode,
    pub new: Gamemode,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeUpdateEvent {
    pub old: u64,
    pub new: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InventoryUpdateEvent(pub Vec<usize>);

#[derive(Debug, Serialize, Deserialize, Clone, Deref)]
pub struct InstabreakChangeEvent(pub bool);

#[derive(Debug, Serialize, Deserialize, Clone, Deref)]
pub struct FlyingAbilityChangeEvent(pub bool);

#[derive(Debug, Serialize, Deserialize, Clone, Deref)]
pub struct BuildingAbilityChangeEvent(pub bool);

#[derive(Debug, Serialize, Deserialize, Clone, Deref)]
pub struct InvulnerabilityChangeEvent(pub bool);
