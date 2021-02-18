use quill_common::entities::ExperienceOrb;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(ExperienceOrb).add(EntityKind::ExperienceOrb);
}
