use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Dolphin};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Dolphin)
        .add(Health::new(10))
        .add(EntityKind::Dolphin);
}
