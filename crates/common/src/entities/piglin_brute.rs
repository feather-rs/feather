use quill_common::entities::PiglinBrute;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(PiglinBrute).add(EntityKind::PiglinBrute);
}
