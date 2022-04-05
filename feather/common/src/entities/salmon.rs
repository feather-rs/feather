// This file is @generated. Please do not edit.
use base::EntityKind;
use vane::EntityBuilder;
use quill_common::entities::Salmon;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Salmon).add(EntityKind::Salmon);
}
