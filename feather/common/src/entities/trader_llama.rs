use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::TraderLlama;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(TraderLlama).add(EntityKind::TraderLlama);
}
