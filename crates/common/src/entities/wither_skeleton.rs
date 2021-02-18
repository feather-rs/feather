use quill_common::entities::WitherSkeleton;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(WitherSkeleton).add(EntityKind::WitherSkeleton);
}
