// This file is @generated. Please do not edit.
use base::EntityKind;
use quill::entities::Panda;
use vane::EntityBuilder;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Panda).add(EntityKind::Panda);
}
