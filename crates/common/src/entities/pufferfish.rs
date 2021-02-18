use quill_common::entities::Pufferfish;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Pufferfish).add(EntityKind::Pufferfish);
}
