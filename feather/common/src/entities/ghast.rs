use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Ghast};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Ghast)
        .add(Health::new(10))
        .add(EntityKind::Ghast);
}
