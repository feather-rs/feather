use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Player;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Player).add(EntityKind::Player);
}
