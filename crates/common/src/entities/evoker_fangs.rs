use quill_common::entities::EvokerFangs;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(EvokerFangs).add(EntityKind::EvokerFangs);
}
