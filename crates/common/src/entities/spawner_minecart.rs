use quill_common::entities::SpawnerMinecart;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(SpawnerMinecart).add(EntityKind::SpawnerMinecart);
}
