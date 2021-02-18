use quill_common::entities::Hoglin;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Hoglin).add(EntityKind::Hoglin);
}
