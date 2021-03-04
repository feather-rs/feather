use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Hoglin;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Hoglin).add(EntityKind::Hoglin);
}
