use quill_common::entities::Mooshroom;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Mooshroom).add(EntityKind::Mooshroom);
}
