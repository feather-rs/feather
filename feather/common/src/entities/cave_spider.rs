// This file is @generated. Please do not edit.
use base::EntityKind;
use quill_common::entities::CaveSpider;
use vane::EntityBuilder;
pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(CaveSpider).add(EntityKind::CaveSpider);
}
