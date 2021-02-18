use quill_common::entities::LeashKnot;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(LeashKnot).add(EntityKind::LeashKnot);
}
