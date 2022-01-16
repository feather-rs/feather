use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Wolf};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Wolf).add(Health::new(8)).add(EntityKind::Wolf);
}
