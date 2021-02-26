use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Parrot;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Parrot).add(EntityKind::Parrot);
}
