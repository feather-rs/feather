use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::SpawnerMinecart};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(SpawnerMinecart)
        .add(Health::new(6))
        .add(EntityKind::SpawnerMinecart);
}
