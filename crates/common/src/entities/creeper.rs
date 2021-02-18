use quill_common::entities::Creeper;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Creeper).add(EntityKind::Creeper);
}
