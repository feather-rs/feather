use quill_common::entities::Strider;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Strider).add(EntityKind::Strider);
}
