use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::FurnaceMinecart;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(FurnaceMinecart)
        .add(EntityKind::FurnaceMinecart);
}
