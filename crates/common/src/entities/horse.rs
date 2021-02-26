use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Horse;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Horse).add(EntityKind::Horse);
}
