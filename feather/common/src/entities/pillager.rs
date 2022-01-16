use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Pillager};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Pillager)
        .add(Health::new(24))
        .add(EntityKind::Pillager);
}
