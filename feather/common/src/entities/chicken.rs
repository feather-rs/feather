use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::Chicken};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(Chicken)
        .add(Health::new(4))
        .add(EntityKind::Chicken);
}
