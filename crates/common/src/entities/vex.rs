use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Vex;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Vex).add(EntityKind::Vex);
}
