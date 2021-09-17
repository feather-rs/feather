use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Rabbit};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Rabbit)
        .add(Health::new(3))
        .add(EntityKind::Rabbit);
}
