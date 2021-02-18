use quill_common::entities::Panda;
use base::EntityKind;
use ecs::EntityBuilder;

pub fn build_default(builder: &mut EntityBuilder) {
    super::build_default(builder);
    builder.add(Panda).add(EntityKind::Panda);
}
