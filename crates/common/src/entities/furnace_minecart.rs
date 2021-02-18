use quill_common::entities::FurnaceMinecart;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(FurnaceMinecart).add(EntityKind::FurnaceMinecart);
}
