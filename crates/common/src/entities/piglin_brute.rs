use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::PiglinBrute;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(PiglinBrute).add(EntityKind::PiglinBrute);
}
