use libcraft::{Hand, InteractionType, Vec3f};
use serde::{Deserialize, Serialize};
use vane::{Component, Entity};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InteractEntityEvent {
    pub target: Entity,
    pub ty: InteractionType,
    pub target_pos: Option<Vec3f>,
    pub hand: Option<Hand>,
    pub sneaking: bool,
}

impl Component for InteractEntityEvent {}
