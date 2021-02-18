use quill_common::entities::Painting;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Painting).add(EntityKind::Painting);
}
