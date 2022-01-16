use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Piglin};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Piglin)
        .add(Health::new(16))
        .add(EntityKind::Piglin);
}
