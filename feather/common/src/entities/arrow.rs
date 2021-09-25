use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Arrow;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Arrow).add(EntityKind::Arrow);
}
