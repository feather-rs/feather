use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::EvokerFangs;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(EvokerFangs).add(EntityKind::EvokerFangs);
}
