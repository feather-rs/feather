use quill_common::entities::Fox;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Fox).add(EntityKind::Fox);
}
