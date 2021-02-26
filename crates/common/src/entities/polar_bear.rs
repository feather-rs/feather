use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::PolarBear;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(PolarBear).add(EntityKind::PolarBear);
}
