use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Guardian;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Guardian).add(EntityKind::Guardian);
}
