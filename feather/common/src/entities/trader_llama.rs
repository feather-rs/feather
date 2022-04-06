// This file is @generated. Please do not edit.
use base::EntityKind;
use quill_common::entities::TraderLlama;
use vane::EntityBuilder;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(TraderLlama).add(EntityKind::TraderLlama);
}
