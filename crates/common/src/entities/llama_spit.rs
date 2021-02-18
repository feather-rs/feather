use quill_common::entities::LlamaSpit;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(LlamaSpit).add(EntityKind::LlamaSpit);
}
