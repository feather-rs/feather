use quill_common::entities::Guardian;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Guardian).add(EntityKind::Guardian);
}
