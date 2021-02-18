use quill_common::entities::FireworkRocket;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(FireworkRocket).add(EntityKind::FireworkRocket);
}
