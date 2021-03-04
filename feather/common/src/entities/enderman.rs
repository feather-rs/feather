use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Enderman;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Enderman).add(EntityKind::Enderman);
}
