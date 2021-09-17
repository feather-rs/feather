use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Squid};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Squid)
        .add(Health::new(10))
        .add(EntityKind::Squid);
}
