use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Creeper;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Creeper).add(EntityKind::Creeper);
}
