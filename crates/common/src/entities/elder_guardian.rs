use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::ElderGuardian;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(ElderGuardian).add(EntityKind::ElderGuardian);
}
