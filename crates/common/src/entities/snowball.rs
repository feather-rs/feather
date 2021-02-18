use quill_common::entities::Snowball;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Snowball).add(EntityKind::Snowball);
}
