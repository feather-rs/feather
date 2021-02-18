use quill_common::entities::Stray;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Stray).add(EntityKind::Stray);
}
