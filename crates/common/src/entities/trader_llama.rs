use quill_common::entities::TraderLlama;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(TraderLlama).add(EntityKind::TraderLlama);
}
