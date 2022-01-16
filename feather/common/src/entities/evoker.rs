use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Evoker};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Evoker)
        .add(Health::new(24))
        .add(EntityKind::Evoker);
}
