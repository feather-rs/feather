use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::LeashKnot;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(LeashKnot).add(EntityKind::LeashKnot);
}
