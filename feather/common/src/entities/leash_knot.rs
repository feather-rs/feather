// This file is @generated. Please do not edit.
use libcraft::EntityKind;
use quill::entities::LeashKnot;
use vane::EntityBuilder;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(LeashKnot).add(EntityKind::LeashKnot);
}
