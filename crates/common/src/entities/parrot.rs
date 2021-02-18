use quill_common::entities::Parrot;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Parrot).add(EntityKind::Parrot);
}
