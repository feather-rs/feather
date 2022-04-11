// This file is @generated. Please do not edit.
use libcraft::EntityKind;
use quill::components::EntityKindComponent;
use quill::entities::Spider;
use vane::EntityBuilder;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Spider)
        .add(EntityKindComponent(EntityKind::Spider));
}
