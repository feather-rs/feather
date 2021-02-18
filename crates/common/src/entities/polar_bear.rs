use quill_common::entities::PolarBear;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(PolarBear).add(EntityKind::PolarBear);
}
