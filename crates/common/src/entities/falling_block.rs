use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::FallingBlock;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(FallingBlock).add(EntityKind::FallingBlock);
}
