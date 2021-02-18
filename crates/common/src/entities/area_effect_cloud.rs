use quill_common::entities::AreaEffectCloud;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(AreaEffectCloud).add(EntityKind::AreaEffectCloud);
}
