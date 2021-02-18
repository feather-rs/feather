use quill_common::entities::Drowned;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Drowned).add(EntityKind::Drowned);
}
