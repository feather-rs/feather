use quill_common::entities::Spider;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Spider).add(EntityKind::Spider);
}
