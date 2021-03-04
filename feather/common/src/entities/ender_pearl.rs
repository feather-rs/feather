use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::EnderPearl;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(EnderPearl).add(EntityKind::EnderPearl);
}
