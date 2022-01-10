// This file is @generated. Please do not edit.
use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::GlowSquid;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(GlowSquid).add(EntityKind::GlowSquid);
}
