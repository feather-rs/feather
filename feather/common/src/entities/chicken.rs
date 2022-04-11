// This file is @generated. Please do not edit.
use quill::entities::Chicken;
use vane::EntityBuilder;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Chicken);
}
