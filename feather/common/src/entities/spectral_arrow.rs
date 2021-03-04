use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::SpectralArrow;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(SpectralArrow).add(EntityKind::SpectralArrow);
}
