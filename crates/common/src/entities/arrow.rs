use quill_common::entities::Arrow;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Arrow).add(EntityKind::Arrow);
}
