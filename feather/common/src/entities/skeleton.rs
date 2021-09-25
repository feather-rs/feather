use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Skeleton;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Skeleton).add(EntityKind::Skeleton);
}
