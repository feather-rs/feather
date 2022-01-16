use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Salmon};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Salmon)
        .add(Health::new(3))
        .add(EntityKind::Salmon);
}
