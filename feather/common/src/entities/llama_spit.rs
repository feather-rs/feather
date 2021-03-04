use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::LlamaSpit;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(LlamaSpit).add(EntityKind::LlamaSpit);
}
