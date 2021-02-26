use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Sheep;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Sheep).add(EntityKind::Sheep);
}
