use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Fox;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Fox).add(EntityKind::Fox);
}
