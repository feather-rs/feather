use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Phantom};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Phantom)
        .add(Health::new(20))
        .add(EntityKind::Phantom);
}
