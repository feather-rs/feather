use quill_common::entities::SmallFireball;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(SmallFireball).add(EntityKind::SmallFireball);
}
