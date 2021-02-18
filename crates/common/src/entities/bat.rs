use quill_common::entities::Bat;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Bat).add(EntityKind::Bat);
}
