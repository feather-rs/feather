use quill_common::entities::CommandBlockMinecart;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(CommandBlockMinecart).add(EntityKind::CommandBlockMinecart);
}
