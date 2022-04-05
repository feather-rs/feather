// This file is @generated. Please do not edit.
use base::EntityKind;
use vane::EntityBuilder;
use quill_common::entities::WitherSkull;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(WitherSkull).add(EntityKind::WitherSkull);
}
