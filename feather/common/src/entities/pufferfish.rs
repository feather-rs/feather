use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Pufferfish};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Pufferfish)
        .add(Health::new(3))
        .add(EntityKind::Pufferfish);
}
