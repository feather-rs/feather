use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Boat};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Boat).add(Health::new(6)).add(EntityKind::Boat);
}
