use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::EndCrystal};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(EndCrystal)
        .add(Health::new(5))
        .add(EntityKind::EndCrystal);
}
