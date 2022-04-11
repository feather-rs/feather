// This file is @generated. Please do not edit.
use libcraft::EntityKind;
use quill::components::EntityKindComponent;
use quill::entities::SpawnerMinecart;
use vane::EntityBuilder;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(SpawnerMinecart)
        .add(EntityKindComponent(EntityKind::SpawnerMinecart));
}
