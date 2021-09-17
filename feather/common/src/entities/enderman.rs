use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Enderman};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Enderman)
        .add(Health::new(40))
        .add(EntityKind::Enderman);
}
