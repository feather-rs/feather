use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Giant;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Giant).add(EntityKind::Giant);
}
