use quill_common::entities::ZombifiedPiglin;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(ZombifiedPiglin).add(EntityKind::ZombifiedPiglin);
}
