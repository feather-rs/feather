use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Vindicator;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Vindicator).add(EntityKind::Vindicator);
}
