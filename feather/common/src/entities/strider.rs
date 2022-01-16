use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Strider};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Strider)
        .add(Health::new(20))
        .add(EntityKind::Strider);
}
