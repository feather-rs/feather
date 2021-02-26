use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Silverfish;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Silverfish).add(EntityKind::Silverfish);
}
