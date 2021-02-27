use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Pillager;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Pillager).add(EntityKind::Pillager);
}
