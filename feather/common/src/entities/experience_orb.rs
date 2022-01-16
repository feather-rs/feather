use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::{components::Health, entities::ExperienceOrb};

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder
        .add(ExperienceOrb)
        .add(Health::new(5))
        .add(EntityKind::ExperienceOrb);
}
