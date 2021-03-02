use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Player;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Player).add(EntityKind::Player);
}

/// The hotbar slot a player's cursor is currently on
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct HotbarSlot(pub usize);
