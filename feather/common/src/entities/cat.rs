use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Cat;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Cat).add(EntityKind::Cat);
}
