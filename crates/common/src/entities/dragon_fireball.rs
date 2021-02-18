use quill_common::entities::DragonFireball;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(DragonFireball).add(EntityKind::DragonFireball);
}
