use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Cow;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Cow).add(EntityKind::Cow);
}
