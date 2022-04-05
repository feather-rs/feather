// This file is @generated. Please do not edit.
use base::EntityKind;
use vane::EntityBuilder;
use quill_common::entities::Phantom;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Phantom).add(EntityKind::Phantom);
}
