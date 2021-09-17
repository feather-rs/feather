use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Ravager};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Ravager)
        .add(Health::new(100))
        .add(EntityKind::Ravager);
}
