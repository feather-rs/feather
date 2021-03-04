use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::ArmorStand;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(ArmorStand).add(EntityKind::ArmorStand);
}
