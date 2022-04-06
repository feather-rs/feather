// This file is @generated. Please do not edit.
use base::EntityKind;
use quill_common::entities::EvokerFangs;
use vane::EntityBuilder;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(EvokerFangs).add(EntityKind::EvokerFangs);
}
