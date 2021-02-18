use quill_common::entities::Blaze;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Blaze).add(EntityKind::Blaze);
}
