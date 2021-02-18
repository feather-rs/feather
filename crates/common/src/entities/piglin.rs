use quill_common::entities::Piglin;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Piglin).add(EntityKind::Piglin);
}
