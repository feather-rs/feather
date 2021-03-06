use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Egg;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Egg).add(EntityKind::Egg);
}
