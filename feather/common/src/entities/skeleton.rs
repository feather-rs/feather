use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Skeleton};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Skeleton)
        .add(Health::new(20))
        .add(EntityKind::Skeleton);
}
