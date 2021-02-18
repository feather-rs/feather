use quill_common::entities::Horse;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Horse).add(EntityKind::Horse);
}
