// This file is @generated. Please do not edit.
use base::EntityKind;
use quill::entities::Mule;
use vane::EntityBuilder;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Mule).add(EntityKind::Mule);
}
