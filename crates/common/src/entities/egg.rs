use quill_common::entities::Egg;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Egg).add(EntityKind::Egg);
}
