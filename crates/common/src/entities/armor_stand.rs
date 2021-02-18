use quill_common::entities::ArmorStand;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(ArmorStand).add(EntityKind::ArmorStand);
}
