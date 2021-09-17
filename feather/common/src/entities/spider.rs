use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Spider};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Spider)
        .add(Health::new(16))
        .add(EntityKind::Spider);
}
