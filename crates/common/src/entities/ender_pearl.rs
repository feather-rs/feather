use quill_common::entities::EnderPearl;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(EnderPearl).add(EntityKind::EnderPearl);
}
