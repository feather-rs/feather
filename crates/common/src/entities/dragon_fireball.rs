use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::DragonFireball;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(DragonFireball).add(EntityKind::DragonFireball);
}
