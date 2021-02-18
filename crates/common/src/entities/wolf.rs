use quill_common::entities::Wolf;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Wolf).add(EntityKind::Wolf);
}
