use quill_common::entities::EnderDragon;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(EnderDragon).add(EntityKind::EnderDragon);
}
