// This file is @generated. Please do not edit.
use base::EntityKind;
use quill_common::entities::Horse;
use vane::EntityBuilder;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Horse).add(EntityKind::Horse);
}
