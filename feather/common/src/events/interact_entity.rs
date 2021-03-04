use ecs::Entity;
use libcraft_core::{Hand, InteractionType, Vec3f};

#[derive(Debug)]
pub struct InteractEntityEvent {
    pub target: Entity,
    pub ty: InteractionType,
    pub target_pos: Option<Vec3f>,
    pub hand: Option<Hand>,
    pub sneaking: bool,
}
