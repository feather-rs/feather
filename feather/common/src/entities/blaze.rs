use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Blaze;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Blaze).add(EntityKind::Blaze);
}
