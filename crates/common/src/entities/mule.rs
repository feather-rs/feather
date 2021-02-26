use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Mule;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Mule).add(EntityKind::Mule);
}
