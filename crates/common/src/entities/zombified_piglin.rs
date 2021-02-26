use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::ZombifiedPiglin;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(ZombifiedPiglin)
        .add(EntityKind::ZombifiedPiglin);
}
