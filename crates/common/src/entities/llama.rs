use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Llama;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Llama).add(EntityKind::Llama);
}
