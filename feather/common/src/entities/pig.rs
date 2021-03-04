use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Pig;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Pig).add(EntityKind::Pig);
}
