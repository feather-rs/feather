use quill_common::entities::Skeleton;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Skeleton).add(EntityKind::Skeleton);
}
