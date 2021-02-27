use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::HopperMinecart;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(HopperMinecart).add(EntityKind::HopperMinecart);
}
