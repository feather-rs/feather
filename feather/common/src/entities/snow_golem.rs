use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::SnowGolem};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(SnowGolem)
        .add(Health::new(4))
        .add(EntityKind::SnowGolem);
}
