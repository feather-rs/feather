// This file is @generated. Please do not edit.
use base::EntityKind;
use quill_common::entities::Ocelot;
use vane::EntityBuilder;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Ocelot).add(EntityKind::Ocelot);
}
