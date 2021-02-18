use quill_common::entities::Wither;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Wither).add(EntityKind::Wither);
}
