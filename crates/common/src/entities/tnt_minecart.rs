use quill_common::entities::TntMinecart;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(TntMinecart).add(EntityKind::TntMinecart);
}
