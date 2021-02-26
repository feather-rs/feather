use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::WitherSkeleton;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(WitherSkeleton).add(EntityKind::WitherSkeleton);
}
