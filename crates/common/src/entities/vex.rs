use quill_common::entities::Vex;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Vex).add(EntityKind::Vex);
}
