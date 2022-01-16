use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::CommandBlockMinecart};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(CommandBlockMinecart)
        .add(Health::new(6))
        .add(EntityKind::CommandBlockMinecart);
}
