// This file is @generated. Please do not edit.
use libcraft::EntityKind;
use quill::entities::Slime;
use vane::EntityBuilder;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Slime).add(EntityKind::Slime);
}
