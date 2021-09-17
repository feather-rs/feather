use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::ElderGuardian};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(ElderGuardian)
        .add(Health::new(80))
        .add(EntityKind::ElderGuardian);
}
