use quill_common::entities::Giant;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Giant).add(EntityKind::Giant);
}