use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::ZombifiedPiglin};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(ZombifiedPiglin)
        .add(Health::new(20))
        .add(EntityKind::ZombifiedPiglin);
}
