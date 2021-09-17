use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::PiglinBrute};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(PiglinBrute)
        .add(Health::new(50))
        .add(EntityKind::PiglinBrute);
}
