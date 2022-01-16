use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Drowned};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Drowned)
        .add(Health::new(20))
        .add(EntityKind::Drowned);
}
