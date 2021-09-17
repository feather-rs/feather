use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::EnderDragon};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(EnderDragon)
        .add(Health::new(200))
        .add(EntityKind::EnderDragon);
}
