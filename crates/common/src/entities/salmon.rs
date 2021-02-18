use quill_common::entities::Salmon;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Salmon).add(EntityKind::Salmon);
}
