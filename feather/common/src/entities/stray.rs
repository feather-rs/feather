use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Stray};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Stray)
        .add(Health::new(20))
        .add(EntityKind::Stray);
}
