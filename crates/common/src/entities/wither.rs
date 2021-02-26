use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Wither;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Wither).add(EntityKind::Wither);
}
