/*
All events in this file are triggered when there is a change in a certain value.
*/

use derive_more::Deref;
use libcraft::{Area, Gamemode};
use serde::{Deserialize, Serialize};
use vane::{Component, Entity};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CreativeFlyingEvent {
    pub is_flying: bool,
}
impl Component for CreativeFlyingEvent {}

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
impl Component for SneakEvent {}

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
impl Component for SprintEvent {}

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
impl Component for GamemodeEvent {}

/// This event is called when player's ability to instantly break blocks changes.
#[derive(Debug, Serialize, Deserialize, Clone, Deref)]
pub struct InstabreakEvent(pub bool);
impl Component for InstabreakEvent {}

/// This event is called when player's ability to fly changes.
#[derive(Debug, Serialize, Deserialize, Clone, Deref)]
pub struct FlyingAbilityEvent(pub bool);
impl Component for FlyingAbilityEvent {}

/// This event is called when player's ability to place or break blocks changes.
#[derive(Debug, Serialize, Deserialize, Clone, Deref)]
pub struct BuildingAbilityEvent(pub bool);
impl Component for BuildingAbilityEvent {}

/// This event is called when player's invulnerability property changes.
#[derive(Debug, Serialize, Deserialize, Clone, Deref)]
pub struct InvulnerabilityEvent(pub bool);
impl Component for InvulnerabilityEvent {}

/// Invoked when a slot in an inventory is updated.
#[derive(Debug)]
pub struct InventorySlotUpdateEvent {
    /// The entity whose slot was updated
    pub entity: Entity,
    pub area: Area,
    pub slot: usize,
}
impl Component for InventorySlotUpdateEvent {}

#[derive(Debug)]
pub struct HeldItemChangeEvent;
impl Component for HeldItemChangeEvent {}
