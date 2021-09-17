use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Guardian};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Guardian)
        .add(Health::new(30))
        .add(EntityKind::Guardian);
}
