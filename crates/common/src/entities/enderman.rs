use quill_common::entities::Enderman;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Enderman).add(EntityKind::Enderman);
}
