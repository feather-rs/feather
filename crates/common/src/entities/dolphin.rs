use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Dolphin;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Dolphin).add(EntityKind::Dolphin);
}
