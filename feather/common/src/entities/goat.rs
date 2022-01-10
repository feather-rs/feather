// This file is @generated. Please do not edit.
use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Goat;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Goat).add(EntityKind::Goat);
}
