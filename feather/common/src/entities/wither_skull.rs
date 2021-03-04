use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::WitherSkull;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(WitherSkull).add(EntityKind::WitherSkull);
}
