// This file is @generated. Please do not edit.
use base::EntityKind;
use vane::EntityBuilder;
use quill_common::entities::Wither;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Wither).add(EntityKind::Wither);
}
