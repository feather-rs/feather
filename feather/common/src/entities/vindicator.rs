use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Vindicator};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Vindicator)
        .add(Health::new(24))
        .add(EntityKind::Vindicator);
}
