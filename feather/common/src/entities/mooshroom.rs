use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Mooshroom};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Mooshroom)
        .add(Health::new(10))
        .add(EntityKind::Mooshroom);
}
