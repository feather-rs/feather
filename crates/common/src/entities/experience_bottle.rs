use quill_common::entities::ExperienceBottle;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(ExperienceBottle).add(EntityKind::ExperienceBottle);
}
