use quill_common::entities::Evoker;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Evoker).add(EntityKind::Evoker);
}