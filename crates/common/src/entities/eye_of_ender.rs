use quill_common::entities::EyeOfEnder;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(EyeOfEnder).add(EntityKind::EyeOfEnder);
}
