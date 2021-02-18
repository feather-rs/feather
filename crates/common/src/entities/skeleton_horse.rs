use quill_common::entities::SkeletonHorse;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(SkeletonHorse).add(EntityKind::SkeletonHorse);
}
