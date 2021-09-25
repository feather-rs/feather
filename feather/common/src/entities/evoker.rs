use base::EntityKind;
use ecs::EntityBuilder;
use quill_common::entities::Evoker;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Evoker).add(EntityKind::Evoker);
}
