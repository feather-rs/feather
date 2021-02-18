use quill_common::entities::SnowGolem;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(SnowGolem).add(EntityKind::SnowGolem);
}
