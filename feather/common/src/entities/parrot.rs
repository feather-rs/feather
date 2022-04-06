// This file is @generated. Please do not edit.
use base::EntityKind;
use quill_common::entities::Parrot;
use vane::EntityBuilder;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Parrot).add(EntityKind::Parrot);
}
