use quill_common::entities::Sheep;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Sheep).add(EntityKind::Sheep);
}
