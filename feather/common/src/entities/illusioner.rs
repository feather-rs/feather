use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Illusioner};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Illusioner)
        .add(Health::new(32))
        .add(EntityKind::Illusioner);
}
