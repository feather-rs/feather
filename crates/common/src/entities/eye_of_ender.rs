use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::EyeOfEnder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(EyeOfEnder).add(EntityKind::EyeOfEnder);
}
