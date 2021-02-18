use quill_common::entities::Mule;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Mule).add(EntityKind::Mule);
}
