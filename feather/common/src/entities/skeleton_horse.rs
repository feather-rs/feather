use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::SkeletonHorse};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(SkeletonHorse)
        .add(Health::new(15))
        .add(EntityKind::SkeletonHorse);
}
