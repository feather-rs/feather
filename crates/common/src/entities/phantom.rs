use quill_common::entities::Phantom;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Phantom).add(EntityKind::Phantom);
}
