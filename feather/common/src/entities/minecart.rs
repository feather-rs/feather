use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Minecart};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Minecart)
        .add(Health::new(6))
        .add(EntityKind::Minecart);
}
