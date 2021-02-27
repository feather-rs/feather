use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Wolf;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Wolf).add(EntityKind::Wolf);
}
