use quill_common::entities::Vindicator;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Vindicator).add(EntityKind::Vindicator);
}
