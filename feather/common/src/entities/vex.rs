use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Vex};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Vex).add(Health::new(14)).add(EntityKind::Vex);
}
