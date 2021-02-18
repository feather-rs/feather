use quill_common::entities::ChestMinecart;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(ChestMinecart).add(EntityKind::ChestMinecart);
}
