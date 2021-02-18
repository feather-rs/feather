use quill_common::entities::Silverfish;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Silverfish).add(EntityKind::Silverfish);
}
