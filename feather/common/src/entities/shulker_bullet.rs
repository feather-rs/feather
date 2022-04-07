// This file is @generated. Please do not edit.
use base::EntityKind;
use quill::entities::ShulkerBullet;
use vane::EntityBuilder;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(ShulkerBullet).add(EntityKind::ShulkerBullet);
}
