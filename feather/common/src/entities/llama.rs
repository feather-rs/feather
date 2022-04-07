// This file is @generated. Please do not edit.
use libcraft::EntityKind;
use quill::entities::Llama;
use vane::EntityBuilder;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Llama).add(EntityKind::Llama);
}
