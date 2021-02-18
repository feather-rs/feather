use quill_common::entities::Llama;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Llama).add(EntityKind::Llama);
}
